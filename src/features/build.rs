use std::{
    io::{BufRead, BufReader, Read},
    path::Path,
    process::{Command, Stdio},
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
};

use anyhow::Result;
use cancellation::CancellationToken;
use chashmap::CHashMap;
use crossbeam_channel::Sender;
use encoding_rs_io::DecodeReaderBytesBuilder;
use lsp_types::{
    notification::{LogMessage, Progress},
    LogMessageParams, NumberOrString, Position, ProgressParams, ProgressParamsValue,
    TextDocumentIdentifier, TextDocumentPositionParams, WorkDoneProgress, WorkDoneProgressBegin,
    WorkDoneProgressEnd,
};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::{client, Uri};

use super::{forward_search, FeatureRequest};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildParams {
    pub text_document: TextDocumentIdentifier,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum BuildStatus {
    SUCCESS = 0,
    ERROR = 1,
    FAILURE = 2,
    CANCELLED = 3,
    BUSY = 4,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildResult {
    pub status: BuildStatus,
}

#[derive(Default)]
pub struct BuildEngine {
    lock: Mutex<()>,
    pub positions_by_uri: CHashMap<Arc<Uri>, Position>,
}

impl BuildEngine {
    pub fn build(
        &self,
        request: FeatureRequest<BuildParams>,
        cancellation_token: &CancellationToken,
        lsp_sender: &Sender<lsp_server::Message>,
    ) -> Result<BuildResult> {
        let lock = self.lock.lock().unwrap();

        let document = request
            .subset
            .documents
            .iter()
            .find(|document| {
                if let Some(data) = document.data.as_latex() {
                    data.extras.has_document_environment
                } else {
                    false
                }
            })
            .map(|document| document.as_ref())
            .unwrap_or_else(|| request.main_document());

        if document.uri.scheme() != "file" {
            return Ok(BuildResult {
                status: BuildStatus::FAILURE,
            });
        }
        let path = document.uri.to_file_path().unwrap();

        let supports_progress = {
            request
                .context
                .client_capabilities
                .lock()
                .unwrap()
                .window
                .as_ref()
                .and_then(|window| window.work_done_progress)
                .unwrap_or_default()
        };

        let token = "texlab-build";

        if supports_progress {
            client::send_notification::<Progress>(
                lsp_sender,
                ProgressParams {
                    token: NumberOrString::String(token.to_string()),
                    value: ProgressParamsValue::WorkDone(WorkDoneProgress::Begin(
                        WorkDoneProgressBegin {
                            title: "Building".to_string(),
                            message: Some(document.uri.as_str().to_string()),
                            cancellable: Some(false),
                            percentage: None,
                        },
                    )),
                },
            )?;
        }

        let options = { request.context.options.read().unwrap().clone() };

        let build_dir = options
            .root_directory
            .as_ref()
            .map(AsRef::as_ref)
            .or_else(|| path.parent())
            .unwrap();

        let args: Vec<_> = options
            .build
            .args()
            .into_iter()
            .map(|arg| replace_placeholder(arg, &path))
            .collect();

        let mut process = Command::new(options.build.executable())
            .args(args)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .current_dir(build_dir)
            .spawn()?;

        let log_handle = capture_output(&mut process, lsp_sender);
        let success = process.wait().map(|status| status.success())?;
        log_handle.join().unwrap();
        let status = if success {
            BuildStatus::SUCCESS
        } else {
            BuildStatus::ERROR
        };

        if supports_progress {
            client::send_notification::<Progress>(
                lsp_sender,
                ProgressParams {
                    token: NumberOrString::String(token.to_string()),
                    value: ProgressParamsValue::WorkDone(WorkDoneProgress::End(
                        WorkDoneProgressEnd { message: None },
                    )),
                },
            )?;
        }
        drop(lock);

        if options.build.forward_search_after {
            let request = FeatureRequest {
                params: TextDocumentPositionParams {
                    position: self
                        .positions_by_uri
                        .get(&request.main_document().uri)
                        .map(|guard| guard.clone())
                        .unwrap_or_default(),
                    text_document: TextDocumentIdentifier::new(
                        request.main_document().uri.as_ref().clone().into(),
                    ),
                },
                context: request.context,
                workspace: request.workspace,
                subset: request.subset,
            };
            forward_search::execute_forward_search(request, cancellation_token);
        }

        Ok(BuildResult { status })
    }
}

fn capture_output(
    process: &mut std::process::Child,
    lsp_sender: &Sender<lsp_server::Message>,
) -> JoinHandle<()> {
    let (log_sender, log_receiver) = crossbeam_channel::unbounded();
    track_output(process.stdout.take().unwrap(), log_sender.clone());
    track_output(process.stderr.take().unwrap(), log_sender);
    let log_handle = {
        let lsp_sender = lsp_sender.clone();
        thread::spawn(move || {
            for message in &log_receiver {
                client::send_notification::<LogMessage>(
                    &lsp_sender,
                    LogMessageParams {
                        message,
                        typ: lsp_types::MessageType::Log,
                    },
                )
                .unwrap();
            }
        })
    };
    log_handle
}

fn replace_placeholder(arg: String, file: &Path) -> String {
    if arg.starts_with('"') || arg.ends_with('"') {
        arg
    } else {
        arg.replace("%f", &file.to_string_lossy())
    }
}

fn track_output(output: impl Read + Send + 'static, sender: Sender<String>) -> JoinHandle<()> {
    let reader = BufReader::new(
        DecodeReaderBytesBuilder::new()
            .encoding(Some(encoding_rs::UTF_8))
            .utf8_passthru(true)
            .strip_bom(true)
            .build(output),
    );

    thread::spawn(move || {
        for line in reader.lines() {
            sender.send(line.unwrap()).unwrap();
        }
    })
}

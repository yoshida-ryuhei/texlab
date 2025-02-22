use base_db::DocumentData;
use rowan::{TextRange, TextSize};
use rustc_hash::FxHashMap;
use syntax::latex;

use crate::util::cursor::CursorContext;

use super::{Indel, Params, RenameResult};

pub(super) fn prepare_rename<T>(context: &CursorContext<T>) -> Option<TextRange> {
    context.cursor.command_range(context.offset)
}

pub(super) fn rename<'a>(context: &CursorContext<'a, Params>) -> Option<RenameResult<'a>> {
    prepare_rename(context)?;
    let name = context.cursor.as_tex()?.text();
    let mut changes = FxHashMap::default();
    for document in &context.project.documents {
        let DocumentData::Tex(data) = &document.data else { continue };

        let root = data.root_node();
        let edits = root
            .descendants_with_tokens()
            .filter_map(|element| element.into_token())
            .filter(|token| token.kind() == latex::COMMAND_NAME && token.text() == name)
            .map(|token| {
                let range = token.text_range();
                Indel {
                    delete: TextRange::new(range.start() + TextSize::from(1), range.end()),
                    insert: context.params.new_name.clone(),
                }
            })
            .collect();

        changes.insert(*document, edits);
    }

    Some(RenameResult { changes })
}

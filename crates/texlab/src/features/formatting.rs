mod bibtex_internal;
mod latexindent;

use base_db::{Formatter, Workspace};
use distro::Language;
use lsp_types::{FormattingOptions, TextEdit, Url};

use self::{bibtex_internal::format_bibtex_internal, latexindent::format_with_latexindent};

pub fn format_source_code(
    workspace: &Workspace,
    uri: &Url,
    options: &FormattingOptions,
) -> Option<Vec<TextEdit>> {
    let document = workspace.lookup(uri)?;
    match document.language {
        Language::Tex => match workspace.config().formatting.tex_formatter {
            Formatter::Null => None,
            Formatter::Server => None,
            Formatter::LatexIndent => format_with_latexindent(workspace, document),
        },
        Language::Bib => match workspace.config().formatting.bib_formatter {
            Formatter::Null => None,
            Formatter::Server => format_bibtex_internal(workspace, document, options),
            Formatter::LatexIndent => format_with_latexindent(workspace, document),
        },
        Language::Aux | Language::Log | Language::Root | Language::Tectonic => None,
    }
}

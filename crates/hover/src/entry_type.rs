use base_db::data::BibtexEntryType;
use syntax::bibtex;

use crate::{Hover, HoverData, HoverParams};

pub(super) fn find_hover<'db>(params: &'db HoverParams) -> Option<Hover<'db>> {
    let data = params.document.data.as_bib()?;
    let root = data.root_node();
    let name = root
        .token_at_offset(params.offset)
        .find(|x| x.kind() == bibtex::TYPE)?;

    let entry_type = BibtexEntryType::find(&name.text()[1..])?;
    Some(Hover {
        range: name.text_range(),
        data: HoverData::EntryType(entry_type),
    })
}

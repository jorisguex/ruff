use std::collections::BTreeMap;

use lsp_types::{NotebookCellKind, Url};
use ruff_source_file::LineIndex;
use rustc_hash::FxHashMap;

use super::DocumentVersion;

#[derive(Clone)]
pub(crate) struct NotebookDocument {
    cells: Vec<NotebookCell>,
    version: DocumentVersion,
}

#[derive(Clone)]
struct NotebookCell {
    url: Url,
    kind: NotebookCellKind,
    contents: String,
}

impl NotebookDocument {
    pub(crate) fn new(
        version: DocumentVersion,
        cells: Vec<lsp_types::NotebookCell>,
        cell_documents: Vec<lsp_types::TextDocumentItem>,
    ) -> Self {
        let mut cell_contents: FxHashMap<_, _> = cell_documents
            .into_iter()
            .map(|document| (document.uri, document.text))
            .collect();
        Self {
            version,
            cells: cells
                .into_iter()
                .map(|cell| {
                    let contents = cell_contents.remove(&cell.document).unwrap_or_default();
                    NotebookCell::new(cell, contents)
                })
                .collect(),
        }
    }

    pub(crate) fn make_ruff_notebook(&self) -> ruff_notebook::Notebook {
        todo!()
    }

    pub(crate) fn version(&self) -> DocumentVersion {
        self.version
    }
}

impl NotebookCell {
    pub(crate) fn new(cell: lsp_types::NotebookCell, contents: String) -> Self {
        Self {
            url: cell.document,
            kind: cell.kind,
            contents,
        }
    }
}

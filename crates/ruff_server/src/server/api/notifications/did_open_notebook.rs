use crate::edit::NotebookDocument;
use crate::server::client::{Notifier, Requester};
use crate::server::Result;
use crate::session::Session;
use lsp_types as types;
use lsp_types::notification as notif;

pub(crate) struct DidOpenNotebook;

impl super::NotificationHandler for DidOpenNotebook {
    type NotificationType = notif::DidOpenNotebookDocument;
}

impl super::SyncNotificationHandler for DidOpenNotebook {
    fn run(
        session: &mut Session,
        _notifier: Notifier,
        _requester: &mut Requester,
        types::DidOpenNotebookDocumentParams {
            notebook_document:
                types::NotebookDocument {
                    uri: url,
                    version,
                    cells,
                    ..
                },
            cell_text_documents,
        }: types::DidOpenNotebookDocumentParams,
    ) -> Result<()> {
        let notebook = NotebookDocument::new(version, cells, cell_text_documents);

        session.open_notebook_document(&url, notebook);

        Ok(())
    }
}

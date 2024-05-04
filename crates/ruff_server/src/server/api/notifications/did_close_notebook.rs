use crate::server::client::{Notifier, Requester};
use crate::server::Result;
use crate::session::Session;
use lsp_types as types;
use lsp_types::notification as notif;

pub(crate) struct DidCloseNotebook;

impl super::NotificationHandler for DidCloseNotebook {
    type NotificationType = notif::DidCloseNotebookDocument;
}

impl super::SyncNotificationHandler for DidCloseNotebook {
    fn run(
        session: &mut Session,
        _notifier: Notifier,
        _requester: &mut Requester,
        types::DidCloseNotebookDocumentParams {
            notebook_document: types::NotebookDocumentIdentifier { uri: url },
            ..
        }: types::DidCloseNotebookDocumentParams,
    ) -> Result<()> {
        session.close_notebook_document(&url);

        Ok(())
    }
}

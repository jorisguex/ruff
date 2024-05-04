use crate::server::client::{Notifier, Requester};
use crate::server::Result;
use crate::session::Session;
use lsp_types as types;
use lsp_types::notification as notif;

pub(crate) struct DidChangeNotebook;

impl super::NotificationHandler for DidChangeNotebook {
    type NotificationType = notif::DidChangeNotebookDocument;
}

impl super::SyncNotificationHandler for DidChangeNotebook {
    #[tracing::instrument(skip_all, fields(file=%uri))]
    fn run(
        session: &mut Session,
        _notifier: Notifier,
        _requester: &mut Requester,
        types::DidChangeNotebookDocumentParams {
            notebook_document: types::VersionedNotebookDocumentIdentifier { uri, version },
            change: types::NotebookDocumentChangeEvent { cells, .. },
        }: types::DidChangeNotebookDocumentParams,
    ) -> Result<()> {
        if let Some(types::NotebookDocumentCellChange {
            structure,
            data,
            text_content,
        }) = cells
        {
            todo!()
        }
        Ok(())
    }
}

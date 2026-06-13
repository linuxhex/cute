use std::path::PathBuf;

use warpui::{AppContext, ModelContext, SingletonEntity};

use crate::ai::agent::{SearchCodebaseFailureReason, SearchCodebaseResult};
use crate::ai::blocklist::SessionContext;
use crate::ai::codebase_auto_indexing::{
    should_use_codebase_indexing, CodebaseAutoIndexingSurface,
};
use crate::ai::get_relevant_files::controller::GetRelevantFilesController;
use crate::remote_server::codebase_index_model::{
    RemoteCodebaseIndexModel, RemoteCodebaseSearchAvailability,
};

pub(super) enum RemoteSearchRequest {
    Pending(futures_util::stream::AbortHandle),
    Ready(SearchCodebaseResult),
}

pub(super) fn root_directory_for_search(
    session_context: &SessionContext,
    requested_codebase_path: Option<&str>,
    app: &AppContext,
) -> Option<PathBuf> {
    RemoteCodebaseIndexModel::as_ref(app)
        .active_repo_path(session_context, requested_codebase_path)
        .or_else(|| {
            requested_codebase_path
                .is_none()
                .then(|| session_context.current_working_directory().as_ref().map(|s| PathBuf::from(s)))
                .flatten()
        })
}

pub(super) fn send_request(
    _query: String,
    _partial_paths: Option<Vec<String>>,
    session_context: SessionContext,
    requested_codebase_path: Option<String>,
    _action_id: crate::ai::agent::AIAgentActionId,
    ctx: &mut ModelContext<GetRelevantFilesController>,
) -> RemoteSearchRequest {
    if !should_use_codebase_indexing(CodebaseAutoIndexingSurface::Remote, ctx) {
        return RemoteSearchRequest::Ready(SearchCodebaseResult::Failed {
            reason: SearchCodebaseFailureReason::CodebaseNotIndexed,
            message: "Remote codebase search is not enabled.".to_string(),
        });
    }

    let availability = RemoteCodebaseIndexModel::as_ref(ctx)
        .active_repo_availability(&session_context, requested_codebase_path.as_deref());
    match availability {
        RemoteCodebaseSearchAvailability::Ready(search_context) => {
            let Some(remote_path) = search_context.remote_path.as_ref() else {
                return RemoteSearchRequest::Ready(SearchCodebaseResult::Failed {
                    reason: SearchCodebaseFailureReason::ClientError,
                    message: "Remote codebase search is unavailable because the remote path is not set.".to_string(),
                });
            };
            // RemoteServerManager has been removed; remote codebase search is unavailable.
            let _ = remote_path;
            return RemoteSearchRequest::Ready(SearchCodebaseResult::Failed {
                reason: SearchCodebaseFailureReason::ClientError,
                message: "Remote codebase search is unavailable because the remote server module has been removed.".to_string(),
            });
        }
        availability @ RemoteCodebaseSearchAvailability::NotIndexed { .. } => {
            RemoteSearchRequest::Ready(remote_availability_failure(availability))
        }
        RemoteCodebaseSearchAvailability::NoConnectedHost
        | RemoteCodebaseSearchAvailability::NoActiveRepo
        | RemoteCodebaseSearchAvailability::Indexing { .. }
        | RemoteCodebaseSearchAvailability::Unavailable { .. } => {
            RemoteSearchRequest::Ready(remote_availability_failure(availability))
        }
    }
}

fn remote_availability_failure(
    availability: RemoteCodebaseSearchAvailability,
) -> SearchCodebaseResult {
    match availability {
        RemoteCodebaseSearchAvailability::NoConnectedHost => SearchCodebaseResult::Failed {
            reason: SearchCodebaseFailureReason::ClientError,
            message:
                "Remote codebase search is unavailable because the remote host is not connected."
                    .to_string(),
        },
        RemoteCodebaseSearchAvailability::NoActiveRepo => SearchCodebaseResult::Failed {
            reason: SearchCodebaseFailureReason::CodebaseNotIndexed,
            message: "The current remote directory is not in a known codebase.".to_string(),
        },
        RemoteCodebaseSearchAvailability::NotIndexed { remote_path } => {
            SearchCodebaseResult::Failed {
                reason: SearchCodebaseFailureReason::CodebaseNotIndexed,
                message: format!(
                    "The remote codebase at {} is not indexed yet.",
                    remote_path.path.as_str()
                ),
            }
        }
        RemoteCodebaseSearchAvailability::Indexing { remote_path } => {
            SearchCodebaseResult::Failed {
                reason: SearchCodebaseFailureReason::CodebaseNotIndexed,
                message: format!(
                    "The remote codebase at {} is still being indexed. Try again later.",
                    remote_path.path.as_str()
                ),
            }
        }
        RemoteCodebaseSearchAvailability::Unavailable {
            remote_path,
            message,
        } => SearchCodebaseResult::Failed {
            reason: SearchCodebaseFailureReason::CodebaseNotIndexed,
            message: format!(
                "Remote codebase search is unavailable for {}: {message}",
                remote_path.path.as_str()
            ),
        },
        RemoteCodebaseSearchAvailability::Ready(_) => SearchCodebaseResult::Failed {
            reason: SearchCodebaseFailureReason::ClientError,
            message: "Remote codebase search was unexpectedly unavailable.".to_string(),
        },
    }
}

#[cfg(test)]
#[path = "native_tests.rs"]
mod tests;

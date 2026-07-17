//! Snapshot file writer handle for declarations tracking.
//!
//! Minimal stub module providing the DeclarationsWriterHandle used by AgentDriver.
//! Full snapshot upload pipeline functionality has been removed as unused.

use std::collections::HashSet;
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::time::Duration;

use anyhow::{Context as _, Result};
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt as _;
use tokio::sync::{mpsc, oneshot};
use cuteui::r#async::executor::Background;

use crate::ai::ambient_agent_types::AmbientAgentTaskId;

/// Upper bound on declarations-script runtime.
pub(super) const DEFAULT_DECLARATIONS_SCRIPT_TIMEOUT: Duration = Duration::from_secs(60);

/// Upper bound on the end-of-run upload pipeline's total runtime.
pub(super) const DEFAULT_SNAPSHOT_UPLOAD_TIMEOUT: Duration = Duration::from_secs(120);

#[allow(dead_code)]
const DECLARATION_VERSION: u32 = 1;
#[allow(dead_code)]
const DECLARATIONS_PATH_ENV_VAR: &str = "OZ_SNAPSHOT_DECLARATIONS_FILE";
#[allow(dead_code)]
const DEFAULT_DECLARATIONS_DIR: &str = "/tmp/oz";
#[allow(dead_code)]
const DEFAULT_DECLARATIONS_FILENAME: &str = "snapshot-declarations.jsonl";

// --- Declarations writer handle (used by AgentDriver) ---

/// Commands accepted by the async declarations writer task.
enum WriterCommand {
    /// Append file entries for the given paths to the declarations file.
    #[allow(dead_code)]
    Append(Vec<String>),
    /// Acknowledge once every previously-queued command has finished its fs writes.
    #[allow(dead_code)]
    Flush(oneshot::Sender<()>),
}

/// Handle used by the SDK driver to enqueue file declaration appends.
///
/// Minimal stub: the underlying writer task and flush logic remain for
/// structural compatibility with AgentDriver, but no actual upload happens.
#[derive(Clone)]
pub(super) struct DeclarationsWriterHandle {
    tx: mpsc::UnboundedSender<WriterCommand>,
}

impl DeclarationsWriterHandle {
    /// Spawn the writer task on background and return a handle.
    #[allow(dead_code)]
    pub(super) fn new(
        task_id: AmbientAgentTaskId,
        working_dir: PathBuf,
        background: &Background,
    ) -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        let declarations_path = resolve_declarations_path(Some(&task_id));
        background
            .spawn(writer_task(rx, declarations_path, working_dir, task_id))
            .detach();
        Self { tx }
    }

    /// Enqueue paths for appending as file entries (non-blocking).
    pub(super) fn append(&self, paths: Vec<String>) {
        if paths.is_empty() {
            return;
        }
        if let Err(e) = self.tx.send(WriterCommand::Append(paths)) {
            log::warn!("Declarations writer channel closed; dropping append: {e}");
        }
    }

    /// Awaits until every previously-queued append has finished its fs writes.
    #[allow(dead_code)]
    pub(super) async fn flush(&self) {
        let (ack_tx, ack_rx) = oneshot::channel();
        if self.tx.send(WriterCommand::Flush(ack_tx)).is_err() {
            return;
        }
        if ack_rx.await.is_err() {
            log::warn!("Declarations writer flush oneshot dropped without ack");
        }
    }
}

/// Writer task loop: owns the seen set, lazily opens the file per write.
#[allow(dead_code)]
async fn writer_task(
    mut rx: mpsc::UnboundedReceiver<WriterCommand>,
    declarations_path: PathBuf,
    working_dir: PathBuf,
    task_id: AmbientAgentTaskId,
) {
    let mut seen: HashSet<String> = HashSet::new();
    while let Some(cmd) = rx.recv().await {
        match cmd {
            WriterCommand::Append(paths) => {
                for path in paths {
                    process_append_path(
                        path,
                        &declarations_path,
                        &working_dir,
                        &task_id,
                        &mut seen,
                    )
                    .await;
                }
            }
            WriterCommand::Flush(ack) => {
                let _ = ack.send(());
            }
        }
    }
}

/// Normalize and write one JSONL line for raw_path.
#[allow(dead_code)]
async fn process_append_path(
    raw_path: String,
    declarations_path: &Path,
    working_dir: &Path,
    task_id: &AmbientAgentTaskId,
    seen: &mut HashSet<String>,
) {
    let candidate = Path::new(&raw_path);
    let absolute = if candidate.is_absolute() {
        candidate.to_path_buf()
    } else {
        working_dir.join(candidate)
    };
    if !absolute.is_absolute() {
        log::warn!(
            "Skipping non-absolute file-edit path {absolute:?} for declarations (task {task_id})"
        );
        return;
    }
    let Some(absolute_str) = absolute.to_str().map(str::to_owned) else {
        log::warn!(
            "Skipping non-UTF-8 file-edit path {absolute:?} for declarations (task {task_id})"
        );
        return;
    };
    if seen.contains(&absolute_str) {
        return;
    }
    if path_is_under_existing_repo(&absolute).await {
        log::debug!(
            "Skipping file declaration for '{absolute_str}': already inside an existing git repo (task {task_id})"
        );
        seen.insert(absolute_str);
        return;
    }
    match append_declaration_line(declarations_path, &absolute_str).await {
        Ok(()) => {
            seen.insert(absolute_str);
        }
        Err(e) => {
            log::warn!(
                "Failed to append file declaration for '{absolute_str}': {e:#} (task {task_id})"
            );
        }
    }
}

/// Check if path is under an existing repo by checking for .git directories.
#[allow(dead_code)]
async fn path_is_under_existing_repo(path: &Path) -> bool {
    let mut current = path.parent();
    while let Some(dir) = current {
        let git_dir = dir.join(".git");
        if tokio::fs::try_exists(&git_dir).await.unwrap_or(false) {
            return true;
        }
        current = dir.parent();
    }
    false
}

/// Append one JSONL line for path to the declarations file.
#[allow(dead_code)]
async fn append_declaration_line(declarations_path: &Path, path: &str) -> Result<()> {
    if let Some(parent) = declarations_path.parent() {
        tokio::fs::create_dir_all(parent)
            .await
            .with_context(|| format!("create_dir_all {}", parent.display()))?;
    }
    let mut line = serde_json::to_string(&serde_json::json!({
        "version": DECLARATION_VERSION,
        "kind": "file",
        "path": path
    }))
    .context("serialize file declaration")?;
    line.push('\n');
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(declarations_path)
        .await
        .with_context(|| format!("open declarations file {}", declarations_path.display()))?;
    file.write_all(line.as_bytes())
        .await
        .with_context(|| format!("write declarations file {}", declarations_path.display()))?;
    file.flush()
        .await
        .with_context(|| format!("flush declarations file {}", declarations_path.display()))?;
    Ok(())
}

/// Resolve the declarations file path from env and optional task ID.
#[allow(dead_code)]
fn resolve_declarations_path(task_id: Option<&AmbientAgentTaskId>) -> PathBuf {
    resolve_declarations_path_with_override(task_id, std::env::var_os(DECLARATIONS_PATH_ENV_VAR))
}

/// Pure resolver: returns the declarations file path given an explicit override.
#[allow(dead_code)]
fn resolve_declarations_path_with_override(
    task_id: Option<&AmbientAgentTaskId>,
    override_path: Option<OsString>,
) -> PathBuf {
    if let Some(override_path) = override_path {
        return PathBuf::from(override_path);
    }
    match task_id {
        Some(id) => PathBuf::from(DEFAULT_DECLARATIONS_DIR)
            .join(id.to_string())
            .join(DEFAULT_DECLARATIONS_FILENAME),
        None => PathBuf::from(DEFAULT_DECLARATIONS_DIR).join(DEFAULT_DECLARATIONS_FILENAME),
    }
}

use async_trait::async_trait;
use command_group::{AsyncCommandGroup, AsyncGroupChild};
use tokio::process::Command;
use uuid::Uuid;

use crate::{
    executor::{
        Executor, ExecutorError, NormalizedConversation, NormalizedEntry,
        NormalizedEntryType,
    },
    models::task::Task,
    utils::shell::get_shell_command,
};

/// An executor that generates PRPs (Project Research Papers) for tasks
/// This executor creates comprehensive task documentation during the Planning phase
pub struct PrpExecutor;

#[async_trait]
impl Executor for PrpExecutor {
    async fn spawn(
        &self,
        pool: &sqlx::SqlitePool,
        task_id: Uuid,
        worktree_path: &str,
    ) -> Result<AsyncGroupChild, ExecutorError> {
        // Get the task to fetch its details
        let task = Task::find_by_id(pool, task_id)
            .await?
            .ok_or(ExecutorError::TaskNotFound)?;

        // Ensure PRPs directory structure exists
        let prps_dir = format!("{}/PRPs", worktree_path);
        let generated_dir = format!("{}/generated", prps_dir);
        let task_dir = format!("{}/task-{}", generated_dir, task_id);

        // Create the PRP generation command using the Python script
        let (shell_cmd, shell_arg) = get_shell_command();
        let prp_script = format!(
            r#"python3 backend/scripts/prp_generator.py --task-id {} --project-id {} --title '{}' --description '{}' --output-dir '{}' --worktree-path '{}'"#,
            task_id,
            task.project_id,
            task.title.replace("'", "\\'"),
            task.description.as_deref().unwrap_or("No description provided").replace("'", "\\'"),
            task_dir,
            worktree_path
        );

        let mut command = Command::new(shell_cmd);
        command
            .kill_on_drop(true)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .current_dir(worktree_path)
            .arg(shell_arg)
            .arg(&prp_script);

        let child = command
            .group_spawn()
            .map_err(|e| {
                crate::executor::SpawnContext::from_command(&command, "PRP")
                    .with_task(task_id, Some(task.title.clone()))
                    .with_context("PRP generation script execution")
                    .spawn_error(e)
            })?;

        Ok(child)
    }

    fn normalize_logs(
        &self,
        logs: &str,
        _worktree_path: &str,
    ) -> Result<NormalizedConversation, String> {
        let mut entries = Vec::new();
        
        for line in logs.lines() {
            if line.trim().is_empty() {
                continue;
            }
            
            // Handle PRP-specific log markers
            if line.starts_with("[PRP]") {
                entries.push(NormalizedEntry {
                    timestamp: None,
                    entry_type: NormalizedEntryType::SystemMessage,
                    content: line.trim_start_matches("[PRP]").trim().to_string(),
                    metadata: None,
                });
            } else if line.contains("PRP generation") {
                entries.push(NormalizedEntry {
                    timestamp: None,
                    entry_type: NormalizedEntryType::AssistantMessage,
                    content: line.to_string(),
                    metadata: None,
                });
            } else {
                // Regular output line
                entries.push(NormalizedEntry {
                    timestamp: None,
                    entry_type: NormalizedEntryType::SystemMessage,
                    content: line.to_string(),
                    metadata: None,
                });
            }
        }
        
        Ok(NormalizedConversation {
            entries,
            session_id: None,
            executor_type: "prp".to_string(),
            prompt: None,
            summary: Some("PRP generation completed".to_string()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_prp_executor_log_normalization() {
        let executor = PrpExecutor;
        let logs = "[PRP] Starting generation\nGenerating task analysis...\n[PRP] Complete";
        
        let result = executor.normalize_logs(logs, "/tmp/test").unwrap();
        assert_eq!(result.executor_type, "prp");
        assert_eq!(result.entries.len(), 3);
        assert_eq!(result.entries[0].content, "Starting generation");
        
        // Test that we can process PRP-specific log markers
        match &result.entries[0].entry_type {
            NormalizedEntryType::SystemMessage => {
                // Expected for PRP log markers
            }
            _ => panic!("Expected SystemMessage for PRP log marker"),
        }
    }
}
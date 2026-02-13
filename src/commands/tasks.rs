use crate::cli::TasksAction;
use crate::client::CraftClient;
use crate::error::{CliError, Result};
use serde_json::{json, Value};

pub async fn handle(client: &CraftClient, action: TasksAction) -> Result<()> {
    match action {
        TasksAction::List { scope } => {
            let tasks = client.list_tasks(&scope).await?;
            println!("{}", serde_json::to_string_pretty(&tasks)?);
            Ok(())
        }
        TasksAction::Add {
            text,
            schedule,
            deadline,
            to,
            date,
        } => {
            let mut task_info = json!({});

            if let Some(schedule) = schedule {
                task_info["scheduleDate"] = json!(schedule);
            }

            if let Some(deadline) = deadline {
                task_info["deadlineDate"] = json!(deadline);
            }

            let location = match to.as_str() {
                "daily" => {
                    let d = date.ok_or_else(|| {
                        CliError::Argument("--date required when --to is daily".to_string())
                    })?;
                    json!({"type": "dailyNote", "date": d})
                }
                _ => json!({"type": "inbox"}),
            };

            let tasks = vec![json!({
                "markdown": text,
                "taskInfo": task_info,
                "location": location
            })];

            let result = client.add_tasks(tasks).await?;
            println!("{}", serde_json::to_string_pretty(&result)?);
            Ok(())
        }
        TasksAction::Update {
            id,
            text,
            schedule,
            deadline,
            state,
        } => {
            let mut update = json!({"id": id});

            if let Some(text) = text {
                update["markdown"] = json!(text);
            }

            let mut task_info = json!({});

            if let Some(schedule) = schedule {
                task_info["scheduleDate"] = json!(schedule);
            }

            if let Some(deadline) = deadline {
                task_info["deadlineDate"] = json!(deadline);
            }

            if let Some(state) = state {
                task_info["state"] = json!(state);
            }

            if task_info.as_object().map(|o| !o.is_empty()).unwrap_or(false) {
                update["taskInfo"] = task_info;
            }

            let tasks = vec![update];
            let result = client.update_tasks(tasks).await?;
            println!("{}", serde_json::to_string_pretty(&result)?);
            Ok(())
        }
        TasksAction::Done { ids } => {
            let tasks: Vec<Value> = ids
                .into_iter()
                .map(|id| {
                    json!({
                        "id": id,
                        "taskInfo": {"state": "done"}
                    })
                })
                .collect();

            let result = client.update_tasks(tasks).await?;
            println!("{}", serde_json::to_string_pretty(&result)?);
            Ok(())
        }
        TasksAction::Delete { ids } => {
            let result = client.delete_tasks(ids).await?;
            println!("{}", serde_json::to_string_pretty(&result)?);
            Ok(())
        }
    }
}

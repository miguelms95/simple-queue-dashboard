use aws_config::{BehaviorVersion, Region};
use aws_credential_types::Credentials;
use aws_sdk_sqs::{Client, types::QueueAttributeName};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::State;

#[derive(Default)]
struct SqsState {
    client: Mutex<Option<Client>>,
    endpoint: Mutex<Option<String>>,
    region: Mutex<Option<String>>,
}

#[derive(Serialize, Deserialize)]
struct QueueInfo {
    name: String,
    url: String,
}

#[derive(Serialize, Deserialize)]
struct Message {
    id: String,
    body: String,
    receipt_handle: Option<String>,
}

#[tauri::command]
async fn configure_sqs(
    endpoint: String,
    region: String,
    state: State<'_, SqsState>,
) -> Result<String, String> {
    // Create dummy credentials for LocalStack
    let credentials = Credentials::new(
        "test",
        "test",
        None,
        None,
        "static"
    );

    let config = aws_config::defaults(BehaviorVersion::latest())
        .endpoint_url(&endpoint)
        .region(Region::new(region.clone()))
        .credentials_provider(credentials)
        .load()
        .await;

    let client = Client::new(&config);

    *state.client.lock().unwrap() = Some(client);
    *state.endpoint.lock().unwrap() = Some(endpoint);
    *state.region.lock().unwrap() = Some(region);

    Ok("SQS configured successfully".to_string())
}

#[tauri::command]
async fn list_queues(state: State<'_, SqsState>) -> Result<Vec<QueueInfo>, String> {
    let client = state.client.lock().unwrap().clone();
    let client = client.ok_or("SQS not configured")?;

    let resp = client
        .list_queues()
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let mut queues = Vec::new();
    for url in resp.queue_urls() {
        let name = url.split('/').last().unwrap_or(url).to_string();
        queues.push(QueueInfo {
            name,
            url: url.to_string(),
        });
    }

    Ok(queues)
}

#[tauri::command]
async fn create_queue(
    queue_name: String,
    state: State<'_, SqsState>,
) -> Result<String, String> {
    let client = state.client.lock().unwrap().clone();
    let client = client.ok_or("SQS not configured")?;

    let resp = client
        .create_queue()
        .queue_name(&queue_name)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    Ok(resp.queue_url().unwrap_or("").to_string())
}

#[tauri::command]
async fn send_message(
    queue_url: String,
    message_body: String,
    state: State<'_, SqsState>,
) -> Result<String, String> {
    let client = state.client.lock().unwrap().clone();
    let client = client.ok_or("SQS not configured")?;

    let resp = client
        .send_message()
        .queue_url(&queue_url)
        .message_body(&message_body)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    Ok(resp.message_id().unwrap_or("").to_string())
}

#[tauri::command]
async fn receive_messages(
    queue_url: String,
    max_messages: i32,
    state: State<'_, SqsState>,
) -> Result<Vec<Message>, String> {
    let client = state.client.lock().unwrap().clone();
    let client = client.ok_or("SQS not configured")?;

    let resp = client
        .receive_message()
        .queue_url(&queue_url)
        .max_number_of_messages(max_messages)
        .wait_time_seconds(5)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let mut messages = Vec::new();
    for msg in resp.messages() {
        messages.push(Message {
            id: msg.message_id().unwrap_or("").to_string(),
            body: msg.body().unwrap_or("").to_string(),
            receipt_handle: msg.receipt_handle().map(|s: &str| s.to_string()),
        });
    }

    Ok(messages)
}

#[tauri::command]
async fn delete_message(
    queue_url: String,
    receipt_handle: String,
    state: State<'_, SqsState>,
) -> Result<(), String> {
    let client = state.client.lock().unwrap().clone();
    let client = client.ok_or("SQS not configured")?;

    client
        .delete_message()
        .queue_url(&queue_url)
        .receipt_handle(&receipt_handle)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(SqsState::default())
        .invoke_handler(tauri::generate_handler![
            configure_sqs,
            list_queues,
            create_queue,
            send_message,
            receive_messages,
            delete_message
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

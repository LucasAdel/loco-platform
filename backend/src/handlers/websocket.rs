use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::Response,
};
use futures_util::{sink::SinkExt, stream::StreamExt};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tokio::sync::broadcast;
use tracing::{error, info, warn};
use uuid::Uuid;

use crate::AppState;

// WebSocket message types for real-time communication
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum WsMessage {
    // Job-related messages
    JobPosted { job: JobNotification },
    JobUpdated { job: JobNotification },
    JobDeleted { job_id: String },
    JobApplicationReceived { job_id: String, application_count: u32 },
    
    // System messages
    SystemNotification { message: String, level: NotificationLevel },
    UserConnected { user_id: String },
    UserDisconnected { user_id: String },
    
    // Client messages
    Subscribe { topics: Vec<String> },
    Unsubscribe { topics: Vec<String> },
    Ping,
    Pong,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobNotification {
    pub id: String,
    pub title: String,
    pub company: Option<String>,
    pub location: Option<String>,
    pub job_type: Option<String>,
    pub salary_range_start: Option<i32>,
    pub salary_range_end: Option<i32>,
    pub is_urgent: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotificationLevel {
    Info,
    Success,
    Warning,
    Error,
}

// Global WebSocket connection manager
pub type ConnectionManager = Arc<Mutex<HashMap<Uuid, broadcast::Sender<WsMessage>>>>;

lazy_static::lazy_static! {
    pub static ref CONNECTIONS: ConnectionManager = Arc::new(Mutex::new(HashMap::new()));
}

// WebSocket upgrade handler
pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(_state): State<AppState>,
) -> Response {
    info!("WebSocket connection requested");
    ws.on_upgrade(handle_socket)
}

// Demo WebSocket upgrade handler for demo mode
pub async fn demo_websocket_handler(ws: WebSocketUpgrade) -> Response {
    info!("Demo WebSocket connection requested");
    ws.on_upgrade(handle_demo_socket)
}

// Handle individual WebSocket connections
async fn handle_socket(socket: WebSocket) {
    let connection_id = Uuid::new_v4();
    info!("New WebSocket connection established: {}", connection_id);

    let (mut sender, mut receiver) = socket.split();
    let (tx, mut rx) = broadcast::channel::<WsMessage>(100);

    // Store connection in global manager
    {
        let mut connections = CONNECTIONS.lock().unwrap();
        connections.insert(connection_id, tx.clone());
    }

    // Send welcome message
    let welcome_msg = WsMessage::SystemNotification {
        message: "Connected to Loco Platform real-time feed".to_string(),
        level: NotificationLevel::Success,
    };
    
    if let Ok(msg_json) = serde_json::to_string(&welcome_msg) {
        if sender.send(Message::Text(msg_json)).await.is_err() {
            warn!("Failed to send welcome message to {}", connection_id);
        }
    }

    // Spawn task to handle outgoing messages
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if let Ok(msg_json) = serde_json::to_string(&msg) {
                if sender.send(Message::Text(msg_json)).await.is_err() {
                    break;
                }
            }
        }
    });

    // Handle incoming messages
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            match msg {
                Message::Text(text) => {
                    if let Ok(ws_msg) = serde_json::from_str::<WsMessage>(&text) {
                        handle_client_message(connection_id, ws_msg, &tx).await;
                    }
                }
                Message::Binary(_) => {
                    warn!("Received binary message, not supported");
                }
                Message::Close(_) => {
                    info!("WebSocket connection closed by client: {}", connection_id);
                    break;
                }
                _ => {}
            }
        }
    });

    // Wait for either task to complete
    tokio::select! {
        _ = (&mut send_task) => {
            recv_task.abort();
        },
        _ = (&mut recv_task) => {
            send_task.abort();
        },
    }

    // Clean up connection
    {
        let mut connections = CONNECTIONS.lock().unwrap();
        connections.remove(&connection_id);
    }
    
    info!("WebSocket connection cleaned up: {}", connection_id);
}

// Handle demo WebSocket connections (simplified)
async fn handle_demo_socket(socket: WebSocket) {
    let connection_id = Uuid::new_v4();
    info!("New demo WebSocket connection: {}", connection_id);

    let (mut sender, mut receiver) = socket.split();

    // Send welcome message
    let welcome_msg = WsMessage::SystemNotification {
        message: "Connected to Loco Platform Demo (real-time simulation)".to_string(),
        level: NotificationLevel::Info,
    };
    
    if let Ok(msg_json) = serde_json::to_string(&welcome_msg) {
        if sender.send(Message::Text(msg_json)).await.is_err() {
            return;
        }
    }

    // Simulate real-time job updates in demo mode
    let mut demo_task = tokio::spawn(async move {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(10));
        let mut counter = 1;
        
        loop {
            interval.tick().await;
            
            let demo_job = JobNotification {
                id: format!("demo-job-{}", counter),
                title: format!("Demo Pharmacist Position #{}", counter),
                company: Some("Demo Pharmacy Group".to_string()),
                location: Some("Sydney, NSW".to_string()),
                job_type: Some("FullTime".to_string()),
                salary_range_start: Some(90000 + (counter * 1000)),
                salary_range_end: Some(110000 + (counter * 1000)),
                is_urgent: counter % 3 == 0,
                created_at: chrono::Utc::now(),
            };

            let demo_msg = if counter % 2 == 0 {
                WsMessage::JobPosted { job: demo_job }
            } else {
                WsMessage::JobApplicationReceived { 
                    job_id: format!("demo-job-{}", counter - 1),
                    application_count: (counter * 2) as u32,
                }
            };

            if let Ok(msg_json) = serde_json::to_string(&demo_msg) {
                if sender.send(Message::Text(msg_json)).await.is_err() {
                    break;
                }
            }
            
            counter += 1;
            if counter > 10 {
                counter = 1;
            }
        }
    });

    // Handle incoming messages (simplified for demo)
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            match msg {
                Message::Text(text) => {
                    if let Ok(ws_msg) = serde_json::from_str::<WsMessage>(&text) {
                        if matches!(ws_msg, WsMessage::Ping) {
                            // Respond to ping with pong (would need sender access)
                            info!("Received ping from demo client");
                        }
                    }
                }
                Message::Close(_) => {
                    info!("Demo WebSocket connection closed: {}", connection_id);
                    break;
                }
                _ => {}
            }
        }
    });

    // Wait for either task to complete
    tokio::select! {
        _ = (&mut demo_task) => {
            recv_task.abort();
        },
        _ = (&mut recv_task) => {
            demo_task.abort();
        },
    }

    info!("Demo WebSocket connection cleaned up: {}", connection_id);
}

// Handle messages from clients
async fn handle_client_message(
    connection_id: Uuid,
    message: WsMessage,
    sender: &broadcast::Sender<WsMessage>,
) {
    match message {
        WsMessage::Subscribe { topics } => {
            info!("Client {} subscribed to topics: {:?}", connection_id, topics);
            
            // Send confirmation
            let confirmation = WsMessage::SystemNotification {
                message: format!("Subscribed to {} topics", topics.len()),
                level: NotificationLevel::Success,
            };
            
            if sender.send(confirmation).is_err() {
                error!("Failed to send subscription confirmation to {}", connection_id);
            }
        }
        WsMessage::Unsubscribe { topics } => {
            info!("Client {} unsubscribed from topics: {:?}", connection_id, topics);
        }
        WsMessage::Ping => {
            if sender.send(WsMessage::Pong).is_err() {
                error!("Failed to send pong to {}", connection_id);
            }
        }
        _ => {
            warn!("Received unexpected message type from client {}", connection_id);
        }
    }
}

// Broadcast message to all connected clients
pub async fn broadcast_to_all(message: WsMessage) {
    let connections = CONNECTIONS.lock().unwrap();
    let connection_count = connections.len();
    
    if connection_count == 0 {
        return;
    }

    info!("Broadcasting message to {} connections", connection_count);
    
    for (connection_id, sender) in connections.iter() {
        if sender.send(message.clone()).is_err() {
            warn!("Failed to broadcast to connection {}", connection_id);
        }
    }
}

// Broadcast job-related events
pub async fn broadcast_job_posted(job: JobNotification) {
    let message = WsMessage::JobPosted { job };
    broadcast_to_all(message).await;
}

pub async fn broadcast_job_updated(job: JobNotification) {
    let message = WsMessage::JobUpdated { job };
    broadcast_to_all(message).await;
}

pub async fn broadcast_job_deleted(job_id: String) {
    let message = WsMessage::JobDeleted { job_id };
    broadcast_to_all(message).await;
}

pub async fn broadcast_application_received(job_id: String, application_count: u32) {
    let message = WsMessage::JobApplicationReceived {
        job_id,
        application_count,
    };
    broadcast_to_all(message).await;
}

// System notification broadcasting
pub async fn broadcast_system_notification(message: String, level: NotificationLevel) {
    let ws_message = WsMessage::SystemNotification { message, level };
    broadcast_to_all(ws_message).await;
}
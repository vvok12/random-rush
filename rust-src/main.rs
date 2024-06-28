use std::sync::Arc;
use axum::{
    extract::{
        ws::{Message, WebSocket}, 
        State, 
        WebSocketUpgrade
    },
    response::{Html, Response}, 
    routing::get, 
    Router
};
use tokio::sync::{broadcast::{self, Receiver, Sender}, Mutex};
use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};

mod api;

#[derive(Debug, Clone)]
struct AppState {
    broadcast_tx: Arc<Mutex<Sender<Message>>>,
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let (tx, _) = broadcast::channel(32);
    let app = AppState {
        broadcast_tx: Arc::new(Mutex::new(tx)),
    };
    
    let app = Router::new()
        .route("/", get(|| async {  Html(include_str!("index.html")) }))
        .route("/playroom-ws", get(handler)).with_state(app);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handler(
    ws: WebSocketUpgrade, 
    State(app): State<AppState>,
) -> Response {
    ws.on_upgrade(|socket| handle_socket(socket, app))
}

async fn handle_socket(
    mut ws: WebSocket, 
    app: AppState,
) {
    let (ws_tx, ws_rx) = ws.split();
    let ws_tx = Arc::new(Mutex::new(ws_tx));

    {
        let broadcast_rx = app.broadcast_tx.lock().await.subscribe();
        tokio::spawn(async move {
            recv_broadcast(ws_tx, broadcast_rx).await;
        });
    }

    recv_from_client(ws_rx, app.broadcast_tx).await;
}

async fn recv_broadcast(
    client_tx: Arc<Mutex<SplitSink<WebSocket, Message>>>,
    mut broadcast_rx: Receiver<Message>,
) {
    while let Ok(msg) = broadcast_rx.recv().await {
        if client_tx.lock().await.send(msg).await.is_err() {
            return; // disconnected.
        }
    }
}

async fn recv_from_client(
    mut client_rx: SplitStream<WebSocket>,
    broadcast_tx: Arc<Mutex<Sender<Message>>>,
) {
    while let Some(Ok(msg)) = client_rx.next().await {
        if matches!(msg, Message::Close(_)) {
            return;
        }
        if broadcast_tx.lock().await.send(msg).is_err() {
            println!("Failed to broadcast a message");
        }
    }
}

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
use processor::PlayroomProcessingCtx;
use tokio::sync::{broadcast::{self, Receiver, Sender}, Mutex};
use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use uuid::Uuid;

mod api;
mod processor;

#[derive(Debug, Clone)]
struct AppState {
    input_tx: Arc<Mutex<Sender<PlayroomProcessingCtx>>>,
    output_tx: Arc<Mutex<Sender<PlayroomProcessingCtx>>>,
    playroom_id: Uuid,
}

#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() {
    let (processor_tx, _) = broadcast::channel(32);
    let (output_tx, _) = broadcast::channel::<PlayroomProcessingCtx>(32);
    let app = AppState {
        input_tx: Arc::new(Mutex::new(processor_tx)),
        output_tx: Arc::new(Mutex::new(output_tx)),
        playroom_id: Uuid::new_v4(),
    };
    
    let processor_rx = app.input_tx.lock().await.subscribe();
    let output_tx = app.output_tx.clone();
    tokio::spawn(async move {
        processor::processor(processor_rx, output_tx).await
    });

    let app = Router::new()
        .route("/", get(|| async {  Html(include_str!("index.html")) }))
        .route("/socket-test", get(|| async {  Html(include_str!("socket-test.html")) }))
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
    ws: WebSocket, 
    app: AppState,
) {
    let (ws_tx, ws_rx) = ws.split();
    let ws_tx = Arc::new(Mutex::new(ws_tx));
    let ws_bounce_tx = ws_tx.clone();
    let ws_id = Uuid::new_v4();

    {
        let playroom_rx = app.output_tx.lock().await.subscribe();
        tokio::spawn(async move {
            recv_playroom_output(ws_id, ws_tx, playroom_rx).await;
        });
    }

    recv_client_input(ws_id, ws_bounce_tx, ws_rx, app.input_tx, app.playroom_id).await;
}

async fn recv_playroom_output(
    websocket_id: Uuid,
    client_tx: Arc<Mutex<SplitSink<WebSocket, Message>>>,
    mut playroom_rx: Receiver<PlayroomProcessingCtx>,
) {
    while let Ok(ctx) = playroom_rx.recv().await {
        if ctx.output_socket_id == None {
            println!("recived message without destination {:?}", ctx);
            continue;
        }

        if ctx.raw_output == None {
            println!("recived message without output {:?}", ctx);
            continue;
        }
        
        if Some(websocket_id) != ctx.output_socket_id
        {
            continue;
        }
        
        let msg = Message::Text(ctx.raw_output.unwrap());
        if client_tx.lock().await.send(msg).await.is_err() {
            return; // disconnected.
        }
    }
}

async fn recv_client_input(
    websocket_id: Uuid,
    client_tx: Arc<Mutex<SplitSink<WebSocket, Message>>>,
    mut client_rx: SplitStream<WebSocket>,
    playroom_tx: Arc<Mutex<Sender<PlayroomProcessingCtx>>>,
    playroom_id: Uuid,
) {
    while let Some(Ok(msg)) = client_rx.next().await {
        if matches!(msg, Message::Close(_)) {
            return; // socket disconnected
        }

        if let Message::Text(m) = msg {
            let ctx = PlayroomProcessingCtx::new(m, websocket_id, playroom_id);
            let sender = playroom_tx.lock().await;
            let send_res = sender.send(ctx);
            send_res.unwrap();
            continue;
        }

        // return same message or drop connection
        if let Result::Err(_) = client_tx.lock().await.send(msg).await {
            return; 
        }
    }
}

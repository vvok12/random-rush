use std::sync::Arc;

use tokio::sync::{broadcast::{Receiver, Sender}, Mutex};
use uuid::Uuid;

use crate::api::ServerEvent;

#[derive(Debug, Clone)]
pub(crate) struct PlayroomProcessingCtx
{
    raw_input: String,
    pub raw_output: Option<String>,
    input_socket_id: Uuid,
    pub output_socket_id: Option<Uuid>,
}

impl PlayroomProcessingCtx 
{
    pub(crate) fn new(msg: String, socket_id: Uuid) -> Self {
        PlayroomProcessingCtx {
            raw_input: msg,
            raw_output: None,
            input_socket_id: socket_id,
            output_socket_id: None,
        }
    }
}

pub(crate) async fn processor(mut rx: Receiver<PlayroomProcessingCtx>, tx: Arc<Mutex<Sender<PlayroomProcessingCtx>>>) {
    loop {
        match rx.recv().await {
            Result::Ok(mut ctx) => {
                processor_impl(&mut ctx).await;
                let _ = tx.lock().await.send(ctx);
            },
            Result::Err(_) => println!("error while recieveing context")
        }
    }
}

async fn processor_impl(ctx: &mut PlayroomProcessingCtx) {
    ctx.output_socket_id = Some(ctx.input_socket_id);
    
    let serialized_res = serde_json::to_string(&ServerEvent::ServerError("not implemented just yet".to_string()));  
    if let Result::Ok(s) = serialized_res {
        ctx.raw_output = Some(s);
    } else {
        println!("processor_impl ran into serialization error: {:?}", ctx);
    }
}

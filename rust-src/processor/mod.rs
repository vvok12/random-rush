use std::sync::Arc;

use processing_error::ProcessingError;
use tokio::sync::{broadcast::{Receiver, Sender}, Mutex};
use uuid::Uuid;

use crate::api::{self, Board, BoardCell, ClientEvent, ServerEvent, UserId};
mod processing_error;

#[derive(Debug, Clone)]
pub(crate) struct PlayroomProcessingCtx
{
    playroom_id: Uuid,
    raw_input: String,
    pub raw_output: Option<String>,
    input_socket_id: Uuid,
    pub output_socket_id: Option<Uuid>,
}

impl PlayroomProcessingCtx 
{
    pub(crate) fn new(msg: String, socket_id: Uuid, playroom_id: Uuid) -> Self {
        PlayroomProcessingCtx {
            playroom_id: playroom_id,
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
    match processor_flow(ctx) {
        Ok(s) => ctx.raw_output = Some(s),
        Err(e) => println!("processor_impl ran into an error:{:?} {:?}", e, ctx)
    }
}

fn processor_flow(ctx: &mut PlayroomProcessingCtx) -> Result<String, ProcessingError>
{
    let ce = serde_json::from_str::<ClientEvent>(&ctx.raw_input)?;
    let se = process_event(ce, &ctx)?;
    let res = serde_json::to_string(&se)?;
    Ok(res)
}

fn process_event(client_event: ClientEvent, ctx: &PlayroomProcessingCtx) -> Result<ServerEvent, ProcessingError> {
    match client_event {
        ClientEvent::ReciveUserId => recive_user_id(ctx),
        ClientEvent::RecivePlayroomId => recive_playroom_id(ctx),
        ClientEvent::LoadBoard(board_id) => load_board(board_id, ctx),
        ClientEvent::LoadHand(_) => Err(ProcessingError::NotImplemented),
        ClientEvent::MakeMove(_) => Err(ProcessingError::NotImplemented),
    }
}

fn recive_user_id(ctx: &PlayroomProcessingCtx) -> Result<ServerEvent, ProcessingError> {
    Ok(ServerEvent::SendUserId(api::UserId(ctx.input_socket_id)))
}

fn recive_playroom_id(ctx: &PlayroomProcessingCtx) -> Result<ServerEvent, ProcessingError> {
    Ok(ServerEvent::SendPlayroomId(api::PlayroomId(api::BoardId(ctx.playroom_id))))
}

fn load_board(board_id: api::BoardId, ctx: &PlayroomProcessingCtx) -> Result<ServerEvent, ProcessingError> {
    let user_id = UserId(ctx.input_socket_id);
    Ok(ServerEvent::SendBoard(Board {
        id: board_id,
        users_connected: vec![user_id],
        cells: [BoardCell::default(),BoardCell::default(),BoardCell::default(),BoardCell::default(),BoardCell::default(),BoardCell::default(),BoardCell::default(),BoardCell::default(),BoardCell::default(),BoardCell::default()],
    }))
}

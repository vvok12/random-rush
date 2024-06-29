use ts_rs::TS;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(TS)]
#[ts(export)]
#[derive(Serialize, Deserialize)]
pub struct UserId(pub Uuid);

#[derive(TS)]
#[ts(export)]
#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Copy)]
pub struct UnitId(Colour);

#[derive(TS)]
#[ts(export)]
#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Colour {
    Red,
    Green,
    Blue,
    Yellow, 
    Violet,
}

#[derive(TS)]
#[ts(export)]
#[derive(Serialize, Deserialize)]
pub enum ActionTarget {
    Any,
    Last,
    Unit(UnitId),
}

#[derive(TS)]
#[ts(export)]
#[derive(Serialize, Deserialize)]
pub enum ActionEffect {
    MoveForward(u8),
    MoveBackward,
}

#[derive(TS)]
#[ts(export)]
#[derive(Serialize, Deserialize)]
pub struct Action
{
    target: ActionTarget, 
    effect: ActionEffect
}

#[derive(TS)]
#[ts(export)]
#[derive(Serialize, Deserialize)]
pub struct BoardId(pub Uuid);

#[derive(TS)]
#[ts(export)]
#[derive(Serialize, Deserialize)]
pub struct PlayroomId(pub BoardId);

#[derive(TS)]
#[ts(export)]
#[derive(Serialize, Deserialize)]
pub struct Board {
    pub id: BoardId,
    pub users_connected: Vec<UserId>,
    pub cells: [BoardCell; 10]
}

#[derive(TS)]
#[ts(export)]
#[derive(Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct BoardCell
{
    pub units: Vec<UnitId>,
}

#[derive(TS)]
#[ts(export)]
#[derive(Serialize, Deserialize)]
pub struct UserMove
{
    user: UserHandId,
    action: Action,
}

#[derive(TS)]
#[ts(export)]
#[derive(Serialize, Deserialize)]
pub struct UserHandId
{
    user: UserId,
    board: BoardId,
}

#[derive(TS)]
#[ts(export)]
#[derive(Serialize, Deserialize)]
pub struct UserHand
{
    id: UserHandId,
    available_actions: Vec<Action>,
}

#[derive(TS)]
#[ts(export)]
#[derive(Serialize, Deserialize)]
#[serde(tag = "event", content="data")]
pub enum ClientEvent {
    ReciveUserId,
    RecivePlayroomId,
    LoadBoard(BoardId),
    LoadHand(UserHandId),
    MakeMove(UserMove),
}

#[derive(TS)]
#[ts(export)]
#[derive(Serialize, Deserialize)]
#[serde(tag = "event", content = "data")]
pub enum ServerEvent {
    SendUserId(UserId),
    SendPlayroomId(PlayroomId),
    SendBoard(Board),
    SendUserHand(UserHand),
    ConfirmMove(UserMove),
    ServerError(String)
}
use ts_rs::TS;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(TS)]
#[ts(export)]
#[derive(Serialize, Deserialize)]
pub struct UserId(Uuid);

#[derive(TS)]
#[ts(export)]
#[derive(Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Unit {
    id: Colour
}

#[derive(TS)]
#[ts(export)]
#[derive(Serialize, Deserialize, PartialEq, Eq, Hash)]
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
    #[serde(untagged)]
    Exact(Unit),
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
pub struct BoardId(Uuid);

#[derive(TS)]
#[ts(export)]
#[derive(Serialize, Deserialize)]
pub struct Board {
    id: BoardId,
    users_connected: Vec<UserId>,
    cells: [BoardCell; 10]
}

#[derive(TS)]
#[ts(export)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BoardCell
{
    units_here: Vec<Unit>,
}

#[derive(TS)]
#[ts(export)]
#[derive(Serialize, Deserialize)]
pub enum ClientEvent {
    ReciveUserId,
    LoadBoard(BoardId),
    LoadHand(UserId),
    MakeMove(UserMove),
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
pub enum ServerEvent {
    SendUserId(UserId),
    SendBoard(Board),
    SendUserHand(UserHand),
    ConfirmMove(UserMove),
    Error(String)
}
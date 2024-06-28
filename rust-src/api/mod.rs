use ts_rs::TS;
use serde::{Serialize, Deserialize};

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
pub struct Board {
    cells: [BoardCell; 10]
}

#[derive(TS)]
#[ts(export)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BoardCell
{
    units_here: Vec<Unit>
}
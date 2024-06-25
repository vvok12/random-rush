use std::collections::HashMap;

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
pub enum CardTarget {
    Any,
    Last,
    #[serde(untagged)]
    Exact(Unit),
}

#[derive(TS)]
#[ts(export)]
#[derive(Serialize, Deserialize)]
pub enum CardAction {
    MoveForward(u8),
    MoveBackward,
}

#[derive(TS)]
#[ts(export)]
#[derive(Serialize, Deserialize)]
pub struct Card
{
    target: CardTarget, 
    action: CardAction
}


#[derive(TS)]
#[ts(export)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Board {
    cells: Vec<BoardCell>,
    //unit_map: HashMap<Unit, BoardCell>
}

#[derive(TS)]
#[ts(export)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BoardCell
{
    units_here: Vec<Unit>
}
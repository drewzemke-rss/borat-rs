#![warn(clippy::all, clippy::pedantic, clippy::unwrap_used)]

use serde::Deserialize;

/// The status (open or closed) of a breakout room.
/// This reflects the physical status of the breakout room's door.
#[derive(Debug, Deserialize)]
pub enum BreakoutRoomStatus {
    #[serde(rename = "open")]
    Open,

    #[serde(rename = "closed")]
    Closed,
}

/// A representation of a breakout room, including its English name and
/// current status.
#[derive(Debug, Deserialize)]
pub struct BreakoutRoomInfo {
    pub name: String,
    pub status: BreakoutRoomStatus,
}

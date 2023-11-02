use serde::Deserialize;
use std::fmt::Display;

#[derive(Debug, Deserialize)]
pub enum BreakoutRoomStatus {
    #[serde(rename = "open")]
    Open,

    #[serde(rename = "closed")]
    Closed,
}

impl Display for BreakoutRoomStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            BreakoutRoomStatus::Open => "open",
            BreakoutRoomStatus::Closed => "closed",
        };
        s.fmt(f)
    }
}

#[derive(Debug, Deserialize)]
pub struct BreakoutRoomInfo {
    pub name: String,
    pub status: BreakoutRoomStatus,
}

impl Display for BreakoutRoomInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{}: {}", self.name, self.status);
        s.fmt(f)
    }
}

#[derive(Debug, Deserialize)]
pub struct BoratResponse(Vec<BreakoutRoomInfo>);

impl Display for BoratResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: String = self.0.iter().map(|s| format!("  {s}\n")).collect();

        s.fmt(f)
    }
}

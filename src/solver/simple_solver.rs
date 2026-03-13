use crate::network::network::{LineId, StationId, Transport};

pub struct SimpleSolverState {
    pub station: StationId,
    pub line: Option<LineId>,
    pub transport: Option<Transport>,
    pub direction_from: Option<StationId>,
    pub stops_on_line: u8,
}
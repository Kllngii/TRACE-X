use std::cmp::PartialEq;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct StationId(pub usize);

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct LineId(pub usize);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Transport {
    Bahn,
    Bus,
    Faehre,
}

#[derive(Debug)]
pub struct Station {
    pub id: StationId,
    pub name: String,
}

#[derive(Debug)]
pub struct Line {
    pub id: LineId,
    pub name: String,
}

#[derive(Debug)]
pub struct Edge {
    pub from: StationId,
    pub to: StationId,
    pub line: LineId,
    pub transport: Transport,
}

#[derive(Debug)]
pub struct Network {
    pub stations: Vec<Station>,
    pub lines: Vec<Line>,
    pub edges: Vec<Edge>,
    pub adjacency: Vec<Vec<StationId>>,
    pub station_lookup: HashMap<String, StationId>,
    pub line_lookup: HashMap<String, LineId>,
}

impl Network {
    pub fn new() -> Network {
        Self {
            stations: Vec::new(),
            lines: Vec::new(),
            edges: Vec::new(),
            adjacency: Vec::new(),
            station_lookup: HashMap::new(),
            line_lookup: HashMap::new(),
        }
    }

    pub fn get_or_create_station(&mut self, name: &str) -> StationId {
        //Wenn die Station schon existiert: vorhandene Kennung zurück
        if let Some(id) = self.station_lookup.get(name) {
            return *id;
        }

        //Bei einer neuen Station: Station bekommt nächste freie Id
        let id = StationId(self.stations.len());

        self.stations.push(Station {
            id,
            name: name.to_string(),
        });

        self.station_lookup.insert(name.to_string(), id);
        //erzeugt unter self.adjacency[from_id] einen Vec, damit self.adjacency[from_id.0].push(to_id); später Edges speichern kann
        self.adjacency.push(Vec::new());

        id
    }

    fn get_or_create_line(&mut self, name: &str) -> LineId {
        //Wenn die Linie schon existiert: vorhandene Kennung zurück
        if let Some(id) = self.line_lookup.get(name) {
            return *id;
        }

        //Bei neuer Linie: Linie bekommt nächste freie Id
        let id = LineId(self.lines.len());

        self.lines.push(Line {
            id,
            name: name.to_string(),
        });

        self.line_lookup.insert(name.to_string(), id);

        id
    }

    pub fn add_edge(&mut self, from: &str, to: &str, line: &str, transport: Transport) {
        let from_id = self.get_or_create_station(from);
        let to_id = self.get_or_create_station(to);
        let line_id = self.get_or_create_line(line);

        let edge_index = self.edges.len();

        self.edges.push(Edge {
            from: from_id,
            to: to_id,
            line: line_id,
            transport,
        });

        self.adjacency[from_id.0].push(to_id);
    }
}
impl PartialEq<StationId> for &StationId {
    fn eq(&self, other: &StationId) -> bool {
        self == other
    }
}

impl PartialEq<LineId> for &LineId {
    fn eq(&self, other: &LineId) -> bool {
        self == other
    }
}

impl Network {
    pub fn station_id_to_string(&self, id: &StationId) -> String {
        (&self.stations[id.0].name).into()
    }
    pub fn line_id_to_string(&self, id: &LineId) -> String {
        (&self.lines[id.0].name).into()
    }
}
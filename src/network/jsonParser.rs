use serde::Deserialize;
use crate::network::model::{Network, Transport};

#[derive(Deserialize)]
struct JsonNetworkInput {
    lines: Vec<JsonLineInput>,
}

#[derive(Deserialize)]
struct JsonLineInput {
    name: String,
    transport: String,
    stations: Vec<String>,
}


impl Network {
    pub fn from_lines_json(data: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let input: JsonNetworkInput = serde_json::from_str(data)?;

        let mut net = Network::new();

        for line in input.lines {
            let transport = match line.transport.as_str() {
                "bahn" => Transport::Bahn,
                "bus" => Transport::Bus,
                "faehre" => Transport::Faehre,
                other => {
                    return Err(format!("unknown transport type: {}", other).into());
                }
            };

            // Sicherstellen, dass alle Stationen existieren
            for station in &line.stations {
                net.get_or_create_station(station);
            }

            // Edges erzeugen
            for pair in line.stations.windows(2) {
                let from = &pair[0];
                let to = &pair[1];

                // Hinrichtung
                net.add_edge(from, to, &line.name, transport);

                // Rückrichtung
                net.add_edge(to, from, &line.name, transport);
            }
        }

        Ok(net)
    }
}
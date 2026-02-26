use log::error;
use crate::network::constraints::NetworkConstraints;
use crate::network::network::Network;

pub mod network;
pub mod solver;
pub mod test;

fn main() {
    let network_json = include_str!("../data/Minimal28U1U3Network.json");
    let constraints_json = include_str!("../data/ConstraintsHamburg2026.json");

    let network_result = Network::from_lines_json(network_json);
    let constraints_result: Result<NetworkConstraints, _> = serde_json::from_str(constraints_json);

    println!("{:#?}", network_result);
    println!("{:#?}", constraints_result);

    if let Err(error) = network_result {
        error!("Fehler beim Einlesen des Netzwerks: {}", error);
        return;
    }
    if let Err(error) = constraints_result {
        error!("Fehler beim Einlesen der Constraints: {}", error);
        return;
    }

    let constraints: NetworkConstraints = constraints_result.unwrap();
    let network = network_result.unwrap();
}

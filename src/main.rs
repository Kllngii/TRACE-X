use crate::network::model::Network;

pub mod network;
pub mod solver;

fn main() {
    let json = include_str!("../data/Minimal28U1U3Network.json");

    let network = Network::from_lines_json(json);

    println!("{:#?}", network);
}

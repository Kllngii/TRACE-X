use crate::network::constraints::NetworkConstraints;
use crate::network::network::{Network, StationId};

#[test]
fn test_parse_minimal_network() {
    let input = r#"{"lines":[{"name":"U1","transport":"bahn","stations":["Wandsbek-Gartenstadt","Alter Teichweg","Straßburger Straße","Wandsbek Markt"]},{"name":"U3","transport":"bahn","stations":["Wandsbek-Gartenstadt","Habichtstraße","Barmbek"]},{"name":"28","transport":"bahn","stations":["Wandsbek Markt","Straßburger Straße","Alter Teichweg","Lämmersieth (Nord)","Habichtstr. (Mitte)","Habichtstraße","Habichtsplatz"]}]}"#;
    let network_result = Network::from_lines_json(input);

    let network = network_result.unwrap();

    println!("{:?}", network);

    //let max_id = network.stations.iter().max_by(|x, y| x.id.cmp(&y.id)).unwrap().id;

    for station in &network.stations {
        println!("{:?}", station);
        assert_eq!(network.stations[station.id.0].name, station.name);
    }
}
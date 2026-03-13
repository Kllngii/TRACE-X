use serde::Deserialize;
use crate::network::network::Transport;

#[derive(Deserialize, Debug, Copy, Clone)]
pub struct NetworkConstraints {
    pub r_x_count: u16, //N - wie viele Mr. X sich durch das Netzwerk bewegen
    pub station_limit_train: u16, //X - Anzahl am Stück gefahrener Bahnstationen gleicher Linie und Fahrtrichtung
    pub station_limit_bus: u16, //Y - Anzahl am Stück gefahrener Busstationen gleicher Linie und Fahrtrichtung
    pub station_limit_ferry: u16, //Z - Anzahl am Stück gefahrener Fährstationen gleicher Linie und Fahrtrichtung
    pub direction_change_between_calls_permitted: bool, //Darf der Mr. X zwischen zwei Standortmeldungen die Fahrtrichtung wechseln?
    pub minutes_between_calls: u16, //A - Anzahl Minuten zwischen zwei Standortmeldungen
    pub minutes_before_recatch: u16, //B - Anzahl Minuten bis ein gefangener Mr. X erneut gefangen werden kann
    pub minutes_before_recatch_with_other_catch_between: u16, //K - Anzahl Minuten bis ein gefangener Mr. X erneut gefangen werden kann, wenn ein anderer Mr. X zwischendurch gefangen
}

impl NetworkConstraints {
    pub fn get_limit_by_transport(&self, t: Transport) -> u16 {
        match t {
            Transport::Bahn => self.station_limit_train,
            Transport::Bus => self.station_limit_bus,
            Transport::Faehre => self.station_limit_ferry,
        }
    }
}
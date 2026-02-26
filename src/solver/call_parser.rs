use std::cell::LazyCell;
use std::str::FromStr;
use regex::Regex;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CallInput {
    pub time: String,
    pub name: String,
    pub station: String,
    pub line: String,
    pub direction: String,
}

/**
 * Bei einer stationären Standortmeldung sind die hinteren beiden Blöcke mit ' ' oder mit '-' gefüllt
 */
const STATIONARY_REGEX: LazyCell<Regex> = LazyCell::new(|| {
    Regex::new(r"^(?P<time>\d{2}:\d{2})/(?P<name>Mr\.X\d)/(?P<station>[\w\s-]+)/(?P<line>[\s-]+)/(?P<direction>[\s-]+)$").unwrap()
});
/**
 *
 */
const VALID_REGEX: LazyCell<Regex> = LazyCell::new(|| {
    Regex::new(r"^(?P<time>\d{2}:\d{2})/(?P<name>Mr\.X\d)/(?P<station>[\w\s-]+)/(?P<line>[\w\s-]+)/(?P<direction>[\w\s-]+)$").unwrap()
});
impl CallInput {
    pub fn parse(line: &str) -> Result<Self, String> {
        let caps = STATIONARY_REGEX
            .captures(line)//Mit Stationärmeldung-Regex abgleichen
            .or_else(|| VALID_REGEX.captures(line))//Wenn der nicht matchen sollte zusätzlich gegen Regulärmeldung-Regex abgleichen
            .ok_or_else(|| "Fehler".to_string())?;

        Ok(Self {
            time: caps["time"].to_string(),
            name: caps["name"].to_string(),
            station: caps["station"].to_string(),
            //Stationärmeldungen mit einer Poison-Pill '!-!' versehen
            line: Self::poisonpill_stationary(&caps["line"]),
            direction: Self::poisonpill_stationary(&caps["direction"]),
        })
    }
    fn poisonpill_stationary(value: &str) -> String {
        if value == " " || value == "-" {
            "!-!".to_string()
        } else {
            value.to_string()
        }
    }
}
impl FromStr for CallInput {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}
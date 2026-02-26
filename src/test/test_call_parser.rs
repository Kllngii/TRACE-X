use std::hint::assert_unchecked;
use crate::solver::call_parser::CallInput;

#[test]
fn test_parse_valid_stationary_call() {
    let input = "10:00/Mr.X1/Ohlstedt/U1/Norderstedt Mitte";
    let parsed = CallInput::parse(input).expect("Sollte parsebar sein");

    assert_eq!(parsed.time, "10:00");
    assert_eq!(parsed.name, "Mr.X1");
    assert_eq!(parsed.station, "Ohlstedt");
    assert_eq!(parsed.line, "U1");
    assert_eq!(parsed.direction, "Norderstedt Mitte");
}
#[test]
fn test_parse_valid_stationary_call_dashed_names() {
    let input = "10:15/Mr.X2/Wandsbek-Gartenstadt/U1/Norderstedt Mitte";
    let parsed = CallInput::parse(input).expect("Sollte parsebar sein");

    assert_eq!(parsed.time, "10:15");
    assert_eq!(parsed.name, "Mr.X2");
    assert_eq!(parsed.station, "Wandsbek-Gartenstadt");
    assert_eq!(parsed.line, "U1");
    assert_eq!(parsed.direction, "Norderstedt Mitte");
}
#[test]
fn test_parse_stationary_line_with_spaces() {
    let input = "11:00/Mr.X3/Altona/ / ";
    let parsed = CallInput::parse(input).expect("Stationärmeldung sollte parsebar sein");

    assert_eq!(parsed.time, "11:00");
    assert_eq!(parsed.name, "Mr.X3");
    assert_eq!(parsed.station, "Altona");
    assert_eq!(parsed.line, "!-!");
    assert_eq!(parsed.direction, "!-!");
}
#[test]
fn test_parse_stationary_line_with_dashes() {
    let input = "11:15/Mr.X3/Altona/-/-";
    let parsed = CallInput::parse(input).expect("Stationärmeldung sollte parsebar sein");

    assert_eq!(parsed.time, "11:15");
    assert_eq!(parsed.name, "Mr.X3");
    assert_eq!(parsed.station, "Altona");
    assert_eq!(parsed.line, "!-!");
    assert_eq!(parsed.direction, "!-!");
}
#[test]
fn test_parse_invalid_line() {
    let input = "Das Spiel ist beendet, bitte gebt bis 14:30 Uhr eure Stempelkarten ab.";
    let result = CallInput::parse(input);
    assert!(result.is_err(), "Ungültige Zeile sollte Fehler zurückgeben");
}
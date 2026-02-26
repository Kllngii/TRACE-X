use crate::network::constraints::NetworkConstraints;

#[test]
fn test_parse_network_constraints() {
    let input = r#"{"r_x_count":5,"station_limit_train":5,"station_limit_bus":8,"station_limit_ferry":5,"direction_change_between_calls_permitted":false,"minutes_between_calls":15,"minutes_before_recatch":60,"minutes_before_recatch_with_other_catch_between":30}"#;
    let constraints_result: Result<NetworkConstraints, _> = serde_json::from_str(input);

    let constraints = constraints_result.unwrap();
    
    assert_eq!(constraints.r_x_count, 5);
    assert_eq!(constraints.station_limit_train, 5);
    assert_eq!(constraints.station_limit_bus, 8);
    assert_eq!(constraints.station_limit_ferry, 5);
    assert_eq!(constraints.direction_change_between_calls_permitted, false);
    assert_eq!(constraints.minutes_between_calls, 15);
    assert_eq!(constraints.minutes_before_recatch, 60);
    assert_eq!(constraints.minutes_before_recatch_with_other_catch_between, 30);
}
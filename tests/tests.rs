use chrono::NaiveDateTime;
use clisurf::{get_swell_data, SwellData, SwellDataError};
use mockito;

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::Server;

    #[test]
    fn test_swell_data_new() {
        let date_time =
            NaiveDateTime::parse_from_str("2023-08-28 12:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let swell_data = SwellData::new("46225".to_string(), date_time, 1.5, 10, 270, Some(20.0));

        assert_eq!(swell_data.station_id, "46225");
        assert_eq!(swell_data.wave_height, 1.5);
        assert_eq!(swell_data.wave_period, 10);
        assert_eq!(swell_data.wave_direction, 270);
        assert_eq!(swell_data.water_temp, Some(20.0));
    }

    #[test]
    fn test_swell_data_from_json_success() {
        let json = r#"{"stationId":"46225","year":"2023","month":"08","day":"28","hour":"12","minute":"00","waveHeight":"1.5","wavePeriod":"10","waveDirection":"270","waterTemp":"20.0"}"#;
        let result = SwellData::from_json(json);
        assert!(result.is_ok());

        let swell_data = result.unwrap();
        assert_eq!(swell_data.station_id, "46225");
        assert_eq!(swell_data.wave_height, 1.5);
        assert_eq!(swell_data.wave_period, 10);
        assert_eq!(swell_data.wave_direction, 270);
        assert_eq!(swell_data.water_temp, Some(20.0));
    }

    #[test]
    fn test_swell_data_from_json_invalid_data() {
        let json = r#"{"stationId":"46225","year":"2023","month":"08","day":"28","hour":"12","minute":"00","waveHeight":"invalid","wavePeriod":"10","waveDirection":"270","waterTemp":"20.0"}"#;
        let result = SwellData::from_json(json);
        assert!(result.is_ok()); // This is still ok because of unwrap_or(-1.0)

        let swell_data = result.unwrap();
        assert_eq!(swell_data.wave_height, -1.0);
    }

    #[test]
    fn test_get_swell_data_success() {
        let mut server = Server::new();
        let mock = server.mock("GET", "/swellData/46225")
            .with_status(200)
            .with_body(r#"{"stationId":"46225","year":"2023","month":"08","day":"28","hour":"12","minute":"00","waveHeight":"1.5","wavePeriod":"10","waveDirection":"270","waterTemp":"20.0"}"#)
            .create();

        let url = server.url();
        let result = get_swell_data(&format!("{}/swellData/46225", url));

        assert!(result.is_ok());
        mock.assert();
    }

    #[test]
    fn test_get_swell_data_not_found() {
        let mut server = Server::new();
        let mock = server
            .mock("GET", "/swellData/99999")
            .with_status(404)
            .create();

        let url = server.url();
        let result = get_swell_data(&format!("{}/swellData/99999", url));

        assert!(matches!(result, Err(SwellDataError::NotFound)));
        mock.assert();
    }

    #[test]
    fn test_get_swell_data_network_error() {
        let mut server = Server::new();
        let mock = server
            .mock("GET", "/swellData/46225")
            .with_status(500)
            .create();

        let url = server.url();
        let result = get_swell_data(&format!("{}/swellData/46225", url));

        assert!(matches!(result, Err(SwellDataError::NetworkError(_))));
        mock.assert();
    }
}

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::error::Error;
use ureq;

#[derive(Deserialize, Serialize, Debug)]
pub struct SwellData {
    pub station_id: String,
    pub date_time: NaiveDateTime,
    pub wave_height: f32,
    pub wave_period: i32,
    pub wave_direction: i32,
    pub water_temp: Option<f32>,
}

impl SwellData {
    fn new(
        station_id: String,
        date_time: NaiveDateTime,
        wave_height: f32,
        wave_period: i32,
        wave_direction: i32,
        water_temp: Option<f32>,
    ) -> Self {
        Self {
            station_id,
            date_time,
            wave_height,
            wave_period,
            wave_direction,
            water_temp,
        }
    }

    fn from_json(json: &str) -> Result<Self, Box<dyn Error>> {
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct RawSwellData {
            station_id: String,
            year: String,
            month: String,
            day: String,
            hour: String,
            minute: String,
            wave_height: String,
            wave_period: String,
            wave_direction: String,
            water_temp: Option<String>,
        }

        let raw: RawSwellData = serde_json::from_str(json)?;
        let date_str = format!(
            "{}-{}-{} {}:{}:00",
            raw.year, raw.month, raw.day, raw.hour, raw.minute
        );
        let date_time = NaiveDateTime::parse_from_str(&date_str, "%Y-%m-%d %H:%M:%S")?;
        Ok(Self::new(
            raw.station_id,
            date_time,
            raw.wave_height.parse().unwrap_or(-1.0),
            raw.wave_period.parse().unwrap_or(-1),
            raw.wave_direction.parse().unwrap_or(-1),
            raw.water_temp.and_then(|s| s.parse().ok()),
        ))
    }
}

pub fn get_swell_data(url: &str) -> Result<SwellData, Box<dyn Error>> {
    let response = ureq::get(url).call()?.into_string()?;
    let swell_data = SwellData::from_json(&response)?;
    Ok(swell_data)
}

pub fn convert_meters_to_feet(meters: f32) -> f32 {
    meters * 3.28084
}

pub fn convert_celsius_to_fahrenheit(celcius: f32) -> f32 {
    celcius * 9.0 / 5.0 + 32.0
}

pub fn convert_degrees_to_cardinal(degrees: i32) -> String {
    let directions = ["N", "NE", "E", "SE", "S", "SW", "W", "NW"];
    let index = ((degrees as f32 + 22.5) / 45.0).floor() as usize % 8;
    directions[index].to_string()
}

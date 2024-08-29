use clap::Parser;
use clisurf::get_swell_data;
use util::{convert_celsius_to_fahrenheit, convert_degrees_to_cardinal, convert_meters_to_feet};
mod util;

#[derive(Parser, Debug)]
#[command(
    author = "Rick Sutherland <suthe.rick@gmail.com>",
    version = "0.1.0",
    about = "A command-line swell data checker",
    long_about = "This program fetches and displays ocean conditions for a specified CDIP station ID. It can show wave height, wave period, wave direction, and water temperature in either metric or imperial units."
)]
struct Args {
    /// Station ID for swell data
    #[arg(default_value = "46225")]
    station_id: String,

    /// Display wave height
    #[arg(short = 'f', long)]
    wave_height: bool,

    /// Display wave period
    #[arg(short = 'p', long)]
    wave_period: bool,

    /// Display wave direction
    #[arg(short = 'd', long)]
    wave_direction: bool,

    /// Display water temperature
    #[arg(short = 't', long)]
    water_temp: bool,

    /// Use metric units (meters, Celsius)
    #[arg(short = 'm', long)]
    metric: bool,
}

fn main() {
    let args = Args::parse();
    let url = format!("https://api.swellbar.app/swellData/{}", args.station_id);

    let swell_data = get_swell_data(&url);

    match swell_data {
        Ok(data) => {
            let output = format_output(&args, &data);
            println!("{}", output);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            // exit with a non-zero status code
            std::process::exit(1);
        }
    }
}

fn format_output(args: &Args, data: &clisurf::SwellData) -> String {
    let mut output = Vec::new();
    let show_all =
        !(args.wave_height || args.wave_period || args.wave_direction || args.water_temp);

    if show_all || args.wave_height {
        let wave_height = if args.metric {
            data.wave_height
        } else {
            convert_meters_to_feet(data.wave_height)
        };
        if wave_height < 0.0 {
            output.push("Height: --".to_string());
        } else {
            output.push(format!(
                "Height: {:.1} {}",
                wave_height,
                if args.metric { "m" } else { "ft" }
            ));
        }
    }

    if show_all || args.wave_period {
        if data.wave_period < 0 {
            output.push("Period: --".to_string());
        } else {
            output.push(format!("Period: {}s", data.wave_period));
        }
    }

    if show_all || args.wave_direction {
        if data.wave_direction < 0 {
            output.push("Direction: --".to_string());
        } else {
            output.push(format!(
                "Direction: {}° {}",
                data.wave_direction,
                convert_degrees_to_cardinal(data.wave_direction)
            ))
        }
    }

    if show_all || args.water_temp {
        if let Some(temp) = data.water_temp {
            let temp_str = if args.metric {
                format!("{:.1}°C", temp)
            } else {
                format!("{:.1}°F", convert_celsius_to_fahrenheit(temp))
            };
            output.push(format!("Temp: {}", temp_str));
        } else {
            output.push("Temp: --".to_string());
        }
    }

    output.join("\n")
}

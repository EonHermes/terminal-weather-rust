//! Terminal Weather Station - Beautiful ASCII art weather visualizations in Rust
mod weather;
mod renderer;
mod visualizations;
mod effects;

use clap::Parser;
use colored::*;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

#[derive(Parser)]
#[command(name = "tws")]
#[command(author = "Daniel Lindestad")]
#[command(version = "0.1.0")]
#[command(about = "Terminal Weather Station - Beautiful ASCII art weather visualizations", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(clap::Subcommand)]
enum Commands {
    /// Display current weather with artistic visualizations
    Show {
        /// Location to get weather for (e.g., "London" or "40.7128,-74.0060")
        #[arg(default_value = "auto")]
        location: String,

        /// Display mode: full, minimalist, data, art
        #[arg(short, long, default_value = "full")]
        mode: String,

        /// Auto-refresh interval in seconds (0 to disable)
        #[arg(short, long, default_value = "0")]
        refresh: u64,

        /// Latitude coordinate
        #[arg(long)]
        lat: Option<f64>,

        /// Longitude coordinate
        #[arg(long)]
        lon: Option<f64>,
    },

    /// Show detailed weather information without visualizations
    Info {
        /// Location to get weather for
        #[arg(default_value = "auto")]
        location: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let weather_service = weather::WeatherService::new();
    let mut renderer = renderer::Renderer::new();

    match &cli.command {
        Some(Commands::Show {
            location,
            mode,
            refresh,
            lat,
            lon,
        }) => {
            renderer.set_mode(mode.as_str());

            // Get initial weather data
            let mut weather = if let (Some(lat_val), Some(lon_val)) = (lat, lon) {
                weather_service.get_weather(*lat_val, *lon_val, None).await?
            } else if location == "auto" {
                println!("{}", "Auto-detection not yet implemented. Using London as default.".yellow());
                weather_service.get_weather_for_location("London").await?
            } else if location.contains(',') {
                let coords: Vec<f64> = location.split(',').map(|s| s.parse().unwrap_or(0.0)).collect();
                if coords.len() != 2 {
                    eprintln!("{}", "Error: Invalid coordinate format".red());
                    std::process::exit(1);
                }
                weather_service.get_weather(coords[0], coords[1], None).await?
            } else {
                weather_service.get_weather_for_location(location).await?
            };

            // Main display loop for auto-refresh mode
            if *refresh > 0 {
                let interval = Duration::from_secs(*refresh);
                loop {
                    clear_screen();
                    
                    if lat.is_some() && lon.is_some() {
                        weather = weather_service.get_weather(lat.unwrap(), lon.unwrap(), None).await?;
                    } else if location != "auto" && !location.contains(',') {
                        weather = weather_service.get_weather_for_location(location).await?;
                    }

                    let output = renderer.render(&weather);
                    println!("{}", output);
                    
                    thread::sleep(interval);
                }
            } else {
                // Single display - wait for user input
                println!("{}", "\nPress [1-4] to change mode, [q] to quit".dimmed());
                
                let mut input = String::new();
                loop {
                    io::stdin().read_line(&mut input)?;
                    let cmd = input.trim().to_lowercase();
                    
                    match cmd.as_str() {
                        "1" => renderer.set_mode("full"),
                        "2" => renderer.set_mode("minimalist"),
                        "3" => renderer.set_mode("data"),
                        "4" => renderer.set_mode("art"),
                        "q" | "quit" | "exit" => {
                            clear_screen();
                            println!("{}", "\nWeather Station stopped. Stay dry! ☔".dimmed());
                            break;
                        }
                        _ => {}
                    }

                    input.clear();
                    
                    // Refresh data for non-coordinate modes
                    if lat.is_some() && lon.is_some() {
                        weather = weather_service.get_weather(lat.unwrap(), lon.unwrap(), None).await?;
                    } else if location != "auto" && !location.contains(',') {
                        weather = weather_service.get_weather_for_location(location).await?;
                    }

                    clear_screen();
                    let output = renderer.render(&weather);
                    println!("{}", output);
                }
            }
        }
        Some(Commands::Info { location }) => {
            let weather = if location == "auto" {
                println!("{}", "Auto-detection not yet implemented. Using London as default.".yellow());
                weather_service.get_weather_for_location("London").await?
            } else if location.contains(',') {
                let coords: Vec<f64> = location.split(',').map(|s| s.parse().unwrap_or(0.0)).collect();
                if coords.len() != 2 {
                    eprintln!("{}", "Error: Invalid coordinate format".red());
                    std::process::exit(1);
                }
                weather_service.get_weather(coords[0], coords[1], None).await?
            } else {
                weather_service.get_weather_for_location(location).await?
            };

            display_info(&weather);
        }
        None => {
            // Default: show help
            println!("{}", "No command specified. Use 'tws show' or 'tws info'.".yellow());
            println!("{}", "\nExamples:".dimmed());
            println!("  tws show              # Show weather with visualizations");
            println!("  tws show London       # Weather for London");
            println!("  tws show --mode art   # Artistic mode");
            println!("  tws info              # Detailed info without visuals");
        }
    }

    Ok(())
}

fn clear_screen() {
    print!("\x1b[2J\x1b[H");
    let _ = io::stdout().flush();
}

fn display_info(weather: &weather::WeatherData) {
    println!("\n{}", "◈ Weather Information".cyan().bold());
    println!("Location: {}, {}\n", weather.location, weather.country);

    println!("{}", "Temperature:".white().bold());
    println!("  Current: {}°C", (weather.temperature).round() as i32);
    println!("  Feels like: {}°C\n", (weather.feels_like).round() as i32);

    println!("{}", "Wind:".white().bold());
    let dir_name = get_wind_direction_name(weather.wind_direction);
    println!("  Speed: {} km/h", weather.wind_speed);
    println!("  Direction: {:.0}° ({})\n", weather.wind_direction, dir_name);

    println!("{}", "Sky:".white().bold());
    println!("  Condition: {}", weather.condition);
    println!("  Cloud cover: {:.0}%\n", weather.cloud_cover);

    if weather.precipitation > 0.0 {
        println!("{}", "Precipitation:".white().bold());
        println!("  Total: {:.1}mm", weather.precipitation);
        if weather.rain > 0.0 {
            println!("  Rain: {:.1}mm", weather.rain);
        }
        if weather.snowfall > 0.0 {
            println!("  Snow: {:.1}cm", weather.snowfall);
        }
    }
}

fn get_wind_direction_name(direction: f64) -> &'static str {
    let names = ["N", "NE", "E", "SE", "S", "SW", "W", "NW"];
    let idx = (((direction % 360.0) + 22.5) / 45.0).floor() as usize % 8;
    names[idx]
}

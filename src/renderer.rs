//! Main Renderer - Combines all weather visualizations

use crate::effects::color;
use crate::visualizations::{wind, precipitation, temperature, clouds};
use crate::weather::WeatherData;

pub struct Renderer {
    width: usize,
    height: usize,
    mode: String,
}

impl Renderer {
    pub fn new() -> Self {
        // Get terminal dimensions (fallback to 80x24)
        let (width, height) = get_terminal_size();
        
        Self {
            width,
            height,
            mode: "full".to_string(),
        }
    }

    /// Render complete weather visualization
    pub fn render(&self, weather: &WeatherData) -> String {
        let mut lines = Vec::new();

        // Header
        lines.push(self.render_header(weather));
        lines.push(String::new());

        match self.mode.as_str() {
            "full" => lines.extend(self.render_full_mode(weather)),
            "minimalist" => lines.extend(self.render_minimalist_mode(weather)),
            "data" => lines.extend(self.render_data_mode(weather)),
            "art" => lines.extend(self.render_art_mode(weather)),
            _ => lines.extend(self.render_full_mode(weather)),
        }

        // Footer with controls hint
        lines.push(String::new());
        lines.push(self.render_footer());

        lines.join("\n")
    }

    /// Render header section
    fn render_header(&self, weather: &WeatherData) -> String {
        let location = &weather.location;
        let country = if !weather.country.is_empty() {
            format!(", {}", weather.country)
        } else {
            String::new()
        };
        let time = chrono::Local::now().format("%H:%M:%S").to_string();

        format!(
            "◈ Weather Station {} [{}]",
            location.to_owned() + &country,
            time
        )
    }

    /// Render full immersion mode (all visualizations combined)
    fn render_full_mode(&self, weather: &WeatherData) -> Vec<String> {
        let mut lines = Vec::new();

        // Top section: Temperature with thermal background
        let temp_output = temperature::generate(60, 12, weather);
        lines.extend(temp_output.lines().map(String::from));

        lines.push(String::new());

        // Middle section: Split view - Wind left, Clouds right
        let mid_height = 8;
        
        // Wind visualization
        let wind_output = wind::generate(35, mid_height, weather);
        
        // Cloud visualization  
        let cloud_output = clouds::generate(35, mid_height, weather);

        // Combine side by side with separator
        let wind_lines: Vec<&str> = wind_output.lines().collect();
        let cloud_lines: Vec<&str> = cloud_output.lines().collect();
        
        for y in 0..mid_height {
            let wind_line = wind_lines.get(y).unwrap_or(&"").to_string();
            let cloud_line = cloud_lines.get(y).unwrap_or(&"").to_string();
            
            let combined = format!(
                "{} │ {}",
                pad_end(&wind_line, 35),
                cloud_line
            );
            lines.push(combined);
        }

        lines.push(String::new());

        // Bottom section: Precipitation if active, otherwise data summary
        if weather.precipitation > 0.0 {
            let precip_output = precipitation::generate(60, 8, weather);
            lines.extend(precip_output.lines().map(String::from));
        } else {
            // Data summary when no precipitation
            lines.push(self.render_data_summary(weather));
        }

        lines
    }

    /// Render minimalist mode (clean, subtle)
    fn render_minimalist_mode(&self, weather: &WeatherData) -> Vec<String> {
        let mut lines = Vec::new();

        // Simple temperature display
        let temp_color = color::colorize_temp(
            &format!("◈ {}°C", (weather.temperature).round() as i32),
            weather.temperature
        );
        lines.push(format!("  {} {}", temp_color, weather.condition));
        lines.push(String::new());

        // Compact data row
        let wind_dir = get_wind_direction_symbol(weather.wind_direction);
        lines.push(format!("  Wind: {} {} km/h", wind_dir, weather.wind_speed));
        
        if weather.cloud_cover > 5.0 {
            let cloud_icon = if weather.cloud_cover < 50.0 { "⛅" } else { "☁️" };
            lines.push(format!("  Sky: {} {:.0}%", cloud_icon, weather.cloud_cover));
        }

        if weather.precipitation > 0.0 {
            let rain_icon = if weather.is_snowing { "❄️" } else { "🌧️" };
            lines.push(format!("  Precip: {} {:.1}mm", rain_icon, weather.precipitation));
        }

        lines
    }

    /// Render data-focused mode (numbers prominent)
    fn render_data_mode(&self, weather: &WeatherData) -> Vec<String> {
        let mut lines = Vec::new();

        // Large temperature
        let temp_color = color::colorize_temp(
            &format!("◈ Temperature: {}°C", (weather.temperature).round() as i32),
            weather.temperature
        );
        lines.push(format!("  {}", temp_color));
        lines.push(format!("     Feels like: {}°C", (weather.feels_like).round() as i32));
        lines.push(String::new());

        // Data table
        let data = vec![
            ("Wind", format!("{} km/h", weather.wind_speed), 
             format!("from {:.0}° ({})", weather.wind_direction, get_wind_direction_name(weather.wind_direction))),
            ("Humidity", format!("{}%", weather.humidity), String::new()),
            ("Cloud Cover", format!("{:.0}%", weather.cloud_cover), get_cloud_status(weather.cloud_cover)),
            ("Precipitation", format!("{:.1}mm", weather.precipitation), 
             if weather.is_snowing { "(snow)".to_string() } else if weather.rain > 0.0 { "(rain)".to_string() } else { String::new() }),
        ];

        let max_label_len = data.iter().map(|(l, _, _)| l.len()).max().unwrap_or(0);
        
        for (label, value, note) in data {
            if !value.is_empty() || !note.is_empty() {
                let padded_label = format!("{:width$}", label, width = max_label_len);
                let line = format!(
                    "  {}: {}{}",
                    padded_label,
                    value,
                    if !note.is_empty() { format!(" ({})", note) } else { String::new() }
                );
                lines.push(line);
            }
        }

        lines
    }

    /// Render art mode (maximum visual expression)
    fn render_art_mode(&self, weather: &WeatherData) -> Vec<String> {
        let mut lines = Vec::new();

        // Artistic temperature gradient bar
        let gradient = color::create_temp_gradient(-10.0, 35.0, 20);
        lines.push(format!("  {}", gradient));
        lines.push(String::new());

        // Large artistic display combining all elements
        let temp_art = temperature::generate(68, 13, weather);
        let wind_art = wind::generate(68, 13, weather);
        
        // Merge the two visualizations (simplified overlay)
        let temp_lines: Vec<&str> = temp_art.lines().collect();
        let wind_lines: Vec<&str> = wind_art.lines().collect();
        
        for y in 0..13 {
            let t_line = temp_lines.get(y).unwrap_or(&"");
            let w_line = wind_lines.get(y).unwrap_or(&"");
            
            // Simple overlay - prefer non-space characters from wind
            let mut merged = String::new();
            for (t_ch, w_ch) in t_line.chars().zip(w_line.chars()).take(68) {
                if w_ch != ' ' && !w_ch.is_whitespace() {
                    merged.push(w_ch);
                } else {
                    merged.push(t_ch);
                }
            }
            lines.push(merged);
        }

        // Artistic summary at bottom
        lines.push(String::new());
        let summary = format!("  {}", get_weather_poem(weather));
        lines.push(summary);

        lines
    }

    /// Render data summary (compact)
    fn render_data_summary(&self, weather: &WeatherData) -> String {
        let items = vec![
            format!("◈ {}°C", (weather.temperature).round() as i32),
            format!("{} {} km/h", get_wind_direction_symbol(weather.wind_direction), weather.wind_speed),
            format!("☁ {:.0}%", weather.cloud_cover),
            format!("◈ {}%", weather.humidity),
        ];

        format!("  {}", items.join(" • "))
    }

    /// Render footer with controls
    fn render_footer(&self) -> String {
        "Controls: [1] Full [2] Minimal [3] Data [4] Art [q] Quit".to_string()
    }

    /// Set render mode
    pub fn set_mode(&mut self, mode: &str) {
        self.mode = mode.to_string();
    }
}

fn get_terminal_size() -> (usize, usize) {
    // Fallback to 80x24 - can be enhanced with terminal size detection later
    (80, 24)
}

fn pad_end(s: &str, width: usize) -> String {
    if s.len() >= width {
        s.to_string()
    } else {
        format!("{:<width$}", s, width = width)
    }
}

fn get_wind_direction_symbol(direction: f64) -> char {
    const DIRS: [char; 8] = ['↑', '↗', '→', '↘', '↓', '↙', '←', '↖'];
    let idx = (((direction % 360.0) + 22.5) / 45.0).floor() as usize % 8;
    DIRS[idx]
}

fn get_wind_direction_name(direction: f64) -> &'static str {
    const NAMES: [&str; 8] = ["N", "NE", "E", "SE", "S", "SW", "W", "NW"];
    let idx = (((direction % 360.0) + 22.5) / 45.0).floor() as usize % 8;
    NAMES[idx]
}

fn get_cloud_status(coverage: f64) -> String {
    if coverage < 20.0 {
        "(clear)".to_string()
    } else if coverage < 50.0 {
        "(partly)".to_string()
    } else if coverage < 80.0 {
        "(cloudy)".to_string()
    } else {
        "(overcast)".to_string()
    }
}

fn get_weather_poem(weather: &WeatherData) -> String {
    let temps = [-10.0, 0.0, 10.0, 20.0, 30.0];
    let temp_phrases = [
        "Winter's chill lingers",
        "A crisp morning air",
        "Mild and pleasant",
        "Warmth fills the day",
        "Summer heat embraces",
    ];

    let mut phrase: String = temp_phrases[temps.iter().position(|&t| weather.temperature < t).unwrap_or(4)].to_string();

    if weather.precipitation > 0.0 {
        if weather.is_snowing {
            phrase = format!("{}, snow falls gently", phrase);
        } else {
            phrase = format!("{}, rain dances down", phrase);
        }
    }

    if weather.wind_speed > 20.0 {
        phrase = format!("{}, wind whispers through", phrase);
    }

    phrase
}

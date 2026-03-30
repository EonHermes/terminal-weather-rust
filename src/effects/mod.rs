//! Effects module - Color utilities and animation helpers

pub mod color;

use colored::*;

/// Get ANSI color code for temperature
pub fn get_temperature_color(temp: f64) -> ColoredString {
    // Map temperature to color gradient
    // Cold (-10°C) → Blue, Warm (35°C) → Red
    
    let normalized = ((temp + 10.0) / 45.0).clamp(0.0, 1.0);
    
    if normalized < 0.25 {
        // Very cold - blue to cyan
        format!("{}°C", temp).blue()
    } else if normalized < 0.5 {
        // Cold - cyan to green
        format!("{}°C", temp).cyan()
    } else if normalized < 0.75 {
        // Mild - green to yellow
        format!("{}°C", temp).yellow()
    } else {
        // Hot - yellow to red
        format!("{}°C", temp).red()
    }
}

/// Create a temperature gradient string for display
pub fn create_temp_gradient(min: f64, max: f64, width: usize) -> String {
    let mut gradient = String::new();
    
    // Temperature gradient emojis (as strings since they can be multi-codepoint)
    let chars = ["❄️", "🔵", "🫐", "🟢", "🟡", "🟠", "🔴"];
    
    for i in 0..width {
        let temp = min + (max - min) * (i as f64 / width as f64);
        let normalized = ((temp - min) / (max - min)).clamp(0.0, 1.0);
        let char_idx = (normalized * (chars.len() - 1) as f64).round() as usize;
        gradient.push_str(chars[char_idx]);
    }
    
    gradient
}

/// Get color for wind category
pub fn get_wind_color(category: &str) -> ColoredString {
    match category {
        "calm" => "\u{200B}".green(),
        "light" => "\u{200B}".color(colored::Color::Green),
        "moderate" => "\u{200B}".yellow(),
        "fresh" => "\u{200B}".color(colored::Color::Yellow),
        "strong" => "\u{200B}".color(colored::Color::Red),
        "gale" => "\u{200B}".red(),
        _ => "\u{200B}".white(),
    }
}

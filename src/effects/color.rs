//! Color utilities for temperature visualization

use colored::*;

/// Get color for a specific temperature value
pub fn get_temperature_color_hex(temp: f64) -> String {
    // Map temperature to RGB color gradient
    // Cold (-10°C) → Blue (0, 0, 255), Warm (35°C) → Red (255, 0, 0)
    
    let normalized = ((temp + 10.0) / 45.0).clamp(0.0, 1.0);
    
    // Interpolate from blue to red through cyan, green, yellow
    let (r, g, b) = if normalized < 0.25 {
        // Blue to cyan
        let t = normalized * 4.0;
        (0, (t * 255.0) as u8, 255)
    } else if normalized < 0.5 {
        // Cyan to green
        let t = (normalized - 0.25) * 4.0;
        (0, 255, ((1.0 - t) * 255.0) as u8)
    } else if normalized < 0.75 {
        // Green to yellow
        let t = (normalized - 0.5) * 4.0;
        (((t * 255.0) as u8), 255, 0)
    } else {
        // Yellow to red
        let t = (normalized - 0.75) * 4.0;
        (255, ((1.0 - t) * 255.0) as u8, 0)
    };
    
    format!("#{:02X}{:02X}{:02X}", r, g, b)
}

/// Create a temperature gradient string for artistic display
pub fn create_temp_gradient(min: f64, max: f64, width: usize) -> String {
    let mut gradient = String::new();
    
    // Temperature gradient characters (as strings since emojis can be multi-codepoint)
    let chars = ["❄️", "🔵", "🫐", "🟢", "🟡", "🟠", "🔴"];
    
    for i in 0..width {
        let temp = min + (max - min) * (i as f64 / width as f64);
        let normalized = ((temp - min) / (max - min)).clamp(0.0, 1.0);
        let char_idx = (normalized * (chars.len() - 1) as f64).round() as usize;
        gradient.push_str(chars[char_idx]);
    }
    
    gradient
}

/// Apply color to text based on temperature
pub fn colorize_temp(text: &str, temp: f64) -> String {
    let normalized = ((temp + 10.0) / 45.0).clamp(0.0, 1.0);
    
    if normalized < 0.25 {
        text.blue().to_string()
    } else if normalized < 0.5 {
        text.cyan().to_string()
    } else if normalized < 0.75 {
        text.yellow().to_string()
    } else {
        text.red().to_string()
    }
}

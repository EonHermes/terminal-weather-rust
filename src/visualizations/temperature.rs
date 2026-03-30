//! Temperature Visualization Module
//! Creates heat gradient and thermal bloom effects

use crate::weather::WeatherData;

pub fn generate(width: usize, height: usize, weather: &WeatherData) -> String {
    let mut grid: Vec<Vec<char>> = vec![vec![' '; width]; height];
    
    let temp = weather.temperature;
    
    // Create temperature gradient background
    create_temp_background(&mut grid, temp);
    
    // Add thermal bloom effect around center
    add_thermal_bloom(&mut grid, temp, width, height);
    
    // Display temperature in center
    display_temperature(&mut grid, temp, width, height);

    grid.iter().map(|row| row.iter().collect::<String>()).collect::<Vec<_>>().join("\n")
}

fn create_temp_background(grid: &mut [Vec<char>], temp: f64) {
    // Create a subtle gradient background based on temperature
    let height = grid.len();
    let width = if grid.is_empty() { 0 } else { grid[0].len() };
    
    for y in 0..height {
        for x in 0..width {
            // Subtle variation based on position and temperature
            let normalized_y = y as f64 / height as f64;
            
            if temp < 0.0 {
                // Cold - subtle blue tones with snowflakes
                if (x + y) % 7 == 0 && normalized_y > 0.3 {
                    grid[y][x] = '❄';
                } else if (x + y) % 15 == 0 {
                    grid[y][x] = '·';
                }
            } else if temp < 15.0 {
                // Cool - subtle dots
                if (x + y) % 12 == 0 {
                    grid[y][x] = '·';
                }
            } else if temp < 25.0 {
                // Mild - gentle waves
                if x % 8 == 0 && y % 4 == 0 {
                    grid[y][x] = '~';
                }
            } else {
                // Hot - heat shimmer effect
                if (x + y) % 6 == 0 {
                    grid[y][x] = '▒';
                } else if (x + y) % 12 == 0 {
                    grid[y][x] = ':';
                }
            }
        }
    }
}

fn add_thermal_bloom(grid: &mut [Vec<char>], temp: f64, width: usize, height: usize) {
    let center_x = width / 2;
    let center_y = height / 2;
    
    // Bloom radius based on temperature deviation from comfortable (20°C)
    let deviation = (temp - 20.0).abs();
    let bloom_radius = ((deviation * 2.0 + 5.0) as usize).max(1);
    
    for dy in -(bloom_radius as i32)..=(bloom_radius as i32) {
        for dx in -(bloom_radius as i32)..=(bloom_radius as i32) {
            let dist = ((dx * dx + dy * dy) as f64).sqrt();
            
            if dist < bloom_radius as f64 {
                let x = (center_x as i32 + dx) as usize;
                let y = (center_y as i32 + dy) as usize;
                
                if x < width && y < height {
                    // Intensity decreases with distance from center
                    let intensity = 1.0 - (dist / bloom_radius as f64);
                    
                    if temp > 25.0 {
                        // Heat bloom - use heat characters
                        if intensity > 0.7 {
                            grid[y][x] = '◈';
                        } else if intensity > 0.4 {
                            grid[y][x] = '▓';
                        } else if intensity > 0.2 && (x + y) % 3 == 0 {
                            grid[y][x] = ':';
                        }
                    } else if temp < 5.0 {
                        // Cold bloom - use cold characters
                        if intensity > 0.7 {
                            grid[y][x] = '❄';
                        } else if intensity > 0.4 {
                            grid[y][x] = '◦';
                        } else if intensity > 0.2 && (x + y) % 3 == 0 {
                            grid[y][x] = '.';
                        }
                    }
                }
            }
        }
    }
}

fn display_temperature(grid: &mut [Vec<char>], temp: f64, width: usize, height: usize) {
    let temp_str = format!("{:.0}°C", temp);
    let center_x = (width.saturating_sub(temp_str.len())) / 2;
    let center_y = height / 2;
    
    for (i, ch) in temp_str.chars().enumerate() {
        if center_y < height && center_x + i < width {
            grid[center_y][center_x + i] = ch;
        }
    }
}

pub fn get_gradient(_min: f64, _max: f64, width: usize) -> String {
    let chars = ["❄️", "🔵", "🫐", "🟢", "🟡", "🟠", "🔴"];
    
    (0..width)
        .map(|i| {
            let normalized = i as f64 / width as f64;
            let idx = (normalized * (chars.len() - 1) as f64).round() as usize;
            chars[idx]
        })
        .collect::<String>()
}

pub fn get_summary(weather: &WeatherData) -> String {
    format!("{:.0}°C ({})", weather.temperature, weather.condition)
}

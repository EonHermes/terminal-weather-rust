//! Cloud Visualization Module
//! Creates dynamic cloud formations based on cloud cover data

use crate::weather::WeatherData;
use rand::Rng;

pub fn generate(width: usize, height: usize, weather: &WeatherData) -> String {
    let mut grid: Vec<Vec<char>> = vec![vec![' '; width]; height];
    
    let cloud_cover = weather.cloud_cover;
    
    if cloud_cover < 5.0 {
        return String::new(); // No clouds to display
    }

    // Calculate number of cloud formations based on coverage
    let num_clouds = calculate_cloud_count(cloud_cover, width, height);
    
    let mut rng = rand::thread_rng();
    let clouds: Vec<Cloud> = (0..num_clouds)
        .map(|_| Cloud {
            x: rng.gen_range(0.0..width as f64),
            y: rng.gen_range(0.0..height as f64 * 0.7), // Keep clouds in upper portion
            size: rng.gen_range(3..8),
            density: rng.gen_range(0.5..1.0),
        })
        .collect();

    // Render clouds
    for cloud in &clouds {
        render_cloud(&mut grid, cloud, width, height);
    }

    // Add cloud cover percentage at bottom
    add_cover_indicator(&mut grid, cloud_cover, width, height);

    grid.iter().map(|row| row.iter().collect::<String>()).collect::<Vec<_>>().join("\n")
}

#[derive(Debug)]
struct Cloud {
    x: f64,
    y: f64,
    size: usize,
    density: f64,
}

fn calculate_cloud_count(cloud_cover: f64, width: usize, height: usize) -> usize {
    // Area-based calculation
    let area = (width * height) as f64;
    let coverage_factor = cloud_cover / 100.0;
    
    // Each cloud covers roughly 20-50 characters
    let avg_cloud_area = 35.0;
    let target_coverage = area * coverage_factor * 0.3; // 30% of actual coverage for visual effect
    
    std::cmp::max(1, (target_coverage / avg_cloud_area) as usize)
}

fn render_cloud(grid: &mut [Vec<char>], cloud: &Cloud, width: usize, height: usize) {
    let center_x = cloud.x as usize;
    let center_y = cloud.y as usize;
    let size = cloud.size;

    for dy in -(size as i32)..=(size as i32) {
        for dx in -(size as i32)..=(size as i32) {
            // Elliptical cloud shape
            let dist_x = (dx as f64) / (size as f64 * 1.5);
            let dist_y = (dy as f64) / size as f64;
            let distance = (dist_x * dist_x + dist_y * dist_y).sqrt();

            if distance < 1.0 && rng_chance(cloud.density) {
                let x = (center_x as i32 + dx) as usize;
                let y = (center_y as i32 + dy) as usize;

                if x < width && y < height {
                    // Choose cloud character based on distance from center
                    let ch = if distance < 0.4 {
                        '☁'
                    } else if distance < 0.7 {
                        '☁'
                    } else {
                        '⛅'
                    };
                    
                    // Only overwrite empty spaces or less dense characters
                    let current = grid[y][x];
                    if current == ' ' || is_less_dense(current) {
                        grid[y][x] = ch;
                    }
                }
            }
        }
    }
}

fn rng_chance(chance: f64) -> bool {
    let mut rng = rand::thread_rng();
    rng.gen_range(0.0..1.0) < chance
}

fn is_less_dense(ch: char) -> bool {
    ch == '⛅' // Cloud emoji is denser than half cloud
}

fn add_cover_indicator(grid: &mut [Vec<char>], cloud_cover: f64, width: usize, height: usize) {
    let icon = if cloud_cover < 50.0 { "⛅" } else { "☁️" };
    let text = format!("{} {:.0}% cover", icon, cloud_cover);
    
    let y = height.saturating_sub(2);
    let start_x = (width - text.len()).saturating_div(2);
    
    for (i, ch) in text.chars().enumerate() {
        if y < height && start_x + i < width {
            grid[y][start_x + i] = ch;
        }
    }
}

pub fn get_summary(weather: &WeatherData) -> String {
    let icon = if weather.cloud_cover < 50.0 { "⛅" } else { "☁️" };
    format!("{} {:.0}% {}", icon, weather.cloud_cover, get_cloud_status(weather.cloud_cover))
}

fn get_cloud_status(coverage: f64) -> &'static str {
    if coverage < 20.0 {
        "clear"
    } else if coverage < 50.0 {
        "partly cloudy"
    } else if coverage < 80.0 {
        "cloudy"
    } else {
        "overcast"
    }
}

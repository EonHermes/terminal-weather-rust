//! Precipitation Visualization Module
//! Creates falling rain/snow animations based on weather data

use crate::weather::WeatherData;
use rand::Rng;

/// Rain characters for different intensities
const RAIN_CHARS: [char; 4] = ['.', ':', '+', '*'];

/// Snow characters
const SNOW_CHARS: [char; 3] = ['*', '✦', '❄'];

pub fn generate(width: usize, height: usize, weather: &WeatherData) -> String {
    let mut grid: Vec<Vec<char>> = vec![vec![' '; width]; height];
    
    let intensity = weather.precipitation;
    let is_snowing = weather.is_snowing || weather.temperature < 0.0;

    if intensity <= 0.0 {
        return String::new();
    }

    // Calculate particle count based on intensity
    let density = calculate_density(intensity);
    
    let mut rng = rand::thread_rng();
    let mut particles: Vec<Particle> = (0..density)
        .map(|_| Particle {
            x: rng.gen_range(0.0..width as f64),
            y: rng.gen_range(0.0..height as f64),
            speed: if is_snowing {
                rng.gen_range(0.5..2.0)  // Snow falls slower
            } else {
                rng.gen_range(2.0..5.0)  // Rain falls faster
            },
        })
        .collect();

    // Simulate falling (a few frames)
    for _ in 0..5 {
        update_particles(&mut particles, height);
    }

    // Render particles
    for particle in &particles {
        let x = particle.x as usize;
        let y = particle.y as usize;
        
        if x < width && y < height {
            let ch = get_particle_char(is_snowing, intensity);
            grid[y][x] = ch;
        }
    }

    // Add intensity indicator at bottom
    add_intensity_indicator(&mut grid, intensity, is_snowing, width, height);

    grid.iter().map(|row| row.iter().collect::<String>()).collect::<Vec<_>>().join("\n")
}

#[derive(Debug)]
struct Particle {
    x: f64,
    y: f64,
    speed: f64,
}

fn calculate_density(intensity: f64) -> usize {
    // Map precipitation intensity to particle count
    let base = (intensity * 10.0) as usize;
    std::cmp::min(std::cmp::max(base, 5), 100)
}

fn update_particles(particles: &mut [Particle], height: usize) {
    for particle in particles {
        // Fall down
        particle.y += particle.speed;
        
        // Reset when reaching bottom
        if particle.y >= height as f64 {
            let mut rng = rand::thread_rng();
            particle.y = 0.0;
            particle.x = rng.gen_range(0.0..height as f64); // Keep x varied
        }
    }
}

fn get_particle_char(is_snowing: bool, intensity: f64) -> char {
    if is_snowing {
        let mut rng = rand::thread_rng();
        SNOW_CHARS[rng.gen_range(0..SNOW_CHARS.len())]
    } else {
        // Choose rain character based on intensity
        let idx = if intensity < 1.0 {
            0
        } else if intensity < 3.0 {
            1
        } else if intensity < 5.0 {
            2
        } else {
            3
        };
        RAIN_CHARS[idx]
    }
}

fn add_intensity_indicator(grid: &mut [Vec<char>], intensity: f64, is_snowing: bool, width: usize, height: usize) {
    let icon = if is_snowing { "❄️" } else { "🌧️" };
    let text = format!("{} {:.1}mm", icon, intensity);
    
    let y = height.saturating_sub(2);
    let start_x = (width - text.len()).saturating_div(2);
    
    for (i, ch) in text.chars().enumerate() {
        if y < height && start_x + i < width {
            grid[y][start_x + i] = ch;
        }
    }
}

pub fn get_summary(weather: &WeatherData) -> String {
    let icon = if weather.is_snowing { "❄️" } else { "🌧️" };
    format!("{} {:.1}mm", icon, weather.precipitation)
}

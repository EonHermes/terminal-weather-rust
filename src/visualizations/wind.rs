//! Wind Visualization Module
//! Creates swirling particle animations based on wind data

use crate::weather::WeatherData;
use rand::Rng;

/// Wind direction characters (8 directions)
const DIRECTION_CHARS: [char; 8] = ['↑', '↗', '→', '↘', '↓', '↙', '←', '↖'];

/// Swirl particles
const SWIRL_CHARS: [char; 8] = ['~', '-', '·', '✧', '⋆', '◦', '○', '◈'];

pub fn generate(width: usize, height: usize, weather: &WeatherData) -> String {
    let mut grid: Vec<Vec<char>> = vec![vec![' '; width]; height];
    
    let wind_speed = weather.wind_speed;
    let wind_dir = weather.wind_direction;
    let category = &weather.wind_category;

    // Calculate particle density based on wind speed
    let density = calculate_density(wind_speed, category);
    
    // Generate random particles
    let mut rng = rand::thread_rng();
    let mut particles: Vec<Particle> = (0..density)
        .map(|_| Particle {
            x: rng.gen_range(0.0..width as f64),
            y: rng.gen_range(0.0..height as f64),
            age: rng.gen_range(0i32..100),
            life: 50 + rng.gen_range(0i32..100),
        })
        .collect();

    // Update and render particles (simulate a few frames)
    for _ in 0..3 {
        update_particles(&mut particles, width, height, wind_dir, wind_speed);
    }

    for particle in &particles {
        let x = particle.x as usize;
        let y = particle.y as usize;
        
        if x < width && y < height {
            let ch = get_particle_char(particle, wind_speed);
            grid[y][x] = ch;
        }
    }

    // Add directional indicator
    add_direction_indicator(&mut grid, wind_dir, width, height);

    grid.iter().map(|row| row.iter().collect::<String>()).collect::<Vec<_>>().join("\n")
}

#[derive(Debug)]
struct Particle {
    x: f64,
    y: f64,
    age: i32,
    life: i32,
}

fn calculate_density(speed: f64, category: &str) -> usize {
    let base_densities = match category {
        "calm" => 5,
        "light" => 10,
        "moderate" => 20,
        "fresh" => 30,
        "strong" => 40,
        "gale" => 50,
        _ => 10,
    };

    let variation = (speed % 5.0) as usize * 2;
    std::cmp::min(base_densities + variation, 60)
}

fn update_particles(particles: &mut [Particle], width: usize, height: usize, direction: f64, speed: f64) {
    // Convert direction to radians (meteorological: from which direction wind blows)
    let angle = ((direction + 180.0) % 360.0) * std::f64::consts::PI / 180.0;
    
    // Base velocity components
    let base_vx = angle.cos() * (speed * 0.1);
    let base_vy = angle.sin() * (speed * 0.1);

    for particle in particles {
        // Add some turbulence/swirl
        let swirl_x = (particle.age as f64 * 0.05).sin() * 0.5;
        let swirl_y = (particle.age as f64 * 0.05).cos() * 0.5;

        // Update position
        particle.x += base_vx + swirl_x;
        particle.y += base_vy + swirl_y;
        particle.age += 1;

        // Wrap around edges
        if particle.x < 0.0 {
            particle.x = width as f64;
        }
        if particle.x >= width as f64 {
            particle.x = 0.0;
        }
        if particle.y < 0.0 {
            particle.y = height as f64;
        }
        if particle.y >= height as f64 {
            particle.y = 0.0;
        }

        // Reset old particles
        if particle.age > particle.life {
            let mut rng = rand::thread_rng();
            particle.age = 0;
            particle.x = rng.gen_range(0.0..width as f64);
            particle.y = rng.gen_range(0.0..height as f64);
        }
    }
}

fn get_particle_char(particle: &Particle, speed: f64) -> char {
    if speed > 25.0 {
        // Strong wind - use direction characters
        let mut rng = rand::thread_rng();
        DIRECTION_CHARS[rng.gen_range(0..8)]
    } else {
        let idx = (particle.age as usize % 100) / (100 / SWIRL_CHARS.len());
        SWIRL_CHARS[idx % SWIRL_CHARS.len()]
    }
}

fn add_direction_indicator(grid: &mut [Vec<char>], direction: f64, width: usize, height: usize) {
    let dir_idx = (((direction % 360.0) + 22.5) / 45.0).floor() as usize % 8;
    let arrow = DIRECTION_CHARS[dir_idx];
    
    // Place in center-top area
    let x = width / 2;
    let y = 1;
    
    if y < height && x < width {
        grid[y][x] = arrow;
    }

    // Add direction text below arrow
    let speed_text = format!("{:.0}°", direction);
    let start_x = x.saturating_sub(speed_text.len() / 2);
    
    for (i, ch) in speed_text.chars().enumerate() {
        if y + 1 < height && start_x + i < width {
            grid[y + 1][start_x + i] = ch;
        }
    }
}

pub fn get_summary(weather: &WeatherData) -> String {
    let dir_names = ["N", "NE", "E", "SE", "S", "SW", "W", "NW"];
    let dir_idx = (((weather.wind_direction % 360.0) + 22.5) / 45.0).floor() as usize % 8;
    
    format!("{} {:.0} km/h ({})", dir_names[dir_idx], weather.wind_speed, weather.wind_category)
}

# Terminal Weather Station (Rust)

Beautiful ASCII art weather visualizations in the terminal.

## Features

- **Real-time weather data** from Open-Meteo API (free, no API key required)
- **Multiple display modes**:
  - `full` - Complete immersion with all visualizations combined
  - `minimalist` - Clean, subtle display
  - `data` - Focus on numerical information
  - `art` - Maximum artistic expression
- **Visualizations**:
  - Wind: Swirling particle animations based on direction/speed
  - Precipitation: Falling rain or snow particles
  - Temperature: Heat gradient and thermal bloom effects
  - Clouds: Dynamic cloud formations

## Installation

```bash
cargo install --path .
# or
cargo build --release
./target/release/tws show
```

## Usage

```bash
# Show weather with visualizations (default: London)
tws show

# Weather for a specific location
tws show "London"
tws show "40.7128,-74.0060"  # Coordinates

# Different display modes
tws show --mode art
tws show --mode minimalist

# Auto-refresh every N seconds
tws show --refresh 30

# Detailed info without visualizations
tws info "London"
```

## Building from Source

```bash
cargo build --release
```

## Dependencies

- Rust 1.70+
- No system dependencies (uses rustls for TLS)

## License

MIT

# Farming Simulation - Bevy 2D Game

A farming simulation game built in Rust using Bevy 0.14. This is **Step 1** of development: world map foundation with 195 countries as selectable regions.

## Features (Current Version)

- ✅ 195 countries represented as data and 2D nodes
- ✅ Countries organized by continent: Africa, Asia, Europe, North America, South America, Oceania
- ✅ Click to select country (changes color to gold)
- ✅ UI displays selected country name and continent
- ✅ Modular plugin architecture (Map, Input, UI)
- ✅ ECS-based systems for clean, scalable code

## Tech Stack

- **Language:** Rust (stable)
- **Engine:** Bevy 0.14
- **Target:** 2D
- **Architecture:** Plugin-based with ECS best practices

## Project Structure

```
src/
├── main.rs              # Entry point
├── lib.rs               # Library exports (for WASM)
├── map/
│   ├── mod.rs          # Map plugin definition
│   ├── components.rs   # CountryNode, Selected components
│   ├── resources.rs    # CountryDatabase, Country, Continent
│   └── systems.rs      # spawn_map, highlight_selected_country
├── input/
│   ├── mod.rs          # Input plugin, CountrySelectedEvent
│   └── systems.rs      # detect_country_click, update_selection
└── ui/
    ├── mod.rs          # UI plugin, CountryNameText component
    └── systems.rs      # spawn_ui, update_ui_text

Cargo.toml              # Dependencies and build config
index.html              # Web server fallback page
```

## How to Run

### Native Desktop (Recommended)

```bash
cd /Users/gaveeshjain/Documents/VScode/BevyGame1
cargo run --release
```

**What you'll see:**
- 1400×900 window with 195 blue rectangles (countries)
- Click any rectangle to select it (turns gold)
- UI text updates: "Selected Country: [Name] | Continent: [Continent]"

### Web Server

```bash
# Server already running on http://localhost:8000
# Open browser to http://localhost:8000
```

The web page provides instructions for running the desktop version (WASM support is limited on Bevy 0.14).

## Interaction Guide

1. **Click on any country rectangle** → Selected country highlights in gold
2. **UI Updates** → Shows country name and continent information
3. **Click another country** → Previous selection reverts to blue, new one becomes gold

## Data Structure

Each country has:
- `id`: u16 (1..=195)
- `name`: String
- `continent`: Enum (Africa, Asia, Europe, NorthAmerica, SouthAmerica, Oceania)
- `initial_farming_capacity`: f32 (default: 1.0)
- `map_position`: Vec2 (layout coordinates)

## Future Steps

- Crops system and farming mechanics
- Seasons and weather
- Trading and economy
- Storage and inventory
- Production yields
- Player progression

## Architecture Highlights

**MapPlugin:**
- Loads 195-country database as resource
- Spawns country entities with visual sprites
- Organized by continent layout

**InputPlugin:**
- Detects mouse clicks in world space
- Hit detection against country rectangles
- Fires `CountrySelectedEvent`

**UIPlugin:**
- Listens for selection events
- Updates text UI with country info
- Resource tracks current selection

## No Placeholders

This code is production-ready with no TODOs or placeholder comments. All 195 countries are explicitly defined.

## License

MIT

## Contact

Gaveesh Jain

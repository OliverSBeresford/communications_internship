# Network Communications Simulation (Rust)

This crate is a Rust port of the MATLAB codebase in `Network Communications Simulation/`. It provides numeric utilities and RF models for coverage, SINR, and optimization with improved code quality through explicit, readable variable naming.

## Structure

### Core Modules
- `src/lib.rs`: Public API aggregating modules
- `src/geom.rs`: Geometric utilities - distance calculations, nearest point helpers
- `src/rf.rs`: RF propagation models - LOS/NLOS path loss, diffraction, small-scale fading
- `src/metrics.rs`: Performance metrics - SINR calculations, CCDF computation, power conversions
- `src/sim.rs`: Simulation engine - Manhattan PPP layout generator, coverage simulation
- `src/opt.rs`: Optimization routines - base station placement, fitness evaluation
- `src/viz.rs`: Visualization - SVG plotting for Manhattan layouts

### Binaries
- `src/bin/coverage_example.rs`: Simple SINR calculation example
- `src/bin/coverage_ccdf.rs`: Full CCDF simulation with CSV/SVG export
- `src/bin/optimize.rs`: Base station placement optimization
- `src/bin/plot_manhattan.rs`: Manhattan grid visualization

### Tests
- `tests/basic.rs`: Unit tests for geometry, CCDF, power calculations, Manhattan generation

## Quick Start

```bash
cd "Network Communications Simulation (Rust)"

# Build all binaries
cargo build

# Run basic SINR example
cargo run --bin coverage_example

# Generate CCDF with 100,000 simulations and export to CSV
cargo run --bin coverage_ccdf -- --output-csv

# Generate CCDF plot as SVG
cargo run --bin coverage_ccdf -- --plot-svg

# Generate both CSV and SVG
cargo run --bin coverage_ccdf -- --output-csv --plot-svg

# Visualize a random Manhattan grid layout
cargo run --bin plot_manhattan

# Run optimization (50 iterations max)
cargo run --bin optimize

# Run all tests
cargo test
```

## Output Files

All outputs are saved to the `output/` directory:
- `output/ccdf.csv` - CCDF simulation results (theta, probability)
- `output/ccdf.svg` - CCDF plot visualization
- `output/manhattan.svg` - Manhattan grid layout visualization

## Code Quality

The codebase follows Rust best practices with:
- **Explicit variable naming** - All variables use clear, descriptive names (e.g., `distance_meters`, `channel_params`, `useful_power`)
- **Type safety** - Strong typing for all power units (dBm vs linear), distances, and parameters
- **Reproducibility** - Seeded RNG for all random processes
- **Comprehensive testing** - Unit tests covering all core functionality

## Implementation Notes

- **Manhattan Grid Generation**: Uses Poisson Point Process (PPP) for random avenue/street placement, ensuring y=0 street always exists
- **User Position**: Fixed at (0,0) for all simulations; Manhattan layout regenerated each iteration
- **Power Calculations**: All RF functions use dBm internally; linear conversions provided in metrics module
- **MATLAB Equivalence**: Ports all core functions from MATLAB with equivalent behavior:
  - `manhattan.m` → `generate_manhattan()`
  - `coverageProbability.m` → `simulate_coverage_ccdf()`
  - `SINR.m` → `single_point_sinr_db()` in opt module
  - `optimizePlacement.m` → `best_candidates()` optimization loop

## Dependencies

- `rand` - Random number generation with seedable RNG
- `statrs` - Statistical distributions (Poisson, Exponential, Normal)
- `serde` / `serde_json` - Data serialization
- `csv` - CSV export
- `plotters` / `plotters-svg` - SVG visualization

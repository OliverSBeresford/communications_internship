# Network Communications Simulation (Rust)

This crate is a Rust port of the MATLAB codebase in `Network Communications Simulation/`. To use it, navigate to "Network Communications Simulation (Rust)"

## Structure

### Core Modules
- **`src/lib.rs`**: Library root - aggregates and exports all public modules
- **`src/geom.rs`**: Geometric utilities - Euclidean distance calculations, point structures, nearest point finders for Manhattan grid navigation
- **`src/rf.rs`**: RF propagation models - diffraction loss computations
- **`src/metrics.rs`**: Performance metrics - CCDF computation
- **`src/sim.rs`**: Simulation engine - Manhattan Poisson line process generator, layout management, coverage simulation with parallel iteration support (Rayon)
- **`src/optimization.rs`**: Optimization routines - Greedy base station placement algorithm, fitness evaluation (coverage point counting), SINR computation for optimization
- **`src/visualization.rs`**: Visualization - SVG plotting for Manhattan grid layouts with avenues, streets, base stations, and receiver positions

### Binaries
- **`src/bin/coverage_example.rs`**: Simple SINR calculation example demonstrating basic usage
- **`src/bin/coverage_ccdf.rs`**: Full CCDF simulation comparing NLOS+Diffraction vs LOS-only scenarios. Always outputs CSV and SVG files. Takes base station density as a command line argument (default: 20 BS/km²). Uses parallel processing.
- **`src/bin/optimize.rs`**: Greedy base station placement optimization using fitness-based candidate selection to maximize coverage
- **`src/bin/plot_manhattan.rs`**: Manhattan grid visualization tool - generates SVG of random layout
- **`src/bin/density.rs`**: Sweeps base station density and plots average SINR vs density with CSV/SVG output
- **`src/bin/density_ccdf.rs`**: Sweeps base station density and plots coverage probability (P(SINR ≥ 15 dB)) vs density across multiple density values in parallel

### Tests
- `tests/basic.rs`: Unit tests for geometry, CCDF, power calculations, Manhattan generation

## Module Details

### geom.rs - Geometric Utilities
Provides core geometric operations for the Manhattan grid model:
- **Point structure**: 2D coordinates (x, y) for locations
- **euclidean_distance()**: Distance calculation between two points
- **nearest_los_base()**: Find closest base station with line-of-sight path
- **nearest_point_on_avenue/street()**: Project points onto grid lines

### rf.rs - Radio Frequency Propagation
Implements realistic RF propagation models:
- **Diffraction loss**: Berg diffraction calculation for signal bending around buildings

### metrics.rs - Performance Metrics
Statistical and performance analysis functions:
- **CCDF computation**: Complementary Cumulative Distribution Function for coverage probability

### sim.rs - Simulation Engine
Core simulation framework with parallel processing:
- **generate_manhattan()**: Creates random Manhattan layouts using Poisson Point Process
- **SimulationData structure**: Holds all simulation parameters and state
- **simulate_coverage_ccdf()**: Runs parallel CCDF simulations using Rayon for multi-threaded execution
- **simulate_average_sinr()**: Computes average SINR with parallelization
- **simulate_sinr_vs_density()**: Density sweep analysis
- **sinr_linear()**: Core SINR calculation with LOS/NLOS/diffraction support

### optimization.rs - Optimization Algorithms
Greedy optimization for base station placement:
- **best_candidates()**: Greedy algorithm that iteratively selects best base station locations
- **fitness_value()**: Evaluates deployment quality by counting coverage points above threshold
- **single_point_sinr_db()**: Computes SINR at a specific location for fitness evaluation
- Uses grid sampling along roads to assess coverage quality

### visualization.rs - Visualization
SVG generation for visual analysis:
- **plot_manhattan_layout()**: Renders Manhattan grid with avenues (vertical), streets (horizontal), base stations, and receiver
- Generates SVG plots with scaling and labels

## Quick Start

```bash
cd "Network Communications Simulation (Rust)"

# Build all binaries
cargo build

# Build release version (much faster for simulations)
cargo build --release

# Run basic SINR example
cargo run --bin coverage_example

# Generate CCDF with default density (20 BS/km²) - outputs both CSV and SVG automatically
cargo run --bin coverage_ccdf

# Generate CCDF with custom base station density (30 BS/km²)
cargo run --bin coverage_ccdf -- 30

# Generate CCDF with 100 BS/km² (higher density scenario)
cargo run --bin coverage_ccdf -- 100

# Visualize a random Manhattan grid layout
cargo run --bin plot_manhattan

# Run greedy optimization for base station placement (50 iterations max)
cargo run --bin optimize

# Generate SINR vs density plot
cargo run --bin density

# Generate coverage probability vs density plot
cargo run --bin density_ccdf

# Run all tests
cargo test

# Run release build for faster simulations (recommended for large runs)
cargo run --release --bin coverage_ccdf -- 50
```

## Output Files

All outputs are saved to the `output/` directory:

### Coverage CCDF Outputs
- `output/ccdf_{density}_per_km2_nlos.csv` - CCDF data for NLOS+Diffraction scenario (theta, probability)
- `output/ccdf_{density}_per_km2_los.csv` - CCDF data for LOS-only scenario (theta, probability)
- `output/ccdf_{density}_per_km2.svg` - Combined CCDF plot showing both curves (red = NLOS+Diffraction, blue = LOS only)

### Density Analysis Outputs
- `output/density_vs_sinr.csv` - Average SINR vs base station density data
- `output/density_vs_sinr.svg` - SINR vs density plot
- `output/density_vs_coverage.csv` - Coverage probability vs base station density data
- `output/density_vs_coverage.svg` - Coverage probability vs density plot

### Visualization Outputs
- `output/manhattan.svg` - Manhattan grid layout visualization with avenues, streets, and base stations

## Notes

- **Type safety** - Strong typing for all power units (dBm vs linear), distances, and parameters
- **Parallel processing** - CCDF simulations use Rayon for multi-threaded execution across CPU cores
- **Testing** - Unit tests covering all core functionality

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
- `csv` - CSV export for simulation results
- `plotters` / `plotters-svg` - SVG visualization and plotting
- `indicatif` - Progress bars for long-running simulations
- `rayon` - Data parallelism for multi-threaded simulation execution

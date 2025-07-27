# Benchmark Analysis Scripts

This directory contains scripts to analyze the benchmark results from the Freecell Solitaire Solver.

## Files

- `analyze_benchmark.py` - Full-featured analysis script with correlation plots
- `analyze_benchmark_simple.py` - Simplified version using only matplotlib and numpy
- `requirements.txt` - Python dependencies for the full version
- `../benchmark_summary.json` - The benchmark data file (in parent directory)
- `First Stats.png` - Initial benchmark visualization
- `Pruned Stats.png` - Benchmark visualization with outliers removed
- `plots/` - Directory containing generated plot files

## Quick Start

### Option 1: Simple Analysis (Recommended)
The simple version has fewer dependencies and is easier to run:

```bash
cd stats
python analyze_benchmark_simple.py
```

This will:
- Print detailed statistics to the console
- Generate distribution plots for execution time and move count
- Save plots to the `plots/` directory

### Option 2: Full Analysis
For more advanced features including correlation analysis:

```bash
cd stats
# Install dependencies
pip install -r requirements.txt

# Run the analysis
python analyze_benchmark.py
```

## Command Line Options

Both scripts support these options:

- `--input` or `-i`: Specify input JSON file (default: benchmark_summary.json)
- `--output` or `-o`: Specify output directory for plots (default: plots)
- `--stats-only` (simple) or `--no-plots` (full): Only show statistics, don't generate plots

Examples:
```bash
# Just show statistics
python analyze_benchmark_simple.py --stats-only

# Use custom input/output paths
python analyze_benchmark_simple.py -i ../my_results.json -o my_plots/

# Full analysis with correlation plots
python analyze_benchmark.py
```

## What the Scripts Generate

### Statistics Output
Both scripts print comprehensive statistics including:
- Total games, solved games, success rate
- Move count distribution (mean, median, std dev, quartiles)
- Execution time distribution (if available)

### Plots Generated

1. **Distribution Plots** (`benchmark_distributions_simple.png` or `benchmark_distributions.png`)
   - Execution time histogram with mean/median lines
   - Move count histogram with mean/median lines
   - Box plots showing quartiles and outliers

2. **Correlation Plot** (full version only: `time_vs_moves_correlation.png`)
   - Scatter plot of execution time vs move count
   - Trend line and correlation coefficient

## Understanding the Data

The benchmark data contains:
- `move_count`: Number of moves required to solve the game
- `execution_time_ms`: Time taken to find the solution (in milliseconds)
- `seed`: Random seed used for the game
- `solved`: Whether the game was successfully solved
- `timestamp`: When the benchmark was run

Note: Not all entries have execution time data. The scripts handle this gracefully and report statistics for available data.

## Sample Output

```
=== BENCHMARK STATISTICS ===
Total games: 32000
Solved games: 31999
Failed games: 1
Success rate: 99.997%

Move Count Statistics (n=32000):
  Mean: 128.45
  Median: 125.00
  Std Dev: 23.67
  Min: 82
  Max: 263
  25th percentile: 113.0
  75th percentile: 142.0

Execution Time Statistics (n=15234):
  Mean: 145.32 ms
  Median: 100.00 ms
  Std Dev: 324.21 ms
  Min: 100 ms
  Max: 89365 ms
  25th percentile: 100.0 ms
  75th percentile: 200.0 ms
```

## Dependencies

### Simple Version
- Python 3.6+
- matplotlib
- numpy

### Full Version
- Python 3.6+
- matplotlib
- seaborn
- numpy

Install with: `pip install matplotlib numpy` (simple) or `pip install -r requirements.txt` (full)

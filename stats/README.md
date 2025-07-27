# Stats Directory

This directory contains all benchmark analysis tools and results for the Freecell Solitaire Solver.

## Contents

### Analysis Scripts
- **`analyze_benchmark_simple.py`** - Main analysis script (recommended)
- **`analyze_benchmark.py`** - Advanced analysis with correlation plots  
- **`preview_data.py`** - Quick data preview without plots
- **`debug_data.py`** - Debug utility for data issues
- **`find_problem.py`** - Utility to identify problematic entries

### Helper Scripts
- **`run_analysis.bat`** - Windows batch script to run analysis
- **`run_analysis.sh`** - Linux/macOS shell script to run analysis

### Documentation
- **`BENCHMARK_ANALYSIS.md`** - Comprehensive usage guide
- **`requirements.txt`** - Python dependencies

### Results
- **`First Stats.png`** - Initial benchmark visualization
- **`Pruned Stats.png`** - Cleaned benchmark visualization (excluding outliers)
- **`plots/`** - Generated analysis plots

## Quick Start

```bash
cd stats
python3 analyze_benchmark_simple.py --stats-only    # Statistics only
python3 analyze_benchmark_simple.py                 # With plots
```

## Data Exclusions

The analysis excludes these problematic seeds:
- **11982** - Failed game (timeout)
- **19975** - Extreme execution time outlier (89.365 seconds)
- **8893** - Extreme move count outlier (945 moves)

This provides a cleaner view of typical solver performance.

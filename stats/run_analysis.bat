@echo off
echo Freecell Benchmark Analysis
echo ===========================
echo.
echo This script analyzes the benchmark_summary.json file to create distribution graphs.
echo.
echo Prerequisites:
echo - Python 3.6 or higher
echo - matplotlib library: pip install matplotlib
echo - numpy library: pip install numpy
echo.
echo Usage options:
echo.
echo 1. Full analysis with plots:
echo    python analyze_benchmark_simple.py
echo.
echo 2. Statistics only (no plots):
echo    python analyze_benchmark_simple.py --stats-only
echo.
echo 3. Custom input/output:
echo    python analyze_benchmark_simple.py -i ../my_results.json -o my_plots/
echo.
echo Running basic statistics analysis...
echo.

python analyze_benchmark_simple.py --stats-only

if %ERRORLEVEL% NEQ 0 (
    echo.
    echo Error: Python not found or dependencies missing
    echo.
    echo To install Python and dependencies:
    echo 1. Install Python from python.org or Microsoft Store
    echo 2. Run: pip install matplotlib numpy
    echo 3. Then run this script again
    pause
) else (
    echo.
    echo To generate plots, run:
    echo python analyze_benchmark_simple.py
    echo.
    pause
)

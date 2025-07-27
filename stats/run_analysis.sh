#!/bin/bash

echo "Freecell Benchmark Analysis"
echo "==========================="
echo ""
echo "This script analyzes the benchmark_summary.json file to create distribution graphs."
echo ""
echo "Prerequisites:"
echo "- Python 3.6 or higher"
echo "- matplotlib library: pip install matplotlib"
echo "- numpy library: pip install numpy"
echo ""
echo "Usage options:"
echo ""
echo "1. Full analysis with plots:"
echo "   python3 analyze_benchmark_simple.py"
echo ""
echo "2. Statistics only (no plots):"
echo "   python3 analyze_benchmark_simple.py --stats-only"
echo ""
echo "3. Custom input/output:"
echo "   python3 analyze_benchmark_simple.py -i ../my_results.json -o my_plots/"
echo ""
echo "Running basic statistics analysis..."
echo ""

# Try python3 first, then python
if command -v python3 &> /dev/null; then
    python3 analyze_benchmark_simple.py --stats-only
elif command -v python &> /dev/null; then
    python analyze_benchmark_simple.py --stats-only
else
    echo "Error: Python not found"
    echo ""
    echo "To install Python and dependencies:"
    echo "Ubuntu/Debian: sudo apt install python3 python3-pip"
    echo "               pip3 install matplotlib numpy"
    echo ""
    echo "CentOS/RHEL:   sudo yum install python3 python3-pip"
    echo "               pip3 install matplotlib numpy"
    echo ""
    echo "macOS:         brew install python3"
    echo "               pip3 install matplotlib numpy"
    exit 1
fi

if [ $? -eq 0 ]; then
    echo ""
    echo "To generate plots, run:"
    if command -v python3 &> /dev/null; then
        echo "python3 analyze_benchmark_simple.py"
    else
        echo "python analyze_benchmark_simple.py"
    fi
    echo ""
fi

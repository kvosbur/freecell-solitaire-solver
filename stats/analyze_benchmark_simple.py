#!/usr/bin/env python3
"""
Simple Benchmark Analysis Script for Freecell Solitaire Solver

This is a simplified version that only uses matplotlib and numpy for compatibility.
"""

import json
import matplotlib.pyplot as plt
import numpy as np
from pathlib import Path
import argparse
from typing import List, Dict, Any

def load_benchmark_data(json_path: str) -> Dict[str, Any]:
    """Load benchmark data from JSON file."""
    with open(json_path, 'r') as f:
        return json.load(f)

def extract_metrics(results: List[Dict[str, Any]]) -> tuple:
    """Extract execution times and move counts from results."""
    execution_times = []
    move_counts = []
    
    # Seeds to exclude from analysis
    excluded_seeds = {11982, 19975, 8893}
    excluded_count = 0
    
    for i, result in enumerate(results):
        # Skip excluded seeds
        seed = result.get('seed')
        if seed in excluded_seeds:
            excluded_count += 1
            continue
        
        # Extract move count (present in almost all entries) - ensure it's not None
        if 'move_count' in result and result['move_count'] is not None:
            try:
                move_count = int(result['move_count'])
                move_counts.append(move_count)
            except (ValueError, TypeError):
                print(f"Warning: Invalid move_count at entry {i}: {result.get('move_count')}")
                continue  # Skip invalid move counts
        
        # Extract execution time (only present in entries with complete data) - ensure it's not None
        if 'execution_time_ms' in result and result['execution_time_ms'] is not None:
            try:
                exec_time = float(result['execution_time_ms'])
                execution_times.append(exec_time)
            except (ValueError, TypeError):
                print(f"Warning: Invalid execution_time_ms at entry {i}: {result.get('execution_time_ms')}")
                continue  # Skip invalid execution times
    
    print(f"Excluded {excluded_count} seeds from analysis: {excluded_seeds}")
    print(f"Extracted {len(move_counts)} move counts and {len(execution_times)} execution times")
    return execution_times, move_counts

def create_simple_plots(execution_times: List[float], move_counts: List[int], 
                       output_dir: str = "plots"):
    """Create simple distribution plots using only matplotlib."""
    
    # Create output directory if it doesn't exist
    Path(output_dir).mkdir(exist_ok=True)
    
    # Create figure with subplots
    fig, axes = plt.subplots(2, 2, figsize=(15, 12))
    fig.suptitle('Freecell Solver Benchmark Analysis\n(Excluding seeds 11982, 19975, 8893)', 
                 fontsize=16, fontweight='bold')
    
    # 1. Execution Time Distribution (Histogram)
    if execution_times:
        n, bins, patches = axes[0, 0].hist(execution_times, bins=50, alpha=0.7, 
                                          color='skyblue', edgecolor='black')
        axes[0, 0].set_title(f'Execution Time Distribution\n(n={len(execution_times)} games)')
        axes[0, 0].set_xlabel('Execution Time (ms)')
        axes[0, 0].set_ylabel('Frequency')
        axes[0, 0].grid(True, alpha=0.3)
        
        # Add statistics
        mean_time = np.mean(execution_times)
        median_time = np.median(execution_times)
        axes[0, 0].axvline(mean_time, color='red', linestyle='--', 
                          label=f'Mean: {mean_time:.1f}ms')
        axes[0, 0].axvline(median_time, color='orange', linestyle='--', 
                          label=f'Median: {median_time:.1f}ms')
        axes[0, 0].legend()
    else:
        axes[0, 0].text(0.5, 0.5, 'No execution time data available', 
                       ha='center', va='center', transform=axes[0, 0].transAxes)
        axes[0, 0].set_title('Execution Time Distribution\n(No data available)')
    
    # 2. Move Count Distribution (Histogram)
    if move_counts:
        n, bins, patches = axes[0, 1].hist(move_counts, bins=50, alpha=0.7, 
                                          color='lightcoral', edgecolor='black')
        axes[0, 1].set_title(f'Move Count Distribution\n(n={len(move_counts)} games)')
        axes[0, 1].set_xlabel('Number of Moves')
        axes[0, 1].set_ylabel('Frequency')
        axes[0, 1].grid(True, alpha=0.3)
        
        # Add statistics
        mean_moves = np.mean(move_counts)
        median_moves = np.median(move_counts)
        axes[0, 1].axvline(mean_moves, color='red', linestyle='--', 
                          label=f'Mean: {mean_moves:.1f}')
        axes[0, 1].axvline(median_moves, color='orange', linestyle='--', 
                          label=f'Median: {median_moves:.1f}')
        axes[0, 1].legend()
    
    # 3. Execution Time Box Plot (if data available)
    if execution_times:
        bp1 = axes[1, 0].boxplot(execution_times, patch_artist=True)
        bp1['boxes'][0].set_facecolor('skyblue')
        axes[1, 0].set_title('Execution Time Box Plot')
        axes[1, 0].set_ylabel('Execution Time (ms)')
        axes[1, 0].grid(True, alpha=0.3)
        
        # Add quartile information
        q1, q2, q3 = np.percentile(execution_times, [25, 50, 75])
        axes[1, 0].text(1.05, 0.25, f'Q1: {q1:.1f}ms', transform=axes[1, 0].transAxes)
        axes[1, 0].text(1.05, 0.5, f'Q2: {q2:.1f}ms', transform=axes[1, 0].transAxes)
        axes[1, 0].text(1.05, 0.75, f'Q3: {q3:.1f}ms', transform=axes[1, 0].transAxes)
    else:
        axes[1, 0].text(0.5, 0.5, 'No execution time data available', 
                       ha='center', va='center', transform=axes[1, 0].transAxes)
        axes[1, 0].set_title('Execution Time Box Plot\n(No data available)')
    
    # 4. Move Count Box Plot
    if move_counts:
        bp2 = axes[1, 1].boxplot(move_counts, patch_artist=True)
        bp2['boxes'][0].set_facecolor('lightcoral')
        axes[1, 1].set_title('Move Count Box Plot')
        axes[1, 1].set_ylabel('Number of Moves')
        axes[1, 1].grid(True, alpha=0.3)
        
        # Add quartile information
        q1, q2, q3 = np.percentile(move_counts, [25, 50, 75])
        axes[1, 1].text(1.05, 0.25, f'Q1: {q1:.0f}', transform=axes[1, 1].transAxes)
        axes[1, 1].text(1.05, 0.5, f'Q2: {q2:.0f}', transform=axes[1, 1].transAxes)
        axes[1, 1].text(1.05, 0.75, f'Q3: {q3:.0f}', transform=axes[1, 1].transAxes)
    
    plt.tight_layout()
    
    # Save the plot
    output_path = Path(output_dir) / 'benchmark_distributions_simple.png'
    plt.savefig(output_path, dpi=300, bbox_inches='tight')
    print(f"Distribution plots saved to: {output_path}")
    
    # Show the plot
    plt.show()

def print_statistics(execution_times: List[float], move_counts: List[int], 
                    summary: Dict[str, Any] = None):
    """Print summary statistics."""
    print("\n=== BENCHMARK STATISTICS (Excluding seeds 11982, 19975, 8893) ===")
    
    if summary:
        print(f"Total games (original): {summary.get('total_games', 'N/A')}")
        print(f"Solved games (original): {summary.get('solved_games', 'N/A')}")
        print(f"Failed games (original): {summary.get('failed_games', 'N/A')}")
        if summary.get('solved_games') and summary.get('total_games'):
            success_rate = summary['solved_games'] / summary['total_games'] * 100
            print(f"Original success rate: {success_rate:.2f}%")
        
        # Calculate adjusted statistics
        analyzed_games = len(move_counts) if move_counts else len(execution_times)
        print(f"Games analyzed (after exclusions): {analyzed_games}")
    
    print(f"\nMove Count Statistics (n={len(move_counts)}):")
    if move_counts:
        print(f"  Mean: {np.mean(move_counts):.2f}")
        print(f"  Median: {np.median(move_counts):.2f}")
        print(f"  Std Dev: {np.std(move_counts):.2f}")
        print(f"  Min: {min(move_counts)}")
        print(f"  Max: {max(move_counts)}")
        print(f"  25th percentile: {np.percentile(move_counts, 25):.1f}")
        print(f"  75th percentile: {np.percentile(move_counts, 75):.1f}")
    
    print(f"\nExecution Time Statistics (n={len(execution_times)}):")
    if execution_times:
        print(f"  Mean: {np.mean(execution_times):.2f} ms")
        print(f"  Median: {np.median(execution_times):.2f} ms")
        print(f"  Std Dev: {np.std(execution_times):.2f} ms")
        print(f"  Min: {min(execution_times)} ms")
        print(f"  Max: {max(execution_times)} ms")
        print(f"  25th percentile: {np.percentile(execution_times, 25):.1f} ms")
        print(f"  75th percentile: {np.percentile(execution_times, 75):.1f} ms")
    else:
        print("  No execution time data available")

def main():
    parser = argparse.ArgumentParser(description='Analyze Freecell solver benchmark results (simple version)')
    parser.add_argument('--input', '-i', default='../benchmark_summary.json',
                       help='Path to benchmark JSON file (default: ../benchmark_summary.json)')
    parser.add_argument('--output', '-o', default='plots',
                       help='Output directory for plots (default: plots)')
    parser.add_argument('--stats-only', action='store_true',
                       help='Only show statistics, no plots')
    
    args = parser.parse_args()
    
    try:
        # Load data
        print(f"Loading benchmark data from: {args.input}")
        data = load_benchmark_data(args.input)
        
        # Extract metrics
        execution_times, move_counts = extract_metrics(data['results'])
        
        # Print statistics
        print_statistics(execution_times, move_counts, data.get('summary'))
        
        # Create plots if requested
        if not args.stats_only:
            print(f"\nGenerating plots...")
            create_simple_plots(execution_times, move_counts, args.output)
        
        print("\nAnalysis complete!")
        
    except FileNotFoundError:
        print(f"Error: Could not find file '{args.input}'")
        print("Make sure the benchmark_summary.json file exists in the parent directory")
    except json.JSONDecodeError as e:
        print(f"Error: Invalid JSON format in '{args.input}': {e}")
    except Exception as e:
        print(f"Error: {e}")

if __name__ == "__main__":
    main()

#!/usr/bin/env python3
"""
Benchmark Data Sampler

This script provides a quick preview of the benchmark data structure 
and basic statistics without requiring matplotlib.
"""

import json
import sys
from pathlib import Path
from typing import List, Dict, Any

def load_benchmark_data(json_path: str) -> Dict[str, Any]:
    """Load benchmark data from JSON file."""
    with open(json_path, 'r') as f:
        return json.load(f)

def extract_metrics(results: List[Dict[str, Any]]) -> tuple:
    """Extract execution times and move counts from results."""
    execution_times = []
    move_counts = []
    complete_entries = []
    
    # Seeds to exclude from analysis
    excluded_seeds = {11982, 19975, 8893}
    excluded_count = 0
    
    for result in results:
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
                continue
        
        # Extract execution time (only present in entries with complete data) - ensure it's not None
        if 'execution_time_ms' in result and result['execution_time_ms'] is not None:
            try:
                exec_time = float(result['execution_time_ms'])
                execution_times.append(exec_time)
            except (ValueError, TypeError):
                continue
        
        # Complete entries have all fields and valid data
        if (all(key in result for key in ['seed', 'solved', 'execution_time_ms', 'move_count']) and
            result['move_count'] is not None and result['execution_time_ms'] is not None):
            complete_entries.append(result)
    
    print(f"Excluded {excluded_count} seeds from analysis: {excluded_seeds}")
    return execution_times, move_counts, complete_entries

def simple_stats(data: List[float]) -> Dict[str, float]:
    """Calculate basic statistics without numpy."""
    if not data:
        return {}
    
    # Filter out None values
    clean_data = [x for x in data if x is not None]
    if not clean_data:
        return {}
    
    sorted_data = sorted(clean_data)
    n = len(sorted_data)
    
    mean = sum(clean_data) / n
    median = sorted_data[n // 2] if n % 2 == 1 else (sorted_data[n // 2 - 1] + sorted_data[n // 2]) / 2
    
    # Calculate standard deviation
    variance = sum((x - mean) ** 2 for x in clean_data) / n
    std_dev = variance ** 0.5
    
    # Quartiles
    q1_idx = n // 4
    q3_idx = 3 * n // 4
    q1 = sorted_data[q1_idx]
    q3 = sorted_data[q3_idx]
    
    return {
        'mean': mean,
        'median': median,
        'std_dev': std_dev,
        'min': min(clean_data),
        'max': max(clean_data),
        'q1': q1,
        'q3': q3,
        'count': n
    }

def print_data_preview(results: List[Dict[str, Any]], limit: int = 10):
    """Print a preview of the data structure."""
    print("=== DATA PREVIEW ===")
    print(f"Showing first {min(limit, len(results))} entries:\n")
    
    for i, result in enumerate(results[:limit]):
        print(f"Entry {i + 1}:")
        for key, value in result.items():
            print(f"  {key}: {value}")
        print()

def print_detailed_statistics(execution_times: List[float], move_counts: List[int], 
                            complete_entries: List[Dict], summary: Dict[str, Any] = None):
    """Print comprehensive statistics."""
    print("=== COMPREHENSIVE BENCHMARK ANALYSIS ===\n")
    
    # Summary from JSON
    if summary:
        print("OVERALL SUMMARY:")
        print(f"  Total games: {summary.get('total_games', 'N/A')}")
        print(f"  Solved games: {summary.get('solved_games', 'N/A')}")
        print(f"  Failed games: {summary.get('failed_games', 'N/A')}")
        if summary.get('solved_games') and summary.get('total_games'):
            success_rate = summary['solved_games'] / summary['total_games'] * 100
            print(f"  Success rate: {success_rate:.3f}%")
        if 'average_time_ms' in summary:
            print(f"  Average execution time: {summary['average_time_ms']:.2f} ms")
        if 'timeout_secs' in summary:
            print(f"  Timeout threshold: {summary['timeout_secs']} seconds")
        print()
    
    # Data availability
    print("DATA AVAILABILITY:")
    print(f"  Entries with move_count: {len(move_counts)}")
    print(f"  Entries with execution_time_ms: {len(execution_times)}")
    print(f"  Complete entries (all fields): {len(complete_entries)}")
    print()
    
    # Move count statistics
    if move_counts:
        move_stats = simple_stats(move_counts)
        print("MOVE COUNT ANALYSIS:")
        print(f"  Count: {move_stats['count']}")
        print(f"  Mean: {move_stats['mean']:.2f} moves")
        print(f"  Median: {move_stats['median']:.2f} moves")
        print(f"  Standard Deviation: {move_stats['std_dev']:.2f}")
        print(f"  Range: {move_stats['min']} - {move_stats['max']} moves")
        print(f"  Interquartile Range: {move_stats['q1']:.1f} - {move_stats['q3']:.1f}")
        print()
        
        # Move count distribution
        print("MOVE COUNT DISTRIBUTION:")
        ranges = [
            (0, 99, "Very Few Moves (0-99)"),
            (100, 119, "Few Moves (100-119)"),
            (120, 139, "Average Moves (120-139)"),
            (140, 159, "Many Moves (140-159)"),
            (160, 199, "Very Many Moves (160-199)"),
            (200, float('inf'), "Exceptional Moves (200+)")
        ]
        
        for min_val, max_val, label in ranges:
            count = sum(1 for x in move_counts if min_val <= x <= max_val)
            percentage = count / len(move_counts) * 100
            print(f"  {label}: {count} games ({percentage:.1f}%)")
        print()
    
    # Execution time statistics
    if execution_times:
        time_stats = simple_stats(execution_times)
        print("EXECUTION TIME ANALYSIS:")
        print(f"  Count: {time_stats['count']}")
        print(f"  Mean: {time_stats['mean']:.2f} ms")
        print(f"  Median: {time_stats['median']:.2f} ms")
        print(f"  Standard Deviation: {time_stats['std_dev']:.2f} ms")
        print(f"  Range: {time_stats['min']} - {time_stats['max']} ms")
        print(f"  Interquartile Range: {time_stats['q1']:.1f} - {time_stats['q3']:.1f} ms")
        print()
        
        # Execution time distribution
        print("EXECUTION TIME DISTRIBUTION:")
        ranges = [
            (0, 99, "Very Fast (0-99ms)"),
            (100, 199, "Fast (100-199ms)"),
            (200, 499, "Moderate (200-499ms)"),
            (500, 999, "Slow (500-999ms)"),
            (1000, 9999, "Very Slow (1-9.99s)"),
            (10000, float('inf'), "Extremely Slow (10s+)")
        ]
        
        for min_val, max_val, label in ranges:
            count = sum(1 for x in execution_times if min_val <= x <= max_val)
            percentage = count / len(execution_times) * 100
            print(f"  {label}: {count} games ({percentage:.1f}%)")
        print()
    
    # Difficulty analysis (if we have both metrics)
    if execution_times and move_counts and len(complete_entries) > 10:
        print("DIFFICULTY ANALYSIS:")
        
        # Find games that took longest time
        sorted_by_time = sorted(complete_entries, key=lambda x: x.get('execution_time_ms', 0), reverse=True)
        print("  Most time-consuming games:")
        for i, entry in enumerate(sorted_by_time[:5]):
            print(f"    {i+1}. Seed {entry.get('seed', 'N/A')}: {entry.get('execution_time_ms', 0)} ms, {entry.get('move_count', 0)} moves")
        
        # Find games that required most moves
        sorted_by_moves = sorted(complete_entries, key=lambda x: x.get('move_count', 0), reverse=True)
        print("  Games requiring most moves:")
        for i, entry in enumerate(sorted_by_moves[:5]):
            print(f"    {i+1}. Seed {entry.get('seed', 'N/A')}: {entry.get('move_count', 0)} moves, {entry.get('execution_time_ms', 0)} ms")
        print()

def main():
    json_file = '../benchmark_summary.json'
    
    if len(sys.argv) > 1:
        json_file = sys.argv[1]
    
    try:
        print(f"Loading benchmark data from: {json_file}")
        data = load_benchmark_data(json_file)
        
        # Extract metrics
        execution_times, move_counts, complete_entries = extract_metrics(data['results'])
        
        # Show data preview
        print_data_preview(data['results'], 5)
        
        # Print comprehensive statistics
        print_detailed_statistics(execution_times, move_counts, complete_entries, data.get('summary'))
        
        print("=== ANALYSIS COMPLETE ===")
        print("\nTo generate visual plots, install matplotlib and numpy, then run:")
        print("  python3 analyze_benchmark_simple.py")
        print("or")
        print("  python3 analyze_benchmark.py")
        
    except FileNotFoundError:
        print(f"Error: Could not find file '{json_file}'")
        print("Make sure the benchmark_summary.json file exists in the parent directory")
        print("\nUsage: python3 preview_data.py [json_file]")
    except json.JSONDecodeError as e:
        print(f"Error: Invalid JSON format: {e}")
    except Exception as e:
        print(f"Error: {e}")

if __name__ == "__main__":
    main()

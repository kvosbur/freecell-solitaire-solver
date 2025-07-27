#!/usr/bin/env python3
"""
Check for the problematic entry
"""

import json

def find_problem():
    with open('../benchmark_summary.json', 'r') as f:
        data = json.load(f)
    
    results = data['results']
    print(f"Total entries: {len(results)}")
    
    # Find entries without move_count
    for i, result in enumerate(results):
        if 'move_count' not in result or result['move_count'] is None:
            print(f"Entry {i} missing or None move_count: {result}")
    
    # Check the last few entries
    print("\nLast 5 entries:")
    for i, result in enumerate(results[-5:], len(results)-5):
        print(f"Entry {i}: {result}")

if __name__ == "__main__":
    find_problem()

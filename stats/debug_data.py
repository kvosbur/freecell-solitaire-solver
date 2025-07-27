#!/usr/bin/env python3
"""
Quick test to debug the data extraction issue
"""

import json

def quick_test():
    print("Testing data extraction...")
    
    try:
        with open('../benchmark_summary.json', 'r') as f:
            data = json.load(f)
        
        results = data['results']
        print(f"Total entries: {len(results)}")
        
        # Check the first few entries
        print("\nFirst 5 entries:")
        for i, result in enumerate(results[:5]):
            print(f"Entry {i}: {result}")
        
        # Check some entries around where user selected (line 139822)
        # This would be around entry index in the results array
        print(f"\nChecking entry structure around middle of data...")
        mid_idx = len(results) // 2
        for i in range(max(0, mid_idx-2), min(len(results), mid_idx+3)):
            print(f"Entry {i}: {results[i]}")
        
        # Count how many have move_count vs execution_time_ms
        move_count_entries = 0
        exec_time_entries = 0
        none_move_counts = 0
        none_exec_times = 0
        
        for result in results:
            if 'move_count' in result:
                if result['move_count'] is not None:
                    move_count_entries += 1
                else:
                    none_move_counts += 1
            
            if 'execution_time_ms' in result:
                if result['execution_time_ms'] is not None:
                    exec_time_entries += 1
                else:
                    none_exec_times += 1
        
        print(f"\nData summary:")
        print(f"Entries with move_count: {move_count_entries}")
        print(f"Entries with None move_count: {none_move_counts}")
        print(f"Entries with execution_time_ms: {exec_time_entries}")
        print(f"Entries with None execution_time_ms: {none_exec_times}")
        
    except Exception as e:
        print(f"Error: {e}")
        import traceback
        traceback.print_exc()

if __name__ == "__main__":
    quick_test()

#!/usr/bin/env python3
"""
Plot game metrics from CSV to visualize when manhattan distance
and shortest path diverge.
"""

import pandas as pd
import matplotlib.pyplot as plt
import sys

def plot_metrics(csv_file='game_metrics.csv'):
    # Read the CSV
    df = pd.read_csv(csv_file)
    
    # Create figure with subplots
    fig, axes = plt.subplots(2, 2, figsize=(14, 10))
    
    # Plot 1: Player 0 metrics
    ax1 = axes[0, 0]
    ax1.plot(df['turn'], df['p0_manhattan'], label='Manhattan Distance', marker='o', markersize=3)
    ax1.plot(df['turn'], df['p0_shortest_path'], label='Shortest Path (BFS)', marker='s', markersize=3)
    ax1.set_xlabel('Turn Number')
    ax1.set_ylabel('Distance to Goal')
    ax1.set_title('Player 0: Manhattan vs Shortest Path')
    ax1.legend()
    ax1.grid(True, alpha=0.3)
    
    # Plot 2: Player 1 metrics
    ax2 = axes[0, 1]
    ax2.plot(df['turn'], df['p1_manhattan'], label='Manhattan Distance', marker='o', markersize=3)
    ax2.plot(df['turn'], df['p1_shortest_path'], label='Shortest Path (BFS)', marker='s', markersize=3)
    ax2.set_xlabel('Turn Number')
    ax2.set_ylabel('Distance to Goal')
    ax2.set_title('Player 1: Manhattan vs Shortest Path')
    ax2.legend()
    ax2.grid(True, alpha=0.3)
    
    # Plot 3: Divergence (shortest - manhattan)
    ax3 = axes[1, 0]
    df['p0_divergence'] = df['p0_shortest_path'] - df['p0_manhattan']
    df['p1_divergence'] = df['p1_shortest_path'] - df['p1_manhattan']
    ax3.plot(df['turn'], df['p0_divergence'], label='Player 0 Divergence', marker='o', markersize=3)
    ax3.plot(df['turn'], df['p1_divergence'], label='Player 1 Divergence', marker='s', markersize=3)
    ax3.set_xlabel('Turn Number')
    ax3.set_ylabel('Shortest Path - Manhattan')
    ax3.set_title('Divergence: Shows Wall Impact')
    ax3.legend()
    ax3.grid(True, alpha=0.3)
    ax3.axhline(y=0, color='k', linestyle='--', alpha=0.3)
    
    # Plot 4: Walls placed over time
    ax4 = axes[1, 1]
    ax4.plot(df['turn'], df['p0_walls_placed'], label='Player 0 Walls', marker='o', markersize=3)
    ax4.plot(df['turn'], df['p1_walls_placed'], label='Player 1 Walls', marker='s', markersize=3)
    ax4.set_xlabel('Turn Number')
    ax4.set_ylabel('Walls Placed')
    ax4.set_title('Wall Placement Over Time')
    ax4.legend()
    ax4.grid(True, alpha=0.3)
    
    plt.tight_layout()
    plt.savefig('metrics_plot.png', dpi=150)
    print(f"Plot saved to metrics_plot.png")
    plt.show()
    
    # Print statistics
    print("\n=== Statistics ===")
    print(f"Total turns: {len(df)}")
    print(f"\nPlayer 0:")
    print(f"  Max divergence: {df['p0_divergence'].max():.1f}")
    print(f"  Avg divergence: {df['p0_divergence'].mean():.1f}")
    print(f"  Walls placed: {df['p0_walls_placed'].iloc[-1]}")
    print(f"\nPlayer 1:")
    print(f"  Max divergence: {df['p1_divergence'].max():.1f}")
    print(f"  Avg divergence: {df['p1_divergence'].mean():.1f}")
    print(f"  Walls placed: {df['p1_walls_placed'].iloc[-1]}")

if __name__ == '__main__':
    csv_file = sys.argv[1] if len(sys.argv) > 1 else 'game_metrics.csv'
    plot_metrics(csv_file)

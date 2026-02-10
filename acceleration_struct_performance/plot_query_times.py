import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns
import os

def plot_query_times():
    data = []

    # Process Octree average times file
    octree_file = 'octree_avg_query_times.csv'
    if os.path.exists(octree_file):
        df_octree = pd.read_csv(octree_file)
        for _, row in df_octree.iterrows():
            data.append({'num_agents': row['n'], 'avg_query_time': row['time'], 'structure': 'Octree'})
    else:
        print(f"Warning: {octree_file} not found.")

    # Process SpatialHash average times file
    spatial_hash_file = 'spatial_hash_avg_query_times.csv'
    if os.path.exists(spatial_hash_file):
        df_spatial_hash = pd.read_csv(spatial_hash_file)
        for _, row in df_spatial_hash.iterrows():
            data.append({'num_agents': row['n'], 'avg_query_time': row['time'], 'structure': 'SpatialHash'})
    else:
        print(f"Warning: {spatial_hash_file} not found.")

    if not data:
        print("No data found to plot. Please ensure the Rust simulation has been run to generate the data.")
        return

    df_plot = pd.DataFrame(data)
    
    # Sort by num_agents for a clear plot
    df_plot = df_plot.sort_values(by='num_agents')

    sns.set_style("whitegrid")
    plt.figure(figsize=(12, 7))
    sns.lineplot(data=df_plot, x='num_agents', y='avg_query_time', hue='structure', marker='o')
    plt.title('Average Query Time vs. Number of Agents')
    plt.xlabel('Number of Agents')
    plt.ylabel('Average Query Time (nanoseconds)')
    plt.xscale('log') # Log scale for better visualization of N
    plt.yscale('log') # Log scale for query times
    plt.grid(True, which="both", ls="--", c='0.7')
    plt.tight_layout()
    plt.savefig('query_times_performance.png')
    print("Plot saved to 'query_times_performance.png'")

if __name__ == '__main__':
    plot_query_times()

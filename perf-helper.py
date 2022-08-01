#%% Setup

import os
import pandas as pd
import matplotlib.pyplot as plt

exe = "./target/release/rust-wasm-landlock"

file_simple = 'perf-results/execution-times-simple.csv'
file_medium = 'perf-results/execution-times-medium.csv'

stats_simple = 'perf-results/stats-simple.csv'
stats_medium = 'perf-results/stats-medium.csv'

graph_simple = 'perf-results/graph_simple.png'
graph_simple_c = 'perf-results/graph_simple_c.png'
graph_medium = 'perf-results/graph_medium.png'
graph_medium_c = 'perf-results/graph_medium_c.png'

cols = ['args_parsing','module_init','preopen','landlock','running','args_parsing_c','module_init_c','preopen_c','landlock_c','running_c']
col_names = ['Argument parsing', 'Module initialization', 'Preopen', 'Landlock enforcement', 'Running WASM binary']

wasm_bins = {
    file_simple: './wasm-bin/hello.wasm',
    file_medium: './wasm-bin/rw-file.wasm'
}

#%% Init files

for file in [file_simple, file_medium]:
    with open(file, 'w') as f:
        row = ','.join(cols)
        f.write(f'{row}\n')

for file in [stats_simple, stats_medium]:
    with open(file, 'w') as f:
        f.write('type,mean,stddev,meanc,stddevc\n')

#%% Getting all data

print('Doing file simple...')
os.environ['FILE'] = file_simple
for _ in range(100):
    os.system(f'{exe} {wasm_bins[file_simple]}')

print('Doing file medium...')
os.environ['FILE'] = file_medium
for _ in range(100):
    os.system(f'{exe} {wasm_bins[file_medium]} --mapdir="tmp-dir:." --fs-allow="tmp-dir:R,W"')

print('Ok')

#%% Getting statistics

def write_stats(file, out):
    df = pd.read_csv(file)
    with open(out, 'a') as f:
        for col, col_name in zip(cols, col_names):
            mean = df[col].mean()
            std = df[col].std()
            meanc = df[f'{col}_c'].mean()
            stdc = df[f'{col}_c'].std()
            f.write('{},{:0.2f},{:0.2f},{:0.2f},{:0.2f}\n'.format(col_name,mean,std,meanc,stdc))

write_stats(file_simple, stats_simple)
write_stats(file_medium, stats_medium)

# %% Graphs

def graphs_for(file, out, out_c):
    df = pd.read_csv(file)
    xs = list(reversed(['A', 'B', 'C', 'D', 'E']))

    ys = list(reversed(df['mean']))
    plt.figure()
    plt.barh(xs, ys)
    plt.savefig(out)

    ys = list(reversed(df['meanc']))
    plt.figure()
    plt.barh(xs, ys)
    plt.savefig(out_c)

graphs_for(stats_simple, graph_simple, graph_simple_c)
graphs_for(stats_medium, graph_medium, graph_medium_c)

# %%

#%% Setup

import os
import pandas as pd
import matplotlib.pyplot as plt
import numpy as np

permissions_tests = [
    'R,W',
    'R,W,X',
    'R,W,X,D',
    'R,W,X,D,RDir',
    'R,W,X,D,RDir,DDir',
    'R,W,X,D,RDir,DDir,MChar',
    'R,W,X,D,RDir,DDir,MChar,MDir',
    'R,W,X,D,RDir,DDir,MChar,MDir,MReg',
    'R,W,X,D,RDir,DDir,MChar,MDir,MReg,MSock',
    'R,W,X,D,RDir,DDir,MChar,MDir,MReg,MSock,MFifo',
    'R,W,X,D,RDir,DDir,MChar,MDir,MReg,MSock,MFifo,MBlock',
    '*',
]

folder_tests = [
    ['tmp-dir'],
    ['tmp-dir', 'tmp-dir/subdir1'],
    ['tmp-dir', 'tmp-dir/subdir1', 'tmp-dir/subdir2'],
    ['tmp-dir', 'tmp-dir/subdir1', 'tmp-dir/subdir2', 'tmp-dir/subdir3'],
    ['tmp-dir', 'tmp-dir/subdir1', 'tmp-dir/subdir2', 'tmp-dir/subdir3', 'tmp-dir/subdir4'],
    ['tmp-dir', 'tmp-dir/subdir1', 'tmp-dir/subdir2', 'tmp-dir/subdir3', 'tmp-dir/subdir4', 'tmp-dir/subdir5'],
]

#%% Permission tests

# for test in permissions_tests:
#     l = len(test.split(','))
#     file = f'perf-results/landlock-impact-{l}.csv'
#     os.environ['FILE'] = file
#     with open(file, 'w') as f:
#         f.write('args_parsing,module_init,preopen,landlock,running,args_parsing_c,module_init_c,preopen_c,landlock_c,running_c\n')

#     for _ in range(100):
#         os.system(f'cargo run -r -- ./wasm-bin/program-complex.wasm --mapdir="tmp-dir:." --fs-allow="tmp-dir:{test}"')

# #%% Folder tests

# for folders in folder_tests:
#     l = len(folders)
#     file = f'perf-results/landlock-impact-folders-{l}.csv'
#     os.environ['FILE'] = file
#     with open(file, 'w') as f:
#         f.write('args_parsing,module_init,preopen,landlock,running,args_parsing_c,module_init_c,preopen_c,landlock_c,running_c\n')

#     exe = 'cargo run -r -- ./wasm-bin/program-complex.wasm --mapdir="tmp-dir:.'
#     allows = ''
#     for folder in folders:
#         allows += f'--fs-allow "{folder}:R,W" '
#     allows = allows[:-1]
#     for _ in range(100):
#         os.system(f'{exe} {allows}')


#%% Permission graphs

xs = list(range(2, 14))
means = []
all_times = []
for x in xs:
    file = f'perf-results/landlock-impact-{x}.csv'
    df = pd.read_csv(file)
    all_times.append(df['landlock'])
    means.append(df['landlock'].mean())

b, a = np.polyfit(xs, means, deg=1)

plt.figure()
plt.ylim(10, 30)
plt.scatter(xs, means, )
plt.plot(xs, [a + b * x for x in xs], color='C1')
plt.xlabel('Number of active permissions')
plt.ylabel('Time (µs)')
plt.legend(('Average time', 'Linear Regression line'))
plt.savefig('perf-results/landlock-impact.png')

plt.figure()
plt.boxplot(all_times, labels=xs, showfliers=False)
plt.xlabel('Number of active permissions')
plt.ylabel('Time (µs)')
plt.savefig('perf-results/landlock-impact-box.png')

#%% Folder graphs

xs = list(range(len(folder_tests[0]), len(folder_tests[-1]) + 1))
means = []
all_times = []
for x in xs:
    df = pd.read_csv(f'perf-results/landlock-impact-folders-{x}.csv')
    all_times.append(df['landlock'])
    means.append(df['landlock'].mean())

b, a = np.polyfit(xs, means, deg=1)

plt.figure()
plt.scatter(xs, means)
plt.plot(xs, [a + b * x for x in xs], color='C1')
plt.xlabel('Number of rules considered')
plt.ylabel('Time (µs)')
plt.legend(('Average time', 'Linear Regression line'))
plt.savefig('perf-results/landlock-impact-folders.png')

plt.figure()
plt.boxplot(all_times, labels=xs, showfliers=False)
plt.xlabel('Number of rules considered')
plt.ylabel('Time (µs)')
plt.savefig('perf-results/landlock-impact-folders-box.png')

# %%

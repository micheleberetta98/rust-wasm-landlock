#%% Setup

import os
import pandas as pd
import matplotlib.pyplot as plt
import numpy as np

tests = [
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

#%% Tests

for test in tests:
    l = len(test.split(','))
    file = f'perf-results/landlock-impact-{l}.csv'
    os.environ['FILE'] = file
    with open(file, 'w') as f:
        f.write('args_parsing,module_init,preopen,landlock,running,args_parsing_c,module_init_c,preopen_c,landlock_c,running_c\n')

    for _ in range(100):
        os.system(f'cargo run -r -- ./wasm-bin/program-complex.wasm --mapdir="tmp-dir:." --fs-allow="tmp-dir:{test}"')

#%% Data

xs = list(range(2, 14))
means = []
all_times = []
for x in xs:
    file = f'perf-results/landlock-impact-{x}.csv'
    df = pd.read_csv(file)
    all_times.append(df['landlock'])
    means.append(df['landlock'].mean())

#%% Graphs

b, a = np.polyfit(xs, means, deg=1)

plt.figure()
plt.plot(xs, means)
plt.plot(xs, [a + b * x for x in xs])
plt.xlabel('Number of active permissions')
plt.ylabel('Time (ns)')
plt.legend(('Average time', 'Linear Regression line'))
plt.savefig('perf-results/landlock-impact.png')

plt.figure()
plt.boxplot(all_times, showfliers=False)
plt.xlabel('Number of active permissions')
plt.ylabel('Time (ns)')
plt.savefig('perf-results/landlock-impact-box.png')

# %%

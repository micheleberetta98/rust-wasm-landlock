# %%
import pandas as pd
import numpy as np
import matplotlib.pyplot as plt

#%% Access graphs

xs = list(range(2, 10))
means = []
all_times = []
df = pd.read_csv('access.csv')
for x in xs:
    all_times.append(df[str(x)])
    means.append(df[str(x)].mean())

b, a = np.polyfit(xs, means, deg=1)

plt.figure()
plt.scatter(xs, means)
plt.plot(xs, [a + b * x for x in xs], color='C1')
plt.xlabel('Number of active permissions')
plt.ylabel('Time (µs)')
plt.legend(('Average time', 'Linear Regression line'))
plt.savefig('ebpf-impact.png')

plt.figure()
plt.boxplot(all_times, labels=xs, showfliers=False)
plt.xlabel('Number of active permissions')
plt.ylabel('Time (µs)')
plt.savefig('ebpf-impact-box.png')

#%% Folder graphs

xs = list(range(1, 7))
means = []
all_times = []
df = pd.read_csv('dir.csv')
for x in xs:
    all_times.append(df[str(x)])
    means.append(df[str(x)].mean())

b, a = np.polyfit(xs, means, deg=1)

plt.figure()
plt.scatter(xs, means)
plt.plot(xs, [a + b * x for x in xs], color='C1')
plt.xlabel('Number of rules considered')
plt.ylabel('Time (µs)')
plt.legend(('Average time', 'Linear Regression line'))
plt.savefig('ebpf-impact-folders.png')

plt.figure()
plt.boxplot(all_times, labels=xs, showfliers=False)
plt.xlabel('Number of rules considered')
plt.ylabel('Time (µs)')
plt.savefig('ebpf-impact-folders-box.png')

# %%

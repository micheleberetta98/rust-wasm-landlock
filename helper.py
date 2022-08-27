import os
import subprocess

num_runs = 100
policies_access = [f'policy-d1-p{i}.yml' for i in range(2, 10)]
policies_dir = [f'policy-d{i+1}.yml' for i in range(6)]

def setup():
    for p in policies_access + policies_dir:
        os.system(f'sudo cp ./policies/{p} /var/lib/bpfcontain/policy')


def run_test(policy):
    result = subprocess.check_output(f'bpfcontaintest run {policy}', shell=True, text=True)
    time = int(result.split('\n')[0])
    return time
    
def tests_access():
    values = [[i for i in range(2, 10)]]
    for _ in range(num_runs):
        times = [run_test(p) for p in policies_access]
        values.append(times)
    
    return values

def tests_dir():
    values = [[i+1 for i in range(6)]]
    for _ in range(num_runs):
        times = [run_test(p) for p in policies_dir]
        values.append(times)
    
    return values

def save(values, fname):
    with open(fname, 'w') as f:
        for row in values:
            line = ','.join(str(x) for x in row)
            f.write(f'{line}\n')

if __name__ == '__main__':
    setup()

    input('Please start the server and press ENTER...')

    save(tests_access(), 'results/access.csv')
    save(tests_dir(), 'results/dir.csv')


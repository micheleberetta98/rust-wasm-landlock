import os

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

for test in tests:
    l = len(test.split(','))
    file = f'perf-results/landlock-impact-{l}.csv'
    os.environ['FILE'] = file
    with open(file, 'w') as f:
        f.write('args_parsing,module_init,preopen,landlock,running,args_parsing_c,module_init_c,preopen_c,landlock_c,running_c\n')

    for _ in range(100):
        os.system(f'cargo run -r -- ./wasm-bin/program-complex.wasm --mapdir="tmp-dir:." --fs-allow="tmp-dir:{test}"')

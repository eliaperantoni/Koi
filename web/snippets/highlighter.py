import re
import glob
import os

conv = {
    1: 'kw',
    2: 'str',
    3: 'fn',
    4: 'int'
}

for path in glob.glob('*.koi'):
    with open(path, 'r') as f:
        koi = f.read()

    while True:
        match = re.search('°(\d)[^°]*°', koi)
        if match is None:
            break

        t = int(match.group(1))
        a,b = match.span()

        koi = koi[:a] + f'<span class="{conv[t]}">{koi[a+2:b-1]}</span>' + koi[b:]

    with open(os.path.join('out', path), 'w') as f:
        f.write(koi)

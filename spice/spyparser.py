#!/usr/bin/env python3

import sys
import json
import csv

phc = []
with open("simbodiespc") as file:
    phcf = csv.reader(file, delimiter="\t")
    for line in phcf:
        phc.append(line)

results = []


spkid = None
while line := sys.stdin.readline():
    if 'Sample STATE Results' in line:
        spkid = None
    if 'Target' in line:
        (_, spkid) = line.split(':')
        spkid = spkid.strip()
    if '2018-04-06 00:00:00' in line:
        values = [spkid]
        parts = line.split(' ')
        for part in parts:
            if len(part) != 0 and part not in ['2018-04-06','00:00:00']:
                f = float(part) * 1000
                values.append(str(f))

        values = '\t'.join(values)
        n = [spkid, f"{values}"]
        results.append(n)

results = sorted(results, key=lambda x:x[0])

for (i, r) in enumerate(results):
    print(r[1])



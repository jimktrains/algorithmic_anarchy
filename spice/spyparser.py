#!/usr/bin/env python3

import sys
import json


with open('api.le-systeme-solaire.net/rest/bodies/index.json') as f:
    meta = json.load(f)

results = []

name = None
while line := sys.stdin.readline():
    if 'Sample STATE Results' in line:
        name = None
    if 'Target' in line:
        (_, name) = line.split(':')
        name = name.strip()
    if '2018-04-06 00:00:00' in line:
        values = []
        parts = line.split(' ')
        for part in parts:
            if len(part) != 0 and part not in ['2018-04-06','00:00:00']:
                f = float(part) * 1000
                values.append(str(f))

        for p in meta['bodies']:
            if p['id'].upper() == name.upper() or p['englishName'].upper() == name.upper():
                if p['mass'] is not None:
                    values.append(f"{p['mass']['massValue']}e{p['mass']['massExponent']}")
                break
        values = ','.join(values)
        around = p['aroundPlanet']
        if around is not None:
            around = around['planet']
        n = [name, p['id'], around, f"{values}"]
        results.append(n)

def planet_index(pid):
    planets = [
            'soleil',
            'mercure',
            'venus',
            'terre',
            'mars',
            'jupiter',
            'saturne',
            'uranus',
            'neptune',
            'pluton',
            ]

    try:
        return planets.index(pid) * 10
    except ValueError:
        return 999

def sort_key(r):
    pid = r[1]
    offset = 0
    if r[2] is not None:
        pid = r[2]
        offset = 1
    key= planet_index(pid) + offset
    return key
   
results = sorted(results, key=sort_key)

for (i, r) in enumerate(results):
    print(f"// {i: 3} {r[0]} ({r[1]})", end='')
    if r[2] is not None:
        print(f" (around {r[2]})", end='')
    print("")
    print(f"[{r[3]}],")



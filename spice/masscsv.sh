jq  -r '.bodies.[] | select(.mass.massValue != null) | "\(.id),\(.englishName),\(.mass.massValue)e\(.mass.massExponent),\(.aroundPlanet.planet)"' api.le-systeme-solaire.net/rest/bodies/index.json | grep -v "S/" | sed 's/,[0-9]\+ /,/g' | grep -v " " > masses.csv


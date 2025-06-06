grep -Rh RADII kernels/pck |sed 's/^\s\+\(BODY\([0-9]\+\)_RADII\s\+=\s\+(\s\+\([0-9.]\+\).*\)/\2\t\3/' |sort -nu

# It's actually a diameter field :\
csvtool col 1,6 -u TAB small_bodies.csv | grep -v spkid >> radii


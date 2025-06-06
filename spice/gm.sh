grep -Rh "_GM " kernels/pck |sed 's/^\s\+\(BODY\([0-9]\+\)_GM\s\+=\s\+(\s\+\([-0-9.E+]\+\).*\)/\2\t\3/' |sort -n 
csvtool col 1,10 -u TAB small_bodies.csv | grep -v spkid >> gm



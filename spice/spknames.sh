ls kernels/spk/* | xargs -n 1 python -m jplephem spk |  sed 's/.*\s\+\(.*\) (\([0-9]\+\))\s\+->\s\+\(.*\) (\([0-9]\+\))/\2\t\1\n\4\t\3/'  | grep -v '\.\.' | grep -v "File type" | LC_COLLATE=c sort -u > spknames
csvtool col 1,4 -u TAB small_bodies.csv | grep -v spkid >> spknames


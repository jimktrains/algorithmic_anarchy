join -t $'\t' <(join  -t $'\t'  spknames gm ) radii | grep -v "^2000...\s" | awk '{print NR-1 " " $0}' > simbodiespc




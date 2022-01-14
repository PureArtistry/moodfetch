#!/bin/sh

# this is being run as root so the usual variables can't be used here
# edit this to the absolute path to your cache folder
cache_path=/home/dave/.cache

n=$(pacman -Qn --color never | wc -l)
m=$(pacman -Qm --color never | wc -l)
[ -d ${cache_path}/moodfetch ] || mkdir -p ${cache_path}/moodfetch
echo "${n} ${m}" > ${cache_path}/moodfetch/pkg_stats

return 0

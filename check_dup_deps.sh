#!/usr/bin/env bash

total_list=$(mktemp)
package_versions=$(mktemp -d)

cargo tree | sed -E 's!\s+\(.*\)!!g' | rev | cut -d ' ' -f 1,2 | rev | sort | uniq > ${total_list}

dup_list=$(
    cat $total_list |
    cut -d ' ' -f 1 |
    uniq -c         |
    sort -n -r      |
    sed -E 's/^\s+//g' |
    grep -vE '1 ' |
    cut -d ' ' -f 2
)

for crate in $dup_list
do
    current_versions=$(grep -E "^${crate}\s" $total_list | cut -d ' ' -f 2)
    package_dir="${package_versions}/${crate}"
    mkdir -p $package_dir 

    for current_version in $current_versions
    do  touch ${package_dir}/${current_version}
    done
done

(
    cd ${package_versions}
    LANG=C ls -laht */* | rev | cut -d ' ' -f 1 | rev
)
rm -rf ${total_list} ${package_versions}

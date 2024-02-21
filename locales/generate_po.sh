#!/bin/sh

function custom_finder(){
    local keyword=$1
    shift 1
    local ids=( "${@}" )
    local pos="#:"
    for file in $(find .. -type f -name '*.rs')
    do
        match=$(cat -n $file |
          sed 's/^\s*//g' | 
          grep 'gettext(&\?'"${keyword}"
        )
        if [ $? == 0 ]
        then
            pos+=" ${file}:$(cut -f 1 <<< $match)"
        fi
    done

    for id in ${ids[@]}
    do
        echo "$pos"
        echo msgid "\"${id}\""
        echo msgstr "\"\""
        echo
    done
}

package_name=$(
    cargo metadata --no-deps --format-version 1  |
    jq .packages[].name -r)
    
version=$(
    cargo metadata --no-deps --format-version 1 |
    jq .packages[].version -r)

outfile=${package_name}.pot 

xgettext -o ${outfile} \
  --package-name ${package_name} \
  --package-version ${version} \
  $(find ../src/ -type f -name '*.rs') &> /dev/null

echo >> ${outfile}

custom_finder 'display_mode' \
  show_both show_both_rev origin prefer_translation \
  >> ${outfile}

custom_finder 'lyric_align' \
  Center Start End Fill \
  >> ${outfile}
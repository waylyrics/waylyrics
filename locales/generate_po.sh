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
  $(find ../src/ -type f -name '*.rs')

echo >> ${outfile}

pos="#:"
for file in $(find .. -type f -name '*.rs')
do
    match=$(cat -n $file |
      sed 's/^\s*//g' | 
      grep 'gettext(&\?display_mode')
    if [ $? == 0 ]
    then
        pos+=" ${file}:$(cut -f 1 <<< $match)"
    fi
done

for id in show_both show_both_rev prefer_translation origin
do
    echo "$pos"
    echo msgid "\"${id}\""
    echo msgstr "\"\""
    echo
done >> ${outfile}

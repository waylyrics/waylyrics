package_name=$(
    cargo metadata --no-deps --format-version 1  | \
    jq .packages[].name -r)
    
version=$(
    cargo metadata --no-deps --format-version 1 | \
    jq .packages[].version -r)

xgettext -o ${package_name}.pot \
  --package-name ${package_name} \
  --package-version ${version} \
  $(find ../src/ -type f -name '*.rs')

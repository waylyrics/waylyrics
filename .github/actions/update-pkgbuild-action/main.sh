#!/usr/bin/bash

# main.sh

if [[ "$INPUT_TAG" =~ v[0-9]+\.[0-9]+\.[0-9]+ ]]
then
    echo "Applying tag $INPUT_TAG to PKGBUILD..."
    pkgver=${INPUT_TAG//v/}
    if [[ -n "$INPUT_DIR" ]] && [[ ! "$INPUT_DIR" =~ ^\.+$ ]]
    then
        cd "$INPUT_DIR" || exit
    fi
    echo "Updating pkgver in PKGBUILD..."
    sed -i "s/pkgver=.*/pkgver=$pkgver/" PKGBUILD
    echo "Updating checksums in PKGBUILD..."
    OLDPWD=$PWD
    mkdir /tmp/makepkg
    chown -R builder:builder /tmp/makepkg
    su builder -c "cp -r . /tmp/makepkg"
    cd /tmp/makepkg || exit
    su builder -c updpkgsums
    echo "Updating .SRCINFO..."
    su builder -c "makepkg --printsrcinfo" > .SRCINFO
    cd "$OLDPWD" || exit
    cp /tmp/makepkg/PKGBUILD PKGBUILD
    cp /tmp/makepkg/.SRCINFO .SRCINFO
    echo "pkgver=$pkgver" >> "$GITHUB_OUTPUT"
else
    echo "Invalid tag $INPUT_TAG"
    exit 1
fi

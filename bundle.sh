#!/usr/bin/env sh
set -eu

APPNAME="createac"

# only call build.sh if the file was not built yet
if [ ! -f ./dist/release/createac.exe ]; then
    ./build.sh release
fi
mkdir -p ./dist/js-dos
# add the exe, assets, dosbox.conf and CWSDPMI.EXE into new zip file
cd bundle
rm -f dos_createac.zip
cp ../dist/release/$APPNAME.exe ./
zip -q dos_createac.zip \
    CWSDPMI.EXE \
    $APPNAME.exe \
    .jsdos/dosbox.conf
rm -f $APPNAME.exe

# rename it as dos_createac.jsdos
cp dos_createac.zip ../dist/js-dos/dos_createac.jsdos
rm dos_createac.zip
echo "Created bundle dist/js-dos/dos_createac.jsdos"

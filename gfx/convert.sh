#!/usr/bin/env bash
# Quantize all PNGs in current folder to 16 colors + alpha
# Requires: ImageMagick (magick command)

shopt -s nullglob
for f in *.png; do
    echo "Processing $f"
    magick "$f" -colors 16 -type TrueColorMatte PNG32:"$f"
done
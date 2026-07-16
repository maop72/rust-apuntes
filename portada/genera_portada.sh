#!/bin/bash
inkscape portada.svg \
    --export-width=2480 \
    --export-filename=../portada.png


inkscape portada.svg \
    --export-filename=../portada.pdf

#!/usr/bin/env bash

ROOT="inputs"
OUTDIR="expected"

[[ ! -d "$OUTDIR" ]] && mkdir -p "$OUTDIR"

EMPTY="$ROOT/empty.txt"
FOX="$ROOT/fox.txt"
SPIDERS="$ROOT/spiders.txt"
BUSTLE="$ROOT/the-bustle.txt"
ALL="$EMPTY $FOX $SPIDERS $BUSTLE"

for FILE in $ALL; do
    BASENAME=$(basename "$FILE")
    cat    $FILE > ${OUTDIR}/${BASENAME}.out
    cat -n $FILE > ${OUTDIR}/${BASENAME}.n.out
    cat -b $FILE > ${OUTDIR}/${BASENAME}.b.out
done

cat    $ALL > $OUTDIR/all.txt.out
cat -n $ALL > $OUTDIR/all.txt.n.out
cat -b $ALL > $OUTDIR/all.txt.b.out

cat    < $BUSTLE > $OUTDIR/$(basename $BUSTLE).stdin.txt.out
cat -n < $BUSTLE > $OUTDIR/$(basename $BUSTLE).stdin.txt.n.out
cat -b < $BUSTLE > $OUTDIR/$(basename $BUSTLE).stdin.txt.b.out
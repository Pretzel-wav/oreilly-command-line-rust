#!/usr/bin/env bash

OUTDIR="tests/expected"
[[ ! -d "$OUTDIR" ]] && mkdir -p "$OUTDIR"

# skips_bad_file
cat inputs/fox.txt > $OUTDIR/skips_bad_file.txt

# reads_stdin_by_default
ls inputs | cat > $OUTDIR/reads_stdin_by_default.txt

# reads_dash_as_stdin`
ls inputs | cat - > $OUTDIR/reads_dash_as_stdin.txt

# numbers_lines
cat -n inputs/the-bustle.txt > $OUTDIR/numbers_lines.txt

# skips_blank_lines
cat -b inputs/the-bustle.txt > $OUTDIR/skips_blank_lines.txt

# concatenates_files
cat inputs/the-bustle.txt inputs/fox.txt inputs/spiders.txt > $OUTDIR/concatenates_files.txt
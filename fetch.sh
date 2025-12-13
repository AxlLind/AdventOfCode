#!/bin/bash
set -euo pipefail

fail() { echo "$@" && exit 1; }

if [[ $# != 2 ]]; then
  fail "usage: $0 YEAR DAY"
fi
year=$1
day=$2

if [[ ! "$year" =~ ^20(1[5-9]|2[0-9])$ ]]; then
  fail "Not a valid year: $year"
fi
if [[ ! "$day" =~ ^(0[1-9]|1[0-9]|2[0-5])$ ]]; then
  fail "Not a valid day: $day"
fi
if [[ -z "${AOC_SESSION-""}" ]]; then
  fail "\$AOC_SESSION not set"
fi

TMPFILE=$(mktemp)
trap 'rm -f "$TMPFILE"' EXIT

curl "https://adventofcode.com/$year/day/${day#0}/input"       \
  -s --fail-with-body --cookie "session=$AOC_SESSION"          \
  -A "Bash script at $(git remote -v | awk 'NR==1{print $2}')" \
  | tee "$TMPFILE"

outdir=$(realpath "$(dirname "$0")")/$year/inputs
mkdir -p "$outdir"
mv "$TMPFILE" "$outdir/$day.in"

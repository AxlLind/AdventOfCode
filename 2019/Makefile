DAYS := 01 02 03 04 05 06 07 08 09 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25
.DEFAULT_GOAL := all
.PHONY: $(DAYS) all

inputs/%.in:
	../fetch.sh 2019 $*

src/bin/%.rs:
	DAY=$* envsubst < src/template.rs > $@

$(DAYS): %: src/bin/%.rs inputs/%.in
	cargo run --quiet --release --bin $*

all: $(patsubst %,inputs/%.in,$(DAYS))
	cargo run --quiet --release

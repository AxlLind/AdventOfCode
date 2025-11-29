YEARS   := $(wildcard 20*)
YEAR    := $(lastword $(YEARS))
DAYS    := 01 02 03 04 05 06 07 08 09 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25
TARGETS := $(foreach year,$(YEARS),$(addprefix $(year)-,$(DAYS)) $(year)-all)
TODAY   := $(shell TZ=America/New_York date +%Y%m%d)
.PHONY: $(DAYS) $(TARGETS) help

# if today is an AOC-day set it as the default goal
.DEFAULT_GOAL := $(or $(filter $(TODAY:$(YEAR)12%=%),$(DAYS)),help)

$(DAYS): %: $(YEAR)-%

$(TARGETS):
	@$(MAKE) --no-print-directory -C $(subst -, ,$@)

help:
	@echo 'usage: make [TARGET..]'
	@echo 'Automatically downloads input, sets up files, and runs solutions.'
	@echo
	@echo 'TARGET:'
	@echo '  DAY       run a specific day from $(YEAR) (e.g 07)'
	@echo '  YEAR-DAY  run a specific day           (e.g 2018-09)'
	@echo '  YEAR-all  run all days                 (e.g 2022-all)'
	@echo "During the AoC month just 'make' will run the current day's"

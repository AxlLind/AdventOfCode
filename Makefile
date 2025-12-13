get_days = $(shell head -n1 $(1)/Makefile | cut -f 3- -d ' ')

YEARS     := $(wildcard 20*)
YEAR      := $(lastword $(YEARS))
YEAR_DAYS := $(call get_days,$(YEAR))
TARGETS   := $(foreach year,$(YEARS),$(addprefix $(year)-,$(call get_days,$(year))) $(year)-all)
TODAY     := $(shell TZ=America/New_York date +%Y%m%d)

# if today is an AOC-day set it as the default goal
.DEFAULT_GOAL := $(or $(filter $(TODAY:$(YEAR)12%=%),$(YEAR_DAYS)),help)
.PHONY: $(YEAR_DAYS) $(TARGETS) help

$(TARGETS):
	@$(MAKE) --no-print-directory -C $(subst -, ,$@)

help:
	@echo 'usage: make [TARGET..]'
	@echo 'Automatically downloads input and runs solutions, e.g:'
	@echo '  make 2025-03'
	@echo
	@echo 'TARGET:'
	@echo '  YEAR-DAY  run a specific day   (e.g 2025-09)'
	@echo '  YEAR-all  run all days         (e.g 2022-all)'
	@echo "During the AoC month just 'make' will run the current day's"

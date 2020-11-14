# Complete Makefile based of gitlab.com/C0balt/oxidized-cms wth slight modifications

# Allow command line args to be passed instead of serial execution of steps
# Based off https://stackoverflow.com/a/14061796 
ifeq (db-migration,$(firstword $(MAKECMDGOALS)))
  RUN_ARGS := $(wordlist 2,$(words $(MAKECMDGOALS)),$(MAKECMDGOALS))
  $(eval $(RUN_ARGS):;@:)
endif

setup: 
	./scripts/server-setup.sh
db-setup:
	./scripts/diesel.sh
.PHONY: db-migration
db-migration:
	./scripts/diesel.sh migration $(RUN_ARGS)
db-print:
	./scripts/diesel.sh print-schema
db-reset:
	./scripts/diesel.sh reset
serve:
	cargo b --release 
	./target/release/pentagame serve
build:
	cargo b --release 
	cd static/ &&  yarn run compile
generate:
	cargo b --release 
	./target/release/pentagame generate
dev-generate:
	cargo b
	./target/debug/pentagame generate
dev-build:
	cargo b 
	cd static/ && yarn run compile
ci-build:
	cargo build --verbose
ci-test:
	cargo check --verbose
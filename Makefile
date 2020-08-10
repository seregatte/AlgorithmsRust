SHELL:=/bin/sh 

all: run_test

run_test:
	@cargo test

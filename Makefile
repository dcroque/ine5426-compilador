.RECIPEPREFIX = >

.PHONY: test
test: 
> cargo test

# make run filename=src/examples/code1.lcc
.PHONY: run
run: 
> cargo run -- -f $(filename)

.PHONY: example1
run: 
> cargo run -- -f src/examples/code1.lcc

.PHONY: help
help: 
> cargo run -- --help
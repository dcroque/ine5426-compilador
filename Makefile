SHELL := /bin/bash

# Colors
CCGREEN=\033[0;32m
CCYELLOW=\033[0;33m
CCRED=\033[0;31m
CCEND=\033[0m

.RECIPEPREFIX = >

filename=src/examples/code1.lcc
RUSTFLAGS="$$RUSTFLAGS -A dead_code"

.PHONY: test
test: 
> RUSTFLAGS=$(RUSTFLAGS) cargo test

.PHONY: run_all
run_all: 
> @echo -e "${CCGREEN}Testando todos os testes atualmente definidos${CCEND}"
> RUSTFLAGS=$(RUSTFLAGS) cargo run -- -f src/examples/code1.lcc
> RUSTFLAGS=$(RUSTFLAGS) cargo run -- -f src/examples/code2.lcc
> RUSTFLAGS=$(RUSTFLAGS) cargo run -- -f src/examples/code3.lcc
> @echo -e "${CCGREEN}codigos 1, 2 e 3 não possuem nenhum erro léxico.${CCEND}"

.PHONY: run
run: 
> @echo -e "${GREEN}Checando validade de código segundo a definição da linguagem${CCEND}"
> @echo -e "Se voce nao especificou um programa fonte"
> @echo -e "um arquivo de exemplo sera utilizado."
> @echo -e "Voce pode muda-lo utilizando o comando ${CCYELLOW}make run filename=<path/to/file>${CCEND}"
> RUSTFLAGS=$(RUSTFLAGS) cargo run -- -f $(filename)
> @echo -e "${CCGREEN}$(filename) nao possui operacoes invalidas.${CCEND}"

.PHONY: clean
clean:
> cargo clean

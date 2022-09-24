# ine5426-compilador

## INSTRUÇÕES

Entrega:
- `make run` espera um argumento de filename, e testa se o conteudo desse arquivo pertence à linguagem (ex: `make example filename=src/examples/code1.lcc`).
- `make example` roda um exemplo fixo ou passado por argumento (ex: `make example src=<path/to/source_code>`).

Extra:
- `make clean` as vezes pode ser necessário ao mudar a gramática, ou processos de build (uso esperado apenas por desenvolvedores, provavelmente não é necessário para quem nao estiver alterando o código fonte)

## Exemplos
Diversos exemplos da linguagem estão na pasta `src/examples`, cada pasta procura reunir exemplos para mostrar diversas partes do compilador funcionando ou dando erro como esperado.

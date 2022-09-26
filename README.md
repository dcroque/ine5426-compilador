# ine5426-compilador

## INSTRUÇÕES

- `make run_all` roda o programa em todos os exemplos de código atualmente no projeto (code1.lcc, code2.lcc e code3.lcc).
- `make run` espera um argumento de filename, e testa se o conteudo desse arquivo pertence à linguagem (ex: `make run filename=src/examples/code1.lcc`).
- `make test` roda a bateria de testes interna do projeto.

Extra:
- `make clean` as vezes pode ser necessário ao mudar a gramática, ou processos de build (uso esperado apenas por desenvolvedores, provavelmente não é necessário para quem nao estiver alterando o código fonte)

## Exemplos
Diversos exemplos da linguagem estão na pasta `src/examples`, cada pasta procura reunir exemplos para mostrar diversas partes do compilador funcionando ou dando erro como esperado.

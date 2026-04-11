# Comum Protocol

O Comum e uma infraestrutura para produzir memoria publica, autoridade local
auditavel e relacao federativa entre mundos que permanecem diferentes.

Ele parte de uma aposta simples e exigente: coordenacao legitima nao precisa
nascer de um centro irrevisavel. Pode nascer de testemunhos locais,
verificaveis, referenciaveis e interpretados por instituicoes situadas, desde
que o sistema preserve limite, reprodutibilidade e responsabilidade publica.

O atomo do protocolo e o `Testimony`. O resto e composicao:

- grafo causal local
- serializacao canonica
- suites criptograficas
- sync eventual
- prova de contexto modular
- capsulas com semantica local

O Comum nao elimina autoridade. Ele tenta impedir que autoridade apareca como
instancia opaca, superior e sem rastro. Seu horizonte e outro: tornar delegacao,
contestacao, revogacao e memoria publica legiveis sem reduzir a vida coletiva a
plataforma, centro ou medida unica.

## O que existe hoje

- especificacao normativa viva via CIPs, registries, CDDL e vetores
- implementacao de referencia forte em Rust
- SDK TypeScript/React Native em consolidacao de paridade
- capsulas de referencia para governanca, troca local e mutirao
- experimentos reais de topologia distribuida e convergencia

## Como ler o repositorio

- teoria e visao: `docs/corpus/`
- resumo condensado do projeto: `ABSTRACT.md`
- especificacao normativa: `spec/cips/CIP-0001.md`
- registries e contratos publicos: `spec/registries/`, `spec/schemas/`, `spec/test-vectors/`
- arquitetura e implementacao: `docs/implementation/`
- contexto operacional do projeto: `docs/project/`
- laboratorio vivo: `docs/notes/`

## Principios curtos

- abstracao suficiente para coordenar, nao para colonizar
- interoperabilidade sem equivalencia unica
- federacao sem centro soberano
- determinismo tecnico sem captura da legitimidade
- autoridade situada, limitada e revogavel

## Comecar

```sh
just test
```

Ou manual:

```sh
cargo test
cd impl/comum-js && npm run build && npm test
```

## Licenca

CC0 1.0 Universal (dominio publico)

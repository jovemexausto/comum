# Comum Protocol

## Manifesto

O Comum Protocol e infraestrutura de autodeterminacao comunitaria. Parte da
ideia de que coordenacao legitima nasce no territorio e nas pessoas, nao em
plataformas. Ele torna possivel existir, decidir e trocar valor sem depender
de conectividade continua, nem de mediacao estatal ou corporativa.

Chamamos isso de "subsidiariedade tecnica": o poder de coordenacao retorna
as pessoas no nivel local, com regras publicas e limites legiveis. E uma
recusa concreta a captura da atencao, dos dados, do territorio e do tempo.

A unidade atomica e o Testemunho: uma afirmacao local, verificavel e assinada.
O resto e disciplina publica, reprodutivel e auditavel, porque o poder precisa
de limites, e os limites precisam ser claros.

- A rede e humana antes de ser tecnica.
- A confianca e local antes de ser global.
- O tempo e offline-first antes de ser always-on.
- O contexto e vivido antes de ser medido.
- A governanca e do comum antes de ser da plataforma.
- A autonomia vem antes da escala.
- A infraestrutura deve servir a vida, nao o contrario.
- A soberania e pratica, nao slogan.

Nao buscamos consenso global obrigatorio. Buscamos convergencia local,
auditavel e reprodutivel, com garantias claras, limites explicitos e
responsabilidade comunitaria. O Comum nao elimina autoridade; ele permite
autoridade local limitada, contestavel e revogavel sem dependencia de um
centro irrevisavel.

## Recusas

- Nao aceitamos a inevitabilidade de plataformas como destino politico.
- Nao aceitamos que privacidade seja luxo ou excecao.
- Nao aceitamos que comunidade seja variavel de escala.
- Nao aceitamos que infraestrutura seja um privilegio de quem tem capital.

## Compromissos

- Construir regras claras, simples e auditaveis.
- Operar sob limites explicitos, nao sob promessas vagas.
- Garantir interoperabilidade por meio de especificacao publica.
- Priorizar presenca local, cuidado comunitario e autonomia material.
- Manter o protocolo aberto, sem pedagios, sem cadastro, sem vigilancia.
- Tornar delegacao, contestacao e revogacao de autoridade legiveis no grafo.

## Principios

- Minimal surface: o protocolo tem um unico primitivo, o Testemunho.
- Determinismo: o runtime e WASM, com limites normativos.
- Interop: CIPs, registries, CDDL e test-vectors obrigatorios.
- Sem GPS: Prova de Contexto e modular, sem dependencia global.
- Portabilidade: implementacoes de referencia em Rust e JS.

## O que e

Offline-first. Local-first. O Comum e um protocolo para existir, decidir e
trocar valor sem depender de conectividade continua, sem intermediarios
centrais e sem captura de dados.

Aqui, o minimo e o maximo importam: um unico primitivo (`Testimony`) e uma
disciplina publica que torna o sistema verificavel, auditavel e interoperavel.

Comum Protocol e um sistema de coordenacao soberana baseado em Testemunhos
assinados, um grafo de contexto e execucao deterministica em WASM. Ele existe
para reduzir dependencia de infraestrutura central e permitir operacao em
condicoes de baixa conectividade e alta autonomia local.

## Caracteristicas-chave

- Offline-first real: opera com latencia alta e bandwidth baixa
- Determinismo: CBOR canonical, hashing e ordenacao normativos
- Interop: CIPs + registries + CDDL + vetores oficiais
- Prova de contexto modular (sem GPS obrigatorio)
- Runtime WASM com limites normativos
- Capsulas com semantica local (governanca, mercado, mutirao)
- Fundacao comunitaria minima com 2 pessoas; triades e coletivos maiores
  aumentam robustez institucional

## Arquitetura em uma linha

`Testimony + DAG + CBOR canonical + Suites criptograficas + CTE + Sync + Capsule WASM`

## Documentacao

- Especificacao principal: `spec/cips/CIP-0001.md`
- CIPs adicionais: `spec/cips/`
- Registries normativos: `spec/registries/`
- Vetores oficiais: `spec/test-vectors/`
- Interoperabilidade: `docs/interop/README.md`
- Teoria do protocolo: `docs/teoria-do-protocolo.md`
- Roadmap: `docs/roadmap.md`

## Implementacoes de referencia

- Rust (runtime/ABI/utilitarios): `impl/comum-rs/`
- JS (API de alto nivel): `impl/comum-js/`
- Capsulas (WASM, semantica local): `impl/capsulas/`
- Simulacoes: `impl/simulations/`

## Comecar rapido

```sh
just test
```

Ou manual:

```sh
cargo test
cd impl/comum-js && npm run build && npm test
```

## Wrapper JS com N-API (opcional)

`comum-js` usa N-API quando disponivel (fallback automatico para `comum-cbor`).

```sh
cargo build -p comum-napi
```

```sh
export COMUM_NAPI_PATH=/caminho/para/comum-napi.node
```

Se `COMUM_NAPI_PATH` nao estiver definido, o `comum-js` tenta resolver
`comum-napi` pelo Node e, em seguida, usa `comum-cbor` como fallback.

## Simulacoes

- Agora (governanca):

```sh
just agora-sim
```

- Feira (mercado local):

```sh
just feira-sim
```

- Mutirao (trabalho coletivo):

```sh
just mutirao-sim
```

## Qualidade e verificacao

- Vetores oficiais garantem paridade entre linguagens
- Conformance runner em `tests/conformance/run.js`
- Testes Rust e JS sao tratados como contrato

## Licenca

CC0 1.0 Universal (dominio publico)

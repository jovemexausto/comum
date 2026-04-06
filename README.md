# Comum Protocol

## Manifesto

O Comum Protocol é infraestrutura de autodeterminação comunitária. Parte da
ideia de que coordenação legítima nasce no território e nas pessoas, não em
plataformas. Ele torna possível existir, decidir e trocar valor sem depender
de conectividade contínua, nem de mediação estatal ou corporativa.

Chamamos isso de "subsidiariedade técnica": o poder de coordenação retorna
às pessoas no nível local, com regras públicas e limites legíveis. É uma
recusa concreta à captura da atenção, dos dados, do território e do tempo.

A unidade atômica é o Testemunho: uma afirmação local, verificável e assinada.
O resto é disciplina pública, reprodutível e auditável, porque o poder precisa
de limites, e os limites precisam ser claros.

- A rede é humana antes de ser técnica.
- A confiança é local antes de ser global.
- O tempo é offline-first antes de ser always-on.
- O contexto é vivido antes de ser medido.
- A governança é do comum antes de ser da plataforma.
- A autonomia vem antes da escala.
- A infraestrutura deve servir à vida, não o contrário.
- A soberania é prática, não slogan.

Não buscamos consenso global. Buscamos convergência local, auditável e
reprodutível, com garantias claras, limites explícitos e responsabilidade
comunitária. A recusa do centro não é uma metáfora: é um requisito técnico
e uma escolha ética.

## Recusas

- Não aceitamos a inevitabilidade de plataformas como destino político.
- Não aceitamos que privacidade seja luxo ou exceção.
- Não aceitamos que comunidade seja variável de escala.
- Não aceitamos que infraestrutura seja um privilégio de quem tem capital.

## Compromissos

- Construir regras claras, simples e auditáveis.
- Operar sob limites explícitos, não sob promessas vagas.
- Garantir interoperabilidade por meio de especificação pública.
- Priorizar presença local, cuidado comunitário e autonomia material.
- Manter o protocolo aberto, sem pedágios, sem cadastro, sem vigilância.

## Princípios

- Minimal surface: o protocolo tem um único primitivo, o Testemunho.
- Determinismo: o runtime é WASM, com limites normativos.
- Interop: CIPs, registries, CDDL e test-vectors obrigatórios.
- Sem GPS: Prova de Contexto é modular, sem dependência global.
- Portabilidade: implementações de referência em Rust e JS.

## Arquitetura (em uma frase)

Testimony + DAG + CBOR canonical + Suites criptográficas + Envelope + Sync +
Capsule WASM = coordenação soberana.

## Estrutura do repo

- spec/cips: CIPs normativos (inclui CIP-0001)
- spec/registries: registries oficiais (verbs, suites, envelopes)
- spec/schemas: CDDL e schemas CBOR
- spec/test-vectors: vetores oficiais
- impl: implementações de referência (comum-rs, comum-js, capsules)
- tests/conformance: suíte de conformidade
- docs: governança, segurança e planos

## Começar rápido

```sh
just test
```

Ou manual:

```sh
cargo test
cd impl/comum-js && npm run build && npm test
```

## Documentos-chave

- spec/cips/CIP-0001.md
- GOVERNANCE.md
- SECURITY.md

Licença: CC0 1.0 Universal (domínio público)

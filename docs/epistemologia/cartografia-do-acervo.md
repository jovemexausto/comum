# Cartografia do acervo

Objetivo: mapear o corpus atual do Comum por tipo de documento, publico
principal, status editorial e destino desejado no futuro livro/whitepaper
modular.

## Legenda

- Categoria:
  - manifesto
  - normativo
  - arquitetura
  - implementacao
  - teoria
  - economia-politica
  - exploratorio
  - governanca
  - release/planejamento
- Status:
  - ativo
  - maduro
  - precisa atualizar
  - precisa mover
  - precisa fundir
  - legado

## Diagnostico rapido

O acervo ja possui densidade suficiente para deixar de ser apenas uma colecao
de docs dispersos. O problema atual nao e falta de material, e falta de
arquitetura editorial.

Os principais sintomas sao:

- notas fortes ainda sem lugar canonico no corpus
- mistura entre teoria, arquitetura, contrato tecnico e exploracao
- arquivos com valor historico coexistindo com SSOTs sem marcacao editorial forte
- lacunas de costura entre manifesto, teoria, economia politica e implementacao

## 1. Camada de entrada e orientacao

| Arquivo | Categoria | Publico principal | Status | Destino sugerido |
| --- | --- | --- | --- | --- |
| `README.md` | manifesto | publico geral, devs, comunidades | ativo | manter como porta de entrada manifesto/productish |
| `ABSTRACT.md` | manifesto/tecnico | leitores novos, pesquisadores, implementadores | ativo | manter como compressao precisa do estado e do escopo do repo |
| `CONTRIBUTING.md` | governanca | contribuidores | ativo | manter, mas expandir com guia editorial futuro |
| `GOVERNANCE.md` | governanca | mantenedores, comunidade | ativo | manter em camada institucional |
| `SECURITY.md` | governanca | mantenedores, auditores | ativo | manter |
| `CODE_OF_CONDUCT.md` | governanca | comunidade | ativo | manter |

## 2. Nucleo normativo

| Arquivo | Categoria | Publico principal | Status | Destino sugerido |
| --- | --- | --- | --- | --- |
| `spec/cips/CIP-0001.md` | normativo | implementadores, revisores de protocolo | maduro | manter como SSOT central |
| `spec/cips/CIP-0002.md` | normativo | implementadores SDK/runtime | ativo | manter |
| `spec/cips/CIP-0003.md` | normativo | implementadores de capsula | ativo | manter |
| `spec/registries/README.md` | normativo | implementadores | ativo | manter |
| `spec/registries/verbs.md` | normativo | implementadores | ativo | manter |
| `spec/registries/context-types.md` | normativo | implementadores | ativo | manter |
| `spec/registries/transport-profiles.md` | normativo | implementadores | ativo | manter |
| `spec/registries/cte-types.md` | normativo | implementadores | ativo | manter |
| `spec/registries/suites.md` | normativo | implementadores | ativo | manter |
| `spec/schemas/testimony.cddl` | normativo | implementadores, validares | ativo | manter |
| `spec/schemas/sync.cddl` | normativo | implementadores, validares | ativo | manter |
| `spec/schemas/cte.cddl` | normativo | implementadores, validares | ativo | manter |
| `spec/schemas/snapshot.cddl` | normativo | implementadores, validares | ativo | manter |
| `spec/test-vectors/README.md` | normativo | implementadores | ativo | manter |
| `spec/test-vectors/manifest.json` + `vector-0001..0006.json` | normativo | implementadores, CI | ativo | manter como artefato normativo, nao capitulo textual |

## 3. RFCs e material historico de transicao

| Arquivo | Categoria | Publico principal | Status | Destino sugerido |
| --- | --- | --- | --- | --- |
| `.archive/spec/00-rfcs/00-RFC-0001-v0.1.md` | legado | historiadores do projeto | arquivado | manter fora do corpus vivo |
| `.archive/spec/00-rfcs/01-RFC-0001-v0.2.md` | legado | historiadores do projeto | arquivado | manter fora do corpus vivo |
| `.archive/spec/00-rfcs/02-RFC-0001-v0.3.md` | legado | historiadores do projeto | arquivado | manter fora do corpus vivo |
| `.archive/spec/00-rfcs/03-RFC-0002-Commoner-Facade.md` | arquitetura | implementadores | arquivado | absorver no corpus tecnico se ainda relevante |
| `.archive/spec/00-rfcs/04-RFC-0003-Agora-Feira-e-Mutirao.md` | arquitetura | implementadores, designers de capsula | arquivado | alinhar com CIP-0003 e docs de capsula |
| `.archive/spec/00-rfcs/05-RFC-0004-Capsulas-Flexiveis-e-VM-Deterministica.md` | arquitetura | implementadores runtime | arquivado | alinhar com CIP-0001 e docs de ABI |

## 4. Teoria do protocolo e fundamento conceitual

| Arquivo | Categoria | Publico principal | Status | Destino sugerido |
| --- | --- | --- | --- | --- |
| `docs/corpus/` | teoria | leitores conceituais, designers de sistema | maduro | concentrar o corpus teorico do projeto |
| `docs/epistemologia/README.md` | teoria | mantenedores, editores do corpus | ativo | manter como mapa epistemologico mestre |
| `docs/epistemologia/plano-de-reestruturacao.md` | teoria | mantenedores, editores do corpus | ativo | manter como plano editorial |
| `docs/epistemologia/cartografia-do-acervo.md` | teoria | mantenedores, editores do corpus | ativo | manter como diagnostico editorial vivo |

## 5. Arquitetura e implementacao

| Arquivo | Categoria | Publico principal | Status | Destino sugerido |
| --- | --- | --- | --- | --- |
| `docs/architecture/overview.md` | arquitetura | implementadores, novos contribuidores | precisa expandir | virar indice de arquitetura tecnica |
| `docs/architecture/wasm-abi.md` | arquitetura | implementadores runtime/capsula | ativo | manter |
| `docs/sdk/mobile-contract.md` | implementacao | implementadores SDK/mobile | precisa atualizar | revisar com realidade pos v0.3 e v0.4 |
| `docs/design/README.md` | implementacao | design/UX mobile | precisa expandir | manter como raiz de UX, hoje esta raso |
| `docs/interop/README.md` | implementacao | implementadores, release | precisa atualizar | refletir runtime mobile/WS e paridade pendente |
| `docs/interop/2026-10-12.md` | implementacao | mantenedores | legado/cronica | manter como registro datado |
| `impl/comum-rs/README.md` | implementacao | devs Rust | ativo | manter |
| `impl/comum-js/README.md` | implementacao | devs JS/mobile | ativo | manter |
| `apps/mobile/README.md` | implementacao | devs mobile | ativo | manter |
| `impl/capsulas/README.md` | implementacao | devs de capsula | ativo | manter |
| `impl/capsulas/agora/README.md` | implementacao | devs de capsula | ativo | manter |

## 6. Release, contexto e planejamento

| Arquivo | Categoria | Publico principal | Status | Destino sugerido |
| --- | --- | --- | --- | --- |
| `docs/project/releases/v0.1.md` | release/planejamento | comunidade, mantenedores | ativo | manter |
| `docs/project/releases/v0.2.md` | release/planejamento | comunidade, mantenedores | ativo | manter |
| `docs/project/releases/v0.3.md` | release/planejamento | comunidade, mantenedores | ativo | manter |
| `docs/project/roadmap.md` | release/planejamento | comunidade, mantenedores | ativo | manter |
| `docs/project/implementation-plan.md` | release/planejamento | mantenedores | ativo | manter, como plano operacional |
| `.archive/docs/04-project-ops/00-contexto-de-evolucao.md` | release/planejamento | mantenedores, agentes, onboarding | arquivado | preservar apenas como contexto historico |

## 7. Simulacoes e laboratorios

| Arquivo | Categoria | Publico principal | Status | Destino sugerido |
| --- | --- | --- | --- | --- |
| `docs/simulations.md` | exploratorio | implementadores, pesquisadores | ativo | manter |
| `tests/conformance/README.md` | implementacao | implementadores, CI | ativo | manter |
| `tools/cbor/README.md` | implementacao | devs de tooling | ativo | manter |
| `tools/lint/README.md` | implementacao | devs de tooling | ativo | manter |

## 8. Notas de pesquisa ja maduras o bastante para consolidacao futura

| Arquivo | Categoria | Publico principal | Status | Destino sugerido |
| --- | --- | --- | --- | --- |
| `docs/notes/coherence.md` | exploratorio | pesquisadores, mantenedores | precisa fundir | virar indice do bloco de coerencia |
| `docs/notes/triadic-coherence-comum.md` | exploratorio | pesquisadores | ativo | manter no laboratorio |
| `docs/notes/coercive-masking.md` | exploratorio | pesquisadores | ativo | manter no laboratorio |
| `docs/notes/phi-expectations.md` | exploratorio | pesquisadores | ativo | manter no laboratorio |
| `docs/notes/coherence-findings-001.md` | exploratorio | pesquisadores | ativo | manter no laboratorio |
| `docs/notes/identity-and-sybil.md` | exploratorio | pesquisadores, arquitetos | maduro | possivel capitulo futuro de economia politica/identidade |
| `docs/notes/community-boundaries.md` | teoria/economia-politica | arquitetos, pesquisadores | maduro | possivel capitulo futuro de teoria social do protocolo |
| `docs/notes/dani-to-comum.md` | exploratorio | mantenedores | legado/ponte | manter como nota historica ou arquivar |
| `docs/notes/research-plan-001.md` | exploratorio | mantenedores | legado/planejamento | manter, mas fora da camada principal |
| `docs/notes/dx-api-ergonomics-post-v0.3.md` | implementacao/exploratorio | mantenedores SDK | ativo | possivel base de capitulo de DX/SDK |

## 9. Notas conceituais recentes (alto valor, ainda sem casa canonica)

| Arquivo | Categoria | Publico principal | Status | Destino sugerido |
| --- | --- | --- | --- | --- |
| `docs/notes/brainstorm-inferencia-social.md` | exploratorio | pesquisadores, designers de sistema | ativo | manter no laboratorio exploratorio |
| `docs/notes/triggers-e-camadas.md` | teoria/arquitetura | arquitetos, implementadores | ativo | possivel fundir em teoria + arquitetura |
| `docs/notes/comum-e-hawala.md` | economia-politica | pesquisadores, mantenedores | ativo | possivel capitulo futuro de economia politica do Comum |
| `docs/notes/interoperabilidade-sem-equivalencia-unica.md` | economia-politica | mantenedores, pesquisadores | maduro | forte candidato a capitulo-ponte |
| `docs/notes/sintese-jornada-exploratoria.md` | teoria | mantenedores | ativo | manter como indice de transicao |
| `docs/notes/mapa-de-prioridades-pos-exploracao.md` | release/planejamento | mantenedores | ativo | manter como disciplina interna |

## 10. Lacunas identificadas

Os seguintes documentos ainda nao existem como capitulos deliberados, embora o
conteudo ja tenha emergido nas notas e conversas:

- economia politica do Comum
- limites anti-coloniais do protocolo
- abstracao necessaria vs abstracao totalizante
- remessa/compensacao intercomunitaria
- o que o projeto realmente tem em maos (alcance paradigmatico)
- guia editorial do corpus

## 11. Ordem sugerida de promocao editorial

### Promover primeiro

- `docs/corpus/`
- `docs/notes/interoperabilidade-sem-equivalencia-unica.md`
- `docs/notes/community-boundaries.md`
- `docs/notes/identity-and-sybil.md`

### Fundir ou costurar em seguida

- `docs/notes/triggers-e-camadas.md`
- `docs/architecture/overview.md`
- `docs/sdk/mobile-contract.md`
- `docs/interop/README.md`

### Manter explicitamente como laboratorio

- bloco de coerencia
- `docs/notes/brainstorm-inferencia-social.md`
- `docs/notes/comum-e-hawala.md`
- `docs/notes/sintese-jornada-exploratoria.md`

## 12. Proximo passo editorial

Usar esta cartografia para executar a Fase 2 do plano:

- fixar a arvore editorial mestre
- decidir onde cada grupo de arquivos passa a morar no corpus futuro
- criar um guia editorial curto para evitar nova dispersao

## Status

Cartografia inicial do acervo. Documento vivo.

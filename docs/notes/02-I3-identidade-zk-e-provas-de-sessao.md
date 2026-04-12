---
note_class: "02"
integration_state: "I3"
status: "parcialmente absorvida pela especificacao"
destino: "spec"
rationale: "Consolida decisoes de identidade ZK, commitments e provas de sessao que ja estao convergindo para CIP."
---

# ZK identity: dump de pesquisa e plano

Objetivo: registrar o estado atual da exploracao de identidade ZK no Comum,
preservando decisoes ja tomadas, riscos conhecidos e o plano de fechamento do
draft antes de qualquer mudanca em registry, schema ou implementacao.

## Estado atual

- Nao existe conflito numerico para `CIP-0004` no repo neste momento.
- O nome `ZK Identity` e amplo demais e pode colidir semanticamente com outras
  ideias de identidade privada no futuro.
- Nome recomendado para o draft atual: `ZK Identity Commitments and Session Proofs`.

## Decisoes fechadas

- Compatibilidade total com `claim.verb + claim.payload` da CIP-0001.
- Sem `type:` novo no topo de `Testimony`.
- Novos verbos:
  - `comum/identity_commitment`
  - `comum/auth_nullifier`
  - `comum/identity_vouch`
- HKDF alinhado a CIP-0001: HKDF-SHA3-256.
- Distincao explicita entre:
  - `proof.nullifiers`: anti double-spend do Testemunho base
  - `session_nullifier`: anti-replay de autenticacao
- `proof.zk_proofs` continua `[* bstr]`.
- Se uma prova ZK for carregada em um Testemunho, cada item de
  `proof.zk_proofs` deve ser um `bstr` contendo CBOR canonico de um
  `ZKProofEntry`.

## Ajuste mais importante apos a pesquisa

O desenho inicial acoplava a prova ZK de autenticacao ao registro
`identity_commitment`. Isso cria uma inconsistencia temporal: a prova de sessao
depende de `session_nonce`, mas o commitment e um registro persistente criado
antes da sessao.

Direcao adotada:

- `comum/identity_commitment` = registro persistente no DAG
- prova ZK de sessao = mensagem efemera no fluxo online
- `comum/auth_nullifier` = recibo persistente apos verificacao

## Pesquisa externa resumida

### Semaphore

O desenho do Semaphore reforca tres pontos uteis para o Comum:

- commitment publico separado do segredo
- nullifier derivado de `scope + secret`
- prova de sessao separada do registro persistente da identidade

Isso fortalece a decisao de manter a prova online fora do DAG, e usar o DAG
para commitment, vouch e recibo anti-replay.

### Noir / Barretenberg

- Noir suporta `pub` inputs explicitamente no `main`, entao a ordem dos public
  inputs precisa ser normativa no CIP.
- NoirJS + bb.js tornam plausivel prova local em browser e mobile.
- As fontes encontradas foram suficientes para viabilidade tecnica, mas ainda
  nao sao uma base forte para promessas de benchmark normativo no CIP.

### PASETO v4.local

- Continua sendo uma boa opcao para token de sessao local.
- Deve ser tratado como mecanismo de sessao de aplicacao, nao como parte da
  suite criptografica base do protocolo.

## Ponto de atencao no texto do CIP

- O repo atual define `proof.zk_proofs` como `[* bstr]`.
- Portanto, o CIP deve evitar modelar `ZKProofEntry` como mapa diretamente
  dentro de `proof.zk_proofs`.
- A forma compativel e: `proof.zk_proofs[i] = bstr(CBOR canonical de ZKProofEntry)`.

## Pendencias abertas

### 1. Nome final do CIP

Opcoes razoaveis:

- `ZK Identity Commitments and Session Proofs` (recomendado)
- `Private Identity Commitments`
- `Anonymous Session Proofs for Comum`

### 2. Fluxo offline

Ainda falta decidir se o modo offline deve:

- introduzir um novo verbo, por exemplo `comum/auth_request`; ou
- permitir que um Testemunho carregue a prova ZK em `proof.zk_proofs` como
  pedido assincrono de autenticacao.

Recomendacao: tratar isso como segunda etapa do CIP ou como extensao posterior.
O nucleo do CIP fica menor, mais coerente e mais facil de implementar.

### 3. Extensao de context `vouch`

Ainda esta em aberto se parametros como `min_vouches`, `min_voucher_depth` e
`epoch_duration` devem entrar como extensao do contexto `vouch` neste mesmo CIP
ou em um CIP separado de politica de admissao/contexto.

Recomendacao: manter no texto como politica local, sem schema novo nesta fase.

## Plano de fechamento

1. Fechar o texto normativo de `spec/cips/CIP-0004.md` com foco apenas no fluxo
   online e nos tres verbos principais.
2. Atualizar `spec/registries/verbs.md` quando o texto estiver estabilizado.
3. Atualizar `crates/comum-rs/src/verbs.rs` com novas constantes.
4. Adicionar validacao de payloads e testes minimos em Rust/JS.
5. So depois decidir se o fluxo offline entra no mesmo CIP ou numa extensao.

## Fontes externas usadas

- https://docs.semaphore.pse.dev/
- https://docs.semaphore.pse.dev/glossary
- https://docs.semaphore.pse.dev/technical-reference/circuits
- https://raw.githubusercontent.com/privacy-ethereum/zkspecs/main/specs/3/README.md
- https://noir-lang.org/docs/dev/noir/concepts/data_types
- https://noir-lang.org/docs/dev/noir/concepts/data_types/fields
- https://noir-lang.org/docs/dev/tutorials/noirjs_app
- https://raw.githubusercontent.com/paseto-standard/paseto-spec/master/docs/01-Protocol-Versions/Version4.md

# Feira MVP Packet

Status: ativo

Objetivo: definir o vertical slice principal do Comum no curto prazo e usá-lo
como disciplinador entre teoria, protocolo, SDK, app e evidence.

## Escolha

Vertical slice escolhido: `Feira MVP`.

Razao:

- ja existe semantica capsular de Feira
- ja existe simulacao
- ja existe fluxo E2E `offer -> accept -> receipt`
- o caso e concreto o bastante para orientar produto sem exigir governanca rica

## Fluxo minimo

1. Nodo A cria uma oferta
2. Nodo B recebe a oferta e aceita
3. Nodo A e Nodo B observam receipt sincronizado

O fluxo minimo nao inclui ainda:

- dispute UX rica
- compensacao intercomunitaria
- autoridade delegada
- ZK identity

## Usuario e resultado visivel

Usuario deve conseguir:

- abrir o app
- ver o proprio node ativo
- criar uma oferta
- aceitar uma oferta remota
- ver o receipt final apos sync

Resultado visivel do MVP:

- o Comum deixa de ser apenas protocolo e simulacao e passa a existir como fluxo
  de app compreensivel

## Artefatos impactados

### Produto

- `apps/mobile/`
- `packages/comum-js/`

### Execucao canonica e suporte

- `crates/comum-rs/`
- `capsules/feira/`
- `tests/conformance/`
- `simulations/feira/`

### Documentacao

- `docs/project/implementation-plan.md`
- `docs/project/roadmap.md`
- `docs/implementation/`

## Dependencias da stack de referencia

### Corpus

Da sentido ao que o fluxo representa:

- memoria publica
- legibilidade local
- coordenacao sem centro obrigatorio

### Spec

Define o contrato minimo:

- testemunho
- claim/context/proof
- sync e convergencia
- capsula e envelope

### Implementation

Entrega o fluxo real:

- Rust
- JS/React Native
- capsula Feira

### Evidence

Deve sustentar o MVP com:

- testes
- conformance
- simulacao
- demonstracao manual reproduzivel

## Backlog orientado por fluxo

### 1. Superficie de app

- esconder detalhes de baixo nivel atras de `ComumClient` ou equivalente
- reduzir vazamento de CBOR no boundary do app

### 2. Convergencia observavel

- garantir que o receipt remoto apareca de forma confiavel
- deixar claro quando o estado ainda esta local vs sincronizado

### 3. Paridade relevante

- fechar parity nao no protocolo inteiro de uma vez, mas no fluxo Feira que o
  app realmente expoe

### 4. Demo reproduzivel

- dois nos
- passo a passo curto
- criterio claro de sucesso/falha

## Criterio de pronto

Feira MVP esta pronto quando:

1. duas instancias conseguem completar `offer -> accept -> receipt`
2. o receipt e observavel em ambos os lados
3. o app nao exige conhecimento de CBOR ou runtime interno para operar o fluxo
4. existe um roteiro curto de reproducao manual
5. os checks automatizados relevantes continuam passando

## O que fica explicitamente para depois

- paper como prioridade principal
- governanca rica
- identity/ZK
- autoridade emergente como semantica executavel
- economia intercomunitaria mais forte

Esses temas nao desaparecem; apenas deixam de liderar a cadencia enquanto o
Comum ainda precisa provar a si mesmo como produto vivo.

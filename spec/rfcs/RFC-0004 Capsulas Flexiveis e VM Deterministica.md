# RFC-0004 — Capsulas Flexiveis e VM Deterministica

```
Status: Draft
Data: 2026-04-06
Categoria: Standards Track
Licenca: CC0 1.0 Universal - dominio publico
Repositorio: github.com/comum-protocol/spec
```

## Resumo (Abstract)

Este RFC define um modelo de execucao de capsulas que permite riqueza de apps
sem perder o determinismo, a autonomia local e a interoperabilidade. Propomos
dois niveis: (1) uma DSL total (sempre termina) como base interoperavel e
(2) uma VM Turing-complete (WASM) com gas e memoria limitados como camada
opcional. Inclui restricoes deterministicas e um exemplo completo de capsula
Tamagotchi.

## Motivacao

Comunidades precisam de apps ricos, mas a riqueza nao pode custar auditabilidade
ou convergencia. Sem limites claros, uma capsula pode divergir entre nos,
dependendo de IO, relogio externo ou efeitos colaterais. Este RFC define
um caminho para flexibilidade sem perder a filosofia do protocolo.

## Objetivos

- Permitir capsulas expressivas com determinismo forte.
- Manter interoperabilidade entre implementacoes.
- Evitar dependencia de infraestrutura central.
- Estabelecer limites claros para Turing-complete.

## Nao-objetivos

- Definir UX ou politicas de governanca.
- Substituir CIPs de capsulas existentes.
- Permitir IO externo dentro da capsula.

## Proposta

### 1) Dois niveis de execucao

1. **DSL total (recomendada)**
   - Linguagem funcional/total, sem loops ilimitados.
   - Toda execucao MUST terminar.
   - Foco em interoperabilidade e auditabilidade.

2. **WASM com gas (opcional)**
   - Bytecode Turing-complete com orcamento de gas e memoria limitados.
   - Execucao MUST abortar ao exceder gas/memoria.
   - Exige vetores de teste oficiais e regras de gas normativas.

### 2) Restricoes deterministicas (ambos os niveis)

- Nenhum IO externo: sem rede, sem disco, sem sensores, sem RNG.
- Sem relogio externo: tempo so pode entrar via eventos assinados.
- Toda execucao MUST ser pura: mesmo input => mesmo output.
- Acesso a criptografia apenas via biblioteca padrao deterministica.

### 3) Capabilities declaradas

Cada capsula MUST declarar capacidades usadas, para permitir validacao e
controle local. Exemplo de capabilities:

- `time.logical` (tempo logico via evento)
- `hash.sha3_256`
- `sig.ed25519_verify`

Implementacoes MAY recusar capsulas com capabilities nao suportadas.

### 4) Vetores de teste como contrato

Toda capsula publicada MUST incluir vetores de teste normativos:

- estado inicial
- sequencias de eventos
- estado final esperado

Implementacoes MUST passar nos vetores para declarar conformidade.

### 5) Compatibilidade e extensoes

- Extensoes MUST ser versionadas.
- Uma capsula MUST indicar se requer DSL total ou WASM.
- Implementacoes que nao suportam WASM MUST rejeitar essas capsulas com erro
  explicito.

## Analise de trade-offs

- Determinismo: DSL total > WASM com gas
- Interoperabilidade: DSL total > WASM com gas
- Complexidade/auditoria: DSL total < WASM com gas
- Riqueza de apps: WASM com gas > DSL total
- Evolucao rapida: WASM com gas > DSL total

## Exemplo: Capsula Tamagotchi (deterministica)

Este exemplo segue a filosofia: estado minimo, eventos assinados, sem RNG,
sem relogio externo.

### Estado (CBOR map, chaves tstr)

```
{
  "id":        <bytes32>,
  "phase":     <tstr>,    // egg | baby | teen | adult
  "age":       <uint64>,  // ticks acumulados
  "hunger":    <uint64>,  // 0..100
  "mood":      <uint64>,  // 0..100
  "health":    <uint64>,  // 0..100
  "last_tick": <uint64>   // seq do ultimo tick
}
```

### Acoes (capsule/invoke)

1) `init`

```
{
  "pet_id": <bytes32>,
  "seed":   <bytes32>  // apenas para id, nao para RNG
}
```

2) `feed`

```
{
  "pet_id": <bytes32>,
  "amount": <uint64>   // 1..10
}
```

3) `play`

```
{
  "pet_id": <bytes32>,
  "effort": <uint64>   // 1..10
}
```

4) `sleep`

```
{
  "pet_id": <bytes32>,
  "hours":  <uint64>   // 1..12
}
```

5) `tick`

```
{
  "pet_id":  <bytes32>,
  "seq":     <uint64>, // monotonic
  "elapsed": <uint64>  // minutos desde o ultimo tick
}
```

### Regras deterministicas

- `init` cria estado base com:
  - phase = "egg", age = 0, hunger = 50, mood = 50, health = 100, last_tick = 0
- `feed`: hunger = max(hunger - amount*5, 0)
- `play`: mood = min(mood + effort*4, 100); hunger = min(hunger + effort*2, 100)
- `sleep`: health = min(health + hours*3, 100)
- `tick`:
  - MUST ter seq > last_tick, senao ignorar
  - age += 1
  - hunger = min(hunger + floor(elapsed/30)*3, 100)
  - mood = max(mood - floor(elapsed/60)*2, 0)
  - health = max(health - floor(hunger/25), 0)
  - fase evolui quando age atinge 3/7/14 (egg->baby->teen->adult)

### Resultado (capsule/result)

O campo `result` SHOULD conter o estado completo atualizado em CBOR canonical.

### Vetor de teste minimo (exemplo)

Estado inicial: vazio.

Sequencia:

1. init(pet_id=A, seed=S)
2. tick(seq=1, elapsed=60)
3. feed(amount=4)
4. play(effort=3)

Estado esperado:

```
{
  "id":        A,
  "phase":     "baby",
  "age":       1,
  "hunger":    47,
  "mood":      62,
  "health":    100,
  "last_tick": 1
}
```

## Consideracoes de seguranca

- Gas e memoria limitados sao obrigatorios para WASM.
- Capsulas que dependem de tempo externo MUST ser rejeitadas.
- Vetores de teste reduzem risco de divergencia entre runtimes.

## Questoes em aberto

- DSL total: quais operadores minimos sao suficientes?
- Regras de gas para WASM: tabela de custos e versao.
- Distribuicao e registro de capsulas (CIP separada).

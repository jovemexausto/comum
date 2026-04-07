# Triggers e camadas (nota)

Objetivo: registrar a distincao entre diferentes tipos de "trigger" no
projeto, para evitar confusao entre protocolo, runtime semantico e politica
local de aplicacao.

## Tipos de trigger

### 1. Link triggers

Eventos de transporte que iniciam sincronizacao.

Exemplos:

- BLE connect
- NFC tap
- QR reassembly
- WS connect

Esses triggers pertencem a camada de protocolo/sync.

Referencias:

- `spec/cips/CIP-0001.md` (secao 9.1)
- `spec/registries/transport-profiles.md`
- `impl/comum-rs/src/transport.rs`

### 2. Execution triggers

Verbos que ativam Capsulas.

No manifesto de Capsula:

```json
{
  "triggers": ["capsule/invoke", "comum/vouch"]
}
```

Esses triggers pertencem a camada de runtime semantico.

Referencias:

- `spec/cips/CIP-0001.md` (secao 12.5 e 12.7)

### 3. Policy triggers

Regras locais de app/node que disparam decisoes, UX ou automacoes.

Exemplos:

- ao receber `offer`, decidir se aceita
- ao receber `receipt`, aumentar confianca
- ao conectar transporte, atualizar estado local

Esses triggers pertencem a camada de aplicacao/politica local.

## Distincao recomendada

- Protocolo/sync: sabe sobre link triggers
- Runtime/capsulas: sabe sobre execution triggers
- App/cliente: sabe sobre policy triggers

## Observacao

Usar a mesma palavra "trigger" para as tres camadas pode induzir acoplamento
conceitual. Vale manter essa separacao explicita em docs e API.

## Status

Nota exploratoria. Nao normativa.

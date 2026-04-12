# Feira

Projeto vertical de referencia para troca local no Comum.

Estrutura:

- `app/`: app React Native da Feira
- `capsules/`: capsula local de referencia usada pelo fluxo da Feira
- `sims/`: simulacao Rust do fluxo capsular

SSOT local da semantica da capsula:

- `capsules/capsule.yaml`

Esse arquivo descreve os verbos locais da capsula Feira e seus payloads. Ele nao
deve ser contaminado por backlog de interface nem por detalhes contingentes do
runtime atual.

Dependencias capsulares do slice:

- `capsules.yaml`

Esse arquivo declara quais packages capsulares o projeto Feira usa hoje.

Feira e mantida pelo time do Comum como app/case study do ecossistema. Nao e
parte normativa do protocolo.

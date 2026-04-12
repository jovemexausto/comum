# Agora

Projeto vertical de referencia para governanca local no Comum.

Estrutura atual:

- `capsules/`: capsula de referencia para proposta, voto e fechamento
- `sims/`: simulacao Rust do fluxo da capsula

SSOT local da semantica da capsula:

- `capsules/capsule.yaml`

Esse arquivo descreve os verbos locais da capsula Agora e seus payloads. Ele nao
deve ser contaminado por backlog de interface nem por detalhes contingentes do
runtime atual.

Uma superficie de app dedicada pode ser adicionada depois sem alterar a posicao
de Agora: app/case study mantido pelo time do Comum, nao semantica normativa do
protocolo.

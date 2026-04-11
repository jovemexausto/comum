# DANI -> Comum: Padroes, Distincoes e Possiveis Incorporações

Objetivo: extrair padroes, referencias e ideias transferiveis do DANI
para o Comum, sem contaminar a especificacao com hipoteses externas.

## 1) Padroes transferiveis (alto nivel)

### 1.1 Determinismo operacional

- DANI reforca que determinismo end-to-end e requisito para consenso
  offline. No Comum, isso reforca CBOR canonical + WASM limites.
- Transferivel como principio: *tudo que afeta estado deve ser
  deterministico e auditavel*.

### 1.2 Homeostase (anti-especulacao)

- DANI usa estabilidade (homeostase) para evitar degeneracao sob fluxo
  continuo. No Comum, isso se traduz em *mecanismos de estabilidade
  economica* para moedas locais e reputacao.
- Transferivel como direcao: *mecanismos de amortecimento* em capsulas
  (ex.: limites de oferta, taxas de ajuste, janelas de agregacao).

### 1.3 Prova de conhecimento (PoK) como lastro

- A ideia de PoK aplicada a volume de testemunhos sugere um lastro
  baseado em participacao verificavel, nao em liquidez especulativa.
- Transferivel como hipotese: *valor emergente ancorado em evidencias
  de conhecimento/participacao*, e nao em mercado global.

## 2) Distincoes essenciais (nao transferir como normativo)

- SNARKs, VRFs, SRS e acumuladores nao fazem parte do Comum atual.
- BSP, WAL deterministico e fixed-point sao arquitetura de indexacao
  (DANI), nao do protocolo.
- DANI lida com ANN e alta dimensionalidade; Comum lida com Testimony
  e grafos sociais. Escopos distintos.

## 3) O que pode virar nota tecnica no Comum

### 3.1 Homeostase economica (capsulas)

- Hipotese: introduzir mecanismos de amortecimento em capsulas de
  economia local para evitar especulacao extrema.
- Exemplo: janelas de recalculo de precos, limites de variacao por epoca,
  ou curvas de emissao com feedback local.

### 3.2 PoK de participacao

- Hipotese: criar um indicador de participacao (volume de Testimonies
  validos) como proxy de conhecimento/compromisso local.
- Uso: ajustar parametros de governanca ou limites de emissao.

## 4) Referencias que valem trazer para o Comum

- CRDTs / Strong Eventual Consistency: Shapiro et al. (2011)
- Lamport / Vector Clocks (causalidade sem relogio global)
- SybilLimit / SybilGuard (filtro de confianca local)
- MMR e SMT (snapshots eficientes)

## 5) Riscos se importar sem filtro

- Misturar pesquisa de indexacao (DANI) com protocolo social (Comum)
  gera especificacao fantasma.
- Adicionar PoK/VRF como normativo sem implementacao testada quebra
  interoperabilidade.

## 6) Proximo passo sugerido

- Criar uma nota tecnica separada sobre **homeostase economica** e
  **PoK de participacao** aplicada a capsulas locais.
- Manter referencias apenas como contexto, sem normatividade.

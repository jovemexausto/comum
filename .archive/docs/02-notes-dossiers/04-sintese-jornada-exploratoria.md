# Sintese da jornada exploratoria (nota)

Objetivo: consolidar os principais fios conceituais, tecnicos e politicos que
emergiram ao longo desta exploracao, sem perder riqueza nem transformar tudo em
roadmap imediato.

Esta nota organiza o que ficou em aberto como contexto + direcao acionavel,
servindo como indice vivo para futuras decisoes.

## Eixo 1 — Estabilidade tecnica

### 1. Paridade de runtime (JS mobile vs Rust)

Contexto:

- O sistema roda ponta a ponta.
- O runtime Rust continua sendo a referencia canônica.
- O runtime JS mobile ainda e aproximacao funcional.

Acao:

- Formalizar papeis: Rust = canonico, JS mobile = provisoriamente nao-canônico.
- Criar suite de paridade com cenarios identicos nos dois runtimes.
- Comparar IDs, validacao, ingest, sync e outputs de capsula.

### 2. Extrair `AppNode` para SDK como `ComumClient`

Contexto:

- `AppNode` ja e a abstracao real do sistema no mobile.
- Ainda vive no app, nao no SDK.

Acao:

- Levar a abstracao para `impl/comum-js`.
- Encapsular `Commoner + transport + ingest + sync + flows`.
- Definir uma API publica de alto nivel.

### 3. Padronizacao de naming no boundary TS

Contexto:

- A API ainda mistura `snake_case` e `camelCase`.
- Isso atrapalha DX e estabilizacao da superficie publica.

Acao:

- Fixar: TS publico em `camelCase`; interno/CBOR em `snake_case`.
- Revisar nomes como `id_hex`, `payload_cbor`, `zk_proofs`.

### 4. Interface unica de transporte

Contexto:

- Transporte plugavel ja existe, mas o contrato ainda nao esta consolidado.

Acao:

- Definir interface unica: `connect`, `publish`, `onMessage`, `close`.
- Alinhar WS, e depois BLE/NFC, sob o mesmo contrato.

### 5. Fault injection no relay

Contexto:

- Ja existe E2E multi-node em rede real via WS.
- Falta testar atraso, perda, duplicacao e reorder.

Acao:

- Adicionar `dropRate`, `delayMin/Max`, `duplicateRate`, `partition`.
- Medir convergencia e divergencia sob caos.

## Eixo 2 — Laboratorio experimental

### 6. Playground do sistema

Contexto:

- O projeto ja permite experimentos com comportamento emergente.

Acao:

- Criar playground com spawn de nodes, relay caotico e metricas.
- Rodar cenarios como mercado fantasma, sybil controlado e rede fragmentada.

### 7. Politicas de confianca / inferencia local

Contexto:

- Surgiu o tema de inferencia distribuida eventual e decisao sob incerteza social.

Referencia:

- `docs/notes/brainstorm-inferencia-social.md`

Acao:

- Definir 2 ou 3 politicas de confianca.
- Plugar no playground.
- Observar efeitos em receipts, aceitacao e formacao de clusters.

### 8. Teoria dos jogos como lente

Contexto:

- O Comum se parece mais com jogos repetidos com informacao incompleta.

Acao:

- Usar teoria dos jogos para analisar estrategias emergentes.
- Comparar comportamentos de nodes em cenarios repetidos.

### 9. Inteligencia social programavel

Contexto:

- A expressao apareceu como nome possivel para um fenomeno real do sistema.

Acao:

- Tratar como hipotese exploratoria, nao branding.
- Conectar com inferencia distribuida, experiencia local e politicas de confianca.

## Eixo 3 — Arquitetura conceitual

### 10. Triggers e camadas

Contexto:

- Ficou clara a diferenca entre link triggers, execution triggers e policy triggers.

Referencia:

- `docs/notes/triggers-e-camadas.md`

Acao:

- Refletir essa separacao em docs e API.
- Evitar usar "trigger" como conceito unico.

### 11. Relacao entre verbo, capsula e app

Contexto:

- Surgiu o incomodo com verbo sem capsula e com mistura de camadas.
- A sintese parcial: verbo = rotulo semantico minimo; capsula = interpretacao; app = decisao local.

Acao:

- Consolidar arquitetura em 3 camadas:
  - protocolo
  - runtime semantico
  - aplicacao/politica
- Revisar SDK para reduzir mistura entre essas camadas.

### 12. Thin waist semantico

Contexto:

- Coordenacao exige alguma semantica comum.
- O perigo e a abstracao comum se tornar totalizante.

Acao:

- Formular explicitamente o core como um `thin waist` semantico.
- Distinguir entre acoplamento suficiente e captura semantica maxima.

## Eixo 4 — Economia politica do Comum

### 13. Hawala como inspiracao e limite

Contexto:

- Hawala mostrou afinidade com confianca relacional, remessa e infraestrutura social informal.

Referencia:

- `docs/notes/comum-e-hawala.md`

Acao:

- Usar hawala como lente, nao como modelo a copiar.
- Preservar logica relacional sem matar flexibilidade local.

### 14. Interoperabilidade sem equivalencia unica

Contexto:

- Esta pergunta virou um eixo forte:

> Como permitir interoperabilidade sem reduzir a pluralidade a uma equivalencia unica?

Referencia:

- `docs/notes/interoperabilidade-sem-equivalencia-unica.md`

Acao:

- Usar essa pergunta como teste de desenho para qualquer proposta de remessa/compensacao.
- Evitar linguagem como "camada global" sem qualificacao forte.

### 15. Camada de compensacao intercomunitaria

Contexto:

- Emergiram formulacoes melhores que "moeda global":
  - camada de compensacao intercomunitaria
  - infraestrutura federativa de compensacao
  - malha de remessa entre comuns

Acao:

- Consolidar vocabulario preferido.
- Definir principios e anti-principios.
- Explorar gramatica minima: `lock`, `commit`, `settle`, `redeem`, `release`, `cancel`.

### 16. Petrodolar e alternativa federativa

Contexto:

- Surgiu o insight de que uma camada de liquidacao pode facilmente virar poder imperial.

Acao:

- Sintetizar contraste entre modelo imperial e modelo federado.
- Usar isso como criterio politico para propostas economicas futuras.

### 17. O proprio Comum como risco de colonizacao abstrata

Contexto:

- O projeto pode virar colonizador se exigir que o local caiba no protocolo.

Acao:

- Tornar explicito o teste:
  - isso amplia autonomia local?
  - ou obriga o local a caber no protocolo?

### 18. Refinar o conceito de colonizacao

Contexto:

- Coordenacao exige informacao mutua e semantica comum.
- O problema nao e abstrair; e abstrair demais, de modo totalizante.

Acao:

- Registrar a distincao entre abstracao necessaria para coordenar e abstracao que captura o real.
- Conectar isso ao regulador theorem e ao criterio de informacao mutua suficiente.

## Eixo 5 — Imaginario institucional

### 19. Sociedades de codigo aberto

Contexto:

- Surgiu a hipotese de que o Comum poderia habilitar instituicoes abertas, auditaveis e forkaveis.

Acao:

- Decidir se isso vira linguagem oficial ou fica como hipotese exploratoria.
- Se ficar, formular com cuidado: nao sociedade como codigo, mas infraestrutura institucional aberta e forkavel.

### 20. O que realmente temos em maos

Contexto:

- O projeto pode ser mais que protocolo ou app.
- Talvez seja base para coordenacao federada entre mundos locais soberanos.

Acao:

- Sintetizar isso em nota propria, para nao perder a ambicao real do que foi construido.

## Referencias cruzadas

- `docs/notes/brainstorm-inferencia-social.md`
- `docs/notes/triggers-e-camadas.md`
- `docs/notes/comum-e-hawala.md`
- `docs/notes/interoperabilidade-sem-equivalencia-unica.md`

## Sintese final

Esta jornada apontou para quatro compromissos simultaneos:

1. Estabilidade tecnica sem perder simplicidade
2. Experimentacao real sem normatizacao precoce
3. Arquitetura conceitual clara entre protocolo, runtime e app
4. Rigor politico para interoperar sem recolonizar o local

Se houver uma frase-guia para o que emergiu ate aqui, talvez seja esta:

> O Comum precisa de abstracao suficiente para coordenar, mas nao de tanta
> abstracao a ponto de substituir os mundos locais que pretende servir.

## Status

Nota de sintese. Nao normativa.

# Paper 01 Packet

Status: ativo

Objetivo: transformar a decisao de framing do primeiro paper do Comum em um
artefato operacional curto, suficiente para orientar escrita, coleta de
evidencia e escolha de venue sem inflar o corpus teorico nem a spec.

## Framing

Framing escolhido: protocol / systems vision.

Formula curta:

> Comum e um substrate local-first para coordenacao comunitaria verificavel sem
> consenso global obrigatorio.

Formula expandida:

> Comum oferece uma base para memoria publica, coordenacao verificavel e leitura
> local de estado social a partir de testemunhos assinados, grafo causal, sync
> eventual, prova de contexto modular e semantica local via capsulas, sem exigir
> ordenacao global forte nem uma autoridade central irrevisavel.

O paper 1 nao deve ser vendido como prova final de governanca, justica ou
accountability institucional. Essas camadas entram como motivacao e horizonte,
nao como claim empirico principal.

## Claims

### Claim central

Um protocolo baseado em `Testimony` assinado, grafo causal local, serializacao
canonico-deterministica, sync eventual e semantica local via capsulas e uma base
viavel para coordenacao verificavel sem consenso global obrigatorio.

### Claims de suporte

1. O protocolo tem um nucleo fino o bastante para interoperar sem absorver a
   semantica local.
2. O estado social pode ser derivado localmente do grafo sem armazenamento de
   "verdade global".
3. O projeto ja possui implementacao suficiente para sustentar o framing como
   paper de visao arquitetural, nao apenas manifesto.
4. A arquitetura torna possivel discutir memoria publica e autoridade local sem
   prometer que o protocolo resolve sozinho legitimidade substantiva.

### Claims que NAO devem entrar como claim principal do paper 1

- "autoridade auditavel melhora governanca na pratica"
- "o sistema resolve disputas reais melhor que alternativas"
- "pluralidade de valor ja esta demonstrada empiricamente"
- "mobile parity esta completa"

Esses pontos podem aparecer como horizonte, limitacao ou agenda de pesquisa.

## Evidence

### Evidencia tecnica ja disponivel

- CIPs vivos, registries, CDDL e vetores em `spec/`
- implementacao de referencia Rust em `crates/comum-rs/`
- SDK JS em `packages/comum-js/`
- runtime capsular e capsulas de referencia em `capsules/`
- simulacoes em `simulations/`
- conformance runner em `tests/conformance/`

### Evidencia verificada agora

- `cargo test -p comum-rs` passa
- `npm run build && npm test` em `packages/comum-js` passa
- `node tests/conformance/run.js` passa
- `cargo run -p agora-sim` passa
- `cargo run -p feira-sim` passa
- `cargo run -p mutirao-sim` passa
- `cargo run -p coherence-sim` passa

### Evidencia argumentativa disponivel

- corpus teorico consolidado em `docs/corpus/`
- framing de autoridade local auditavel e revogavel
- recusa explicita de consenso global obrigatorio e centro irrevisavel

### Evidencia que ainda falta para papers posteriores

- estudos de uso real em comunidade
- metrica de captura institucional
- benchmark sistematico de convergencia sob particao/churn
- avaliacao forte de parity mobile

## Limits

### Limites do paper 1

O paper 1 nao deve afirmar:

- que o Comum ja demonstrou superioridade social sobre plataformas centralizadas
- que a arquitetura resolve disputas reais sem instituicoes locais
- que a camada capsular ja cobre accountability completa
- que a interoperabilidade economica plural ja esta validada empiricamente

### Limites atuais do projeto

- parity cross-runtime ainda esta em consolidacao
- mobile continua sendo fronteira pratica relevante
- a narrativa institucional esta mais avancada que a evidencia empirica
- simulacao existe, mas ainda nao equivale a deployment real

### Formula de honestidade para o paper

O Comum deve ser apresentado como:

- uma arquitetura real
- um protocolo real
- um projeto com implementacao e testes reais
- mas ainda nao uma validacao empirica forte de todas as suas teses politicas

## Venues iniciais

Mais promissoras para este framing:

1. PaPoC
2. HotNets
3. workshops de EuroSys / Middleware / distributed systems
4. short paper ou vision/position paper em venue de sistemas distribuidos

Menos recomendadas como primeira submissao:

- CSCW/CHI, se nao houver estudo de uso
- FAccT, se nao houver caso institucional empirico forte
- venues de cripto/privacidade, se o foco principal ainda nao for ZK

## Estrutura sugerida do paper

1. Problema
2. Tese
3. Modelo do protocolo
4. Arquitetura e superficie implementada
5. Capsulas e leitura local de estado
6. Evidencia atual: vetores, conformance, simulacoes
7. Limites e trabalho futuro

## Proximos passos

1. escrever abstract de 200-250 palavras
2. fechar um outline de 6-8 secoes
3. separar evidencias ja reproduziveis das apenas planejadas
4. decidir venue primaria e venue secundaria
5. derivar backlog curto de lacunas antes da submissao

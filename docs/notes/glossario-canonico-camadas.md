# Glossario canonico de camadas (rascunho)

Objetivo: reduzir ambiguidade terminologica entre teoria, spec e implementacao.

## Termos

### Comum Protocol

Infraestrutura minima para testemunho, verificabilidade, causalidade e sincronizacao.

Nao e:

- moeda global
- soberania superior de valor
- substituto da comunidade

### Node

Instancia tecnica que valida, ingere, emite e sincroniza Testemunhos.

### Commoner (tecnico)

Fachada de alto nivel para operar um Node.

Nota: quando o termo for usado no sentido social, isso deve ser explicitado.

### App

Cliente/interface que opera sobre Node/Commoner.

Pode incluir UX e politicas locais.
Nao define, por si, a comunidade.

### Capsule

Semantica executavel local (WASM + envelope de invocacao/resultado).

Nao e:

- a comunidade inteira
- a ontologia total de governanca/valor

### Comunidade nominal

Recorte ancorado por Genesis/Community ID para leitura institucional.

### Comunidade funcional

Recorte emergente do grafo relacional, possivelmente sem Genesis compartilhado.

### Regime local de valor

Conjunto local de regras de emissao, reconhecimento, revogacao e circulacao de valor.

### Unidade local de valor

Nome da unidade de conta de um regime local, quando houver.

Nao deve herdar automaticamente o nome do protocolo.

### Federacao de borda

Infraestrutura de traducao/compensacao limitada entre comunidades.

Nao deve impor equivalencia unica nem governar o interior dos regimes locais.

### Identidade soberana

Capacidade de a pessoa controlar chaves/personas/provas sem dependencia estrutural de um app unico.

## Regra de higiene

Quando um documento usar um termo fora deste sentido, deve declarar explicitamente a variacao.

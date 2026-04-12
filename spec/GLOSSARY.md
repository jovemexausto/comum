# Glossario Canonico

Status: normativo

Objetivo: servir como SSOT terminologico da especificacao.

Regra: quando houver conflito entre este glossario e outro documento da `spec/`,
este glossario vence, salvo se um CIP declarar explicitamente uma variacao local
de termo para seu proprio escopo.

## Termos canonicos

### Comum Protocol

Infraestrutura minima para testemunho, verificabilidade, causalidade,
sincronizacao e extensibilidade local.

`Comum` nomeia o protocolo/projeto. O termo MUST NOT ser interpretado, por si,
como nome de moeda universal, unidade monetaria global ou ativo soberano do
protocolo.

### Testimony

Menor unidade de informacao verificavel do protocolo.

### Node

Instancia tecnica local que valida, ingere, emite e sincroniza Testimonies.

### Commoner

Fachada tecnica de alto nivel para operar um `Node`.

Quando um documento mais antigo da spec usar `Commoner` para se referir a um no
conforme, essa leitura deve ser entendida como atalho historico, nao como
licenca para confundir papel social, app e runtime.

### App

Cliente ou interface que opera sobre `Node`/`Commoner`.

Apps podem carregar UX, automacoes e politicas locais. Apps MUST NOT ser
tratados como definicao normativa de comunidade, identidade ou valor.

### Capsule

Semantica executavel local, normalmente empacotada como artefato WASM e
acionada por envelopes normativos do protocolo.

Capsules expressam recortes executaveis de instituicoes locais. Capsules MUST
NOT ser tratadas como totalidade da comunidade viva.

### Comunidade nominal

Recorte institucional ancorado por `Genesis` e identificado por `Community ID`.

### Comunidade funcional

Recorte emergente do grafo relacional e das leituras locais, possivelmente sem
`Genesis` compartilhado.

O protocolo base pode ancorar comunidades nominais sem pretender esgotar todas
as comunidades funcionais que emergem do grafo.

### Community ID

Identificador da comunidade nominal. Em `CIP-0001`, e definido como o `id` do
Testimony de `Genesis`.

### Regime local de valor

Conjunto local de regras de emissao, reconhecimento, circulacao, compensacao e
revogacao de valor.

Regimes locais de valor pertencem a comunidades e suas instituicoes. O protocolo
base MUST NOT impor uma equivalencia unica entre regimes locais diferentes.

### Unidade local de valor

Nome da unidade de conta de um regime local de valor, quando houver.

O protocolo base MAY transportar atos de valor sem fixar um nome universal para
essa unidade.

### Federacao de borda

Infraestrutura de traducao, compensacao e interoperabilidade limitada entre
comunidades e regimes locais diferentes.

Federacao de borda MUST NOT ser interpretada como soberania superior sobre o
interior dos regimes locais.

### Identidade soberana

Capacidade de controlar chaves, personas, commitments e provas sem dependencia
estrutural de um app unico.

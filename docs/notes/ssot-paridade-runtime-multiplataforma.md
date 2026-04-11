# SSOT, paridade e runtime multiplataforma

Objetivo: registrar de forma lossless a conversa sobre SSOT, paridade entre
implementacoes, `comum-js` multiplataforma, uso de Rust, N-API, WASM,
React Native / Expo e a direcao arquitetural recomendada.

## Contexto

- O desejo e que `comum-js` funcione em browser, Node e mobile.
- Node pode usar backend nativo naturalmente.
- O problema mais sensivel hoje e mobile com Expo.
- A pergunta central foi se faz sentido depender de bindings Rust, ou se o
  correto e escrever o core 100% em TypeScript.

## Leitura da base atual

- `impl/comum-rs/napi` ja existe como crate Rust, mas ainda nao esta empacotado
  como pacote npm pronto para consumo multiplataforma.
- `impl/comum-js/src/index.ts` e Node-only na pratica:
  - importa `node:child_process`
  - importa `node:module`
  - usa `process.env`
  - usa `Buffer`
  - usa `spawnSync`
  - usa `require`
- `impl/comum-js/src/mobile.ts` ja existe como entrada separada para React
  Native, com runtime JS proprio.
- O README atual do `comum-js` ja admite que o runtime mobile e
  `non-canonical`.
- O runtime WASM em Rust (`impl/comum-rs/src/wasm_runtime.rs`) aceita bytes de
  WASM fornecidos pelo chamador; ele nao resolve artefatos automaticamente.
- Testes/e2e ainda leem `.wasm` diretamente do filesystem do repo.

## Decisoes e conclusoes

### 1. `comum-js` deve ser API unica, mas nao precisa ter uma unica implementacao interna

- A API publica pode e deve ser unica.
- Cada alvo pode ter backend proprio:
  - browser
  - Node
  - React Native / Expo
  - iOS/macOS
  - Android
- O contrato publico precisa ser o mesmo.
- O backend pode variar por plataforma.

### 2. O core portavel deve ser 100% TypeScript

- Recomendacao clara: o core semantico e canonico deve viver em TypeScript.
- Isso inclui, idealmente:
  - CBOR canonico
  - builders de payload
  - validators
  - encode/decode
  - hashing/KDF/HMAC quando viavel no runtime alvo
  - logica de protocolo que precisa existir em browser, Node e mobile
- Rust nao deve ser a unica fonte de comportamento se a execucao real em varios
  alvos nao depender dele.
- Rust pode continuar existindo como implementacao de referencia, oracle local
  e acelerador, mas nao como gargalo arquitetural.

### 3. N-API continua sendo otimo para Node, mas nao pode ser a base da portabilidade

- Node/desktop podem usar `napi` naturalmente.
- Isso resolve muito bem ambiente servidor, CLI e desktop local.
- Mas nao resolve browser.
- E tambem nao resolve Expo/React Native de forma universal.

### 4. WASM pode ser aceleracao, mas nao deve ser requisito universal

- WASM cabe muito bem como aceleracao em:
  - browser
  - Node
- Para React Native / Expo, a historia e muito menos direta.
- Expo/Metro conseguem empacotar `.wasm` como asset.
- Mas empacotar asset nao significa que o runtime JS conseguira executar esse
  WASM de forma universal e confiavel.
- O runtime padrao do React Native e Hermes.
- A documentacao consultada e o estado do ecossistema indicam cautela com a
  suposicao de `WebAssembly` universal em Hermes.
- Portanto:
  - `WASM` como asset: sim
  - `WASM` como acelerador universal em Expo Go: nao assumir
  - `WASM` via modulo nativo/plataforma: possivel, mas isso ja entra na camada
    de implementacao nativa por alvo

### 5. Expo Go versus dev build muda a resposta tecnica

- Se o objetivo for funcionar em Expo Go:
  - precisa existir caminho puro TS
  - nao pode depender de native module proprio
  - nao pode depender de WASM como precondicao de runtime
- Se for aceitavel usar Expo development build:
  - abre-se o caminho para modulos nativos locais
  - iOS/macOS podem ter bridge propria
  - Android pode ter bridge Kotlin/JNI propria
  - a aceleracao pode ser empurrada para a camada nativa por plataforma

### 6. Cada plataforma pode ter implementacao propria

Foi explicitamente afirmado e aceito que isso e valido e natural.

Exemplos:

- Browser: TypeScript puro, com WASM opcional
- Node: N-API e/ou WASM, com fallback TS
- Expo Go: TypeScript puro + assets
- Expo development build: facade TS + modulo nativo
- iOS/macOS: Swift bridge ou outro backend nativo
- Android: Kotlin bridge ou outro backend nativo

Regra principal:

- a API publica deve ser unificada
- a implementacao por baixo pode variar

## Consequencia principal: necessidade de paridade rigorosa

- Se houver multiplas implementacoes reais, precisa existir teste de paridade
  forte.
- Ter implementacao em Rust nao elimina o problema se browser/mobile nao usam
  exatamente aquela mesma implementacao.
- O risco passa a ser drift semantico entre Rust, TS e backends futuros.

Tipos de drift citados ou implicados:

- drift de serializacao canonica
- drift de validacao
- drift de defaults
- drift de ordem de chaves/campos
- drift de edge cases
- drift de mensagens de erro e rejeicoes

## Direcao para SSOT

Foi discutida a ideia de uma SSOT que nao fosse apenas markdown e que pudesse
servir melhor para portabilidade, codegen e implementacoes assistidas por IA.

### O que foi descartado

- `IR-first` foi considerado excessivo neste momento.
- A avaliacao final foi que isso seria overkill para o estado atual do projeto.

### O que foi recomendado

Usar uma composicao simples e antifragil:

- `CIP` e docs normativos para semantica, fluxo, rationale e seguranca
- `CDDL` para wire format e estruturas CBOR
- registries machine-readable (`YAML` ou `JSON`) para tabelas normativas
- test vectors JSON para comportamento observavel e paridade

Formula sintetica:

- `CDDL` = shape do wire format
- registries = enums, tabelas, identificadores normativos
- vectors = contrato executavel de comportamento/paridade

### Recomendacao final sobre SSOT

Modelo recomendado:

- `spec/schemas/*.cddl`
- `spec/registries/*.yaml` ou `*.json`
- `spec/test-vectors/**/*.json`

E usar codegen pequeno para:

- constantes de verbos
- suites
- context types
- tabelas de registry
- manifest de vetores
- types simples / esqueletos leves

E nao tentar, por enquanto, gerar automaticamente:

- toda a logica de encoding/decoding
- toda a logica semantica de DAG/merge
- toda a validacao complexa

Essas partes continuam implementadas manualmente, mas presas pelos vetores de
paridade.

## Motivo pelo qual esta abordagem foi considerada boa

- Nao absolutiza cedo demais.
- Nao prende o protocolo a Rust.
- Nao transforma markdown em unica fonte de verdade.
- Mantem a base auditavel.
- Facilita criar novas implementacoes sob demanda.
- Facilita criar adaptadores e implementacoes agenticas com IA.
- Mantem a opcao de aceleracao por alvo sem comprometer portabilidade.

## Estrutura arquitetural recomendada para `comum-js`

Foi proposta uma divisao conceitual assim:

- `core`
  - TypeScript puro
  - sem dependencias Node-only
  - sem `spawnSync`
  - sem `createRequire`
  - sem depender de `Buffer` como primitivo central
- `node`
  - N-API quando existir
  - fallback para core TS
  - WASM opcional
- `browser`
  - core TS
  - WASM opcional
- `mobile`
  - core TS como base
  - assets quando necessario
  - backend nativo por plataforma, se e quando fizer sentido

Regra de ouro formulada na conversa:

- o acelerador nunca define a verdade
- ele apenas implementa mais rapido o que o core ja define

## Principio de crescimento adotado

- Fechar o contrato agora, nao o futuro inteiro.
- Implementacoes crescem sob demanda.
- Nao tentar resolver todos os alvos com uma unica estrategia prematura.
- Se houver necessidade de aceleracao, ela entra depois, por backend.

## Governanca implicita sugerida

Para cada nova feature importante:

1. CIP / documentacao normativa
2. schema / CDDL
3. registry machine-readable
4. test vectors
5. implementacoes

Isso evita que uma implementacao especifica vire a spec de fato.

## Frase-sintese da decisao

Boa documentacao, abstracao clara, `CDDL`, registries e vetores ja formam uma
base suficientemente forte para implementar o protocolo em qualquer runtime que
for necessario, deixando o sistema crescer sob demanda sem absolutizar agora.

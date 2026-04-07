# Pesquisa Comum Protocol — Documento Unico

Este documento substitui os rascunhos anteriores e consolida toda a pesquisa
em um formato unico, sem perda de informacao. Ele separa o que ja e normativo
(CIP-0001) do que e hipotese/proposta, e define o destino de cada ponto
(nota tecnica, RFC/CIP, ou manter como esta).

## 0) Contexto minimo do Comum

- Primitivo: Testimony (Claim vs Context + Proof).
- Grafo: DAG com referencias (`prev_id`, `refs`), sem relogio global.
- Serializacao: CBOR canonical deterministico.
- Runtime: WASM com limites normativos.
- Sync: offline-first com CTE e perfis de Commoner.
- Revogacao: sempre local ao escopo de comunidade.

## 1) Fundamentos normativos (CIP-0001)

Estes pontos ja sao especificacao e nao devem ser alterados por pesquisa:

- Testimony como unidade atomica.
- Separacao Claim vs Context + Proof.
- CBOR canonical (determinismo de hash).
- DAG com causalidade por referencias.
- WASM deterministico com limites normativos.
- Nullifiers HKDF+HMAC (double-spend offline).
- Snapshots e perfis LIGHT/FULL/ARCHIVE.
- Revogacao `comum/revoke` com escopo local.

## 2) Achados principais (da pesquisa v1)

**Aproveitaveis (alinhados com CIP-0001):**

- Offline-first como estado natural.
- Consistencia eventual forte em DAGs.
- Necessidade de determinismo de serializacao.
- Separacao semantica/justificativa.

**Nao normativos (devem virar hipoteses):**

- MMR/SMT como estrutura interna de snapshot.
- Encointer/PoP como membership normativo.
- Ring-VRF/Bandersnatch/BBS/accumulators.
- Regras globais de desempate por hash.
- Detalhamento de RNS alem do uso como transporte.

## 3) Lacunas abertas (problemas reais)

### A) Fronteiras de comunidade (nominal vs funcional)

- Como um Commoner decide que um Testimony pertence a uma comunidade?
- Como evitar injecao adversarial sem autoridade global?

### B) Membership explicito

- Precisamos de `comum/member`?
- Como emitir membership sem centralizar?
- Como revogar/expirar membership?

### C) Revogacao local (escopo e efeitos)

- Revogacao transitiva e desejavel?
- Como evitar abuso politico?

### D) Snapshots (estrutura interna)

- SMT ou MMR ou outro?
- Como construir roots deterministicas sem ordem global?

### E) Conflitos offline

- Heuristica de desempate deve ser local ou normativa?

## 4) Propostas e hipoteses (com destino)

### 4.1 Filtro por confianca local (hipotese)

- **Proposta:** filtrar sync por caminho de `comum/vouch`/`comum/encounter`
  ate sementes confiaveis.
- **Trade-off:** risco de particao funcional se a rede fragmentar.
- **Destino:** Nota tecnica (heuristica de sync).
- **Teste minimo:** injetar 1.000 testemunhos sem vouch e medir descarte.

### 4.2 `comum/member` (proposta forte)

- **Proposta:** verbo normativo para membership auditavel.
- **Campos sugeridos:** `subject`, `role?`, `expires?`, `community`.
- **Validade:** aprovada por Capsula de entrada (regra local).
- **Trade-off:** aumenta carga social e complexidade.
- **Destino:** RFC/CIP nova (nao alterar CIP-0001 agora).

### 4.3 Snapshots internos (hipotese)

- **Proposta:** SMT para estado atual, MMR para historico append-only.
- **Trade-off:** implementacao mais complexa; benchmarking necessario.
- **Destino:** Nota tecnica; virar CIP apenas se adotado.

### 4.4 Desempate offline (hipotese)

- **Proposta:** desempate lexicografico por hash como heuristica local.
- **Trade-off:** pode conflitar com autonomia de Capsulas.
- **Destino:** Nota tecnica para Capsulas (nao normativo).

### 4.5 Revogacao relacional (hipotese)

- **Proposta:** modelos de reputacao (ex.: Jøsang) aplicados por Capsulas/cliente.
- **Trade-off:** maior complexidade; risco de imprevisibilidade.
- **Destino:** Nota tecnica (nao normativo).

## 5) Matriz de decisao (o que vira CIP/nota)

| Topico               | Classificacao  | Destino      | Proximo passo            |
| -------------------- | -------------- | ------------ | ------------------------ |
| Nullifiers HKDF      | Normativo      | CIP-0001     | Implementado em comum-rs |
| `comum/member`       | Proposta forte | RFC/CIP      | Definir CDDL + fluxo     |
| SMT/MMR              | Hipotese       | Nota tecnica | Benchmark de prova       |
| Filtro por vouch     | Hipotese       | Nota tecnica | Simulacao de spam        |
| Tie-break hash       | Hipotese       | Nota tecnica | Guia de Capsulas         |
| Revogacao relacional | Hipotese       | Nota tecnica | Avaliar Jøsang           |

## 6) Entregaveis esperados da pesquisa

- Notas tecnicas em `docs/notes/` para hipoteses.
- RFC/CIP draft para `comum/member`.
- Matriz achado → implicacao → teste.

## 7) Criterios de qualidade

- Tudo que nao esta na CIP deve ser marcado como hipotese.
- Propostas devem vir com trade-offs e teste minimo.
- Nenhuma proposta pode introduzir autoridade global implicita.

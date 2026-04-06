# RFC-0002 — Commoner Facade (API de Nó)

## Resumo

Este RFC propõe um facade de alto nível para representar um nó do Comum
(`Commoner`), unificando emissão, ingestão, validação e sincronização de
Testemunhos. O objetivo é fornecer uma interface estável para apps sem
alterar o protocolo.

## Motivação

Hoje há peças isoladas (CTE, sync, validação, runtime) mas não existe um
objeto que represente um nó com comportamento coerente. Isso dificulta a
criação de apps que precisam instanciar participantes (ex.: Alice/Bob) e
orquestrar fluxo offline-first.

## Objetivos

- Definir uma interface mínima e explícita para um nó.
- Reduzir atrito na integração (mobile/desktop/cli).
- Evitar abstrações que escondam a semântica do protocolo.

## Não-objetivos

- Não altera CIPs nem registries.
- Não define UX nem políticas de governança.
- Não impõe armazenamento, rede ou UI específicos.

## Definição Proposta

Um `Commoner` encapsula:

- identidade local (DID, chaves)
- estado de grafo (clock, índice local)
- validação de Testemunhos
- ingestão/emissão
- sync (HELLO/REQUEST/RESPONSE)
- CTE encode/decode + fragmentação

### Interface mínima (pseudo)

```
Commoner {
  // identidade e estado
  did() -> DID
  clock() -> uint64

  // validação / ingestão
  validate(testimony) -> Result
  ingest(testimony) -> Result

  // emissão
  emit(verb, payload, context) -> Testimony

  // sync
  build_hello(profile) -> HelloPayload
  build_request(clock, limit) -> RequestPayload
  apply_response(payload) -> Result

  // CTE
  encode_cte(payload) -> CTE
  fragment_cte(cte, mtu, frag_id) -> [CTEFragment]
  reassemble(fragments) -> CTE
}
```

## Regras

- O facade MUST expor APIs que preservem as garantias normativas da CIP.
- O facade MUST NOT esconder falhas de validacao.
- Erros MUST ser distinguiveis entre:
  - erro de formato (CBOR/CTE)
  - erro de prova (assinatura/nullifier/context)
  - erro de estado (clock/prev_id/refs)

## Implementacao de referencia

Inicialmente em `comum-rs`, com wrapper opcional em `comum-js`.

## Impacto

- Facilita SDKs e apps.
- Reduz divergencia entre implementacoes.

## Questões em aberto

- Nome final: Commoner, Witness, Testemunha ou Node.
- Modelo de armazenamento (memoria vs persistente).
- Nivel de API (sync automatizado vs manual).

## Status

- Proposto

# Conjunto de Conformidade

Objetivo: validar implementacoes contra os vetores de teste.

Requisitos minimos:

- Parse CBOR canonical
- Recalcular id com SHA3-256
- Validar estrutura via CDDL

Fluxo sugerido:

1. Carregar spec/test-vectors/manifest.json
2. Para cada vetor, gerar CBOR canonical de testimony_without_id
   (se testimony_without_id_cbor_hex estiver presente, comparar)
3. Calcular SHA3-256 e comparar com expected_id
4. (Opcional) validar assinatura se skip_signature_verify = false

Execucao:

```
node runtime/conformance/run.js
```

Implementacoes de referencia devem publicar resultados em docs/interop.

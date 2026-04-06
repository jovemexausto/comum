# Vetores de Teste

Os vetores aqui sao iniciais e validam:

- CBOR canonical
- Recomputo de id
- Validacao de estrutura

`expected_id` deve ser calculado a partir de
`testimony_without_id` usando SHA3-256.
Quando presente, `testimony_without_id_cbor_hex` deve corresponder
ao CBOR canonical do objeto acima.

Formato:

- vector-XXXX.json
- manifest.json lista vetores ativos

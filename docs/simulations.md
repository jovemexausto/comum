# Simulacoes Locais

Objetivo: simular nos e sincronizacao sem hardware real.

## Cenario 1 — Encounter + Transfer + Sync

1. A e B registram um encounter (comum/encounter)
2. A cria uma transferencia (comum/transfer)
3. A sincroniza com C
4. B sincroniza com C

Execucao:

```
just sim
```

Saida esperada:

- C recebe 2 testemunhos (encounter + transfer)
- Estado final mostra contagem e saldos derivados

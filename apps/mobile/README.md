# App Mobile

Aplicativo de referencia (React Native) em que cada instancia do app e um node/Commoner.

## Estado atual

- App sobe um node local (`AppNode`) e expõe DID/status
- Node encapsula Commoner e fluxo Feira (offer/accept/receipt)
- Node aceita transporte plugavel (`attachTransport`)
- E2E usa relay WebSocket real + workers isolados

## E2E multi-node automatizado

O teste sobe um relay WebSocket, cria 3 instancias isoladas de node (processos separados), executa um fluxo Feira completo e valida convergencia de estado via rede.

```sh
npm install
npm run e2e:multi-node
```

## Rodar app

```sh
npx expo start
```

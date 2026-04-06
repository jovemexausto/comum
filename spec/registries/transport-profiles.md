# Registro: Perfis de Transporte

| Perfil | Transporte | MTU (bytes) | Half-duplex | Throughput >=5 bps | Framing/Encoding | Link trigger  | Prova de proximidade | Origin hint                       | Notas                   | Status | CIP      |
| ------ | ---------- | ----------- | ----------- | ------------------ | ---------------- | ------------- | -------------------- | --------------------------------- | ----------------------- | ------ | -------- |
| ble-5  | BLE 5.0    | 512         | sim         | sim                | gatt + cte       | ble connect   | sim (proximity)      | opcional                          | MAC rotation por sessao | Active | CIP-0001 |
| nfc-1  | NFC        | 8192        | sim         | sim                | ndef + cte       | nfc tap       | sim (proximity)      | opcional                          | alcance ~5cm            | Active | CIP-0001 |
| qr-1   | QR fisico  | 3072        | sim         | sim                | base45 + qr      | qr reassembly | nao                  | nao                               | assincrono              | Active | CIP-0001 |
| rns-1  | RNS/LoRa   | 500         | sim         | sim                | rns + cte        | rns peer      | nao                  | obrigatorio quando author omitido | mesh sem infraestrutura | Active | CIP-0001 |

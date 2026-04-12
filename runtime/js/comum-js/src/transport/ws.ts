/**
 * WebSocket transport para ComumClient.
 *
 * Funciona em Node.js e React Native (via 'ws' ou WebSocket global).
 * O relay espera msgs JSON com type 'join' e 'testimony'.
 *
 * Nao normativo: este transporte e infraestrutura de demo/teste,
 * nao parte do contrato do protocolo.
 */

import type { NodeTransport } from "../client.js";

type WsLike = {
  send(data: string): void;
  close(): void;
  readyState: number;
  onopen: ((ev: unknown) => void) | null;
  onmessage: ((ev: { data: unknown }) => void) | null;
  onerror: ((ev: unknown) => void) | null;
  onclose: ((ev: unknown) => void) | null;
};

type WsConstructor = new (url: string) => WsLike;

export type WsTransportOptions = {
  /** URL do relay: ex: ws://127.0.0.1:8787 */
  url: string;
  /** Sala de coordenacao — todos os nos na mesma sala trocam testemunhos */
  room: string;
  /** DID do no, usado como nodeId para evitar eco */
  did: string;
  /**
   * Construtor de WebSocket. Em Node.js, passe a classe `WebSocket` de 'ws'.
   * Em RN, omitir (usa o global).
   */
  WebSocket?: WsConstructor;
  /** Timeout de conexao em ms (default: 5000) */
  connectTimeoutMs?: number;
};

function encodeBase64(bytes: Uint8Array): string {
  // Node.js
  if (typeof Buffer !== "undefined") return Buffer.from(bytes).toString("base64");
  // Browser/RN
  let binary = "";
  for (const b of bytes) binary += String.fromCharCode(b);
  return btoa(binary);
}

function decodeBase64(str: string): Uint8Array {
  if (typeof Buffer !== "undefined") return new Uint8Array(Buffer.from(str, "base64"));
  const binary = atob(str);
  const out = new Uint8Array(binary.length);
  for (let i = 0; i < binary.length; i++) out[i] = binary.charCodeAt(i);
  return out;
}

/**
 * Cria um NodeTransport que sincroniza testemunhos via relay WebSocket.
 *
 * Uso:
 *   const t = makeWsTransport({ url, room, did, WebSocket })
 *   await client.connect(t)
 */
export function makeWsTransport(opts: WsTransportOptions): NodeTransport {
  const {
    url,
    room,
    did,
    connectTimeoutMs = 5_000,
  } = opts;

  // Resolve WsConstructor: preferir opcao explicita, depois global, depois erro claro
  const WsCtor: WsConstructor | undefined =
    opts.WebSocket ??
    (typeof WebSocket !== "undefined" ? (WebSocket as unknown as WsConstructor) : undefined);

  if (!WsCtor) {
    throw new Error(
      "WebSocket not available. Pass opts.WebSocket (from 'ws' package) in Node.js environments."
    );
  }

  let ws: WsLike | null = null;
  let onTestimony: ((cbor: Uint8Array) => void) | null = null;

  const start = (cb: (cbor: Uint8Array) => void): Promise<void> => {
    onTestimony = cb;
    return new Promise<void>((resolve, reject) => {
      const socket = new WsCtor(url);
      const timeout = setTimeout(
        () => reject(new Error(`ws connect timeout to ${url}`)),
        connectTimeoutMs
      );

      socket.onopen = () => {
        socket.send(JSON.stringify({ type: "join", room, nodeId: did }));
      };

      socket.onmessage = (ev) => {
        let msg: Record<string, unknown>;
        try {
          msg = JSON.parse(String(ev.data)) as Record<string, unknown>;
        } catch {
          return;
        }

        if (msg.type === "joined") {
          clearTimeout(timeout);
          ws = socket;
          resolve();
          return;
        }

        if (msg.type === "testimony" && msg.from !== did && typeof msg.testimony === "string") {
          const cbor = decodeBase64(msg.testimony);
          onTestimony?.(cbor);
        }
      };

      socket.onerror = (ev) => {
        clearTimeout(timeout);
        reject(new Error(`ws error: ${String(ev)}`));
      };

      socket.onclose = () => {
        ws = null;
      };
    });
  };

  const publish = (cbor: Uint8Array): void => {
    if (!ws || ws.readyState !== 1 /* OPEN */) return;
    ws.send(
      JSON.stringify({
        type: "testimony",
        room,
        from: did,
        testimony: encodeBase64(cbor),
      })
    );
  };

  const close = (): void => {
    ws?.close();
    ws = null;
  };

  return { start, publish, close };
}

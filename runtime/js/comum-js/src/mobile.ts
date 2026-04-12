export * from "./core.mobile.js";

// Re-export high-level client (mobile build)
export { ComumClient } from "./client.mobile.js";
export type {
  OfferParams,
  Offer,
  AcceptResult,
  ReceiptResult,
  NodeTransport,
} from "./client.mobile.js";

// Re-export WebSocket transport (usa apenas APIs de plataforma :)
export { makeWsTransport } from "./transport/ws.js";
export type { WsTransportOptions } from "./transport/ws.js";

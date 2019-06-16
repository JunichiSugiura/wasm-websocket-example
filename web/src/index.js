import { WebSocketClient } from 'wasm-ws'

const ws = new WebSocketClient('wss://echo.websocket.org')

setTimeout(() => {
  ws.close()
  ws.free()
}, 3 * 1000);

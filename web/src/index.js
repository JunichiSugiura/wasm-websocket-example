import { WebSocketClient } from 'wasm-ws'

const ws = new WebSocketClient('wss://echo.websocket.org')

// this code is likely to be ignored
// since opening a connection takes some time
ws.send('message 1')

// wait reasonable time for a connection to be established
setTimeout(() => {
  ws.send('message 2')
}, 2000)

window.addEventListener('beforeunload', () => {
  ws.close()
  ws.free()
})

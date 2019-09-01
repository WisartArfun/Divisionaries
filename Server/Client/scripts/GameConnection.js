// import { ProtocolInterpreter } from './ProtocolInterpreter.js';
// import { State } from './State.js';

class GameConnection {
    constructor(ip, port) {
        this.ip = ip;
        this.port = port;
    }

    start(callback) {
        if (this.socket != undefined) return;
        let socket = new WebSocket('ws://' + this.ip + ':' + this.port);

        socket.onopen = function(event) {

            socket.send('Connected');

            // Listen for messages
            socket.onmessage = function(event) {
                console.log('Client received a message', event);
                callback(event.data);
            };

            // Listen for socket closes
            socket.onclose = function(event) {
                console.log('Client notified socket has closed', event);
            };

            //socket.close()
        };

        this.socket = socket;
    }
}

const _GameConnection = GameConnection;
export { _GameConnection as GameConnection };
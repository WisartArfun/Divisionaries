import { ProtocolInterpreter } from './ProtocolInterpreter.js';
import { State } from './State.js';
import { Map } from './Renderer.js';
import { GameConnection } from './GameConnection.js';

class Game {
    constructor(canvas, size_x, size_y, ip, port) {
        this.size_x = size_x;
        this.size_y = size_y;

        this.ip = ip;
        this.port = port;

        this.game_connection = new GameConnection(ip, port);
        this.game_connection.start(this.update_state.bind(this));

        this.state = new State(this.size_x, this.size_y);
        this.state.add_map(canvas);
    }

    update_state(message) {
        let state_update = ProtocolInterpreter.translate_packet(message);
        this.state.update(state_update);
    }
}

let canvas = document.getElementById('game-canvas');
let game = new Game(canvas, 10, 10, 'localhost', '8008');
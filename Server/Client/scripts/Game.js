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

let start_game_instance = function(x_size, y_size, canvas_name) {
    let canvas = document.getElementById(canvas_name);
    let game = new Game(canvas, x_size, y_size, 'localhost', '8008');
}

const _start_game_instance = start_game_instance
export { _start_game_instance as start_game_instance };
import { ProtocolInterpreter } from './ProtocolInterpreter.js';
import { State } from './State.js';
import { Map } from './Renderer.js';
import { GameConnection } from './GameConnection.js';

let canvas = document.getElementById('game-canvas');
let state = new State(10, 10);
state.add_map(canvas);

let callback = function(message) {
    let state_update = ProtocolInterpreter.translate_packet(message);
    state.update(state_update);
}

let game_connection = new GameConnection('localhost', '9001');
game_connection.start(callback);
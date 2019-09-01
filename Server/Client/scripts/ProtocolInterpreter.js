import { State } from './State.js';

class ProtocolInterpreter {
    // constructor(state) {
    //     this.state = state;
    // }

    static get_type(type_encoding) {
        let types = { 0: 'ground', 1: 'fog', 2: 'king' };
        return types[type_encoding];
    }

    static get_colors(color_encoding) {
        let colors = { 0: 'red', 1: 'blue', 2: 'green', 3: 'yellow', 4: 'purple', 5: 'cyan' };
        return colors[color_encoding];
    }

    static translate_packet(packet) { // packet = list of u8 (unsigned 8 bit integers)
        let x = packet[0];
        let y = packet[1];
        let color = packet[2];
        let type = ProtocolInterpreter.get_type(packet[3]);
        let type_specific = (packet[4] << 16) + (packet[5] << 8) + (packet[6]); // 24 bit number // for example amount of troops // check somewhere when overflow
        console.log(x + " " + y + "\t" + color + "\t" + type + " " + type_specific);

        let state = { type: type };
        return ({ x: x, y: y, state: state });
        // this.state.update(x, y, { type: type });
    }
}

const _ProtocolInterpreter = ProtocolInterpreter;
export { _ProtocolInterpreter as ProtocolInterpreter };

///////
// OUTSIDE
///////

// let canvas = document.getElementById('game-canvas');
// let state = new State(10, 10, canvas);
// let inp = new ProtocolInterpreter(state);

// inp.translate_packet([2, 1, 0, 2, 0, 3, 1, 1]);
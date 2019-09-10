import { State } from './State.js';

class ProtocolInterpreter {
    static get_type(type_encoding) {
        let types = { 0: 'ground', 1: 'fog', 2: 'king' };
        return types[type_encoding];
    }

    static get_color(color_encoding) {
        let colors = { 0: 'emtpy', 1: 'red', 2: 'green', 3: 'blue' };
        return colors[color_encoding];
    }

    static translate_packet(packet) { // packet = list of u8 (unsigned 8 bit integers)
        let x = packet[0];
        let y = packet[1];
        let color = ProtocolInterpreter.get_color(packet[2]);
        let type = ProtocolInterpreter.get_type(packet[3]);
        let type_specific = (packet[4] << 16) + (packet[5] << 8) + (packet[6]); // 24 bit number // for example amount of troops // check somewhere when overflow
        console.log(x + " " + y + "\t" + color + "\t" + type + " " + type_specific);

        let state = { type: type, color: color };
        return ({ x: x, y: y, state: state });
    }
}

const _ProtocolInterpreter = ProtocolInterpreter;
export { _ProtocolInterpreter as ProtocolInterpreter };
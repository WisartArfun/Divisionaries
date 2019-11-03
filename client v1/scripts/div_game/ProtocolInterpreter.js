"use strict";

import { State } from './State.js';

class ProtocolInterpreter {
    static get_type(type_encoding) {
        // let types = { 0: 'ground', 1: 'fog', 2: 'king' };
        let types = {'field': 'ground', 'king': 'king'};
        return types[type_encoding.toLowerCase()];
    }

    static get_color(color_encoding) {
        // let colors = { 0: 'emtpy', 1: 'red', 2: 'green', 3: 'blue' };
        let colors = {'empty': 'empty', 'red': 'red', 'blue': 'blue', 'green': 'green'};
        return colors[color_encoding.toLowerCase()];
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

    static translate_state(message) {
        console.log("translating state...")
        let state = [];
        for (let y in message) {
            let row = message[y];
            let part = [];
            for (let x in row) {
                let field = row[x];

                let type = Object.keys(field)[0];
                switch (type) {
                    case 'Field':
                        {
                            let color = ProtocolInterpreter.get_color(field[type]['color']);
                            let troops = field[type]['troops'];
                            type = ProtocolInterpreter.get_type(type);
                            part.push({'type': type, 'color': color, 'troops': troops});
                        } break;
                    case 'King':
                        {
                            let color = ProtocolInterpreter.get_color(field[type]['color']);
                            let troops = field[type]['troops'];
                            type = ProtocolInterpreter.get_type(type);
                            part.push({'type': type, 'color': color, 'troops': troops});
                        } break;
                    default:
                        alert('unknow protocol type');
                        break;
                }
            }
            state.push(part);
        }
        return state;
    }
}

export {ProtocolInterpreter};
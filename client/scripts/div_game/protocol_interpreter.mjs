"use strict";

function log(message) {
    console.log('[DivGame] - [ProInt] - ' + message);
}

class ProtocolInterpreter {
    static get_type(type_encoding) {
        type_encoding = type_encoding.toLowerCase();
        let types = { 'field': 'field', 'king': 'king', 'city': 'city', 'fog': 'fog' };
        if (type_encoding in types) return types[type_encoding];
        return type_encoding; // WARN: dangerous
    }

    static get_color(color_encoding) {
        color_encoding = color_encoding.toLowerCase();
        let colors = { 'empty': 'empty', 'red': 'red', 'blue': 'blue', 'green': 'green' };
        if (color_encoding in colors) return colors[color_encoding];
        return color_encoding;
    }

    static translate_state(message) {
        log("translating state...")
        let state = [];
        for (let y in message) {
            let row = message[y];
            let part = [];
            for (let x in row) {
                let field = row[x];
                part.push(ProtocolInterpreter.tranlate_field(field));
            }
            state.push(part);
        }
        return state;
    }

    static translate_update_state(message) {
        log('updating state...');
        let state = [];
        for (let i in message) {
            let [x, y, part] = message[i];
            state.push({
                'x': x,
                'y': y,
                'state': ProtocolInterpreter.tranlate_field(part),
            });
        }
        return state;
    }

    static tranlate_field(field) {
        let type = Object.keys(field)[0];
        switch (type) {
            case 'Field':
                {
                    let color = ProtocolInterpreter.get_color(field[type]['color']);
                    let troops = field[type]['troops'];
                    type = ProtocolInterpreter.get_type(type);
                    return { 'type': type, 'color': color, 'troops': troops };
                }
                break;
            case 'King':
                {
                    let color = ProtocolInterpreter.get_color(field[type]['color']);
                    let troops = field[type]['troops'];
                    type = ProtocolInterpreter.get_type(type);
                    return { 'type': type, 'color': color, 'troops': troops };
                }
                break;
            default:
                alert('unknow protocol type:', field);
                return { 'type': '', 'color': '', 'troops': '' };
                break;
        }
    }
}

export { ProtocolInterpreter };
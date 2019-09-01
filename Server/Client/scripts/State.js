import { Map } from './Renderer.js';

class State {
    constructor(x_size, y_size) { //, map) { // [DANGER] create map or pass map???
        this.state = {};
        this.state.x_fields = x_size;
        this.state.y_fields = y_size;
        this.maps = [];

        let col = new Array(this.state.x_fields);
        col.fill({ type: 'fog' });
        this.state.fields = new Array(this.state.y_fields);
        this.state.fields.fill(col);
    }

    add_map(canvas) {
        this.maps.push(new Map(canvas, this.state));
    }

    update(input) {
        let x = input.x;
        let y = input.y;
        let state = input.state;
        for (let i in this.maps) {
            this.maps[i].update_single_state(x, y, state);
        }
    }
}

/////////
// DONE OUTSIDE
/////////

// let canvas = document.getElementById('game-canvas');
// let state_interpreter = new State(10, 10, canvas);

// let kings = [
//     [1, 2],
//     [5, 8],
//     [8, 9],
//     [8, 1]
// ];
// for (let i in kings) {
//     let k = kings[i];
//     let x = k[0];
//     let y = k[1];
//     state_interpreter.update({ x: x, y: y, state: { type: 'king' } });
// }

const _State = State;
export { _State as State };
import { Map } from './Renderer.js';

class State {
    constructor(x_size, y_size, canvas) {
        this.state = {};
        this.state.x_fields = x_size;
        this.state.y_fields = y_size;

        let col = new Array(this.state.x_fields);
        col.fill({ type: 'fog' });
        this.state.fields = new Array(this.state.y_fields);
        this.state.fields.fill(col);

        this.map = new Map(canvas, this.state);
    }

    change(change) {
        let x = change.x;
        let y = change.y;
        let new_state = change.state;
        this.map.update_single_state(x, y, new_state);
    }
}

/////////
// DONE OUTSIDE
/////////

let canvas = document.getElementById('game-canvas');
let state_interpreter = new State(10, 10, canvas);

let kings = [
    [1, 2],
    [5, 8],
    [8, 9],
    [8, 1]
];
for (let i in kings) {
    let k = kings[i];
    let x = k[0];
    let y = k[1];
    state_interpreter.change({ x: x, y: y, state: { type: 'king' } });
}
import { Map } from './Renderer.js';



let state = {}
state.x_fields = 10;
state.y_fields = 10;
state.fields = []

for (let y = 0; y < state.y_fields; y += 1) {
    let col = [];
    for (let x = 0; x < state.x_fields; x += 1) {
        let field = {};
        field.type = 'fog';
        col.push(field);
    }
    state.fields.push(col);
}

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
    state.fields[y][x].type = 'king';
}

///// called from outside

let canvas = document.getElementById('game-canvas');

let field_size = 20;
let map = new Map(canvas, state, field_size);

function timeout() {
    setTimeout(function() {
        field_size -= 1;
        // map.update(state, field_size);
        map.update_size(field_size);
        timeout();
    }, 250);
}

// timeout()
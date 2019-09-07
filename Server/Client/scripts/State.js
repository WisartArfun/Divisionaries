import { Map } from './Renderer.js';

class State {
    constructor(x_size, y_size) {
        this.state = {};
        this.state.x_fields = x_size;
        this.state.y_fields = y_size;
        this.maps = [];

        let col = new Array(this.state.x_fields);
        col.fill({ type: 'fog', color: 'empty' }); // references to object => editing will affect whole row
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

const _State = State;
export { _State as State };
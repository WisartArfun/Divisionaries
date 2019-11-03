function log(message) {
    console.log('[DivGame] - [State] - ' + message);
}

log('imported');

class State {
    constructor(state) {
        log('new State created');
        this.state = undefined;
        this.set(state);
    }

    update(input) {
        log('updating state');
        let x = input.x;
        let y = input.y;
        let data = input.state;
        this.state[y][x] = data;
    }

    set(state) {
        log('setting state');
        this.state = state;
    }

    get_state() {
        return this.state;
    }
}

export { State };
"use strict";

import { ProtocolInterpreter } from './protocol_interpreter.mjs';
import { State } from './state.mjs';
import { Map } from './renderer.mjs';

function log(message) {
    console.log('[DivGame] - [Game] - ' + message);
}
log('imported');


class Game {
    constructor(canvas) {
        log('new game created');
        this.canvas = canvas; // QUES: why save??? bad to safe???
        this.started = false;
        // this.state = undefined; // useless step
        this.map = undefined;
    }

    start(state) {
        if (this.started) {
            log('can not start game, it is already running');
            return;
        }
        log('starting game...');
        this.started = true;
        state = ProtocolInterpreter.translate_state(state);
        // this.state = new State(state); // QUES: WARN: initialize State before???
        // this.map = new Map(this.canvas, this.state.get_state());
        this.map = new Map(this.canvas, state);
    }

    set_state(state) {
        log('state received');
        state = ProtocolInterpreter.translate_state(state);
        // this.state.set(state);
        this.map.set_state(state);
    }

    update_state(state) {
        log('updating state');
        state = ProtocolInterpreter.translate_update_state(state);
        for (let i in state) {
            // this.state.update(state[i]);
            this.map.update_state(state[i]);
        }
        // this.map.render();
    }
}

export { Game };
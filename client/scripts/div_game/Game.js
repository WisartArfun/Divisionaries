"use strict";

import { ProtocolInterpreter } from './ProtocolInterpreter.js';
import { State } from './State.js';
// import { GameConnection } from './GameConnection.js';

function log(message) {
    console.log('[Game] - ' + message);
}

log('Game was imported');

class Game {
    // constructor(canvas, size_x, size_y, ip, port) {
    //     this.size_x = size_x;
    //     this.size_y = size_y;

    //     this.ip = ip;
    //     this.port = port;

    //     this.started = false;

    //     this.game_connection = new GameConnection(ip, port);
    //     this.game_connection.start(this.update_state.bind(this));

    //     this.state = new State(this.size_x, this.size_y);
    //     this.state.add_map(canvas);
    // }

    constructor(ip, port, canvas_name) { //, load_game_callback) { // change this
        log('new Game created');
        this.ip = ip;
        this.port = port;

        this.canvas_name = canvas_name;
        // this.load_game_callback = load_game_callback;

        this.started = false;

        // this.game_connection = new GameConnection(ip, port);
        // this.game_connection.start(this.update_state.bind(this));
        // this.game_connection.start(this.receive_message.bind(this));
    }

    start_game() {
        this.size_x = 10;
        this.size_y = 10;

        // this.load_game_callback();

        this.canvas = document.getElementById(this.canvas_name);

        this.state = new State(this.size_x, this.size_y);
        this.state.add_map(this.canvas);
    }

    // ready() {
    //     this.game_connection.socket.send("ready");
    // }

    // receive_message(message) {
    //     if (!this.started) {
    //         if (message == "game_started") {
    //             this.started = true;
    //             this.start_game();
    //         }
    //     } else {
    //         console.log(message);
    //         this.update_state(message);
    //     }
    // }

    update_state(message) {
        let state_update = ProtocolInterpreter.translate_packet(message);
        this.state.update(state_update);
    }

    set_state(message) {
        let state = ProtocolInterpreter.translate_state(message);
        this.state.set(state);
    }
}

// let start_connection = function(ip, port, canvas_name) { //, callback) {
//     console.log("starting game connection");
//     let game = new Game(ip, port, canvas_name); //, callback);
//     game.start_game();

//     return game;
// }

// let start_game_instance = function(x_size, y_size, canvas_name, ip, port) {
//     let canvas = document.getElementById(canvas_name);
//     let game = new Game(canvas, x_size, y_size, ip, port);
// }

// const _start_game_instance = start_game_instance
// export { _start_game_instance as start_game_instance };

// export { start_connection };
export { Game };
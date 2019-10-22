"use strict";

// import { Game } from './Game.js';
import { Game } from './game.mjs';

function log(message) {
    console.log("[DivGame] - " + message);
}

log('imported DivGame');

let game_html = fetch('/files/nor_div_game.html').then(response => response.text());

class DivGame {
    constructor(id, ip, port) {
        log('initializing...');
        this.ready = false;
        let socket = new WebSocket('ws://' + ip + ':' + port);
        this.socket = socket;

        this.player_ready = false;
        this.running = false;

        this.x_pos = 2;
        this.y_pos = 0;

        let that = this;
        socket.onopen = function(event) {
            that.ready = true;
            log('ready');
            that.send('{"Lobby":"Join"}');

            socket.onmessage = function(event) {
                log("message received");
                try {
                    event.data.text().then(res => {
                        // if (this.running) {
                        //     game.then(div_game.handle_message(res));
                        // } else {
                        //     div_game.handle_message(res);
                        // }
                        that.handle_message(res);
                    });
                } catch (err) {
                    log("error occured while receiving message: " + err.message);
                    console.log(err); // handle or close ???
                }
            }

            socket.onclose = function(event) {
                log("closing...");
            }
        }

        window.addEventListener('keydown', function(event) {
            that.handle_input(event.keyCode);
        }, false);
    }

    handle_input(input) {
        if (!this.running) {
            return;
        }
        let ox = this.x_pos;
        let oy = this.y_pos;
        switch (input) {
            case 37: // Left
                this.x_pos -= 1;
                break;
            case 38: // Up
                this.y_pos -= 1;
                break;
            case 39: // Right
                this.x_pos += 1;
                break;
            case 40: // Down
                this.y_pos += 1;
                break;
        }
        let message = {
            "Running": {
                "Move": {
                    "Step": [
                        [ox, oy],
                        [this.x_pos, this.y_pos], 1.0
                    ]
                }
            }
        };
        this.send(JSON.stringify(message));
    }

    handle_message(message) {
        log('handling: ' + message);
        let parsed = JSON.parse(message);
        let first_key = Object.keys(parsed)[0];
        log("1. key: " + first_key);
        switch (first_key) {
            case 'Lobby':
                {
                    let second_key = parsed[first_key];
                    if (typeof second_key != 'string') {
                        second_key = Object.keys(parsed[first_key])[0];
                    }
                    log("2. key: " + second_key);
                    switch (second_key) {
                        case 'StartGame':
                            {
                                log('starting game...');
                                let game_container = document.getElementById("game-container");
                                window.game = game_html.then(text => {
                                    game_container.innerHTML = text;
                                    let canvas = document.getElementById('game-canvas'); // WARN: better solution
                                    let game = new Game(canvas);
                                    game.start(parsed[first_key]['StartGame']);
                                    this.running = true;
                                    return new Promise(function(resolve, reject) {
                                        resolve(game);
                                    });
                                });
                            }
                            break;
                        default:
                            {
                                log("An unknown message type was received - Lobby: " + JSON.stringify(parsed['Lobby']));
                                alert(JSON.stringify(parsed));
                            }
                            break;
                    }
                }
                break;
            case 'Running':
                {
                    // if (!this.running) {
                    //     log("[ERROR] - The game has not been started yet!!!");
                    //     return;
                    // }
                    let second_key = parsed[first_key];
                    if (typeof second_key != 'string') {
                        second_key = Object.keys(parsed[first_key])[0];
                    }
                    log("2. key: " + second_key);
                    switch (second_key) {
                        case 'StateUpdate':
                            {
                                game.then(game => game.update_state(parsed[first_key]['StateUpdate'])); // QUES: better solution
                            }
                            break;
                        case 'State':
                            {
                                game.then(game => {
                                    game.set_state(parsed[first_key]['State']);
                                }); // QUES: PROB: WARN: will order of execution stay the same??? what if state after stateupdate or something, wrong order???
                            }
                            break;
                        default:
                            {
                                log("An unknown message type was received - Running: " + JSON.stringify(parsed['Running']));
                                alert(JSON.stringify(parsed));
                            }
                            break;
                    }
                }
                break;
            default:
                {
                    log("An unknown message type was received: " + JSON.stringify(parsed));
                    alert(JSON.stringify(parsed));
                }
                break;
        }
    }

    // util
    send(message) {
        log("trying to send message");
        let div_game = this; // QUES: why is it redefined
        setTimeout(function() {
            if (!div_game.ready) { // QUES: better solution
                div_game.send(message);
                return;
            }
            log("sending message: " + message);
            div_game.socket.send(message);
            return;
        }, 10);
    }

    get_bucket_data() {
        let id = document.getElementById("bucket_id");
        this.send('{"GetLobbyLocation": "' + id.innerText + '"}');
    }

    // set player ready
    set_player_ready() {
        this.send('{"Lobby":"Ready"}');
        this.player_ready = true; // QUES: WARN: only switch when received?
    }

    set_player_not_ready() {
        this.send('{"Lobby":"NotReady"}');
        this.player_ready = false;
    }

    player_switch_ready(button_field, text_field) {
        if (this.player_ready) {
            this.set_player_not_ready();
        } else {
            this.set_player_ready();
        }
        document.getElementById(button_field).innerText = this.player_ready ? "Not Ready" : "Ready";
        document.getElementById(text_field).innerText = this.player_ready ? "The player is ready" : "The player is not ready";
    }
}

export { DivGame };
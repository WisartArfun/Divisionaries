"use strict";

function log(message) {
    console.log("[DivGame] - " + message);
}

log('imported DivGame');

class DivGame {
    constructor(id, ip, port) {
        log('initializing...');
        this.ready = false;
        let socket = new WebSocket('ws://' + ip + ':' + port);
        this.socket = socket;

        this.player_ready = false;

        let div_game = this;
        socket.onopen = function(event) {
            div_game.ready = true;
            log('ready');

            socket.onmessage = function(event) {
                log("message received");
                try {
                    event.data.text().then(res => div_game.handle_message(res));
                } catch (err) {
                    log("error occured while receiving message: " + err.message);
                    console.log(err); // handle or close ???
                }
            }

            socket.onclose = function(event) {
                log("closing...");
            }
        }
    }

    handle_message(message) {
        log('handling: ' + message);
        let parsed = JSON.parse(message);
        let first_key = Object.keys(parsed)[0];
        log("First key: " + first_key);
        switch (first_key) {
            case 'Lobby':
                {
                    let second_key = parsed[first_key];
                    console.log("second key: " + second_key);
                    switch (second_key) {
                        case 'StartGame':
                            let game_container = document.getElementById("game-container");
                            fetch('/files/nor_div_game.html')
                                .then(response => response.text())
                                .then(text => {
                                    game_container.innerHTML = text;
                                });
                            import ('./Game.js') .then((module) => module.start_connection(self.ip, self.port, 'game-canvas')); // PROB: QUES: WARN: better solution, pass canvas name somehow
                            
                            break;
                        default:
                            {
                                log("An unknown message type was received: " + parsed);
                                alert(JSON.stringify(parsed));
                            }
                            break;
                    }
                }
                break;
            default:
                {
                    log("An unknown message type was received: " + parsed);
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
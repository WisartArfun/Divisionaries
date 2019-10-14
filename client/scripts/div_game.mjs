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
        socket.onopne = function(event) {
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
        let parsed = JSON.parse(res);
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

    // function init_game_bucket(id, ip, port) {
    //     if (bucket_init) return; // two readies?
    //     bucket_init = true;
    //     console.log("connecting to " + ip + ":" + port + " called " + id);

    //     bucket_socket = new WebSocket('ws://' + ip + ':' + port);

    //     bucket_socket.onopen = function(event) {
    //         console.log("bucket open");
    //         bucket_ready = true;

    //         bucket_socket.onmessage = function(event) {
    //             event.data.text().then(res => {
    //                 console.log(res);
    //                 console.log(event);
    //                 try {
    //                     let parsed = JSON.parse(res);
    //                     let first_key = Object.keys(parsed)[0];
    //                     console.log("First key: " + first_key);
    //                     switch (first_key) {
    //                         case 'Lobby':
    //                             let second_key = parsed[first_key];
    //                             console.log("second key: " + second_key);
    //                             switch (second_key) {
    //                                 case 'StartGame':
    //                                     let game_container = document.getElementById("game-container");
    //                                     fetch('/files/nor_div_game.html')
    //                                         .then(response => response.text())
    //                                         .then(text => {
    //                                             game_container.innerHTML = text;
    //                                         });
    //                                     break;
    //                                 default:
    //                                     alert(JSON.stringify(parsed));
    //                             }
    //                             break;
    //                         default:
    //                             alert("1");
    //                             alert(JSON.stringify(parsed));
    //                     }
    //                 } catch (err) {
    //                     console.log(err.message);
    //                     console.log(err);
    //                 }
    //             });
    //         }

    //         bucket_socket.onclose = function(event) {
    //             console.log("connection with api server closed");
    //             bucket_closed = true;
    //         }
    //     }
    // }

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

    // let ready = false;
    // document.getElementById("ready_button").innerText = ready ? "Not Ready" : "Ready";

    // set player ready
    player_ready() {
        this.send('{"Lobby":"Ready"}');
        this.ready = true; // QUES: WARN: only switch when received?
    }

    player_not_ready() {
        this.send('{"Lobby":"NotReady"}');
        this.ready = false;
    }

    player_switch_ready(button_field, text_field) {
        if (ready) {
            player_not_ready();
        } else {
            player_ready();
        }
        document.getElementById(button_field).innerText = ready ? "Not Ready" : "Ready";
        document.getElementById(text_field).innerText = ready ? "The player is ready" : "The player is not ready";
    }
}

export { DivGame };
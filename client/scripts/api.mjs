"use strict";

function log(message) {
    console.log("[API] - " + message);
}

log("imported api.mjs");

class Api {
    constructor(ip, port) {
        log("initializing...");
        this.ready = false;
        let socket = new WebSocket('ws://' + ip + ':' + port);
        this.socket = socket;

        let api = this;
        socket.onopen = function(event) {
            log("ready");
            api.ready = true;

            socket.onmessage = function(event) {
                log("message received");
                try {
                    event.data.text().then(res => api.handle_message(res));
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
        log("handling: " + message);
        let parsed = JSON.parse(message);
        let first_key = Object.keys(parsed)[0];
        switch (first_key) {
            // general
            case 'JoinGame':
                {
                    window.location.href = "/games/" + parsed['JoinGame'];
                }
                break;
            case 'OpenLobbies':
                {
                    let content = "<table><tr><th>Lobby Id</th><th>Players</th><th>Max Players</th></tr>";
                    let running_games = parsed['OpenLobbies'];
                    for (let lobby in running_games) {
                        content += '<tr><th><button onclick="join_div_game_direct()">' + running_games[lobby]['id'] + "</button></th><th>" + running_games[lobby]['current_users'] + "</th><th>" + running_games[lobby]['max_user_size'] + "</th></tr>";
                    }
                    document.getElementById("open_lobbies").innerHTML = content;
                }
                break;
            case 'RunningGames':
                {
                    let content = "<table><tr><th>Game Id</th><th>Players</th><th>Max Players</th></tr>";
                    let running_games = parsed['OpenLobbies'];
                    for (game in running_games) {
                        content += "<tr><th>" + running_games[game]['id'] + "</th><th>" + running_games[game]['current_users'] + "</th><th>" + running_games[game]['max_user_size'] + "</th></tr>";
                    }
                    document.getElementById("running_games").innerHTML = content;
                }
                break;

                // in bucket lobby // QUES: WARN: check where at or just do???
            case 'LobbyLocation':
                {
                    let data = parsed[first_key];
                    import ('./scripts/Game.js')
                    .then((module) => module.init_game_bucket(data[0], data[1], data[2]));
                }
                break;
            default:
                {
                    log("An unknown message type was received");
                    alert(parsed);
                }
                break;
        }
    }



    // api_socket.onopen = function(event) {
    //     console.log("api started");
    //     api_ready = true;

    //     api_socket.onmessage = function(event) {
    //         event.data.text().then(res => {
    //             console.log(res);
    //             console.log(event);
    //             try {
    //                 let parsed = JSON.parse(res);
    //                 let first_key = Object.keys(parsed)[0];
    //                 switch (first_key) {
    //                     // general
    //                     case 'JoinGame':
    //                         window.location.href = "/games/" + parsed['JoinGame'];
    //                         break;
    //                     case 'OpenLobbies':
    //                         content = "<table><tr><th>Lobby Id</th><th>Players</th><th>Max Players</th></tr>";
    //                         running_games = parsed['OpenLobbies'];
    //                         for (lobby in running_games) {
    //                             content += '<tr><th><button onclick="join_div_game_direct()">' + running_games[lobby]['id'] + "</button></th><th>" + running_games[lobby]['current_users'] + "</th><th>" + running_games[lobby]['max_user_size'] + "</th></tr>";
    //                         }
    //                         document.getElementById("open_lobbies").innerHTML = content;
    //                         break;
    //                     case 'RunningGames':
    //                         content = "<table><tr><th>Game Id</th><th>Players</th><th>Max Players</th></tr>";
    //                         running_games = parsed['OpenLobbies'];
    //                         for (game in running_games) {
    //                             content += "<tr><th>" + running_games[game]['id'] + "</th><th>" + running_games[game]['current_users'] + "</th><th>" + running_games[game]['max_user_size'] + "</th></tr>";
    //                         }
    //                         document.getElementById("running_games").innerHTML = content;
    //                         break;
    //                         // in bucket lobby
    //                     case 'LobbyLocation':
    //                         data = parsed[first_key];
    //                         // let script = document.createElement('script');
    //                         // script.src = "/scripts/div_game_index.js";
    //                         // document.body.appendChild(script);
    //                         // script.onload = function(event) {
    //                         //     console.log("initializing game bucket");
    //                         //     init_game_bucket(data[0], data[1], data[2]);
    //                         // };
    //                         // api_socket.close();

    //                         init_game_bucket(data[0], data[1], data[2]);
    //                         break;
    //                     default:
    //                         alert(parsed);
    //                 }
    //             } catch (err) {
    //                 console.log(err.message);
    //                 console.log(err);
    //             }
    //         });
    //     }

    //     api_socket.onclose = function(event) {
    //         console.log("connection with api server closed");
    //     }

    // util
    send(message) {
        log("trying to send message");
        let api = this; // QUES: why is it redefined
        setTimeout(function() {
            log(api.ready);
            if (!api.ready) { // QUES: better solution
                api.send(message);
                return;
            }
            log("sending message: " + message);
            api.socket.send(message);
            return;
        }, 10);
    }

    // general
    join_div_game_normal() {
        this.send('"JoinDivGameNormal"');
    }

    join_div_game_id(id) {
        this.send('{"JoinDivGameDirect": "' + id + '"}');
    }

    join_div_game_direct(field) {
        let lobby_id_input = document.getElementById(field);
        this.join_div_game_id(lobby_id_input.value);
        // send('{"JoinDivGameDirect": "' + lobby_id_input.value + '"}');
    }

    get_open_lobbies() {
        this.send('"GetOpenLobbies"');
    }

    get_running_games() {
        this.send('"GetRunningGames"');
    }

    // bucket lobby
    get_bucket_data(bucket_id) { // QUES: PROB: better solution
        // let id = document.getElementById(bucket_id);
        // this.send('{"GetLobbyLocation": "' + id.innerText + '"}');
        this.send('{"GetLobbyLocation": "' + bucket_id + '"}');
    }
}

export { Api };
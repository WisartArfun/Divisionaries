let bucket_init = false;
let bucket_ready = false;
let bucket_socket = null;
let bucket_closed = false;

function init_game_bucket(id, ip, port) {
    if (bucket_init) return; // two readies?
    bucket_init = true;
    console.log("connecting to " + ip + ":" + port + " called " + id);

    bucket_socket = new WebSocket('ws://' + ip + ':' + port);

    bucket_socket.onopen = function(event) {
        console.log("bucket open");
        bucket_ready = true;

        bucket_socket.onmessage = function(event) {
            event.data.text().then(res => {
                console.log(res);
                console.log(event);
                try {
                    let parsed = JSON.parse(res);
                    let first_key = Object.keys(parsed)[0];
                    console.log("First key: " + first_key);
                    switch (first_key) {
                        case 'Lobby':
                            let second_key = parsed[first_key];
                            console.log("second key: " + second_key);
                            switch (second_key) {
                                case 'StartGame':
                                    let game_container = document.getElementById("game-container");
                                    fetch('/files/nor_div_game.html')
                                        .then(response => response.text())
                                        .then(text => {
                                            game_container.innerHTML = text;
                                            let script = document.createElement('script');
                                            script.type = "module";
                                            script.src = "/scripts/game_template.js";
                                            document.head.appendChild(script);
                                            // let script = document.createElement('script');
                                            // script.type = "module";
                                            // // script.src = "/scripts/Game.js";
                                            // text = "import { start_connection } from '/scripts/Game.js'; start_connection(" + ip + "," + port + ", 'game-canvas');"
                                            // console.log(text);
                                            // // script.innerHTML = "import { start_connection } from '/scripts/Game.js'; start_connection(" + text + ");";
                                            // // game_container.appendChild(script);
                                            // document.body.appendChild(script);
                                            // script.onload = function(event) {
                                            //     console.log("script has been loaded");
                                            //     start_connection(ip, port, 'game-canvas');
                                            // };
                                            // // let script = document.createElement('script');
                                            // // script.type = "javascript/application";
                                            // // script.src = "/scripts/Game.js";
                                            // // script.onload = function(event) {
                                            // //     start_connection("")
                                            // // }
                                            // console.log("loaded game");
                                        })
                                    break;
                                default:
                                    alert(JSON.stringify(parsed));
                            }
                            break;
                        default:
                            alert("1");
                            alert(JSON.stringify(parsed));
                    }
                } catch (err) {
                    console.log(err.message);
                    console.log(err);
                }
            });
        }

        bucket_socket.onclose = function(event) {
            console.log("connection with api server closed");
            bucket_closed = true;
        }
    }
}

function send_bucket(message) {
    if (bucket_closed) {
        console.log("The connection to the bucket server was already closed");
        return;
    }
    setTimeout(function() {
        if (!api_ready) {
            send_bucket(message);
            return;
        }
        console.log("sending message to bucket: " + message);
        bucket_socket.send(message);
        return;
    }, 10);
}

function get_bucket_data() {
    let id = document.getElementById("bucket_id");
    send_api('{"GetLobbyLocation": "' + id.innerText + '"}');
}

let ready = false;
document.getElementById("ready_button").innerText = ready ? "Not Ready" : "Ready";

function player_ready() {
    send_bucket('{"Lobby":"Ready"}');
    ready = true;
}

function player_not_ready() {
    send_bucket('{"Lobby":"NotReady"}');
    ready = false;
}

function player_switch_ready() {
    if (ready) {
        player_not_ready();
    } else {
        player_ready();
    }
    document.getElementById("ready_button").innerText = ready ? "Not Ready" : "Ready";
    document.getElementById("player_ready").innerText = ready ? "The player is ready" : "The player is not ready";
}
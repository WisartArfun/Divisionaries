let api_init = false;
let api_ready = false;
let api_socket = null;
let api_closed = false;
let bucket_init = false;
let bucket_ready = false;
let bucket_socket = null;
let bucket_closed = false;

open_api_bucket('127.0.0.1', '8050');

function open_api_bucket(ip, port) {
    if (api_init) return;
    api_init = true;

    api_socket = new WebSocket('ws://' + ip + ':' + port);
    
    api_socket.onopen = function(event) {
        api_ready = true;
        get_bucket_data();

        api_socket.onmessage = function(event) {
            event.data.text().then(res => {
                console.log(res);
                console.log(event);
                try {
                    let parsed = JSON.parse(res);
                    let first_key = Object.keys(parsed)[0];
                    switch (first_key) {
                        case 'LobbyLocation':
                            data = parsed[first_key];
                            init_bucket_api(data[0], data[1], data[2]);
                            api_socket.close();
                            console.log("api closed");
                            break;
                        default:
                            alert(parsed);
                    }
                } catch (err) {
                    console.log(err.message);
                    console.log(err);
                }
            });
        }

        api_socket.onclose = function(event) {
            api_closed = true;
            console.log("connection with api server closed");
        }
    }
}

function init_bucket_api(id, ip, port) {
    if (bucket_init) return; // two readies?
    bucket_init = true;

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
                    switch (first_key) {
                        // case 'LobbyLocation':
                        //     data = parsed[first_key];
                        //     socket = new WebSocket('ws://' + data[1] + ':' + data[2]);
                        //     console.log("hi");
                        //     break;
                        default: alert(parsed);
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

function send_api(message) {
    if (api_closed) {
        console.log("The connection to the api server was already closed");
        return;
    }
    while (!api_ready) {} // sleep
    console.log("send to api: " + message);
    api_socket.send(message);
}

function send_bucket(message) {
    if (bucket_closed) {
        console.log("The connection to the bucket server was already closed");
        return;
    }
    while (!bucket_ready) {} // sleep
    console.log("send to bucket: " + message);
    bucket_socket.send(message);
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
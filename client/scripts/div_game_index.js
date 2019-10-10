let ready = false;
let socket = new WebSocket('ws://127.0.0.1:8020');
let bucket_ready = false;
let bucket_socket = null;

socket.onopen = function(event) {
    ready = true;

    socket.onmessage = function(event) {
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

    socket.onclose = function(event) {
        console.log("connection with api server closed");
    }
}

function init_bucket_api(id, ip, port) {
    bucket_socket = new WebSocket('ws://' + data[1] + ':' + data[2]);

    bucket_socket.onopen = function(event) {
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
                        default:
                            alert(parsed);
                    }
                } catch (err) {
                    console.log(err.message);
                    console.log(err);
                }
            });
        }

        bucket_socket.onclose = function(event) {
            console.log("connection with api server closed");
        }
    }
}

function send(message) {
    while (!ready) {} // sleep
    socket.send(message);
}

function get_bucket_data() {
    let id = document.getElementById("bucket_id");
    socket.send('{"GetLobbyLocation": "' + id.innerText + '"}');
}
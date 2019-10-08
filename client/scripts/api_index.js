
let ready = false;
let socket = new WebSocket('ws://localhost:8030');
socket.onopen = function(event) {
    ready = true;

    socket.onmessage = function(event) {
        event.data.text().then(res => {
            console.log(res);
            console.log(event);
            try {
                let parsed = JSON.parse(res);
                var first_key = Object.keys(parsed)[0];
                switch(first_key) {
                    case 'JoinGame':
                        window.location.href = "/games/" + parsed['JoinGame'];
                        break;
                    default:
                        alert('default');
                }
            }
            catch(err) {
                console.log(err.message);
                console.log(err);
            }
        });
    }

    socket.onclose = function(event) {
        console.log("connection with api server closed");
    }
}

function send(message) {
    while (!ready) {} // sleep
    socket.send(message);
}

function join_div_game_direct() { // change this
    let lobby_id_input = document.getElementById("lobby_id_input");
    let input = lobby_id_input.value;
    window.location.href = "/games/" + input;
}

function join_div_game_normal() {
    send('"JoinDivGameNormal"');
}
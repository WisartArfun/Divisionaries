console.log("starting api");
let api_ready = false;
let api_socket = new WebSocket('ws://127.0.0.1:8050');

api_socket.onopen = function(event) {
    console.log("api started");
    api_ready = true;

    api_socket.onmessage = function(event) {
        event.data.text().then(res => {
            console.log(res);
            console.log(event);
            try {
                let parsed = JSON.parse(res);
                let first_key = Object.keys(parsed)[0];
                switch (first_key) {
                    // general
                    case 'JoinGame':
                        window.location.href = "/games/" + parsed['JoinGame'];
                        break;
                    case 'OpenLobbies':
                        content = "<table><tr><th>Lobby Id</th><th>Players</th><th>Max Players</th></tr>";
                        running_games = parsed['OpenLobbies'];
                        for (lobby in running_games) {
                            content += '<tr><th><button onclick="join_div_game_direct()">' + running_games[lobby]['id'] + "</button></th><th>" + running_games[lobby]['current_users'] + "</th><th>" + running_games[lobby]['max_user_size'] + "</th></tr>";
                        }
                        document.getElementById("open_lobbies").innerHTML = content;
                        break;
                    case 'RunningGames':
                        content = "<table><tr><th>Game Id</th><th>Players</th><th>Max Players</th></tr>";
                        running_games = parsed['OpenLobbies'];
                        for (game in running_games) {
                            content += "<tr><th>" + running_games[game]['id'] + "</th><th>" + running_games[game]['current_users'] + "</th><th>" + running_games[game]['max_user_size'] + "</th></tr>";
                        }
                        document.getElementById("running_games").innerHTML = content;
                        break;
                        // in bucket lobby
                    case 'LobbyLocation':
                        data = parsed[first_key];
                        let script = document.createElement('script');
                        script.src = "/scripts/div_game_index.js";
                        document.body.append(script);
                        script.onload = function(event) {
                            console.log("initializing game bucket");
                            init_game_bucket(data[0], data[1], data[2]);
                        };
                        // api_socket.close();
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
        console.log("connection with api server closed");
    }
}

// util
function send_api(message) {
    setTimeout(function() {
        if (!api_ready) {
            send_api(message);
            return;
        }
        console.log("sending message to api");
        api_socket.send(message);
        return;
    }, 10);
}

// general
function join_div_game_normal() {
    send_api('"JoinDivGameNormal"');
}

function join_div_game_id(id) {
    send_api('{"JoinDivGameDirect": "' + id + '"}');
}

function join_div_game_direct(id) {
    let lobby_id_input = document.getElementById("lobby_id_input");
    join_div_game_id(lobby_id_input.value);
    // send('{"JoinDivGameDirect": "' + lobby_id_input.value + '"}');
}

function get_open_lobbies() {
    send_api('"GetOpenLobbies"');
}

function get_running_games() {
    send_api('"GetRunningGames"');
}

// bucket lobby
function get_bucket_data() {
    let id = document.getElementById("bucket_id");
    send_api('{"GetLobbyLocation": "' + id.innerText + '"}');
}
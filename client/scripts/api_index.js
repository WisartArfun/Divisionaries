let ready = false;
let socket = new WebSocket('ws://127.0.0.1:8020');
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

function send(message) {
    while (!ready) {} // sleep
    socket.send(message);
}

function join_div_game_normal() {
    send('"JoinDivGameNormal"');
}

function join_div_game_id(id) {
    send('{"JoinDivGameDirect": "' + id + '"}');
}

function join_div_game_direct(id) {
    let lobby_id_input = document.getElementById("lobby_id_input");
    join_div_game_id(lobby_id_input.value);
    // send('{"JoinDivGameDirect": "' + lobby_id_input.value + '"}');
}

function get_open_lobbies() {
    send('"GetOpenLobbies"');
}

function get_running_games() {
    send('"GetRunningGames"');
}
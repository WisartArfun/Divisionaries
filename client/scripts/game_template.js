// import { start_game_instance } from "/scripts/Game.js";
// import { start_connection } from "./Game";
import { start_connection } from "/scripts/div_game/Game.js";

// start_game_instance(10, 10, 'game-canvas', '#IP#', '#PORT#');
// start_game_instance(10, 10, 'game-canvas', '', '');
// start_connection("#IP#", "#PORT#", callback);
start_connection("127.0.0.1", "8022", 'game-canvas');
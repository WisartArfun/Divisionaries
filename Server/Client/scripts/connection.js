import { ProtocolInterpreter } from './ProtocolInterpreter.js';
import { State } from './State.js';

let canvas = document.getElementById('game-canvas');
let state = new State(10, 10, canvas);
let inp = new ProtocolInterpreter(state);

// inp.translate_packet([2, 1, 0, 2, 0, 3, 1, 1]);

var socket = new WebSocket('ws://localhost:9001');

// Open the socket
socket.onopen = function(event) {

    // Send an initial message
    // socket.send('I am the client and I\'m listening!');

    // Listen for messages
    socket.onmessage = function(event) {
        console.log('Client received a message', event);
        console.log(event.data.split(''));
        inp.translate_packet(event.data.split(''));
        // socket.send(event.data);
    };

    // Listen for socket closes
    socket.onclose = function(event) {
        console.log('Client notified socket has closed', event);
    };

    // To close the socket....
    //socket.close()

    socket.send("hello");

};
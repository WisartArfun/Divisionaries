import { GraphicMapper } from './GraphicMapping.js';

class Sprite {
    constructor(src, x_pos, y_pos, width, height, ctx) {
        this.x_pos = x_pos;
        this.y_pos = y_pos;
        this.width = width;
        this.height = height;
        this.ctx = ctx;

        this.image = new Image();
        this.image.src = src;
        this.image.onload = () => {
            this.render(this.x_pos, this.y_pos);
        };
    }

    render(x_pos, y_pos) {
        this.x_pos = x_pos;
        this.y_pos = y_pos;
        this.ctx.drawImage(this.image, this.x_pos, this.y_pos, this.width, this.height);
    }

    change_src(src) {
        this.src = src;
        this.image.src = this.src;
    }

    update(size, x_pos, y_pos) {
        this.width = size;
        this.height = size;
        this.x_pos = x_pos;
        this.y_pos = y_pos;
        this.ctx.drawImage(this.image, this.x_pos, this.y_pos, this.width, this.height);
    }
}

class Field {
    constructor(x_num, y_num, field_size, ctx) {
        this.x_num = x_num;
        this.y_num = y_num;
        this.field_size = field_size;
        this.ctx = ctx;

        this.x_pos = this.x_num * this.field_size;
        this.y_pos = this.y_num * this.field_size;
        this.sprite = new Sprite("", this.x_pos, this.y_pos, this.field_size, this.field_size, this.ctx);
    }

    change_field_type(type) {
        let src = GraphicMapper.get_src(type);
        this.sprite.change_src(src);
    }

    change_field_size(field_size) {
        this.field_size = field_size;
        this.x_pos = this.x_num * this.field_size;
        this.y_pos = this.y_num * this.field_size;
        this.sprite.update(this.field_size, this.x_pos, this.y_pos);
    }
}

class Map {
    constructor(canvas, state, field_size) {
        this.canvas = canvas;
        this.ctx = this.canvas.getContext("2d");

        this.x_fields = state.x_fields;
        this.y_fields = state.y_fields;

        this.fields = [];

        for (let y = 0; y < this.x_fields; y += 1) {
            let col = [];
            for (let x = 0; x < this.y_fields; x += 1) {
                let obj = new Field(y, x, field_size, this.ctx);
                col.push(obj);
            }
            this.fields.push(col);
        }

        this.update(state, field_size);
    }

    update(state, field_size = this.field_size) { // add field size update
        this.state = state;
        this.field_size = field_size;

        this.canvas.width = this.x_fields * this.field_size;
        this.canvas.height = this.y_fields * this.field_size;

        for (let y = 0; y < this.x_fields; y += 1) {
            for (let x = 0; x < this.y_fields; x += 1) {
                let obj = this.fields[y][x];
                let type = this.state.fields[y][x].type;
                obj.change_field_type(type);
                obj.change_field_size(this.field_size);
            }
        }
    }
}


///////
// WHAT IS DONE BY STATE
///////

state = {}
state.x_fields = 10;
state.y_fields = 10;
state.fields = []

for (let i = 0; i < state.x_fields; i += 1) {
    let col = [];
    for (let i = 0; i < state.x_fields; i += 1) {
        let field = {};
        field.type = 'fog';
        col.push(field);
    }
    state.fields.push(col);
}

let kings = [
    [1, 2],
    [5, 8],
    [8, 9],
    [8, 1]
];
for (i in kings) {
    let k = kings[i];
    let x = k[0];
    let y = k[1];
    state.fields[x][y].type = 'king';
}

console.log(state);
///// called from outside

let canvas = document.getElementById('game-canvas');

field_size = 20;
map = new Map(canvas, state, field_size);

// function sleep(ms) {
//     return new Promise(resolve => setTimeout(resolve, ms));
// }

// while (true) {
// sleep(2000).then(() =>{
// field_size -= 5;
// console.log(field_size);
// map.update(state, field_size);
// });
function timeout() {
    setTimeout(function() {
        field_size -= 1;
        console.log(field_size);
        map.update(state, field_size);
        timeout();
    }, 250);
}

// timeout();
// }

// map.fields[5][3].change_field_type('fog');

// Map = new Map(canvas, 10, 10, 80);
// Map.fields[0][2].change_field_type('king');
// Map.fields[8][7].change_field_type('king');
// Map.fields[5][4].change_field_type('fog')
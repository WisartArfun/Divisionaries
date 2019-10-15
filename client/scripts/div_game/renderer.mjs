import { GraphicMapper } from './graphic_mapper.mjs';

// SPRITE   - an object, that can be displayed on a canvas
class Sprite {
    // CONSTRUCTOR  - initialize content, transform and connection to display
    constructor(src, x_pos, y_pos, width, height, ctx) {
        // transform
        this.set_position(x_pos, y_pos);
        this.set_size(width, height);

        // function
        this.ctx = ctx;
        this.image = new Image();
        this.image.onload = () => {
            this.render(this.x_pos, this.y_pos);
        };

        // content
        this.set_src(src);
        this.set_color('');
    }

    // CONTENT  - image src
    set_src(src) {
        this.src = src;
        this.image.src = this.src;
    }

    set_color(color) {
        this.color = color;
    }

    // TRANSFORM    - position and size
    set_position(x_pos, y_pos) {
        this.x_pos = x_pos;
        this.y_pos = y_pos;
    }

    set_size(width, height) {
        this.width = width;
        this.height = height;
    }

    // FUNCTION - display image on screen
    update_transform(x_pos, y_pos, width, height) {
        this.set_position(x_pos, y_pos);
        this.set_size(width, height);
        this.render();
    }

    render() {
        this.ctx.drawImage(this.image, this.x_pos, this.y_pos, this.width, this.height)
        this.ctx.fillStyle = this.color;
        this.ctx.fillRect(this.x_pos, this.y_pos, this.width, this.height);
    }
}

// FIELD - logical field on map, manages sprite and meta data
class Field {
    // CONSTRUCTOR  - initializes position (logical), initializes sprite
    constructor(x_num, y_num, state, size, ctx) {
        this.size = size;

        let src = GraphicMapper.get_src(state.type);
        this.sprite = new Sprite(src, state.color, x_num, y_num, size, size, ctx);

        this.update_transform(size);
    }

    set_state(state) {
        this.set_type(state.type);
        this.set_color(state.color);
    }

    // FUNCTION - communication with sprite
    set_type(type) {
        this.sprite.set_src(GraphicMapper.get_src(type));
    }

    set_color(color) {
        this.sprite.set_color(GraphicMapper.get_color(color));
    }

    update_transform(size) {
        this.size = size;
        this.sprite.update_transform(this.x_num * this.size, this.y_num * this.size, this.size, this.size);
    }

    render() {
        this.sprite.render();
    }
}

// MAP  - collection of logical fields and meta data management
class Map {
    // CONSTRUCTOR  - initializes all the fields, sets content, sets transform
    constructor(canvas, state, field_size = 30) {
        let ctx = this.canvas.getContext("2d");

        this.fields = [];
        for (let y in state) {
            let row = state[y];
            let col = [];
            for (let x in row) {
                col.push(new Field(x, y, state[y][x], field_size, ctx));
            }
            this.fields.push(col);
        }

        this.render();
        // this.update_size(field_size);
    }

    render() {
        for (y in this.fields) {
            for (x in this.fields[y]) {
                this.fields[y][x].render();
            }
        }
    }

    // // CONTENT
    // update_state(state) {
    //     for (let y in state) {
    //         let row = state[y];
    //         for (let x in row) {
    //             // this.fields[y][x].update_state(state.fields[y][x]);
    //             // this.update_single_state(x, y, state.fields[y][x]); // [DANGER] unnecessary abstraction
    //             if (x == 0 && y == 0) console.log(state);
    //             this.update_single_state(x, y, row[x]);
    //         }
    //     }
    //     this.update_size(this.field_size);
    // }

    // // TRANSFORM/FUNCTION
    // update_size(field_size) {
    //     this.field_size = field_size;

    //     this.canvas.width = this.x_fields * this.field_size;
    //     this.canvas.height = this.y_fields * this.field_size;

    //     for (let y = 0; y < this.x_fields; y += 1) {
    //         for (let x = 0; x < this.y_fields; x += 1) {
    //             this.fields[y][x].update_transform(this.field_size);
    //         }
    //     }
    // }

    // // FUNCTION
    // update_single_state(x, y, single_state, field_size = this.field_size) { // change field size when selected? only state should affect map???
    //     this.fields[y][x].update_state(single_state);
    // }
}

export { Map };
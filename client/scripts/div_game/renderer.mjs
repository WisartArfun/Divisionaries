import { GraphicMapper } from './graphic_mapper.mjs';

// SPRITE   - an object, that can be displayed on a canvas
class Sprite {
    // CONSTRUCTOR  - initialize content, transform and connection to display
    constructor(src, color, x_pos, y_pos, width, height, ctx) {
        this.set_color(color);
        this.ctx = ctx;
        this.update_transform(x_pos, y_pos, width, height);

        this.image = new Image();
        this.set_src(src);
        this.image.onload = () => {
            this.render();
        };
    }

    update_transform(x_pos, y_pos, width, height) {
        this.x_pos = x_pos;
        this.y_pos = y_pos;
        this.width = width;
        this.height = height;
    }

    set_color(color) {
        this.color = color;
    }

    set_src(src) {
        this.image.src = src;
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
        this.x_num = x_num;
        this.y_num = y_num;

        let src = GraphicMapper.get_src(state.type);
        let color = GraphicMapper.get_color(state.color);
        this.sprite = new Sprite(src, color, x_num * size, y_num * size, size, size, ctx);
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
        this.sprite.update_transform(this.x_num * size, this.y_num * size, size, size);
    }

    render() {
        this.sprite.render();
    }
}

// MAP  - collection of logical fields and meta data management
class Map {
    // CONSTRUCTOR  - initializes all the fields, sets content, sets transform
    constructor(canvas, state, field_size = 30) {
        this.field_size = field_size;
        canvas.width = field_size * state.length;
        canvas.height = field_size * state[0].length; // QUES: what if x == 0???
        this.ctx = canvas.getContext("2d");

        this.set_state(state);

        // this.fields = [];
        // for (let y in state) {
        //     let row = state[y];
        //     let col = [];
        //     for (let x in row) {
        //         col.push(new Field(x, y, state[y][x], field_size, ctx));
        //     }
        //     this.fields.push(col);
        // }

        // this.render(); // QUES: sprite renders automatically
    }

    render() {
        for (let y in this.fields) {
            for (let x in this.fields[y]) {
                this.fields[y][x].render();
            }
        }
    }

    set_state(state) {
        this.fields = [];
        for (let y in state) {
            let row = state[y];
            let col = [];
            for (let x in row) {
                let field = new Field(x, y, state[y][x], this.field_size, this.ctx);
                col.push(field);
            }
            this.fields.push(col);
        }
    }

    update_state(input) {
        let x = input.x;
        let y = input.y;
        let data = input.state;
        this.fields[y][x].set_state(data);
        // this.fields[y][x].render();
    }
}

export { Map };
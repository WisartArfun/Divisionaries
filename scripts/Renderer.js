// GRAPHIC_MAPPER   - maps logical types to sources
import { GraphicMapper } from './GraphicMapping.js';

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
    }

    // CONTENT  - image src
    set_src(src) {
        this.src = src;
        this.image.src = this.src;
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
        this.ctx.drawImage(this.image, this.x_pos, this.y_pos, this.width, this.height);
    }
}

// FIELD - logical field on map, manages sprite and meta data
class Field {
    // CONSTRUCTOR  - initializes position (logical), initializes sprite
    constructor(x_num, y_num, size, ctx) {
        // function
        this.sprite = new Sprite("", 0, 0, size, size, ctx); // [WARNING] initializing this way executes certain functions several times

        // ~transform
        this.set_nums(x_num, y_num);
        this.update_transform(size); //

        // ~content
        this.update_state({}); // [WARNING] find a better solution than {}; default state
    }

    // CONTENT  - state (logical), type (logical)
    set_state(state) {
        this.state = state;
    }

    update_state(state) { // [WARNING] better division of update and set?
        this.set_state(state);
        this.set_type(this.state.type);
    }

    // TRANSFORM - position in map (logical), size of a field (for sprite :( => initialize sprites in map???)
    set_nums(x_num, y_num) {
        this.x_num = x_num;
        this.y_num = y_num;
    }

    set_size(field_size) {
        this.size = field_size;
    }

    // FUNCTION - communication with sprite
    set_type(type) {
        this.sprite.set_src(GraphicMapper.get_src(type));
    }

    update_transform(field_size) {
        this.set_size(field_size);
        this.sprite.update_transform(this.x_num * this.size, this.y_num * this.size, this.size, this.size);
    }
}

// MAP  - collection of logical fields and meta data management
class Map {
    // CONSTRUCTOR  - initializes all the fields, sets content, sets transform
    constructor(canvas, state, field_size) {
        this.canvas = canvas;
        let ctx = this.canvas.getContext("2d");

        this.x_fields = state.x_fields;
        this.y_fields = state.y_fields;

        this.fields = [];
        for (let y = 0; y < this.x_fields; y += 1) {
            let col = [];
            for (let x = 0; x < this.y_fields; x += 1) {
                col.push(new Field(x, y, field_size, ctx));
            }
            this.fields.push(col);
        }

        this.update_state(state); // [WARNING] less overhead when updating together? when applied?
        this.update_size(field_size);
    }

    // CONTENT
    update_state(state) {
        for (let y = 0; y < this.x_fields; y += 1) {
            for (let x = 0; x < this.y_fields; x += 1) {
                // this.fields[y][x].update_state(state.fields[y][x]);
                this.update_single_state(x, y, state.fields[y][x]); // [DANGER] unnecessary abstraction
            }
        }
    }

    // TRANSFORM/FUNCTION
    update_size(field_size) {
        this.field_size = field_size;

        this.canvas.width = this.x_fields * this.field_size;
        this.canvas.height = this.y_fields * this.field_size;

        for (let y = 0; y < this.x_fields; y += 1) {
            for (let x = 0; x < this.y_fields; x += 1) {
                this.fields[y][x].update_transform(this.field_size);
            }
        }
    }

    // FUNCTION
    update_single_state(x, y, single_state, field_size = this.field_size) { // change field size when selected? only state should affect map???
        this.fields[y][x].update_state(single_state);
    }
}

const _Map = Map;
export { _Map as Map };
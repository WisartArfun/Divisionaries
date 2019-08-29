var c = document.getElementById('game-canvas');
c.height = 500;
c.width = 500;

field_size = 50;

var ctx = c.getContext('2d');

class GraphicMapper {
    static get_src(type, mapping = 'default') {
        let field_types = {
            default: {
                ground: "graphics/empty.jpg",
                king: "graphics/crown.jpg",
            },
        };

        return field_types[mapping][type];
    }
}

class Sprite {
    constructor(src, x_pos, y_pos, width, height) {   
        this.x_pos = x_pos;
        this.y_pos = y_pos;
        this.width = width;
        this.height = height;
        
        this.image = new Image();
        this.image.src = src;
        this.image.onload = () => {
            this.render(this.x_pos, this.y_pos);
        };
    }

    render(x_pos, y_pos) {
        this.x_pos = x_pos;
        this.y_pos = y_pos;
        ctx.drawImage(this.image, this.x_pos, this.y_pos, this.width, this.height);
    }

    change_src(src) {
        this.src = src;
        this.image.src = this.src;
    }
}

class Field {
    constructor(x_num, y_num, field_size) {
        this.x_num = x_num;
        this.y_num = y_num;
        this.field_size = field_size;
        this.x_pos = this.x_num * this.field_size;
        this.y_pos = this.y_num * this.field_size;
        this.sprite = new Sprite("", this.x_pos, this.y_pos, this.field_size, this.field_size);
    }

    change_field_type(type) {
        let src = GraphicMapper.get_src(type);
        this.sprite.change_src(src);
    }
}

class Map {
    constructor(field_canavas) {
        this.canvas = field_canavas; // better canvas and ctx flow
        this.fields = [];

        for (let i = 0; i < c.width / field_size; i += 1) {
            let col = [];
            for (let j = 0; j < c.height / field_size; j += 1) {
                let obj = new Field(i, j, field_size);
                obj.change_field_type(GraphicMapper.get_src('ground'));
                col.push(obj);
            }
            this.fields.push(col);
        }
    }
}

Map = new Map(c);
Map.fields[0][2].change_field_type('king');
Map.fields[8][7].change_field_type('king');
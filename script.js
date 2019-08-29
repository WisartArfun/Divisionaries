var c = document.getElementById('game-canvas');
c.height = 500;
c.width = 500;

field_size = 50;

var ctx = c.getContext('2d');

// var map;

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

function sprite(src, x_pos, y_pos, width, height) {
    let that = {};

    that.image = new Image();
    that.image.src = src;
    that.x_pos = x_pos;
    that.y_pos = y_pos;
    that.width = width;
    that.height = height;

    that.render = (x_pos, y_pos) => {
        that.x_pos = x_pos;
        that.y_pos = y_pos;
        ctx.drawImage(that.image, that.x_pos, that.y_pos, that.width, that.height);
    }

    that.change_src = (src) => {
        that.src = src;
        that.image.src = that.src;
    };

    that.image.onload = () => {
        that.render(that.x_pos, that.y_pos);
    };

    return that;
}

function field(x_num, y_num, field_size) {
    var that = {};

    that.x_num = x_num;
    that.y_num = y_num;
    that.field_size = field_size;
    that.x_pos = that.x_num * that.field_size;
    that.y_pos = that.y_num * that.field_size;

    that.update_data = function() {
        that.x_pos = that.x_num * that.field_size;
        that.y_pos = that.y_num * that.field_size;
    }

    that.change_field_type = function(type) {
        // src = map.get_type
        // src = map_ttt.get_type(type); // PROBLEM HERE
        src = GraphicMapper.get_src(type);
        that.sprite.change_src(src);
        // that.sprite.change_src(field_types[type]);
    }

    that.sprite = sprite("", that.x_pos, that.y_pos, that.field_size, that.field_size);

    return that;
}

function map(field_canvas) { // (field_canvas) {
    var that = {};

    that.canvas = field_canvas; // change ctx and canvas path
    that.fields = [];

    that.get_type = function(type) {
        return that.field_types[type];
    };

    for (i = 0; i < c.width / field_size; i += 1) {
        var col = [];
        for (j = 0; j < c.height / field_size; j += 1) {
            // obj = sprite("graphics/empty.jpg", i, j, image_size, image_size);
            obj = field(i, j, field_size);
            obj.change_field_type(GraphicMapper.get_src('ground'));
            // obj.change_field_type()
            col.push(obj);
        }
        that.fields.push(col);
    }
    // that.canvas = field_canvas;

    return that;
}

map = map();

// var map = [];
// for (i = 0; i < c.width/field_size; i+=1) {
//     var col = [];
//     for (j = 0; j < c.height/field_size; j+=1) {
//         // obj = sprite("graphics/empty.jpg", i, j, image_size, image_size);
//         obj = field(i, j, field_size);
//         col.push(obj);
//     }
//     map.push(col);
// }

// map[0][3].sprite.change_src("graphics/crown.jpg");
// map[0][3].change_field_type('king');

// map[0][3].change_src("graphics/crown.jpg");
// map[8][9].change_src("graphics/crown.jpg");
map.fields[0][2].change_field_type('king');

// while (true) {

// }
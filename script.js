var c = document.getElementById('game-canvas');
c.height = 500;
c.width = 500;

image_size = 50;

var ctx = c.getContext('2d');


function sprite(src, x_pos, y_pos, width, height) { //(options) {
    var that = {};

    that.image = new Image();
    that.image.src = src;
    that.x_pos = x_pos;
    that.y_pos = y_pos;
    that.width = width;
    that.height = height;

    that.render = function(x_pos, y_pos) {
        that.x_pos = x_pos;
        that.y_pos = y_pos;
        ctx.drawImage(that.image, that.x_pos, that.y_pos, that.width, that.height);
    }

    that.image.onload = function() {
        that.render(that.x_pos, that.y_pos);
    }

    return that;
}

var map = [];
for (i = 0; i < c.width; i += image_size) {
    var col = [];
    for (j = 0; j < c.height; j += image_size) {
        obj = sprite("graphics/empty.jpg", i, j, image_size, image_size);
        col.push(obj);
    }
    map.push(col);
}

map[0][3].image.src = "graphics/crown.jpg";
map[8][9].image.src = "graphics/crown.jpg";

// while (true) {

// }
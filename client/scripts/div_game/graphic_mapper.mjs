class GraphicMapper {
    static get_src(type, mapping = 'default') {
        let field_types = { // bad to make a new one every call???
            default: {
                ground: "../graphics/empty.jpg",
                king: "../graphics/crown.jpg",
                fog: "../graphics/fog.jpg",
            },
        };

        return field_types[mapping][type]; // [WARNING] what to return when nothing? default value?
    }

    static get_color(color, mapping = 'default') {
        let colors = {
            default: {
                empty: 'rgba(0, 0, 0, 0)',
                red: 'rgba(255, 0, 0, 0.5)',
                green: 'rgba(0, 255, 0, 0.5)',
                blue: 'rgba(0, 0, 255, 0.5)',
            }
        }

        let res = colors[mapping][color];
        if (res == undefined) {
            return colors[mapping]['empty'];
        }
        return res;
    }
}

export { GraphicMapper };
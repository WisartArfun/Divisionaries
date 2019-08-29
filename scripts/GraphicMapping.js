class GraphicMapper {
    static get_src(type, mapping = 'default') {
        let field_types = {
            default: {
                ground: "graphics/empty.jpg",
                king: "graphics/crown.jpg",
                fog: "graphics/fog.jpg",
            },
        };

        return field_types[mapping][type];
    }
}
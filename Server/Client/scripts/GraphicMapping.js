class GraphicMapper {
    static get_src(type, mapping = 'default') {
        let field_types = {
            default: {
                ground: "Client/graphics/empty.jpg",
                king: "Client/graphics/crown.jpg",
                fog: "Client/graphics/fog.jpg",
            },
        };

        return field_types[mapping][type]; // [WARNING] what to return when nothing? default value?
    }
}

const _GraphicMapper = GraphicMapper;
export { _GraphicMapper as GraphicMapper };
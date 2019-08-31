struct GameHttpServer {
    fn constructor

    spawn(move || -> std::io::Result<()> {
        HttpServer::new(|| {
            App::new() // only one handler for scripts, html, graphics?
                // INDEX
                .route("/", web::get().to(get_index))
                // JS
                .route("/Client/scripts/ProtocolInterpreter.js", web::get().to(get_protocol_interpreter))
                .route("/Client/scripts/State.js", web::get().to(get_state))
                .route("/Client/scripts/Renderer.js", web::get().to(get_renderer))
                .route("/Client/scripts/GraphicMapping.js", web::get().to(get_graphic_mapping))
                .route("/Client/scripts/Connection.js", web::get().to(get_connection))
                // GRAPHICS
                .route("/Client/graphics/crown.jpg", web::get().to(get_crown))
                .route("/Client/graphics/fog.jpg", web::get().to(get_fog))
                .route("/Client/graphics/king.jpg", web::get().to(get_empty))
                // SOCKET
                // .route("/ws", web::get().to(ws_index))
        })
        .bind("127.0.0.1:8000")
        .unwrap()
        .run()
        .unwrap();

        Ok(())
    });
}
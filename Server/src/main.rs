use std::fs::File;
use std::io::Read;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};

fn index() -> impl Responder {
    // match File::open(format!(".{}",res)) {
    match File::open("Client/index.html") {
        Ok(mut file) => {
            let mut buf = String::new();
            file.read_to_string(&mut buf).unwrap();
            
            HttpResponse::Ok().body(buf)
        },
        Err(_) => {
            HttpResponse::Ok().body("Hello world!")
        }
        // HttpResponse::Ok().body("Hello world again!")
    }

    // HttpResponse::Ok().body("hello")
}

fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8000")
    .unwrap()
    .run()
    .unwrap();

    Ok(())
}












// // extern crate ws;
// // extern crate rand;
// // extern crate mime_guess;

// // use std::thread;
// // use rand::Rng;
// // use std::sync::mpsc;
// use std::fs::File;
// use std::io::Read;
// // use std::fmt;
// // use std::fmt::Debug;
// use ws::{Handler, Sender, Message, Request, Response};
// // use std::env::args;

// #[derive(Clone)]
// struct Server {
//     out: Sender,
//     name: String,
//     number: usize,
//     local_addr: String,
// }

// impl Handler for Server{

//     fn on_request(&mut self, req: &Request) -> ws::Result<Response> {
        
//         println!("{})", req);
//         match req.resource() {
//             "/" => {

//                 let mut buf = String::new();

//                 File::open("Client/index.html").unwrap().read_to_string(&mut buf).unwrap();
                
//                 buf = buf.replace("<IP>", self.local_addr.as_str());
                
//                 let mut response = Response::new(200, "Ok", buf.as_bytes().into());
//                 response.header_mut("Content-Type = application/javascript");

//                 Ok(response)
//             },
//             "/ws" => Response::from_request(req),
//             res => {

                // match File::open(format!(".{}",res)) {
                //     Ok(mut file) => {
                //         let mut buf = String::new();

                //         file.read_to_string(&mut buf).unwrap();
                        
                //         Ok(Response::new(200, "Ok", buf.into()))
                //     },
                //     Err(_) => {
                //         Ok(Response::new(404,"Not Found", "404 - Not Found".into()))
                //     }
                // }
//             }
//         }
//     }

//     fn on_message(&mut self, msg: Message) -> ws::Result<()>{

//         println!("received message: {}", msg);
//         // self.out.send(format!("new_shift {}", newshift.clone())).expect("server was unable to send new_shift");

//         self.out.broadcast(format!("[{}] {}<br>", self.name, msg))?;

//         let msgs = msg.as_text().unwrap().split(" ").collect::<Vec<&str>>();
//         match msgs.iter().nth(0) {
//             Some(&cmd) => match cmd {
//                 _ => {
//                     Ok(())
//                 }
//             }
//             ,
//             None => {
//                 Ok(())
//             }
//         }
//     }
// }

// fn main() {
//     let ip = "0.0.0.0:8000";
//     ws::listen(&ip,|out| Server {
//         out,
//         name: "".into(),
//         number: rand::random(),
//         local_addr: ip.clone().to_string(),
//     }).unwrap();
// }
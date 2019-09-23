use crate::connection::trait_connection::Connection;

pub trait HandleNewConnection<T: Connection> {
    fn handle_new_connection(&mut self, connection: T);
}
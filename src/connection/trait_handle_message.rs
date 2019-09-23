pub trait HandleMessage<T> {
    fn handle_message(&mut self, message: T);
}
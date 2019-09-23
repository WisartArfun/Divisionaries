pub trait Connection {
    fn send(&self, message: Vec<u8>); // QUES: better to use &[u8] ?

    fn try_recv(&self) -> Option<Vec<u8>>;
}
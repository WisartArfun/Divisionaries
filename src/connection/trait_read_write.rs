// pub trait ReadWrite<I: Into<Vec<u8>>, O: Into<Vec<u8>>> { // PROB: how to do this???
// pub trait ReadWrite<I> {
pub trait ReadWrite {
    fn read(&mut self) -> Vec<u8>; // PROB: wont work with -> O

    // fn write(&mut self, content: I);
    fn write<I: Into<Vec<u8>>>(&mut self, content: I);
}
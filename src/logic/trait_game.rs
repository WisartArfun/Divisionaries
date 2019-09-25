pub trait Game { // QUES: more stuff???
    fn start_server(&mut self); // QUES: does this belong here??? // QUES: return handle???

    fn start_game(&mut self); // QUES: return handle???

    fn update(&mut self);
}
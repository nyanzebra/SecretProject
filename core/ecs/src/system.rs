pub trait System {
    type Input;

    fn run(&mut self, input: Self::Input);
}

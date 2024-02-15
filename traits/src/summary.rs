pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read from {}...)", self.summarize_author() )
    }
}

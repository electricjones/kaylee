use kaylee::repl::Repl;

// @todo: Test REPL
// @todo: print() instruction with values
// @todo: Any iteration of program consumes it because it increments the counter. Add way to set counter specifically
// @todo: cache decoded instructions to save on performance when jumping

fn main() {
    let mut repl = Repl::new();
    repl.run();
}

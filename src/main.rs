use kaylee::repl::Repl;

// @todo: Test REPL
// @todo: print() instruction with values
// @todo: cache decoded instructions to save on performance when jumping

fn main() {
    let mut repl = Repl::new();
    repl.run();
}

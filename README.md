# Kaylee
This is a weekend attempt at a rudimentary byte-code virtual machine, 
simple general purpose programming language, and associated tooling.
The purpose of this project is to learn more about virtual machines, programming languages, and compilers.

Named in honor of [Kaylee Frye](https://en.wikipedia.org/wiki/List_of_Firefly_(TV_series)_characters#Kaylee_Frye) from Firefly. Just 'cause :smile:

**Please don't use for anything you care about. I guarantee this has bugs.**

## Goals and Features:
- Memory types (pointer, slice, byte)
- Primitive types (bool, int, char, etc.)
- Complex types (arrays, string, vectors)
- Cast between types when appropriate
- Statically typed
- Basic Math Operators (+ - * / ++ --)
- Basic Memory Operators (<< >> | &)
- Basic Boolean Operators (&& ||)
- Variables
- If statements control flow
- A simple `loop` with a `continue` and `break` keyword. Work out the iterations yourself.
- A simple `for item in iterable` loop with mutable access to each thing
- No key-value collections
- Define functions with parameters
- Include a `vm` object to interact with the virtual machine and console
- Very simple error handling and control flow with an `error` type
- Garbage Collected (by reference count, simple)
- Heap and Stack Allocation

## Non Goals (for this toy project):

- Exceptions or useful error handling.
- Classes / Structs / Aliases / Other user defined types
- More control-flow structures
- Optimizations (beyond basic ones)
- A true standard library. Only the `vm` and methods bolted onto types. All written in Rust.
- To be actually useful for anything except learning

## Learning / Architecture Goals

- Move from intermediate to advanced competency in Rust
- No "specific" dependencies like parsers, compilers, or assembly. Only use dependencies that are about generic data
  structures and helpers.
- To experiment with all the above. NOT to make a MIPS clone or any other clone.

See `docs/` for the ever-evolving language specs and such.

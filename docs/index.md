There is no organization to this document yet. I'm just jotting things down as I play with them.

# Architecture Notes

- I went with the unconventional idea of variable instruction length
- Instructions are 1-4 `u8` words
- Register Identifiers are `usize`
- Register Values are `i32`

# Unconventional Choices

- I am not following RISC or CISC religiously. I am playing with a little of both.
- The Instructions are all structs that implement Traits instead of Enums. I tried several combinations and liked the
  ergonomics of this approach. I wanted all the definitions to be separate and enforced. But it made the instruction
  handling more complex than I would tolerate in a production machine, I think.
- I went with full names for the AssemblyLanguage: `JumpForward` instead of `JUMPF`. We all have IDEs and the former is
  easier to read.
- I am experimenting with allowing variation in the instruction operands instead of multiple instructions. See below.

## Virtual Machine

TODO: Memory Allocation

## Byte Code and Assembly

TODO: Opcode table

I am experimenting with a different operand structure in the Assembly Language. It isn't implemented yet.
`$` = Register
`#` = Constant
`@` = Program Point
`&` = Memory Address
`> <` = Memory Boundary

So, instead of an instruction requiring a register OR memory address OR constant value, the same instruction can have a
mixture.

```
COPY $D #300 // Loads a literal 300 into the destination register
COPY $D $A   // Copies the value of register $A into the destination register
COPY $D &100 // Copies the first four bytes from the memory at address 100 (100-103) into the destination register
COPY $D &100..3 // Copies the first THREE bytes from the memory at address 100 (100-102) into the destination register
COPY $D &$A     // Copies 4 bytes from memory starting at the value in A
COPY $D &$A..2  // Copies 2 bytes from memory starting at the value in A
```

## High Level Language

Goals and Features:

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

Nothing else.

### Types
There are no user-defined types. Only primitive types.
However, there are a few types that act as primitive, but are built on more primitive types (like strings)

The only standard library are functions attached to each type. Users cannot make types.

| Category   | Type            | Keyword                 | Notes                                                            |
|------------|-----------------|-------------------------|------------------------------------------------------------------|
| Memory     | None            | `null`                  | uninitialized, but allocated memory                              |
| Memory     | Pointer         | `pointer`               | memory space address                                             |
| Memory     | Slice           | `slice[type; length]`   | A slice of memory for a certain length                           |
| Scalar     | Byte            | `byte`                  | Single byte (really just a boolean)                              |
| Scalar     | Integer         | `integer`               | always an i32 integer                                            |
| Scalar     | Float           | `float`                 | always a f32 float                                               |
| Scalar     | Character       | `char`                  | Single unicode character                                         |
| Scalar     | Boolean         | `bool`                  | True or false                                                    |
| Complex    | Fixed Array     | `array[type; length]`   | Single type and fixed length. Stack Allocated                    |
| Complex    | Static Vector   | `vector[type]`          | Grow and shrink an array like structure                          |
| Complex    | String          | `string`                | A Character Vector with some utils                               |
| Error      | error           | `error`                 | Any type can be an `error` which will panic if used as a value   |
| ---------- | --------------- | ----------------------- | ---------------------------------------------------------------- |

All types are Rust Structs that implement a `DataValue` trait.
This trait handles the data types. It can:
- Access Stack and Heap Memory
- Keep a pointer (and optionally a length)
- Know the length of its own type
- has a `set<KayleeInteger>(value: T)` method that is given a Kaylee Value and saves it to the stack/heap
- has a `get<KayleeInteger>()` function that fetches the data from memory and coerces it into a Kaylee Type.
- has a `free()` method that frees it from the heap/stack
- has a `claim()` method that claims the space in the Heap/Stack.
- has a `cast<KayleeType>()` method that can cast to some types, but panics with others.

Then, each type also implements various types for the methods associated with it.
This includes other traits like `Iterable`, `Growable`, `Shrinkable`, etc

```
let name: string = "Michael";
name.reverse();
name.trim();
```

### Standard Library and `vm` object
There is no standard library that is truly accessible. There is a rust-written set of methods attached to types. That's it.

To interact with the outside world, the always-accessible `vm` object can be used to write to the console, get environment data, and various other things.

None of these are written in Kaylee. They are all directly written in the Virtual Machine in Rust

## Error Handling
Errors are always explicit and basically just panic or don't.

Any value (literally every type) can either be its static type OR a special `error` value.
This and panic are the only two options. 
Which means all `vm` functions that execute against the machine return something, even if just `null|error`.
If that value is captured, then the machine doesn't panic until that value is used.
If that value is NOT captured, it panics in place.
Any `error` object attempting to be used results in a panic. But you can handle errors in a few ways.

### 1. Let it panic (Simplest)
```
// We don't capture the output, so this will panic in place.
vm.read_file("/does/not/exist");

// Here we capture the output, so it panics when we try to use the value
let contents: string = vm.read_file("/does/not/exist"); // Does not panic here

// Does some other stuff

vm.print(contents); // Panics here
```

### 2. Explicitly check for an error (Recommended)
```
let contents: string = vm.read_file("/does/not/exist");
if (contents is error) {
    // Do something
    error.message(); error.code(); etc
}
```

### 3. Tell the Error Object What to do
Force a panic in place
```
let contents: string = vm.read_file("/does/not/exist"){panic};
```

Custom Panic Message (will panic when used)
```
let contents: string = vm.read_file("/does/not/exist"){message: "That's not the right file"};
```

Default Value (must be of correct type or will panic in place). Variables or functions allowed.
```
let contents: string = vm.read_file("/does/not/exist"){default: "Some default contents"};
let contents: string = vm.read_file("/does/not/exist"){default: some_default_string};
let contents: string = vm.read_file("/does/not/exist"){default: some_function()};
```

You can continue to chain the values methods
```
let contents: string = vm.read_file("/does/not/exist") {
    message: "That's not the right file"
}.trim();
  ^ Will panic here if error, otherwise just works
```

## Examples
Hello World
```
vm.print("Hello World");
```

Guessing Game
```
vm.println("Welcome!");

let answer: integer = vm.random_number(1, 10);

loop {
    vm.println("Guess a number between 1 and 10");
    vm.print("> "); // For a nice cursor
    let input: string = vm.get_user_input().clean();
    
    // We have a couple ways to cast the string to an integer.
    // The first is to check for an error value
    // This is what we'll actually do
    let guess: integer = input.into();
    if (guess is error) {
        vm.println("Sorry, that answer didn't make sense. Try again");
        continue;      
    }
    
    // Or, we can just decide to panic with a custom message.
    // let guess: integer = input.into(){panic: "That was not a number"};
    
    // We could also have a default value
    // let guess: integer = input.into(){default: 12};
    
    // We can also call a function if we fail
    // let guess: integer = input.into(){default: some_fail_function(input)};
    
    // Or, last, we can just let the system panic on its own
    // guess++; // This would treat an `error` as an `integer` and simply panic.

    // In any case, we eventually get to a `guess` that is a valid number
    // Let's make sure it's between 1 and 10
    if (!guess > 0 || !guess < 11) {
        vm.println("Guess must be between 1 and 10");
        continue;
    }

    // So, we finally have a valid guess. Let's give some feedback.
    if (guess == answer) {
        vm.println("Congratulations! You got it right!");
        vm.exit();
    }
    
    // This could be done with two ifs and continues, but I wanted to show the `else`
    if (guess > answer) {
        vm.println("Your answer is too high");
    } else {
        vm.println("Your answer is too low");
    }
```

## Example that uses all language features and types (as a test of sorts)
```
// ToDo
```

## Compiler
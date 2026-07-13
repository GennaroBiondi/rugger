# rugger.rs

*Simple* and *Flexible* logging crate.

## Installation
use `cargo` to easily add it to your crate's dependencies!

```bash
cargo add rugger
```

## Usage
use the `log_info!()`, `log_warning!()` and `log_error!()` to conveniently Log messages.

```rust
fn main() {
    log_info!("Initiating program!");
    let a = 1;
    log_warning!("Variable 'a' = {a}"); // Easily format variables!

    log_error!("This is an error!");
}
```

if using the `log_warning!()` or `log_error!()` macros, the output will be printed to `stderr`

there are also the `tee_*!()` macros that work exactly like the `log_*!()` ones but also write to a file
(hence the name, from the UNIX utility)

### **Note**:
The macros use `LogMessage` struct internally.
You can use `LogMessage::new()` directly if needed, but the macros are simpler.

## Flexibility
as the crate provides some basic log levels (Info, Warning, Error), you can make your own easily!

firstly, create a new enum:

```rust
enum MyLogLevels {
    Foo,
    Bar,
}
```

then you can implement the `LogLevel` trait.

```rust
impl Display for MyLogLevels {
    ...
}

impl LogLevel for MyLogLevels {}
```

### **Note**:
Your enum also needs to implement `Display` to be able to implement the `LogLevel` trait.
The `LogLevel` has a method called `is_error()` which dictates if the enum should be printed to
`stderr` when printing.

# rugger.rs

*Simple* and *Flexible* logging crate.

## Usage
use the `log_info!()`, `log_warning!()` and `log_error!()` to conveniently Log messages.

```rust
fn main() {
    log_info!("Initiating program!");
    let a = 1;
    log_warn!("Variable 'a' = {a}"); // Easily format variables!

    log_error!("This is an error!");
}
```

if using the `log_warning!()` or `log_error!()` macros, the output will be printed to `stderr`

### **Note**:
the macros use the `LogMessage` struct, which has a `message` and `log_level` field,
the usage of the `LogMessage` struct is **heavily discouraged**, the crate is supposed to be **convenient**,
so always use the provided macros.

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
impl LogLevel for MyLogLevels {}
```

### **Note**:
Your enum also needs to implement `Display` to be able to implement the `LogLevel` trait.
The `LogLevel` has a method called `is_error()` which dictates if the enum should be printed to
`stderr` when printing.

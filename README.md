# Rust Macros for the Strategy Pattern

This repository provides Rust macros to facilitate the use of the strategy pattern. The strategy pattern allows you to select the algorithm to execute at runtime. These macros simplify the definition and implementation of strategies.

## Usage

### Defining Strategies

To define a new strategy type, use the `#[strategy_pattern_type]` macro. This macro takes a struct as an argument and a parameter called `search`. The `search` parameter can have the values `"Exact"`, `"IgnoreCase"`, or `"RegExp"`. This parameter defines the type of search that will be performed to find the function to execute. The struct will have as a parameter the type of the functions it will manage.

```rust
#[strategy_pattern_type(search = "IgnoreCase")]
pub struct GreetingStrategy(fn(String) -> String);
```

### Implementing Functions

To implement a function that can be selected using the strategy pattern, use the `#[strategy_pattern_fn]` macro. This macro takes as arguments the strategy to which the function belongs, and the key that will be used to select the function.

```rust
#[strategy_pattern_fn(strategy = GreetingStrategy, key = "formal")]
pub fn formal_greeting(name: String) -> String {
    format!("Hello, {}!", name)
}

#[strategy_pattern_fn(strategy = GreetingStrategy, key = "informal")]
pub fn informal_greeting(name: String) -> String {
    format!("Hey {}!", name)
}
```

### Executing Strategies

To execute a strategy, use the exec function of the struct that was defined with the `#[strategy_pattern_type]` macro. This function takes as arguments the key of the function to execute, and the arguments of the function.

```rust
fn main() {
    let greeting = GreetingStrategy::exec("formal", "John".to_string()).unwrap();

    assert_eq!(greeting, "Hello, John!");

    let greeting = GreetingStrategy::exec("informal", "John".to_string()).unwrap();

    assert_eq!(greeting, "Hey John!");
}
```

## Complete Example

```rust
#[strategy_pattern_type(search = "IgnoreCase")]
pub struct GreetingStrategy(fn(String) -> String);

#[strategy_pattern_fn(strategy = GreetingStrategy, key = "formal")]
pub fn formal_greeting(name: String) -> String {
    format!("Hello, {}!", name)
}

#[strategy_pattern_fn(strategy = GreetingStrategy, key = "informal")]
pub fn informal_greeting(name: String) -> String {
    format!("Hey {}!", name)
}

fn main() {
    let greeting = GreetingStrategy::exec("formal", "John".to_string()).unwrap();

    assert_eq!(greeting, "Hello, John!");

    let greeting = GreetingStrategy::exec("informal", "John".to_string()).unwrap();

    assert_eq!(greeting, "Hey John!");
}
```

## Installation

To use these macros in your project, add the following dependency to your `Cargo.toml` file:

```rust
[dependencies]
strategy-pattern-rs = "0.1.0"
inventory = "0.3"
#regex = "1.11.1" # Optional. Required if you are using the `search = "RegExp"` option.
```

## Contributing

Contributions are welcome! If you find any errors or have any suggestions, please open an issue or a pull request.

## License

This project is licensed under the MIT license.
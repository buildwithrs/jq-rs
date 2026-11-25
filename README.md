#jq-rs

jq written by Rust

## Demo

```sh
jq-rs on  main is 📦 v0.1.0 via 🦀 v1.99.0 
❯ cargo run -- ".name" example.json 
   Compiling jq-rs v0.1.0 (/Users/alexz/Documents/Code/OrganicCoding/jq-rs)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.05s
     Running `/Users/alexz/.cargo/target-shared/debug/jq-rs .name example.json`
Welcome to the live session for building jq(json query) in Rust
success parsed json
fields: ["name"]
result: Ok(Str("Alice"))

jq-rs on  main is 📦 v0.1.0 via 🦀 v1.99.0 took 3s 
❯ cargo run -- ".job" example.json
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.02s
     Running `/Users/alexz/.cargo/target-shared/debug/jq-rs .job example.json`
Welcome to the live session for building jq(json query) in Rust
success parsed json
fields: ["job"]
result: Ok(Str("Software Eng"))

jq-rs on  main is 📦 v0.1.0 via 🦀 v1.99.0 
❯ cargo run -- ".nest.y.A" example.json
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.02s
     Running `/Users/alexz/.cargo/target-shared/debug/jq-rs .nest.y.A example.json`
Welcome to the live session for building jq(json query) in Rust
success parsed json
fields: ["nest", "y", "A"]
result: Ok(Number(1000.0))
```

## Array Query

```sh
jq-rs on  main is 📦 v0.1.0 via 🦀 v1.99.0 
❯ cargo run -- ".[]" example1.json
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.03s
     Running `/Users/alexz/.cargo/target-shared/debug/jq-rs '.[]' example1.json`
Welcome to the live session for building jq(json query) in Rust
success parsed json
whole array
result: Ok(Array([Str("one"), Str("two"), Str("3"), Str("6666")]))

jq-rs on  main is 📦 v0.1.0 via 🦀 v1.99.0 
❯ cargo run -- ".[0]" example1.json
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.02s
     Running `/Users/alexz/.cargo/target-shared/debug/jq-rs '.[0]' example1.json`
Welcome to the live session for building jq(json query) in Rust
success parsed json
get value at: 0
result: Ok(Str("one"))

jq-rs on  main is 📦 v0.1.0 via 🦀 v1.99.0 
❯ cargo run -- ".[5]" example1.json
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.02s
     Running `/Users/alexz/.cargo/target-shared/debug/jq-rs '.[5]' example1.json`
Welcome to the live session for building jq(json query) in Rust
success parsed json
get value at: 5
result: Err("index out of range")
```
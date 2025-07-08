##Ownership Rules in Rust

1. **Each value in Rust has a variable that's called its _owner.**
2. **There can only be one owner at a time.**
3. **When the owner goes out of scope, the value will be _dropped_ (memory is freed).**

These rules ensure memory safety without needing a garbage collector.
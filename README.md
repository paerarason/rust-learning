# rust-learning


### Ownership Rules
First, let’s take a look at the ownership rules. Keep these rules in mind as we
work through the examples that illustrate them:
- Each value in Rust has a variable that’s called its owner.
- There can be only one owner at a time.
- When the owner goes out of scope, the value will be dropped.

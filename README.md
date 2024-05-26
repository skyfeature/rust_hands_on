## rust_hands_on

#### Start here:
* Got the GitHub repo setup
* Got the Rust compiler installed
* Got VSCode setup for Rust development

#### Rust examples
* Hello World!
* Add a unit test for it.

### Rust rules
#### Ownership Rules
* Each value in Rust has an owner.
* There can only be one owner at a time.
* When the owner goes out of scope, the value will be dropped.

* Rust will never automatically create “deep” copies of your data.
* If we do want to deeply copy the heap data of the String, not just the stack data, we can use a common method called clone.
* Types such as integers, &str that have a known size at compile time are stored entirely on the stack, so copies of the actual values are quick to make.
* drop is used to free the memory.


#### Rules of References
* At any given time, you can have either one mutable reference or any number of immutable references.
* References must always be valid.

#### Strings
* Strings are a wrapper over Vec<u8>
* Internally it's bytes but also chars
* Each chars could be 1 byte or more
* So direct String indexing is not valid
* Be explicit using .bytes() and .chars() functions and then you can iterate.
* String slicing works only if char boundary is specified.

#### Closure
* Closures have state unlike functions. It captures a snapshot of environment when created.
* They are lazy evaluated: meaning it will be evaluated only if required.
* You can capture the variables by immutable borrow, mutable borrow or move.
* Use move if the closure can outlive the environment and you are capturing some variable.

#### Expect vs Unwrap
* Both works on `Result<V, E>` and `Option<T>`.
* Prefer using expect if you want the code to crash upon failure.
* expect lets you choose your own panic msg.
* Both are used to get the `value` from `Result::Ok(value)`.
* Using `unwrap_or_else(|error| {})`, you can capture the `error` from the `Result::Err(error)`.

#### ?
* Return early from the Result or Option.
* Returns when `Err(error)` or `None` is received.

#### Object Oriented Programming
* Rust has structs and methods and objects
* Rust has encapsulation and access modifiers
* By default everything is private and can be made public using pub
* Rust has limited inheritance and polymorphism
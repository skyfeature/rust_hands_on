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
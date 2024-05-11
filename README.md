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

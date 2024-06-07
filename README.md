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


#### Garbage collection
* Rust doesn't have GC
* Rust have ownership and borrow checking
* It's like C++, but more strict ownership

#### Referencces
* Borrow of some owned value
* Every reference has a lifetime attached, which is the scope where it's valid.
* Reference is a pointer that is not null, and points to memory containing a valid value of T
* Make one using &T and &mut T
* Since it's a borrow, it has a lifetime attached to it
* It can be used same as the original value. Field access, method calling, indexing works the same. But mutability rules may differ.
* Comparision operators can be used same as original values.
* Reference equality by address can be done using the raw pointer coercion.
* References are primitive type.

#### References warning
* References are implemented using pointers, but thinking them as merely C-like pointers will bring pain
* References are sematically their own thing, with very specific behaviour around ownership and automatic deferencing.
* They are more like a handle or lock on an object.
* & is not for taking the address, it is for borrowing the object.
* & creates a lifetime bound to a scope, and locks the object from being moved.
* Although there is an address behind the scene for references in the compiled code, that is only the implementation detail and not the point of references.


#### Rules of References
* At any given time, you can have either one mutable reference or any number of immutable references.
* References must always be valid.

#### Pass by value or reference
* In Rust, everything is pass by value
* For primitive type, the values are copied during passing
* For others, the values are moved
* References are primitive type
* Other primitive types: integer types, float types, bool type

#### Strings
* Strings are a wrapper over Vec<u8>
* Internally it's bytes but also chars
* Each chars could be 1 byte or more
* So direct String indexing is not valid
* Be explicit using .bytes() and .chars() functions and then you can iterate.
* String slicing works only if char boundary is specified.

#### Slices
* Equivalent to but not same as span in C++
* It provides a acess to the underlying contiguous data without copying
* Has a pointer to the begining of the data and a length
* Slices themselves could be mutable-immutable.
* Use &[..] to create slice

#### Tuples
* Tuple elements are accessed using . (dot) instead of [] (index).
* Tuple object is similar to a Struct, where elements are accessed using . (dot).
* Since tuple elements are anonymous unlike Struct, we use tuple.n to get nth element.
* [] (index) assumes that all element returned will be of same type.

#### Closure
* Closures have state unlike functions. It captures a snapshot of environment when created.
* They are lazy evaluated: meaning it will be evaluated only if required.
* If closure body has only one expression, no curly braces are required.
* Anything between || are arguments to the closure.
* Closure can capture any variables from it's scope by directly using variable name.
* By default they are borrowed immutably, but mutable borrow is also allowed.
* Use move if the closure can outlive the environment and you are capturing some variable.
* Closure implements one of these function traits: 
* * FnOnce: These closures can only be called once. It can move captured values out of its body. This is default implemented by a closure.
* * FnMut: These closures can be called more than once. They don’t move captured values out of their body, but that might mutate the captured values.
* * Fn: These closures can be called more than once without mutating their environment


#### Null values
* Rust has no null values.
* Every variable declared must be initialized before being used.
* If you want null behaviour use Option.

#### Option<T>
* Enum having two possible values: None|Some(x: T)
* Usually a return type in many functions where it may or may not return a value
* Use match or expect or unwrap to get the inner value.

#### Result<V, E>
* Enum having two possible values: Ok(val: V)|Err(error: E)
* Usually a return type in many functions where it will return either the value or error to do so
* Use match or expect or unwrap to get the inner values.

#### Expect vs Unwrap
* Both works on `Result<V, E>` and `Option<T>`.
* Prefer using expect if you want the code to crash upon failure.
* expect lets you choose your own panic msg.
* Both are used to get the `value` from `Result::Ok(value)`.
* Using `unwrap_or_else(|error| {})`, you can capture the `error` from the `Result::Err(error)`.
* The return type from the closure inside unwrap_or_else be same as the type inside Some.

#### ?
* Return early from the Result or Option.
* Returns when `Err(error)` or `None` is received.

#### Object Oriented Programming
* Rust has structs and methods and objects
* Rust has encapsulation and access modifiers
* By default everything is private and can be made public using pub
* Rust has limited inheritance and polymorphism

#### 3 types of Generics
* Type
* Trait
* Lifetime

#### Traits
* Traits could be considered somewhat similar to Interfaces.
* Trait could have functions, some of them just signature. Such functions must be given definition in the Trait implementation.
* Trait functions could be overwritten during implementation.
* Orphan rule: You can only implement a Trait on a type, if at least one of the Trait or the type should be local to the implementor.
* C++ doesn't have this rule.
* E.g: you can't implement Display trait on Vec<T>.
* Traits can be passed in function parameter. The argument could be replaces by any Struct which implements the trait.
* If multiple Traits are applied to a function argument type, then the type should be implementing all those traits.

#### Display vs Debug


#### Copy vs Clone
* 

#### PartialEq and PartialOrd

#### Lifetimes
* Main aim is to prevent Dangling references.
* Whenever you deal with references, you have to think about their lifetime.
* Most of the times lifetimes are implicit and inferred by the compiler, just like types.
* But we need to annotate the lifetime when the lifetime of a reference could have multiple values.
* Lifetime annotation is a guide for the compiler.
* The annotation doesn't itself change the lifetime of the parameter.
* Lifetime annotation by itself doesn't have much meaning, it's relationship with other annotations makes them useful.
* Annotating lifetime is not a concept in most of other languages.

#### Borrow checker
* Rust compiler has it.
* It compares scopes of reference to determine whether all borrows are valid

#### self vs &self vs Self
* Self is the type
* self is take ownership inside the method. The object is destroyed after method call
* &self/&mut self is borrow immutably/mutably. The object stays after method call

#### Iterators
* Perform some task on a sequence of items
* Iterators in Rust are lazy. They don't do anything unless consumed.
* arr.iter() to iterate over items by reference, gives immutable reference for each item.
* arr.iter_mut() to iterate over items by reference, gives mutable reference for each item.
* arr.into_iter() to get owning iterator. arr is destroyed after the iteration.

* make the iterator variable mutable if next() is going to be called.
* Type returned from iter().next() is Some(&T)
* Iterator adaptor can help perform some operation on the iterator without consuming it


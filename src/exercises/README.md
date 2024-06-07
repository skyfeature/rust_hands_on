### Some useful Rust APIs

#### Type conversion
* i8, u8, i32, u32, i64, u64, String, &str, char, bool
* convert from one type to another using `as`
* char to ascii, use `as u8``
* ascii to char, use `as char

#### Struct
* To create a class method, pass `&self`
* To create a class method, and the class object can die at the spot of call, pass `self`
* To create a class method that will also modify class variables, pass `&mut self`
* To create a class function, which is not taking self as parameter, call it using `Self::method_name`

#### Vec
* is_empty()
* push(T)
* pop() returns Option<T>
* Coalesce vector into a giant string: `arr.into_iter().collect()`

#### String
* Underneath is Vec<u8>
* .chars() to get iterator into characters in the string.
* Each char could be more than 1 byte
* .bytes() to get into each byte.
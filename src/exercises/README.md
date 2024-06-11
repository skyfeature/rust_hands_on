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
* is_empty() returns bool
* push(T) returns ()
* pop() returns Option<T>
* Coalesce vector into a giant string: `arr.into_iter().collect()`
* In general, iterator could be created using iter over one data structure, and then collect could be used to collect into a different data structure.
* create vector using vec![3, 5, 8];


#### String
* Underneath is Vec<u8>
* .chars() to get iterator into characters in the string.
* Each char could be more than 1 byte
* .bytes() to get into each byte.

#### Iterator:
* If you just need to look at the data, use iter, if you need to edit/mutate it, use iter_mut, and if you need to give it a new owner, use into_iter.
* So for x in my_vec { ... } is essentially equivalent to my_vec.into_iter().for_each(|x| ... ) - both move the elements of my_vec into the ... scope.

* enumerate() will return an iterator having a tuple of (index, element).

#### Tuple
* use .0, .1, .n to get nth element of tuple.
* tuple is like struct with anonymous fields. So .num is used to extract its fields.

#### Sort
* sort_by_key(|key| key.1) can be used to sort by a particular element of tuple/struct etc.

#### match
* multiple cases can be matched to the same using or (|), like: a|b|c => Some(5),
* if can be put before => but after the case value: a if condition(1) => Some(5),
* use _ to match remaining.
* match arms must have same type
* type of a return statement is ()
* match guard
* The match arm with guard won't be taken into account, when (the compiler) checking if all patterns have been covered.
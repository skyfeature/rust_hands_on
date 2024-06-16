### Some useful Rust APIs

#### Type conversion
* i8, u8, i32, u32, i64, u64, String, &str, char, bool
* convert from one type to another using `as`
* char to ascii, use `as u8`
* ascii to char, use `as char`

#### Numeric types
* `std::cmp::max(a, b)`
* `std::cmp::min(a, b)`


#### Struct
* To create a class method, pass `&self`
* To create a class method, and the class object can die at the spot of call, pass `self`
* To create a class method that will also modify class variables, pass `&mut self`
* To create a class function, which is not taking self as parameter, call it using `Self::method_name`

#### Vec
* `is_empty()` returns `bool`
* `push(T)` returns `()`
* `pop()` returns `Option<T>`
* Coalesce vector into a giant string: `arr.into_iter().collect()`
* In general, iterator could be created using iter over one data structure, and then collect could be used to collect into a different data structure.
* create vector using `vec![3, 5, 8]`
* `resize_with(size, closure to create default elements)` create a vector with size elements and all having value set by closure
* `with_capacity(size)` creates a vector with capacity size.


#### String
* Underneath is `Vec<u8>`
* .chars() to get iterator into characters in the string.
* Each char could be more than 1 byte
* .bytes() to get into each byte.

#### Iterator:
* If you just need to look at the data, use `iter`, if you need to edit/mutate it, use `iter_mut`, and if you need to give it a new owner, use `into_iter`.
* So `for x in my_vec { ... }` is essentially equivalent to `my_vec.into_iter().for_each(|x| ... )` - both move the elements of my_vec into the ... scope.

* enumerate() will return an iterator having a tuple of (index, element).
* collect() will let you collect the iterator into a container.
* iter, iter_mut, into_iter will let you iterate through a container.
* iter().rev() will let you reverse iterate through a container.

#### Tuple
* use .0, .1, .n to get nth element of tuple.
* tuple is like struct with anonymous fields. So .num is used to extract its fields.
* Struct tuples are like small structs with little work
* `struct TupleName(i32, String, bool);`
* It can be pattern matched. `let TupleName(a, b, c) = tuple_var;`

#### Sort
* `sort_by_key(|key| key.1)` can be used to sort by a particular element of tuple/struct etc.
* `sort_by(|a, b| a.cmp(b))` sort in increasing order
* `sort_by(|a, b| b.cmp(a))` sort in decreasing order

#### match
* multiple cases can be matched to the same using or (|), like: `a|b|c => Some(5)`,
* if can be put before `=>` but after the case value: `a if condition(1) => Some(5)`,
* use `_` to match remaining.
* match arms must have same type
* type of a return statement is `()`

* match guard
* The match arm with guard won't be taken into account, when (the compiler) checking if all patterns have been covered.

* You can match multiple things at a time by putting them inside a tuple: `match (list1, list2) { /**/ }`


#### Box<T>
* You can treat boxed variable as a variable without Box for all calculation
* Box of a struct instance: `Box::new(MyStruct { var1: 5, var2: true })`


#### Recursion
* Rust has recursive functions.


#### Result
* ok() converts a Result<T, E> into Option<T>

#### Option
* is_some, is_none
* Use ? to return early with None
* Any calculation with None is None
* as_ref converts a reference to Option into an Option of reference.

#### BTreeSet
* Like ordered set from C++
* new()
* insert()
* len()
* begin() to get iterator to smallest element.
* pop_first() to get the smallest element.

#### BTreeMap
* Like ordered map from C++
* uses most of the functions from Map
* Can be iterated using for loop like `for (key, val) in map.iter()`
* Insert a key wih a value only if it doesn't already exist, and get reference to the present value in map.
* `let val = my_map.entry("key").or_insert(my_default_val);`
* new()
* enter()
* or_insert()


#### Reference
* &[i32] is a like a reference to a struct of pointer pointing to begining address and number of elements stored contiguously in memory.
* &Vec is a reference to vector
* [i32, 8] is an array of 8 elements of i32
* prefer passing &[i32] as function parameter. It can take both vector, array, subslices of vector, etc.

#### turbofish (::<>)
* Used when you need to pass a type parameter to a function
* turbofish is used to remove the ambiguity during parsing. Whether < is less than operator or start of type enclosure.


#### Find type of a variable using compiler
* let _: () = my_var;
* Compiler will complain with the type mismatch telling you the type of my_var.

#### Read arguments from command line
* Get a handle to the global Stdin using `io::stdin()`
* lock() to acquire lock over the Stdin handle
* lines() to get iterator over the buffer
* map() to apply some closure to all of the lines
* next() to get ietrator to next line
* trim() remove whitespaces from front and back
* parse() to convert String to a different type.
* collect() to collect into a collection.

#### Read arguments from file
* read_to_string(filename) read from file into a big String
* lines() to get iterator over the lines in the String
* use next() to get iterator to next line.

#### Write to a file
* `File::create("MaxResult.txt")` will create a file and get you a file handle
* `writeln!(&mut fptr, "{}", result).ok();` writes a formatter line into the file.
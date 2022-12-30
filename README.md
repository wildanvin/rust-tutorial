## Rust tutorial

This is the code that I wrote/copy-paste from Derek Banas's [tutorial](https://www.youtube.com/watch?v=ygL_xcavzQ4), it was pretty good and learned a lot. Here are some notes that I took from the course:

My main takeaway: **First, model program state as structs then write functions to move between these valid states. That is the pattern in Rust.**

- If python has pip and JS has npm, rust has cargo; it is the package manager of rust.

- Working with strings:

```rust
// Create a string slice
let string_slice: &str = "Hello, world!";

// Create a String
let mut string = String::new();

// Append a string slice to a String
string.push_str("Hello, ");
string.push_str("world!");

// Convert a String to a string slice
let string_slice: &str = &string;

// Convert a string slice to a String
let string = String::from(string_slice);
```

- Rust infers the types for you, the vscode extension `rust-analizer` shows you the type of your variable, it is pretty nice.

- You define strings with double quotes and characters with single quotes

- You can do what is called shadowing in Rust, which is have variables with the same name but different data types.

- Only the last statement can exclude the semicolon. For exmaple when we are returning a value from a function we just write the value without semicolon

- Tuples can contain different data types

- You have `String` that is a vector of characters and `&str` that is a reference to the string

- Vectors are like arrays, they can grow if mutable, they can only store values of the same type.

- For getting an element from a vector:

```rust
let v = vec![1, 2, 3, 4, 5];

if let Some(x) = v.get(2) {
    println!("The third element is {}", x);
}
```

- The **`get`** method returns an **`Option<&T>`**, where **`T`**is the type of the elements in the **`Vec`**. If the index is out of bounds, it returns **`None`**.

- Types are namespaces too.

- In rust everything is an expression, that means that everything returns a value

# Explanations

Here are some explanations to Rust features that have not been covered in detail in the Rust lectures but appear in this project.

- `Box<AExp>`
  
  A **box** belongs to the class of so-called "smart pointers". The box type is a struct that is parameterized by some type `T` and contains only one field, namely a reference to some object of `T` on the heap. Therefore, if you have some piece of data whose size is large or unknown at compile-time, it can be explicitly stored in the heap by putting it in a box via `Box::new(myvalue)`. Being classified as a "smart pointer", it supports the dereference operation `*`, making `Box<T>` behave like the `&T` type to the outside. 

- `[derive(PartialEq,Clone,Debug,Eq,Hash)]` 
  
  These **derive macros** in front of a struct definition trigger the automatic derivation of trait implementations for that struct. How can the compiler derive these? The idea is to use the trait implementations of the struct's *components* to derive a trait implementation of the *composite*. For example:
  - `Eq`: Two values of the composite are equal `<=>` All values of their components are equal
  - `Clone`: A value of the composite can be cloned by first cloning all of its components, and then cloning the composite object itself

  Using derive macros thus saves quite some boilerplate code.

- `struct VarName(String)`

  This introduces a struct that merely **wraps some type**, a technique also known as the "newtype idiom". The idea behind such a wrapper type is to introduce a new type (here: `VarName`) that has the same functionality as the wrapped type (here: `String`) but is recognized as distinct by the type system. This allows for the programmer to make a semantic distinction between `String`s and `VarName`s, allowing e.g. different trait implementations. See also [newtype].

- `impl Display for VarName { ... }`

  **Pretty-printing** can be implemented by the `Display` trait, the analogue to Java's `toString` method. In Rust, the `fmt` method is implemented for this purpose. The string to display is not returned as a value but nstead written to a buffer. The easiest way to do this is to use the following macro
  ```
  write!(f, "this string contains {} words", 4)
  ```
  which performs the buffer-writing using the `Formatter` object `f` (no need to know the internals here). See also [Display].

- `&self`, `&mut self`, `self`

  To enable an **object-oriented style** for structs ("structs as classes") that allow notation such as `my_exp.contains_var(x)`, one simply has to define the function `contains_var` such that it takes a struct as first argument, named `self`. There are three options for this:

  1) *Move* semantics (`self: Self` or short `self`)
  2) *Immutable Reference* semantics (`self: &Self` or short `&self`)
  3) *Mutable Reference* semantics (`self: &mut Self` or short `&mut self`)
  
  On the caller's side, when writing `my_exp.contains_var(x)`, the correct variant 1., 2. or 3. is implicitly taken; e.g. one does not have to write `(&mut my_exp).contains_var(x)` for variant 3.

- `myvec.map(|x| { x + 1 })`

  It is sometimes useful to **pass a function as an argument**. E.g., `map` applied to a vector takes a function that transforms each items of the vector, yielding a new vector (here e.g., the argument `[1, 2, 3]` would result in the new vector `[2, 3, 4]`). Here, the function is constructed "on the fly" and is also called a **closure**. The argument of the closure is declared by `|x|` and the closure body is then given by `{ x + 1 }`.

- `write!(f, "hello")?;`
  
  What does the question mark `?` before the statement mean? It is basically syntactic sugar for the following:
  1) If the `write!(f, "hello")` command returns an `Err` value, then return ("forward") that `Err` value.
  2) If the `write!(f, "hello")` command returns **no** `Err` Value, then proceed with the next command. See also [question-mark-operator]. You can also think of the `?` operator as the `>>` operator of the `Maybe` monad in Haskell.
  

[newtype]: https://doc.rust-lang.org/stable/rust-by-example/generics/new_types.html "New Type Idiom"
[Display]: https://doc.rust-lang.org/stable/rust-by-example/hello/print/print_display.html "Display"
[question-mark-operator]: https://doc.rust-lang.org/edition-guide/rust-2018/error-handling-and-panics/the-question-mark-operator-for-easier-error-handling.html
# Basics

## Hello, Rust!

Let's start with a typical "Hello, world!" program. 

```rust,editable
fn main() {
    println!("Hello, Rust!");
}
```

First we declare a main function, which is the entry point in every Rust program. 

The `println!` macro is used to print text to the console. The exclamation mark (!) indicates that `println!` is a macro, not a function.


We can compile and execute this program from the terminal using the Rust compiler. 

```console
$ echo "fn main() {\n    println!(\"Hello, Rust\"); \n}" > hello.rs
$ rustc hello.rs
$ ./hello
Hello, Rust!
```


## Variables

Variables in Rust are declared with the `let` keyword (thanks FP).

```rust,editable
fn main() {
    let answer: u64 = 42;
    println!("The answer to life the universe and everything is {}.", answer);
}
```

The variable answer has a _strong_, _static_ type `u64` (denoting an unsigned
integer with 64 bits). Rust has type inference, so the type annotation could
have been omitted.
> Το συγκεκριμένο παράδειγμα είναι λίγο περίεργο για το type inference γιατί
> χωρίς το annotation κανονικά δεν θα είχε αρκετή πληροφορία για τον τύπο της
> μεταβλητής και θα έβγαζε compile error.
> Τα number literals στη Rust έχουν ειδική μεταχείρηση και στο type inference.
> Ο τύπος τους είναι κάποιου είδους type variable (μιας και πχ το 42 μπορεί να
> είναι είτε u8, u16, κλπ) αλλά αν δεν γίνουν constraint σε έναν τύπο από
> annotations του προγράμματος τότε ο compiler διαλέγει το i32 by default. Αυτό
> γίνεται μόνο για τα number literals. Στη γενική περίπτωση αν ένα type
> variable δεν γίνει constrain κατά το type inference τότε ο compiler πετάει
> error. Πχ αυτό δεν παίζει:
> fn main() {
>     let answer = None;
>     if answer.is_none() {
>         println!("hello");
>     }
> }

By default, variables are _immutable_ (yes, thanks FP again). 

Let's look at what happens when we try to change the value of `answer`.

```rust,editable
fn main() {
    let answer = 42;
    println!("The answer to life the universe and everything is {}.", answer);
    answer = 41;
    println!("I changed my mind. The answer to life the universe and everything is {}.", answer);
}
```

The compiler complains that `cannot assign twice to immutable variable`. To get around this, we must explicitly declare `answer` as a mutable variable.

The following works fine. 

```rust,editable
fn main() {
    let mut answer = 42;
    println!("The answer to life the universe and everything is {}.", answer);
    answer = 41;
    println!("I changed my mind. The answer to life the universe and everything is {}.", answer);

}
```

## Data Types

### Scalar Types

Scalar types in Rust are base types that represent a single value. These can be
integers, floating-point numbers, Booleans, and characters.

#### Integers

Rust provides several built-in integer types with different bit widths. These
types are denoted as `uN` and `iN`, for unsigned and signed integers
respectively, where N represents the bit width and can be one of 8, 16, 32, 64,
or 128.

For example, `u16` is an 16-bit unsigned integer, holding values from 0 to
2^16-1 and `i64` is a 64-bit integer holding values from -2^63 to 2^63-1.  

The overflow behavior is different depending on compiler flags. When compiling
in debug mode, Rust will trap overflows throwing an unrecoverable error
(_panic_). Then compiling release mode, there are no runtime checks for
overflows and will wrap around. 
> Note: Το overflow είναι undefined behavior, ίδιο με το να κάνεις access out
> of bounds. Δηλαδή ο compiler υποθέτει ότι δεν συμβαίνει ποτέ, ακόμα και στο
> release build. Αν θες να κάνεις overflowing arithmetic πρέπει να
> χρησιμοποιήσεις ειδικά functions.
> https://doc.rust-lang.org/std/num/struct.Wrapping.html

Additionally, Rust has the `isize` and `usize` types that depend on the machine's
architecture. 

#### Floating Point Numbers

The `f32` type is a single-precision float, and `f64` is a double precision
float.

#### Booleans

The type `bool` with values `true` and `false`.

#### Chars

Rust has a `char` type that represents unicode characters. They are written in
single quotes. 

Snippet from the Rust book:

```rust, editable
fn main() {
    let c = 'z';
    let z : char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
}
```


### Compound Types

#### Tuples 

```rust,editable
fn main() {
  // constructing tuples
  let t : (u64, f32, char) = (42, 10.0, 'z');

  println!("My tuple is {:?}", t);

  // eliminating tuples: projection 
  println!("The first component is {}", t.0);

  // eliminating tuples: decomposition pattern
  let (x, y, z) = t;

  println!("The first component is {}", x);
}
```

Notice that we use the `{:?}` format specifier instead of `{}`. The reason is
that tuple does not implement the `Display` trait, required to print a value
with `{}`, but it does implement the `Debug` trait witch is what the `{:?}`
specifier calls.


#### Arrays

Arrays are similar to tuples, but all elements must have the same type. Its
type is fixed and statically known. They are allocated on the stack. 

```rust,editable
fn main() {
  let a: [u32; 5] = [1,2,3,4,5];

  println!("The 2nd element is {}", a[1]);
}
```

Play around to see what happens when you try to access the array out of bounds. 

When the compiler can statically determine that the array will be accessed out of bounds it will throw a compile time error. 

However, this is not a conservative static check. Eliminating all accesses that cannot be guaranteed to be in bounds would be too restrictive.  

The following program compiles fine and throws an error at runtime. You can also
see an example of Rust function (yes, arrows again).

```rust,editable
fn access(a: [u32; 5], n: usize) -> u32{ 
  return a[n];
}

fn main() {
  let a: [u32; 5]= [1,2,3,4,5];

  println!("The 2nd element is {}", access(a, 7));
}
```

Notice, that accessing an array out of bound throws is a _trapped_ error (unlike
C and C++). The program will panic at runtime, throwing an unrecoverable error.
This ensures that a program can never access invalid memory.

## Functions 

We just saw an example of a Rust function. In Rust functions can have zero or more parameters, and optionally a return type. 

Here are some examples of Rust functions. 


```rust,editable
fn fourtytwo() -> u32{ 
  return 42;
}

fn sayhi() { 
  println!("Hello!");
}

fn add(x: u32, y:u32) -> u32{ 
  x+y
}

fn main() {
  println!("One more time {}", fourtytwo());
  
  sayhi();

  println!("Add two numbers: {}", add(5,6));

}
```

We can declare function parameters as mutable. 

```rust,editable
fn add1(mut t : (u32, u32)) -> u32 { 
    t.0 += 1; 
    return t.0;
}

fn main() {
    let mut t = (1,2);
    
    println!("add1 result: {}", add1(t));
    println!("The first component is {}", t.0);

}
```

## Control Flow

We see some of Rust control flow constructs using a Fibonacci example. For more
control flow constructs refer to [the Rust
Book](https://doc.rust-lang.org/book/ch03-05-control-flow.html). 


```rust,editable

fn fib(n : u32) -> u32 { 
    if n == 0 { 0 }
    else if n == 1  { 1 }
    else { fib(n-1) + fib(n-2) }
}


fn fib_iter1(n : u32) -> u32 { 
    let mut i = n;
    let mut curr = 0;
    let mut next = 1;

    loop {
      let tmp = curr;
      curr = next;
      next = next + tmp;
      i-=1;
      if i == 0 { break curr }
    }
}

fn fib_iter2(n : u32) -> u32 { 
    let mut i = n;
    let mut curr = 0;
    let mut next = 1;

    while i != 0 {
      let tmp = curr;
      curr = next;
      next = next + tmp;
      i-=1;
    }

    return curr;
}

fn fib_iter3(n : u32) -> u32 { 
    let mut i = n;
    let mut curr = 0;
    let mut next = 1;

    for i in 0..n {
      let tmp = curr;
      curr = next;
      next = next + tmp;
    }

    return curr;
}


fn main() {

  let n = 8;

  println!("Functional: The {}th fib is: {}", n, fib(n));
  println!("Iterative 1: The {}th fib is: {}", n, fib_iter1(n));
  println!("Iterative 2: The {}th fib is: {}", n, fib_iter2(n));
  println!("Iterative 3: The {}th fib is: {}", n, fib_iter3(n));

}
```

https://doc.rust-lang.org/reference/introduction.html

# Input formate 

```rust
#!/usr/bin/env rustx

fn main() {
    println!("Hello!");
}
```


# Comments

```rust
//! a doc
//! 
pub mod outer_module {

//! - Inner line doc
//!!  - still an inner line doc
 
/*! - inner block doc */
/*!! - still an inner block doc */

// - only a comment
/// - outer line doc 
//// - only a comment 

/* - outer a comment  */
/** - outer block doc */
/*** - only a comment */

    pub mod inner_module {

    }

    pub mod nested_comments {
        /* In rust /* we can /* nest comments */ */ */

        // all three types of block comments can contain 

        pub mod dummy_item{}
    }

    pub mod degenerate_cases {
        
        pub mod dummy_item {}
    }
} 
```

``` rust
'\t'
'\n'
'\r'
' '
```

# Macros

```rust

// used as a expression
let x = vec![1,2,3];

// used as a statement. 
println!("Hello!");


// used in a pattern.
macro_rules! pat {
    ($i:ident) => (Some($i))
}

if let pat!(x) => Some(1) {
    assert_eq!(x,1)
}

// used in a type
macro_rules! Tuple {
    { $A:ty, $B:ty } => { ($A, $B) };
}


type N2 = Tuple!(i32, i32);

// used as an item.
thread_local!(static FOO: RefCell<u32> = RefCell:new(1));

// used as an associated item.
macro_rules! const_maker {
    ( $t:ty, $v:tt ) => { const CONST: $t = $v; };
}

trait T {
    const_maker ! {i32, 7}
}


// macro calls within macros.
macro_rules! example {
    () => { println("Macro call in a macro!") };
}

// outer macro `example` is expanded, then inner macro `println` is expanded.
example!();
```

--

# creates

```rust

fn main() {}

fn main() -> ! {
    std::process::exit(0);
}

fn main() -> impl std::process::Termination {
    std::process::ExitCode::SUCCESS
}


mod foo {
    pub fn bar() {
        println!("Hello, world!");
    }
}

use foo::bar as main;
```



# conditional compilation 

``` rust 

// the function is only included in the build when compiled for macos
#[cfg(target_os = "macos")]
fn macos_only() {
    //..
}

#[cfg(any(foo, bar))]
fn needs_foo_or_bar() {
    //..
}

#[cfg(all(unix, target_pointer_width = "32"))]
fn on_32bit_unix() {
    //..
}


// when foo is not defined 
#[cfg(foo(foo))]
fn needs_not_foo() {
    //..
}


// when foo is not defined 
#[cfg(painc = "unwind")]
fn when_unwinding() {
    //..
}
```


# Modules

```rust

mod math {
    type Complex = (f64, f64);

    fn sin(f: 64)  -> 64 {

    }

    fn cos(f: 64)  -> 64 {
        
    }

    fn tan(f: 64)  -> 64 {
        
    }
}

#[path = "foo.rs"]
mod c;

mod inline {
    #[path = "other.rs"]
    mod inner;
}

#[path = "thread_files"]
mod thread {
    // Load the `local_data` module from `thread_files/tls.rs` relative to
    // this source file's directory.
    #[path = "tls.rs"]
    mod local_data;
}

```

# extern create declaration


# use declaration

```rust

use std::collections::hash_map::{self, HashMap};

fn foo<T>(_: T) {}

fn bar(map1: HashMap<String, usize>, map2: hash_map::HashMap<String, usize>){}

fn main() {
    
}
```
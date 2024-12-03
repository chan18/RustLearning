# How rust uses memory

vs-code rust extension

1. rust-analyzer
2. codeLLDB


create a project with cargo
* cargo init to init a new project


Different Approaches to memory
* rust memory management vs garbage collection vs manual memory management

rust - ownership memory management.

ownership :

pros:
* full control
* efficient code
* *error free

cons: 
takes time to learn

---

Review  
* data type
* immutability
* scope
* stack vs heap

data types: 
Scalar

```rust

Integer (i8, u8, i16, u16, i32, u32, i64,
u64, i128, u128, isize, usize)
let x: i32 = 2;

// Float (f32, f64)
let x: f64 = 2.0;

// Boolean (bool)
let f: bool = false;

// Character (char)
let c: char = 'z';
```

Compound
```rust
// Tuple
let tup: (i32, f64, u8) = (100, 2.5, 4);
// Array
let a: [i32; 5] = [1, 2, 3, 4, 5];
```

Stack: 
Variables with a known, fixed size
will be allocated on the Stack.

Heap:
Variables with an unknown or a
changeable size will be stored in
the Heap.

mutability

scope

`
Ownership is a set of rules that
govern how a Rust program
manages memory. It helps
ensure the memory safety of
you program without a garbage
collector.
`

---
# managing memory with ownership

Concept of ownership in rust
* what it is ? 
* how it works? 
* types of issues that it helps prevent.

ownership

**Rules of Ownership**
- ( code will not compile if we broke the rules.)

**Copyable Data Types**
- primitives types stored on stack which implements the copy trait.

**Non-Copyable Data Types**
- other complex types which don't implements the copy trait.
- why it does this?  what errors borrow checker trying to protect against.

**Copy and Clone Traits**
- often we use borrowing and pass the value as reference, some times you might want to make a copy or clone a value from heap.

**Rules of Ownership**

Each value in Rust has an owner
There can only be one owner at a time
When the owner goes out of scope, the value
will be dropped

ownership model: The two pillars of Rust are
Speed and Safety.

**what does the borrow checker do?**
* Ensure lifetimes are correct.
* That you can't move a value while it is borrowed.
* That you can't move the same value twice.
* That you can't access a place while it is mutably
* borrowed (except through the reference).
* That you can't mutate a place while it is immutably
* borrowed.
* That all variables are initialized before they are
* used.
* Etc.


**Copyable Data Types**
* Primitive Types (Scalar and Compound)
* Known Size at Compile Time
* Stored on the Stack
* *Implements Copy Trait

    Data types that implement
    the copy trait “ignore” the
    Ownership Rules.
    The copy trait is implemented
    by the primitive data types
    stored on the stack.

**Non-Copyable Data Types**

dangling pointers

when the initial var has been freed which we borrowed value from, Than we still points to the memory address of the freed var is known as dangling pointers.

double fee

other process uses this same memory and we point to that memory. we free that memory and corrupts the other process using it. 

**Copy and Clone Traits**

Data Types That Implement the Copy Trait

scalar
```rust
//Integer (i8, u8, i16, u16, i32, u32, i64,
//u64, i128, u128, isize, usize)
let x: i32 = 2;

//Float (f32, f64)
let x: f64 = 2.0;

//Boolean (bool)
let f: bool = false;

// Character (char)
let c: char = 'z';
```

compound
```rust
// Tuple
let tup: (i32, f64, u8) = (100, 2.5, 4);
// Array
let a: [i32; 5] = [1, 2, 3, 4, 5];
```

* Rules of Ownership
* Copyable Data Types
* Non-Copyable Data Types
* Copy and Clone Traits


# borrowing values by reference

Borrowing
* which let's multiple parts of the program use same piece of data without give up ownership.

**Borrowing Values by Reference**

`
    “We call the action of creating a
    reference borrowing. As in real life,
    if a person owns something, you can
    borrow it from them. When you’re
    done, you have to give it back. You
    don’t own it.”
`

* Immutable References
* Mutable References
* Reference Restrictions
* String Slices

#### Restrictions When Using References

* References can never be NULL

* Multiple immutable references to the same
value are allowed

* But only one mutable reference to the same
value per scope

* No mixing of mutable and immutable
references ( to avoid data races )


    rust helps prevent data races from ownership and borrowing systems.This helps only one thread can access the data at any given time. making concurrent programming much more safer and predictable.

#### string slices

string vs string slice

string
* Standard Library (std::string module)
* Growable
* Mutable
* Own their data

string slice
* Reference to a portion of String
* Don’t own the data they reference
* Data type &str
* Immutable

string rules

* Use String when you need to own the string data.
* Use &str when you only need to borrow a string.
* Consider performance.

# using lifetimes to reduce ambiguity

Discussion of lifetimes
* how long a reference to a data allowed to live. 

---

`“Every reference in Rust has
a lifetime, which is the scope
for which that reference is
valid.”`
-- Rust Book – Chapter 10

`“The main aim of lifetimes is to
prevent dangling references,
which cause a program to
reference data other than the
data it’s intended to reference.”`
-- Rust Book – Chapter 10

`“A lifetime is a construct the
compiler (or more specifically, its
borrow checker) uses to ensure
all borrows are valid.”`
-- Rust Book – Chapter 10

`“We only must annotate types when
multiple types are possible. In a
similar way, we must annotate
lifetimes when the lifetimes of
references could be related in a few
different ways.”`



`“Most of the time, lifetimes are
implicit and inferred, just like most
of the time, types are inferred.”` -- Rust Book – Chapter 10

* What is a Lifetime?
( some times we have to explicitly declare life times.)

* Lifetimes in Functions
we have to often need to annotated the lifetimes of references in your functions.


* Lifetimes in Structs
we have to often need to annotated the lifetimes of references in your structs.

fact: lifetimes are all about references.

* Lifetime Elision Rules

some times compiler (borrow checker) smart enough to figure out all the lifetimes
rues of when the compiler refer this lifetimes. ( i don't have to explicitly specify them)

* Static Lifetimes

which represents the entire lifetime of the program.


*what is a lifetimes?* 


`“Lifetime annotations don’t change
how long any of the references live.
Rather, they describe the
relationships of the lifetimes of
multiple references to each other
without affecting the lifetimes.”`



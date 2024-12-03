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
let c: bool = 'z';
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

Rules of Ownership 
- ( code will not compile if we broke the rules.)

Copyable Data Types
- primitives types stored on stack which implements the copy trait.

Non-Copyable Data Types
- other complex types which don't implements the copy trait.
- why it does this?  what errors borrow checker trying to protect against.

Copy and Clone Traits
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

---

# borrowing values by reference

Borrowing
* which let's multiple parts of the program use same piece of data without give up ownership.

---
# using lifetimes to reduce ambiguity

Discussion of lifetimes
* how long a reference to a data allowed to live. 

---



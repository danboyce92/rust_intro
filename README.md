# Rust

## Install
To install Rust locally on your computer, follow the directions on this page:
<br/>

https://www.rust-lang.org/tools/install <br/>



After installation, restart your terminal then try running cargo --version. If you see a version print up then you're good to go! If you get an error then a little troubleshooting is in order.

## Project

*Cargo is used to create, compile, run and manage projects.*

Create a new project :

```
cargo new <projectname>
```

Run a project

```
cargo run
```

Add the -q flag to just run the project and avoid receiving any debugging messages

```
cargo run -q
```



### Inherent Implementations
There are two types of functions to be aware of in Inherent Implementations.

**Associated Functions**
```
impl Deck {
    fn new() -> Self {
        //code..
    }
}

fn main() {
    Deck::new();
}
```
Use when you have functionality not tied to a specific instance
<br/>

**Methods**
```
impl Deck {
    fn shuffle(&self) {
        //code..
    }
}

fn main() {
    deck.shuffle();
}
```
Use when you need to read or change fields in a specific instance



### Implicit Returns
**If you remove the ; from the final expression of a function it will automatically be returned**

This 
```
fn is_even(num: i32) -> bool {
    return num % 2 == 0;
}
```

Is the same as 
```
fn is_even(num: i32) -> bool {
    num % 2 == 0
}
```


## 12 Rules of Ownership, Borrowing & Lifetimes

### Ownership
1. Every value is 'owned' by a single variable, argument, struct, vector, etc at a time. <br/>
2. Reassigning the value to a variable, passing it to a function, putting it into a vector, etc, moves the value. The old owner can't be used to access the value anymore. <br/>
### Borrowing
3. You can create many read-only references to a value that exists at the same time. These refs can all exist at the same time. <br/>
4. You can't move a value while a ref to the value exists. <br/>
5. You can make a writeable(mutable) reference to a value only if there are no read-only references currently in use. One mutable ref to a value can exist at a time. <br/>
6. You can't mutate a value through the owner when any ref(mutable or immutable) to the value exists. <br/>
7. Some types of values are copied instead of moved(numbers, bools, chars, arrays/tuples with copyable elements). <br/>
### Lifetimes
8. When an owner goes out of scope, the value owned by it is dropped(cleaned up in memory). <br/>
9. There can't be references to a value when its owner goes out of scope. <br/>
10. References to a value can't outlive the value they refer to.
### Remember!
11. **These rules will dramatically change how you write code(compared to other languages)** <br/>
12. **When in doubt, remember that Rust wants to minimize unexpected updates to data.** <br/>


### Argument types
*Tips on choosing the correct argument types*

1. If you need to store the argument somewhere - Favor taking ownership(receive a value). <br/>
2. If you need to do a calculation with the value - Favor receiving a read-only ref. <br/>
3. If you need to change the value in some way - Favor receiving a mutable ref. <br/>

**These rules are not absolute!!!**
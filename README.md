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
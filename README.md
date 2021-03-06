[![Build Status](https://travis-ci.org/joshmarlow/behold-rs.svg?branch=master)](https://travis-ci.org/joshmarlow/behold-rs)

Behold
======

A simple library to aide in contextual debugging.  This is a partial Rust port of the [Python Behold library](https://github.com/robdmc/behold).

Documentation
=============
API documentation is available [here](https://docs.rs/behold/).

Changelog is available [here](CHANGELOG.md).

Core Concepts
=============

Sometimes print debugging is the best way to examine the state of a running program.
But frequently you only care about the prints in a particular code path.
With `Behold`, one part of the program can control debugging in another.


All Behold instances share the same global context to control what to print.
However, a Behold instance can be created and configured to determine how or what it prints.

Contextual Debugging
====================

`BEHOLD.show("testing".to_string())` will print "testing" to the screen.
`BEHOLD.when(true).show("testing".to_string())` will print "testing" to the screen.
`BEHOLD.when(false).show("testing".to_string())` will do nothing.
`BEHOLD.when_context("key").show("testing".to_string())` will print "testing" to the screen but only if the "testing" key has been set to `true` previously.


```rust
extern crate behold;

use behold::behold;

fn f(idx: usize) {
    behold().show(format!("Hello from f({})!", idx));
}

fn f2(idx: usize) {
    // Do something hard to debug
    behold().when(idx % 2 == 0).show(format!("Hello from f2({})!", idx));
}

fn f3(idx: usize) {
    // Do something hard to debug
    behold().when_context(format!("f3-{}", idx).as_str()).show(format!("Hello from f3({})!", idx));
}

fn main() {
    for i in 0..3 {
        println!("\n");
        f(i);
        f2(i);
        f3(i);
        // Context is global
        behold().set_context("f3-1", true);
    }
}
```

Produces the output:

```
Behold: Hello from f(0)!
Behold: Hello from f2(0)!


Behold: Hello from f(1)!
Behold: Hello from f3(1)!


Behold: Hello from f(2)!
Behold: Hello from f2(2)!
```

Tagged Printing
===============

Each instance of Behold can be configured with a tag to help distinguish output.

```rust
extern crate behold;

use behold::behold;

fn main() {
    behold().tag("yolo").show("Hello world!".to_string());
}
```

Produces the output:

```
Hello world!, yolo
```

Contextual Execution
====================

Sometimes it's useful to perform some debugging task (such as saving state to a file) when some context is true.

```rust
extern crate behold;

use behold::behold;

fn main() {
    behold().call(&|| { println!("Hello world!"); } );
}
```

Will output:

```
"Hello world!"
```

Whereas the following:

```rust
extern crate behold;

use behold::behold;

fn main() {
    behold().when(false).call(&|| { println!("Hello world!"); } );
}
```

Will output nothing.

# C to Rust

## Step 1: Create a C library.

In this repo: ``src/add.c`` and ``src/add.h``

## Step 2: Write the ``build.rs`` file

The ``build.rs`` file will be executed before building the Rust project.
You can find informations about this file [here](https://doc.rust-lang.org/cargo/reference/build-scripts.html)

We will use it to compile and create the static C library.

### Step 2.0: Link the project to the ``build.rs`` file

In ``Cargo.toml``, under ``[package]``, add the following line:

```toml
# Cargo.toml
name = "..."
# ...
edition = "..."
build = "build.rs"
```

(You can actually name the building file as you want, however, the convention is to call it ``build.rs``)

### Step 2.1: Compile the C library

With the ``Process`` crate, write the commands needed to compile the library in ``build.rs``.

```rust
// build.rs

fn main() {
   assert!(Command::new("cc")
          .args(&["-fPIC", "-O", "-g", "-c", "src/add.c", "src/add.h",])
          .status()
          .expect("Could not compile")
  .success());

  // ...
}
```
### Step 2.2: Tell ``Cargo`` where to look

We now need to link the Rust code to the C ABI.

To do so add at the end of the ``build.rs`` file the path and the name of the library.

```rust
// build.rs

fn main() {
    // ...

    println!("cargo:rustc-link-lib={}", lib_name); // the "-l" flag
    println!("cargo:rustc-link-search={}", path_to_the_lib); // the "-L" flag
}
```

## Step 3: Write some Rust !

### Step 3.1: Tell Rust you will use extern functions

In order to use the functions from the C library, we need to tell Rust that these functions are extern to the project.

To do so, we will add an ``extern`` block to introduce the functions signatures:

```rust
// main.rs

extern "C" {
    fn add(x: i32, y: i32) -> i32;
    // ...
}
```
TODO: Write about types

### Step 3.2: Finally use the C functions

We can now use the C functions in our Rust code.
However, every call to a C functions must be done in an ``unsafe`` block, as Rust can not ensure the safety of those functions.

```rust
// main.rs

extern "C" {
    // ...
}

fn main() {
    unsafe {
        // C function call
    }
}
```

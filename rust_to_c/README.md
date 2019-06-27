# Rust to C

## Step 1: Write the Rust library

Once our Rust functions have been written, we need to wrap a bit to be able to use them in some C code.

Before: 

```rust
pub fn my_function() {
    // ...
}
```

After:

```rust
#[no_mangle]
pub extern "C" fn my_function() {
    // ...
}
```
TODO: Write about types

## Step 2: Write some C

Do not forget to include the C signature of your Rust functions, with a header file, or directly in the C code.

```c
// main.c

// Our Rust function
void my_function();

int main() {
    my_function();
    return 0;
}
```

## Step 3: Create the Rust library

In ``Cargo.toml``, we need to add the following lines to make our library:

```toml
# Cargo.toml

# ...

[lib]
name = "my_lib"
path = "src/lib.rs"
crate-type = ["staticlib"]

# ...
```

We are here creating a static library (``staticlib``).
However, we could have build other types (see [here](https://doc.rust-lang.org/cargo/reference/manifest.html#building-dynamic-or-static-libraries))

## Step 4: Link the C code to the Rust library

We now just have to compile our C code and tell the compiler where to look for the Rust lib.

```Makefile
gcc main.c -L target/release -l lib -o run
```

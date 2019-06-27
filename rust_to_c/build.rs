use std::env;

fn main() {
    env::set_var(
        "CARGO_TARGET_DIR",
        "/home/taitnet/workspace/skel/temp/rust/foo",
    );
}

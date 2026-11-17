//! SigmaOS-kernel-object — thin CLI entry point.
//!
//! The library lives in `src/lib.rs`; this binary only wires argv/stdout.

use sigmaos_kernel_object_lib::Component;

fn main() {
    let mut component = Component::new();
    if let Err(e) = component.init().and_then(|_| component.run()) {
        eprintln!("SigmaOS-kernel-object: {e}");
        std::process::exit(1);
    }
    println!("SigmaOS-kernel-object: ok");
}

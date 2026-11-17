// Integration test for SigmaOS-kernel-object
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use kernel_object as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}

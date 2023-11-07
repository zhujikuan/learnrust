// integration_test/tests/sum.rs
use integration_test::sum;
mod common;
use common::{setup, teardown};
#[test]
fn sum_test() {
    setup();
    assert_eq!(sum(6, 8), 14);
    teardown();
}

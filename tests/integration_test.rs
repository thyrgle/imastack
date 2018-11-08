extern crate imastack;

#[test]
fn basic_add() {
    assert_eq!(
        imastack::eval("1 2 + print"),
        vec![3.0]);
}

#[test]
fn basic_sub() {
    assert_eq!(
        imastack::eval("1 2 - print"),
        vec![1.0]);
}

#[test]
fn basic_mul() {
    assert_eq!(
        imastack::eval("3 3 * print"),
        vec![9.0]);
}

#[test]
fn basic_div() {
    assert_eq!(
        imastack::eval("3 6 / print"),
        vec![2.0]);
}

#[test]
fn div_by_zero_is_zero() {
    assert_eq!(
        imastack::eval("0 1 / print"),
        vec![0.0]);
}

#[test]
fn basic_swp() {
    assert_eq!(
        imastack::eval("1 2 swp print print"),
        vec![1.0, 2.0]);

}

#[test]
fn basic_dup() {
    assert_eq!(
        imastack::eval("1 dup print print"),
        vec![1.0, 1.0]);
}

#[test]
fn basic_jnz() {
    assert_eq!(
        imastack::eval("1 4 jnz 0 1 print"),
        vec![1.0]);
}

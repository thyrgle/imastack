extern crate imastack;
use imastack::ast::float::Float;

#[test]
fn basic_add() {
    assert_eq!(
        imastack::eval("1 2 + print").output,
        vec![Float(3.0)]);
}

#[test]
fn basic_sub() {
    assert_eq!(
        imastack::eval("1 2 - print").output,
        vec![Float(1.0)]);
}

#[test]
fn basic_mul() {
    assert_eq!(
        imastack::eval("3 3 * print").output,
        vec![Float(9.0)]);
}

#[test]
fn basic_div() {
    assert_eq!(
        imastack::eval("3 6 / print").output,
        vec![Float(2.0)]);
}

#[test]
fn div_by_zero_is_zero() {
    assert_eq!(
        imastack::eval("0 1 / print").output,
        vec![Float(0.0)]);
}

#[test]
fn basic_swp() {
    assert_eq!(
        imastack::eval("1 2 swp print print").output,
        vec![Float(1.0), Float(2.0)]);

}

#[test]
fn basic_dup() {
    assert_eq!(
        imastack::eval("1 dup print print").output,
        vec![Float(1.0), Float(1.0)]);
}

#[test]
fn basic_jnz() {
    assert_eq!(
        imastack::eval("1 4 jnz 0 1 print").output,
        vec![Float(1.0)]);
}

extern crate imastack;

#[test]
fn basic_add() {
    assert_eq!(
        imastack::eval("1 2 + print".to_string()),
        vec![3.0]);
}

#[test]
fn basic_sub() {
    assert_eq!(
        imastack::eval("2 1 - print".to_string()),
        vec![1.0]);
}

#[test]
fn basic_mul() {
    assert_eq!(
        imastack::eval("3 3 * print".to_string()),
        vec![9.0]);
}

#[test]
fn basic_div() {
    assert_eq!(
        imastack::eval("6 3 / print".to_string()),
        vec![2.0]);
}

#[test]
fn div_by_zero_is_zero() {
    assert_eq!(
        imastack::eval("1 0 / print".to_string()),
        vec![0.0]);
}

#[test]
fn basic_swp() {
    assert_eq!(
        imastack::eval("1 2 swp print print".to_string()),
        vec![2.0, 1.0]);

}

#[test]
fn basic_dup() {
    assert_eq!(
        imastack::eval("1 dup print print".to_string()),
        vec![1.0, 1.0]);
}

#[test]
fn basic_jnz() {
    assert_eq!(
        imastack::eval("1 4 jnz 0 1 print".to_string()),
        vec![1.0]);
}

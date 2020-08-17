use persil_derive::trace;

#[test]
fn factorial_function() {
    #[trace("calculations", "factorial")]
    fn factorial(mut n: u64) -> u64 {
        let mut p = 1;
        while n > 1 {
            p *= n;
            n -= 1;
        }
        p
    }

    persil::init("tests-app");
    factorial(10);
}

#[test]
fn single_argument() {
    #[trace("foo")]
    fn bar() {}

    persil::init("tests-app");
    bar();
}

#[test]
#[should_panic]
fn panic_if_not_init() {
    #[trace("foo", "bar")]
    fn foo() {}

    foo();
}

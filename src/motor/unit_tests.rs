use super::*;

#[test]
fn args_given_a_non_valid_argument_fails() {
    // setup
    let app = "test";
    let arg = "3";
    let args = vec![app, arg];

    // given a `StructOpt::from_iter()` constructor
    let sut = Args::from_iter_safe::<&Vec<&str>>;

    // when it is invoked
    let result = sut(&args);

    // then it should be OK
    assert_eq!(result.is_err(), true);
}

#[test]
fn args_given_a_valid_argument_succeeds() {
    // setup
    let app = "test";
    let args = vec![app];

    // given a `StructOpt::from_iter()` constructor
    let sut = Args::from_iter_safe::<&Vec<&str>>;

    // when it is invoked
    let result = sut(&args);

    // then it should be OK
    assert_eq!(result.is_ok(), true);
}


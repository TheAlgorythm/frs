use crate::try_wrap_err;

fn try_wrap_err_or_none(res: Result<u16, u16>) -> Option<Result<u16, u32>> {
    let _ok_val = try_wrap_err!(res);
    None
}

#[test]
fn try_wrap_err_of_ok() {
    assert_eq!(try_wrap_err_or_none(Ok(42)), None);
}

#[test]
fn try_wrap_err_of_err() {
    assert_eq!(try_wrap_err_or_none(Err(42)), Some(Err(42)));
}

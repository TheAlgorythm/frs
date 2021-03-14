#[macro_export]
macro_rules! try_wrap_err {
    ($res:expr) => {
        match $res {
            Ok(ok) => ok,
            Err(error) => return Some(Err(error.into())),
        }
    };
}

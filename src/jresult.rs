use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct JResult<'a, T>
where
    T: Serialize,
{
    ok: bool,
    error: Option<&'a str>,
    data: Option<T>,
}

impl<'a, T: Serialize> JResult<'a, T> {
    pub fn ok(data: T) -> Self {
        Self {
            ok: true,
            error: None,
            data: Some(data),
        }
    }

    pub fn err(err: &'a str) -> Self {
        Self {
            ok: false,
            error: Some(err),
            data: None,
        }
    }
}

#[macro_export]
macro_rules! split_colon {
    ($input:expr, [$($var:ident),+]) => {
        $(
            let Some($var) = $input.next() else {
                anyhow::bail!($crate::global::INVALID_REQUEST);
            };
        )+
    };
}

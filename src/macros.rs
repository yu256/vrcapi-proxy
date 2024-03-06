#[macro_export]
macro_rules! split_colon {
    ($input:expr, [$($var:ident),+]) => {
		let mut iter_ = $input.split(':');
        $(
            let Some($var) = iter_.next() else {
                anyhow::bail!($crate::global::INVALID_REQUEST);
            };
        )+
    };
}
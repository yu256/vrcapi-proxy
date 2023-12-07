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

#[macro_export]
macro_rules! validate {
	($auth:expr, $auth_:ident, $token:ident) => {
        let ($auth_, ref $token) = *$crate::global::AUTHORIZATION.read().await;
        anyhow::ensure!($auth == $auth_, $crate::global::INVALID_AUTH);
    };
    ($auth:expr, $token:ident) => {
        let (auth_, ref $token) = *$crate::global::AUTHORIZATION.read().await;
        anyhow::ensure!($auth == auth_, $crate::global::INVALID_AUTH);
    };
	($auth_:expr) => {
        anyhow::ensure!($auth_ == $crate::global::AUTHORIZATION.read().await.0, $crate::global::INVALID_AUTH);
    };
}

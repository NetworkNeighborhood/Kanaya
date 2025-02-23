use chrono;
use chrono::TimeZone;

// Network Neighborhood common build number convention: days past since project development begun.
pub fn get_build_number() -> i64 {
    let now = chrono::Utc::now();
    let base = chrono::Utc.with_ymd_and_hms(2025, 1, 17, 0, 0, 0).unwrap();
    let diff = now - base;
    diff.num_days()
}

#[macro_export]
macro_rules! extract_environment {
	() => {};

	($val:expr,) => {};

	(
		$name:ident
		$( $others:tt )*
	) => {
		const $name: &str = env!(stringify!($name));
		extract_environment!($( $others )*);
	};
}
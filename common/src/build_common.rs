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

/**
 * Allows generating a static array with a compile-time calculated length (like C).
 */
#[macro_export]
macro_rules! auto_length_arr {
    (
        $( #[$attr:meta] )*
        $v:vis $id:ident $name:ident: [$ty:ty; _] = $value:expr
    ) => {
        $( #[$attr] )*
        $v $id $name: [$ty; $value.len()] = $value;
    }
}
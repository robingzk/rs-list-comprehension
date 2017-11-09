#[macro_use(iproduct)] extern crate itertools;

#[macro_export]
///
/// List comprehension.
///
/// ```
/// #[macro_use] extern crate rs_list_comp;
/// assert_eq!(vec![1, 1], lc!{x*x + y ; x in 0..2, y in 0..2 ; x != y }.collect::<Vec<i32>>());
///	assert_eq!(vec!["banana", "batman"], lc!{ x.to_string() + &y.to_string() ; x in "abc".chars(), y in "de".chars()}.collect::<Vec<String>>());
/// ```
///
macro_rules! lc{
	( $map:expr; $var:pat in $iter:expr ) => (iproduct!($iter).map(|$var| $map));

	( $map:expr; $( $var:pat in $iter:expr ),* ) => (iproduct!($($iter),*).map(|($($var),*)| $map));

	( $map:expr ; $( $var:pat in $iter:expr ),* ; $filter:expr ) =>
		(iproduct!($($iter),*).filter(|&($($var),*)| $filter).map(|($($var),*)| $map));
}


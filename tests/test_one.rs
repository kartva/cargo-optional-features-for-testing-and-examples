use test_features::one::add_one;

#[test]
fn test_one () {
	assert_eq!(1 + 1, add_one(1));
}
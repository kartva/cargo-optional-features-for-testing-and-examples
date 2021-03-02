use test_features::two::add_two;

#[test]
fn test_two () {
	assert_eq!(1 + 2, add_two(1));
}
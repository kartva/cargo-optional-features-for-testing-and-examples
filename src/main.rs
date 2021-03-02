use test_features::one::add_one;

#[cfg(feature = "add_two")]
use test_features::two::add_two;

fn main () -> std::io::Result<()> {
    let num: usize = 5;

    println!("Add 1 to num: {}", add_one(num));

    #[cfg(feature = "add_two")]
    {
        println!("Add 2 to num: {}", add_two(num));
    }

    Ok(())
}

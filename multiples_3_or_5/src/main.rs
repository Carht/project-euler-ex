/*
    Multiples of 3 or 5
      https://projecteuler.net/problem=1
*/

fn mul_3_5(x: u64) -> u64 {
    if x % 3 == 0 || x % 5 == 0 {
	x
    } else {
	0
    }
}

fn main() {
    let v10: u64 = (1..10)
	.collect::<Vec<u64>>()
	.iter()
	.map(|&x: &u64| mul_3_5(x))
	.sum();
    println!("{}", v10);

    let v1000: u64 = (1 .. 1000)
	.collect::<Vec<u64>>()
	.iter()
	.map(|&x: &u64| mul_3_5(x))
	.sum();
    println!("{}", v1000);
}

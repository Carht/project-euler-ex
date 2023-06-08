fn mult_5(x: u64) -> bool {
    if x % 5 == 0 {
	return true
    } else {
	return false
    }
}

fn mult_3(x: u64) -> bool {
    if x % 3 == 0 {
	return true
    } else {
	return false
    }
}

fn main() {
    let mut sum10 = 0;
    let mut sum1000 = 0;
    
    for i in 1 .. 10 {
	if mult_3(i) || mult_5(i) {
	    sum10 += i
	}
    }

    for j in 1 .. 1000 {
	if mult_3(j) || mult_5(j) {
	    sum1000 += j
	}
    }

    println!("{}", sum10);
    println!("{}", sum1000);
}

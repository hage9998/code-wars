// Write a function that takes an integer as input, and returns the number of bits that are equal to one in the binary representation of that number. 
// You can guarantee that input is non-negative.
// Example: The binary representation of 1234 is 10011010010, so the function should return 5 in this case

fn count_bits(n: i64) -> u32 {
	let mut st: String = format!("{:b}", n);
	st.retain(|c| c == '1');
	st.len() as u32
  }

fn count_bits2(n: i64) -> u32 {
	n.count_ones()
}

fn count_bits3(n: i64) -> u32 {
    format!("{:b}", n).matches('1').count() as u32
}


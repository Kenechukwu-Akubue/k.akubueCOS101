fn main(){
	let sum:f64 = 450_000.00 + 1_500_000.00 + 750_000.00 + 2_850_000.00 + 250_000.00;
	let avg:f64 = sum/5.0;
	println!("Sum of Sales: ₦{:.2}", sum);
    println!("Average of Sales: ₦{:.2}", avg);
}
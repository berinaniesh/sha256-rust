mod calc_sum;
mod constants;

fn main() {
    let arguments = std::env::args().skip(1);
    for argument in arguments {
        let sha256sum: String = calc_sum::calc(&argument);
        println!("The SHA256 sum of {} is {}", argument, sha256sum);
    }
}

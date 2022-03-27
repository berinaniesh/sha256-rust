mod calc_sum;
mod constants;

fn main() {
    let arguments = std::env::args().skip(1); //skip first argument cuz it's the path of the executable itself. 
    for argument in arguments {
        let argu = argument.clone();
        let sha256sum: String = calc_sum::calc(argument); //returns empty string if file not found or some other error
        if sha256sum != "" {
            println!("The SHA256 sum of {} is {}", argu, sha256sum);
        }
    }
}

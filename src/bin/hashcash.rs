struct MD5HashCashInput {
    // complexity in bits
    complexity: u32,
    // message to sign
    message: String,
}

struct MD5HashCashOutput {
    // Seed used to solve the challenge
    seed: u64,
    // hashcode found using seed + message
    hashcode: String,
}

fn main() {
    let input = MD5HashCashInput {
        complexity: 9,
        message: String::from("hello")
    };

    println!("mesage: {}, complexity: {}", input.message, input.complexity);
}
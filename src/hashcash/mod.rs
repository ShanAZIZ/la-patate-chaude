
use std::num::ParseIntError;

// TODO: Multithreading
// TODO: Optimize counting zeros
// TODO: Remove panic and manage error

pub struct MD5HashCashInput {
    pub complexity: u32,
    pub message: String,
}

pub struct MD5HashCashOutput {
    pub seed: u64,
    pub hashcode: String,
}

pub fn u64_to_hexadecimal_string(i: u64) -> String {
    format!("{:016x}", i)
}

fn count_zeros(hexadecimal: &str) -> Result<u32, ParseIntError> {
    let value = u128::from_str_radix(hexadecimal, 16)?;
    Ok(value.leading_zeros())
}

fn hash(seed: &str) -> String {
    let digest = md5::compute(seed.as_bytes());
    format!("{:x}", digest)
}

pub fn find_seed(input: &MD5HashCashInput) -> MD5HashCashOutput  {
    for i in 0..u64::MAX {
        let concatenated_seed = format!("{}{}", u64_to_hexadecimal_string(i), input.message);
        let hash_string = hash(&concatenated_seed);
        let zeros = count_zeros(&hash_string);
        match zeros {
            Ok(zeros) => {
                if zeros >= input.complexity {
                    return MD5HashCashOutput {
                        seed: i as u64,
                        hashcode: hash_string,
                    };
                }
            }
            Err(e) => panic!("error: cannot convert hexadecimal to binary: {}", e)
        }
    }

    panic!("error: seed not fount");

}

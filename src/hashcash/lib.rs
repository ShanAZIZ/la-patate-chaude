
pub mod utils {
    use std::num::ParseIntError;

    pub struct MD5HashCashInput {
        pub complexity: u32,
        pub message: String,
    }

    pub struct MD5HashCashOutput {
        seed: u64,
        hashcode: String,
    }

    fn u64_to_hexadecimal_string(i: u64) -> String {
        format!("{:016x}", i)
    }

    // TODO: Optimize counting zeros
    fn count_zeros(hexadecimal: &str) -> Result<u32, ParseIntError> {
        let value = u128::from_str_radix(hexadecimal, 16)?;
        Ok(value.leading_zeros())
    }

    fn hash(seed: &str) -> String {
        let digest = md5::compute(seed.as_bytes());
        format!("{:x}", digest)
    }

    // TODO: Multithreading
    pub fn find_seed(message: &str, complexity: u32) -> String  {
        let mut i = 0;
        loop {
            let concatenated_seed = format!("{}{}", u64_to_hexadecimal_string(i), message);
            let hash_string = hash(&concatenated_seed);
            let zeros = count_zeros(&hash_string);
            match zeros {
                Ok(zeros) => {
                    if zeros >= complexity {
                        return u64_to_hexadecimal_string(i);
                    }
                }
                Err(e) => panic!("error: cannot convert hexadecimal to binary: {}", e)
            }
            i += 1;
        }
    }
}

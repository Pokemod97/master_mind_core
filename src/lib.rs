use rand::Rng;
pub fn vec_to_string(digits: Vec<u32>) -> String {
    let result: String = digits.iter().fold(String::new(), |mut result, x| {
        result.push_str(&x.to_string());
        result
    });
    result
}
pub fn generate_secret_number() -> [u32;6] {
    let mut rand_generator = rand::thread_rng();
    let secret_number: [u32; 6] = [
        rand_generator.gen_range(0, 10),
        rand_generator.gen_range(0, 10),
        rand_generator.gen_range(0, 10),
        rand_generator.gen_range(0, 10),
        rand_generator.gen_range(0, 10),
        rand_generator.gen_range(0, 10),
    ];
    secret_number

}
pub fn positions(secret_number: Vec<u32>,  guess: Vec<u32>) -> (u32,u32){
    let mut wrong_position: u32 = 0;
    let mut right_position: u32 = 0;
    let mut secret_number_clone = secret_number.clone();
    let mut guess_digits = guess.clone();
    for (i, val) in secret_number.iter().enumerate() {
        if guess_digits[i] == *val {
            right_position += 1;
            secret_number_clone[i] = 11;
            guess_digits[i] = 12;
        }
    }
    for val in guess_digits.iter() {
        if secret_number_clone.contains(val) {
            let index: usize = secret_number_clone
                .iter()
                .position(|x| x == val)
                .unwrap_or(7);
            if index != 7 {
                wrong_position += 1;
                secret_number_clone[index] = 11;
            }
        }
    }
    (right_position,wrong_position)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn vec_to_string_test() {
        assert_eq!("123".to_string(), vec_to_string([1,2,3].to_vec()));
    }
    #[test]
    fn positions_test(){
        assert_eq!((0,0), positions([0,0,0,0,0,0].to_vec(), [1,1,1,1,1,1].to_vec()));
        assert_eq!((6,0), positions([1,1,1,1,1,1].to_vec(), [1,1,1,1,1,1].to_vec()));
        assert_eq!((1,0), positions([0,0,0,0,0,1].to_vec(), [1,1,1,1,1,1].to_vec()));
        assert_eq!((2,0), positions([0,0,1,0,0,1].to_vec(), [1,1,1,1,1,1].to_vec()));
        assert_eq!((1,1), positions([0,0,1,0,0,1].to_vec(), [2,2,1,2,1,2].to_vec()));
    }
}

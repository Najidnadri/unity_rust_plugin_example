use rand::{thread_rng, Rng};


#[no_mangle]
pub extern fn random_num() -> i32 {
    let mut rng = thread_rng();
    rng.gen()
}



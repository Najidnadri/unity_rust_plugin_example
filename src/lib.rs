use rand::{thread_rng, Rng};


#[no_mangle]
pub extern fn random_num() -> i32 {
    thread_rng().gen()
}



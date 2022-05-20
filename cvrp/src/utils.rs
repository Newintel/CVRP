use rand::{distributions::Standard, prelude::Distribution};
use wasm_bindgen::prelude::*;

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn log(text: &str);
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn log_i8(number: i8);
}

pub fn rand<T>(limit: T, forbidden: Option<T>) -> T
where
    Standard: Distribution<T>,
    T: std::ops::Rem<Output = T> + std::cmp::PartialEq + Copy,
{
    if forbidden.is_none() {
        return rand::random::<T>() % limit;
    }

    let forbidden = forbidden.unwrap();
    let mut rand = forbidden;
    while rand == forbidden {
        rand = rand::random::<T>() % limit;
    }
    rand
}

pub fn two_different_random(max: usize) -> (usize, usize) {
    let i = rand(max, None);
    let j = rand(max, Some(i));
    (i, j)
}

mod sliding_window; 
use std::env;
mod binary_search; 
#[allow(unused_imports)]
fn main() {
    use binary_search::find_min::find_min;
    use sliding_window::max_profit::max_profit;
    env::set_var("RUST_BACKTRACE", "1")
   
}


mod ad_fn_closure;
mod ad_trait;
mod unsafe_rust;

fn main() {
    unsafe_rust::run();
    ad_trait::run();
    ad_fn_closure::run();
}

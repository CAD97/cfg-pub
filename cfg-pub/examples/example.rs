use cfg_pub::cfg_pub;

#[cfg_pub(
    if #[cfg(SET_CFG)] pub
    else if #[cfg(not(SET_CFG))] pub(self)
)]
fn main() {
    println!("do something");
}

pub mod hosting;
pub mod serving {
    pub fn take_order() {
        serve_order();
        take_payment();
    }
    fn serve_order() {}
    fn take_payment() {}
}
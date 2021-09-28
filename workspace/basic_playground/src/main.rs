pub mod closures;
pub mod find_and_show;
pub mod slices;

#[warn(unused_variables)]
fn main() {
    find_and_show::find_and_show();
    closures::run();
    // slices::slices_play();
    let _a: String = String::from("akapulko");
}

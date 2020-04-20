#[allow(dead_code)]
trait SomeTrait {

}

#[allow(dead_code)]
pub fn run() {
    println!("size of the trait object is {}", std::mem::size_of::<&dyn SomeTrait>());
}
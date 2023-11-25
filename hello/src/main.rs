use haiku_kits;
use haiku_kits::methods::*;

fn main() {
    println!("Hello, World.");
    let signature = "application/x-vnd.vendor-application";
    let app = haiku_kits::Application::new_with_str(signature);
    app.run();
}

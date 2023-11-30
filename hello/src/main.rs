use haiku_kits as B;
use B::methods::*;

fn main() {
    println!("Hello, World.");
    let signature = "application/x-vnd.vendor-application";
    let app = B::Application::new_with_str(signature);

    // Haiku/BeOS seems lack of default positioning API.
    let rect = B::Rect::new_with_float_float(100.0, 100.0, 320.0, 240.0);
    let window = B::Window::new_with_rect_window_type(
        &rect,
        "Hello",
        B::B_TITLED_WINDOW,
        0,
        B::B_CURRENT_WORKSPACE as u32,
    );
    window.show();

    app.run();
}

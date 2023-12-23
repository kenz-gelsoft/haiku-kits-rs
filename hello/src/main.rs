use B::methods::*;
use four_cc::FourCC;

struct MyWindowHooks();

impl WindowHooks for MyWindowHooks {
    fn message_received<M: MessageMethods>(&self, message: &M) {
    	println!("Hello");
    }
}

fn main() {
    println!("Hello, World.");
    let signature = "application/x-vnd.vendor-application";
    let app = B::Application::new_with_str(Some(signature));

    let rect = B::Rect::new_with_float_float(0.0, 0.0, 320.0, 240.0);
    let hooks = MyWindowHooks {};
    let window = B::RustWindow::new(
        &rect,
        "Hello",
        B::TITLED_WINDOW,
        0,
        0,//B::CURRENT_WORKSPACE as u32,
        hooks,
    );
    // Place window to left top of screen.
    let pt = window.decorator_frame();
    window.move_by(-pt.left(), -pt.top());

    let hello_message = B::Message::new_with_uint32(FourCC(*b"helo").into());
    let button = B::Button::new_with_rect(
        &rect,
        None,
        Some("Button"),
        Some(&hello_message),
        B::FOLLOW_ALL_SIDES as u32,
        0,
    );
    window.add_child_view(Some(&button), B::View::none());

    if let Some(first_child) = window.child_at(0) {
        if let Some(as_button) = B::Button::dynamic_cast(&first_child) {
            // Set label through dynamic_cast
            as_button.set_label(Some("World"));
        }
    }

    window.show();

    app.run();
}

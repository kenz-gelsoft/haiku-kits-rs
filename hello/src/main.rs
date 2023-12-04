use B::methods::*;

fn main() {
    println!("Hello, World.");
    let signature = "application/x-vnd.vendor-application";
    let app = B::Application::new_with_str(Some(signature));

    let rect = B::Rect::new_with_float_float(0.0, 0.0, 320.0, 240.0);
    let window = B::Window::new_with_rect_window_type(
        &rect,
        Some("Hello"),
        B::B_TITLED_WINDOW,
        0,
        B::B_CURRENT_WORKSPACE as u32,
    );
    // Place window to left top of screen.
    let pt = window.decorator_frame();
    window.move_by(-pt.left(), -pt.top());

    let button = B::Button::new_with_rect(
        &rect,
        None,
        Some("Button"),
        B::Message::none(),
        B::B_FOLLOW_ALL_SIDES as u32,
        0,
    );
    window.add_child_view(Some(&button), B::View::none());

    window.show();

    app.run();
}

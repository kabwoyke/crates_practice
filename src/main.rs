fn main() {
    let mut app = simple::Window::new("Example App", 1920, 1080);
    app.set_color(200, 100, 50, 255);
    while app.next_frame(){}
}

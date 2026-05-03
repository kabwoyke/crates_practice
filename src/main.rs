fn main() {
    let mut app = simple::Window::new("Example App", 1920, 1080);
    app.set_color(200, 100, 50, 255);
    app.draw_rect(simple::Rect::new(110, 210, 900, 400));
    while app.next_frame(){}
}

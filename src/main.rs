use gpui::*;

mod base64_state;
mod components;

use components::text_convert_view::TextConvertView;

fn main() {
    Application::new().run(|cx: &mut App| {
        cx.open_window(WindowOptions::default(), |_, cx| {
            cx.new(|_cx| TextConvertView::new())
        })
        .unwrap();
    });
}
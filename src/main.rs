use gpui::*;

mod base64_state;
mod components;

use gpui_component::{
    *,
};
use components::text_convert_view::TextConvertView;

fn main() {
    let app = Application::new();

    app.run(|cx| {
        gpui_component::init(cx);

        let window_options = WindowOptions {
            window_bounds: Some(WindowBounds::centered(size(px(800.), px(600.)), cx)),
            ..Default::default()
        };

        cx.spawn(async move |cx| {
            let _ = cx.open_window(window_options, |window, cx| {
                let view = cx.new(|cx| TextConvertView::new(window, cx));

                cx.new(|cx| Root::new(view.into(), window, cx))
            });

            Ok::<_, anyhow::Error>(())
        })
        .detach();
    })
}
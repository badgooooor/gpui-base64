use gpui::*;

pub struct TextConvertView {
    plain_text: SharedString,
    cipher_text: SharedString,
}

impl TextConvertView {
    pub fn new() -> Self {
        Self {
            plain_text: "aGVsbG8=".into(),
            cipher_text: "hello".into(),
        }
    }
}

impl Render for TextConvertView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0xf5f5f5))
            .p_4()
            .gap_4()
            .child(
                // Header
                div()
                    .flex()
                    .justify_center()
                    .mb_2()
                    .child(
                        div()
                            .text_xl()
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0x333333))
                            .child("Base64 Text Converter")
                    )
            )
            .child(
                // Main content container
                div()
                    .flex()
                    .flex_col()
                    .gap_6()
                    .max_w(px(800.0))
                    .mx_auto()
                    .child(
                        // Plain text section
                        div()
                            .flex()
                            .flex_col()
                            .gap_2()
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(rgb(0x555555))
                                    .child("Plain Text")
                            )
                            .child(
                                div()
                                    .w_full()
                                    .p(px(12.0))
                                    .border(px(1.0))
                                    .border_color(rgb(0xcccccc))
                                    .rounded_md()
                                    .bg(rgb(0xffffff))
                                    .text_color(rgb(0x333333))
                                    .child(self.plain_text.clone())
                            )
                    )
                    .child(
                        // Cipher text section
                        div()
                            .flex()
                            .flex_col()
                            .gap_2()
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(rgb(0x555555))
                                    .child("Cipher Text (Base64)")
                            )
                            .child(
                                div()
                                    .w_full()
                                    .p(px(12.0))
                                    .border(px(1.0))
                                    .border_color(rgb(0xcccccc))
                                    .rounded_md()
                                    .bg(rgb(0xffffff))
                                    .text_color(rgb(0x333333))
                                    .child(self.cipher_text.clone())
                            )
                    )
            )
            .child(
                // Example section
                div()
                    .flex()
                    .justify_center()
                    .mt_4()
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(0x666666))
                            .child("Example: 'hello' ↔ 'aGVsbG8='")
                    )
            )
    }
}
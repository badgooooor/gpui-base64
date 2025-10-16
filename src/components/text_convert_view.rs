use gpui::*;
use gpui_component::{
    input::{InputEvent, InputState, TextInput}
};

use crate::base64_state::Base64State;

#[derive(PartialEq, Copy, Clone)]
enum FocusedInput {
    None,
    PlainText,
    CipherText,
}

pub struct TextConvertView {
    base64_state: Base64State,

    plain_text_input: Entity<InputState>,
    plain_text: SharedString,
    cipher_text_input: Entity<InputState>,
    cipher_text: SharedString,

    focused_input: FocusedInput,

    _subscriptions: Vec<Subscription>,
}

impl TextConvertView {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let base64_state = Base64State::new();

        let mut focused_input = FocusedInput::None;

        let plain_text_input = cx.new(|cx| InputState::new(window, cx).placeholder("Plain Text"));
        let cipher_text_input = cx.new(|cx| InputState::new(window, cx).placeholder("Cipher Text")); 

        let _subscriptions = vec![
            cx.subscribe_in(&plain_text_input, window, {
                let plain_text_input = plain_text_input.clone();
                let cipher_text_input = cipher_text_input.clone();
                move |this, _, ev: &InputEvent, window, cx| match ev {
                    InputEvent::Focus => {
                        focused_input = FocusedInput::PlainText;
                    }
                    InputEvent::Blur => {
                        focused_input = FocusedInput::None;
                    }
                    InputEvent::Change => {
                        if focused_input == FocusedInput::PlainText {
                            // Update plain text input
                            let value = plain_text_input.read(cx).value();
                            this.plain_text = value.clone().into();

                            // Update cipher text input
                            this.base64_state.encode(&value);
                            this.cipher_text = this.base64_state.cipher_text.clone().into();
                        
                            // Update the cipher text input directly
                            let cipher_text_value: SharedString = this.base64_state.cipher_text.clone().into();
                            cipher_text_input.update(cx, |state, cx| {
                                state.set_value(cipher_text_value, window, cx);
                            });
                        }
                    }
                    _ => {}
                }
            }),
            cx.subscribe_in(&cipher_text_input, window, {
                let cipher_text_input = cipher_text_input.clone();
                let plain_text_input = plain_text_input.clone();
                move |this, _, ev: &InputEvent, window, cx| match ev {
                    InputEvent::Focus => {
                        focused_input = FocusedInput::CipherText;
                    }
                    InputEvent::Blur => {
                        focused_input = FocusedInput::None;
                    }
                    InputEvent::Change => {
                        if focused_input == FocusedInput::CipherText {
                            // Update cipher text input
                            let value = cipher_text_input.read(cx).value();
                            this.cipher_text = value.clone().into();

                            // Update plain text input
                            this.base64_state.decode(&value);
                            this.plain_text = this.base64_state.plain_text.clone().into();

                            // Update the plain text input directly
                            let plain_text_value: SharedString = this.base64_state.plain_text.clone().into();
                            plain_text_input.update(cx, |state, cx| {
                                state.set_value(plain_text_value, window, cx);
                            });
                        }
                    }
                    _ => {}
                }
            }),
        ];
    
        Self {
            base64_state,
            plain_text: SharedString::default(),
            cipher_text: SharedString::default(),
            plain_text_input,
            cipher_text_input,
            focused_input,
            _subscriptions
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
                        div()
                            .flex()
                            .flex_col()
                            .gap_2()
                            .child(
                                TextInput::new(&self.plain_text_input)
                            )
                            .child(
                                self.plain_text.clone()
                            )
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_2()
                            .child(
                                TextInput::new(&self.cipher_text_input)
                            )
                            .child(
                                self.cipher_text.clone()
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
use gpui::*;
use gpui_component::{
    input::{InputEvent, InputState, TextInput}
};

use crate::{actions::app_actions::{FocusCipherTextInput, FocusPlainTextInput, Reset}, base64_state::Base64State};
use super::state_tab::StateTab;

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

    state_tab: StateTab,

    _subscriptions: Vec<Subscription>,
    
    plain_text_input_focus_handle: FocusHandle,
    cipher_text_input_focus_handle: FocusHandle,
    focus_handle: FocusHandle,
}

impl TextConvertView {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let mut state_tab = StateTab::new();
        state_tab.set_info_text("Ready to encode/decode base64 text".to_string());


        let base64_state = Base64State::new();
        let mut focused_input = FocusedInput::None;

        let plain_text_input = cx.new(|cx| 
            InputState::new(window, cx)
                .code_editor("plaintext")
                .placeholder("Plain Text")
                .line_number(true)
                .rows(10)
                .searchable(true)
        );
        let cipher_text_input = cx.new(|cx| 
            InputState::new(window, cx)
                .code_editor("ciphertext")
                .placeholder("Cipher Text")
                .line_number(true)
                .rows(10)
                .searchable(true)
        );

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

                            this.update_state_tab();
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

                            this.update_state_tab();
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
            state_tab,
            _subscriptions,
            plain_text_input_focus_handle: cx.focus_handle(),
            cipher_text_input_focus_handle: cx.focus_handle(),
            focus_handle: cx.focus_handle(),
        }
    }

    fn focus_plain_text_input(&mut self, _: &FocusPlainTextInput, window: &mut Window, cx: &mut Context<Self>) {        
        self.plain_text_input.update(cx, |input_state, cx| {
            input_state.focus(window, cx);
        });
        cx.notify();
    }

    fn focus_cipher_text_input(&mut self, _: &FocusCipherTextInput, window: &mut Window, cx: &mut Context<Self>) {
        self.cipher_text_input.update(cx, |input_state, cx| {
            input_state.focus(window, cx);
        });
        cx.notify();
    }

    fn reset(&mut self, _: &Reset, window: &mut Window, cx: &mut Context<Self>) {
        self.base64_state.reset();
        self.plain_text = SharedString::default();
        self.cipher_text = SharedString::default();

        self.plain_text_input.update(cx, |input_state, cx| {
            input_state.set_value(SharedString::default(), window, cx);
        });
        self.cipher_text_input.update(cx, |input_state, cx| {
            input_state.set_value(SharedString::default(), window, cx);
        });
        cx.notify();
    }

    fn update_state_tab(&mut self) {
        if self.base64_state.invalid_base64 {
            self.state_tab.set_error_text("Invalid base64 text".to_string());
        } else {
            self.state_tab.set_info_text("Ready to encode/decode base64 text".to_string());
        }
    }
}

impl Focusable for TextConvertView {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for TextConvertView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .track_focus(&self.focus_handle)
            .key_context("TextConvertView")
            .on_action(cx.listener(Self::focus_plain_text_input))
            .on_action(cx.listener(Self::focus_cipher_text_input))
            .on_action(cx.listener(Self::reset))
            .flex()
            .flex_col()
            .size_full()
            .gap_0()
            .bg(rgb(0xf5f5f5))
            .child(
                // Main content container
                div()
                    .flex()
                    .flex_col()
                    .size_full()
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .size_full()
                            .track_focus(&self.plain_text_input_focus_handle)
                            .child(
                                TextInput::new(&self.plain_text_input)
                                    .size_full()
                                    .min_h(px(200.0))
                                    .rounded_none()
                            )
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .size_full()
                            .track_focus(&self.cipher_text_input_focus_handle)
                            .child(
                                TextInput::new(&self.cipher_text_input)
                                    .size_full()
                                    .min_h(px(200.0))
                                    .rounded_none()
                            )
                    )
            )
            .child(
                self.state_tab.clone().render(window, cx)
            )
    }
}
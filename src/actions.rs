use gpui::*;

pub mod app_actions {
    use gpui::actions;

    actions!(app, [
        FocusPlainTextInput,
        FocusCipherTextInput,
        Reset
    ]);
}

pub fn bind_app_actions(cx: &mut App) {
    cx.bind_keys([
        KeyBinding::new("cmd-e", app_actions::FocusPlainTextInput, Some("TextConvertView")),
        KeyBinding::new("cmd-d", app_actions::FocusCipherTextInput, Some("TextConvertView")),
        KeyBinding::new("cmd-r", app_actions::Reset, Some("TextConvertView")),
    ]);
}

use app_actions::*;

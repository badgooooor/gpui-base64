use gpui::*;

#[derive(PartialEq, Copy, Clone)]
enum StateTabStatus {
    Info,
    Error,
}

#[derive(IntoElement, Clone)]
pub struct StateTab {
    status: StateTabStatus,
    text: String,
}

impl StateTab {
    pub fn new() -> Self {
        Self {
            status: StateTabStatus::Info,
            text: String::new(),
        }
    }

    pub fn set_info_text(&mut self, text: String) {
        self.status = StateTabStatus::Info;
        self.text = text;
    }

    pub fn set_error_text(&mut self, text: String) {
        self.status = StateTabStatus::Error;
        self.text = text;
    }
}

impl RenderOnce for StateTab {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .px_4()
            .py_2()
            .bg(match self.status {
                StateTabStatus::Info => rgb(0xe3f2fd),
                StateTabStatus::Error => rgb(0xffebee),
            })
            .border_b_1()
            .border_color(match self.status {
                StateTabStatus::Info => rgb(0x2196f3),
                StateTabStatus::Error => rgb(0xf44336),
            })
            .text_color(match self.status {
                StateTabStatus::Info => rgb(0x1976d2),
                StateTabStatus::Error => rgb(0xd32f2f),
            })
            .text_sm()
            .child(
                div()
                    .child(self.text)
            )
    }
}

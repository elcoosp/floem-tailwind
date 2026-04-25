use floem::style::{CursorStyle, Style};

pub trait TailwindInteractivityExt {
    fn cursor_pointer(self) -> Self;
    fn cursor_default(self) -> Self;
    fn cursor_wait(self) -> Self;
    fn cursor_text(self) -> Self;
    fn cursor_move(self) -> Self;
    fn cursor_grab(self) -> Self;
    fn select_none(self) -> Self;
    fn select_text(self) -> Self;
    fn select_all(self) -> Self;
    fn select_auto(self) -> Self;
    fn pointer_events_none(self) -> Self;
    fn pointer_events_auto(self) -> Self;
}

impl TailwindInteractivityExt for Style {
    fn cursor_pointer(mut self) -> Self { self = self.cursor(CursorStyle::Pointer); self }
    fn cursor_default(mut self) -> Self { self = self.cursor(CursorStyle::Default); self }
    fn cursor_wait(mut self) -> Self { self = self.cursor(CursorStyle::Wait); self }
    fn cursor_text(mut self) -> Self { self = self.cursor(CursorStyle::Text); self }
    fn cursor_move(mut self) -> Self { self = self.cursor(CursorStyle::Move); self }
    fn cursor_grab(mut self) -> Self { self = self.cursor(CursorStyle::Grab); self }
    fn select_none(mut self) -> Self { self = self.selectable(false); self }
    fn select_text(mut self) -> Self { self = self.selectable(true); self }
    fn select_all(mut self) -> Self { self = self.selectable(true); self }
    fn select_auto(mut self) -> Self { self = self.selectable(true); self }
    fn pointer_events_none(self) -> Self { self }
    fn pointer_events_auto(self) -> Self { self }
}

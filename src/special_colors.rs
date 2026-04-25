use super::colors;
use floem::style::Style;

pub trait TailwindSpecialColorExt {
    fn bg_white(self) -> Self;
    fn bg_black(self) -> Self;
    fn bg_transparent(self) -> Self;

    fn text_white(self) -> Self;
    fn text_black(self) -> Self;
    fn text_transparent(self) -> Self;

    fn border_white(self) -> Self;
    fn border_black(self) -> Self;
    fn border_transparent(self) -> Self;
}

impl TailwindSpecialColorExt for Style {
    fn bg_white(mut self) -> Self {
        self = self.background(colors::WHITE);
        self
    }
    fn bg_black(mut self) -> Self {
        self = self.background(colors::BLACK);
        self
    }
    fn bg_transparent(mut self) -> Self {
        self = self.background(colors::TRANSPARENT);
        self
    }

    fn text_white(mut self) -> Self {
        self = self.color(colors::WHITE);
        self
    }
    fn text_black(mut self) -> Self {
        self = self.color(colors::BLACK);
        self
    }
    fn text_transparent(mut self) -> Self {
        self = self.color(colors::TRANSPARENT);
        self
    }

    fn border_white(mut self) -> Self {
        self = self.border_color(colors::WHITE);
        self
    }
    fn border_black(mut self) -> Self {
        self = self.border_color(colors::BLACK);
        self
    }
    fn border_transparent(mut self) -> Self {
        self = self.border_color(colors::TRANSPARENT);
        self
    }
}

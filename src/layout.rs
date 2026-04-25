use floem::style::{Display, Position, Style};

pub trait TailwindLayoutExt {
    fn block(self) -> Self;
    fn flex(self) -> Self;
    fn grid(self) -> Self;
    fn hidden(self) -> Self;
    fn relative(self) -> Self;
    fn absolute(self) -> Self;
    fn z_0(self) -> Self;
    fn z_10(self) -> Self;
    fn z_20(self) -> Self;
    fn z_30(self) -> Self;
    fn z_40(self) -> Self;
    fn z_50(self) -> Self;
    fn z_auto(self) -> Self;
    fn inset_x_0(self) -> Self;
    fn inset_y_0(self) -> Self;
    fn top_0(self) -> Self;
    fn right_0(self) -> Self;
    fn bottom_0(self) -> Self;
    fn left_0(self) -> Self;
    fn truncate(self) -> Self;
    fn text_ellipsis(self) -> Self;
    fn text_clip(self) -> Self;
}

impl TailwindLayoutExt for Style {
    fn block(mut self) -> Self { self = self.display(Display::Block); self }
    fn flex(mut self) -> Self { self = self.display(Display::Flex); self }
    fn grid(mut self) -> Self { self = self.display(Display::Grid); self }
    fn hidden(mut self) -> Self { self = self.hide(); self }
    fn relative(mut self) -> Self { self = self.position(Position::Relative); self }
    fn absolute(mut self) -> Self { self = self.position(Position::Absolute); self }
    fn z_0(mut self) -> Self { self = self.z_index(0); self }
    fn z_10(mut self) -> Self { self = self.z_index(10); self }
    fn z_20(mut self) -> Self { self = self.z_index(20); self }
    fn z_30(mut self) -> Self { self = self.z_index(30); self }
    fn z_40(mut self) -> Self { self = self.z_index(40); self }
    fn z_50(mut self) -> Self { self = self.z_index(50); self }
    fn z_auto(mut self) -> Self { self = self.z_index(i32::MAX); self }
    fn inset_x_0(mut self) -> Self { self = self.inset_left(0.0).inset_right(0.0); self }
    fn inset_y_0(mut self) -> Self { self = self.inset_top(0.0).inset_bottom(0.0); self }
    fn top_0(mut self) -> Self { self = self.inset_top(0.0); self }
    fn right_0(mut self) -> Self { self = self.inset_right(0.0); self }
    fn bottom_0(mut self) -> Self { self = self.inset_bottom(0.0); self }
    fn left_0(mut self) -> Self { self = self.inset_left(0.0); self }
    fn truncate(mut self) -> Self { self = self.text_ellipsis(); self }
    fn text_ellipsis(mut self) -> Self { self = self.text_ellipsis(); self }
    fn text_clip(mut self) -> Self { self = self.text_clip(); self }
}

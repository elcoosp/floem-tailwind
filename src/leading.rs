use floem::style::Style;

pub trait TailwindLeadingExt {
    fn leading_3(self) -> Self;
    fn leading_4(self) -> Self;
    fn leading_5(self) -> Self;
    fn leading_6(self) -> Self;
    fn leading_7(self) -> Self;
    fn leading_8(self) -> Self;
    fn leading_9(self) -> Self;
    fn leading_10(self) -> Self;
    fn leading_none(self) -> Self;
    fn leading_tight(self) -> Self;
    fn leading_snug(self) -> Self;
    fn leading_normal(self) -> Self;
    fn leading_relaxed(self) -> Self;
    fn leading_loose(self) -> Self;
}

impl TailwindLeadingExt for Style {
    fn leading_3(mut self) -> Self { self = self.line_height(12.0); self }
    fn leading_4(mut self) -> Self { self = self.line_height(16.0); self }
    fn leading_5(mut self) -> Self { self = self.line_height(20.0); self }
    fn leading_6(mut self) -> Self { self = self.line_height(24.0); self }
    fn leading_7(mut self) -> Self { self = self.line_height(28.0); self }
    fn leading_8(mut self) -> Self { self = self.line_height(32.0); self }
    fn leading_9(mut self) -> Self { self = self.line_height(36.0); self }
    fn leading_10(mut self) -> Self { self = self.line_height(40.0); self }
    fn leading_none(mut self) -> Self { self = self.line_height(1.0); self }
    fn leading_tight(mut self) -> Self { self = self.line_height(1.25); self }
    fn leading_snug(mut self) -> Self { self = self.line_height(1.375); self }
    fn leading_normal(mut self) -> Self { self = self.line_height(1.5); self }
    fn leading_relaxed(mut self) -> Self { self = self.line_height(1.625); self }
    fn leading_loose(mut self) -> Self { self = self.line_height(2.0); self }
}

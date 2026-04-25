use floem::style::Style;

pub trait TailwindNumberFontSizeExt {
    fn text_12(self) -> Self;
    fn text_14(self) -> Self;
    fn text_16(self) -> Self;
    fn text_18(self) -> Self;
    fn text_20(self) -> Self;
    fn text_24(self) -> Self;
    fn text_28(self) -> Self;
    fn text_32(self) -> Self;
    fn text_36(self) -> Self;
    fn text_40(self) -> Self;
    fn text_44(self) -> Self;
    fn text_48(self) -> Self;
    fn text_52(self) -> Self;
    fn text_56(self) -> Self;
    fn text_60(self) -> Self;
    fn text_64(self) -> Self;
    fn text_72(self) -> Self;
    fn text_80(self) -> Self;
    fn text_96(self) -> Self;
}

impl TailwindNumberFontSizeExt for Style {
    fn text_12(mut self) -> Self {
        self = self.font_size(12.0);
        self
    }
    fn text_14(mut self) -> Self {
        self = self.font_size(14.0);
        self
    }
    fn text_16(mut self) -> Self {
        self = self.font_size(16.0);
        self
    }
    fn text_18(mut self) -> Self {
        self = self.font_size(18.0);
        self
    }
    fn text_20(mut self) -> Self {
        self = self.font_size(20.0);
        self
    }
    fn text_24(mut self) -> Self {
        self = self.font_size(24.0);
        self
    }
    fn text_28(mut self) -> Self {
        self = self.font_size(28.0);
        self
    }
    fn text_32(mut self) -> Self {
        self = self.font_size(32.0);
        self
    }
    fn text_36(mut self) -> Self {
        self = self.font_size(36.0);
        self
    }
    fn text_40(mut self) -> Self {
        self = self.font_size(40.0);
        self
    }
    fn text_44(mut self) -> Self {
        self = self.font_size(44.0);
        self
    }
    fn text_48(mut self) -> Self {
        self = self.font_size(48.0);
        self
    }
    fn text_52(mut self) -> Self {
        self = self.font_size(52.0);
        self
    }
    fn text_56(mut self) -> Self {
        self = self.font_size(56.0);
        self
    }
    fn text_60(mut self) -> Self {
        self = self.font_size(60.0);
        self
    }
    fn text_64(mut self) -> Self {
        self = self.font_size(64.0);
        self
    }
    fn text_72(mut self) -> Self {
        self = self.font_size(72.0);
        self
    }
    fn text_80(mut self) -> Self {
        self = self.font_size(80.0);
        self
    }
    fn text_96(mut self) -> Self {
        self = self.font_size(96.0);
        self
    }
}

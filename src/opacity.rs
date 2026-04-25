use floem::style::Style;

pub trait TailwindOpacityExt {
    fn opacity_0(self) -> Self;
    fn opacity_5(self) -> Self;
    fn opacity_10(self) -> Self;
    fn opacity_15(self) -> Self;
    fn opacity_20(self) -> Self;
    fn opacity_25(self) -> Self;
    fn opacity_30(self) -> Self;
    fn opacity_35(self) -> Self;
    fn opacity_40(self) -> Self;
    fn opacity_45(self) -> Self;
    fn opacity_50(self) -> Self;
    fn opacity_55(self) -> Self;
    fn opacity_60(self) -> Self;
    fn opacity_65(self) -> Self;
    fn opacity_70(self) -> Self;
    fn opacity_75(self) -> Self;
    fn opacity_80(self) -> Self;
    fn opacity_85(self) -> Self;
    fn opacity_90(self) -> Self;
    fn opacity_95(self) -> Self;
    fn opacity_100(self) -> Self;
}

impl TailwindOpacityExt for Style {
    fn opacity_0(mut self) -> Self {
        self = self.opacity(0.00);
        self
    }
    fn opacity_5(mut self) -> Self {
        self = self.opacity(0.05);
        self
    }
    fn opacity_10(mut self) -> Self {
        self = self.opacity(0.10);
        self
    }
    fn opacity_15(mut self) -> Self {
        self = self.opacity(0.15);
        self
    }
    fn opacity_20(mut self) -> Self {
        self = self.opacity(0.20);
        self
    }
    fn opacity_25(mut self) -> Self {
        self = self.opacity(0.25);
        self
    }
    fn opacity_30(mut self) -> Self {
        self = self.opacity(0.30);
        self
    }
    fn opacity_35(mut self) -> Self {
        self = self.opacity(0.35);
        self
    }
    fn opacity_40(mut self) -> Self {
        self = self.opacity(0.40);
        self
    }
    fn opacity_45(mut self) -> Self {
        self = self.opacity(0.45);
        self
    }
    fn opacity_50(mut self) -> Self {
        self = self.opacity(0.50);
        self
    }
    fn opacity_55(mut self) -> Self {
        self = self.opacity(0.55);
        self
    }
    fn opacity_60(mut self) -> Self {
        self = self.opacity(0.60);
        self
    }
    fn opacity_65(mut self) -> Self {
        self = self.opacity(0.65);
        self
    }
    fn opacity_70(mut self) -> Self {
        self = self.opacity(0.70);
        self
    }
    fn opacity_75(mut self) -> Self {
        self = self.opacity(0.75);
        self
    }
    fn opacity_80(mut self) -> Self {
        self = self.opacity(0.80);
        self
    }
    fn opacity_85(mut self) -> Self {
        self = self.opacity(0.85);
        self
    }
    fn opacity_90(mut self) -> Self {
        self = self.opacity(0.90);
        self
    }
    fn opacity_95(mut self) -> Self {
        self = self.opacity(0.95);
        self
    }
    fn opacity_100(mut self) -> Self {
        self = self.opacity(1.00);
        self
    }
}

use super::colors;
use floem::style::Style;

pub trait TailwindColorExt {
    // Background colors
    fn bg_neutral_50(self) -> Self; fn bg_neutral_100(self) -> Self; fn bg_neutral_200(self) -> Self; fn bg_neutral_300(self) -> Self;
    fn bg_neutral_400(self) -> Self; fn bg_neutral_500(self) -> Self; fn bg_neutral_600(self) -> Self; fn bg_neutral_700(self) -> Self;
    fn bg_neutral_800(self) -> Self; fn bg_neutral_900(self) -> Self; fn bg_neutral_950(self) -> Self;
    fn bg_stone_50(self) -> Self; fn bg_stone_100(self) -> Self; fn bg_stone_200(self) -> Self; fn bg_stone_300(self) -> Self;
    fn bg_stone_400(self) -> Self; fn bg_stone_500(self) -> Self; fn bg_stone_600(self) -> Self; fn bg_stone_700(self) -> Self;
    fn bg_stone_800(self) -> Self; fn bg_stone_900(self) -> Self; fn bg_stone_950(self) -> Self;
    fn bg_amber_50(self) -> Self; fn bg_amber_100(self) -> Self; fn bg_amber_200(self) -> Self; fn bg_amber_300(self) -> Self;
    fn bg_amber_400(self) -> Self; fn bg_amber_500(self) -> Self; fn bg_amber_600(self) -> Self; fn bg_amber_700(self) -> Self;
    fn bg_amber_800(self) -> Self; fn bg_amber_900(self) -> Self; fn bg_amber_950(self) -> Self;
    fn bg_lime_50(self) -> Self; fn bg_lime_100(self) -> Self; fn bg_lime_200(self) -> Self; fn bg_lime_300(self) -> Self;
    fn bg_lime_400(self) -> Self; fn bg_lime_500(self) -> Self; fn bg_lime_600(self) -> Self; fn bg_lime_700(self) -> Self;
    fn bg_lime_800(self) -> Self; fn bg_lime_900(self) -> Self; fn bg_lime_950(self) -> Self;
    fn bg_green_50(self) -> Self; fn bg_green_100(self) -> Self; fn bg_green_200(self) -> Self; fn bg_green_300(self) -> Self;
    fn bg_green_400(self) -> Self; fn bg_green_500(self) -> Self; fn bg_green_600(self) -> Self; fn bg_green_700(self) -> Self;
    fn bg_green_800(self) -> Self; fn bg_green_900(self) -> Self; fn bg_green_950(self) -> Self;
    fn bg_emerald_50(self) -> Self; fn bg_emerald_100(self) -> Self; fn bg_emerald_200(self) -> Self; fn bg_emerald_300(self) -> Self;
    fn bg_emerald_400(self) -> Self; fn bg_emerald_500(self) -> Self; fn bg_emerald_600(self) -> Self; fn bg_emerald_700(self) -> Self;
    fn bg_emerald_800(self) -> Self; fn bg_emerald_900(self) -> Self; fn bg_emerald_950(self) -> Self;
    fn bg_teal_50(self) -> Self; fn bg_teal_100(self) -> Self; fn bg_teal_200(self) -> Self; fn bg_teal_300(self) -> Self;
    fn bg_teal_400(self) -> Self; fn bg_teal_500(self) -> Self; fn bg_teal_600(self) -> Self; fn bg_teal_700(self) -> Self;
    fn bg_teal_800(self) -> Self; fn bg_teal_900(self) -> Self; fn bg_teal_950(self) -> Self;
    fn bg_cyan_50(self) -> Self; fn bg_cyan_100(self) -> Self; fn bg_cyan_200(self) -> Self; fn bg_cyan_300(self) -> Self;
    fn bg_cyan_400(self) -> Self; fn bg_cyan_500(self) -> Self; fn bg_cyan_600(self) -> Self; fn bg_cyan_700(self) -> Self;
    fn bg_cyan_800(self) -> Self; fn bg_cyan_900(self) -> Self; fn bg_cyan_950(self) -> Self;
    fn bg_sky_50(self) -> Self; fn bg_sky_100(self) -> Self; fn bg_sky_200(self) -> Self; fn bg_sky_300(self) -> Self;
    fn bg_sky_400(self) -> Self; fn bg_sky_500(self) -> Self; fn bg_sky_600(self) -> Self; fn bg_sky_700(self) -> Self;
    fn bg_sky_800(self) -> Self; fn bg_sky_900(self) -> Self; fn bg_sky_950(self) -> Self;
    fn bg_blue_50(self) -> Self; fn bg_blue_100(self) -> Self; fn bg_blue_200(self) -> Self; fn bg_blue_300(self) -> Self;
    fn bg_blue_400(self) -> Self; fn bg_blue_500(self) -> Self; fn bg_blue_600(self) -> Self; fn bg_blue_700(self) -> Self;
    fn bg_blue_800(self) -> Self; fn bg_blue_900(self) -> Self; fn bg_blue_950(self) -> Self;
    fn bg_indigo_50(self) -> Self; fn bg_indigo_100(self) -> Self; fn bg_indigo_200(self) -> Self; fn bg_indigo_300(self) -> Self;
    fn bg_indigo_400(self) -> Self; fn bg_indigo_500(self) -> Self; fn bg_indigo_600(self) -> Self; fn bg_indigo_700(self) -> Self;
    fn bg_indigo_800(self) -> Self; fn bg_indigo_900(self) -> Self; fn bg_indigo_950(self) -> Self;
    fn bg_violet_50(self) -> Self; fn bg_violet_100(self) -> Self; fn bg_violet_200(self) -> Self; fn bg_violet_300(self) -> Self;
    fn bg_violet_400(self) -> Self; fn bg_violet_500(self) -> Self; fn bg_violet_600(self) -> Self; fn bg_violet_700(self) -> Self;
    fn bg_violet_800(self) -> Self; fn bg_violet_900(self) -> Self; fn bg_violet_950(self) -> Self;
    fn bg_purple_50(self) -> Self; fn bg_purple_100(self) -> Self; fn bg_purple_200(self) -> Self; fn bg_purple_300(self) -> Self;
    fn bg_purple_400(self) -> Self; fn bg_purple_500(self) -> Self; fn bg_purple_600(self) -> Self; fn bg_purple_700(self) -> Self;
    fn bg_purple_800(self) -> Self; fn bg_purple_900(self) -> Self; fn bg_purple_950(self) -> Self;
    fn bg_fuchsia_50(self) -> Self; fn bg_fuchsia_100(self) -> Self; fn bg_fuchsia_200(self) -> Self; fn bg_fuchsia_300(self) -> Self;
    fn bg_fuchsia_400(self) -> Self; fn bg_fuchsia_500(self) -> Self; fn bg_fuchsia_600(self) -> Self; fn bg_fuchsia_700(self) -> Self;
    fn bg_fuchsia_800(self) -> Self; fn bg_fuchsia_900(self) -> Self; fn bg_fuchsia_950(self) -> Self;
    fn bg_pink_50(self) -> Self; fn bg_pink_100(self) -> Self; fn bg_pink_200(self) -> Self; fn bg_pink_300(self) -> Self;
    fn bg_pink_400(self) -> Self; fn bg_pink_500(self) -> Self; fn bg_pink_600(self) -> Self; fn bg_pink_700(self) -> Self;
    fn bg_pink_800(self) -> Self; fn bg_pink_900(self) -> Self; fn bg_pink_950(self) -> Self;
    fn bg_rose_50(self) -> Self; fn bg_rose_100(self) -> Self; fn bg_rose_200(self) -> Self; fn bg_rose_300(self) -> Self;
    fn bg_rose_400(self) -> Self; fn bg_rose_500(self) -> Self; fn bg_rose_600(self) -> Self; fn bg_rose_700(self) -> Self;
    fn bg_rose_800(self) -> Self; fn bg_rose_900(self) -> Self; fn bg_rose_950(self) -> Self;

    // Text colors
    fn text_neutral_50(self) -> Self; fn text_neutral_100(self) -> Self; fn text_neutral_200(self) -> Self; fn text_neutral_300(self) -> Self;
    fn text_neutral_400(self) -> Self; fn text_neutral_500(self) -> Self; fn text_neutral_600(self) -> Self; fn text_neutral_700(self) -> Self;
    fn text_neutral_800(self) -> Self; fn text_neutral_900(self) -> Self; fn text_neutral_950(self) -> Self;
    fn text_stone_50(self) -> Self; fn text_stone_100(self) -> Self; fn text_stone_200(self) -> Self; fn text_stone_300(self) -> Self;
    fn text_stone_400(self) -> Self; fn text_stone_500(self) -> Self; fn text_stone_600(self) -> Self; fn text_stone_700(self) -> Self;
    fn text_stone_800(self) -> Self; fn text_stone_900(self) -> Self; fn text_stone_950(self) -> Self;
    fn text_amber_50(self) -> Self; fn text_amber_100(self) -> Self; fn text_amber_200(self) -> Self; fn text_amber_300(self) -> Self;
    fn text_amber_400(self) -> Self; fn text_amber_500(self) -> Self; fn text_amber_600(self) -> Self; fn text_amber_700(self) -> Self;
    fn text_amber_800(self) -> Self; fn text_amber_900(self) -> Self; fn text_amber_950(self) -> Self;
    fn text_lime_50(self) -> Self; fn text_lime_100(self) -> Self; fn text_lime_200(self) -> Self; fn text_lime_300(self) -> Self;
    fn text_lime_400(self) -> Self; fn text_lime_500(self) -> Self; fn text_lime_600(self) -> Self; fn text_lime_700(self) -> Self;
    fn text_lime_800(self) -> Self; fn text_lime_900(self) -> Self; fn text_lime_950(self) -> Self;
    fn text_green_50(self) -> Self; fn text_green_100(self) -> Self; fn text_green_200(self) -> Self; fn text_green_300(self) -> Self;
    fn text_green_400(self) -> Self; fn text_green_500(self) -> Self; fn text_green_600(self) -> Self; fn text_green_700(self) -> Self;
    fn text_green_800(self) -> Self; fn text_green_900(self) -> Self; fn text_green_950(self) -> Self;
    fn text_emerald_50(self) -> Self; fn text_emerald_100(self) -> Self; fn text_emerald_200(self) -> Self; fn text_emerald_300(self) -> Self;
    fn text_emerald_400(self) -> Self; fn text_emerald_500(self) -> Self; fn text_emerald_600(self) -> Self; fn text_emerald_700(self) -> Self;
    fn text_emerald_800(self) -> Self; fn text_emerald_900(self) -> Self; fn text_emerald_950(self) -> Self;
    fn text_teal_50(self) -> Self; fn text_teal_100(self) -> Self; fn text_teal_200(self) -> Self; fn text_teal_300(self) -> Self;
    fn text_teal_400(self) -> Self; fn text_teal_500(self) -> Self; fn text_teal_600(self) -> Self; fn text_teal_700(self) -> Self;
    fn text_teal_800(self) -> Self; fn text_teal_900(self) -> Self; fn text_teal_950(self) -> Self;
    fn text_cyan_50(self) -> Self; fn text_cyan_100(self) -> Self; fn text_cyan_200(self) -> Self; fn text_cyan_300(self) -> Self;
    fn text_cyan_400(self) -> Self; fn text_cyan_500(self) -> Self; fn text_cyan_600(self) -> Self; fn text_cyan_700(self) -> Self;
    fn text_cyan_800(self) -> Self; fn text_cyan_900(self) -> Self; fn text_cyan_950(self) -> Self;
    fn text_sky_50(self) -> Self; fn text_sky_100(self) -> Self; fn text_sky_200(self) -> Self; fn text_sky_300(self) -> Self;
    fn text_sky_400(self) -> Self; fn text_sky_500(self) -> Self; fn text_sky_600(self) -> Self; fn text_sky_700(self) -> Self;
    fn text_sky_800(self) -> Self; fn text_sky_900(self) -> Self; fn text_sky_950(self) -> Self;
    fn text_blue_50(self) -> Self; fn text_blue_100(self) -> Self; fn text_blue_200(self) -> Self; fn text_blue_300(self) -> Self;
    fn text_blue_400(self) -> Self; fn text_blue_500(self) -> Self; fn text_blue_600(self) -> Self; fn text_blue_700(self) -> Self;
    fn text_blue_800(self) -> Self; fn text_blue_900(self) -> Self; fn text_blue_950(self) -> Self;
    fn text_indigo_50(self) -> Self; fn text_indigo_100(self) -> Self; fn text_indigo_200(self) -> Self; fn text_indigo_300(self) -> Self;
    fn text_indigo_400(self) -> Self; fn text_indigo_500(self) -> Self; fn text_indigo_600(self) -> Self; fn text_indigo_700(self) -> Self;
    fn text_indigo_800(self) -> Self; fn text_indigo_900(self) -> Self; fn text_indigo_950(self) -> Self;
    fn text_violet_50(self) -> Self; fn text_violet_100(self) -> Self; fn text_violet_200(self) -> Self; fn text_violet_300(self) -> Self;
    fn text_violet_400(self) -> Self; fn text_violet_500(self) -> Self; fn text_violet_600(self) -> Self; fn text_violet_700(self) -> Self;
    fn text_violet_800(self) -> Self; fn text_violet_900(self) -> Self; fn text_violet_950(self) -> Self;
    fn text_purple_50(self) -> Self; fn text_purple_100(self) -> Self; fn text_purple_200(self) -> Self; fn text_purple_300(self) -> Self;
    fn text_purple_400(self) -> Self; fn text_purple_500(self) -> Self; fn text_purple_600(self) -> Self; fn text_purple_700(self) -> Self;
    fn text_purple_800(self) -> Self; fn text_purple_900(self) -> Self; fn text_purple_950(self) -> Self;
    fn text_fuchsia_50(self) -> Self; fn text_fuchsia_100(self) -> Self; fn text_fuchsia_200(self) -> Self; fn text_fuchsia_300(self) -> Self;
    fn text_fuchsia_400(self) -> Self; fn text_fuchsia_500(self) -> Self; fn text_fuchsia_600(self) -> Self; fn text_fuchsia_700(self) -> Self;
    fn text_fuchsia_800(self) -> Self; fn text_fuchsia_900(self) -> Self; fn text_fuchsia_950(self) -> Self;
    fn text_pink_50(self) -> Self; fn text_pink_100(self) -> Self; fn text_pink_200(self) -> Self; fn text_pink_300(self) -> Self;
    fn text_pink_400(self) -> Self; fn text_pink_500(self) -> Self; fn text_pink_600(self) -> Self; fn text_pink_700(self) -> Self;
    fn text_pink_800(self) -> Self; fn text_pink_900(self) -> Self; fn text_pink_950(self) -> Self;
    fn text_rose_50(self) -> Self; fn text_rose_100(self) -> Self; fn text_rose_200(self) -> Self; fn text_rose_300(self) -> Self;
    fn text_rose_400(self) -> Self; fn text_rose_500(self) -> Self; fn text_rose_600(self) -> Self; fn text_rose_700(self) -> Self;
    fn text_rose_800(self) -> Self; fn text_rose_900(self) -> Self; fn text_rose_950(self) -> Self;

    // Border colors
    fn border_neutral_50(self) -> Self; fn border_neutral_100(self) -> Self; fn border_neutral_200(self) -> Self; fn border_neutral_300(self) -> Self;
    fn border_neutral_400(self) -> Self; fn border_neutral_500(self) -> Self; fn border_neutral_600(self) -> Self; fn border_neutral_700(self) -> Self;
    fn border_neutral_800(self) -> Self; fn border_neutral_900(self) -> Self; fn border_neutral_950(self) -> Self;
    fn border_stone_50(self) -> Self; fn border_stone_100(self) -> Self; fn border_stone_200(self) -> Self; fn border_stone_300(self) -> Self;
    fn border_stone_400(self) -> Self; fn border_stone_500(self) -> Self; fn border_stone_600(self) -> Self; fn border_stone_700(self) -> Self;
    fn border_stone_800(self) -> Self; fn border_stone_900(self) -> Self; fn border_stone_950(self) -> Self;
    fn border_amber_50(self) -> Self; fn border_amber_100(self) -> Self; fn border_amber_200(self) -> Self; fn border_amber_300(self) -> Self;
    fn border_amber_400(self) -> Self; fn border_amber_500(self) -> Self; fn border_amber_600(self) -> Self; fn border_amber_700(self) -> Self;
    fn border_amber_800(self) -> Self; fn border_amber_900(self) -> Self; fn border_amber_950(self) -> Self;
    fn border_lime_50(self) -> Self; fn border_lime_100(self) -> Self; fn border_lime_200(self) -> Self; fn border_lime_300(self) -> Self;
    fn border_lime_400(self) -> Self; fn border_lime_500(self) -> Self; fn border_lime_600(self) -> Self; fn border_lime_700(self) -> Self;
    fn border_lime_800(self) -> Self; fn border_lime_900(self) -> Self; fn border_lime_950(self) -> Self;
    fn border_green_50(self) -> Self; fn border_green_100(self) -> Self; fn border_green_200(self) -> Self; fn border_green_300(self) -> Self;
    fn border_green_400(self) -> Self; fn border_green_500(self) -> Self; fn border_green_600(self) -> Self; fn border_green_700(self) -> Self;
    fn border_green_800(self) -> Self; fn border_green_900(self) -> Self; fn border_green_950(self) -> Self;
    fn border_emerald_50(self) -> Self; fn border_emerald_100(self) -> Self; fn border_emerald_200(self) -> Self; fn border_emerald_300(self) -> Self;
    fn border_emerald_400(self) -> Self; fn border_emerald_500(self) -> Self; fn border_emerald_600(self) -> Self; fn border_emerald_700(self) -> Self;
    fn border_emerald_800(self) -> Self; fn border_emerald_900(self) -> Self; fn border_emerald_950(self) -> Self;
    fn border_teal_50(self) -> Self; fn border_teal_100(self) -> Self; fn border_teal_200(self) -> Self; fn border_teal_300(self) -> Self;
    fn border_teal_400(self) -> Self; fn border_teal_500(self) -> Self; fn border_teal_600(self) -> Self; fn border_teal_700(self) -> Self;
    fn border_teal_800(self) -> Self; fn border_teal_900(self) -> Self; fn border_teal_950(self) -> Self;
    fn border_cyan_50(self) -> Self; fn border_cyan_100(self) -> Self; fn border_cyan_200(self) -> Self; fn border_cyan_300(self) -> Self;
    fn border_cyan_400(self) -> Self; fn border_cyan_500(self) -> Self; fn border_cyan_600(self) -> Self; fn border_cyan_700(self) -> Self;
    fn border_cyan_800(self) -> Self; fn border_cyan_900(self) -> Self; fn border_cyan_950(self) -> Self;
    fn border_sky_50(self) -> Self; fn border_sky_100(self) -> Self; fn border_sky_200(self) -> Self; fn border_sky_300(self) -> Self;
    fn border_sky_400(self) -> Self; fn border_sky_500(self) -> Self; fn border_sky_600(self) -> Self; fn border_sky_700(self) -> Self;
    fn border_sky_800(self) -> Self; fn border_sky_900(self) -> Self; fn border_sky_950(self) -> Self;
    fn border_blue_50(self) -> Self; fn border_blue_100(self) -> Self; fn border_blue_200(self) -> Self; fn border_blue_300(self) -> Self;
    fn border_blue_400(self) -> Self; fn border_blue_500(self) -> Self; fn border_blue_600(self) -> Self; fn border_blue_700(self) -> Self;
    fn border_blue_800(self) -> Self; fn border_blue_900(self) -> Self; fn border_blue_950(self) -> Self;
    fn border_indigo_50(self) -> Self; fn border_indigo_100(self) -> Self; fn border_indigo_200(self) -> Self; fn border_indigo_300(self) -> Self;
    fn border_indigo_400(self) -> Self; fn border_indigo_500(self) -> Self; fn border_indigo_600(self) -> Self; fn border_indigo_700(self) -> Self;
    fn border_indigo_800(self) -> Self; fn border_indigo_900(self) -> Self; fn border_indigo_950(self) -> Self;
    fn border_violet_50(self) -> Self; fn border_violet_100(self) -> Self; fn border_violet_200(self) -> Self; fn border_violet_300(self) -> Self;
    fn border_violet_400(self) -> Self; fn border_violet_500(self) -> Self; fn border_violet_600(self) -> Self; fn border_violet_700(self) -> Self;
    fn border_violet_800(self) -> Self; fn border_violet_900(self) -> Self; fn border_violet_950(self) -> Self;
    fn border_purple_50(self) -> Self; fn border_purple_100(self) -> Self; fn border_purple_200(self) -> Self; fn border_purple_300(self) -> Self;
    fn border_purple_400(self) -> Self; fn border_purple_500(self) -> Self; fn border_purple_600(self) -> Self; fn border_purple_700(self) -> Self;
    fn border_purple_800(self) -> Self; fn border_purple_900(self) -> Self; fn border_purple_950(self) -> Self;
    fn border_fuchsia_50(self) -> Self; fn border_fuchsia_100(self) -> Self; fn border_fuchsia_200(self) -> Self; fn border_fuchsia_300(self) -> Self;
    fn border_fuchsia_400(self) -> Self; fn border_fuchsia_500(self) -> Self; fn border_fuchsia_600(self) -> Self; fn border_fuchsia_700(self) -> Self;
    fn border_fuchsia_800(self) -> Self; fn border_fuchsia_900(self) -> Self; fn border_fuchsia_950(self) -> Self;
    fn border_pink_50(self) -> Self; fn border_pink_100(self) -> Self; fn border_pink_200(self) -> Self; fn border_pink_300(self) -> Self;
    fn border_pink_400(self) -> Self; fn border_pink_500(self) -> Self; fn border_pink_600(self) -> Self; fn border_pink_700(self) -> Self;
    fn border_pink_800(self) -> Self; fn border_pink_900(self) -> Self; fn border_pink_950(self) -> Self;
    fn border_rose_50(self) -> Self; fn border_rose_100(self) -> Self; fn border_rose_200(self) -> Self; fn border_rose_300(self) -> Self;
    fn border_rose_400(self) -> Self; fn border_rose_500(self) -> Self; fn border_rose_600(self) -> Self; fn border_rose_700(self) -> Self;
    fn border_rose_800(self) -> Self; fn border_rose_900(self) -> Self; fn border_rose_950(self) -> Self;
}

macro_rules! gen_color_impl {
    ($prefix:ident, $palette:ident, bg) => {
        fn $prefix_50(mut self) -> Self { self.background(colors::$palette::C50); self }
        fn $prefix_100(mut self) -> Self { self.background(colors::$palette::C100); self }
        fn $prefix_200(mut self) -> Self { self.background(colors::$palette::C200); self }
        fn $prefix_300(mut self) -> Self { self.background(colors::$palette::C300); self }
        fn $prefix_400(mut self) -> Self { self.background(colors::$palette::C400); self }
        fn $prefix_500(mut self) -> Self { self.background(colors::$palette::C500); self }
        fn $prefix_600(mut self) -> Self { self.background(colors::$palette::C600); self }
        fn $prefix_700(mut self) -> Self { self.background(colors::$palette::C700); self }
        fn $prefix_800(mut self) -> Self { self.background(colors::$palette::C800); self }
        fn $prefix_900(mut self) -> Self { self.background(colors::$palette::C900); self }
        fn $prefix_950(mut self) -> Self { self.background(colors::$palette::C950); self }
    };
    ($prefix:ident, $palette:ident, text) => {
        fn $prefix_50(mut self) -> Self { self.color(colors::$palette::C50); self }
        fn $prefix_100(mut self) -> Self { self.color(colors::$palette::C100); self }
        fn $prefix_200(mut self) -> Self { self.color(colors::$palette::C200); self }
        fn $prefix_300(mut self) -> Self { self.color(colors::$palette::C300); self }
        fn $prefix_400(mut self) -> Self { self.color(colors::$palette::C400); self }
        fn $prefix_500(mut self) -> Self { self.color(colors::$palette::C500); self }
        fn $prefix_600(mut self) -> Self { self.color(colors::$palette::C600); self }
        fn $prefix_700(mut self) -> Self { self.color(colors::$palette::C700); self }
        fn $prefix_800(mut self) -> Self { self.color(colors::$palette::C800); self }
        fn $prefix_900(mut self) -> Self { self.color(colors::$palette::C900); self }
        fn $prefix_950(mut self) -> Self { self.color(colors::$palette::C950); self }
    };
    ($prefix:ident, $palette:ident, border) => {
        fn $prefix_50(mut self) -> Self { self.border_color(colors::$palette::C50); self }
        fn $prefix_100(mut self) -> Self { self.border_color(colors::$palette::C100); self }
        fn $prefix_200(mut self) -> Self { self.border_color(colors::$palette::C200); self }
        fn $prefix_300(mut self) -> Self { self.border_color(colors::$palette::C300); self }
        fn $prefix_400(mut self) -> Self { self.border_color(colors::$palette::C400); self }
        fn $prefix_500(mut self) -> Self { self.border_color(colors::$palette::C500); self }
        fn $prefix_600(mut self) -> Self { self.border_color(colors::$palette::C600); self }
        fn $prefix_700(mut self) -> Self { self.border_color(colors::$palette::C700); self }
        fn $prefix_800(mut self) -> Self { self.border_color(colors::$palette::C800); self }
        fn $prefix_900(mut self) -> Self { self.border_color(colors::$palette::C900); self }
        fn $prefix_950(mut self) -> Self { self.border_color(colors::$palette::C950); self }
    };
}

impl TailwindColorExt for Style {
    gen_color_impl!(bg_neutral, neutral, bg);
    gen_color_impl!(text_neutral, neutral, text);
    gen_color_impl!(border_neutral, neutral, border);
    gen_color_impl!(bg_stone, stone, bg);
    gen_color_impl!(text_stone, stone, text);
    gen_color_impl!(border_stone, stone, border);
    gen_color_impl!(bg_amber, amber, bg);
    gen_color_impl!(text_amber, amber, text);
    gen_color_impl!(border_amber, amber, border);
    gen_color_impl!(bg_lime, lime, bg);
    gen_color_impl!(text_lime, lime, text);
    gen_color_impl!(border_lime, lime, border);
    gen_color_impl!(bg_green, green, bg);
    gen_color_impl!(text_green, green, text);
    gen_color_impl!(border_green, green, border);
    gen_color_impl!(bg_emerald, emerald, bg);
    gen_color_impl!(text_emerald, emerald, text);
    gen_color_impl!(border_emerald, emerald, border);
    gen_color_impl!(bg_teal, teal, bg);
    gen_color_impl!(text_teal, teal, text);
    gen_color_impl!(border_teal, teal, border);
    gen_color_impl!(bg_cyan, cyan, bg);
    gen_color_impl!(text_cyan, cyan, text);
    gen_color_impl!(border_cyan, cyan, border);
    gen_color_impl!(bg_sky, sky, bg);
    gen_color_impl!(text_sky, sky, text);
    gen_color_impl!(border_sky, sky, border);
    gen_color_impl!(bg_blue, blue, bg);
    gen_color_impl!(text_blue, blue, text);
    gen_color_impl!(border_blue, blue, border);
    gen_color_impl!(bg_indigo, indigo, bg);
    gen_color_impl!(text_indigo, indigo, text);
    gen_color_impl!(border_indigo, indigo, border);
    gen_color_impl!(bg_violet, violet, bg);
    gen_color_impl!(text_violet, violet, text);
    gen_color_impl!(border_violet, violet, border);
    gen_color_impl!(bg_purple, purple, bg);
    gen_color_impl!(text_purple, purple, text);
    gen_color_impl!(border_purple, purple, border);
    gen_color_impl!(bg_fuchsia, fuchsia, bg);
    gen_color_impl!(text_fuchsia, fuchsia, text);
    gen_color_impl!(border_fuchsia, fuchsia, border);
    gen_color_impl!(bg_pink, pink, bg);
    gen_color_impl!(text_pink, pink, text);
    gen_color_impl!(border_pink, pink, border);
    gen_color_impl!(bg_rose, rose, bg);
    gen_color_impl!(text_rose, rose, text);
    gen_color_impl!(border_rose, rose, border);
}

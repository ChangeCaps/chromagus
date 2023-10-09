mod arg;
mod uv;

use ori::prelude::*;
use uv::UvPicker;

use crate::arg::ArgPicker;

const EXIT_COLOR: Key<Color> = Key::new("chromagus.exit_color");

#[derive(Default)]
struct Data {
    color: Color,
}

impl Data {
    fn top_bar(&mut self) -> impl View<Self> {
        let title = text("Chromagus");
        let exit = button(fa::icon("xmark"))
            .padding(pt(4.0))
            .border_radius(pt(12.0))
            .color(style(EXIT_COLOR));

        let exit = on_click(exit, |cx, _| {
            let id = cx.window().id();
            cx.close_window(id);
        });

        let top_bar = hstack![(), title, exit].justify_content(Justify::SpaceBetween);
        let top_bar = pad(pt(4.0), top_bar);
        let top_bar = on_press(top_bar, |cx, _| {
            cx.window().drag_window();
        });

        container(width(FILL, top_bar))
            .background(style(Palette::BACKGROUND))
            .shadow_color(Color::BLACK)
            .shadow_blur(pt(4.0))
            .border_radius([pt(12.0), pt(12.0), pt(0.0), pt(0.0)])
    }

    fn pick(&mut self) -> impl View<Self> {
        let uv_picker = UvPicker::Hsl;
        let arg_picker = ArgPicker::Hsl;

        hstack![uv_picker, arg_picker].gap(pt(16.0))
    }

    fn rgba(&mut self) -> impl View<Self> {
        let rgba = self.color.to_rgba8();
        let rgba = format!("rgba: {}, {}, {}", rgba[0], rgba[1], rgba[2]);

        text(rgba).font_size(pt(12.0))
    }

    fn hex(&mut self) -> impl View<Self> {
        let hex = self.color.to_hex();
        let hex = format!("hex: {}", hex);

        text(hex).font_size(pt(12.0))
    }

    fn hsl(&mut self) -> impl View<Self> {
        let (h, s, l) = self.color.to_hsl();
        let hsl = format!("hsl: {:.2}, {:.2}%, {:.2}%", h, s * 100.0, l * 100.0);

        text(hsl).font_size(pt(12.0))
    }

    fn oklab(&mut self) -> impl View<Self> {
        let (l, a, b) = self.color.to_oklab();
        let oklab = format!("oklab: {:.2}, {:.2}, {:.2}", l, a, b);

        text(oklab).font_size(pt(12.0))
    }

    fn output(&mut self) -> impl View<Self> {
        let rgba = self.rgba();
        let hex = self.hex();
        let hsl = self.hsl();
        let oklab = self.oklab();

        let output = vstack![rgba, hex, hsl, oklab]
            .gap(pt(4.0))
            .align_items(Align::Start);
        let output = pad(pt(8.0), output);

        container(output)
            .border_radius(pt(8.0))
            .border_width(pt(2.0))
            .border_color(style(Palette::SECONDARY_DARKER))
    }

    fn ui(&mut self) -> impl View<Self> {
        let content = vstack![self.pick(), self.output()]
            .align_items(Align::Stretch)
            .gap(pt(16.0));

        let ui = vstack![self.top_bar(), flex(1.0, center(content))];
        let ui = size(FILL, ui);

        container(ui)
            .background(style(Palette::BACKGROUND))
            .border_radius(pt(12.0))
    }

    fn theme() -> Theme {
        Theme::new().with(EXIT_COLOR, hex("#e22f4a"))
    }
}

fn main() {
    App::new(Data::ui, Data::default())
        .title("Chromagus")
        .size(400, 500)
        .resizable(false)
        .decorated(false)
        .transparent(true)
        .color(Color::TRANSPARENT)
        .theme(Data::theme)
        .run();
}

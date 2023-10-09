use ori::{core::canvas::Quad, prelude::*};

use crate::Data;

#[derive(Clone, Debug, PartialEq)]
pub enum UvPicker {
    Hsl,
    Oklab,
}

impl UvPicker {
    fn get_argument(&self, color: Color) -> f32 {
        match self {
            UvPicker::Hsl => color.to_hsl().0,
            UvPicker::Oklab => color.to_oklab().0,
        }
    }

    fn get_uv(&self, color: Color) -> (f32, f32) {
        match self {
            Self::Hsl => {
                let (_, s, l) = color.to_hsl();
                (s, l)
            }
            Self::Oklab => (0.0, 0.0),
        }
    }

    fn render_image(&self, argument: f32) -> Image {
        let mut pixels = Vec::with_capacity(256 * 256 * 4);

        for y in 0..256 {
            for x in 0..256 {
                let u = x as f32 / 255.0;
                let v = 1.0 - y as f32 / 255.0;

                let color = match self {
                    UvPicker::Hsl => hsl(argument, u, v),
                    UvPicker::Oklab => oklab(argument, u, v),
                };

                pixels.extend(color.to_rgba8());
            }
        }

        Image::new(pixels, 256, 256)
    }
}

pub struct UvPickerState {
    pub image: Image,
    pub color: Color,
}

impl View<Data> for UvPicker {
    type State = UvPickerState;

    fn build(&mut self, _cx: &mut BuildCx, data: &mut Data) -> Self::State {
        let argument = self.get_argument(data.color);
        let image = self.render_image(argument);

        UvPickerState {
            image,
            color: data.color,
        }
    }

    fn rebuild(
        &mut self,
        state: &mut Self::State,
        cx: &mut RebuildCx,
        data: &mut Data,
        old: &Self,
    ) {
        if self != old || state.color != data.color {
            let argument = self.get_argument(data.color);
            state.image = self.render_image(argument);
            state.color = data.color;

            cx.request_layout();
            cx.request_draw();
        }
    }

    fn event(&mut self, state: &mut Self::State, cx: &mut EventCx, data: &mut Data, event: &Event) {
        if event.is_handled() {
            return;
        }

        if let Some(pointer) = event.get::<PointerEvent>() {
            if !pointer.is_press() || !cx.is_hot() {
                return;
            }

            let local = cx.local(pointer.position);

            let u = local.x / state.image.width() as f32;
            let v = 1.0 - local.y / state.image.height() as f32;

            let color = match self {
                UvPicker::Hsl => hsl(state.color.to_hsl().0, u, v),
                UvPicker::Oklab => oklab(state.color.to_oklab().0, u, v),
            };

            data.color = color;

            cx.request_rebuild();
        }
    }

    fn layout(
        &mut self,
        state: &mut Self::State,
        _cx: &mut LayoutCx,
        _data: &mut Data,
        space: Space,
    ) -> Size {
        space.fit(state.image.size())
    }

    fn draw(
        &mut self,
        state: &mut Self::State,
        cx: &mut DrawCx,
        data: &mut Data,
        canvas: &mut Canvas,
    ) {
        canvas.draw_quad(
            cx.rect(),
            state.image.clone(),
            pt(8.0),
            0.0,
            Color::TRANSPARENT,
        );

        let (u, v) = self.get_uv(data.color);
        let center = cx.rect().top_left() + Vector::new(u, 1.0 - v) * cx.size().to_vector();

        canvas.draw(Quad {
            rect: Rect::center_size(center, Size::all(pt(8.0))),
            background: Color::TRANSPARENT.into(),
            border_radius: pt(4.0).into(),
            border_width: pt(2.0).into(),
            border_color: style(Palette::SECONDARY_DARKER),
        });
    }
}

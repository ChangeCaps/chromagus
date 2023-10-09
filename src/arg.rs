use ori::prelude::*;

use crate::Data;
#[derive(Clone, Debug, PartialEq)]
pub enum ArgPicker {
    Hsl,
    Oklab,
}

impl ArgPicker {
    fn get_argument(&self, color: Color) -> f32 {
        match self {
            Self::Hsl => color.to_hsl().0,
            Self::Oklab => color.to_oklab().0,
        }
    }

    fn get_arguments(&self, color: Color) -> (f32, f32) {
        match self {
            Self::Hsl => {
                let (_, s, l) = color.to_hsl();
                (s, l)
            }
            Self::Oklab => {
                let (_, a, b) = color.to_oklab();
                (a, b)
            }
        }
    }

    fn render_image(&self, color: Color) -> Image {
        let mut pixels = Vec::with_capacity(256 * 4);

        for y in 0..256 {
            let arg = 1.0 - y as f32 / 255.0;

            let (a, b) = self.get_arguments(color);

            let color = match self {
                Self::Hsl => hsl(arg * 360.0, a, b),
                Self::Oklab => oklab(arg, a, b),
            };

            pixels.extend(color.to_rgba8());
        }

        Image::new(pixels, 1, 256)
    }
}

pub struct ArgPickerState {
    image: Image,
    color: Color,
}

impl View<Data> for ArgPicker {
    type State = ArgPickerState;

    fn build(&mut self, _cx: &mut BuildCx, data: &mut Data) -> Self::State {
        let image = self.render_image(data.color);

        ArgPickerState {
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
            state.image = self.render_image(data.color);
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

            let arg = 1.0 - local.y / state.image.height() as f32;

            let (a, b) = self.get_arguments(data.color);

            let color = match self {
                Self::Hsl => hsl(arg * 360.0, a, b),
                Self::Oklab => oklab(arg, a, b),
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
        space.fit(Size::new(32.0, state.image.height() as f32))
    }

    fn draw(
        &mut self,
        state: &mut Self::State,
        cx: &mut DrawCx,
        _data: &mut Data,
        canvas: &mut Canvas,
    ) {
        canvas.draw_quad(
            cx.rect(),
            state.image.clone(),
            pt(8.0),
            0.0,
            Color::TRANSPARENT,
        );
    }
}

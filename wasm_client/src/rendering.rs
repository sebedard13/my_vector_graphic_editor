use common::pures::{Affine, Vec2};
use common::types::{Length2d, ScreenLength2d, ScreenRect};
use database::RenderOption;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

use crate::{canvas_context_2d_render::CanvasContext2DRender, SceneClient, UserSelectionClient};

#[wasm_bindgen]
impl SceneClient {
    pub fn render_main(
        &self,
        user_selection: &UserSelectionClient,
        ctx: &CanvasRenderingContext2d,
    ) -> Result<(), JsValue> {
        let transform = self.scene_context.camera.get_transform();
        let pixel_region = self.scene_context.camera.get_pixel_region();
        let mut render = CanvasContext2DRender::new(ctx, transform, pixel_region);

        ctx.clear_rect(
            pixel_region.top_left.x as f64,
            pixel_region.top_left.y as f64,
            pixel_region.width() as f64,
            pixel_region.height() as f64,
        );

        self.scene_context
            .scene_render(&mut render)
            .map_err(|e| JsValue::from_str(&e))?;

        self.scene_context
            .draw(&user_selection.selection, &mut render)
            .map_err(|e| JsValue::from_str(&e))?;
        self.scene_context
            .draw_closest_pt(&user_selection.selection, &mut render)
            .map_err(|e| JsValue::from_str(&e))?;

        Ok(())
    }

    pub fn render_cover(
        &self,
        ctx: &CanvasRenderingContext2d,
        width: f32,
        height: f32,
    ) -> Result<(), JsValue> {
        let max_rect = self.scene_context.scene.max_rect();

        let scale_x = width / max_rect.width();
        let scale_y = height / max_rect.height();

        let mut ctx_2d_renderer = CanvasContext2DRender::new(
            ctx,
            Affine::identity()
                .translate(max_rect.top_left * -1.0)
                .scale(Length2d::new(scale_x, scale_y)),
            ScreenRect::new(0.0, 0.0, width, height),
        );

        self.scene_context
            .scene_render(&mut ctx_2d_renderer)
            .map_err(|e| JsValue::from_str(&e))
    }

    pub fn image_layer(&self, layer_id: usize, width: f32, height: f32) -> Result<String, JsValue> {
        let (w, h) = contain(
            self.scene_context.camera.get_base_scale(),
            (width, height),
        );

        let max_rect = self.scene_context.scene.max_rect();

        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.create_element("canvas").unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into().unwrap();
        canvas.set_width(w as u32);
        canvas.set_height(h as u32);

        let ctx = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        let mut ctx_2d_renderer = CanvasContext2DRender::new(
            &ctx,
            Affine::identity()
                .translate(max_rect.top_left * -1.0)
                .scale(Length2d::new(w / max_rect.width(), h / max_rect.height())),
            ScreenRect::new(0.0, 0.0, width, height),
        );

        let option = RenderOption {
            only_layers: vec![layer_id.into()],
            ..Default::default()
        };

        self.scene_context
            .scene
            .render_with_options(&mut ctx_2d_renderer, option)
            .map_err(|e| JsValue::from_str(&e))?;

        canvas.to_data_url()
    }
}

fn contain(mut ratio: ScreenLength2d, size: (f32, f32)) -> (f32, f32) {
    let (width, height) = size;
    ratio.normalize();
    if ratio.x > ratio.y {
        (width, width * ratio.y / ratio.x)
    } else {
        (height * ratio.x / ratio.y, height)
    }
}

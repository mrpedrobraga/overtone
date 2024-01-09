#![allow(dead_code)]

use crate::arrangement::{Arrangement, time::Moment};

pub trait Renderer {
    type PointRenderResult;
    type RenderResult;

    fn render(&self, arrangement: &Arrangement) -> Self::RenderResult;

    fn render_point(&self, arrangement: &Arrangement, moment: Moment) -> Self::PointRenderResult;
}
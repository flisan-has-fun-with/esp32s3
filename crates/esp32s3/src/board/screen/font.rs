use embedded_graphics::{prelude::{Dimensions, DrawTarget, Point}, primitives::Rectangle};
use u8g2_fonts::{
    types::{FontColor, HorizontalAlignment, VerticalPosition},
    Content, Font, FontRenderer, LookupError,
};

pub enum RenderingIssue {
    GlyphMissing,
    NothingRendered,
}

pub struct FontPlanner<C: Content + Clone> {
    content: C,
    point: Point,
    vert_pos: VerticalPosition,
    horizon_align: HorizontalAlignment,
    renderer: FontRenderer,
}

impl<C: Content + Clone> FontPlanner<C> {
    pub fn new<F: Font>(
        content: C,
        point: Point,
        vert_pos: VerticalPosition,
        horizon_align: HorizontalAlignment,
    ) -> (Self, Option<RenderingIssue>) {
        let renderer = FontRenderer::new::<F>().with_ignore_unknown_chars(true);

        let instance = Self {
            content,
            point,
            vert_pos,
            horizon_align,
            renderer,
        };
        let issue = instance.check_rendering_issue();
        (instance, issue)
    }

    pub fn at_center<F: Font>(
        content: C,
        dimension: &impl Dimensions,
    ) -> (Self, Option<RenderingIssue>) {
        let renderer = FontRenderer::new::<F>().with_ignore_unknown_chars(true);

        let instance = Self {
            content,
            point: dimension.bounding_box().center(),
            vert_pos: VerticalPosition::Center,
            horizon_align: HorizontalAlignment::Center,
            renderer,
        };
        let issue = instance.check_rendering_issue();
        (instance, issue)
    }

    pub fn issueless(self) -> Option<IssueLessFontPlanner<C>> {
        if let Some(_) = self.check_rendering_issue() {
            None
        } else {
            Some(IssueLessFontPlanner(self))
        }
    }

    pub fn rendered_box(&self) -> Option<Rectangle> {
        self.renderer
            .get_rendered_dimensions_aligned(self.content.clone(), self.point, self.vert_pos, self.horizon_align)
            .unwrap()
    }

    pub fn check_rendering_issue(&self) -> Option<RenderingIssue> {
        let test_render = self.renderer
            .get_rendered_dimensions(self.content.clone(), self.point, self.vert_pos);

        let test_render_result = match test_render {
            Ok(result) => result,
            Err(LookupError::GlyphNotFound(_)) => {
                return Some(RenderingIssue::GlyphMissing);
            }
        };

        if test_render_result.bounding_box.is_none() {
            return Some(RenderingIssue::NothingRendered);
        }

        None
    }

    pub fn draw<D: DrawTarget>(self, color: D::Color, target: &mut D) -> Result<Option<Rectangle>, D::Error> {
        let result = self.renderer
            .render_aligned(self.content, self.point, self.vert_pos, self.horizon_align, FontColor::Transparent(color), target);

        match result {
            Ok(rect) => Ok(rect),
            Err(u8g2_fonts::Error::DisplayError(error)) => Err(error),
            Err(u8g2_fonts::Error::BackgroundColorNotSupported) => unreachable!(),
            Err(u8g2_fonts::Error::GlyphNotFound(_)) => unreachable!(),
        }
    }
}

pub struct IssueLessFontPlanner<C: Content + Clone>(pub FontPlanner<C>);

impl<C: Content + Clone> IssueLessFontPlanner<C> {
    pub fn ensured_rendered_box(&self) -> Rectangle {
        self.0.rendered_box().unwrap()
    }
}



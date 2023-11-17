use anyhow::Result;
use pangocairo::cairo::{Context, ImageSurface};
use relm4::gtk::gdk::Key;

use crate::{
    math::Vec2D,
    sketch_board::{MouseEventMsg, MouseEventType},
    style::Style,
};

use super::{Drawable, DrawableClone, Tool, ToolUpdateResult};

#[derive(Clone, Copy, Debug)]
pub struct Rectangle {
    top_left: Vec2D,
    size: Option<Vec2D>,
    style: Style,
}

impl Drawable for Rectangle {
    fn draw(&self, cx: &Context, _surface: &ImageSurface) -> Result<()> {
        let size = match self.size {
            Some(s) => s,
            None => return Ok(()), // early exit if none
        };

        let (r, g, b, a) = self.style.color.to_rgba_f64();

        cx.save()?;

        // set style
        cx.set_line_width(self.style.size.to_line_width());
        cx.set_source_rgba(r, g, b, a);

        // make rect
        cx.rectangle(self.top_left.x, self.top_left.y, size.x, size.y);

        // draw
        cx.stroke()?;

        cx.restore()?;

        Ok(())
    }
}

#[derive(Default)]
pub struct RectangleTool {
    rectangle: Option<Rectangle>,
    style: Style,
}

impl Tool for RectangleTool {
    fn handle_mouse_event(&mut self, event: MouseEventMsg) -> ToolUpdateResult {
        match event.type_ {
            MouseEventType::BeginDrag => {
                // start new
                self.rectangle = Some(Rectangle {
                    top_left: event.pos,
                    size: None,
                    style: self.style,
                });

                ToolUpdateResult::Redraw
            }
            MouseEventType::EndDrag => {
                if let Some(a) = &mut self.rectangle {
                    if event.pos == Vec2D::zero() {
                        self.rectangle = None;

                        ToolUpdateResult::Redraw
                    } else {
                        a.size = Some(event.pos);
                        let result = a.clone_box();
                        self.rectangle = None;

                        ToolUpdateResult::Commit(result)
                    }
                } else {
                    ToolUpdateResult::Unmodified
                }
            }
            MouseEventType::UpdateDrag => {
                if let Some(a) = &mut self.rectangle {
                    if event.pos == Vec2D::zero() {
                        return ToolUpdateResult::Unmodified;
                    }
                    a.size = Some(event.pos);

                    ToolUpdateResult::Redraw
                } else {
                    ToolUpdateResult::Unmodified
                }
            }
            _ => ToolUpdateResult::Unmodified,
        }
    }

    fn handle_key_event(&mut self, event: crate::sketch_board::KeyEventMsg) -> ToolUpdateResult {
        if event.key == Key::Escape && self.rectangle.is_some() {
            self.rectangle = None;
            ToolUpdateResult::Redraw
        } else {
            ToolUpdateResult::Unmodified
        }
    }

    fn handle_style_event(&mut self, style: Style) -> ToolUpdateResult {
        self.style = style;
        ToolUpdateResult::Unmodified
    }

    fn get_drawable(&self) -> Option<&dyn Drawable> {
        match &self.rectangle {
            Some(d) => Some(d),
            None => None,
        }
    }
}

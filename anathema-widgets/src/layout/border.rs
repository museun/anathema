use anathema_render::Size;
use anathema_widget_core::error::{Error, Result};
use anathema_widget_core::layout::{Constraints, Layout};
use anathema_widget_core::LayoutNodes;

pub struct BorderLayout {
    pub min_width: Option<usize>,
    pub min_height: Option<usize>,
    pub width: Option<usize>,
    pub height: Option<usize>,
    pub border_size: Size,
}

impl Layout for BorderLayout {
    fn layout(&mut self, nodes: &mut LayoutNodes<'_, '_, '_>) -> Result<Size> {
        // If there is a min width / height, make sure the minimum constraints
        // are matching these
        let mut constraints = nodes.constraints;

        if let Some(min_width) = self.min_width {
            constraints.min_width = constraints.min_width.max(min_width);
        }

        if let Some(min_height) = self.min_height {
            constraints.min_height = constraints.min_height.max(min_height);
        }

        // If there is a width / height then make the constraints tight
        // around the size. This will modify the size to fit within the
        // constraints first.
        if let Some(width) = self.width {
            constraints.make_width_tight(width);
        }

        if let Some(height) = self.height {
            constraints.make_height_tight(height);
        }

        if constraints == Constraints::ZERO {
            return Ok(Size::ZERO);
        }

        let border_size = self.border_size;

        let mut size = Size::ZERO;

        nodes.next(|mut node| {
            // Shrink the constraint for the child to fit inside the border
            let mut constraints = constraints;
            constraints.max_width = match constraints.max_width.checked_sub(border_size.width) {
                Some(w) => w,
                None => return Err(Error::InsufficientSpaceAvailble),
            };

            constraints.max_height = match constraints.max_height.checked_sub(border_size.height) {
                Some(h) => h,
                None => return Err(Error::InsufficientSpaceAvailble),
            };

            if constraints.min_width > constraints.max_width {
                constraints.min_width = constraints.max_width;
            }

            if constraints.min_height > constraints.max_height {
                constraints.min_height = constraints.max_height;
            }

            if constraints.max_width == 0 || constraints.max_height == 0 {
                return Err(Error::InsufficientSpaceAvailble);
            }

            let inner_size = node.layout(constraints)?;

            size = inner_size + border_size;

            if let Some(min_width) = self.min_width {
                size.width = size.width.max(min_width);
            }

            if let Some(min_height) = self.min_height {
                size.height = size.height.max(min_height);
            }

            Ok(())
        })?;

        size.width = size.width.max(constraints.min_width);
        size.height = size.height.max(constraints.min_height);

        Ok(size)
    }
}

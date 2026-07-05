use std::any::Any;

use cuteui::elements::Point;
use cuteui::event::DispatchedEvent;
use cuteui::geometry::rect::RectF;
use cuteui::geometry::vector::Vector2F;
use cuteui::{
    AfterLayoutContext, AppContext, Element, Event, EventContext, LayoutContext, PaintContext,
    SizeConstraint,
};

use super::{FileTreeAction, FileTreeIdentifier};

/// An element that wraps a file tree directory item and handles external file drops
/// (files dragged from outside the application, e.g. from Finder).
pub struct FileTreeDropTargetElement {
    child: Box<dyn Element>,
    id: FileTreeIdentifier,
}

impl FileTreeDropTargetElement {
    pub fn new(id: FileTreeIdentifier, child: Box<dyn Element>) -> Self {
        Self { child, id }
    }

    fn mouse_position_is_in_bounds(&self, position: Vector2F) -> bool {
        let Some(bounds) = self.bounds() else {
            return false;
        };
        bounds.contains_point(position)
    }
}

impl Element for FileTreeDropTargetElement {
    fn layout(
        &mut self,
        constraint: SizeConstraint,
        ctx: &mut LayoutContext,
        app: &AppContext,
    ) -> Vector2F {
        self.child.layout(constraint, ctx, app)
    }

    fn after_layout(&mut self, ctx: &mut AfterLayoutContext, app: &AppContext) {
        self.child.after_layout(ctx, app);
    }

    fn paint(&mut self, origin: Vector2F, ctx: &mut PaintContext, app: &AppContext) {
        self.child.paint(origin, ctx, app);
    }

    fn size(&self) -> Option<Vector2F> {
        self.child.size()
    }

    fn origin(&self) -> Option<Point> {
        self.child.origin()
    }

    fn bounds(&self) -> Option<RectF> {
        self.child.bounds()
    }

    fn parent_data(&self) -> Option<&dyn Any> {
        self.child.parent_data()
    }

    fn dispatch_event(
        &mut self,
        event: &DispatchedEvent,
        ctx: &mut EventContext,
        app: &AppContext,
    ) -> bool {
        let handled_by_child = self.child.dispatch_event(event, ctx, app);
        let Some(z_index) = self.z_index() else {
            return false;
        };

        // Handle external file drag events even if child already handled them
        if let Some(event_at_z_index) = event.at_z_index(z_index, ctx) {
            match event_at_z_index {
                Event::DragAndDropFiles { paths, location } => {
                    if self.mouse_position_is_in_bounds(*location) && !paths.is_empty() {
                        let paths: Vec<String> = paths.iter().map(ToOwned::to_owned).collect();
                        ctx.dispatch_typed_action(FileTreeAction::ExternalFilesDroppedOnDirectory {
                            id: self.id.clone(),
                            paths,
                        });
                        return true;
                    }
                }
                _ => {}
            }
        }

        handled_by_child
    }
}

use super::*;

pub struct Dummy;

impl Dummy {
    pub fn new() -> Self {
        Self {}
    }

    pub fn wrap() -> Box<Self> {
        Box::new(Self::new())
    }
}

impl Widget for Dummy {
    fn render(&mut self, _context: &mut Context, _rect: &Rect, _active: bool) {}
    fn handle_event(&mut self, _context: &mut Context, _event: &Event) {}
}

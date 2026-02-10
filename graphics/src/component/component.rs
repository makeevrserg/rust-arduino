use crate::renderer::Renderer;

pub trait Component {
    fn draw(&self, renderer: &mut dyn Renderer);
}

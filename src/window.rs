use glium::glutin::event_loop::EventLoop;
use glium::glutin::window::WindowBuilder;
use glium::glutin::ContextBuilder;
use glium::glutin::NotCurrent;

pub struct Window<'b> {
	pub event_loop: EventLoop<()>,
	pub window: WindowBuilder,
	pub context: ContextBuilder<'b, NotCurrent>,
}

impl<'b> Window<'b> {
	pub fn new(name: &str) -> Window<'b> {
		Window {
			event_loop: EventLoop::new(),
			window: WindowBuilder::new().with_title(name).with_resizable(false),
			context: ContextBuilder::new().with_depth_buffer(24),
		}
	}
}

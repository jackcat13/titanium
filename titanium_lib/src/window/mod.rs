use winit::{
    application::ApplicationHandler,
    event_loop::{ActiveEventLoop, EventLoop},
    window::Window,
};

use crate::algebra::Vec2;

pub struct TitaniumWindow {
    pub window: Option<Window>,
    pub size: Vec2,
    pub title: String,
}

impl TitaniumWindow {
    pub fn init(title: String) -> TitaniumWindow {
        let window = None;
        let size = Vec2 { x: 640.0, y: 480.0 };
        TitaniumWindow {
            window,
            size,
            title,
        }
    }

    pub fn run(&mut self) {
        let event_loop = EventLoop::new().expect("Failed to create titanium window event loop");
        event_loop.run_app(self).expect("Failed to run titanium window");
    }
}

impl ApplicationHandler for TitaniumWindow {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let size = winit::dpi::LogicalSize::new(self.size.x, self.size.y);
        let window_attributes = Window::default_attributes()
            .with_title(self.title.as_str())
            .with_inner_size(size);

        self.window = Some(
            event_loop
                .create_window(window_attributes)
                .expect("Failed to create titanium window"),
        );
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            winit::event::WindowEvent::ActivationTokenDone { serial, token } => {},
            winit::event::WindowEvent::Resized(_) => {},
            winit::event::WindowEvent::Moved(_) => {},
            winit::event::WindowEvent::CloseRequested => event_loop.exit(),
            winit::event::WindowEvent::Destroyed => {},
            winit::event::WindowEvent::DroppedFile(_) => {},
            winit::event::WindowEvent::HoveredFile(_) => {},
            winit::event::WindowEvent::HoveredFileCancelled => {},
            winit::event::WindowEvent::Focused(_) => {},
            winit::event::WindowEvent::KeyboardInput { device_id, event, is_synthetic } => {},
            winit::event::WindowEvent::ModifiersChanged(_) => {},
            winit::event::WindowEvent::Ime(_) => {},
            winit::event::WindowEvent::CursorMoved { device_id, position } => {},
            winit::event::WindowEvent::CursorEntered { device_id } => {},
            winit::event::WindowEvent::CursorLeft { device_id } => {},
            winit::event::WindowEvent::MouseWheel { device_id, delta, phase } => {},
            winit::event::WindowEvent::MouseInput { device_id, state, button } => {},
            winit::event::WindowEvent::PinchGesture { device_id, delta, phase } => {},
            winit::event::WindowEvent::PanGesture { device_id, delta, phase } => {},
            winit::event::WindowEvent::DoubleTapGesture { device_id } => {},
            winit::event::WindowEvent::RotationGesture { device_id, delta, phase } => {},
            winit::event::WindowEvent::TouchpadPressure { device_id, pressure, stage } => {},
            winit::event::WindowEvent::AxisMotion { device_id, axis, value } => {},
            winit::event::WindowEvent::Touch(_) => {},
            winit::event::WindowEvent::ScaleFactorChanged { scale_factor, inner_size_writer } => {},
            winit::event::WindowEvent::ThemeChanged(_) => {},
            winit::event::WindowEvent::Occluded(_) => {},
            winit::event::WindowEvent::RedrawRequested => {
                if let (Some(window)) = &self.window {
                    window.request_redraw();
                }
            },
        }
    }
}

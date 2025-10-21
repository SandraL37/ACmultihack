use hklib::graphics::opengl::{OpenGL2Renderer, Overlay};
use crate::game::state::SharedState;

use super::style::apply_style;

pub struct Gui {
    pub renderer: OpenGL2Renderer,
    pub ctx: imgui::Context,
}

impl Gui {
    pub fn new() -> Self {
        let window = Overlay::window();
        
        let mut ctx = imgui::Context::create();
        apply_style(&mut ctx);

        let io = ctx.io_mut();
        io.display_size = [window.width, window.height];
        io.display_framebuffer_scale = [1.0, 1.0];

        let renderer = OpenGL2Renderer::new(&mut ctx);
        renderer.create_fonts_texture(&mut ctx);
        
        Self {
            renderer,
            ctx,
        }
    }

    pub fn draw(&mut self, state: &mut SharedState) {
        let ui = self.ctx.frame();
        ui.window("Cheat menu")
            .size([400.0, 400.0], imgui::Condition::Always)
            .position([10.0, 10.0], imgui::Condition::FirstUseEver)
            .build(|| {
                ui.text("Click H to hide/show");
                
                ui.columns(2, "cheat_columns", false);
                
                // Left column
                ui.checkbox("Trigger Bot (F1)", &mut state.triggerbot);
                ui.checkbox("ESP (F2)", &mut state.esp);
                ui.checkbox("Trace Lines (F3)", &mut state.trace);
                ui.checkbox("Aimbot (F4) (LCTRL)", &mut state.aimbot);
                ui.checkbox("No Clip (C)", &mut state.no_clip);
                ui.checkbox("Crosshair (F6)", &mut state.crosshair);
                ui.checkbox("Wallbang (F7)", &mut state.wallbang);
                
                ui.next_column();
                
                // Right column
                ui.checkbox("Maphack (F8)", &mut state.maphack.is_enabled());
                ui.checkbox("No Recoil (F9)", &mut state.norecoil);
                ui.checkbox("Infinite Ammo (NUM0)", &mut state.infinite_ammo);
                ui.checkbox("Fast Shoot (NUM1)", &mut state.fastshoot);
                ui.checkbox("Fullbright (NUM2)", &mut state.fullbright.is_enabled());
                ui.checkbox("Wallhack (NUM3)", &mut state.wallhack.is_enabled());
                
                ui.columns(1, "", false); // Reset to single column
                
                ui.separator();
                ui.text("By SandraL37! Press DEL to exit.");
            });

        
        let draw_data = self.ctx.render();
        self.renderer.render(draw_data);
    }
}
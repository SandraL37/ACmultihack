pub fn apply_style(ctx: &mut imgui::Context) {
    let style = ctx.style_mut();
    style.window_rounding = 8.0;
    style.frame_rounding = 6.0;
    style.grab_rounding = 6.0;
    style.child_rounding = 6.0;
    style.popup_rounding = 8.0;
    style.scrollbar_rounding = 8.0;
    style.window_border_size = 1.0;
    style.frame_border_size = 1.0;
    style.item_spacing = [8.0, 6.0];
    style.window_padding = [10.0, 10.0];
    style.frame_padding = [8.0, 4.0];
    style.scrollbar_size = 12.0;

    // Colors
    let colors = &mut style.colors;
    use imgui::StyleColor::*;
    colors[WindowBg as usize] = [0.10, 0.10, 0.10, 0.90];
    colors[TitleBg as usize] = [0.08, 0.08, 0.08, 1.00];
    colors[TitleBgActive as usize] = [0.16, 0.16, 0.16, 1.00];
    colors[MenuBarBg as usize] = [0.14, 0.14, 0.14, 1.00];
    colors[FrameBg as usize] = [0.20, 0.20, 0.20, 1.00];
    colors[FrameBgHovered as usize] = [0.26, 0.26, 0.26, 1.00];
    colors[FrameBgActive as usize] = [0.30, 0.30, 0.30, 1.00];
    colors[Button as usize] = [0.20, 0.45, 0.20, 1.00];
    colors[ButtonHovered as usize] = [0.25, 0.55, 0.25, 1.00];
    colors[ButtonActive as usize] = [0.15, 0.35, 0.15, 1.00];
    colors[Header as usize] = [0.30, 0.30, 0.30, 1.00];
    colors[HeaderHovered as usize] = [0.40, 0.40, 0.40, 1.00];
    colors[HeaderActive as usize] = [0.25, 0.25, 0.25, 1.00];
    colors[CheckMark as usize] = [0.20, 0.60, 0.20, 1.00];
    colors[Text as usize] = [0.90, 0.90, 0.90, 1.00];
    colors[TextDisabled as usize] = [0.50, 0.50, 0.50, 1.00];
    colors[Border as usize] = [0.28, 0.28, 0.28, 1.00];
}
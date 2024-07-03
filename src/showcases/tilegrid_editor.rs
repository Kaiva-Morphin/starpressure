#[path = "../core/mod.rs"]
mod core;
#[path = "../debug/mod.rs"]
mod debug;


use debug::egui_inspector::plugin::SwitchableEguiInspectorPlugin;
use debug::diagnostics_screen::plugin::ScreenDiagnosticsPlugin;
use core::tiles::tiled_editor::plugin::TilemapEditorPlugin;

use bevy::prelude::*;





fn main() {
    let mut app = App::new();
    app.add_plugins((
        core::default::plugin::DefaultPlugin,
        SwitchableEguiInspectorPlugin,
        ScreenDiagnosticsPlugin,
        TilemapEditorPlugin
    ));
    
    app.run();
}











pub const DEFAULT_FONT_BYTES: &'static [u8; 11480] = include_bytes!("../../assets/fonts/orp.ttf");
pub mod plugin {
    use bevy::{app::{Plugin, Startup}, prelude::{default, Camera2dBundle, Commands, Transform, Vec3, App, PluginGroup}, render::texture::ImagePlugin, window::{PresentMode, Window, WindowPlugin}, DefaultPlugins};

    pub struct DefaultPlugin;

    impl Plugin for DefaultPlugin {
        fn build(&self, app: &mut App) {
            app.add_plugins((
                DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        present_mode: PresentMode::AutoNoVsync,
                        title: "Simple Game!".into(),
                            ..default()
                        }),
                            ..default()
                        }),
                )
            );
            app.add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default());
            app.add_systems(Startup, setup);
        }
    }

    fn setup(
        mut commands: Commands,
    ){
        commands.spawn(Camera2dBundle{
            transform: Transform::from_scale(Vec3::splat(1.)),
            ..default()
        });
    }


}


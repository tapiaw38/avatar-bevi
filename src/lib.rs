use bevy::prelude::*;
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::spawn_local;

#[derive(Resource)]
struct CanvasId(pub String);

#[wasm_bindgen]
pub fn initialize_bevy_app(canvas_id: &str) -> Result<(), JsValue> {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();
    
    #[cfg(target_arch = "wasm32")]
    spawn_local(async move {
        run(canvas_id.to_string());
    });

    #[cfg(not(target_arch = "wasm32"))]
    run(canvas_id.to_string());

    Ok(())
}

pub fn run(canvas_id: String) {
    let mut app = App::new();
    
    // Configuraciones específicas para WebAssembly
    #[cfg(target_arch = "wasm32")]
    {
        use bevy::winit::WinitSettings;
        use bevy::window::{PrimaryWindow, WindowRef, WindowResolution};
        use web_sys::window;
        
        let window = window().unwrap();
        let document = window.document().unwrap();
        let canvas = document.get_element_by_id(&canvas_id).unwrap();
        
        app.insert_resource(WinitSettings {
            return_from_run: true,
            ..WinitSettings::default()
        });
        
        app.insert_resource(CanvasId(canvas_id));
    }
    
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            #[cfg(target_arch = "wasm32")]
            canvas: Some(format!("#{}", app.world.resource::<CanvasId>().0)),
            ..default()
        }),
        ..default()
    }))
    .add_systems(Startup, setup)
    .add_systems(Update, rotate_model)
    .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Cámara
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 1.5, 5.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Luz
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        point_light: PointLight {
            intensity: 2000.0,
            range: 10.0,
            shadows_enabled: true,
            ..default()
        },
        ..default()
    });

    // Modelo 3D
    commands.spawn((
        SceneBundle {
            scene: asset_server.load("Mita.glb#Scene0"),
            ..default()
        },
        AvatarModel,
    ));
}

// Componente para marcar nuestro modelo
#[derive(Component)]
struct AvatarModel;

// Sistema para rotar el modelo
fn rotate_model(mut query: Query<&mut Transform, With<AvatarModel>>, time: Res<Time>) {
    for mut transform in query.iter_mut() {
        transform.rotation *= Quat::from_rotation_y(0.3 * time.delta_seconds());
    }
}

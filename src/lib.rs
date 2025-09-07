use bevy::{
    prelude::*,
    window::{Window, WindowPlugin},
    core_pipeline::clear_color::ClearColorConfig,
    asset::LoadState,
};
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
use web_sys::console;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::spawn_local;

// Función para debuggear la carga del modelo Mita
#[cfg(target_arch = "wasm32")]
fn log_external_model_loading() {
    console::log_1(&"Cargando modelo Mita.glb desde archivo local...".into());
}

#[derive(Resource)]
#[allow(dead_code)]
struct CanvasId(pub String);

// URL del modelo 3D específico del usuario (ahora local)
const MODEL_URL: &str = "models/mita.glb#Scene0";

#[wasm_bindgen]
pub fn initialize_bevy_app(canvas_id: String) -> Result<(), JsValue> {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();
    
    #[cfg(target_arch = "wasm32")]
    spawn_local(async move {
        run(canvas_id);
    });

    #[cfg(not(target_arch = "wasm32"))]
    run(canvas_id);

    Ok(())
}

fn setup(
    mut commands: Commands,
    server: Res<AssetServer>,
) {
    // Luz direccional principal
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 10000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(10.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Luz ambiental
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.5,
    });

    // Cámara con fondo de color
    commands.spawn(Camera3dBundle {
        camera_3d: Camera3d {
            clear_color: ClearColorConfig::Custom(Color::rgb(0.2, 0.2, 0.3)),
            ..default()
        },
        transform: Transform::from_xyz(0.0, 0.0, 5.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Modelo 3D específico del usuario (Mita.glb) - único modelo en la escena
    #[cfg(target_arch = "wasm32")]
    log_external_model_loading();
    
    commands.spawn(SceneBundle {
        scene: server.load(MODEL_URL),
        transform: Transform::from_xyz(0.0, 0.0, 0.0)
            .with_scale(Vec3::splat(1.0)), // Escala normal para ver el modelo
        ..default()
    }).insert(MitaModel);
}

pub fn run(canvas_id: String) {
    let mut app = App::new();
    
    #[cfg(target_arch = "wasm32")]
    {
        use bevy::winit::WinitSettings;
        app.insert_resource(WinitSettings::desktop_app())
            .insert_resource(CanvasId(canvas_id.clone()));
    }
    
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            canvas: Some(format!("#{}", canvas_id)),
            fit_canvas_to_parent: true,
            resolution: (800.0, 600.0).into(),
            ..default()
        }),
        ..default()
    }))
    .add_startup_system(setup)
    .add_system(rotate_model)
    .add_system(debug_mita_model_loading)
    .run();
}

// Componente para el modelo Mita específico
#[derive(Component)]
struct MitaModel;

// Sistema para rotar el modelo Mita
fn rotate_model(time: Res<Time>, mut query: Query<&mut Transform, With<MitaModel>>) {
    for mut transform in query.iter_mut() {
        transform.rotate_y(0.5 * time.delta_seconds());
    }
}

// Sistema para debuggear la carga del modelo Mita
fn debug_mita_model_loading(
    server: Res<AssetServer>,
    mut has_printed: Local<bool>,
    mut error_count: Local<u32>,
    mita_handles: Query<&Handle<Scene>, With<MitaModel>>,
    scenes: Res<Assets<Scene>>,
) {
    if !*has_printed {
        let load_state = server.get_load_state(MODEL_URL);
        
        match load_state {
            LoadState::Failed => {
                *error_count += 1;
                error!("Error al cargar el modelo Mita.glb (intento {})", *error_count);
                
                // Si hemos intentado demasiadas veces, marcar como impreso para evitar spam
                if *error_count >= 3 {
                    error!("Modelo Mita.glb no disponible después de {} intentos.", *error_count);
                    *has_printed = true;
                }
            }
            LoadState::Loaded => {
                info!("Modelo Mita.glb cargado exitosamente");
                *has_printed = true;
            }
            LoadState::Loading => {
                info!("Cargando modelo Mita.glb...");
            }
            LoadState::NotLoaded => {
                info!("Modelo Mita.glb no iniciado");
            }
            LoadState::Unloaded => {
                info!("Modelo Mita.glb descargado");
            }
        }
        
        // Verificar si el modelo se cargó correctamente
        for handle in mita_handles.iter() {
            if let Some(scene) = scenes.get(handle) {
                info!("Modelo Mita cargado con {} entidades", scene.world.entities().len());
                *has_printed = true;
            } else {
                info!("Modelo Mita aún no cargado en Assets<Scene>");
            }
        }
    }
}

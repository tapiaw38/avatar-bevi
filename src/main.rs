fn main() {
    // En entornos nativos, usamos "bevy-canvas" como ID para mantener la coherencia con WebAssembly
    avatar_bevy_lib::run("bevy-canvas".to_string());
}

// Weekly Flow - shell nativo de escritorio (Tauri)
// No modifica la app en si: solo la carga en una ventana nativa.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error al iniciar Weekly Flow");
}

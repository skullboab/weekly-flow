# Weekly Flow — versión nativa (Linux)

Este proyecto envuelve tu `index.html` (idéntico al que ya usas) en una
ventana nativa usando **Tauri**. La app HTML no se tocó — sigue funcionando
igual si la abres suelta en el navegador. El flujo de Importar/Exportar JSON
que ya tenías funciona igual dentro de la app nativa.

## 1. Instalar dependencias (una sola vez, en cada máquina Linux)

```bash
# Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"

# Node (si no lo tienes)
sudo apt update
sudo apt install -y nodejs npm

# Librerías del sistema que Tauri necesita en Linux
sudo apt install -y libwebkit2gtk-4.1-dev build-essential curl wget file \
  libxdo-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev
```

> Si tu distro usa `webkit2gtk-4.0` en vez de `4.1` (Ubuntu 22.04 o más viejo),
> instala `libwebkit2gtk-4.0-dev` en su lugar.

## 2. Compilar

Dentro de la carpeta del proyecto (`weekly-flow-tauri/`):

```bash
npm install
npm run tauri build
```

Esto genera los instalables en:
`src-tauri/target/release/bundle/`
- `appimage/weekly-flow_3.0.0_amd64.AppImage` → ejecutable portátil, no necesita instalación. Cópialo a la torre y a esta máquina y le das doble clic (o `chmod +x` + ejecutar).
- `deb/weekly-flow_3.0.0_amd64.deb` → instalable con `sudo dpkg -i weekly-flow_3.0.0_amd64.deb`, queda en tu menú de aplicaciones con ícono.

## 3. Uso diario

- La app abre con el último estado guardado en su propio `localStorage`
  interno (independiente del navegador).
- Usa los botones **Exportar** / **Importar** de siempre para mover tu JSON
  entre la app nativa, el navegador y tus otras máquinas.
- Puedes seguir abriendo `src/index.html` directo en Chrome/Chromium en la Mac
  sin ningún problema — es el mismo archivo, sin cambios.

## 4. Actualizar la app más adelante

Si en el futuro me pides cambios a Weekly Flow, solo reemplaza
`src/index.html` con la nueva versión y vuelve a correr `npm run tauri build`.
No hay que tocar nada más del proyecto Tauri.

## Nota sobre descargas dentro de la app nativa

El botón "Exportar" usa una descarga de blob del navegador. Dentro de
WebKitGTK normalmente esto guarda el archivo directo en `~/Descargas` (o
pregunta la ubicación, según tu distro/versión de WebKitGTK). Si notas que no
guarda donde esperas, avísame y le agrego el plugin de diálogo nativo de
Tauri (`tauri-plugin-dialog`) para que abra un "Guardar como" de verdad.

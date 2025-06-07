# 🌌 Aether Renderer Core

**Aether Renderer Core** is a lightweight, Rust-based CLI tool that converts image sequences (PNG/WebP) into transparent `.webm` or `.mp4` videos using `ffmpeg`.

Built with love for artists, developers, and sacred animation workflows.

---

## ✨ Features

- ✅ Supports alpha channel export (via `yuva420p`)
- ✅ Input PNG/WebP sequences with alpha from folder or .zip file
- ✅ Export `.webm` with alpha (via `libvpx`)
- ✅ `.mp4` fallback (no alpha)
- ✅ CLI flags for FPS, input folder, output path, format

---

## 🧪 Usage

```bash
cargo run --release -- \
  --input ./frames \
  --output my.webm \
  --fps 30 \
  --format webm
```

📂 Your input folder should contain files like:

```
frame_0000.png
frame_0001.png
frame_0002.png
...
```

---

You can now also pass a .zip file containing frames:

```bash
cargo run -- --input ./my-frames.zip --output my.webm --fps 30 --format webm
```


📂 Your input folder or ZIP file must contain images named like:

```
frame_0000.png
frame_0001.png
frame_0002.png
...
```

Alpha-enabled PNGs are recommended for transparent .webm.

(Just make sure ffmpeg is installed)

---

## 🧰 Requirements

- Rust & Cargo installed: https://rustup.rs
- `ffmpeg` must be installed and accessible in your system path

---

## 🔮 Roadmap

- [x] Render `.png` → `.webm` (with alpha)
- [ ] Support `.mp4` export
- [ ] Add bitrate / CRF quality control
- [ ] `--fade-in`, `--fade-out` for soft loops
- [ ] Handle errors & missing frames gracefully
- [ ] Add optional CLI preview
- [ ] Begin GUI version with Tauri (`aether-renderer`) 🌟

---

## 🌿 License

MIT — created with sacred care by [@madspaaskesen](https://github.com/madspaaskesen)

---

## 🌐 Related Projects

- 🕊️ [Sacred-AI](https://sacred-ai.com)
- 📈 [MySiteChart](https://mysitechart.com)
- 🛠️ [MP-IT](https://mp-it.dk)
- 🧵 [DDD Favoritter](https://ddd-favoritter.dk)

---

## 💛 Made with love by Sacred-AI

🙏 Made with clarity and care by [@mads](https://github.com/madspaaskesen) @ [@sacred-ai](https://github.com/Sacred-AI) 💛

🌸 Powered by [Rust Lang](https://www.rust-lang.org/), [Rust getting started](https://www.rust-lang.org/learn/get-started)

Aether Renderer Core is the sacred heart of a lightweight animation rendering toolkit.
Converts frame sequences to video with love, transparency, and full creative control.

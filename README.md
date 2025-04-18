# MicroCVM-rs-Assembler

**MicroCVM-rs-Assembler** is the official assembler for [MicroCVM-rs](https://github.com/5DPixel/MicroCVM-rs), a retro-style virtual machine emulator written in Rust.

This assembler lets you write clean, readable assembly for the MicroCVM architecture and outputs binary files ready to run on the VM.

---

## 🔧 Features

- Assembles human-readable `.asm` code into MicroCVM-compatible binaries
- Supports all standard MicroCVM instructions
- Easy-to-use command-line interface
- Label and constant support
- Fast and lightweight

---

## 📦 Download

You can:

- 📁 **Download precompiled binaries** from the [Releases page](https://github.com/5DPixel/MicroCVM-rs-assembler/releases)
- 📦 **Use the portable version** (no installation required)
- 🛠️ **Build from source** (see below)

---

## 🛠️ Building from Source

Requires Rust and Cargo.

```bash
git clone https://github.com/5DPixel/MicroCVM-rs-assembler
cd MicroCVM-rs-assembler
cargo build --release

# LORA Organizes & Runs Apps

> [English](./README.md) | [简体中文](./README.zh-CN.md)

> A modern app launcher based on **Tauri + Vue 3**, supporting categorized management and quick launch for efficient desktop workflows.

## 🖼️ Overview

LORA lets you manage, search, and launch Windows programs as easily as mobile apps. Clean UI, smooth operations, and drag-and-drop batch management are supported.

## 🚀 Features

- **Drag & Drop Apps**: Supports multiple executable types, batch drag, and automatic validation.
- **Category Management**: Custom groups, quick move via right-click, rename and delete supported.
- **Quick Launch**: Single/double click, run as admin, more options via right-click.
- **Smart Search**: Real-time search, keyboard shortcut focus.
- **Modern UI**: Custom title bar, responsive design, smooth animations.

## 🖥️ Screenshots

![Screenshot](image/README/1756392629877.png)

## 🛠️ Tech Stack

- **Frontend**: Vue 3 + TypeScript + Vite
- **Backend**: Rust + Tauri
- **Style**: CSS3 + Flexbox/Grid

## 📦 Installation & Usage

### Requirements

- Node.js >= 18
- Rust >= 1.60
- pnpm >= 8

### Development

```bash
pnpm install
pnpm tauri dev
```

### Build

```bash
pnpm tauri build
```

## 🎯 Usage

- **Add Apps**: Drag files or right-click to create a new project
- **Manage Categories**: Right-click the sidebar to create/rename/delete groups
- **Launch Apps**: Single/double click or right-click for options

## 🔧 Shortcuts

- `Esc`: Hide search box
- `Ctrl+F`: Focus search

## 📝 Changelog

### v0.1.2

- � Minor bug fixes and improvements

### v0.1.1

- � Performance optimizations
- 🐞 Bug fixes

### v0.1.0

- 🎉 Initial release
- 📁 Basic category management
- 🔍 App search
- 🎨 Modern UI

## 🤝 Contributing

Feel free to submit Issues and Pull Requests!

## ❓ FAQ

Q: Error on launch or unable to add apps?

A: Make sure Node.js and Rust are up-to-date and run as admin.

Q: How to batch add apps?

A: Drag multiple files to the main window.

Q: Default group can't be deleted?

A: Default group is protected from deletion to avoid accidental data loss.

## 📄 License

MIT License

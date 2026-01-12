# ARCHITECTURE.md

## 1. システム概要
Rust (Wasm) をコアエンジン、React をUI層とするハイパフォーマンスWebベースベクターエディタ。

## 2. 技術スタック
- **Frontend:** React (Vite), TypeScript, Zustand
- **Core Engine:** Rust, wasm-bindgen
- **Build Tool:** wasm-pack
- **Directory:** `rust-core/` (Rust), `src/` (React)

## 3. アーキテクチャ原則
- **Micro-kernel:** Rust側は計算とデータ管理に特化。
- **Command Pattern:** 全操作を `Command` 化し Undo/Redo を実現。
- **Unidirectional Data Flow:** Rust (State) -> Bridge -> React (UI)

## 4. ディレクトリ構造ルール
Rustのコードは必ず `rust-core/` 以下に配置すること。`src-tauri/` は使用しない。

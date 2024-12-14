// I received some assistance from a language model to write this document.

# Yew Todo App

A simple Todo application built with Rust and Yew framework. 

## Prerequisites

Before you begin, you need to install the following:

1. **Rust**
   - Install Rust by following instructions at [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
   - On Windows: Download and run rustup-init.exe
   - On macOS/Linux: Run `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

2. **Wasm Target**
   - After installing Rust, add the WebAssembly target:
   ```bash
   rustup target add wasm32-unknown-unknown
   ```

3. **Trunk**
   - Install Trunk (the build tool):
   ```bash
   cargo install trunk
   ```

## Getting Started

1. **Clone the repository**
   ```bash
   git clone https://github.com/muhammadzeejah/todo-app-with-the-Yew-framework-and-WASM.git
   cd todo-app-with-the-Yew-framework-and-WASM
   ```

2. **Run the development server**
   ```bash
   trunk serve
   ```

3. **Open in browser**
   - Navigate to [http://localhost:{port}]


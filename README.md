# git-clone

A minimal version-control command-line tool written in Rust.  
Simulates basic Git-like commands such as `commit`, `log`, and `reset`.

## 🔧 Installation

1. Clone this repository:
   ```bash
   git clone https://github.com/yourusername/git-clone
   cd git-clone
   ```

2. Build the project in release mode:
   ```bash
   cargo build --release
   ```

3. Copy the executable to a directory in your `$PATH` for global access:
   ```bash
   sudo cp ./target/release/git-clone /usr/local/bin/mygit
   ```

## 🚀 Usage

```bash
mygit commit "Initial commit"
mygit log
mygit reset <hash>
```
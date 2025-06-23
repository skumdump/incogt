# Incogt

**Incogt** is a Rust-based command-line tool that launches an incognito shell session with enhanced privacy and security features. It disables shell history logging, locks memory to prevent swapping, disables core dumps, and verifies terminal (TTY) security to minimize the risk of sensitive data exposure.

## Features

- **Ephemeral Shell History**: Maintains in-session command history for supported shells (`bash`, `zsh`, `fish`, `sh`, `dash`) without writing to disk, preventing data leakage during and after the session.- 
- **Memory Protection**: Locks process memory (Unix only) to prevent sensitive data from being swapped to disk.
- **Core Dump Prevention**: Disables core dumps (Unix only) to avoid crash memory dumps containing sensitive information.
- **TTY Security Checks**: Verifies the terminal environment to detect potential logging mechanisms (e.g., `script`, `tmux`).
- **Custom Prompt**: Provides a consistent `(incogt)` prompt across all supported shells for easy identification.
- **Cross-Platform**: Full support for Unix systems; limited support for non-Unix platforms (e.g., Windows) with fallback warnings.

## Installation

### Prerequisites
- **Rust**: Install Rust using [rustup](https://rustup.rs/).
- **Unix System**: Full functionality requires a Unix-like system (Linux, macOS, BSD). Non-Unix systems have limited support.
- **Supported Shells**: One of `bash`, `zsh`, `fish`, `sh`, or `dash` must be installed.

### Building from Source

   ```bash
   git clone https://github.com/<your-username>/incogt.git
   cd incogt
   
   rustc --version
   cargo --version
   
   #If Rust is not installed, install it via rustup. Update Rust if needed:
   
   rustup update
   cargo build

   #Test the Build: Verify the build by running the binary directly:
   ./target/release/incogt
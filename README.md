# A Warp GUI App System Tray

A Tauri System Tray based on warp-cli for linux based systems with Rust. 

## Installation

Install cargo, rustup, warp-cli and register with warp-cli register.

Then install from source:

    $ git clone https://github.com/Abhay-N-J/warp-sys-tray
    $ cd warp-sys-tray
    $ ./src-tauri/target/release/warp-sys-tray
    or
    $ git clone https://github.com/Abhay-N-J/warp-sys-tray
    $ cd warp-sys-tray
    $ cargo install tauri-cli
    $ cargo tauri build

Note: warp-gui-app from https://github.com/Abhay-N-J/warp-gui-app will also be installed as a dependancy

## Uninstallation

    $ rm -rf warp-sys-tray
    $ cargo uninstall warp-gui-app

Note: Default mode is hidden.

### Todo
    Desktop file
    Docs

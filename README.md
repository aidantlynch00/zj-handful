# zj-pnp
A [zellij](https://github.com/zellij-org/zellij) plugin for quickly picking and placing your panes.

TODO: GIF showing off features

## Features
- **Pick** several panes across tabs
- **Place** panes onto an existing tab
- **Chuck** panes onto a new tab
    - Note: The new tab will be created with the default tab layout, so picked panes will be placed alongside those contents.

## Usage
The easiest way to get started with zj-pnp is to add the following to your zellij keybinds (I prefer mine in pane mode):
```kdl
pane {
    bind "c" {
        MessagePlugin "https://github.com/aidantlynch00/zj-pnp/releases/latest/download/zj-pnp.wasm" {
            payload "pick";
        }
    }

    bind "v" {
        MessagePlugin "https://github.com/aidantlynch00/zj-pnp/releases/latest/download/zj-pnp.wasm" {
            payload "place";
        }
    }

    bind "V" {
        MessagePlugin "https://github.com/aidantlynch00/zj-pnp/releases/latest/download/zj-pnp.wasm" {
            payload "chuck";
        }
    }
}
```

## Installation
If you prefer to install zj-pnp, download the [latest prebuilt binary](https://github.com/aidantlynch00/zj-pnp/releases/latest/download/zj-pnp.wasm) or build from source using the following commands:
```sh
git clone https://github.com/aidantlynch00/zj-pnp.git
cd zj-pnp
cargo build --release
cp target/wasm32-wasip1/release/zj-pnp.wasm /path/to/zj-pnp.wasm
```

When you have the binary where you want it, you can refer to the file in your keybinds:
```kdl
MessagePlugin "file:/path/to/zj-pnp.wasm" {
    payload "pick"
}
```

Alternatively, you can add a plugin alias to your zellij config and refer to that in your keybinds:
```kdl
plugins {
    zj-pnp location="file:/path/to/zj-pnp.wasm" // or GitHub!
}
```

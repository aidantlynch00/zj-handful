# zj-handful
A [zellij](https://github.com/zellij-org/zellij) plugin for quickly picking and placing a handful of panes.

TODO: GIF showing off features

## Features
- **Pick** several panes across tabs
- **Place** panes onto an existing tab
- **Toss** panes into the floating layer
- **Spike** panes into the embedded layer
- **Chuck** panes onto a new tab
    - Note: The new tab will be created with the default tab layout, so picked panes will be placed alongside those contents.

## Usage
The easiest way to get started with zj-handful is to add the following to your zellij keybinds (I prefer mine in pane mode):
```kdl
pane {
    bind "c" {
        MessagePlugin "https://github.com/aidantlynch00/zj-handful/releases/latest/download/zj-handful.wasm" {
            payload "pick";
        }
    }

    bind "v" {
        MessagePlugin "https://github.com/aidantlynch00/zj-handful/releases/latest/download/zj-handful.wasm" {
            payload "place";
        }
    }

    bind "V" {
        MessagePlugin "https://github.com/aidantlynch00/zj-handful/releases/latest/download/zj-handful.wasm" {
            payload "chuck";
        }
    }

    bind "W" {
        MessagePlugin "https://github.com/aidantlynch00/zj-handful/releases/latest/download/zj-handful.wasm" {
            payload "toss";
        }
    }

    bind "E" {
        MessagePlugin "https://github.com/aidantlynch00/zj-handful/releases/latest/download/zj-handful.wasm" {
            payload "spike";
        }
    }
}
```

You can bind whatever set of commands you would like, only the pick command is required for picking panes.

## Installation
If you prefer to install zj-handful, download the [latest prebuilt binary](https://github.com/aidantlynch00/zj-handful/releases/latest/download/zj-handful.wasm) or build from source using the following commands:
```sh
git clone https://github.com/aidantlynch00/zj-handful.git
cd zj-handful
cargo build --release
cp target/wasm32-wasip1/release/zj-handful.wasm /path/to/zj-handful.wasm
```

When you have the binary where you want it, you can refer to the file in your keybinds:
```kdl
MessagePlugin "file:/path/to/zj-handful.wasm" {
    payload "pick"
}
```

Alternatively, you can add a plugin alias to your zellij config and refer to that in your keybinds:
```kdl
plugins {
    zj-handful location="file:/path/to/zj-handful.wasm" // or GitHub!
}
```

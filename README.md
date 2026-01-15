# zj-handful
A [zellij](https://github.com/zellij-org/zellij) plugin for quickly picking and placing a handful of panes.

TODO: GIF showing off features

## Features
- **Pick** several panes across tabs
    - Choose to hide the pane while picked
- **Place** panes onto an existing tab
- **Throw** panes onto a new tab
    - Note: The new tab will be created with the default tab layout, so picked panes will be placed alongside those contents.
- **Toss** panes into the floating layer
- **Spike** panes into the embedded layer
- **Chuck** panes into the void, never to return

## Usage
The easiest way to get started with zj-handful is to add the following keybinds to your zellij config. I personally put mine under pane mode, with all but pick returning me to normal mode. I also recommend setting up a plugin alias.
```kdl
plugins {
    zj-handful location="https://github.com/aidantlynch00/zj-handful/releases/latest/download/zj-handful.wasm"
}

pane {
    bind "c" {
        MessagePlugin "zj-handful" { payload "pick"; }
    }

    bind "C" {
        MessagePlugin "zj-handful" { payload "pick-hide"; }
    }

    bind "v" {
        MessagePlugin "zj-handful" { payload "place"; }
        SwitchMode "Normal"
    }

    bind "V" {
        MessagePlugin "zj-handful" { payload "throw"; }
        SwitchMode "Normal"
    }

    bind "W" {
        MessagePlugin "zj-handful" { payload "toss"; }
        SwitchMode "Normal"
    }

    bind "E" {
        MessagePlugin "zj-handful" { payload "spike"; }
        SwitchMode "Normal"
    }

    bind "D" {
        MessagePlugin "zj-handful" { payload "chuck"; }
        SwitchMode "Normal"
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

When you have the binary where you want it, you can refer to the file in your keybinds or alias:
```kdl
plugins {
    zj-handful "file:/path/to/zj-handful.wasm"
}
```

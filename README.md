# 📋 nu_plugin_clipboard (deprecated)

Since 0.111.0 it will be included in the [nushell](https://www.nushell.sh/) itself and the plugin wont receive further updates.
There are some changes in the commands:

`clipboard copy` -> `clip copy` # does not passes the input to the output by default anymore.
`clipboard paste` -> `clip paste`

Importing `std/clip` shadows this implementation.

A [nushell](https://www.nushell.sh/) plugin for interacting with the clipboard, allowing you to copy/paste text, objects, and tables.

## ✨ Features

- **`clipboard copy`**: Copies input text to the clipboard.
  - **Daemon Mode** (Linux only): Since version **0.105.2**, plugin will try to detect display server using env variables.
    If there is any issue with copy/paste command
    This config will override this behavior. If you need to override this, please file an issue:

    ```bash
    $env.config.plugins.clipboard.NO_DAEMON = true # or false
    ```

  - **Silent Copy**: Added support for silent copying (requested in [#23](https://github.com/fmotalleb/nu_plugin_clipboard/issues/23)).  
    You can disable echoing of copied content globally with:

    ```bash
    $env.config.plugins.clipboard.SILENT_COPY = true
    ```

    or per invocation using `--silent (-s)` flag

  - To make these settings permanent, add it to your env vars using `config env`.

- **`clipboard paste`**: Retrieves the current clipboard content.

## ⚠️ Important (Common issue workaround)

If you encounter the error:
`Error: × Clipboard Error: The clipboard contents were not available in the requested format...` on Linux:

- For users running **Wayland** without the `nupm` installer, enable the `use-wayland` feature as described in [#21](https://github.com/FMotalleb/nu_plugin_clipboard/issues/21).
- Alternatively, try disabling **daemon mode**, as explained in [#20](https://github.com/FMotalleb/nu_plugin_clipboard/issues/20).

Note: These issues are already fixed internally. If you still need to rely on these workarounds, please open a new issue.

## 📌 Usage Examples

### Copying a string

```bash
"test value" | clipboard copy 
```

### Using clipboard content

```bash
clipboard paste
```

### Copying tables and objects

- Tables and objects are internally converted to **JSON**.
- When pasting, `clipboard paste` tries to parse JSON into a table or object.
- If parsing fails, the content is returned as a string.

```bash
$env | clipboard copy
clipboard paste

ps | clipboard copy
clipboard paste
```

## 🔧 Installation

### 🚀 Recommended: Using [nupm](https://github.com/nushell/nupm)

This method automatically handles dependencies and features:

```bash
git clone https://github.com/FMotalleb/nu_plugin_clipboard.git
nupm install --path nu_plugin_clipboard -f
```

### ⚙️ Supported Features

- **`use-wayland`**: Prioritizes the Wayland API, but falls back to X11 if needed.

### 🛠️ Manual Compilation

```bash
git clone https://github.com/FMotalleb/nu_plugin_clipboard.git
cd nu_plugin_clipboard
cargo build -r
plugin add target/release/nu_plugin_clipboard
```

### 📦 Install via Cargo (using git)

```bash
cargo install --git https://github.com/FMotalleb/nu_plugin_clipboard.git
plugin add ~/.cargo/bin/nu_plugin_clipboard
```

### 📦 Install via Cargo (crates.io) _Not Recommended_

- Since I live in Iran and crates.io won't let me update my packages like a normal person, most of the time crates.io is outdated.

```bash
cargo install nu_plugin_clipboard
plugin add ~/.cargo/bin/nu_plugin_clipboard
```

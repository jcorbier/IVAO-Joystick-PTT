# IVAO Joystick PTT

This plugin creates a custom command in X-Plane 12 that you can map to any joystick button. When triggered, it simulates a keyboard key press (default F12) to activate the Push-to-Talk (PTT) on the IVAO Pilot Client (Altitude).

It also includes a compatibility mode for VATSIM (xPilot), allowing it to trigger the xPilot PTT command directly instead of simulating a key press.

## Features

- **Joystick Integration:** Map any joystick button to the "IVAO Push-to-Talk" command in X-Plane settings.
- **Configurable Key:** Change the simulated key if you don't use F12.
- **VATSIM Support:** Optional mode to integrate native xPilot PTT support.
- **Cross-Platform:** Written in Rust, designed for macOS (and easily portable to Windows/Linux).

## Installation

1.  **Download** the latest release.
2.  **Extract** the `ivao-joystick-ptt` folder into your X-Plane plugins directory:
    - Path: `X-Plane 12/Resources/plugins/`
    - You should end up with: `X-Plane 12/Resources/plugins/ivao-joystick-ptt/mac_x64/ivao_joystick_ptt.xpl`

### ⚠️ macOS Notarization Warning

This plugin is **not notarized** by Apple. This means macOS Gatekeeper will block it from running by default, showing a message that it "cannot be opened because the developer cannot be verified" or that it is "damaged".

To allow the plugin to run, you must remove the quarantine attribute.

1.  Open **Terminal**.
2.  Run the following command (replace the path with the actual path to your X-Plane folder):

    ```bash
    xattr -d com.apple.quarantine "path/to/X-Plane 12/Resources/plugins/ivao-joystick-ptt/mac_x64/ivao_joystick_ptt.xpl"
    ```

3.  Restart X-Plane.

Alternatively, you can try going to **System Settings > Privacy & Security** immediately after the error appears and clicking "Open Anyway".

## Configuration

The plugin creates a configuration file automatically the first time it runs.

- **File Path:** `X-Plane 12/Output/preferences/ivao_ptt.toml`

To configure the plugin, open this file with any text editor.

### Options

```toml
# The key to simulate when the joystick button is pressed.
# Supported special keys:
# F1-F12, Control, Shift, Alt/Option, Meta/Command/Windows, 
# Space, Enter/Return, Tab, Backspace, Escape/Esc,
# or any single character (e.g. "a", "1", ".").
# Default: "F12"
key = "F12"

# Enable VATSIM (xPilot) compatibility mode.
# If true, the plugin will try to find the "xpilot/ptt" command and trigger it
# directly. If not found, it falls back to the key press above.
# Default: false
vatsim_compat = false
```

**Note:** You must restart specific plugins or X-Plane for configuration changes to take effect.

## Usage

1.  Start X-Plane 12.
2.  Go to **Settings** > **Joystick**.
3.  Select the joystick button you want to use for PTT.
4.  Search for the command: `IVAO Push-to-Talk` (or search for "ivao").
5.  Assign the button to this command.
6.  Click **Done**.

Now, when you press that button, the plugin will simulate the `F12` key (or your configured key), activating the radio in the IVAO Altitude client.

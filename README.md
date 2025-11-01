# QMK Layout Helper

QMK Layout Helper is a tool to display the layout of a QMK keyboard on your screen as an overlay. It can be useful when learning a new keyboard layout with many layers whose key functions are not printed on the keycaps. It is designed to update automatically whenever the currently active layer changes. The keymap is read from the QMK firmware using the VIA protocol.

## Setup

Unfortunately stock QMK firmware does not report layer changes to the operating system. Therefore, a small modification to the firmware is necessary, that sends layer change notifications over the RAW HID interface.

- Add the following to your `rules.mk` file to enable VIA and RAW HID support:
  ```
  VIA_ENABLE = yes
  RAW_ENABLE = yes
  ```
- Add the following to your `config.h` file to enable active layer reporting:
  ```c
  #include "raw_hid.h"
  #include "usb_descriptor.h"
  
  // Notify VIA of layer changes
  layer_state_t layer_state_set_user(layer_state_t state) {
      uint8_t layer = get_highest_layer(state);
      uint8_t data[RAW_EPSIZE] = { 0 };
      data[0] = 0x01;
      data[1] = layer;
      raw_hid_send(data, RAW_EPSIZE);
      return state;
  }
  ```
- Compile and flash the modified firmware to your keyboard
  ```sh
  qmk compile -kb <your_keyboard> -km <your_keymap>
  ```
- Obtain the keyboard information json file:
  ```sh
  qmk info -kb <your_keyboard> -m -f json > keyboard_info.json
  ```
  This is the input file for the QMK Layout Helper containing the keyboard layout information required for rendering the overlay.

## Usage

The only input required for QMK Layout Helper is the keyboard information json file obtained in the previous step. Make sure to pass the correct layout for your keyboard if it differs from the default `LAYOUT`.

```sh
$ ./qmk-layout-helper --help
QMK layout viewer

Usage: qmk-layout-helper [OPTIONS] <KEYBOARD_CONFIG>

Arguments:
  <KEYBOARD_CONFIG>  Path to the keyboard information JSON file

Options:
  -l, --layout <LAYOUT_NAME>  Name of the keyboard layout to display [default: LAYOUT]
      --size <SIZE>           Size of the overlay window in the format 'width,height' [default: 700,240]
      --position <POSITION>   Position of the overlay window [default: bottom-right] [possible values: top-left, top-right, bottom-left, bottom-right, bottom, top]
      --timeout <TIMEOUT>     Timeout for the overlay in milliseconds [default: 5000]
      --margin <MARGIN>       Margin around the overlay window [default: 10]
  -h, --help                  Print help
  -V, --version               Print version
```

Example usage:

```sh
./qmk-layout-helper keyboard_info.json --layout LAYOUT_60_ansi --position top-right
```
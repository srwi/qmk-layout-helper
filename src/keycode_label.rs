use qmk_via_api::keycodes::Keycode;

pub struct KeycodeLabel {
    pub long: Option<&'static str>,
    pub short: Option<&'static str>,
}

pub fn get_keycode_label(keycode: Keycode) -> KeycodeLabel {
    match keycode {
        Keycode::KC_NO => KeycodeLabel {
            long: None,
            short: None,
        },
        Keycode::KC_TRANSPARENT => KeycodeLabel {
            long: Some("▽"),
            short: None,
        },
        Keycode::KC_A => KeycodeLabel {
            long: Some("A"),
            short: None,
        },
        Keycode::KC_B => KeycodeLabel {
            long: Some("B"),
            short: None,
        },
        Keycode::KC_C => KeycodeLabel {
            long: Some("C"),
            short: None,
        },
        Keycode::KC_D => KeycodeLabel {
            long: Some("D"),
            short: None,
        },
        Keycode::KC_E => KeycodeLabel {
            long: Some("E"),
            short: None,
        },
        Keycode::KC_F => KeycodeLabel {
            long: Some("F"),
            short: None,
        },
        Keycode::KC_G => KeycodeLabel {
            long: Some("G"),
            short: None,
        },
        Keycode::KC_H => KeycodeLabel {
            long: Some("H"),
            short: None,
        },
        Keycode::KC_I => KeycodeLabel {
            long: Some("I"),
            short: None,
        },
        Keycode::KC_J => KeycodeLabel {
            long: Some("J"),
            short: None,
        },
        Keycode::KC_K => KeycodeLabel {
            long: Some("K"),
            short: None,
        },
        Keycode::KC_L => KeycodeLabel {
            long: Some("L"),
            short: None,
        },
        Keycode::KC_M => KeycodeLabel {
            long: Some("M"),
            short: None,
        },
        Keycode::KC_N => KeycodeLabel {
            long: Some("N"),
            short: None,
        },
        Keycode::KC_O => KeycodeLabel {
            long: Some("O"),
            short: None,
        },
        Keycode::KC_P => KeycodeLabel {
            long: Some("P"),
            short: None,
        },
        Keycode::KC_Q => KeycodeLabel {
            long: Some("Q"),
            short: None,
        },
        Keycode::KC_R => KeycodeLabel {
            long: Some("R"),
            short: None,
        },
        Keycode::KC_S => KeycodeLabel {
            long: Some("S"),
            short: None,
        },
        Keycode::KC_T => KeycodeLabel {
            long: Some("T"),
            short: None,
        },
        Keycode::KC_U => KeycodeLabel {
            long: Some("U"),
            short: None,
        },
        Keycode::KC_V => KeycodeLabel {
            long: Some("V"),
            short: None,
        },
        Keycode::KC_W => KeycodeLabel {
            long: Some("W"),
            short: None,
        },
        Keycode::KC_X => KeycodeLabel {
            long: Some("X"),
            short: None,
        },
        Keycode::KC_Y => KeycodeLabel {
            long: Some("Y"),
            short: None,
        },
        Keycode::KC_Z => KeycodeLabel {
            long: Some("Z"),
            short: None,
        },
        Keycode::KC_1 => KeycodeLabel {
            long: Some("!\n1"),
            short: None,
        },
        Keycode::KC_2 => KeycodeLabel {
            long: Some("@\n2"),
            short: None,
        },
        Keycode::KC_3 => KeycodeLabel {
            long: Some("#\n3"),
            short: None,
        },
        Keycode::KC_4 => KeycodeLabel {
            long: Some("$\n4"),
            short: None,
        },
        Keycode::KC_5 => KeycodeLabel {
            long: Some("%\n5"),
            short: None,
        },
        Keycode::KC_6 => KeycodeLabel {
            long: Some("^\n6"),
            short: None,
        },
        Keycode::KC_7 => KeycodeLabel {
            long: Some("&\n7"),
            short: None,
        },
        Keycode::KC_8 => KeycodeLabel {
            long: Some("*\n8"),
            short: None,
        },
        Keycode::KC_9 => KeycodeLabel {
            long: Some("(\n9"),
            short: None,
        },
        Keycode::KC_0 => KeycodeLabel {
            long: Some(")\n0"),
            short: None,
        },
        Keycode::KC_ENTER => KeycodeLabel {
            long: Some("Enter"),
            short: Some("Ent"),
        },
        Keycode::KC_ESCAPE => KeycodeLabel {
            long: Some("Esc"),
            short: None,
        },
        Keycode::KC_BACKSPACE => KeycodeLabel {
            long: Some("Backspace"),
            short: Some("Bksp"),
        },
        Keycode::KC_TAB => KeycodeLabel {
            long: Some("Tab"),
            short: None,
        },
        Keycode::KC_SPACE => KeycodeLabel {
            long: Some("Space"),
            short: Some("Spc"),
        },
        Keycode::KC_MINUS => KeycodeLabel {
            long: Some("_\n-"),
            short: None,
        },
        Keycode::KC_EQUAL => KeycodeLabel {
            long: Some("+\n="),
            short: None,
        },
        Keycode::KC_LEFT_BRACKET => KeycodeLabel {
            long: Some("{\n["),
            short: None,
        },
        Keycode::KC_RIGHT_BRACKET => KeycodeLabel {
            long: Some("}\n]"),
            short: None,
        },
        Keycode::KC_BACKSLASH => KeycodeLabel {
            long: Some("|\n\\"),
            short: None,
        },
        Keycode::KC_NONUS_HASH => KeycodeLabel {
            long: Some("NUHS"),
            short: None,
        },
        Keycode::KC_SEMICOLON => KeycodeLabel {
            long: Some(":\n;"),
            short: None,
        },
        Keycode::KC_QUOTE => KeycodeLabel {
            long: Some("\"\n\'"),
            short: None,
        },
        Keycode::KC_GRAVE => KeycodeLabel {
            long: Some("~\n`"),
            short: None,
        },
        Keycode::KC_COMMA => KeycodeLabel {
            long: Some("<\n,"),
            short: None,
        },
        Keycode::KC_DOT => KeycodeLabel {
            long: Some(">\n."),
            short: None,
        },
        Keycode::KC_SLASH => KeycodeLabel {
            long: Some("?\n/"),
            short: None,
        },
        Keycode::KC_CAPS_LOCK => KeycodeLabel {
            long: Some("Caps Lock"),
            short: Some("Caps"),
        },
        Keycode::KC_F1 => KeycodeLabel {
            long: Some("F1"),
            short: None,
        },
        Keycode::KC_F2 => KeycodeLabel {
            long: Some("F2"),
            short: None,
        },
        Keycode::KC_F3 => KeycodeLabel {
            long: Some("F3"),
            short: None,
        },
        Keycode::KC_F4 => KeycodeLabel {
            long: Some("F4"),
            short: None,
        },
        Keycode::KC_F5 => KeycodeLabel {
            long: Some("F5"),
            short: None,
        },
        Keycode::KC_F6 => KeycodeLabel {
            long: Some("F6"),
            short: None,
        },
        Keycode::KC_F7 => KeycodeLabel {
            long: Some("F7"),
            short: None,
        },
        Keycode::KC_F8 => KeycodeLabel {
            long: Some("F8"),
            short: None,
        },
        Keycode::KC_F9 => KeycodeLabel {
            long: Some("F9"),
            short: None,
        },
        Keycode::KC_F10 => KeycodeLabel {
            long: Some("F10"),
            short: None,
        },
        Keycode::KC_F11 => KeycodeLabel {
            long: Some("F11"),
            short: None,
        },
        Keycode::KC_F12 => KeycodeLabel {
            long: Some("F12"),
            short: None,
        },
        Keycode::KC_PRINT_SCREEN => KeycodeLabel {
            long: Some("Print Screen"),
            short: Some("PrtSc"),
        },
        Keycode::KC_SCROLL_LOCK => KeycodeLabel {
            long: Some("Scroll Lock"),
            short: Some("ScrLk"),
        },
        Keycode::KC_PAUSE => KeycodeLabel {
            long: Some("Pause"),
            short: Some("Paus"),
        },
        Keycode::KC_INSERT => KeycodeLabel {
            long: Some("Insert"),
            short: Some("Ins"),
        },
        Keycode::KC_HOME => KeycodeLabel {
            long: Some("Home"),
            short: None,
        },
        Keycode::KC_PAGE_UP => KeycodeLabel {
            long: Some("Page Up"),
            short: Some("PgUp"),
        },
        Keycode::KC_DELETE => KeycodeLabel {
            long: Some("Del"),
            short: None,
        },
        Keycode::KC_END => KeycodeLabel {
            long: Some("End"),
            short: None,
        },
        Keycode::KC_PAGE_DOWN => KeycodeLabel {
            long: Some("Page Down"),
            short: Some("PgDn"),
        },
        Keycode::KC_RIGHT => KeycodeLabel {
            long: Some("Right"),
            short: Some("→"),
        },
        Keycode::KC_LEFT => KeycodeLabel {
            long: Some("Left"),
            short: Some("←"),
        },
        Keycode::KC_DOWN => KeycodeLabel {
            long: Some("Down"),
            short: Some("↓"),
        },
        Keycode::KC_UP => KeycodeLabel {
            long: Some("Up"),
            short: Some("↑"),
        },
        Keycode::KC_NUM_LOCK => KeycodeLabel {
            long: Some("Num\nLock"),
            short: Some("NumLk"),
        },
        Keycode::KC_KP_SLASH => KeycodeLabel {
            long: Some("÷"),
            short: None,
        },
        Keycode::KC_KP_ASTERISK => KeycodeLabel {
            long: Some("×"),
            short: None,
        },
        Keycode::KC_KP_MINUS => KeycodeLabel {
            long: Some("-"),
            short: None,
        },
        Keycode::KC_KP_PLUS => KeycodeLabel {
            long: Some("+"),
            short: None,
        },
        Keycode::KC_KP_ENTER => KeycodeLabel {
            long: Some("Num\nEnter"),
            short: Some("N.Ent"),
        },
        Keycode::KC_KP_1 => KeycodeLabel {
            long: Some("1"),
            short: None,
        },
        Keycode::KC_KP_2 => KeycodeLabel {
            long: Some("2"),
            short: None,
        },
        Keycode::KC_KP_3 => KeycodeLabel {
            long: Some("3"),
            short: None,
        },
        Keycode::KC_KP_4 => KeycodeLabel {
            long: Some("4"),
            short: None,
        },
        Keycode::KC_KP_5 => KeycodeLabel {
            long: Some("5"),
            short: None,
        },
        Keycode::KC_KP_6 => KeycodeLabel {
            long: Some("6"),
            short: None,
        },
        Keycode::KC_KP_7 => KeycodeLabel {
            long: Some("7"),
            short: None,
        },
        Keycode::KC_KP_8 => KeycodeLabel {
            long: Some("8"),
            short: None,
        },
        Keycode::KC_KP_9 => KeycodeLabel {
            long: Some("9"),
            short: None,
        },
        Keycode::KC_KP_0 => KeycodeLabel {
            long: Some("0"),
            short: None,
        },
        Keycode::KC_KP_DOT => KeycodeLabel {
            long: Some("."),
            short: None,
        },
        Keycode::KC_NONUS_BACKSLASH => KeycodeLabel {
            long: Some("NUBS"),
            short: None,
        },
        Keycode::KC_APPLICATION => KeycodeLabel {
            long: Some("Menu"),
            short: None,
        },
        Keycode::KC_KB_POWER => KeycodeLabel {
            long: Some("Power"),
            short: Some("Pwr"),
        },
        Keycode::KC_KP_EQUAL => KeycodeLabel {
            long: Some("="),
            short: None,
        },
        Keycode::KC_F13 => KeycodeLabel {
            long: Some("F13"),
            short: None,
        },
        Keycode::KC_F14 => KeycodeLabel {
            long: Some("F14"),
            short: None,
        },
        Keycode::KC_F15 => KeycodeLabel {
            long: Some("F15"),
            short: None,
        },
        Keycode::KC_F16 => KeycodeLabel {
            long: Some("F16"),
            short: None,
        },
        Keycode::KC_F17 => KeycodeLabel {
            long: Some("F17"),
            short: None,
        },
        Keycode::KC_F18 => KeycodeLabel {
            long: Some("F18"),
            short: None,
        },
        Keycode::KC_F19 => KeycodeLabel {
            long: Some("F19"),
            short: None,
        },
        Keycode::KC_F20 => KeycodeLabel {
            long: Some("F20"),
            short: None,
        },
        Keycode::KC_F21 => KeycodeLabel {
            long: Some("F21"),
            short: None,
        },
        Keycode::KC_F22 => KeycodeLabel {
            long: Some("F22"),
            short: None,
        },
        Keycode::KC_F23 => KeycodeLabel {
            long: Some("F23"),
            short: None,
        },
        Keycode::KC_F24 => KeycodeLabel {
            long: Some("F24"),
            short: None,
        },
        Keycode::KC_EXECUTE => KeycodeLabel {
            long: Some("Exec"),
            short: None,
        },
        Keycode::KC_HELP => KeycodeLabel {
            long: Some("Help"),
            short: None,
        },
        Keycode::KC_MENU => KeycodeLabel {
            long: Some("Menu"),
            short: None,
        },
        Keycode::KC_SELECT => KeycodeLabel {
            long: Some("Select"),
            short: None,
        },
        Keycode::KC_STOP => KeycodeLabel {
            long: Some("Stop"),
            short: None,
        },
        Keycode::KC_AGAIN => KeycodeLabel {
            long: Some("Again"),
            short: None,
        },
        Keycode::KC_UNDO => KeycodeLabel {
            long: Some("Undo"),
            short: None,
        },
        Keycode::KC_CUT => KeycodeLabel {
            long: Some("Cut"),
            short: None,
        },
        Keycode::KC_COPY => KeycodeLabel {
            long: Some("Copy"),
            short: None,
        },
        Keycode::KC_PASTE => KeycodeLabel {
            long: Some("Paste"),
            short: None,
        },
        Keycode::KC_FIND => KeycodeLabel {
            long: Some("Find"),
            short: None,
        },
        Keycode::KC_KB_MUTE => KeycodeLabel {
            long: Some("Mute"),
            short: None,
        },
        Keycode::KC_KB_VOLUME_UP => KeycodeLabel {
            long: Some("Vol +"),
            short: None,
        },
        Keycode::KC_KB_VOLUME_DOWN => KeycodeLabel {
            long: Some("Vol -"),
            short: None,
        },
        Keycode::KC_LOCKING_CAPS_LOCK => KeycodeLabel {
            long: Some("Locking Caps Lock"),
            short: Some("LCaps"),
        },
        Keycode::KC_LOCKING_NUM_LOCK => KeycodeLabel {
            long: Some("Locking Num Lock"),
            short: Some("LNum"),
        },
        Keycode::KC_LOCKING_SCROLL_LOCK => KeycodeLabel {
            long: Some("Locking Scroll Lock"),
            short: Some("LScrl"),
        },
        Keycode::KC_KP_COMMA => KeycodeLabel {
            long: Some(","),
            short: None,
        },
        Keycode::KC_KP_EQUAL_AS400 => KeycodeLabel {
            long: Some("="),
            short: None,
        },
        Keycode::KC_INTERNATIONAL_1 => KeycodeLabel {
            long: Some("Int1"),
            short: None,
        },
        Keycode::KC_INTERNATIONAL_2 => KeycodeLabel {
            long: Some("かな"),
            short: None,
        },
        Keycode::KC_INTERNATIONAL_3 => KeycodeLabel {
            long: Some("¥"),
            short: None,
        },
        Keycode::KC_INTERNATIONAL_4 => KeycodeLabel {
            long: Some("変換"),
            short: None,
        },
        Keycode::KC_INTERNATIONAL_5 => KeycodeLabel {
            long: Some("無変換"),
            short: None,
        },
        Keycode::KC_INTERNATIONAL_6 => KeycodeLabel {
            long: Some("Int6"),
            short: None,
        },
        Keycode::KC_INTERNATIONAL_7 => KeycodeLabel {
            long: Some("Int7"),
            short: None,
        },
        Keycode::KC_INTERNATIONAL_8 => KeycodeLabel {
            long: Some("Int8"),
            short: None,
        },
        Keycode::KC_INTERNATIONAL_9 => KeycodeLabel {
            long: Some("Int9"),
            short: None,
        },
        Keycode::KC_LANGUAGE_1 => KeycodeLabel {
            long: Some("한영"),
            short: None,
        },
        Keycode::KC_LANGUAGE_2 => KeycodeLabel {
            long: Some("漢字"),
            short: None,
        },
        Keycode::KC_LANGUAGE_3 => KeycodeLabel {
            long: Some("Lang3"),
            short: None,
        },
        Keycode::KC_LANGUAGE_4 => KeycodeLabel {
            long: Some("Lang4"),
            short: None,
        },
        Keycode::KC_LANGUAGE_5 => KeycodeLabel {
            long: Some("Lang5"),
            short: None,
        },
        Keycode::KC_LANGUAGE_6 => KeycodeLabel {
            long: Some("Lang6"),
            short: None,
        },
        Keycode::KC_LANGUAGE_7 => KeycodeLabel {
            long: Some("Lang7"),
            short: None,
        },
        Keycode::KC_LANGUAGE_8 => KeycodeLabel {
            long: Some("Lang8"),
            short: None,
        },
        Keycode::KC_LANGUAGE_9 => KeycodeLabel {
            long: Some("Lang9"),
            short: None,
        },
        Keycode::KC_ALTERNATE_ERASE => KeycodeLabel {
            long: Some("Alt Erase"),
            short: None,
        },
        Keycode::KC_SYSTEM_REQUEST => KeycodeLabel {
            long: Some("SysReq"),
            short: None,
        },
        Keycode::KC_CANCEL => KeycodeLabel {
            long: Some("Cancel"),
            short: None,
        },
        Keycode::KC_CLEAR => KeycodeLabel {
            long: Some("Clear"),
            short: None,
        },
        Keycode::KC_PRIOR => KeycodeLabel {
            long: Some("Prior"),
            short: None,
        },
        Keycode::KC_RETURN => KeycodeLabel {
            long: Some("Return"),
            short: None,
        },
        Keycode::KC_SEPARATOR => KeycodeLabel {
            long: Some("Separator"),
            short: None,
        },
        Keycode::KC_OUT => KeycodeLabel {
            long: Some("Out"),
            short: None,
        },
        Keycode::KC_OPER => KeycodeLabel {
            long: Some("Oper"),
            short: None,
        },
        Keycode::KC_CLEAR_AGAIN => KeycodeLabel {
            long: Some("Clear Again"),
            short: None,
        },
        Keycode::KC_CRSEL => KeycodeLabel {
            long: Some("CrSel"),
            short: None,
        },
        Keycode::KC_EXSEL => KeycodeLabel {
            long: Some("ExSel"),
            short: None,
        },
        Keycode::KC_SYSTEM_POWER => KeycodeLabel {
            long: Some("Power"),
            short: None,
        },
        Keycode::KC_SYSTEM_SLEEP => KeycodeLabel {
            long: Some("Sleep"),
            short: None,
        },
        Keycode::KC_SYSTEM_WAKE => KeycodeLabel {
            long: Some("Wake"),
            short: None,
        },
        Keycode::KC_AUDIO_MUTE => KeycodeLabel {
            long: Some("Mute"),
            short: None,
        },
        Keycode::KC_AUDIO_VOL_UP => KeycodeLabel {
            long: Some("Vol +"),
            short: None,
        },
        Keycode::KC_AUDIO_VOL_DOWN => KeycodeLabel {
            long: Some("Vol -"),
            short: None,
        },
        Keycode::KC_MEDIA_NEXT_TRACK => KeycodeLabel {
            long: Some("Next"),
            short: None,
        },
        Keycode::KC_MEDIA_PREV_TRACK => KeycodeLabel {
            long: Some("Previous"),
            short: Some("Prev"),
        },
        Keycode::KC_MEDIA_STOP => KeycodeLabel {
            long: Some("Media Stop"),
            short: Some("Stop"),
        },
        Keycode::KC_MEDIA_PLAY_PAUSE => KeycodeLabel {
            long: Some("Play"),
            short: None,
        },
        Keycode::KC_MEDIA_SELECT => KeycodeLabel {
            long: Some("Select"),
            short: Some("Sel"),
        },
        Keycode::KC_MEDIA_EJECT => KeycodeLabel {
            long: Some("Eject"),
            short: Some("Ejct"),
        },
        Keycode::KC_MAIL => KeycodeLabel {
            long: Some("Mail"),
            short: None,
        },
        Keycode::KC_CALCULATOR => KeycodeLabel {
            long: Some("Calc"),
            short: None,
        },
        Keycode::KC_MY_COMPUTER => KeycodeLabel {
            long: Some("My Comp"),
            short: None,
        },
        Keycode::KC_WWW_SEARCH => KeycodeLabel {
            long: Some("Search"),
            short: None,
        },
        Keycode::KC_WWW_HOME => KeycodeLabel {
            long: Some("Home"),
            short: None,
        },
        Keycode::KC_WWW_BACK => KeycodeLabel {
            long: Some("Back"),
            short: None,
        },
        Keycode::KC_WWW_FORWARD => KeycodeLabel {
            long: Some("Forward"),
            short: None,
        },
        Keycode::KC_WWW_STOP => KeycodeLabel {
            long: Some("Stop"),
            short: None,
        },
        Keycode::KC_WWW_REFRESH => KeycodeLabel {
            long: Some("Refresh"),
            short: None,
        },
        Keycode::KC_WWW_FAVORITES => KeycodeLabel {
            long: Some("Favorites"),
            short: None,
        },
        Keycode::KC_MEDIA_FAST_FORWARD => KeycodeLabel {
            long: Some("Fast Forward"),
            short: Some("FF"),
        },
        Keycode::KC_MEDIA_REWIND => KeycodeLabel {
            long: Some("Rewind"),
            short: Some("Rwd"),
        },
        Keycode::KC_BRIGHTNESS_UP => KeycodeLabel {
            long: Some("Screen +"),
            short: Some("Scr +"),
        },
        Keycode::KC_BRIGHTNESS_DOWN => KeycodeLabel {
            long: Some("Screen -"),
            short: Some("Scr -"),
        },
        Keycode::KC_CONTROL_PANEL => KeycodeLabel {
            long: Some("Control Panel"),
            short: Some("Ctrl P"),
        },
        Keycode::KC_ASSISTANT => KeycodeLabel {
            long: Some("Assistant"),
            short: Some("Asst"),
        },
        Keycode::KC_MISSION_CONTROL => KeycodeLabel {
            long: Some("Mission Control"),
            short: Some("MC"),
        },
        Keycode::KC_LAUNCHPAD => KeycodeLabel {
            long: Some("Launchpad"),
            short: Some("LP"),
        },
        Keycode::QK_MOUSE_CURSOR_UP => KeycodeLabel {
            long: Some("Mouse ↑"),
            short: None,
        },
        Keycode::QK_MOUSE_CURSOR_DOWN => KeycodeLabel {
            long: Some("Mouse ↓"),
            short: None,
        },
        Keycode::QK_MOUSE_CURSOR_LEFT => KeycodeLabel {
            long: Some("Mouse ←"),
            short: None,
        },
        Keycode::QK_MOUSE_CURSOR_RIGHT => KeycodeLabel {
            long: Some("Mouse →"),
            short: None,
        },
        Keycode::QK_MOUSE_BUTTON_1 => KeycodeLabel {
            long: Some("Mouse Btn1"),
            short: None,
        },
        Keycode::QK_MOUSE_BUTTON_2 => KeycodeLabel {
            long: Some("Mouse Btn2"),
            short: None,
        },
        Keycode::QK_MOUSE_BUTTON_3 => KeycodeLabel {
            long: Some("Mouse Btn3"),
            short: None,
        },
        Keycode::QK_MOUSE_BUTTON_4 => KeycodeLabel {
            long: Some("Mouse Btn4"),
            short: None,
        },
        Keycode::QK_MOUSE_BUTTON_5 => KeycodeLabel {
            long: Some("Mouse Btn5"),
            short: None,
        },
        Keycode::QK_MOUSE_BUTTON_6 => KeycodeLabel {
            long: Some("Mouse Btn6"),
            short: None,
        },
        Keycode::QK_MOUSE_BUTTON_7 => KeycodeLabel {
            long: Some("Mouse Btn7"),
            short: None,
        },
        Keycode::QK_MOUSE_BUTTON_8 => KeycodeLabel {
            long: Some("Mouse Btn8"),
            short: None,
        },
        Keycode::QK_MOUSE_WHEEL_UP => KeycodeLabel {
            long: Some("Mouse Wh ↑"),
            short: None,
        },
        Keycode::QK_MOUSE_WHEEL_DOWN => KeycodeLabel {
            long: Some("Mouse Wh ↓"),
            short: None,
        },
        Keycode::QK_MOUSE_WHEEL_LEFT => KeycodeLabel {
            long: Some("Mouse Wh ←"),
            short: None,
        },
        Keycode::QK_MOUSE_WHEEL_RIGHT => KeycodeLabel {
            long: Some("Mouse Wh →"),
            short: None,
        },
        Keycode::QK_MOUSE_ACCELERATION_0 => KeycodeLabel {
            long: Some("Mouse Acc0"),
            short: None,
        },
        Keycode::QK_MOUSE_ACCELERATION_1 => KeycodeLabel {
            long: Some("Mouse Acc1"),
            short: None,
        },
        Keycode::QK_MOUSE_ACCELERATION_2 => KeycodeLabel {
            long: Some("Mouse Acc2"),
            short: None,
        },
        Keycode::KC_LEFT_CTRL => KeycodeLabel {
            long: Some("Left Ctrl"),
            short: Some("LCtrl"),
        },
        Keycode::KC_LEFT_SHIFT => KeycodeLabel {
            long: Some("Left Shift"),
            short: Some("LShft"),
        },
        Keycode::KC_LEFT_ALT => KeycodeLabel {
            long: Some("Left Alt"),
            short: Some("LAlt"),
        },
        Keycode::KC_LEFT_GUI => KeycodeLabel {
            long: Some("Left Win"),
            short: Some("LWin"),
        },
        Keycode::KC_RIGHT_CTRL => KeycodeLabel {
            long: Some("Right Ctrl"),
            short: Some("RCtrl"),
        },
        Keycode::KC_RIGHT_SHIFT => KeycodeLabel {
            long: Some("Right Shift"),
            short: Some("RShft"),
        },
        Keycode::KC_RIGHT_ALT => KeycodeLabel {
            long: Some("Right Alt"),
            short: Some("RAlt"),
        },
        Keycode::KC_RIGHT_GUI => KeycodeLabel {
            long: Some("Right Win"),
            short: Some("RWin"),
        },
        Keycode::QK_SWAP_HANDS_TOGGLE => KeycodeLabel {
            long: Some("Swap Hands Toggle"),
            short: Some("SwpHT"),
        },
        Keycode::QK_SWAP_HANDS_TAP_TOGGLE => KeycodeLabel {
            long: Some("Swap Hands Tap Toggle"),
            short: Some("SwpTT"),
        },
        Keycode::QK_SWAP_HANDS_MOMENTARY_ON => KeycodeLabel {
            long: Some("Swap Hands On"),
            short: Some("SwpOn"),
        },
        Keycode::QK_SWAP_HANDS_MOMENTARY_OFF => KeycodeLabel {
            long: Some("Swap Hands Off"),
            short: Some("SwpOff"),
        },
        Keycode::QK_SWAP_HANDS_OFF => KeycodeLabel {
            long: Some("Swap Hands Off"),
            short: Some("SwpOff"),
        },
        Keycode::QK_SWAP_HANDS_ON => KeycodeLabel {
            long: Some("Swap Hands On"),
            short: Some("SwpOn"),
        },
        Keycode::QK_SWAP_HANDS_ONE_SHOT => KeycodeLabel {
            long: Some("Swap Hands One Shot"),
            short: Some("SwpOS"),
        },
        Keycode::QK_MAGIC_SWAP_CONTROL_CAPS_LOCK => KeycodeLabel {
            long: Some("Swap Ctrl Caps"),
            short: Some("SwCtCp"),
        },
        Keycode::QK_MAGIC_UNSWAP_CONTROL_CAPS_LOCK => KeycodeLabel {
            long: Some("Unswap Ctrl Caps"),
            short: Some("UnCtCp"),
        },
        Keycode::QK_MAGIC_TOGGLE_CONTROL_CAPS_LOCK => KeycodeLabel {
            long: Some("Toggle Ctrl Caps"),
            short: Some("TgCtCp"),
        },
        Keycode::QK_MAGIC_CAPS_LOCK_AS_CONTROL_OFF => KeycodeLabel {
            long: Some("Caps as Ctrl Off"),
            short: Some("CpCtOf"),
        },
        Keycode::QK_MAGIC_CAPS_LOCK_AS_CONTROL_ON => KeycodeLabel {
            long: Some("Caps as Ctrl On"),
            short: Some("CpCtOn"),
        },
        Keycode::QK_MAGIC_SWAP_LALT_LGUI => KeycodeLabel {
            long: Some("Swap LAlt LGui"),
            short: Some("SwAltG"),
        },
        Keycode::QK_MAGIC_UNSWAP_LALT_LGUI => KeycodeLabel {
            long: Some("Unswap LAlt LGui"),
            short: Some("UnAltG"),
        },
        Keycode::QK_MAGIC_SWAP_RALT_RGUI => KeycodeLabel {
            long: Some("Swap RAlt RGui"),
            short: Some("SwAltG"),
        },
        Keycode::QK_MAGIC_UNSWAP_RALT_RGUI => KeycodeLabel {
            long: Some("Unswap RAlt RGui"),
            short: Some("UnAltG"),
        },
        Keycode::QK_MAGIC_GUI_ON => KeycodeLabel {
            long: Some("GUI On"),
            short: Some("GuiOn"),
        },
        Keycode::QK_MAGIC_GUI_OFF => KeycodeLabel {
            long: Some("GUI Off"),
            short: Some("GuiOff"),
        },
        Keycode::QK_MAGIC_TOGGLE_GUI => KeycodeLabel {
            long: Some("Toggle GUI"),
            short: Some("TgGui"),
        },
        Keycode::QK_MAGIC_SWAP_GRAVE_ESC => KeycodeLabel {
            long: Some("Swap ` Esc"),
            short: Some("Sw`Esc"),
        },
        Keycode::QK_MAGIC_UNSWAP_GRAVE_ESC => KeycodeLabel {
            long: Some("Unswap ` Esc"),
            short: Some("Un`Esc"),
        },
        Keycode::QK_MAGIC_SWAP_BACKSLASH_BACKSPACE => KeycodeLabel {
            long: Some("Swap \\ Bksp"),
            short: Some("Sw\\Bk"),
        },
        Keycode::QK_MAGIC_UNSWAP_BACKSLASH_BACKSPACE => KeycodeLabel {
            long: Some("Unswap \\ Bksp"),
            short: Some("Un\\Bk"),
        },
        Keycode::QK_MAGIC_TOGGLE_BACKSLASH_BACKSPACE => KeycodeLabel {
            long: Some("Toggle \\ Bksp"),
            short: Some("Tg\\Bk"),
        },
        Keycode::QK_MAGIC_NKRO_ON => KeycodeLabel {
            long: Some("NKRO On"),
            short: Some("NKROOn"),
        },
        Keycode::QK_MAGIC_NKRO_OFF => KeycodeLabel {
            long: Some("NKRO Off"),
            short: Some("NKROOf"),
        },
        Keycode::QK_MAGIC_TOGGLE_NKRO => KeycodeLabel {
            long: Some("Toggle NKRO"),
            short: Some("NKRO"),
        },
        Keycode::QK_MAGIC_SWAP_ALT_GUI => KeycodeLabel {
            long: Some("Swap Alt GUI"),
            short: Some("SwAltG"),
        },
        Keycode::QK_MAGIC_UNSWAP_ALT_GUI => KeycodeLabel {
            long: Some("Unswap Alt GUI"),
            short: Some("UnAltG"),
        },
        Keycode::QK_MAGIC_TOGGLE_ALT_GUI => KeycodeLabel {
            long: Some("Toggle Alt GUI"),
            short: Some("TgAltG"),
        },
        Keycode::QK_MAGIC_SWAP_LCTL_LGUI => KeycodeLabel {
            long: Some("Swap LCtl LGui"),
            short: Some("SwCtlG"),
        },
        Keycode::QK_MAGIC_UNSWAP_LCTL_LGUI => KeycodeLabel {
            long: Some("Unswap LCtl LGui"),
            short: Some("UnCtlG"),
        },
        Keycode::QK_MAGIC_SWAP_RCTL_RGUI => KeycodeLabel {
            long: Some("Swap RCtl RGui"),
            short: Some("SwCtlG"),
        },
        Keycode::QK_MAGIC_UNSWAP_RCTL_RGUI => KeycodeLabel {
            long: Some("Unswap RCtl RGui"),
            short: Some("UnCtlG"),
        },
        Keycode::QK_MAGIC_SWAP_CTL_GUI => KeycodeLabel {
            long: Some("Swap Ctl GUI"),
            short: Some("SwCtlG"),
        },
        Keycode::QK_MAGIC_UNSWAP_CTL_GUI => KeycodeLabel {
            long: Some("Unswap Ctl GUI"),
            short: Some("UnCtlG"),
        },
        Keycode::QK_MAGIC_TOGGLE_CTL_GUI => KeycodeLabel {
            long: Some("Toggle Ctl GUI"),
            short: Some("TgCtlG"),
        },
        Keycode::QK_MAGIC_EE_HANDS_LEFT => KeycodeLabel {
            long: Some("EE Hands Left"),
            short: Some("EEHndL"),
        },
        Keycode::QK_MAGIC_EE_HANDS_RIGHT => KeycodeLabel {
            long: Some("EE Hands Right"),
            short: Some("EEHndR"),
        },
        Keycode::QK_MAGIC_SWAP_ESCAPE_CAPS_LOCK => KeycodeLabel {
            long: Some("Swap Esc Caps"),
            short: Some("SwEsCp"),
        },
        Keycode::QK_MAGIC_UNSWAP_ESCAPE_CAPS_LOCK => KeycodeLabel {
            long: Some("Unswap Esc Caps"),
            short: Some("UnEsCp"),
        },
        Keycode::QK_MAGIC_TOGGLE_ESCAPE_CAPS_LOCK => KeycodeLabel {
            long: Some("Toggle Esc Caps"),
            short: Some("TgEsCp"),
        },
        Keycode::QK_MIDI_ON => KeycodeLabel {
            long: Some("MIDI On"),
            short: Some("MDOn"),
        },
        Keycode::QK_MIDI_OFF => KeycodeLabel {
            long: Some("MIDI Off"),
            short: Some("MDOff"),
        },
        Keycode::QK_MIDI_TOGGLE => KeycodeLabel {
            long: Some("MIDI Toggle"),
            short: Some("MDTg"),
        },
        Keycode::QK_MIDI_NOTE_C_0 => KeycodeLabel {
            long: Some("MIDI C0"),
            short: Some("MDC0"),
        },
        Keycode::QK_MIDI_NOTE_C_SHARP_0 => KeycodeLabel {
            long: Some("MIDI C#0"),
            short: Some("MDC#0"),
        },
        Keycode::QK_MIDI_NOTE_D_0 => KeycodeLabel {
            long: Some("MIDI D0"),
            short: Some("MDD0"),
        },
        Keycode::QK_MIDI_NOTE_D_SHARP_0 => KeycodeLabel {
            long: Some("MIDI D#0"),
            short: Some("MDD#0"),
        },
        Keycode::QK_MIDI_NOTE_E_0 => KeycodeLabel {
            long: Some("MIDI E0"),
            short: Some("MDE0"),
        },
        Keycode::QK_MIDI_NOTE_F_0 => KeycodeLabel {
            long: Some("MIDI F0"),
            short: Some("MDF0"),
        },
        Keycode::QK_MIDI_NOTE_F_SHARP_0 => KeycodeLabel {
            long: Some("MIDI F#0"),
            short: Some("MDF#0"),
        },
        Keycode::QK_MIDI_NOTE_G_0 => KeycodeLabel {
            long: Some("MIDI G0"),
            short: Some("MDG0"),
        },
        Keycode::QK_MIDI_NOTE_G_SHARP_0 => KeycodeLabel {
            long: Some("MIDI G#0"),
            short: Some("MDG#0"),
        },
        Keycode::QK_MIDI_NOTE_A_0 => KeycodeLabel {
            long: Some("MIDI A0"),
            short: Some("MDA0"),
        },
        Keycode::QK_MIDI_NOTE_A_SHARP_0 => KeycodeLabel {
            long: Some("MIDI A#0"),
            short: Some("MDA#0"),
        },
        Keycode::QK_MIDI_NOTE_B_0 => KeycodeLabel {
            long: Some("MIDI B0"),
            short: Some("MDB0"),
        },
        Keycode::QK_MIDI_NOTE_C_1 => KeycodeLabel {
            long: Some("MIDI C1"),
            short: Some("MDC1"),
        },
        Keycode::QK_MIDI_NOTE_C_SHARP_1 => KeycodeLabel {
            long: Some("MIDI C#1"),
            short: Some("MDC#1"),
        },
        Keycode::QK_MIDI_NOTE_D_1 => KeycodeLabel {
            long: Some("MIDI D1"),
            short: Some("MDD1"),
        },
        Keycode::QK_MIDI_NOTE_D_SHARP_1 => KeycodeLabel {
            long: Some("MIDI D#1"),
            short: Some("MDD#1"),
        },
        Keycode::QK_MIDI_NOTE_E_1 => KeycodeLabel {
            long: Some("MIDI E1"),
            short: Some("MDE1"),
        },
        Keycode::QK_MIDI_NOTE_F_1 => KeycodeLabel {
            long: Some("MIDI F1"),
            short: Some("MDF1"),
        },
        Keycode::QK_MIDI_NOTE_F_SHARP_1 => KeycodeLabel {
            long: Some("MIDI F#1"),
            short: Some("MDF#1"),
        },
        Keycode::QK_MIDI_NOTE_G_1 => KeycodeLabel {
            long: Some("MIDI G1"),
            short: Some("MDG1"),
        },
        Keycode::QK_MIDI_NOTE_G_SHARP_1 => KeycodeLabel {
            long: Some("MIDI G#1"),
            short: Some("MDG#1"),
        },
        Keycode::QK_MIDI_NOTE_A_1 => KeycodeLabel {
            long: Some("MIDI A1"),
            short: Some("MDA1"),
        },
        Keycode::QK_MIDI_NOTE_A_SHARP_1 => KeycodeLabel {
            long: Some("MIDI A#1"),
            short: Some("MDA#1"),
        },
        Keycode::QK_MIDI_NOTE_B_1 => KeycodeLabel {
            long: Some("MIDI B1"),
            short: Some("MDB1"),
        },
        Keycode::QK_MIDI_NOTE_C_2 => KeycodeLabel {
            long: Some("MIDI C2"),
            short: Some("MDC2"),
        },
        Keycode::QK_MIDI_NOTE_C_SHARP_2 => KeycodeLabel {
            long: Some("MIDI C#2"),
            short: Some("MDC#2"),
        },
        Keycode::QK_MIDI_NOTE_D_2 => KeycodeLabel {
            long: Some("MIDI D2"),
            short: Some("MDD2"),
        },
        Keycode::QK_MIDI_NOTE_D_SHARP_2 => KeycodeLabel {
            long: Some("MIDI D#2"),
            short: Some("MDD#2"),
        },
        Keycode::QK_MIDI_NOTE_E_2 => KeycodeLabel {
            long: Some("MIDI E2"),
            short: Some("MDE2"),
        },
        Keycode::QK_MIDI_NOTE_F_2 => KeycodeLabel {
            long: Some("MIDI F2"),
            short: Some("MDF2"),
        },
        Keycode::QK_MIDI_NOTE_F_SHARP_2 => KeycodeLabel {
            long: Some("MIDI F#2"),
            short: Some("MDF#2"),
        },
        Keycode::QK_MIDI_NOTE_G_2 => KeycodeLabel {
            long: Some("MIDI G2"),
            short: Some("MDG2"),
        },
        Keycode::QK_MIDI_NOTE_G_SHARP_2 => KeycodeLabel {
            long: Some("MIDI G#2"),
            short: Some("MDG#2"),
        },
        Keycode::QK_MIDI_NOTE_A_2 => KeycodeLabel {
            long: Some("MIDI A2"),
            short: Some("MDA2"),
        },
        Keycode::QK_MIDI_NOTE_A_SHARP_2 => KeycodeLabel {
            long: Some("MIDI A#2"),
            short: Some("MDA#2"),
        },
        Keycode::QK_MIDI_NOTE_B_2 => KeycodeLabel {
            long: Some("MIDI B2"),
            short: Some("MDB2"),
        },
        Keycode::QK_MIDI_NOTE_C_3 => KeycodeLabel {
            long: Some("MIDI C3"),
            short: Some("MDC3"),
        },
        Keycode::QK_MIDI_NOTE_C_SHARP_3 => KeycodeLabel {
            long: Some("MIDI C#3"),
            short: Some("MDC#3"),
        },
        Keycode::QK_MIDI_NOTE_D_3 => KeycodeLabel {
            long: Some("MIDI D3"),
            short: Some("MDD3"),
        },
        Keycode::QK_MIDI_NOTE_D_SHARP_3 => KeycodeLabel {
            long: Some("MIDI D#3"),
            short: Some("MDD#3"),
        },
        Keycode::QK_MIDI_NOTE_E_3 => KeycodeLabel {
            long: Some("MIDI E3"),
            short: Some("MDE3"),
        },
        Keycode::QK_MIDI_NOTE_F_3 => KeycodeLabel {
            long: Some("MIDI F3"),
            short: Some("MDF3"),
        },
        Keycode::QK_MIDI_NOTE_F_SHARP_3 => KeycodeLabel {
            long: Some("MIDI F#3"),
            short: Some("MDF#3"),
        },
        Keycode::QK_MIDI_NOTE_G_3 => KeycodeLabel {
            long: Some("MIDI G3"),
            short: Some("MDG3"),
        },
        Keycode::QK_MIDI_NOTE_G_SHARP_3 => KeycodeLabel {
            long: Some("MIDI G#3"),
            short: Some("MDG#3"),
        },
        Keycode::QK_MIDI_NOTE_A_3 => KeycodeLabel {
            long: Some("MIDI A3"),
            short: Some("MDA3"),
        },
        Keycode::QK_MIDI_NOTE_A_SHARP_3 => KeycodeLabel {
            long: Some("MIDI A#3"),
            short: Some("MDA#3"),
        },
        Keycode::QK_MIDI_NOTE_B_3 => KeycodeLabel {
            long: Some("MIDI B3"),
            short: Some("MDB3"),
        },
        Keycode::QK_MIDI_NOTE_C_4 => KeycodeLabel {
            long: Some("MIDI C4"),
            short: Some("MDC4"),
        },
        Keycode::QK_MIDI_NOTE_C_SHARP_4 => KeycodeLabel {
            long: Some("MIDI C#4"),
            short: Some("MDC#4"),
        },
        Keycode::QK_MIDI_NOTE_D_4 => KeycodeLabel {
            long: Some("MIDI D4"),
            short: Some("MDD4"),
        },
        Keycode::QK_MIDI_NOTE_D_SHARP_4 => KeycodeLabel {
            long: Some("MIDI D#4"),
            short: Some("MDD#4"),
        },
        Keycode::QK_MIDI_NOTE_E_4 => KeycodeLabel {
            long: Some("MIDI E4"),
            short: Some("MDE4"),
        },
        Keycode::QK_MIDI_NOTE_F_4 => KeycodeLabel {
            long: Some("MIDI F4"),
            short: Some("MDF4"),
        },
        Keycode::QK_MIDI_NOTE_F_SHARP_4 => KeycodeLabel {
            long: Some("MIDI F#4"),
            short: Some("MDF#4"),
        },
        Keycode::QK_MIDI_NOTE_G_4 => KeycodeLabel {
            long: Some("MIDI G4"),
            short: Some("MDG4"),
        },
        Keycode::QK_MIDI_NOTE_G_SHARP_4 => KeycodeLabel {
            long: Some("MIDI G#4"),
            short: Some("MDG#4"),
        },
        Keycode::QK_MIDI_NOTE_A_4 => KeycodeLabel {
            long: Some("MIDI A4"),
            short: Some("MDA4"),
        },
        Keycode::QK_MIDI_NOTE_A_SHARP_4 => KeycodeLabel {
            long: Some("MIDI A#4"),
            short: Some("MDA#4"),
        },
        Keycode::QK_MIDI_NOTE_B_4 => KeycodeLabel {
            long: Some("MIDI B4"),
            short: Some("MDB4"),
        },
        Keycode::QK_MIDI_NOTE_C_5 => KeycodeLabel {
            long: Some("MIDI C5"),
            short: Some("MDC5"),
        },
        Keycode::QK_MIDI_NOTE_C_SHARP_5 => KeycodeLabel {
            long: Some("MIDI C#5"),
            short: Some("MDC#5"),
        },
        Keycode::QK_MIDI_NOTE_D_5 => KeycodeLabel {
            long: Some("MIDI D5"),
            short: Some("MDD5"),
        },
        Keycode::QK_MIDI_NOTE_D_SHARP_5 => KeycodeLabel {
            long: Some("MIDI D#5"),
            short: Some("MDD#5"),
        },
        Keycode::QK_MIDI_NOTE_E_5 => KeycodeLabel {
            long: Some("MIDI E5"),
            short: Some("MDE5"),
        },
        Keycode::QK_MIDI_NOTE_F_5 => KeycodeLabel {
            long: Some("MIDI F5"),
            short: Some("MDF5"),
        },
        Keycode::QK_MIDI_NOTE_F_SHARP_5 => KeycodeLabel {
            long: Some("MIDI F#5"),
            short: Some("MDF#5"),
        },
        Keycode::QK_MIDI_NOTE_G_5 => KeycodeLabel {
            long: Some("MIDI G5"),
            short: Some("MDG5"),
        },
        Keycode::QK_MIDI_NOTE_G_SHARP_5 => KeycodeLabel {
            long: Some("MIDI G#5"),
            short: Some("MDG#5"),
        },
        Keycode::QK_MIDI_NOTE_A_5 => KeycodeLabel {
            long: Some("MIDI A5"),
            short: Some("MDA5"),
        },
        Keycode::QK_MIDI_NOTE_A_SHARP_5 => KeycodeLabel {
            long: Some("MIDI A#5"),
            short: Some("MDA#5"),
        },
        Keycode::QK_MIDI_NOTE_B_5 => KeycodeLabel {
            long: Some("MIDI B5"),
            short: Some("MDB5"),
        },
        Keycode::QK_MIDI_OCTAVE_N2 => KeycodeLabel {
            long: Some("MIDI Oct -2"),
            short: Some("MDO-2"),
        },
        Keycode::QK_MIDI_OCTAVE_N1 => KeycodeLabel {
            long: Some("MIDI Oct -1"),
            short: Some("MDO-1"),
        },
        Keycode::QK_MIDI_OCTAVE_0 => KeycodeLabel {
            long: Some("MIDI Oct 0"),
            short: Some("MDO0"),
        },
        Keycode::QK_MIDI_OCTAVE_1 => KeycodeLabel {
            long: Some("MIDI Oct 1"),
            short: Some("MDO1"),
        },
        Keycode::QK_MIDI_OCTAVE_2 => KeycodeLabel {
            long: Some("MIDI Oct 2"),
            short: Some("MDO2"),
        },
        Keycode::QK_MIDI_OCTAVE_3 => KeycodeLabel {
            long: Some("MIDI Oct 3"),
            short: Some("MDO3"),
        },
        Keycode::QK_MIDI_OCTAVE_4 => KeycodeLabel {
            long: Some("MIDI Oct 4"),
            short: Some("MDO4"),
        },
        Keycode::QK_MIDI_OCTAVE_5 => KeycodeLabel {
            long: Some("MIDI Oct 5"),
            short: Some("MDO5"),
        },
        Keycode::QK_MIDI_OCTAVE_6 => KeycodeLabel {
            long: Some("MIDI Oct 6"),
            short: Some("MDO6"),
        },
        Keycode::QK_MIDI_OCTAVE_7 => KeycodeLabel {
            long: Some("MIDI Oct 7"),
            short: Some("MDO7"),
        },
        Keycode::QK_MIDI_OCTAVE_DOWN => KeycodeLabel {
            long: Some("MIDI Oct Down"),
            short: Some("MDO-"),
        },
        Keycode::QK_MIDI_OCTAVE_UP => KeycodeLabel {
            long: Some("MIDI Oct Up"),
            short: Some("MDO+"),
        },
        Keycode::QK_MIDI_TRANSPOSE_N6 => KeycodeLabel {
            long: Some("MIDI Trans -6"),
            short: Some("MDT-6"),
        },
        Keycode::QK_MIDI_TRANSPOSE_N5 => KeycodeLabel {
            long: Some("MIDI Trans -5"),
            short: Some("MDT-5"),
        },
        Keycode::QK_MIDI_TRANSPOSE_N4 => KeycodeLabel {
            long: Some("MIDI Trans -4"),
            short: Some("MDT-4"),
        },
        Keycode::QK_MIDI_TRANSPOSE_N3 => KeycodeLabel {
            long: Some("MIDI Trans -3"),
            short: Some("MDT-3"),
        },
        Keycode::QK_MIDI_TRANSPOSE_N2 => KeycodeLabel {
            long: Some("MIDI Trans -2"),
            short: Some("MDT-2"),
        },
        Keycode::QK_MIDI_TRANSPOSE_N1 => KeycodeLabel {
            long: Some("MIDI Trans -1"),
            short: Some("MDT-1"),
        },
        Keycode::QK_MIDI_TRANSPOSE_0 => KeycodeLabel {
            long: Some("MIDI Trans 0"),
            short: Some("MDT0"),
        },
        Keycode::QK_MIDI_TRANSPOSE_1 => KeycodeLabel {
            long: Some("MIDI Trans 1"),
            short: Some("MDT1"),
        },
        Keycode::QK_MIDI_TRANSPOSE_2 => KeycodeLabel {
            long: Some("MIDI Trans 2"),
            short: Some("MDT2"),
        },
        Keycode::QK_MIDI_TRANSPOSE_3 => KeycodeLabel {
            long: Some("MIDI Trans 3"),
            short: Some("MDT3"),
        },
        Keycode::QK_MIDI_TRANSPOSE_4 => KeycodeLabel {
            long: Some("MIDI Trans 4"),
            short: Some("MDT4"),
        },
        Keycode::QK_MIDI_TRANSPOSE_5 => KeycodeLabel {
            long: Some("MIDI Trans 5"),
            short: Some("MDT5"),
        },
        Keycode::QK_MIDI_TRANSPOSE_6 => KeycodeLabel {
            long: Some("MIDI Trans 6"),
            short: Some("MDT6"),
        },
        Keycode::QK_MIDI_TRANSPOSE_DOWN => KeycodeLabel {
            long: Some("MIDI Trans Down"),
            short: Some("MDT-"),
        },
        Keycode::QK_MIDI_TRANSPOSE_UP => KeycodeLabel {
            long: Some("MIDI Trans Up"),
            short: Some("MDT+"),
        },
        Keycode::QK_MIDI_VELOCITY_0 => KeycodeLabel {
            long: Some("MIDI Vel 0"),
            short: Some("MDV0"),
        },
        Keycode::QK_MIDI_VELOCITY_1 => KeycodeLabel {
            long: Some("MIDI Vel 1"),
            short: Some("MDV1"),
        },
        Keycode::QK_MIDI_VELOCITY_2 => KeycodeLabel {
            long: Some("MIDI Vel 2"),
            short: Some("MDV2"),
        },
        Keycode::QK_MIDI_VELOCITY_3 => KeycodeLabel {
            long: Some("MIDI Vel 3"),
            short: Some("MDV3"),
        },
        Keycode::QK_MIDI_VELOCITY_4 => KeycodeLabel {
            long: Some("MIDI Vel 4"),
            short: Some("MDV4"),
        },
        Keycode::QK_MIDI_VELOCITY_5 => KeycodeLabel {
            long: Some("MIDI Vel 5"),
            short: Some("MDV5"),
        },
        Keycode::QK_MIDI_VELOCITY_6 => KeycodeLabel {
            long: Some("MIDI Vel 6"),
            short: Some("MDV6"),
        },
        Keycode::QK_MIDI_VELOCITY_7 => KeycodeLabel {
            long: Some("MIDI Vel 7"),
            short: Some("MDV7"),
        },
        Keycode::QK_MIDI_VELOCITY_8 => KeycodeLabel {
            long: Some("MIDI Vel 8"),
            short: Some("MDV8"),
        },
        Keycode::QK_MIDI_VELOCITY_9 => KeycodeLabel {
            long: Some("MIDI Vel 9"),
            short: Some("MDV9"),
        },
        Keycode::QK_MIDI_VELOCITY_10 => KeycodeLabel {
            long: Some("MIDI Vel 10"),
            short: Some("MDV10"),
        },
        Keycode::QK_MIDI_VELOCITY_DOWN => KeycodeLabel {
            long: Some("MIDI Vel Down"),
            short: Some("MDV-"),
        },
        Keycode::QK_MIDI_VELOCITY_UP => KeycodeLabel {
            long: Some("MIDI Vel Up"),
            short: Some("MDV+"),
        },
        Keycode::QK_MIDI_CHANNEL_1 => KeycodeLabel {
            long: Some("MIDI Ch 1"),
            short: Some("MDC1"),
        },
        Keycode::QK_MIDI_CHANNEL_2 => KeycodeLabel {
            long: Some("MIDI Ch 2"),
            short: Some("MDC2"),
        },
        Keycode::QK_MIDI_CHANNEL_3 => KeycodeLabel {
            long: Some("MIDI Ch 3"),
            short: Some("MDC3"),
        },
        Keycode::QK_MIDI_CHANNEL_4 => KeycodeLabel {
            long: Some("MIDI Ch 4"),
            short: Some("MDC4"),
        },
        Keycode::QK_MIDI_CHANNEL_5 => KeycodeLabel {
            long: Some("MIDI Ch 5"),
            short: Some("MDC5"),
        },
        Keycode::QK_MIDI_CHANNEL_6 => KeycodeLabel {
            long: Some("MIDI Ch 6"),
            short: Some("MDC6"),
        },
        Keycode::QK_MIDI_CHANNEL_7 => KeycodeLabel {
            long: Some("MIDI Ch 7"),
            short: Some("MDC7"),
        },
        Keycode::QK_MIDI_CHANNEL_8 => KeycodeLabel {
            long: Some("MIDI Ch 8"),
            short: Some("MDC8"),
        },
        Keycode::QK_MIDI_CHANNEL_9 => KeycodeLabel {
            long: Some("MIDI Ch 9"),
            short: Some("MDC9"),
        },
        Keycode::QK_MIDI_CHANNEL_10 => KeycodeLabel {
            long: Some("MIDI Ch 10"),
            short: Some("MDC10"),
        },
        Keycode::QK_MIDI_CHANNEL_11 => KeycodeLabel {
            long: Some("MIDI Ch 11"),
            short: Some("MDC11"),
        },
        Keycode::QK_MIDI_CHANNEL_12 => KeycodeLabel {
            long: Some("MIDI Ch 12"),
            short: Some("MDC12"),
        },
        Keycode::QK_MIDI_CHANNEL_13 => KeycodeLabel {
            long: Some("MIDI Ch 13"),
            short: Some("MDC13"),
        },
        Keycode::QK_MIDI_CHANNEL_14 => KeycodeLabel {
            long: Some("MIDI Ch 14"),
            short: Some("MDC14"),
        },
        Keycode::QK_MIDI_CHANNEL_15 => KeycodeLabel {
            long: Some("MIDI Ch 15"),
            short: Some("MDC15"),
        },
        Keycode::QK_MIDI_CHANNEL_16 => KeycodeLabel {
            long: Some("MIDI Ch 16"),
            short: Some("MDC16"),
        },
        Keycode::QK_MIDI_CHANNEL_DOWN => KeycodeLabel {
            long: Some("MIDI Ch Down"),
            short: Some("MDC-"),
        },
        Keycode::QK_MIDI_CHANNEL_UP => KeycodeLabel {
            long: Some("MIDI Ch Up"),
            short: Some("MDC+"),
        },
        Keycode::QK_MIDI_ALL_NOTES_OFF => KeycodeLabel {
            long: Some("MIDI All Off"),
            short: Some("MDAOff"),
        },
        Keycode::QK_MIDI_SUSTAIN => KeycodeLabel {
            long: Some("MIDI Sustain"),
            short: Some("MDSus"),
        },
        Keycode::QK_MIDI_PORTAMENTO => KeycodeLabel {
            long: Some("MIDI Portamento"),
            short: Some("MDPort"),
        },
        Keycode::QK_MIDI_SOSTENUTO => KeycodeLabel {
            long: Some("MIDI Sostenuto"),
            short: Some("MDSost"),
        },
        Keycode::QK_MIDI_SOFT => KeycodeLabel {
            long: Some("MIDI Soft"),
            short: Some("MDSoft"),
        },
        Keycode::QK_MIDI_LEGATO => KeycodeLabel {
            long: Some("MIDI Legato"),
            short: Some("MDLeg"),
        },
        Keycode::QK_MIDI_MODULATION => KeycodeLabel {
            long: Some("MIDI Modulation"),
            short: Some("MDMod"),
        },
        Keycode::QK_MIDI_MODULATION_SPEED_DOWN => KeycodeLabel {
            long: Some("MIDI Mod Speed -"),
            short: Some("MDM-"),
        },
        Keycode::QK_MIDI_MODULATION_SPEED_UP => KeycodeLabel {
            long: Some("MIDI Mod Speed +"),
            short: Some("MDM+"),
        },
        Keycode::QK_MIDI_PITCH_BEND_DOWN => KeycodeLabel {
            long: Some("MIDI Pitch -"),
            short: Some("MDP-"),
        },
        Keycode::QK_MIDI_PITCH_BEND_UP => KeycodeLabel {
            long: Some("MIDI Pitch +"),
            short: Some("MDP+"),
        },
        Keycode::QK_SEQUENCER_ON => KeycodeLabel {
            long: Some("Sequencer On"),
            short: Some("SeqOn"),
        },
        Keycode::QK_SEQUENCER_OFF => KeycodeLabel {
            long: Some("Sequencer Off"),
            short: Some("SeqOff"),
        },
        Keycode::QK_SEQUENCER_TOGGLE => KeycodeLabel {
            long: Some("Sequencer Toggle"),
            short: Some("SeqTg"),
        },
        Keycode::QK_SEQUENCER_TEMPO_DOWN => KeycodeLabel {
            long: Some("Seq Tempo -"),
            short: Some("SeqT-"),
        },
        Keycode::QK_SEQUENCER_TEMPO_UP => KeycodeLabel {
            long: Some("Seq Tempo +"),
            short: Some("SeqT+"),
        },
        Keycode::QK_SEQUENCER_RESOLUTION_DOWN => KeycodeLabel {
            long: Some("Seq Res -"),
            short: Some("SeqR-"),
        },
        Keycode::QK_SEQUENCER_RESOLUTION_UP => KeycodeLabel {
            long: Some("Seq Res +"),
            short: Some("SeqR+"),
        },
        Keycode::QK_SEQUENCER_STEPS_ALL => KeycodeLabel {
            long: Some("Seq All Steps"),
            short: Some("SeqAll"),
        },
        Keycode::QK_SEQUENCER_STEPS_CLEAR => KeycodeLabel {
            long: Some("Seq Clear Steps"),
            short: Some("SeqClr"),
        },
        Keycode::QK_JOYSTICK_BUTTON_0 => KeycodeLabel {
            long: Some("Joy Btn 0"),
            short: Some("JoyB0"),
        },
        Keycode::QK_JOYSTICK_BUTTON_1 => KeycodeLabel {
            long: Some("Joy Btn 1"),
            short: Some("JoyB1"),
        },
        Keycode::QK_JOYSTICK_BUTTON_2 => KeycodeLabel {
            long: Some("Joy Btn 2"),
            short: Some("JoyB2"),
        },
        Keycode::QK_JOYSTICK_BUTTON_3 => KeycodeLabel {
            long: Some("Joy Btn 3"),
            short: Some("JoyB3"),
        },
        Keycode::QK_JOYSTICK_BUTTON_4 => KeycodeLabel {
            long: Some("Joy Btn 4"),
            short: Some("JoyB4"),
        },
        Keycode::QK_JOYSTICK_BUTTON_5 => KeycodeLabel {
            long: Some("Joy Btn 5"),
            short: Some("JoyB5"),
        },
        Keycode::QK_JOYSTICK_BUTTON_6 => KeycodeLabel {
            long: Some("Joy Btn 6"),
            short: Some("JoyB6"),
        },
        Keycode::QK_JOYSTICK_BUTTON_7 => KeycodeLabel {
            long: Some("Joy Btn 7"),
            short: Some("JoyB7"),
        },
        Keycode::QK_JOYSTICK_BUTTON_8 => KeycodeLabel {
            long: Some("Joy Btn 8"),
            short: Some("JoyB8"),
        },
        Keycode::QK_JOYSTICK_BUTTON_9 => KeycodeLabel {
            long: Some("Joy Btn 9"),
            short: Some("JoyB9"),
        },
        Keycode::QK_JOYSTICK_BUTTON_10 => KeycodeLabel {
            long: Some("Joy Btn 10"),
            short: Some("JoyB10"),
        },
        Keycode::QK_JOYSTICK_BUTTON_11 => KeycodeLabel {
            long: Some("Joy Btn 11"),
            short: Some("JoyB11"),
        },
        Keycode::QK_JOYSTICK_BUTTON_12 => KeycodeLabel {
            long: Some("Joy Btn 12"),
            short: Some("JoyB12"),
        },
        Keycode::QK_JOYSTICK_BUTTON_13 => KeycodeLabel {
            long: Some("Joy Btn 13"),
            short: Some("JoyB13"),
        },
        Keycode::QK_JOYSTICK_BUTTON_14 => KeycodeLabel {
            long: Some("Joy Btn 14"),
            short: Some("JoyB14"),
        },
        Keycode::QK_JOYSTICK_BUTTON_15 => KeycodeLabel {
            long: Some("Joy Btn 15"),
            short: Some("JoyB15"),
        },
        Keycode::QK_JOYSTICK_BUTTON_16 => KeycodeLabel {
            long: Some("Joy Btn 16"),
            short: Some("JoyB16"),
        },
        Keycode::QK_JOYSTICK_BUTTON_17 => KeycodeLabel {
            long: Some("Joy Btn 17"),
            short: Some("JoyB17"),
        },
        Keycode::QK_JOYSTICK_BUTTON_18 => KeycodeLabel {
            long: Some("Joy Btn 18"),
            short: Some("JoyB18"),
        },
        Keycode::QK_JOYSTICK_BUTTON_19 => KeycodeLabel {
            long: Some("Joy Btn 19"),
            short: Some("JoyB19"),
        },
        Keycode::QK_JOYSTICK_BUTTON_20 => KeycodeLabel {
            long: Some("Joy Btn 20"),
            short: Some("JoyB20"),
        },
        Keycode::QK_JOYSTICK_BUTTON_21 => KeycodeLabel {
            long: Some("Joy Btn 21"),
            short: Some("JoyB21"),
        },
        Keycode::QK_JOYSTICK_BUTTON_22 => KeycodeLabel {
            long: Some("Joy Btn 22"),
            short: Some("JoyB22"),
        },
        Keycode::QK_JOYSTICK_BUTTON_23 => KeycodeLabel {
            long: Some("Joy Btn 23"),
            short: Some("JoyB23"),
        },
        Keycode::QK_JOYSTICK_BUTTON_24 => KeycodeLabel {
            long: Some("Joy Btn 24"),
            short: Some("JoyB24"),
        },
        Keycode::QK_JOYSTICK_BUTTON_25 => KeycodeLabel {
            long: Some("Joy Btn 25"),
            short: Some("JoyB25"),
        },
        Keycode::QK_JOYSTICK_BUTTON_26 => KeycodeLabel {
            long: Some("Joy Btn 26"),
            short: Some("JoyB26"),
        },
        Keycode::QK_JOYSTICK_BUTTON_27 => KeycodeLabel {
            long: Some("Joy Btn 27"),
            short: Some("JoyB27"),
        },
        Keycode::QK_JOYSTICK_BUTTON_28 => KeycodeLabel {
            long: Some("Joy Btn 28"),
            short: Some("JoyB28"),
        },
        Keycode::QK_JOYSTICK_BUTTON_29 => KeycodeLabel {
            long: Some("Joy Btn 29"),
            short: Some("JoyB29"),
        },
        Keycode::QK_JOYSTICK_BUTTON_30 => KeycodeLabel {
            long: Some("Joy Btn 30"),
            short: Some("JoyB30"),
        },
        Keycode::QK_JOYSTICK_BUTTON_31 => KeycodeLabel {
            long: Some("Joy Btn 31"),
            short: Some("JoyB31"),
        },
        Keycode::QK_PROGRAMMABLE_BUTTON_1 => KeycodeLabel {
            long: Some("Prog Btn 1"),
            short: Some("PB1"),
        },
        Keycode::QK_PROGRAMMABLE_BUTTON_2 => KeycodeLabel {
            long: Some("Prog Btn 2"),
            short: Some("PB2"),
        },
        Keycode::QK_PROGRAMMABLE_BUTTON_3 => KeycodeLabel {
            long: Some("Prog Btn 3"),
            short: Some("PB3"),
        },
        Keycode::QK_PROGRAMMABLE_BUTTON_4 => KeycodeLabel {
            long: Some("Prog Btn 4"),
            short: Some("PB4"),
        },
        Keycode::QK_PROGRAMMABLE_BUTTON_5 => KeycodeLabel {
            long: Some("Prog Btn 5"),
            short: Some("PB5"),
        },
        Keycode::QK_PROGRAMMABLE_BUTTON_6 => KeycodeLabel {
            long: Some("Prog Btn 6"),
            short: Some("PB6"),
        },
        Keycode::QK_PROGRAMMABLE_BUTTON_7 => KeycodeLabel {
            long: Some("Prog Btn 7"),
            short: Some("PB7"),
        },
        Keycode::QK_PROGRAMMABLE_BUTTON_8 => KeycodeLabel {
            long: Some("Prog Btn 8"),
            short: Some("PB8"),
        },
        Keycode::QK_PROGRAMMABLE_BUTTON_9 => KeycodeLabel {
            long: Some("Prog Btn 9"),
            short: Some("PB9"),
        },
        Keycode::QK_PROGRAMMABLE_BUTTON_10 => KeycodeLabel {
            long: Some("Prog Btn 10"),
            short: Some("PB10"),
        },
        Keycode::QK_PROGRAMMABLE_BUTTON_11 => KeycodeLabel {
            long: Some("Prog Btn 11"),
            short: Some("PB11"),
        },
        Keycode::QK_PROGRAMMABLE_BUTTON_12 => KeycodeLabel {
            long: Some("Prog Btn 12"),
            short: Some("PB12"),
        },
        Keycode::QK_PROGRAMMABLE_BUTTON_13 => KeycodeLabel {
            long: Some("Prog Btn 13"),
            short: Some("PB13"),
        },
        Keycode::QK_PROGRAMMABLE_BUTTON_14 => KeycodeLabel {
            long: Some("Prog Btn 14"),
            short: Some("PB14"),
        },
        Keycode::QK_PROGRAMMABLE_BUTTON_15 => KeycodeLabel {
            long: Some("Prog Btn 15"),
            short: Some("PB15"),
        },
        Keycode::QK_PROGRAMMABLE_BUTTON_16 => KeycodeLabel {
            long: Some("Prog Btn 16"),
            short: Some("PB16"),
        },
        Keycode::QK_PROGRAMMABLE_BUTTON_17 => KeycodeLabel {
            long: Some("Prog Btn 17"),
            short: Some("PB17"),
        },
        Keycode::QK_PROGRAMMABLE_BUTTON_18 => KeycodeLabel {
            long: Some("Prog Btn 18"),
            short: Some("PB18"),
        },
        Keycode::QK_PROGRAMMABLE_BUTTON_19 => KeycodeLabel {
            long: Some("Prog Btn 19"),
            short: Some("PB19"),
        },
        Keycode::QK_PROGRAMMABLE_BUTTON_20 => KeycodeLabel {
            long: Some("Prog Btn 20"),
            short: Some("PB20"),
        },
        Keycode::QK_PROGRAMMABLE_BUTTON_21 => KeycodeLabel {
            long: Some("Prog Btn 21"),
            short: Some("PB21"),
        },
        Keycode::QK_PROGRAMMABLE_BUTTON_22 => KeycodeLabel {
            long: Some("Prog Btn 22"),
            short: Some("PB22"),
        },
        Keycode::QK_PROGRAMMABLE_BUTTON_23 => KeycodeLabel {
            long: Some("Prog Btn 23"),
            short: Some("PB23"),
        },
        Keycode::QK_PROGRAMMABLE_BUTTON_24 => KeycodeLabel {
            long: Some("Prog Btn 24"),
            short: Some("PB24"),
        },
        Keycode::QK_PROGRAMMABLE_BUTTON_25 => KeycodeLabel {
            long: Some("Prog Btn 25"),
            short: Some("PB25"),
        },
        Keycode::QK_PROGRAMMABLE_BUTTON_26 => KeycodeLabel {
            long: Some("Prog Btn 26"),
            short: Some("PB26"),
        },
        Keycode::QK_PROGRAMMABLE_BUTTON_27 => KeycodeLabel {
            long: Some("Prog Btn 27"),
            short: Some("PB27"),
        },
        Keycode::QK_PROGRAMMABLE_BUTTON_28 => KeycodeLabel {
            long: Some("Prog Btn 28"),
            short: Some("PB28"),
        },
        Keycode::QK_PROGRAMMABLE_BUTTON_29 => KeycodeLabel {
            long: Some("Prog Btn 29"),
            short: Some("PB29"),
        },
        Keycode::QK_PROGRAMMABLE_BUTTON_30 => KeycodeLabel {
            long: Some("Prog Btn 30"),
            short: Some("PB30"),
        },
        Keycode::QK_PROGRAMMABLE_BUTTON_31 => KeycodeLabel {
            long: Some("Prog Btn 31"),
            short: Some("PB31"),
        },
        Keycode::QK_PROGRAMMABLE_BUTTON_32 => KeycodeLabel {
            long: Some("Prog Btn 32"),
            short: Some("PB32"),
        },
        Keycode::QK_AUDIO_ON => KeycodeLabel {
            long: Some("Audio On"),
            short: Some("AudOn"),
        },
        Keycode::QK_AUDIO_OFF => KeycodeLabel {
            long: Some("Audio Off"),
            short: Some("AudOff"),
        },
        Keycode::QK_AUDIO_TOGGLE => KeycodeLabel {
            long: Some("Audio Toggle"),
            short: Some("AudTg"),
        },
        Keycode::QK_AUDIO_CLICKY_TOGGLE => KeycodeLabel {
            long: Some("Clicky Toggle"),
            short: Some("ClkTg"),
        },
        Keycode::QK_AUDIO_CLICKY_ON => KeycodeLabel {
            long: Some("Clicky Enable"),
            short: Some("ClkOn"),
        },
        Keycode::QK_AUDIO_CLICKY_OFF => KeycodeLabel {
            long: Some("Clicky Disable"),
            short: Some("ClkOff"),
        },
        Keycode::QK_AUDIO_CLICKY_UP => KeycodeLabel {
            long: Some("Clicky Up"),
            short: Some("Clk+"),
        },
        Keycode::QK_AUDIO_CLICKY_DOWN => KeycodeLabel {
            long: Some("Clicky Down"),
            short: Some("Clk-"),
        },
        Keycode::QK_AUDIO_CLICKY_RESET => KeycodeLabel {
            long: Some("Clicky Reset"),
            short: Some("ClkRst"),
        },
        Keycode::QK_MUSIC_ON => KeycodeLabel {
            long: Some("Music On"),
            short: Some("MusicOn"),
        },
        Keycode::QK_MUSIC_OFF => KeycodeLabel {
            long: Some("Music Off"),
            short: Some("MusicOf"),
        },
        Keycode::QK_MUSIC_TOGGLE => KeycodeLabel {
            long: Some("Music Toggle"),
            short: Some("MusicTg"),
        },
        Keycode::QK_MUSIC_MODE_NEXT => KeycodeLabel {
            long: Some("Music Mode"),
            short: Some("MusicMd"),
        },
        Keycode::QK_AUDIO_VOICE_NEXT => KeycodeLabel {
            long: Some("Voice Next"),
            short: Some("Voice+"),
        },
        Keycode::QK_AUDIO_VOICE_PREVIOUS => KeycodeLabel {
            long: Some("Voice Prev"),
            short: Some("Voice-"),
        },
        Keycode::QK_STENO_BOLT => KeycodeLabel {
            long: Some("Steno Bolt"),
            short: Some("StBolt"),
        },
        Keycode::QK_STENO_GEMINI => KeycodeLabel {
            long: Some("Steno Gemini"),
            short: Some("StGem"),
        },
        Keycode::QK_STENO_COMB => KeycodeLabel {
            long: Some("Steno Comb"),
            short: Some("StComb"),
        },
        Keycode::QK_STENO_COMB_MAX => KeycodeLabel {
            long: Some("Steno Comb Max"),
            short: Some("StCMax"),
        },
        Keycode::QK_MACRO_0 => KeycodeLabel {
            long: Some("Macro 0"),
            short: Some("M0"),
        },
        Keycode::QK_MACRO_1 => KeycodeLabel {
            long: Some("Macro 1"),
            short: Some("M1"),
        },
        Keycode::QK_MACRO_2 => KeycodeLabel {
            long: Some("Macro 2"),
            short: Some("M2"),
        },
        Keycode::QK_MACRO_3 => KeycodeLabel {
            long: Some("Macro 3"),
            short: Some("M3"),
        },
        Keycode::QK_MACRO_4 => KeycodeLabel {
            long: Some("Macro 4"),
            short: Some("M4"),
        },
        Keycode::QK_MACRO_5 => KeycodeLabel {
            long: Some("Macro 5"),
            short: Some("M5"),
        },
        Keycode::QK_MACRO_6 => KeycodeLabel {
            long: Some("Macro 6"),
            short: Some("M6"),
        },
        Keycode::QK_MACRO_7 => KeycodeLabel {
            long: Some("Macro 7"),
            short: Some("M7"),
        },
        Keycode::QK_MACRO_8 => KeycodeLabel {
            long: Some("Macro 8"),
            short: Some("M8"),
        },
        Keycode::QK_MACRO_9 => KeycodeLabel {
            long: Some("Macro 9"),
            short: Some("M9"),
        },
        Keycode::QK_MACRO_10 => KeycodeLabel {
            long: Some("Macro 10"),
            short: Some("M10"),
        },
        Keycode::QK_MACRO_11 => KeycodeLabel {
            long: Some("Macro 11"),
            short: Some("M11"),
        },
        Keycode::QK_MACRO_12 => KeycodeLabel {
            long: Some("Macro 12"),
            short: Some("M12"),
        },
        Keycode::QK_MACRO_13 => KeycodeLabel {
            long: Some("Macro 13"),
            short: Some("M13"),
        },
        Keycode::QK_MACRO_14 => KeycodeLabel {
            long: Some("Macro 14"),
            short: Some("M14"),
        },
        Keycode::QK_MACRO_15 => KeycodeLabel {
            long: Some("Macro 15"),
            short: Some("M15"),
        },
        Keycode::QK_MACRO_16 => KeycodeLabel {
            long: Some("Macro 16"),
            short: Some("M16"),
        },
        Keycode::QK_MACRO_17 => KeycodeLabel {
            long: Some("Macro 17"),
            short: Some("M17"),
        },
        Keycode::QK_MACRO_18 => KeycodeLabel {
            long: Some("Macro 18"),
            short: Some("M18"),
        },
        Keycode::QK_MACRO_19 => KeycodeLabel {
            long: Some("Macro 19"),
            short: Some("M19"),
        },
        Keycode::QK_MACRO_20 => KeycodeLabel {
            long: Some("Macro 20"),
            short: Some("M20"),
        },
        Keycode::QK_MACRO_21 => KeycodeLabel {
            long: Some("Macro 21"),
            short: Some("M21"),
        },
        Keycode::QK_MACRO_22 => KeycodeLabel {
            long: Some("Macro 22"),
            short: Some("M22"),
        },
        Keycode::QK_MACRO_23 => KeycodeLabel {
            long: Some("Macro 23"),
            short: Some("M23"),
        },
        Keycode::QK_MACRO_24 => KeycodeLabel {
            long: Some("Macro 24"),
            short: Some("M24"),
        },
        Keycode::QK_MACRO_25 => KeycodeLabel {
            long: Some("Macro 25"),
            short: Some("M25"),
        },
        Keycode::QK_MACRO_26 => KeycodeLabel {
            long: Some("Macro 26"),
            short: Some("M26"),
        },
        Keycode::QK_MACRO_27 => KeycodeLabel {
            long: Some("Macro 27"),
            short: Some("M27"),
        },
        Keycode::QK_MACRO_28 => KeycodeLabel {
            long: Some("Macro 28"),
            short: Some("M28"),
        },
        Keycode::QK_MACRO_29 => KeycodeLabel {
            long: Some("Macro 29"),
            short: Some("M29"),
        },
        Keycode::QK_MACRO_30 => KeycodeLabel {
            long: Some("Macro 30"),
            short: Some("M30"),
        },
        Keycode::QK_MACRO_31 => KeycodeLabel {
            long: Some("Macro 31"),
            short: Some("M31"),
        },
        Keycode::QK_BACKLIGHT_ON => KeycodeLabel {
            long: Some("BL On"),
            short: None,
        },
        Keycode::QK_BACKLIGHT_OFF => KeycodeLabel {
            long: Some("BL Off"),
            short: Some("BL Off"),
        },
        Keycode::QK_BACKLIGHT_TOGGLE => KeycodeLabel {
            long: Some("BL Toggle"),
            short: None,
        },
        Keycode::QK_BACKLIGHT_DOWN => KeycodeLabel {
            long: Some("BL -"),
            short: None,
        },
        Keycode::QK_BACKLIGHT_UP => KeycodeLabel {
            long: Some("BL +"),
            short: None,
        },
        Keycode::QK_BACKLIGHT_STEP => KeycodeLabel {
            long: Some("BL Cycle"),
            short: None,
        },
        Keycode::QK_BACKLIGHT_TOGGLE_BREATHING => KeycodeLabel {
            long: Some("BR Toggle"),
            short: None,
        },
        Keycode::QK_LED_MATRIX_ON => KeycodeLabel {
            long: Some("LED On"),
            short: Some("LEDOn"),
        },
        Keycode::QK_LED_MATRIX_OFF => KeycodeLabel {
            long: Some("LED Off"),
            short: Some("LEDOff"),
        },
        Keycode::QK_LED_MATRIX_TOGGLE => KeycodeLabel {
            long: Some("RGB Toggle"),
            short: None,
        },
        Keycode::QK_LED_MATRIX_MODE_NEXT => KeycodeLabel {
            long: Some("RGB Mode +"),
            short: None,
        },
        Keycode::QK_LED_MATRIX_MODE_PREVIOUS => KeycodeLabel {
            long: Some("RGB Mode -"),
            short: None,
        },
        Keycode::QK_LED_MATRIX_BRIGHTNESS_UP => KeycodeLabel {
            long: Some("LED Bri +"),
            short: Some("LED+"),
        },
        Keycode::QK_LED_MATRIX_BRIGHTNESS_DOWN => KeycodeLabel {
            long: Some("LED Bri -"),
            short: Some("LED-"),
        },
        Keycode::QK_LED_MATRIX_SPEED_UP => KeycodeLabel {
            long: Some("LED Spd +"),
            short: Some("LEDSp+"),
        },
        Keycode::QK_LED_MATRIX_SPEED_DOWN => KeycodeLabel {
            long: Some("LED Spd -"),
            short: Some("LEDSp-"),
        },
        Keycode::QK_UNDERGLOW_TOGGLE => KeycodeLabel {
            long: Some("UG Toggle"),
            short: Some("UGTg"),
        },
        Keycode::QK_UNDERGLOW_MODE_NEXT => KeycodeLabel {
            long: Some("UG Mode +"),
            short: Some("UGM+"),
        },
        Keycode::QK_UNDERGLOW_MODE_PREVIOUS => KeycodeLabel {
            long: Some("UG Mode -"),
            short: Some("UGM-"),
        },
        Keycode::QK_UNDERGLOW_HUE_UP => KeycodeLabel {
            long: Some("Hue +"),
            short: None,
        },
        Keycode::QK_UNDERGLOW_HUE_DOWN => KeycodeLabel {
            long: Some("Hue -"),
            short: None,
        },
        Keycode::QK_UNDERGLOW_SATURATION_UP => KeycodeLabel {
            long: Some("Sat +"),
            short: None,
        },
        Keycode::QK_UNDERGLOW_SATURATION_DOWN => KeycodeLabel {
            long: Some("Sat -"),
            short: None,
        },
        Keycode::QK_UNDERGLOW_VALUE_UP => KeycodeLabel {
            long: Some("Bright +"),
            short: None,
        },
        Keycode::QK_UNDERGLOW_VALUE_DOWN => KeycodeLabel {
            long: Some("Bright -"),
            short: None,
        },
        Keycode::QK_UNDERGLOW_SPEED_UP => KeycodeLabel {
            long: Some("Effect Speed+"),
            short: None,
        },
        Keycode::QK_UNDERGLOW_SPEED_DOWN => KeycodeLabel {
            long: Some("Effect Speed-"),
            short: None,
        },
        Keycode::RGB_MODE_PLAIN => KeycodeLabel {
            long: Some("RGB Mode P"),
            short: None,
        },
        Keycode::RGB_MODE_BREATHE => KeycodeLabel {
            long: Some("RGB Mode B"),
            short: None,
        },
        Keycode::RGB_MODE_RAINBOW => KeycodeLabel {
            long: Some("RGB Mode R"),
            short: None,
        },
        Keycode::RGB_MODE_SWIRL => KeycodeLabel {
            long: Some("RGB Mode SW"),
            short: None,
        },
        Keycode::RGB_MODE_SNAKE => KeycodeLabel {
            long: Some("RGB Mode SN"),
            short: None,
        },
        Keycode::RGB_MODE_KNIGHT => KeycodeLabel {
            long: Some("RGB Mode K"),
            short: None,
        },
        Keycode::RGB_MODE_XMAS => KeycodeLabel {
            long: Some("RGB Mode X"),
            short: None,
        },
        Keycode::RGB_MODE_GRADIENT => KeycodeLabel {
            long: Some("RGB Mode G"),
            short: None,
        },
        Keycode::RGB_MODE_RGBTEST => KeycodeLabel {
            long: Some("RGB Mode Test"),
            short: None,
        },
        Keycode::RGB_MODE_TWINKLE => KeycodeLabel {
            long: Some("RGB Mode T"),
            short: None,
        },
        Keycode::QK_RGB_MATRIX_ON => KeycodeLabel {
            long: Some("RGB On"),
            short: Some("RGBOn"),
        },
        Keycode::QK_RGB_MATRIX_OFF => KeycodeLabel {
            long: Some("RGB Off"),
            short: Some("RGBOff"),
        },
        Keycode::QK_RGB_MATRIX_TOGGLE => KeycodeLabel {
            long: Some("RGB Toggle"),
            short: Some("RGBTg"),
        },
        Keycode::QK_RGB_MATRIX_MODE_NEXT => KeycodeLabel {
            long: Some("RGB Mode +"),
            short: Some("RGBM+"),
        },
        Keycode::QK_RGB_MATRIX_MODE_PREVIOUS => KeycodeLabel {
            long: Some("RGB Mode -"),
            short: Some("RGBM-"),
        },
        Keycode::QK_RGB_MATRIX_HUE_UP => KeycodeLabel {
            long: Some("RGB Hue +"),
            short: Some("RGBH+"),
        },
        Keycode::QK_RGB_MATRIX_HUE_DOWN => KeycodeLabel {
            long: Some("RGB Hue -"),
            short: Some("RGBH-"),
        },
        Keycode::QK_RGB_MATRIX_SATURATION_UP => KeycodeLabel {
            long: Some("RGB Sat +"),
            short: Some("RGBS+"),
        },
        Keycode::QK_RGB_MATRIX_SATURATION_DOWN => KeycodeLabel {
            long: Some("RGB Sat -"),
            short: Some("RGBS-"),
        },
        Keycode::QK_RGB_MATRIX_VALUE_UP => KeycodeLabel {
            long: Some("RGB Val +"),
            short: Some("RGBV+"),
        },
        Keycode::QK_RGB_MATRIX_VALUE_DOWN => KeycodeLabel {
            long: Some("RGB Val -"),
            short: Some("RGBV-"),
        },
        Keycode::QK_RGB_MATRIX_SPEED_UP => KeycodeLabel {
            long: Some("RGB Spd +"),
            short: Some("RGBSp+"),
        },
        Keycode::QK_RGB_MATRIX_SPEED_DOWN => KeycodeLabel {
            long: Some("RGB Spd -"),
            short: Some("RGBSp-"),
        },
        Keycode::QK_BOOTLOADER => KeycodeLabel {
            long: Some("Bootloader"),
            short: Some("Boot"),
        },
        Keycode::QK_REBOOT => KeycodeLabel {
            long: Some("Reboot"),
            short: Some("Reboot"),
        },
        Keycode::QK_DEBUG_TOGGLE => KeycodeLabel {
            long: Some("Debug Toggle"),
            short: Some("DbgTg"),
        },
        Keycode::QK_CLEAR_EEPROM => KeycodeLabel {
            long: Some("Clear EEPROM"),
            short: Some("ClrEE"),
        },
        Keycode::QK_MAKE => KeycodeLabel {
            long: Some("Make"),
            short: Some("Make"),
        },
        Keycode::QK_AUTO_SHIFT_DOWN => KeycodeLabel {
            long: Some("AutoShift -"),
            short: Some("AS -"),
        },
        Keycode::QK_AUTO_SHIFT_UP => KeycodeLabel {
            long: Some("AutoShift +"),
            short: Some("AS +"),
        },
        Keycode::QK_AUTO_SHIFT_REPORT => KeycodeLabel {
            long: Some("AutoShift Rep"),
            short: Some("AS R"),
        },
        Keycode::QK_AUTO_SHIFT_ON => KeycodeLabel {
            long: Some("AutoShift On"),
            short: Some("AS On"),
        },
        Keycode::QK_AUTO_SHIFT_OFF => KeycodeLabel {
            long: Some("AutoShift Off"),
            short: Some("ASOff"),
        },
        Keycode::QK_AUTO_SHIFT_TOGGLE => KeycodeLabel {
            long: Some("AutoShift Tog"),
            short: Some("AS Tg"),
        },
        Keycode::QK_GRAVE_ESCAPE => KeycodeLabel {
            long: Some("Esc `"),
            short: None,
        },
        Keycode::QK_VELOCIKEY_TOGGLE => KeycodeLabel {
            long: Some("Velocikey"),
            short: Some("VelKey"),
        },
        Keycode::QK_SPACE_CADET_LEFT_CTRL_PARENTHESIS_OPEN => KeycodeLabel {
            long: Some("LC ("),
            short: None,
        },
        Keycode::QK_SPACE_CADET_RIGHT_CTRL_PARENTHESIS_CLOSE => KeycodeLabel {
            long: Some("RC )"),
            short: None,
        },
        Keycode::QK_SPACE_CADET_LEFT_SHIFT_PARENTHESIS_OPEN => KeycodeLabel {
            long: Some("LS ("),
            short: None,
        },
        Keycode::QK_SPACE_CADET_RIGHT_SHIFT_PARENTHESIS_CLOSE => KeycodeLabel {
            long: Some("RS )"),
            short: None,
        },
        Keycode::QK_SPACE_CADET_LEFT_ALT_PARENTHESIS_OPEN => KeycodeLabel {
            long: Some("LA ("),
            short: None,
        },
        Keycode::QK_SPACE_CADET_RIGHT_ALT_PARENTHESIS_CLOSE => KeycodeLabel {
            long: Some("RA )"),
            short: None,
        },
        Keycode::QK_SPACE_CADET_RIGHT_SHIFT_ENTER => KeycodeLabel {
            long: Some("SftEnt"),
            short: None,
        },
        Keycode::QK_OUTPUT_AUTO => KeycodeLabel {
            long: Some("Out Auto"),
            short: Some("OutAuto"),
        },
        Keycode::QK_OUTPUT_USB => KeycodeLabel {
            long: Some("Out USB"),
            short: Some("OutUSB"),
        },
        Keycode::QK_OUTPUT_BLUETOOTH => KeycodeLabel {
            long: Some("Out BT"),
            short: Some("OutBT"),
        },
        Keycode::QK_UNICODE_MODE_NEXT => KeycodeLabel {
            long: Some("Unicode +"),
            short: Some("Uni +"),
        },
        Keycode::QK_UNICODE_MODE_PREVIOUS => KeycodeLabel {
            long: Some("Unicode -"),
            short: Some("Uni -"),
        },
        Keycode::QK_UNICODE_MODE_MACOS => KeycodeLabel {
            long: Some("Unicode macOS"),
            short: Some("UniMac"),
        },
        Keycode::QK_UNICODE_MODE_LINUX => KeycodeLabel {
            long: Some("Unicode Linux"),
            short: Some("UniLnx"),
        },
        Keycode::QK_UNICODE_MODE_WINDOWS => KeycodeLabel {
            long: Some("Unicode Win"),
            short: Some("UniWin"),
        },
        Keycode::QK_UNICODE_MODE_BSD => KeycodeLabel {
            long: Some("Unicode BSD"),
            short: Some("UniBSD"),
        },
        Keycode::QK_UNICODE_MODE_WINCOMPOSE => KeycodeLabel {
            long: Some("Unicode WinC"),
            short: Some("UniWinC"),
        },
        Keycode::QK_UNICODE_MODE_EMACS => KeycodeLabel {
            long: Some("Unicode Emacs"),
            short: Some("UniEm"),
        },
        Keycode::QK_HAPTIC_ON => KeycodeLabel {
            long: Some("Haptic On"),
            short: Some("HapOn"),
        },
        Keycode::QK_HAPTIC_OFF => KeycodeLabel {
            long: Some("Haptic Off"),
            short: Some("HapOff"),
        },
        Keycode::QK_HAPTIC_TOGGLE => KeycodeLabel {
            long: Some("Haptic Toggle"),
            short: Some("HapTg"),
        },
        Keycode::QK_HAPTIC_RESET => KeycodeLabel {
            long: Some("Haptic Reset"),
            short: Some("HapRst"),
        },
        Keycode::QK_HAPTIC_FEEDBACK_TOGGLE => KeycodeLabel {
            long: Some("Haptic FB Tog"),
            short: Some("HapFBTg"),
        },
        Keycode::QK_HAPTIC_BUZZ_TOGGLE => KeycodeLabel {
            long: Some("Haptic Buzz"),
            short: Some("HapBuzz"),
        },
        Keycode::QK_HAPTIC_MODE_NEXT => KeycodeLabel {
            long: Some("Haptic +"),
            short: Some("Hap +"),
        },
        Keycode::QK_HAPTIC_MODE_PREVIOUS => KeycodeLabel {
            long: Some("Haptic -"),
            short: Some("Hap -"),
        },
        Keycode::QK_HAPTIC_CONTINUOUS_TOGGLE => KeycodeLabel {
            long: Some("Haptic Cont"),
            short: Some("HapCont"),
        },
        Keycode::QK_HAPTIC_CONTINUOUS_UP => KeycodeLabel {
            long: Some("Haptic + "),
            short: Some("HapC+"),
        },
        Keycode::QK_HAPTIC_CONTINUOUS_DOWN => KeycodeLabel {
            long: Some("Haptic -"),
            short: Some("HapC-"),
        },
        Keycode::QK_HAPTIC_DWELL_UP => KeycodeLabel {
            long: Some("Haptic Dwell +"),
            short: Some("HapDw+"),
        },
        Keycode::QK_HAPTIC_DWELL_DOWN => KeycodeLabel {
            long: Some("Haptic Dwell -"),
            short: Some("HapDw-"),
        },
        Keycode::QK_COMBO_ON => KeycodeLabel {
            long: Some("Combo On"),
            short: Some("CombOn"),
        },
        Keycode::QK_COMBO_OFF => KeycodeLabel {
            long: Some("Combo Off"),
            short: Some("CombOff"),
        },
        Keycode::QK_COMBO_TOGGLE => KeycodeLabel {
            long: Some("Combo Toggle"),
            short: Some("CombTg"),
        },
        Keycode::QK_DYNAMIC_MACRO_RECORD_START_1 => KeycodeLabel {
            long: Some("DM Rec 1"),
            short: Some("DMRec1"),
        },
        Keycode::QK_DYNAMIC_MACRO_RECORD_START_2 => KeycodeLabel {
            long: Some("DM Rec 2"),
            short: Some("DMRec2"),
        },
        Keycode::QK_DYNAMIC_MACRO_RECORD_STOP => KeycodeLabel {
            long: Some("DM Stop"),
            short: Some("DMStop"),
        },
        Keycode::QK_DYNAMIC_MACRO_PLAY_1 => KeycodeLabel {
            long: Some("DM Play 1"),
            short: Some("DMPlay1"),
        },
        Keycode::QK_DYNAMIC_MACRO_PLAY_2 => KeycodeLabel {
            long: Some("DM Play 2"),
            short: Some("DMPlay2"),
        },
        Keycode::QK_LEADER => KeycodeLabel {
            long: Some("Leader"),
            short: Some("Lead"),
        },
        Keycode::QK_LOCK => KeycodeLabel {
            long: Some("Lock"),
            short: Some("Lock"),
        },
        Keycode::QK_ONE_SHOT_ON => KeycodeLabel {
            long: Some("OneShot On"),
            short: Some("1ShotOn"),
        },
        Keycode::QK_ONE_SHOT_OFF => KeycodeLabel {
            long: Some("OneShot Off"),
            short: Some("1ShotOf"),
        },
        Keycode::QK_ONE_SHOT_TOGGLE => KeycodeLabel {
            long: Some("OneShot Toggle"),
            short: Some("1ShotTg"),
        },
        Keycode::QK_KEY_OVERRIDE_TOGGLE => KeycodeLabel {
            long: Some("KO Toggle"),
            short: Some("KOTg"),
        },
        Keycode::QK_KEY_OVERRIDE_ON => KeycodeLabel {
            long: Some("KO On"),
            short: Some("KOOn"),
        },
        Keycode::QK_KEY_OVERRIDE_OFF => KeycodeLabel {
            long: Some("KO Off"),
            short: Some("KOOff"),
        },
        Keycode::QK_SECURE_LOCK => KeycodeLabel {
            long: Some("Secure Lock"),
            short: Some("SecLock"),
        },
        Keycode::QK_SECURE_UNLOCK => KeycodeLabel {
            long: Some("Secure Unlock"),
            short: Some("SecUnlk"),
        },
        Keycode::QK_SECURE_TOGGLE => KeycodeLabel {
            long: Some("Secure Toggle"),
            short: Some("SecTg"),
        },
        Keycode::QK_SECURE_REQUEST => KeycodeLabel {
            long: Some("Secure Request"),
            short: Some("SecReq"),
        },
        Keycode::QK_DYNAMIC_TAPPING_TERM_PRINT => KeycodeLabel {
            long: Some("DT Term"),
            short: Some("DTTerm"),
        },
        Keycode::QK_DYNAMIC_TAPPING_TERM_UP => KeycodeLabel {
            long: Some("DT Term +"),
            short: Some("DTTerm+"),
        },
        Keycode::QK_DYNAMIC_TAPPING_TERM_DOWN => KeycodeLabel {
            long: Some("DT Term -"),
            short: Some("DTTerm-"),
        },
        Keycode::QK_CAPS_WORD_TOGGLE => KeycodeLabel {
            long: Some("Caps Word"),
            short: Some("CapWord"),
        },
        Keycode::QK_AUTOCORRECT_ON => KeycodeLabel {
            long: Some("Autocorrect On"),
            short: Some("ACOn"),
        },
        Keycode::QK_AUTOCORRECT_OFF => KeycodeLabel {
            long: Some("Autocorrect Off"),
            short: Some("ACOff"),
        },
        Keycode::QK_AUTOCORRECT_TOGGLE => KeycodeLabel {
            long: Some("Autocorrect Tog"),
            short: Some("ACTg"),
        },
        Keycode::QK_TRI_LAYER_LOWER => KeycodeLabel {
            long: Some("Tri Lower"),
            short: Some("TriLow"),
        },
        Keycode::QK_TRI_LAYER_UPPER => KeycodeLabel {
            long: Some("Tri Upper"),
            short: Some("TriUp"),
        },
        Keycode::QK_REPEAT_KEY => KeycodeLabel {
            long: Some("Repeat Key"),
            short: Some("RepKey"),
        },
        Keycode::QK_ALT_REPEAT_KEY => KeycodeLabel {
            long: Some("Alt Repeat"),
            short: Some("ARepKey"),
        },
        Keycode::QK_KB_0 => KeycodeLabel {
            long: Some("KB 0"),
            short: Some("KB0"),
        },
        Keycode::QK_KB_1 => KeycodeLabel {
            long: Some("KB 1"),
            short: Some("KB1"),
        },
        Keycode::QK_KB_2 => KeycodeLabel {
            long: Some("KB 2"),
            short: Some("KB2"),
        },
        Keycode::QK_KB_3 => KeycodeLabel {
            long: Some("KB 3"),
            short: Some("KB3"),
        },
        Keycode::QK_KB_4 => KeycodeLabel {
            long: Some("KB 4"),
            short: Some("KB4"),
        },
        Keycode::QK_KB_5 => KeycodeLabel {
            long: Some("KB 5"),
            short: Some("KB5"),
        },
        Keycode::QK_KB_6 => KeycodeLabel {
            long: Some("KB 6"),
            short: Some("KB6"),
        },
        Keycode::QK_KB_7 => KeycodeLabel {
            long: Some("KB 7"),
            short: Some("KB7"),
        },
        Keycode::QK_KB_8 => KeycodeLabel {
            long: Some("KB 8"),
            short: Some("KB8"),
        },
        Keycode::QK_KB_9 => KeycodeLabel {
            long: Some("KB 9"),
            short: Some("KB9"),
        },
        Keycode::QK_KB_10 => KeycodeLabel {
            long: Some("KB 10"),
            short: Some("KB10"),
        },
        Keycode::QK_KB_11 => KeycodeLabel {
            long: Some("KB 11"),
            short: Some("KB11"),
        },
        Keycode::QK_KB_12 => KeycodeLabel {
            long: Some("KB 12"),
            short: Some("KB12"),
        },
        Keycode::QK_KB_13 => KeycodeLabel {
            long: Some("KB 13"),
            short: Some("KB13"),
        },
        Keycode::QK_KB_14 => KeycodeLabel {
            long: Some("KB 14"),
            short: Some("KB14"),
        },
        Keycode::QK_KB_15 => KeycodeLabel {
            long: Some("KB 15"),
            short: Some("KB15"),
        },
        Keycode::QK_KB_16 => KeycodeLabel {
            long: Some("KB 16"),
            short: Some("KB16"),
        },
        Keycode::QK_KB_17 => KeycodeLabel {
            long: Some("KB 17"),
            short: Some("KB17"),
        },
        Keycode::QK_KB_18 => KeycodeLabel {
            long: Some("KB 18"),
            short: Some("KB18"),
        },
        Keycode::QK_KB_19 => KeycodeLabel {
            long: Some("KB 19"),
            short: Some("KB19"),
        },
        Keycode::QK_KB_20 => KeycodeLabel {
            long: Some("KB 20"),
            short: Some("KB20"),
        },
        Keycode::QK_KB_21 => KeycodeLabel {
            long: Some("KB 21"),
            short: Some("KB21"),
        },
        Keycode::QK_KB_22 => KeycodeLabel {
            long: Some("KB 22"),
            short: Some("KB22"),
        },
        Keycode::QK_KB_23 => KeycodeLabel {
            long: Some("KB 23"),
            short: Some("KB23"),
        },
        Keycode::QK_KB_24 => KeycodeLabel {
            long: Some("KB 24"),
            short: Some("KB24"),
        },
        Keycode::QK_KB_25 => KeycodeLabel {
            long: Some("KB 25"),
            short: Some("KB25"),
        },
        Keycode::QK_KB_26 => KeycodeLabel {
            long: Some("KB 26"),
            short: Some("KB26"),
        },
        Keycode::QK_KB_27 => KeycodeLabel {
            long: Some("KB 27"),
            short: Some("KB27"),
        },
        Keycode::QK_KB_28 => KeycodeLabel {
            long: Some("KB 28"),
            short: Some("KB28"),
        },
        Keycode::QK_KB_29 => KeycodeLabel {
            long: Some("KB 29"),
            short: Some("KB29"),
        },
        Keycode::QK_KB_30 => KeycodeLabel {
            long: Some("KB 30"),
            short: Some("KB30"),
        },
        Keycode::QK_KB_31 => KeycodeLabel {
            long: Some("KB 31"),
            short: Some("KB31"),
        },
        Keycode::QK_USER_0 => KeycodeLabel {
            long: Some("User 0"),
            short: Some("Usr0"),
        },
        Keycode::QK_USER_1 => KeycodeLabel {
            long: Some("User 1"),
            short: Some("Usr1"),
        },
        Keycode::QK_USER_2 => KeycodeLabel {
            long: Some("User 2"),
            short: Some("Usr2"),
        },
        Keycode::QK_USER_3 => KeycodeLabel {
            long: Some("User 3"),
            short: Some("Usr3"),
        },
        Keycode::QK_USER_4 => KeycodeLabel {
            long: Some("User 4"),
            short: Some("Usr4"),
        },
        Keycode::QK_USER_5 => KeycodeLabel {
            long: Some("User 5"),
            short: Some("Usr5"),
        },
        Keycode::QK_USER_6 => KeycodeLabel {
            long: Some("User 6"),
            short: Some("Usr6"),
        },
        Keycode::QK_USER_7 => KeycodeLabel {
            long: Some("User 7"),
            short: Some("Usr7"),
        },
        Keycode::QK_USER_8 => KeycodeLabel {
            long: Some("User 8"),
            short: Some("Usr8"),
        },
        Keycode::QK_USER_9 => KeycodeLabel {
            long: Some("User 9"),
            short: Some("Usr9"),
        },
        Keycode::QK_USER_10 => KeycodeLabel {
            long: Some("User 10"),
            short: Some("Usr10"),
        },
        Keycode::QK_USER_11 => KeycodeLabel {
            long: Some("User 11"),
            short: Some("Usr11"),
        },
        Keycode::QK_USER_12 => KeycodeLabel {
            long: Some("User 12"),
            short: Some("Usr12"),
        },
        Keycode::QK_USER_13 => KeycodeLabel {
            long: Some("User 13"),
            short: Some("Usr13"),
        },
        Keycode::QK_USER_14 => KeycodeLabel {
            long: Some("User 14"),
            short: Some("Usr14"),
        },
        Keycode::QK_USER_15 => KeycodeLabel {
            long: Some("User 15"),
            short: Some("Usr15"),
        },
        Keycode::QK_USER_16 => KeycodeLabel {
            long: Some("User 16"),
            short: Some("Usr16"),
        },
        Keycode::QK_USER_17 => KeycodeLabel {
            long: Some("User 17"),
            short: Some("Usr17"),
        },
        Keycode::QK_USER_18 => KeycodeLabel {
            long: Some("User 18"),
            short: Some("Usr18"),
        },
        Keycode::QK_USER_19 => KeycodeLabel {
            long: Some("User 19"),
            short: Some("Usr19"),
        },
        Keycode::QK_USER_20 => KeycodeLabel {
            long: Some("User 20"),
            short: Some("Usr20"),
        },
        Keycode::QK_USER_21 => KeycodeLabel {
            long: Some("User 21"),
            short: Some("Usr21"),
        },
        Keycode::QK_USER_22 => KeycodeLabel {
            long: Some("User 22"),
            short: Some("Usr22"),
        },
        Keycode::QK_USER_23 => KeycodeLabel {
            long: Some("User 23"),
            short: Some("Usr23"),
        },
        Keycode::QK_USER_24 => KeycodeLabel {
            long: Some("User 24"),
            short: Some("Usr24"),
        },
        Keycode::QK_USER_25 => KeycodeLabel {
            long: Some("User 25"),
            short: Some("Usr25"),
        },
        Keycode::QK_USER_26 => KeycodeLabel {
            long: Some("User 26"),
            short: Some("Usr26"),
        },
        Keycode::QK_USER_27 => KeycodeLabel {
            long: Some("User 27"),
            short: Some("Usr27"),
        },
        Keycode::QK_USER_28 => KeycodeLabel {
            long: Some("User 28"),
            short: Some("Usr28"),
        },
        Keycode::QK_USER_29 => KeycodeLabel {
            long: Some("User 29"),
            short: Some("Usr29"),
        },
        Keycode::QK_USER_30 => KeycodeLabel {
            long: Some("User 30"),
            short: Some("Usr30"),
        },
        Keycode::QK_USER_31 => KeycodeLabel {
            long: Some("User 31"),
            short: Some("Usr31"),
        },
    }
}

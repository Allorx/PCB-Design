// Aleksas Girenas 23/10/2022
// Keyboard key functions and assignments

use usbd_human_interface_device::page::Keyboard;

// ?63 keys excluding fn key and consumer - normal layer
pub fn get_keys(keys: [[i32; 14]; 5]) -> [Keyboard; 63] {
    [
        if keys[0][0] == 1 {
            Keyboard::Escape
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[0][1] == 1 {
            Keyboard::Keyboard1
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[0][2] == 1 {
            Keyboard::Keyboard2
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[0][3] == 1 {
            Keyboard::Keyboard3
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[0][4] == 1 {
            Keyboard::Keyboard4
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[0][5] == 1 {
            Keyboard::Keyboard5
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[0][6] == 1 {
            Keyboard::Keyboard6
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[0][7] == 1 {
            Keyboard::Keyboard7
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[0][8] == 1 {
            Keyboard::Keyboard8
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[0][9] == 1 {
            Keyboard::Keyboard9
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[0][10] == 1 {
            Keyboard::Keyboard0
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[0][11] == 1 {
            Keyboard::Minus
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[0][12] == 1 {
            Keyboard::Equal
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[0][13] == 1 {
            Keyboard::DeleteBackspace
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[1][0] == 1 {
            Keyboard::Tab
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[1][1] == 1 {
            Keyboard::Q
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[1][2] == 1 {
            Keyboard::W
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[1][3] == 1 {
            Keyboard::E
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[1][4] == 1 {
            Keyboard::R
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[1][5] == 1 {
            Keyboard::T
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[1][6] == 1 {
            Keyboard::Y
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[1][7] == 1 {
            Keyboard::U
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[1][8] == 1 {
            Keyboard::I
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[1][9] == 1 {
            Keyboard::O
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[1][10] == 1 {
            Keyboard::P
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[1][11] == 1 {
            Keyboard::LeftBrace
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[1][12] == 1 {
            Keyboard::RightBrace
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[2][0] == 1 {
            Keyboard::CapsLock
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[2][1] == 1 {
            Keyboard::A
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[2][2] == 1 {
            Keyboard::S
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[2][3] == 1 {
            Keyboard::D
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[2][4] == 1 {
            Keyboard::F
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[2][5] == 1 {
            Keyboard::G
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[2][6] == 1 {
            Keyboard::H
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[2][7] == 1 {
            Keyboard::J
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[2][8] == 1 {
            Keyboard::K
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[2][9] == 1 {
            Keyboard::L
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[2][10] == 1 {
            Keyboard::Semicolon
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[2][11] == 1 {
            Keyboard::Apostrophe
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[2][12] == 1 {
            Keyboard::ReturnEnter
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[2][13] == 1 {
            Keyboard::NonUSHash
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[3][0] == 1 {
            Keyboard::LeftShift
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[3][1] == 1 {
            Keyboard::NonUSBackslash
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[3][2] == 1 {
            Keyboard::Z
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[3][3] == 1 {
            Keyboard::X
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[3][4] == 1 {
            Keyboard::C
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[3][5] == 1 {
            Keyboard::V
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[3][6] == 1 {
            Keyboard::B
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[3][7] == 1 {
            Keyboard::N
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[3][8] == 1 {
            Keyboard::M
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[3][9] == 1 {
            Keyboard::Comma
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[3][10] == 1 {
            Keyboard::Dot
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[3][11] == 1 {
            Keyboard::ForwardSlash
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[3][12] == 1 {
            Keyboard::RightShift
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[3][13] == 1 {
            Keyboard::UpArrow
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[4][0] == 1 {
            Keyboard::LeftControl
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[4][1] == 1 {
            Keyboard::LeftGUI
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[4][2] == 1 {
            Keyboard::LeftAlt
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[4][6] == 1 {
            Keyboard::Space
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[4][9] == 1 {
            Keyboard::RightAlt
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[4][11] == 1 {
            Keyboard::LeftArrow
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[4][12] == 1 {
            Keyboard::DownArrow
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[4][13] == 1 {
            Keyboard::RightArrow
        } else {
            Keyboard::NoEventIndicated
        },
    ]
}

// ?63 keys excluding fn key and consumer - fn layer
pub fn get_fnkeys(keys: [[i32; 14]; 5]) -> [Keyboard; 63] {
    [
        if keys[0][0] == 1 {
            Keyboard::Grave
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[0][1] == 1 {
            Keyboard::F1
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[0][2] == 1 {
            Keyboard::F2
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[0][3] == 1 {
            Keyboard::F3
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[0][4] == 1 {
            Keyboard::F4
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[0][5] == 1 {
            Keyboard::F5
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[0][6] == 1 {
            Keyboard::F6
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[0][7] == 1 {
            Keyboard::F7
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[0][8] == 1 {
            Keyboard::F8
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[0][9] == 1 {
            Keyboard::F9
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[0][10] == 1 {
            Keyboard::F10
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[0][11] == 1 {
            Keyboard::F11
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[0][12] == 1 {
            Keyboard::F12
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[0][13] == 1 {
            Keyboard::DeleteForward
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[1][0] == 1 {
            Keyboard::Tab
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[1][1] == 1 {
            Keyboard::Q
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[1][2] == 1 {
            Keyboard::W
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[1][3] == 1 {
            Keyboard::E
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[1][4] == 1 {
            Keyboard::R
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[1][5] == 1 {
            Keyboard::T
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[1][6] == 1 {
            Keyboard::Y
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[1][7] == 1 {
            Keyboard::U
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[1][8] == 1 {
            Keyboard::I
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[1][9] == 1 {
            Keyboard::O
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[1][10] == 1 {
            Keyboard::P
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[1][11] == 1 {
            Keyboard::LeftBrace
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[1][12] == 1 {
            Keyboard::RightBrace
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[2][0] == 1 {
            Keyboard::CapsLock
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[2][1] == 1 {
            Keyboard::A
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[2][2] == 1 {
            Keyboard::S
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[2][3] == 1 {
            Keyboard::D
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[2][4] == 1 {
            Keyboard::F
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[2][5] == 1 {
            Keyboard::G
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[2][6] == 1 {
            Keyboard::H
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[2][7] == 1 {
            Keyboard::J
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[2][8] == 1 {
            Keyboard::K
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[2][9] == 1 {
            Keyboard::L
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[2][10] == 1 {
            Keyboard::Semicolon
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[2][11] == 1 {
            Keyboard::Apostrophe
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[2][12] == 1 {
            Keyboard::ReturnEnter
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[2][13] == 1 {
            Keyboard::DeleteForward
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[3][0] == 1 {
            Keyboard::LeftShift
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[3][1] == 1 {
            Keyboard::NonUSBackslash
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[3][2] == 1 {
            Keyboard::Z
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[3][3] == 1 {
            Keyboard::X
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[3][4] == 1 {
            Keyboard::C
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[3][5] == 1 {
            Keyboard::V
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[3][6] == 1 {
            Keyboard::B
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[3][7] == 1 {
            Keyboard::N
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[3][8] == 1 {
            Keyboard::M
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[3][9] == 1 {
            Keyboard::Comma
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[3][10] == 1 {
            Keyboard::Dot
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[3][11] == 1 {
            Keyboard::ForwardSlash
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[3][12] == 1 {
            Keyboard::NonUSHash
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[3][13] == 1 {
            Keyboard::UpArrow
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[4][0] == 1 {
            Keyboard::LeftControl
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[4][1] == 1 {
            Keyboard::LeftGUI
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[4][2] == 1 {
            Keyboard::LeftAlt
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[4][6] == 1 {
            Keyboard::Space
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[4][9] == 1 {
            Keyboard::RightAlt
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[4][11] == 1 {
            Keyboard::LeftArrow
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[4][12] == 1 {
            Keyboard::DownArrow
        } else {
            Keyboard::NoEventIndicated
        },
        if keys[4][13] == 1 {
            Keyboard::RightArrow
        } else {
            Keyboard::NoEventIndicated
        },
    ]
}

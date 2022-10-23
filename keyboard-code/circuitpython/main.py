import board

from kmk.kmk_keyboard import KMKKeyboard
from kmk.keys import KC
from kmk.scanners import DiodeOrientation

from kmk.modules.layers import Layers

keyboard = KMKKeyboard()
layers_ext = Layers()

fnc_key = KC.MO(1)

keyboard.col_pins = (board.GP13, board.GP14, board.GP15, board.GP12, board.GP11, board.GP10, board.GP9, board.GP8, board.GP2, board.GP3, board.GP4, board.GP5, board.GP6, board.GP7)
keyboard.row_pins = (board.GP20, board.GP19, board.GP18, board.GP17, board.GP16)
keyboard.diode_orientation = DiodeOrientation.COL2ROW

keyboard.keymap = [
    [
        KC.GESC, KC.N1, KC.N2, KC.N3, KC.N4, KC.N5, KC.N6, KC.N7, KC.N8, KC.N9, KC.N0, KC.MINS, KC.EQL, KC.BSPC,
        KC.TAB, KC.Q, KC.W, KC.E, KC.R, KC.T, KC.Y, KC.U, KC.I, KC.O, KC.P, KC.LBRC, KC.RBRC, KC.MPLY,
        KC.CAPS, KC.A, KC.S, KC.D, KC.F, KC.G, KC.H, KC.J, KC.K, KC.L, KC.SCLN, KC.QUOT, KC.ENT, KC.DEL,
        KC.LSHIFT, KC.NUBS, KC.Z, KC.X, KC.C, KC.V, KC.B, KC.N, KC.M, KC.COMM, KC.DOT, KC.SLASH, KC.NUHS, KC.UP,
        KC.LCTRL, KC.LCMD, KC.LALT, KC.NO, KC.NO, KC.NO, KC.SPC, KC.NO, KC.NO, KC.RALT, fnc_key, KC.LEFT, KC.DOWN, KC.RIGHT
    ],
    [
        KC.GRV, KC.F1, KC.F2, KC.F3, KC.F4, KC.F5, KC.F6, KC.F7, KC.F8, KC.F9, KC.F10, KC.F11, KC.F12, KC.TRNS,
        KC.TRNS, KC.TRNS, KC.TRNS, KC.TRNS, KC.TRNS, KC.TRNS, KC.TRNS, KC.TRNS, KC.TRNS, KC.TRNS, KC.TRNS, KC.TRNS, KC.TRNS, KC.TRNS,
        KC.TRNS, KC.TRNS, KC.TRNS, KC.TRNS, KC.TRNS, KC.TRNS, KC.TRNS, KC.TRNS, KC.TRNS, KC.TRNS, KC.TRNS, KC.TRNS, KC.TRNS, KC.TRNS,
        KC.TRNS, KC.TRNS, KC.TRNS, KC.TRNS, KC.TRNS, KC.TRNS, KC.TRNS, KC.TRNS, KC.TRNS, KC.TRNS, KC.TRNS, KC.TRNS, KC.TRNS, KC.TRNS,
        KC.TRNS, KC.TRNS, KC.TRNS, KC.TRNS, KC.TRNS, KC.TRNS, KC.TRNS, KC.TRNS, KC.TRNS, KC.TRNS, KC.TRNS, KC.TRNS, KC.TRNS, KC.TRNS
    ]
]

if __name__ == '__main__':
    keyboard.go()
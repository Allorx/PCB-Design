# PCB-Design
Keyboard design project -> "Orions Shoes"

Contains a custom mechanical keyboard PCB design (keyboard-pcb) and firmware (keyboard-code/rust-code).

![Alt text](PCB_OrionsHands.jpg?raw=true "Keyboard PCB")

## Key Features:
* NKRO support over USB using a RP Pico and Rust
* Custom layout with pushable rotary encoder (vol +/- and pause)

## IMPORTANT!
1) The schematic and pcb is wired wrong for the rotary encoder - should be flipped the other way around (may update this in future but a workaround by rewiring is required atm). I misunderstood the wiring of the rotary encoders so please check diagrams to wire this component correctly.

2) The rp pico usb micro-b connector is inaccesible after soldering - this is because the intention is to solder a connection for usb-c on the back of the rp pico as demonstrated here: [usb-c mod for rp pico](https://www.reddit.com/r/raspberry_pi/comments/m8p2ed/usb_type_c_mod_for_pico/). The PCB has a cutout to make these pads easily accessible. Alternative connectors may also be used.

# PCB-Design
Keyboard design project -> "Orions Shoes"

Contains a custom mechanical keyboard PCB design (keyboard-pcb) and firmware (keyboard-code/rust-code).

![Alt text](PCB_OrionsHands.jpg?raw=true "Keyboard PCB")

## Key Features:
* NKRO support over USB using a RP Pico and Rust
* Custom layout with pushable rotary encoder (vol +/- and pause)

## IMPORTANT!
NOTE: Rotary encoder schematic and pcb is wired wrong - should be flipped the other way around (may update this in future but a workaround by rewiring is required atm). I misunderstood the wiring of the rotary encoders so please check diagrams to wire this component correctly.

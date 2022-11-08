# PCB-Design
Keyboard design project -> "Orions Shoes"

Contains a custom mechanical keyboard PCB design (keyboard-pcb) and firmware (keyboard-code/rust-code).

![Alt text](PCB_OrionsHands.jpg?raw=true "Keyboard PCB")

## Key Features:
* NKRO support over USB using a RP Pico and Rust
* Custom layout with pushable rotary encoder consumer controls (vol +/- and pause)
* Highly customisable no compromises design

## IMPORTANT!
1) The schematic and pcb is wired wrong for the rotary encoder - should be flipped the other way around (may update this in future but a workaround by rewiring is required atm). I misunderstood (or used a rotary encoder diagram that is not common) the wiring of the rotary encoders so please check diagrams to wire this component correctly.

2) The rp pico usb micro-b connector is inaccesible after soldering - this is because the intention is to solder a connection for usb-c on the back of the rp pico as demonstrated here: [usb-c mod for rp pico](https://www.reddit.com/r/raspberry_pi/comments/m8p2ed/usb_type_c_mod_for_pico/). The connector can then be case mounted for more freedom of positioning. The PCB has a cutout to make these pads easily accessible. Alternative 4 wire connectors may also be used. If you would like to put the connector on the back of the keyboard it may be convenient to have a hole in the pcb for connector wires to go to the back rather than over the side of the pcb.

3) I would also recommend soldering the rp pico and usb connections first (and adding the firmware) so you can test the key matrix while soldering diodes and switches (ie to check if the diodes are the right way round).

## Inspiration and key resources
* [How keyboard switch matrix works](https://www.youtube.com/watch?v=vLGklanzQIc) - important for understanding how a keyboard works
* [Keyboard layout editor](http://www.keyboard-layout-editor.com/) - useful for figuring out your layout
* [How to make a simple keyboard PCB in KiCAD](https://wiki.ai03.com/books/pcb-design/page/pcb-guide-part-1---preparations) - good for general guidance and set up
* [Rust crate for rp-pico usb hid devices](https://github.com/dlkj/usbd-human-interface-device) - great crate for usb hid
* For better understanding of how it comes together I would also recommend reading: some documentation on the rp pico; about usb hid protocols; about mechanical switch debounce; about n-key roll over (NKRO) for usb hid keyboards.

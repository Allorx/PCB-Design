# Firmware for keyboard

The primary firmware is originally [rust-code](rust-code). The C version is for a bluetooth keyboard with rp pico w but it is still experimental.  

|Lang  |Folder                |Complete, Tested and Working?    |Features                 |Details                               |
|------|----------------------|:-------------------------------:|-------------------------|--------------------------------------|
|Rust  |[rust-code](rust-code)|&check;/?                        |rp pico + ssd1309 display|has some issues with col1 key ghosting|
|C     |[code](code)          |&cross;                          |rp pico w + bluetooth    |experimental                          |

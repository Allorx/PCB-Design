# Firmware for keyboard

The primary firmware is originally [rust-code](rust-code). The C version is for a bluetooth keyboard with rp pico w but it is still experimental and incomplete.  

|Lang  |Folder                |Complete, Tested and Working?                    |Features
|------|----------------------|-------------------------------------------------|-------------------------|
|Rust  |[rust-code](rust-code)|&check;/&cross; has issues with col1 key ghosting|rp pico + ssd1309 display|
|C     |[code](code)          |&cross;                                          |rp pico w + bluetooth    |

// Aleksas Girenas 23/10/2022
// Consumer control functions and assignments

use usbd_human_interface_device::page::Consumer;

// ? consumer controls
pub fn get_consumer(
    keys: [[i32; 14]; 5],
    rot_dir: i32,
    rot_released: bool,
    rot_can_push: bool,
) -> [Consumer; 1] {
    [if keys[1][13] == 0 && rot_released && rot_can_push {
        // rotary encoder has been released and was pressed and can be pushed (hasn't also been rotated)
        Consumer::PlayPause
    } else if keys[1][13] == 1 && rot_dir == 1 {
        // pushed and rotated
        Consumer::ScanNextTrack
    } else if keys[1][13] == 1 && rot_dir == -1 {
        // pushed and rotated
        Consumer::ScanPreviousTrack
    } else if rot_dir == 1 {
        // only rotated
        Consumer::VolumeIncrement
    } else if rot_dir == -1 {
        // only rotated
        Consumer::VolumeDecrement
    } else {
        Consumer::Unassigned
    }]
}

extern crate uinput;

use std::thread;
use std::time::Duration;

use uinput::event::Absolute::Multi;
use uinput::event::absolute::Multi::{PositionX,PositionY,Slot,TrackingId};
use uinput::event::Controller::Digi;
use uinput::event::controller::Digi::Touch;
use uinput::event::Event::Absolute;
use uinput::event::Event::Controller;

fn main() {
    // Initialize a device
    let mut device = uinput::default().unwrap()
        // Set the device name
        .name("Rust Touch Device").unwrap()
        // Now add the different events that will be reported
        // The first line says the device can report PositionX events as part
        // of an Absolute device of Multi-touch (MT) type
        .event(Absolute(Multi(PositionX))).unwrap()
        // For these events the bounds are
        .min(0).max(1000)
        // Same goes for PositionY
        .event(Absolute(Multi(PositionY))).unwrap()
        .min(0).max(1000)
        // Being an MT device it needs to report what Slot is using
        .event(Absolute(Multi(Slot))).unwrap()
        // In this case only one touch at the same time will be used
        .min(0).max(0)
        // Also a TrackingId needs to be reported to identify a contact
        .event(Absolute(Multi(TrackingId))).unwrap()
        // Finally, the BTN_TOUCH event needs to be send for the OS to actually
        // consider the touch. The used library puts this touch Event as part
        // of a Controller of type Digi, so that's what the following line
        // adds
        .event(Controller(Digi(Touch))).unwrap()
        // Once the device is set up properly, initialize it in the OS
        .create().unwrap();
 
    // Wait for the device to be fully initialized 
    thread::sleep(Duration::from_secs(1));

    // In this example, the touch can be always active, so set it as pressed
    // for the lifetime of the execution
    device.press(&Touch).unwrap();
 
    // Although only one touch is used in the example, the full MT protocol is
    // followed here, so we can extend the example later

    // Select the slot that will be used for the upcoming events
    device.position(&Slot,0).unwrap();

    // Set a contact to that slot
    device.position(&TrackingId,1).unwrap();
    // Horizontally center the touch in the device
    device.position(&PositionX, 500).unwrap();

    // Now move the touch vertically from top to bottom
    for i in 0..1000 {
        device.position(&PositionY, i).unwrap();
        // Send a SYNC so that the OS process the events
        device.synchronize().unwrap();
        // And wait 10ms in between to get a visible effect
        thread::sleep(Duration::from_millis(10));
    }

    // Finally release the contact
    device.position(&TrackingId,-1).unwrap();
    device.synchronize().unwrap();
 
}


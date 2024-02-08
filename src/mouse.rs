use input::{Event, Libinput, LibinputInterface};
use libc::{O_RDONLY, O_RDWR, O_WRONLY};
use std::fs::{File, OpenOptions};
use std::os::unix::{fs::OpenOptionsExt, io::OwnedFd};
use std::path::Path;
use input::event::EventTrait;
use input::event::pointer::ButtonState::Pressed;

struct Interface;

impl LibinputInterface for Interface {
    fn open_restricted(&mut self, path: &Path, flags: i32) -> Result<OwnedFd, i32> {
        OpenOptions::new()
            .custom_flags(flags)
            .read((flags & O_RDONLY != 0) | (flags & O_RDWR != 0))
            .write((flags & O_WRONLY != 0) | (flags & O_RDWR != 0))
            .open(path)
            .map(|file| file.into())
            .map_err(|err| err.raw_os_error().unwrap())
    }
    fn close_restricted(&mut self, fd: OwnedFd) {
        drop(File::from(fd));
    }
}

fn m_main() {
    use x11::xinput::XDevice;

    let mut input = Libinput::new_with_udev(Interface);
    input.udev_assign_seat("seat0").unwrap();
    loop {
        input.dispatch().unwrap();
        for event in &mut input {
            handle_left_click(&event);
            handle_movement(&event);
            // println!("Got event: {:?}", &event);
        }
    }
}

fn handle_movement(evt: &Event) {
    if let input::Event::Pointer(input::event::PointerEvent::Motion(move_evt)) = evt {
        println!("moved: {}:{}", move_evt.absolute_x(), move_evt.absolute_y())
    }
}

fn handle_left_click(event: &Event) {
    if let input::Event::Pointer(input::event::PointerEvent::Button(button_event)) = event {
        let btn = button_event.button();
        let state = button_event.button_state();
        println!("{}", btn);
        println!("{}", state == Pressed);
        if // button_event.button() == 0 &&
        button_event.button_state() == input::event::pointer::ButtonState::Pressed {
            println!("Left click detected!");
            // Handle the left click event here
        }
    }
}
use serde::{Serialize, Deserialize};
use ssh2::Session;
use std::io::prelude::*;
use std::net::TcpStream;
use ring::aead::{Aad, ChaCha20Poly1305};

// Define the Command structure with serde attributes
#[derive(Serialize, Deserialize)]
pub enum Device {
    Keyboard,
    Mouse
}

#[derive(Serialize, Deserialize)]
pub struct Input {
    pub movement: Option<Movement>,
    pub code: u16
}

#[derive(Serialize, Deserialize)]
pub struct Movement {
    pub x: i64,
    pub y: i64
}

#[derive(Serialize, Deserialize)]
pub struct Command {
    pub device: Device,
    pub input: Input,
}

fn main_ssh() {
    // Establish SSH connection
    let tcp = TcpStream::connect("example.com:22").unwrap();
    let mut sess = Session::new().unwrap();
    sess.handshake(&tcp).unwrap();
    sess.userauth_password("username", "password").unwrap();

    // Create a Command instance
    let command = Command {
        device: Device::Keyboard,
        input: Input {
            movement: None,
            code: 65, // ASCII code for 'A'
        },
    };

    // Serialize the Command to bytes
    let command_bytes = serde_json::to_vec(&command).unwrap();

    // Generate a random key for ChaCha20Poly1305
    let key = ChaCha20Poly1305::new_unique();

    // Encrypt the Command bytes using ChaCha20Poly1305
    let aad = b"command"; // Authenticated data (optional)
    let ciphertext = key.encrypt_in_place(aad, &mut command_bytes).unwrap();

    // Send the encrypted Command over SSH
    let mut channel = sess.channel_session().unwrap();
    channel.exec("receiver_script.sh").unwrap(); // Script to handle decryption and execution
    channel.write_all(&[aad.len() as u8]).unwrap(); // Send aad length
    channel.write_all(&aad).unwrap(); // Send aad
    channel.write_all(&ciphertext).unwrap();
    channel.send_eof().unwrap();

    // Wait for command execution and receive output
    let mut output = Vec::new();
    channel.read_to_end(&mut output).unwrap();

    // Handle output as needed
    println!("Output: {}", String::from_utf8_lossy(&output));
}
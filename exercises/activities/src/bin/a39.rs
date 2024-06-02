// Topic: Channels
//
// Summary:
//   Using the existing code, create a program that simulates an internet-of-things
//   remote control light bulb. The color of the light can be changed remotely.
//   Use threads and channels to communicate what color the light bulb should display.
//
// Requirements:
// * Create a separate thread representing the light bulb
// * Use a channel to communicate with the thread
// * Display a color change message using the println! macro
// * The light bulb must also be able to turn on and off
//   * Display whether the light is on or off on each color change
// * Turn off the light when disconnecting from it
//
// Notes:
// * Remember to add `crossbeam-channel` to your Cargo.toml file
// * Use the `colored` crate if you want to get fancy and display actual colors
// * The docs.rs site can be used to read documentation for third-party crates
// * Disconnection can be accomplished by dropping the sender, or
//   by telling the thread to self-terminate
// * Use `cargo test --bin a39` to test your program to ensure all cases are covered

use colored::*;
use crossbeam_channel::{unbounded, Receiver};
use std::thread::{self, JoinHandle};
#[derive(Debug)]
enum LightMsg {
    // Add additional variants needed to complete the exercise
    ChangeColor(u8, u8, u8),
    Disconnect,
    Off,
    On,
}

#[derive(Debug)]
enum LightStatus {
    Off,
    On,
}

// Create a separate thread representing the light bulb
// The function below creates a thread == light bulb
// we internally use message passing via channels inside thread to change it's value
fn spawn_light_thread(receiver: Receiver<LightMsg>) -> JoinHandle<LightStatus> {
    // Add code here to spawn a thread to control the light bulb
    thread::spawn(move || {
        let mut light_status = LightStatus::Off;
        loop{
            if let Ok(msg) = receiver.recv(){
                match msg{
                    LightMsg::ChangeColor(r,g,b ) => {
                        println!("color changed to {}", "          ".on_truecolor(r,g,b));
                        match light_status {
                            LightStatus::Off => println!("Light is OFF"),
                            LightStatus::On => println!("Light is ON"),                            
                        }
                    },
                    LightMsg::On => {
                        println!("Light is Turned on");
                        light_status = LightStatus::On;
                    },
                    LightMsg::Off => {
                        println!("Light is Turned off");
                        light_status = LightStatus::Off;
                    },
                    LightMsg::Disconnect => {
                        println!("Light is Gracefully Disconnected - The channel is shutting down");
                        light_status = LightStatus::Off;
                        break; // this will cause our loop to terminate which will end this channel
                    },
                }
            }
            else{
                println!("This is the Err() with the channel and hence the channel is terminated disgracefully");
                light_status = LightStatus::Off;
                break;
            }
        }
        light_status // returning back the light status at end of the thread to main thread
    })
}

fn main() {
    let (s, r) = unbounded(); // unbounded channel
    let light_bulb_thread = spawn_light_thread(r);

    // send some messages
    s.send(LightMsg::On);
    s.send(LightMsg::ChangeColor(255, 255, 0));
    s.send(LightMsg::ChangeColor(0, 128, 0));
    s.send(LightMsg::ChangeColor(0, 0, 255));
    s.send(LightMsg::Off);
    s.send(LightMsg::Disconnect); // Graceful Disconnect

    // later we'll join bulb thread to main thread
    let light_status_from_thread = light_bulb_thread.join();
    println!("🚀 ~ file: a39.rs ~ line 97 ~ fnmain ~ light_status_from_thread {:?}", light_status_from_thread);

}

#[cfg(test)]
mod test {
    use super::*;
    use crossbeam_channel::unbounded;

    #[test]
    fn light_off_when_disconnect() {
        let (s, r) = unbounded();

        let light = spawn_light_thread(r);
        s.send(LightMsg::Disconnect).expect("channel disconnected");

        let light_status = light.join().expect("failed to join light thread");

        if let LightStatus::On = light_status {
            panic!("light should be off after disconnection");
        }
    }

    #[test]
    fn light_off_when_dropped() {
        let (s, r) = unbounded();

        let light = spawn_light_thread(r);
        drop(s);

        let light_status = light.join().expect("failed to join light thread");

        if let LightStatus::On = light_status {
            panic!("light should be off after dropping sender");
        }
    }
}

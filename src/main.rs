use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode}
};
use std::thread;
use std::process::Command;
use std::time::{
    self,
    Duration
};
use std::thread::sleep;

fn main() {
    enable_raw_mode().expect("Could not enable raw mode");
    // Maybe I'll make this a real animation at some point... 
    let bee_a = 
    [" "," "," "," "," "," "," "," "," "," "," "," "," "," "," "," ",".","'"," ","'","."," "," "," "," "," "," "," "," "," "," "," "," ","_","_","\n", //35 -2 = 33
    " "," "," "," "," "," "," ","."," "," "," "," "," "," "," "," ","."," "," "," ","."," "," "," "," "," "," "," "," "," "," "," ","(","_","_","\\","_","\n", //73 -5 = 68
    " "," "," "," "," "," "," "," ","."," "," "," "," "," "," "," "," "," ","."," "," "," "," "," "," "," "," "," ","."," ","-","{","{","_","(","|","8",")","\n", //112 - 7 =105
    " "," "," "," "," "," "," "," "," "," ","'"," ","."," "," ","."," ","'"," ","'"," ","."," "," ","."," ","'"," "," "," "," "," ","(","_","_","/","\n","W","O","R","K","E","R","_","B","E","E","\n"]; //149 - 4 = 145
    
    let bee_b = 
    [" "," "," "," "," "," "," "," "," "," "," "," "," "," "," "," ",".","'"," ","'","."," "," "," "," "," "," "," "," "," "," "," "," ","_","_","\n", //35 -2 = 33
    " "," "," "," "," "," "," ","."," "," "," "," "," "," "," "," ","."," "," "," ","."," "," "," "," "," "," "," "," "," "," "," ","(","_","_","\\","_","\n", //73 -5 = 68
    " "," "," "," "," "," "," "," ","."," "," "," "," "," "," "," "," "," ","."," "," "," "," "," "," "," "," "," ","."," ","-","{","{","_","(","|","8",")","\n", //112 - 7 =105
    " "," "," "," "," "," "," "," "," "," ","'"," ","."," "," ","."," ","'"," ","'"," ","."," "," ","."," ","'"," "," "," "," "," ","(","_","_","/","\n","B","U","S","Y","_","B","E","E","\n"]; //149 - 4 = 145
    
    // print!("\n");
    // let quit:bool = false;
    let mut i: i32 = 0;
    let mut action = "help";
    let t = time::Duration::from_millis(1000);
    loop {
        if event::poll(Duration::from_millis(100)).expect("msg"){
            if let Event::Key(key_event) = event::read().expect("msg") {
                match key_event.code {
                    KeyCode::Char('1') => {
                        // set back to worker bee
                        action = "worker_bee";
                    },
                    KeyCode::Char('2') => {
                        // set to busy_bee
                        action = "busy_bee";
                    },
                    KeyCode::Char('h') => {
                        // set to help
                        action = "help";
                    },
                    KeyCode::Char('q') => {
                        println!("Exiting...");
                        break;
                    },
                    // default
                    _ => {}
                }
            }
        }

        // set cursor back to row 1 col 1
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

        // This can be:
        // --set-presence-to-available
        // --set-presence-to-dnd
        // --set-presence-to-be-right-back
        // --set-presence-to-away
        // --set-presence-to-offline
        // --reset-presence
        //
        // At some point, I'll add something to let you change this while running. 
        // E.g., keypress 1 = available, 2 = dnd, etc.
        match action {
            // worker_bee is set by default
            "worker_bee" => {
                // different bees for different bees
                for bee in bee_a {
                    print!("{bee}");
                }
                // print!("\nWORKER_BEE");

                Command::new("cmd")
                    .args(["/C", "start ms-teams.exe --set-presence-to-available"])
                    .output()
                    .expect("failed to execute process");
            },
            "busy_bee" =>{
                // different bees for different bees
                for bee in bee_b {
                    print!("{bee}");
                }
                // print!("\nBUSY_BEE");

                Command::new("cmd")
                    .args(["/C", "start ms-teams.exe --set-presence-to-dnd"])
                    .output()
                    .expect("failed to execute process");          
            },
            "help" => {
                // help menu
                print!("Little bees that help you work! \nOptions:\n\t1...worker_Bee who keeps your status green!\n\t2...busy_bee who keeps your status red! (DND)\n\th...display this menu!\n\tq...quit the application!\n")
            },
            //default
            _ => ()
        }

        
        // This might need to be adjusted at some point, idk. 
        sleep(t);
        
        i+=1;
    }
}

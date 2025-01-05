use std::collections::btree_map::Range;
use std::process::Command;
use std::time;
use std::thread::sleep;

fn main() {
    // Maybe I'll make this a real animation at some point... 
    let bee_a = 
    [" "," "," "," "," "," "," "," "," "," "," "," "," "," "," "," ",".","'"," ","'","."," "," "," "," "," "," "," "," "," "," "," "," ","_","_","\n", //35 -2 = 33
    " "," "," "," "," "," "," ","."," "," "," "," "," "," "," "," ","."," "," "," ","."," "," "," "," "," "," "," "," "," "," "," ","(","_","_","\\","_","\n", //73 -5 = 68
    " "," "," "," "," "," "," "," ","."," "," "," "," "," "," "," "," "," ","."," "," "," "," "," "," "," "," "," ","."," ","-","{","{","_","(","|","8",")","\n", //112 - 7 =105
    " "," "," "," "," "," "," "," "," "," ","'"," ","."," "," ","."," ","'"," ","'"," ","."," "," ","."," ","'"," "," "," "," "," ","(","_","_","/","\n"]; //149 - 4 = 145
    // print!("\n");
    let quit:bool = false;
    let mut i: i32 = 0;
    while !quit {
        // set cursor back to row 1 col 1
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        for bee in bee_a {
            print!("{bee}");
        }
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
        let t = time::Duration::from_millis(1000);
        Command::new("cmd")
            .args(["/C", "start ms-teams.exe --set-presence-to-available"])
            .output()
            .expect("failed to execute process");
        // This might need to be adjusted at some point, idk. 
        sleep(t);
        
        i+=1;
    }
}

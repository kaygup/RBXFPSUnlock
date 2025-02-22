use std::process::Command;
use std::path::Path;
use std::io::Write;
use std::env;
use std::io;
use std::fs;

fn main() {
    Command::new("cmd").args(["/C", "title RBXFPSUnlock"]).output().expect("Error setting title");
    print!("Use Unlocked FPS? (y, n) ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error reading line");
    if input.trim() == "y" {
        print!("FPS: ");
        io::stdout().flush().unwrap();

        let mut fpsinput = String::new();
        io::stdin().read_line(&mut fpsinput).expect("error reading line");

        write_new_roblox_fps(String::from(fpsinput.trim()));
        main();
    }
    if input.trim() == "n" {
        rem_client_setting();
        main();
    }
    println!("Invalid input.");
}

fn get_roblox_loc() -> String {
    if let Ok(path) = env::var("APPDATA") {
        let topath = Path::new(&path);
        let parent_path = topath.ancestors().nth(1).unwrap();
        let roblox_version_path = format!(r"{}\Local\Roblox\Versions", parent_path.display().to_string());

        let rvp_path = Path::new(&roblox_version_path);
        if rvp_path.exists() {
            for file in fs::read_dir(rvp_path).unwrap() {
                let rbxlpath = file.unwrap().path();
                let rbxlapppath = format!(r"{}\RobloxPlayerLauncher.exe", rbxlpath.display());
                if Path::new(&rbxlapppath).exists() {
                    return rbxlpath.display().to_string();
                }
            }

            return String::from("Error [Roblox not found]"); 
        } else {
            return String::from("Error [Roblox not found]");
        }

    } else {
        return String::from("Error [APPDATA not found]");
    }
}

fn rem_client_setting() {
    let rdir = get_roblox_loc();
    let clientsettings_path = format!(r"{}\ClientSettings", rdir);
    let clientsettings_dir = Path::new(&clientsettings_path);
    
    if clientsettings_dir.exists() {
        fs::remove_dir_all(clientsettings_dir).expect("Error while trying to remove client settings");
    }
}

fn write_new_roblox_fps(fps: String) {
    let rdir = get_roblox_loc();
    let clientsettings_path = format!(r"{}\ClientSettings", rdir);
    let clientsettings_dir = Path::new(&clientsettings_path);

    if !clientsettings_dir.exists() {
      match fs::create_dir(clientsettings_dir) {
        Ok(_) => println!(""),
        Err(e) => println!("Error Creating Directory! [ {e} ]"),
      };
    }

    let clientsettings = format!(
        r#"{{
            "DFIntTaskSchedulerTargetFps": {},
            "FFlagGameBasicSettingsFramerateCap5": false,
            "FFlagTaskSchedulerLimitTargetFpsTo2402": "False"
        }}"#,
        fps
    );
    fs::write(Path::new(&format!(r"{}\ClientSettings\ClientAppSettings.json", rdir)), clientsettings).expect("Error");
    println!("Set Max FPS to {fps}");
}

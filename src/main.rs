mod args;


use std::process::Command;

use args::YtVtArgs;
use clap::Parser;
fn main() {


    let args = YtVtArgs::parse();

    use directories::ProjectDirs;
    if let Some(proj_dirs) = ProjectDirs::from("com", "dex-archive",  "dex-archive") {
        proj_dirs.config_dir();
        // Linux:   /home/alice/.config/barapp
        // Windows: C:\Users\Alice\AppData\Roaming\Foo Corp\Bar App
        // macOS:   /Users/Alice/Library/Application Support/com.Foo-Corp.Bar-App
    }

    
    
}

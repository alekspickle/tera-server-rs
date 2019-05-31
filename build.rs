use std::process::Command;

fn main() {
    //use output directory
    //let out_dir = env::var("OUT_DIR").unwrap();
    
    // match OS constant for various platforms if needed:
    // match env::consts::OS{
    //     "windows" => {
    //      Command::new("mkdir").args(&["src/hello.c", "-c", "-fPIC", "-o"])
    //                    .arg("")
    //                    .status().unwrap();
    //     },
    //     (_) => {
    //         *some other platform specific ops*   
    // },
    // }

    //create 'downloads' directory before up & running
    Command::new("mkdir").arg("downloads")
                        .status().unwrap();
}
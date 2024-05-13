#![warn(clippy::pedantic)]
use std::io;
use std::fs::rename;
use std::path::PathBuf;

fn main(){
        println!("Debug Mode (default false):");

        let mut debug_mode = String::new();

        io::stdin()
                .read_line(&mut debug_mode)
                .expect("Failed to read line");

        let debug_mode: bool = debug_mode.trim().parse().unwrap_or_default();

        println!("You chose to have debug mode as {debug_mode}");

        println!("Please input the directory:");

        let mut directory: String = String::new();

        io::stdin()
                .read_line(&mut directory)
                .expect("Failed to read line");

        println!("Directory selected as {directory}");

        let directory = PathBuf::from(directory.trim());

        for (from, to, extension) in [
            ("partition-table.bin", "Trafic_Lights_4", ".ino.partitions.bin"),
            ("bootloader.bin", "bootloader_dio_80m", ".bin"),
            ("FHOSS_WALKWAY_ESP32.bin", "1", ".bin"),
        ] {
            let from = directory.join(from);
            let to = if debug_mode {
                format!("{to}_debug{extension}")
            } else {
                format!("{to}{extension}")
            };
            rename(from, to).expect("An error has eccoured.");
        }
}

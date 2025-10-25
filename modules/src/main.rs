mod player;

fn main() {
    player:: play_movie("Inception");
    player:: pause_movie();

    cleanup:: perform_cleanup();
    cleanup:: files:: cleanup_temp_files();
}

mod cleanup {
    pub fn perform_cleanup() {
        println!("Performing cleanup tasks...");
    }

    pub mod files {
        pub fn cleanup_temp_files() {
            println!("Cleaning up temporary files...");
        }
    }
}
use clap::CommandFactory;
use clap_complete::{
    generate_to,
    Shell::{Bash, Elvish, Fish, PowerShell, Zsh},
};

// Include the Cli struct.
include!("src/cli.rs");

fn main() {
    // Get directories.
    let root_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let man_dir = root_dir.join("man");
    let comp_dir = root_dir.join("completions");

    // Create directories if they don't exist.
    std::fs::create_dir_all(&man_dir).unwrap();
    std::fs::create_dir_all(&comp_dir).unwrap();

    // Setup clap command.
    let mut cmd = Cli::command();
    cmd.set_bin_name("dym");

    // Generate man page.
    let man = clap_mangen::Man::new(cmd.to_owned());
    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer).expect("Man page generation failed");
    std::fs::write(man_dir.join("dym.1"), buffer).expect("Failed to write man page");

    // Generate shell completions.
    for shell in [Bash, Elvish, Fish, PowerShell, Zsh] {
        generate_to(shell, &mut cmd, "dym", &comp_dir).unwrap();
    }
}

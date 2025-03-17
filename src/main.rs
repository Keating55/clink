use chrono::Local;
use clap::Parser;
use std::fs::hard_link;
// use std::os::windows::fs;
use std::path::PathBuf;
use std::process::Command;

#[derive(Parser)]
#[command(
    version,
    author,
    about,
    disable_version_flag = true,
    after_help(r#"clink -t [target_path]"#),
    help_template = "\
{name} {version}
\x1B[32mauthors\x1B[0m: {author}
\x1B[32mdescription\x1B[0m: {about-with-newline}
\x1B[32m{usage-heading}\x1B[0m {usage}

\x1B[32m{all-args}\x1B[0m
{after-help}


"
)]
struct Args {
    /// target path
    #[arg(short = 't', long)]
    target_path: String,
}

fn main() {
    let cli = Args::parse();
    let target_path = PathBuf::from(cli.target_path);
    let time = Local::now().format("%m%d-%H%M__").to_string();
    let dst_name = time + target_path.file_name().unwrap().to_str().unwrap();

    // driver_letter
    let driver_letter = target_path
        .components()
        .next()
        .unwrap()
        .as_os_str()
        .to_str()
        .unwrap();
    // link_dir
    let link_dir = PathBuf::from(driver_letter).join(r"\tmp");
    // link_path
    let link_path = link_dir.join(dst_name);
    // link
    if target_path.is_file() {
        hard_link(target_path, link_path).err();
        // fs::symlink_file(target_path, link_path).err();
    } else {
        // fs::symlink_dir(target_path, link_path).err();
        Command::new("cmd")
            .args(&[
                "/C",
                "mklink",
                "/D",
                link_path.to_str().unwrap(),
                target_path.to_str().unwrap(),
            ])
            .status()
            .expect("命令执行失败");
    }
}

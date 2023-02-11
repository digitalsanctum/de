use std::{env, fs};
use std::process::Command;

fn template() -> &'static str {
    r#"
[Desktop Entry]
Name={{NAME}}
Comment="{{NAME}} website"
Exec=google-chrome {{LINK}}
Icon=google-chrome
Terminal=false
Type=Application
    "#
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: {} <name> <link>", args[0]);
        return;
    }
    let name = args[1].clone();
    let mut link = args[2].clone();

    if !link.starts_with("http") {
        link = format!("https://{}", link);
    }

    let out = template().to_string()
        .replace("{{NAME}}", &*name)
        .replace("{{LINK}}", &*link)
        ;

    let tmp_file = format!("/tmp/{}.desktop", &*name);
    let err_msg = format!("Unable to write file: {}", &*tmp_file);
    fs::write(tmp_file.clone(), out).expect(&*err_msg);

    let final_path = format!("/usr/share/applications/{}.desktop", &*name);
    let mv_args = ["mv", &*tmp_file, &*final_path];

    let _moved = Command::new("sudo")
        .args(mv_args)
        .output()
        .expect("Failed to create desktop shortcut");
}

use std::{env, fs};
use std::process::Command;

fn chrome_template() -> &'static str {
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

    let out = chrome_template().to_string()
        .replace("{{NAME}}", &*name)
        .replace("{{LINK}}", &*link)
        ;

    let tmp_file = "/tmp/{{NAME}}.desktop".replace("{{NAME}}", &*name);
    let err_msg = "Unable to write file: {{FILE}}".replace("{{FILE}}", &*tmp_file);
    fs::write(tmp_file.clone(), out).expect(&*err_msg);
    let final_destination = "/usr/share/applications/{{NAME}}.desktop".replace("{{NAME}}", &*name);
    let mv_args = ["mv", &*tmp_file, &*final_destination];

    let _moved = Command::new("sudo")
        .args(mv_args)
        .output()
        .expect("Failed to create desktop shortcut");
}

use anyhow::Result;
use std::{
    fs::read_to_string,
    process::{Command, Output},
    str,
};

#[test]
fn regen() -> Result<()> {
    let regen = Command::new("./regen.sh")
        .current_dir("../test-project")
        .output()?;
    check_output("regen.sh", &regen);

    let run = Command::new("cargo")
        .args(["run"])
        .current_dir("../test-project")
        .output()?;
    check_output("cargo run", &run);

    let expect = r#"
bg-rose-500
hover
hover:bg-blue-50 text-white
"#;
    assert_eq!(str::from_utf8(&run.stdout)?.trim(), expect.trim());

    let css = read_to_string("../test-project/assets/tailwind_compiled.css")?;
    for expect in &[
        ".bg-blue-50 {",
        ".bg-rose-500 {",
        ".text-white {",
        ".hover\\:bg-blue-50:hover {",
    ] {
        assert!(css.contains(expect));
    }

    Ok(())
}

fn check_output(cmd: &str, output: &Output) {
    if !output.status.success() {
        println!("{cmd} failed:");
        if let Ok(stdout) = str::from_utf8(&output.stdout) {
            if !stdout.is_empty() {
                print!("stdout:\n{stdout}");
            }
        }
        if let Ok(stderr) = str::from_utf8(&output.stderr) {
            if !stderr.is_empty() {
                print!("stderr:\n{stderr}");
            }
        }
    }
    assert!(output.status.success());
}

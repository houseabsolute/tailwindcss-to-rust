use anyhow::{Context, Result};
use std::{
    fs::read_to_string,
    path::PathBuf,
    process::{Command, Output},
    str,
};

#[test]
fn regen() -> Result<()> {
    // The commands run below should match the ones in test-project/regen.sh.

    let home = home::home_dir()
        .expect("should be able to determine a home dir")
        .display()
        .to_string();
    let tailwindcss_exe = PathBuf::from_iter([&home, "bin", "tailwindcss"])
        .display()
        .to_string();
    let test_project_dir = PathBuf::from_iter(["..", "test-project"])
        .display()
        .to_string();

    run_command(&["cargo", "build", "-p", "tailwindcss-to-rust"], "..")?;

    let tailwindcss_to_rust_exe =
        PathBuf::from_iter(["..", "target", "debug", "tailwindcss-to-rust"])
            .display()
            .to_string();
    run_command(
        &[
            &tailwindcss_to_rust_exe,
            "--tailwind-config",
            "tailwind.config.js",
            "--input",
            "./css/tailwind.css",
            "--output",
            "./src/generated.rs",
            "--tailwindcss",
            &tailwindcss_exe,
            "--rustfmt",
        ],
        &test_project_dir,
    )?;

    run_command(
        &[
            &tailwindcss_exe,
            "--input",
            "./css/tailwind.css",
            "--output",
            "./assets/tailwind_compiled.css",
        ],
        &test_project_dir,
    )?;

    let run = run_command(&["cargo", "run"], &test_project_dir)?;

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

fn run_command(cmd: &[&str], cwd: &str) -> Result<Output> {
    let cstr = cmd.join(" ");
    let output = Command::new(cmd[0])
        .args(&cmd[1..])
        .current_dir(cwd)
        .output()
        .context(format!("running [{cstr}]"))?;
    check_output(&cstr, &output);
    Ok(output)
}

fn check_output(cstr: &str, output: &Output) {
    if !output.status.success() {
        println!("{cstr} failed:");
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

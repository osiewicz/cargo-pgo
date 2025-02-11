use crate::utils::{get_dir_files, init_cargo_project, run_command};

use crate::utils::OutputExt;

#[test]
fn test_instrument_create_pgo_profiles_dir() -> anyhow::Result<()> {
    let project = init_cargo_project()?;
    project.run(&["build"])?.assert_ok();

    assert!(project.default_pgo_profile_dir().is_dir());

    Ok(())
}

#[test]
fn test_instrument_run_instrumented_binary() -> anyhow::Result<()> {
    let project = init_cargo_project()?;
    project.run(&["build"])?.assert_ok();

    run_command(&project.main_binary())?;

    assert!(!get_dir_files(&project.default_pgo_profile_dir())?.is_empty());

    Ok(())
}

#[test]
fn test_optimize_no_profile() -> anyhow::Result<()> {
    let project = init_cargo_project()?;
    project.run(&["optimize"])?.assert_error();

    Ok(())
}

#[test]
fn test_build_optimize() -> anyhow::Result<()> {
    let project = init_cargo_project()?;

    project.run(&["build"])?.assert_ok();
    run_command(&project.main_binary())?;
    project.run(&["optimize"])?.assert_ok();
    run_command(&project.main_binary())?;

    Ok(())
}

#[test]
fn test_test_optimize() -> anyhow::Result<()> {
    let mut project = init_cargo_project()?;
    project.file(
        "src/lib.rs",
        r#"
pub fn foo(data: &[u32]) -> u32 {
    let mut sum = 0;
    for item in data {
        if *item > 5 {
            sum += *item;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use crate::foo;

    #[test]
    fn test_foo() {
        let data = &[0, 1, 2, 3, 4, 5, 6, 4, 3, 2, 1, 0];
        assert_eq!(foo(data), 6);
    }
}
"#,
    );

    project.run(&["test"])?.assert_ok();
    project.run(&["optimize"])?.assert_ok();
    run_command(&project.main_binary())?;

    Ok(())
}

#[test]
fn test_run_optimize() -> anyhow::Result<()> {
    let project = init_cargo_project()?;
    project.run(&["run"])?.assert_ok();
    project.run(&["optimize"])?.assert_ok();
    run_command(&project.main_binary())?;

    Ok(())
}

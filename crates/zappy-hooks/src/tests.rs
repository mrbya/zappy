#[test]
fn executes_required_hook() {
    let temp_dir = tempfile::TempDir::new().expect("tempdir should be created");

    let hook = zappy_core::hooks::HookSpec {
        name: Some(String::from("echo")),
        command: String::from("rustc"),
        args: vec![String::from("--version")],
        working_dir: None,
        env: indexmap::IndexMap::new(),
        when: None,
        optional: false,
        shell: false,
    };

    let input = crate::ExecuteHooksInput {
        phase: crate::HookPhase::PostGenerate,
        hooks: &[hook],
        output_dir: temp_dir.path(),
        variables: &zappy_core::ResolvedVariables::default(),
    };

    let summary = crate::execute_hooks(&input).expect("hook should run");

    assert_eq!(summary.executed, 1);
    assert_eq!(summary.skipped, 0);
}

#[test]
fn required_hook_failure_fails_execution() {
    let temp_dir = tempfile::TempDir::new().expect("tempdir should be created");

    let hook = zappy_core::hooks::HookSpec {
        name: Some(String::from("missing")),
        command: String::from("zappy-definitely-missing-command"),
        args: Vec::new(),
        working_dir: None,
        env: indexmap::IndexMap::new(),
        when: None,
        optional: false,
        shell: false,
    };

    let input = crate::ExecuteHooksInput {
        phase: crate::HookPhase::PostGenerate,
        hooks: &[hook],
        output_dir: temp_dir.path(),
        variables: &zappy_core::ResolvedVariables::default(),
    };

    let err = crate::execute_hooks(&input).expect_err("required hook failure should fail");

    assert!(err.to_string().contains("failed to spawn hook"));
}

#[test]
fn skips_false_conditional_hook() {
    let temp_dir = tempfile::TempDir::new().expect("tempdir should be created");

    let hook = zappy_core::hooks::HookSpec {
        name: Some(String::from("conditional")),
        command: String::from("rustc"),
        args: vec![String::from("--version")],
        working_dir: None,
        env: indexmap::IndexMap::new(),
        when: Some(String::from("run_hook")),
        optional: false,
        shell: false,
    };

    let input = crate::ExecuteHooksInput {
        phase: crate::HookPhase::PostGenerate,
        hooks: &[hook],
        output_dir: temp_dir.path(),
        variables: &zappy_core::ResolvedVariables::default(),
    };

    let summary = crate::execute_hooks(&input).expect("false conditional hook should skip");

    assert_eq!(summary.executed, 0);
    assert_eq!(summary.skipped, 1);
}

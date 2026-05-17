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

#[test]
fn executes_true_conditional_hook() {
    let temp_dir = tempfile::TempDir::new().expect("tempdir should be created");
    let mut variables = zappy_core::ResolvedVariables::default();

    variables.values.insert(
        String::from("run_hook"),
        zappy_core::VariableValue::Bool(true),
    );

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
        variables: &variables,
    };

    let summary = crate::execute_hooks(&input).expect("true conditional hook should run");

    assert_eq!(summary.executed, 1);
    assert_eq!(summary.skipped, 0);
}

#[test]
fn optional_hook_failure_is_reported_as_warning() {
    let temp_dir = tempfile::TempDir::new().expect("tempdir should be created");
    let hook = zappy_core::hooks::HookSpec {
        name: Some(String::from("optional missing")),
        command: String::from("zappy-definitely-missing-command"),
        args: Vec::new(),
        working_dir: None,
        env: indexmap::IndexMap::new(),
        when: None,
        optional: true,
        shell: false,
    };

    let input = crate::ExecuteHooksInput {
        phase: crate::HookPhase::PostGenerate,
        hooks: &[hook],
        output_dir: temp_dir.path(),
        variables: &zappy_core::ResolvedVariables::default(),
    };

    let summary = crate::execute_hooks(&input).expect("optional hook failure should not fail");

    assert_eq!(summary.executed, 0);
    assert_eq!(summary.optional_failed, 1);
    assert_eq!(summary.warnings.len(), 1);
    assert!(
        summary
            .warnings
            .first()
            .expect("warning should exist")
            .contains("optional missing")
    );
}

#[test]
fn non_zero_hook_exit_fails_with_output() {
    let temp_dir = tempfile::TempDir::new().expect("tempdir should be created");
    let hook = zappy_core::hooks::HookSpec {
        name: Some(String::from("bad rustc flag")),
        command: String::from("rustc"),
        args: vec![String::from("--definitely-not-a-rustc-flag")],
        working_dir: None,
        env: indexmap::IndexMap::new(),
        when: None,
        optional: false,
        shell: false,
    };

    let input = crate::ExecuteHooksInput {
        phase: crate::HookPhase::ValidationStep,
        hooks: &[hook],
        output_dir: temp_dir.path(),
        variables: &zappy_core::ResolvedVariables::default(),
    };

    let err = crate::execute_hooks(&input).expect_err("non-zero hook should fail");

    assert!(err.to_string().contains("bad rustc flag"));
    assert!(err.to_string().contains("failed"));
}

#[test]
fn shell_hooks_are_rejected_before_spawn() {
    let temp_dir = tempfile::TempDir::new().expect("tempdir should be created");
    let hook = zappy_core::hooks::HookSpec {
        name: Some(String::from("shell hook")),
        command: String::from("echo hi"),
        args: Vec::new(),
        working_dir: None,
        env: indexmap::IndexMap::new(),
        when: None,
        optional: false,
        shell: true,
    };

    let input = crate::ExecuteHooksInput {
        phase: crate::HookPhase::PreGenerate,
        hooks: &[hook],
        output_dir: temp_dir.path(),
        variables: &zappy_core::ResolvedVariables::default(),
    };

    let err = crate::execute_hooks(&input).expect_err("shell hook should be rejected");

    assert!(err.to_string().contains("shell hook"));
    assert!(err.to_string().contains("not supported"));
}

#[test]
fn renders_command_args_env_and_working_dir() {
    let temp_dir = tempfile::TempDir::new().expect("tempdir should be created");
    let work_dir = temp_dir.path().join("generated");

    std::fs::create_dir_all(&work_dir).expect("working dir should be created");
    std::fs::write(work_dir.join("marker.txt"), "marker").expect("marker should be written");

    let mut env = indexmap::IndexMap::new();
    env.insert(String::from("ZAPPY_TEST_VALUE"), String::from("__VALUE__"));

    let mut replacements = indexmap::IndexMap::new();
    replacements.insert(String::from("__SHELL__"), String::from("sh"));
    replacements.insert(String::from("__DIR__"), String::from("generated"));
    replacements.insert(String::from("__VALUE__"), String::from("expected"));

    let variables = zappy_core::ResolvedVariables {
        values: zappy_core::VariableValueMap::new(),
        transformations: indexmap::IndexMap::new(),
        replacements,
    };

    let hook = zappy_core::hooks::HookSpec {
        name: Some(String::from("rendered")),
        command: String::from("__SHELL__"),
        args: vec![
            String::from("-c"),
            String::from("test \"$ZAPPY_TEST_VALUE\" = \"expected\" && test -f marker.txt"),
        ],
        working_dir: Some(camino::Utf8PathBuf::from("__DIR__")),
        env,
        when: None,
        optional: false,
        shell: false,
    };

    let input = crate::ExecuteHooksInput {
        phase: crate::HookPhase::PostGenerate,
        hooks: &[hook],
        output_dir: temp_dir.path(),
        variables: &variables,
    };

    let summary = crate::execute_hooks(&input).expect("rendered hook should run");

    assert_eq!(summary.executed, 1);
}

#[test]
fn invalid_rendered_working_dir_fails_before_spawn() {
    let temp_dir = tempfile::TempDir::new().expect("tempdir should be created");
    let mut replacements = indexmap::IndexMap::new();

    replacements.insert(String::from("__DIR__"), String::from("../outside"));

    let variables = zappy_core::ResolvedVariables {
        values: zappy_core::VariableValueMap::new(),
        transformations: indexmap::IndexMap::new(),
        replacements,
    };

    let hook = zappy_core::hooks::HookSpec {
        name: Some(String::from("bad working dir")),
        command: String::from("rustc"),
        args: vec![String::from("--version")],
        working_dir: Some(camino::Utf8PathBuf::from("__DIR__")),
        env: indexmap::IndexMap::new(),
        when: None,
        optional: false,
        shell: false,
    };

    let input = crate::ExecuteHooksInput {
        phase: crate::HookPhase::PostGenerate,
        hooks: &[hook],
        output_dir: temp_dir.path(),
        variables: &variables,
    };

    let err = crate::execute_hooks(&input).expect_err("invalid working dir should fail");

    assert!(err.to_string().contains("working directory"));
}

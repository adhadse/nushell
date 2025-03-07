use nu_test_support::fs::Stub::FileWithContentToBeTrimmed;
use nu_test_support::nu_with_plugins;
use nu_test_support::playground::Playground;

const TEST_CWD: &str = "tests/fixtures/formats";

#[test]
fn parses_ini() {
    let actual = nu_with_plugins!(
        cwd: TEST_CWD,
        plugin: ("nu_plugin_formats"),
        "open sample.ini | get SectionOne.integer"
    );

    assert_eq!(actual.out, "1234")
}

#[test]
fn parses_utf16_ini() {
    let actual = nu_with_plugins!(
        cwd: TEST_CWD,
        plugin: ("nu_plugin_formats"),
        "open ./utf16.ini --raw | decode utf-16 | from ini | rename info | get info | get IconIndex"
    );

    assert_eq!(actual.out, "-236")
}

#[test]
fn read_ini_with_missing_session() {
    Playground::setup("from ini with missiong session", |dirs, sandbox| {
        sandbox.with_files(vec![FileWithContentToBeTrimmed(
            "some_missing.ini",
            r#"
            min-width=450
            max-width=820
            [normal]
            sound-file=/usr/share/sounds/freedesktop/stereo/dialog-information.oga
            [critical]
            border-color=FAB387ff
            default-timeout=20
            sound-file=/usr/share/sounds/freedesktop/stereo/dialog-warning.oga
            "#,
        )]);

        let cwd = dirs.test();
        let actual = nu_with_plugins!(
            cwd: cwd,
            plugin: ("nu_plugin_formats"),
            r#"open some_missing.ini | get "".min-width "#
        );

        assert_eq!(actual.out, "450");
    })
}

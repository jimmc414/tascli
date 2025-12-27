/// Convert Linux path to Windows path for wt.exe
/// /mnt/c/python/myapp -> C:\python\myapp
pub fn linux_to_windows_path(linux_path: &str) -> Result<String, String> {
    if linux_path.starts_with("/mnt/") {
        let parts: Vec<&str> = linux_path.splitn(4, '/').collect();
        // parts = ["", "mnt", "c", "python/myapp"]
        if parts.len() >= 4 {
            let drive = parts[2].to_uppercase();
            let rest = parts[3].replace('/', "\\");
            return Ok(format!("{}:\\{}", drive, rest));
        } else if parts.len() == 3 {
            // Just the drive root: /mnt/c
            let drive = parts[2].to_uppercase();
            return Ok(format!("{}:\\", drive));
        }
    }
    Err(format!("Cannot convert path: {}", linux_path))
}

/// Build spawn command for Windows Terminal
/// Uses /init workaround for WSL interop execute permission issues
pub fn build_spawn_command(
    terminal_profile: &str,
    windows_path: &str,
    conda_env: Option<&str>,
    claude_flags: Option<&str>,
    prompt: Option<&str>,
) -> String {
    let mut bash_cmd = String::from("export PATH=\\$HOME/.local/bin:\\$PATH");

    if let Some(env) = conda_env {
        bash_cmd.push_str(&format!(" && conda activate {}", env));
    }

    bash_cmd.push_str(" && claude");

    if let Some(flags) = claude_flags {
        bash_cmd.push_str(&format!(" {}", flags));
    }

    if let Some(p) = prompt {
        // Escape quotes in prompt for nested shell quoting
        let escaped = p.replace('\"', "\\\\\\\"");
        bash_cmd.push_str(&format!(" \\\\\\\"{}\\\\\\\"", escaped));
    }

    format!(
        "/init /mnt/c/Windows/System32/cmd.exe /c \"wt.exe -p {} -d {} wsl.exe -e bash -c \\\"{}\\\"\"",
        terminal_profile,
        windows_path,
        bash_cmd
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linux_to_windows_path() {
        assert_eq!(
            linux_to_windows_path("/mnt/c/python/myapp").unwrap(),
            "C:\\python\\myapp"
        );
        assert_eq!(
            linux_to_windows_path("/mnt/d/projects/foo/bar").unwrap(),
            "D:\\projects\\foo\\bar"
        );
        assert_eq!(
            linux_to_windows_path("/mnt/c").unwrap(),
            "C:\\"
        );
        assert!(linux_to_windows_path("/home/user/project").is_err());
        assert!(linux_to_windows_path("/usr/local/bin").is_err());
    }

    #[test]
    fn test_build_spawn_command_basic() {
        let cmd = build_spawn_command("Ubuntu", "C:\\python\\ctm", None, None, None);
        assert!(cmd.contains("wt.exe -p Ubuntu"));
        assert!(cmd.contains("-d C:\\python\\ctm"));
        assert!(cmd.contains("export PATH="));
        assert!(cmd.contains("claude"));
    }

    #[test]
    fn test_build_spawn_command_with_options() {
        let cmd = build_spawn_command(
            "Ubuntu",
            "C:\\python\\myapp",
            Some("myapp-env"),
            Some("--dangerously-skip-permissions"),
            Some("Work on task: Fix the bug"),
        );
        assert!(cmd.contains("conda activate myapp-env"));
        assert!(cmd.contains("--dangerously-skip-permissions"));
        assert!(cmd.contains("Work on task: Fix the bug"));
    }
}

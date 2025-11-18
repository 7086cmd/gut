use std::path::Path;
use crate::colors::{error, warning};

pub fn load_config() -> serde_json::Value {
    let config_path = Path::new("gut.config.json");
    if !config_path.exists() { return serde_json::json!({}); }

    let config_str = match std::fs::read_to_string(config_path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{}", error(&format!("Failed to read gut.config.json: {}", e)));
            return serde_json::json!({});
        }
    };

    let config: serde_json::Value = match serde_json::from_str(&config_str) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}", error(&format!("Invalid JSON in gut.config.json: {}", e)));
            eprintln!("Using default configuration.");
            return serde_json::json!({});
        }
    };

    // Validate configuration
    validate_config(&config);

    config
}

fn validate_config(config: &serde_json::Value) {
    let mut warnings = Vec::new();

    // Validate commit config
    if let Some(commit_cfg) = config.get("commit") {
        if let Some(format_mode) = commit_cfg.get("format_mode").and_then(|v| v.as_str()) {
            if format_mode != "upper_case" && format_mode != "lower_case" {
                warnings.push(format!("Invalid format_mode '{}'. Valid values: 'upper_case', 'lower_case'", format_mode));
            }
        }

        if let Some(emoji_enabled) = commit_cfg.get("emoji_enabled") {
            if !emoji_enabled.is_boolean() {
                warnings.push("emoji_enabled should be a boolean (true/false)".to_string());
            }
        }

        if let Some(require_conv) = commit_cfg.get("require_conventional") {
            if !require_conv.is_boolean() {
                warnings.push("require_conventional should be a boolean (true/false)".to_string());
            }
        }
    }

    // Validate log config
    for log_type in ["log", "tlog"] {
        if let Some(log_cfg) = config.get(log_type) {
            if let Some(count) = log_cfg.get("count") {
                if !count.is_u64() {
                    warnings.push(format!("{}.count should be a positive number", log_type));
                }
            }

            if let Some(info) = log_cfg.get("info").and_then(|v| v.as_str()) {
                if info != "less" && info != "more" {
                    warnings.push(format!("{}.info should be 'less' or 'more'", log_type));
                }
            }
        }
    }

    // Validate hooks
    if let Some(hooks) = config.get("hooks").and_then(|v| v.as_array()) {
        for (i, hook) in hooks.iter().enumerate() {
            if hook.get("name").is_none() {
                warnings.push(format!("Hook #{} is missing 'name' field", i + 1));
            }
            if hook.get("commands").is_none() {
                warnings.push(format!("Hook #{} is missing 'commands' field", i + 1));
            }
        }
    }

    // Display warnings
    for warn in &warnings {
        eprintln!("{}", warning(&format!("Config warning: {}", warn)));
    }
}

pub fn check_and_generate_hooks(config: &serde_json::Value) {
    let hooks = config.get("hooks").and_then(|v| v.as_array());
    let git_hooks_dir = Path::new(".git/hooks");
    if hooks.is_none() || !git_hooks_dir.exists() { return; }
    for hook in hooks.unwrap() {
        let name = hook.get("name").and_then(|v| v.as_str());
        let condition = hook.get("condition").and_then(|v| v.as_str()).unwrap_or("");
        let commands = hook.get("commands").and_then(|v| v.as_array());
        if let (Some(name), Some(commands)) = (name, commands) {
            let mut script = String::from("#!/bin/sh\nset -e\n");
            if !condition.is_empty() {
                script.push_str(&format!("if ! ({}); then exit 0; fi\n", condition));
            }
            for cmd in commands {
                if let Some(cmd_str) = cmd.as_str() {
                    script.push_str(cmd_str);
                    script.push('\n');
                }
            }
            let hook_path = git_hooks_dir.join(name);
            if !hook_path.exists() || std::fs::read_to_string(&hook_path).ok().as_deref() != Some(&script) {
                if let Ok(mut f) = std::fs::File::create(&hook_path) {
                    use std::io::Write;
                    let _ = f.write_all(script.as_bytes());
                    #[cfg(unix)]
                    {
                        use std::os::unix::fs::PermissionsExt;
                        let _ = std::fs::set_permissions(&hook_path, std::fs::Permissions::from_mode(0o755));
                    }
                    // No permission setting for Windows; not needed
                }
            }
        }
    }
}

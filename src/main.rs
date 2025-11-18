mod config;
mod utils;
mod commit;
mod log;
mod colors;

use config::{load_config, check_and_generate_hooks};
use utils::{pass_to_git};
use commit::gut_commit;
use log::{gut_log, gut_rlog, gut_tlog};
use colors::{success, error, warning, info, bold, highlight};
use std::env;

fn gut_branch(args: &[String]) {
    if args.is_empty() {
        eprintln!("gut branch <branch-name>");
        std::process::exit(1);
    }
    let branch = &args[0];
    pass_to_git(&["checkout".to_string(), "-b".to_string(), branch.clone()]);
}

fn gut_template(args: &[String]) {
    use std::fs;
    if args.is_empty() {
        eprintln!("{}", error("gut template <template-repo-url> [dest]"));
        std::process::exit(1);
    }
    let url = &args[0];
    let dest = args.get(1).map(|s| s.as_str()).unwrap_or(".");
    let status = std::process::Command::new("git").args(["clone", url, dest]).status().expect("failed to clone");
    if !status.success() { std::process::exit(1); }
    let git_dir = format!("{}/.git", dest);
    if fs::remove_dir_all(&git_dir).is_err() {
        eprintln!("{}", error("Failed to remove .git directory"));
        std::process::exit(1);
    }
    let status = std::process::Command::new("git").current_dir(dest).args(["init"]).status().expect("failed to re-init");
    if !status.success() { std::process::exit(1); }
    println!("{}", success(&format!("Template repo initialized at {}", dest)));
}

fn gut_undo(args: &[String]) {
    // Interactive undo helper - provides common undo operations
    if args.is_empty() {
        println!("{}", bold("gut undo - Smart undo operations"));
        println!("\nAvailable options:");
        println!("  {}  - Undo last commit (keep changes)", highlight("commit"));
        println!("  {}  - Undo last commit (discard changes)", highlight("commit-hard"));
        println!("  {}    - Unstage all files", highlight("stage"));
        println!("  {} - Discard all working changes", highlight("changes"));
        println!("\nUsage: gut undo <option>");
        std::process::exit(1);
    }

    match args[0].as_str() {
        "commit" => {
            println!("{}", info("Undoing last commit (keeping changes)..."));
            pass_to_git(&["reset".to_string(), "--soft".to_string(), "HEAD~1".to_string()]);
            println!("{}", success("Last commit undone. Changes kept in staging area."));
        }
        "commit-hard" => {
            println!("{}", warning("This will discard the last commit and all its changes!"));
            println!("Type 'yes' to confirm: ");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            if input.trim() == "yes" {
                pass_to_git(&["reset".to_string(), "--hard".to_string(), "HEAD~1".to_string()]);
                println!("{}", success("Last commit and changes discarded."));
            } else {
                println!("{}", info("Cancelled."));
            }
        }
        "stage" => {
            println!("{}", info("Unstaging all files..."));
            pass_to_git(&["reset".to_string(), "HEAD".to_string()]);
            println!("{}", success("All files unstaged."));
        }
        "changes" => {
            println!("{}", warning("This will discard ALL uncommitted changes!"));
            println!("Type 'yes' to confirm: ");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            if input.trim() == "yes" {
                pass_to_git(&["reset".to_string(), "--hard".to_string()]);
                pass_to_git(&["clean".to_string(), "-fd".to_string()]);
                println!("{}", success("All changes discarded."));
            } else {
                println!("{}", info("Cancelled."));
            }
        }
        _ => {
            eprintln!("{}", error(&format!("Unknown undo option: {}", args[0])));
            eprintln!("Run 'gut undo' to see available options.");
            std::process::exit(1);
        }
    }
}

fn gut_save(args: &[String]) {
    // Alias for git stash with a better name
    let message = if args.is_empty() {
        None
    } else {
        Some(args.join(" "))
    };

    if let Some(msg) = message {
        println!("{}", info(&format!("Saving changes: {}", msg)));
        pass_to_git(&["stash".to_string(), "push".to_string(), "-m".to_string(), msg]);
    } else {
        println!("{}", info("Saving all changes..."));
        pass_to_git(&["stash".to_string(), "push".to_string()]);
    }
    println!("{}", success("Changes saved. Use 'gut pop' to restore."));
}

fn gut_pop(_args: &[String]) {
    // Alias for git stash pop
    println!("{}", info("Restoring saved changes..."));
    pass_to_git(&["stash".to_string(), "pop".to_string()]);
    println!("{}", success("Changes restored."));
}

fn gut_remove_committed(args: &[String]) {
    // Remove accidentally committed files (like API keys) and recommit
    if args.is_empty() {
        println!("{}", bold("gut remove-committed - Remove files from last commit"));
        println!("\nThis command helps when you accidentally commit sensitive files (API keys, .env, etc.)");
        println!("\nUsage: gut remove-committed <file1> [file2] [file3]...");
        println!("\nExample: gut remove-committed .env api_keys.txt");
        println!("\n{}:", bold("What this does"));
        println!("  1. Gets the last commit message");
        println!("  2. Resets the last commit (keeping changes)");
        println!("  3. Removes specified files from git tracking");
        println!("  4. Re-commits with the original message (without the removed files)");
        std::process::exit(1);
    }

    println!("{}", warning("Removing files from last commit..."));

    // Get the last commit message
    let output = std::process::Command::new("git")
        .args(["log", "-1", "--pretty=%B"])
        .output()
        .expect("Failed to get last commit message");

    if !output.status.success() {
        eprintln!("{}", error("Failed to get last commit message. Are you in a git repository?"));
        std::process::exit(1);
    }

    let commit_msg = String::from_utf8_lossy(&output.stdout).trim().to_string();
    println!("{}", info(&format!("Original commit message: {}", commit_msg)));

    // Reset the last commit (soft)
    println!("{}", info("Resetting last commit (keeping changes)..."));
    pass_to_git(&["reset".to_string(), "--soft".to_string(), "HEAD~1".to_string()]);

    // Remove the specified files from git tracking
    for file in args {
        println!("{}", info(&format!("Removing {} from git...", file)));
        let status = std::process::Command::new("git")
            .args(["rm", "--cached", file])
            .status()
            .expect("Failed to remove file");

        if !status.success() {
            eprintln!("{}", warning(&format!("Could not remove {} (might not be in git)", file)));
        }
    }

    // Check if there are still changes to commit
    let status_output = std::process::Command::new("git")
        .args(["diff", "--cached", "--quiet"])
        .status()
        .expect("Failed to check git status");

    if status_output.success() {
        println!("{}", warning("No changes left to commit after removing files."));
        println!("{}", info("All files from the commit were removed."));
        return;
    }

    // Re-commit with the original message
    println!("{}", info("Re-committing without the removed files..."));
    pass_to_git(&["commit".to_string(), "-m".to_string(), commit_msg]);

    println!("{}", success("Files successfully removed from last commit!"));
    println!("\n{}", bold("Important reminders:"));
    println!("  • Add removed files to .gitignore to prevent re-committing");
    println!("  • If you already pushed, you may need to force push (use with caution)");
    println!("  • Consider rotating any exposed secrets/API keys");
}

fn main() {
    let config = load_config();
    check_and_generate_hooks(&config);
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        println!("\n{}", bold("Gut - A smarter Git CLI"));
        println!("\n{}: gut <command> [args...]", bold("Usage"));
        println!("\n{}", bold("Smart Features:"));
        println!("  {} Auto-infer git subcommands from abbreviations or typos", highlight("•"));
        println!("  {} Interactive commit mode - just run '{}'", highlight("•"), highlight("gut commit"));
        println!("  {} Auto-format conventional commits with emojis", highlight("•"));
        println!("  {} Config-driven hooks and customization", highlight("•"));

        println!("\n{}", bold("Enhanced Commands:"));
        println!("  {}        - Interactive or direct commit with formatting", highlight("commit"));
        println!("  {}         - Smart undo operations (commit, stage, changes)", highlight("undo"));
        println!("  {} / {}          - Save/restore changes (better stash)", highlight("save"), highlight("pop"));
        println!("  {} - Remove sensitive files from last commit", highlight("remove-committed"));
        println!("  {}        - Create branch and switch to it", highlight("branch"));
        println!("  {}    / {}   - Configurable commit logs", highlight("log"), highlight("rlog"));
        println!("  {}         - Tree log showing all branches", highlight("tlog"));
        println!("  {}     - Clone repo as template (remove .git, re-init)", highlight("template"));

        println!("\n{}", bold("Examples:"));
        println!("  {}                      - Interactive commit", highlight("gut commit"));
        println!("  {}        - Quick commit with formatting", highlight("gut commit 'feat: add login'"));
        println!("  {}                  - Undo last commit (keep changes)", highlight("gut undo commit"));
        println!("  {}        - Save work in progress", highlight("gut save 'work in progress'"));
        println!("  {}          - Remove accidentally committed key file", highlight("gut remove-committed .env"));

        println!("\n{}", bold("Configuration:"));
        println!("  Create {} to customize behavior", highlight("gut.config.json"));
        println!("  Configure: commit format, emoji, hooks, log display, and more");

        println!("\n{}", bold("More Info:"));
        println!("  https://github.com/elliot-zzh/gut");
        println!();
        std::process::exit(1);
    }
    let sub = &args[0];
    // Find the subcommand with the shortest Levenshtein distance (including gut-only commands)
    const ALL_COMMANDS: &[&str] = &[
        "template", "rlog", "tlog", "undo", "save", "pop", "remove-committed",
        "init", "clone", "add", "commit", "restore", "rm", "mv", "status", "log", "diff", "show", "branch", "checkout", "merge", "rebase", "fast-forward", "tag", "stash", "pull", "fetch", "push", "remote", "submodule", "reset", "revert", "clean", "gc", "fsck", "archive", "blame", "bisect", "cherry-pick", "config", "help"
    ];
    let mut min_dist = usize::MAX;
    let mut best_cmd = None;
    for &cmd in ALL_COMMANDS {
        let dist = utils::levenshtein(sub, cmd);
        if dist < min_dist {
            min_dist = dist;
            best_cmd = Some(cmd);
        }
    }
    if let Some(cmd) = best_cmd {
        if min_dist > 3 {
            // Fallback: pass to git
            pass_to_git(&args);
            return;
        }
        if min_dist >= 1 {
            println!("[gut] subcommand smart infer: {} \x1b[32m=>\x1b[0m {}", sub, cmd);
        }
        match cmd {
            "template" => gut_template(&args[1..]),
            "rlog" => gut_rlog(&args[1..], &config),
            "tlog" => gut_tlog(&args[1..], &config),
            "undo" => gut_undo(&args[1..]),
            "save" => gut_save(&args[1..]),
            "pop" => gut_pop(&args[1..]),
            "remove-committed" => gut_remove_committed(&args[1..]),
            "commit" => gut_commit(&args[1..], &config),
            "branch" => gut_branch(&args[1..]),
            "log" => gut_log(&args[1..], &config),
            _ => {
                let mut git_args = vec![cmd.to_string()];
                git_args.extend(args[1..].iter().cloned());
                pass_to_git(&git_args);
            }
        }
        return;
    }
    // Fallback: pass to git
    pass_to_git(&args);
}

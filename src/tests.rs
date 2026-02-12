use super::generate_markdown;
use std::path::PathBuf;
use toad_core::{ActivityTier, ProjectDetail, VcsStatus};

#[test]
fn test_generate_markdown_basic() {
    let projects = vec![ProjectDetail {
        name: "test-proj".to_string(),
        path: PathBuf::from("projects/test-proj"),
        stack: "Rust".to_string(),
        activity: ActivityTier::Active,
        vcs_status: VcsStatus::Clean,
        essence: Some("Test essence".to_string()),
        tags: vec!["#test".to_string()],
        taxonomy: vec!["#test".to_string()],
        artifact_dirs: vec!["target".to_string()],
        sub_projects: Vec::new(),
        submodules: Vec::new(),
        source: toad_core::TargetSource::PondProject,
        total_size: 0,
        bloat_index: 0.0,
    }];

    let md = generate_markdown(&projects, 12345, None);
    assert!(md.contains("**Fingerprint:** `12345`"));
    assert!(md.contains(
        "| **`test-proj`** | `Rust` | 🔥 Active | ✅ Clean | Test essence | `[#test]` |"
    ));
}

#[test]
fn test_generate_markdown_empty() {
    let md = generate_markdown(&[], 0, None);
    assert!(md.contains("**Fingerprint:** `0`"));
    assert!(md.contains(
        "| Project | Stack | Activity | VCS | Essence (Extractive) | Taxonomy (Ingredients) |"
    ));
}

#[test]
fn test_generate_markdown_escaping() {
    let projects = vec![ProjectDetail {
        name: "escape-test".to_string(),
        path: PathBuf::from("projects/escape-test"),
        stack: "Generic".to_string(),
        activity: ActivityTier::Archive,
        vcs_status: VcsStatus::None,
        essence: Some("Pipe | [Link] https://google.com".to_string()),
        tags: vec![],
        taxonomy: vec![],
        artifact_dirs: vec![],
        sub_projects: Vec::new(),
        submodules: Vec::new(),
        source: toad_core::TargetSource::PondProject,
        total_size: 0,
        bloat_index: 0.0,
    }];

    let md = generate_markdown(&projects, 999, None);
    // Pipe should be escaped
    assert!(md.contains("Pipe \\|"));
    // Brackets should be escaped
    assert!(md.contains("\\[Link\\]"));
    // URL should be escaped
    assert!(md.contains("https:\\\\/google.com"));
    // No taxonomy case
    assert!(md.contains("Generic"));
}

#[test]
fn test_generate_markdown_truncation() {
    let long_essence = "A".repeat(200);
    let projects = vec![ProjectDetail {
        name: "long-proj".to_string(),
        path: PathBuf::from("projects/long-proj"),
        stack: "Go".to_string(),
        activity: ActivityTier::Cold,
        vcs_status: VcsStatus::Dirty,
        essence: Some(long_essence),
        tags: vec!["#go".to_string()],
        taxonomy: vec!["#go".to_string()],
        artifact_dirs: vec!["bin".to_string()],
        sub_projects: Vec::new(),
        submodules: Vec::new(),
        source: toad_core::TargetSource::PondProject,
        total_size: 0,
        bloat_index: 0.0,
    }];

    let md = generate_markdown(&projects, 1, None);
    // Should be truncated at 100 chars (actually 97 + "...")
    assert!(md.contains("..."));
    // Find the essence part in the row
    let line = md.lines().find(|l| l.contains("long-proj")).unwrap();
    let parts: Vec<&str> = line.split('|').collect();
    let essence_part = parts[5].trim();
    assert_eq!(essence_part.len(), 100);
    assert!(essence_part.ends_with("..."));
}

#[test]
fn test_token_budget_truncation() {
    let projects = vec![ProjectDetail {
        name: "test-proj".to_string(),
        path: PathBuf::from("projects/test-proj"),
        stack: "Rust".to_string(),
        activity: ActivityTier::Active,
        vcs_status: VcsStatus::Clean,
        essence: Some("Test essence".to_string()),
        tags: vec!["#test".to_string()],
        taxonomy: vec!["#test".to_string()],
        artifact_dirs: vec!["target".to_string()],
        sub_projects: Vec::new(),
        submodules: Vec::new(),
        source: toad_core::TargetSource::PondProject,
        total_size: 0,
        bloat_index: 0.0,
    }];

    // Set a very small budget (10 tokens = ~40 chars)
    let md = generate_markdown(&projects, 12345, Some(10));
    assert!(md.contains("[Truncated due to token budget]"));
    assert!(md.len() <= 45 + 40); // 40 chars + truncation message
}

#[test]
fn test_generate_project_context_md() {
    let project = ProjectDetail {
        name: "test-proj".to_string(),
        path: PathBuf::from("projects/test-proj"),
        stack: "Rust".to_string(),
        activity: ActivityTier::Active,
        vcs_status: VcsStatus::Clean,
        essence: Some("Detailed essence here".to_string()),
        tags: vec!["#test".to_string()],
        taxonomy: vec!["#test".to_string()],
        artifact_dirs: vec!["target".to_string()],
        sub_projects: Vec::new(),
        submodules: Vec::new(),
        source: toad_core::TargetSource::PondProject,
        total_size: 1000,
        bloat_index: 10.0,
    };

    let md = super::generate_project_context_md(&project, None);
    assert!(md.contains("# Project Context: test-proj"));
    assert!(md.contains("Detailed essence here"));
    assert!(md.contains("- **Stack:** `Rust`"));
}

#[test]
fn test_generate_system_prompt() {
    let projects = vec![ProjectDetail {
        name: "test-proj".to_string(),
        path: PathBuf::from("projects/test-proj"),
        stack: "Rust".to_string(),
        activity: ActivityTier::Active,
        vcs_status: VcsStatus::Clean,
        essence: Some("essence".to_string()),
        tags: vec!["#test".to_string()],
        taxonomy: vec!["#test".to_string()],
        artifact_dirs: vec!["target".to_string()],
        sub_projects: Vec::new(),
        submodules: Vec::new(),
        source: toad_core::TargetSource::PondProject,
        total_size: 0,
        bloat_index: 0.0,
    }];

    let md = super::generate_system_prompt(&projects, None);
    assert!(md.contains("# Ecosystem System Prompt"));
    assert!(md.contains("| `test-proj` | `Rust` |"));
    assert!(md.contains("`#test`"));
}

#[test]
fn test_generate_llms_txt() {
    let projects = vec![ProjectDetail {
        name: "test-proj".to_string(),
        path: PathBuf::from("projects/test-proj"),
        stack: "Rust".to_string(),
        activity: ActivityTier::Active,
        vcs_status: VcsStatus::Clean,
        essence: Some("essence".to_string()),
        tags: vec!["#test".to_string()],
        taxonomy: vec!["#test".to_string()],
        artifact_dirs: vec!["target".to_string()],
        sub_projects: Vec::new(),
        submodules: Vec::new(),
        source: toad_core::TargetSource::PondProject,
        total_size: 0,
        bloat_index: 0.0,
    }];

    let md = super::generate_llms_txt(&projects);
    assert!(md.contains("# Toad Ecosystem Context"));
    assert!(md.contains("- [test-proj CONTEXT](./test-proj/CONTEXT.md)"));
    assert!(md.contains("- [test-proj AGENTS](./test-proj/AGENTS.md)"));
}

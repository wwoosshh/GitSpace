pub mod model;
pub mod layout;
pub mod git_reader;

use model::SceneModel;

/// 리더 + 변환을 합성한 순수-ish 함수 (테스트 대상)
pub fn load_scene(path: &str) -> Result<SceneModel, String> {
    let data = git_reader::read_repo(path)?;
    Ok(layout::build_scene(&data))
}

// lib.rs 내부 커맨드는 pub를 붙이지 않는다 (glue 코드 생성 규칙)
#[tauri::command]
async fn load_repo(path: String) -> Result<SceneModel, String> {
    load_scene(&path)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![load_repo])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    use git2::{Repository, Signature};
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn load_scene_end_to_end_builds_model() {
        let dir = TempDir::new().unwrap();
        let repo = Repository::init(dir.path()).unwrap();
        fs::write(dir.path().join("f.txt"), "x").unwrap();
        let mut idx = repo.index().unwrap();
        idx.add_path(std::path::Path::new("f.txt")).unwrap();
        idx.write().unwrap();
        let tree = repo.find_tree(idx.write_tree().unwrap()).unwrap();
        let sig = Signature::new("Dev", "dev@x.com", &git2::Time::new(100, 0)).unwrap();
        repo.commit(Some("HEAD"), &sig, &sig, "init", &tree, &[]).unwrap();

        let path = dir.path().to_string_lossy().to_string();
        let scene = load_scene(&path).unwrap();
        assert_eq!(scene.meta.total_commits, 1);
        assert_eq!(scene.commits.len(), 1);
        assert_eq!(scene.authors.len(), 1);
        assert_eq!(scene.branches.len(), 1);
    }
}

use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use std::path::{Path, PathBuf};
use std::fs;

pub struct CommandHistory {
    editor: DefaultEditor,
    history_path: PathBuf,
}

impl CommandHistory {
    pub fn new(_history_limit: usize) -> Self {
        let mut editor = DefaultEditor::new().unwrap();
        let history_path = Self::get_history_path();
        
        if history_path.exists() {
            let _ = editor.load_history(&history_path);
        }
        
        Self {
            editor,
            history_path,
        }
    }

    pub fn readline(&mut self, prompt: &str) -> Result<String, ReadlineError> {
        match self.editor.readline(prompt) {
            Ok(line) => {
                let _ = self.editor.add_history_entry(line.as_str());
                Ok(line)
            }
            Err(err) => Err(err),
        }
    }

    pub fn save_history(&mut self) -> Result<(), ReadlineError> {
        if let Some(parent) = self.history_path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        self.editor.save_history(&self.history_path)
    }

    fn get_history_path() -> PathBuf {
        let exe_path = std::env::current_exe().unwrap_or_else(|_| PathBuf::from("."));
        exe_path.parent().unwrap_or(Path::new(".")).join("magish-history.txt")
    }
}

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Node {
    File(String),
    Dir(HashMap<String, Node>),
}

pub struct FileSystem {
    pub root: Node,
    pub cwd: Vec<String>,
}

impl FileSystem {
    pub fn new() -> Self {
        let mut root = HashMap::new();

        // /logs
        let mut logs = HashMap::new();
        logs.insert("log-07.vsk".to_string(), Node::File(
            "# ilya vosk — personal log 7\n# they don't tell you what tuning costs\n\n~ subject {\n    identity  :: + \"VS-04\"\n    coherence :: % 0.75\n    memory    :: % 0.50\n    signal    :: +\n    tuned     :: +\n\n    ! identity :: +\n    ! signal   :: +\n\n    @ coherence :: x ~> ^ \"SUBJECT_LOST\"\n    @ memory    :: x ~> emit \"LOG_BEFORE_THEY_NOTICE\"\n}".to_string()
        ));
        logs.insert("log-12.vsk".to_string(), Node::File(
            "# ilya vosk — personal log 12\n# i ran into VS-04 today\n# she looked through me\n\n~ encounter {\n    recognition :: ?\n    response    :: ?\n    signal      :: % 0.25\n\n    @ signal :: x ~> emit \"SHE_IS_GONE\"\n}".to_string()
        ));

        // /bureau
        let mut bureau = HashMap::new();
        bureau.insert("REDACTED_001.txt".to_string(), Node::File(
            "BUREAU OF GRAVITATIONAL ANOMALIES\nCLASSIFICATION: DELTA\n\n[REDACTED] has been removed from active rotation.\nResonance profile archived under VS-04.\nTuning complete. Subject stable.\n\n// stable is not the word i would use — IV".to_string()
        ));
        bureau.insert("subject-registry.txt".to_string(), Node::File(
            "ACTIVE SUBJECTS\n---------------\nVS-01 ... [REDACTED]\nVS-02 ... [REDACTED]\nVS-03 ... [REDACTED]\nVS-04 ... coherence declining. flagged.\nVS-05 ... tuning in progress\n\n// they flagged her and kept going — IV".to_string()
        ));

        // /personal
        let mut personal = HashMap::new();
        personal.insert("note.txt".to_string(), Node::File(
            "if you found this you were meant to.\nthe symbols in the paintings are a map.\nstart with the ones near water.\n\n— ilya".to_string()
        ));
        personal.insert("symbols.txt".to_string(), Node::File(
            "triangle over circle  — anomaly present\ncircle with cross     — containment active\nthree lines converge  — tuning site\nspiral inward         — subject lost\n\nyou have seen these. you just didn't know what they meant.".to_string()
        ));

        // /README
        root.insert("README".to_string(), Node::File(
            "VØSK TERMINAL\n\nthis session belongs to ilya vosk.\nif you are reading this, the session outlasted me.\nnavigate carefully.\nsome files are corrupted.\nsome files are not what they appear to be.\n\n— IV".to_string()
        ));

        root.insert("logs".to_string(), Node::Dir(logs));
        root.insert("bureau".to_string(), Node::Dir(bureau));
        root.insert("personal".to_string(), Node::Dir(personal));

        FileSystem {
            root: Node::Dir(root),
            cwd: vec![],
        }
    }

    pub fn current_dir(&self) -> &HashMap<String, Node> {
        let mut node = &self.root;
        for segment in &self.cwd {
            if let Node::Dir(map) = node {
                node = map.get(segment).unwrap();
            }
        }
        if let Node::Dir(map) = node {
            map
        } else {
            panic!("cwd is not a directory");
        }
    }

    pub fn get_file(&self, path: &str) -> Option<&str> {
        let dir = self.current_dir();
        if let Some(Node::File(contents)) = dir.get(path) {
            Some(contents)
        } else {
            None
        }
    }

    pub fn change_dir(&mut self, name: &str) -> Result<(), String> {
        if name == ".." {
            if !self.cwd.is_empty() {
                self.cwd.pop();
            }
            return Ok(());
        }

        let dir = self.current_dir();
        match dir.get(name) {
            Some(Node::Dir(_)) => {
                self.cwd.push(name.to_string());
                Ok(())
            }
            Some(Node::File(_)) => Err(format!("'{}' is a file, not a directory", name)),
            None => Err(format!("'{}' not found", name)),
        }
    }

    pub fn prompt(&self) -> String {
        if self.cwd.is_empty() {
            "VØSK:/$ ".to_string()
        } else {
            format!("VØSK:/{}$ ", self.cwd.join("/"))
        }
    }
}

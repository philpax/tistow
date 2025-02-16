use std::path::PathBuf;

use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};

pub enum SearchMode {
    Search,
    Calculator,
}

pub struct SearchResult {
    pub mode: SearchMode,
    pub text: String,
    pub action: Option<ResultAction>,
}

pub enum ResultAction {
    Open { path: PathBuf },
    Copy { text: String },
}

pub struct Search {
    matcher: SkimMatcherV2,
    shortcuts: Vec<PathBuf>,
}

impl Search {
    pub fn new(shortcuts: Vec<PathBuf>) -> Self {
        Self {
            matcher: SkimMatcherV2::default(),
            shortcuts,
        }
    }

    pub fn search(&self, input: &str) -> Vec<SearchResult> {
        if input.is_empty() {
            return vec![];
        }

        // TODO: find better way to do this
        let mode = if input.starts_with('=') {
            SearchMode::Calculator
        } else {
            SearchMode::Search
        };

        match mode {
            SearchMode::Calculator => self.mode_calculator(&input[1..]),
            SearchMode::Search => self.mode_search(input),
        }
    }

    fn mode_calculator(&self, input: &str) -> Vec<SearchResult> {
        let r = meval::eval_str(input.trim());

        let res = if let Ok(n) = r {
            n.to_string()
        } else {
            "ERROR".to_string()
        };

        vec![SearchResult {
            mode: SearchMode::Calculator,
            text: format!("= {}", res),
            action: Some(ResultAction::Copy { text: res }),
        }]
    }

    fn mode_search(&self, input: &str) -> Vec<SearchResult> {
        self.shortcuts
            .iter()
            .cloned()
            .filter_map(|path| {
                let name = path.file_stem()?.to_str()?.to_string();

                self.matcher
                    .fuzzy_match(&name, input)
                    .map(|_| SearchResult {
                        mode: SearchMode::Search,
                        text: name.to_string(),
                        action: Some(ResultAction::Open { path }),
                    })
            })
            .collect()
    }
}

#[derive(Debug, Clone)]
pub struct Search {
    pub keyword: Option<String>,
    pub search_text: String,
}

pub fn get_search(text: impl Into<String>) -> Search {
    let text = text.into();

    let mut keyword = "".to_string();
    let mut search_text = "".to_string();
    let mut has_keyword = false;

    for char in text.chars() {
        if char == ' ' && !has_keyword {
            has_keyword = true;
        } else if !has_keyword {
            keyword += &char.to_string();
        } else {
            search_text += &char.to_string();
        }
    }

    search_text = search_text.trim().to_string();

    Search {
        keyword: match has_keyword {
            true => Some(keyword),
            false => None,
        },
        search_text,
    }
}

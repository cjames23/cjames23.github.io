use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Comment {
    pub id: String,
    pub post_id: String,
    pub content: String,
    pub author: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CommentDb {
    comments: Vec<Comment>,
}

impl CommentDb {
    pub fn new() -> Self {
        Self {
            comments: Vec::new(),
        }
    }

    pub fn add_comment(&mut self, comment: Comment) {
        self.comments.push(comment);
    }
    pub fn get_comment_for_post(&self, post_id: &str, comment_id: &str) -> Option<&Comment> {
        self.comments
            .iter()
            .find(|comment| comment.post_id == post_id && comment.id == comment_id)
    }
    pub fn get_comments(&self, post_id: &str) -> Vec<&Comment> {
        self.comments
            .iter()
            .filter(|comment| comment.post_id == post_id)
            .collect()
    }
}

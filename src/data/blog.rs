use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use uuid::Uuid;
use yew::prelude::*;

// Blog Post model
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlogPost {
    pub id: String,
    pub title: String,
    pub content: String,
    pub author: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    pub tags: Vec<String>,
}

impl BlogPost {
    pub fn new(title: String, content: String, author: String, tags: Vec<String>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            title,
            content,
            author,
            created_at: Utc::now(),
            updated_at: None,
            tags,
        }
    }
}

// Blog Database
#[derive(Clone, Debug, Default)]
pub struct BlogDb {
    posts: HashMap<String, BlogPost>,
}

impl BlogDb {
    pub fn new() -> Self {
        let mut db = Self {
            posts: HashMap::new(),
        };

        db.load_posts();
        db
    }

    fn load_posts(&mut self) {
        // Include the JSON file at compile time
        let data = include_str!("blog_data.json");

        // Parse the JSON into blog posts
        if let Ok(posts) = serde_json::from_str::<Vec<BlogPost>>(data) {
            for post in posts {
                self.posts.insert(post.id.clone(), post);
            }
        }
    }

    pub fn add_post(&mut self, post: BlogPost) -> String {
        let id = post.id.clone();
        self.posts.insert(id.clone(), post);
        id
    }

    pub fn get_post(&self, id: &str) -> Option<&BlogPost> {
        self.posts.get(id)
    }

    pub fn get_all_posts(&self) -> Vec<&BlogPost> {
        self.posts.values().collect()
    }

    pub fn update_post(
        &mut self,
        id: &str,
        mut updated_post: BlogPost,
    ) -> Result<(), &'static str> {
        if let Some(existing) = self.posts.get(id) {
            updated_post.id = id.to_string();
            updated_post.created_at = existing.created_at;
            updated_post.updated_at = Some(Utc::now());
            self.posts.insert(id.to_string(), updated_post);
            Ok(())
        } else {
            Err("Post not found")
        }
    }

    pub fn delete_post(&mut self, id: &str) -> Result<(), &'static str> {
        if self.posts.remove(id).is_some() {
            Ok(())
        } else {
            Err("Post not found")
        }
    }

    pub fn search_posts(&self, query: &str) -> Vec<&BlogPost> {
        let query = query.to_lowercase();
        self.posts
            .values()
            .filter(|post| {
                post.title.to_lowercase().contains(&query)
                    || post.content.to_lowercase().contains(&query)
            })
            .collect()
    }

    pub fn filter_by_tag(&self, tag: &str) -> Vec<&BlogPost> {
        let tag = tag.to_lowercase();
        self.posts
            .values()
            .filter(|post| post.tags.iter().any(|t| t.to_lowercase() == tag))
            .collect()
    }
}

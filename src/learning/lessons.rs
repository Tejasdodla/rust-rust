use crate::config::GlobalConfig;
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lesson {
    pub id: String,
    pub title: String,
    pub description: String,
    pub difficulty: Difficulty,
    pub content: String,
    pub code_examples: Vec<CodeExample>,
    pub next_lesson: Option<String>,
    pub prev_lesson: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Difficulty {
    Beginner,
    Intermediate,
    Advanced,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeExample {
    pub title: String,
    pub code: String,
    pub explanation: String,
    pub runnable: bool,
}

#[derive(Debug)]
pub struct LessonManager {
    lessons: HashMap<String, Lesson>,
    lesson_order: Vec<String>,
    config: GlobalConfig,
}

impl LessonManager {
    pub fn new(config: GlobalConfig) -> Result<Self> {
        let mut manager = Self {
            lessons: HashMap::new(),
            lesson_order: Vec::new(),
            config,
        };
        manager.load_lessons()?;
        Ok(manager)
    }

    pub fn load_lessons(&mut self) -> Result<()> {
        let lessons_dir = self.get_lessons_dir();
        
        // If lessons directory doesn't exist, create it with default lessons
        if !lessons_dir.exists() {
            fs::create_dir_all(&lessons_dir)?;
            self.create_default_lessons(&lessons_dir)?;
        }

        // Load all lesson files
        for entry in fs::read_dir(&lessons_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.extension().and_then(|s| s.to_str()) == Some("yaml") {
                let content = fs::read_to_string(&path)?;
                let lesson: Lesson = serde_yaml::from_str(&content)?;
                self.lessons.insert(lesson.id.clone(), lesson);
            }
        }

        // Build lesson order
        self.build_lesson_order();
        Ok(())
    }

    pub fn get_lesson(&self, id: &str) -> Option<&Lesson> {
        self.lessons.get(id)
    }

    pub fn list_lessons(&self) -> Vec<&Lesson> {
        self.lesson_order
            .iter()
            .filter_map(|id| self.lessons.get(id))
            .collect()
    }

    pub fn get_lessons_by_difficulty(&self, difficulty: &Difficulty) -> Vec<&Lesson> {
        self.lessons
            .values()
            .filter(|lesson| std::mem::discriminant(&lesson.difficulty) == std::mem::discriminant(difficulty))
            .collect()
    }

    pub fn render_lesson_html(&self, lesson: &Lesson) -> String {
        // Simple HTML rendering - replace markdown-like syntax
        let content = &lesson.content;
        
        // Convert basic markdown to HTML
        let html = content
            .replace("# ", "<h1>")
            .replace("\n\n", "</h1>\n<p>")
            .replace("## ", "<h2>")
            .replace("### ", "<h3>")
            .replace("**", "<strong>")
            .replace("**", "</strong>")
            .replace("- ", "<li>")
            .replace("\n", "</li>\n");
            
        // Wrap in basic HTML structure
        format!("<div class='lesson-content'>{}</div>", html)
    }

    fn get_lessons_dir(&self) -> PathBuf {
        // Try to get lessons directory from config, default to ./lessons
        PathBuf::from("lessons")
    }

    fn build_lesson_order(&mut self) {
        // Build a simple ordered list of lessons
        let mut lesson_ids: Vec<String> = self.lessons.keys().cloned().collect();
        lesson_ids.sort();
        self.lesson_order = lesson_ids;
    }

    fn create_default_lessons(&self, lessons_dir: &Path) -> Result<()> {
        // Create some default beginner lessons
        let intro_lesson = Lesson {
            id: "01-intro".to_string(),
            title: "Introduction to Rust".to_string(),
            description: "Learn the basics of the Rust programming language".to_string(),
            difficulty: Difficulty::Beginner,
            content: r#"# Introduction to Rust

Welcome to Rust! Rust is a systems programming language that focuses on safety, speed, and concurrency.

## What makes Rust special?

- **Memory Safety**: Rust prevents common programming errors like null pointer dereferences and buffer overflows
- **Zero-cost abstractions**: High-level features don't sacrifice performance
- **Fearless concurrency**: Safe and efficient concurrent programming

## Your first Rust program

Let's start with the classic "Hello, World!" program:

```rust
fn main() {
    println!("Hello, World!");
}
```

This program defines a `main` function, which is the entry point of every Rust program. The `println!` macro prints text to the console.
"#.to_string(),
            code_examples: vec![
                CodeExample {
                    title: "Hello World".to_string(),
                    code: r#"fn main() {
    println!("Hello, World!");
}"#.to_string(),
                    explanation: "This is the simplest Rust program. It prints 'Hello, World!' to the console.".to_string(),
                    runnable: true,
                }
            ],
            next_lesson: Some("02-variables".to_string()),
            prev_lesson: None,
        };

        let variables_lesson = Lesson {
            id: "02-variables".to_string(),
            title: "Variables and Mutability".to_string(),
            description: "Learn about variables, mutability, and data types in Rust".to_string(),
            difficulty: Difficulty::Beginner,
            content: r#"# Variables and Mutability

In Rust, variables are **immutable by default**. This means once you assign a value to a variable, you can't change it unless you explicitly make it mutable.

## Immutable Variables

```rust
let x = 5;
// x = 6; // This would cause a compile error!
```

## Mutable Variables

To make a variable mutable, use the `mut` keyword:

```rust
let mut x = 5;
x = 6; // This is allowed
```

## Data Types

Rust has several built-in data types:

- **Integers**: `i32`, `u32`, `i64`, `u64`, etc.
- **Floating point**: `f32`, `f64`
- **Boolean**: `bool`
- **Character**: `char`
- **String**: `String` and `&str`
"#.to_string(),
            code_examples: vec![
                CodeExample {
                    title: "Immutable Variables".to_string(),
                    code: r#"fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
}"#.to_string(),
                    explanation: "Variables are immutable by default in Rust.".to_string(),
                    runnable: true,
                },
                CodeExample {
                    title: "Mutable Variables".to_string(),
                    code: r#"fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    
    x = 6;
    println!("The value of x is: {}", x);
}"#.to_string(),
                    explanation: "Use 'mut' to make variables mutable.".to_string(),
                    runnable: true,
                }
            ],
            next_lesson: Some("03-ownership".to_string()),
            prev_lesson: Some("01-intro".to_string()),
        };

        // Write lessons to files
        let intro_path = lessons_dir.join("01-intro.yaml");
        let intro_content = serde_yaml::to_string(&intro_lesson)?;
        fs::write(intro_path, intro_content)?;

        let variables_path = lessons_dir.join("02-variables.yaml");
        let variables_content = serde_yaml::to_string(&variables_lesson)?;
        fs::write(variables_path, variables_content)?;

        Ok(())
    }
}
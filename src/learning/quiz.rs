use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quiz {
    pub id: String,
    pub title: String,
    pub description: String,
    pub lesson_id: Option<String>,
    pub questions: Vec<Question>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Question {
    pub id: String,
    pub question_type: QuestionType,
    pub title: String,
    pub content: String,
    pub hint: Option<String>,
    pub explanation: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum QuestionType {
    MultipleChoice {
        options: Vec<String>,
        correct_answer: usize,
    },
    CodeCompletion {
        template: String,
        solution: String,
        test_cases: Vec<TestCase>,
    },
    ShortAnswer {
        expected_keywords: Vec<String>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestCase {
    pub input: String,
    pub expected_output: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuizAttempt {
    pub quiz_id: String,
    pub answers: HashMap<String, UserAnswer>,
    pub score: Option<f32>,
    pub completed_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum UserAnswer {
    MultipleChoice { selected: usize },
    CodeCompletion { code: String },
    ShortAnswer { text: String },
}

#[derive(Debug)]
pub struct QuizManager {
    quizzes: HashMap<String, Quiz>,
}

impl QuizManager {
    pub fn new() -> Self {
        let mut manager = Self {
            quizzes: HashMap::new(),
        };
        manager.load_default_quizzes();
        manager
    }

    pub fn get_quiz(&self, id: &str) -> Option<&Quiz> {
        self.quizzes.get(id)
    }

    pub fn list_quizzes(&self) -> Vec<&Quiz> {
        self.quizzes.values().collect()
    }

    pub fn evaluate_answer(&self, question: &Question, answer: &UserAnswer) -> (bool, Option<String>) {
        match (&question.question_type, answer) {
            (QuestionType::MultipleChoice { correct_answer, .. }, UserAnswer::MultipleChoice { selected }) => {
                let is_correct = *selected == *correct_answer;
                let feedback = if is_correct {
                    Some("Correct!".to_string())
                } else {
                    Some(format!("Incorrect. The correct answer was option {}.", *correct_answer + 1))
                };
                (is_correct, feedback)
            }
            (QuestionType::ShortAnswer { expected_keywords }, UserAnswer::ShortAnswer { text }) => {
                let text_lower = text.to_lowercase();
                let matching_keywords: Vec<_> = expected_keywords
                    .iter()
                    .filter(|keyword| text_lower.contains(&keyword.to_lowercase()))
                    .collect();
                
                let is_correct = !matching_keywords.is_empty();
                let feedback = if is_correct {
                    Some(format!("Good! You mentioned: {}", matching_keywords.join(", ")))
                } else {
                    Some(format!("Try to include these concepts: {}", expected_keywords.join(", ")))
                };
                (is_correct, feedback)
            }
            _ => (false, Some("Answer type mismatch".to_string()))
        }
    }

    fn load_default_quizzes(&mut self) {
        // Create a sample quiz for the intro lesson
        let intro_quiz = Quiz {
            id: "quiz-01-intro".to_string(),
            title: "Introduction to Rust Quiz".to_string(),
            description: "Test your understanding of basic Rust concepts".to_string(),
            lesson_id: Some("01-intro".to_string()),
            questions: vec![
                Question {
                    id: "q1".to_string(),
                    question_type: QuestionType::MultipleChoice {
                        options: vec![
                            "Memory safety and performance".to_string(),
                            "Ease of learning".to_string(),
                            "Large standard library".to_string(),
                            "Dynamic typing".to_string(),
                        ],
                        correct_answer: 0,
                    },
                    title: "What makes Rust special?".to_string(),
                    content: "What is one of the main advantages of Rust over other systems programming languages?".to_string(),
                    hint: Some("Think about what prevents common programming errors".to_string()),
                    explanation: Some("Rust focuses on memory safety without sacrificing performance, preventing common errors like null pointer dereferences.".to_string()),
                },
                Question {
                    id: "q2".to_string(),
                    question_type: QuestionType::ShortAnswer {
                        expected_keywords: vec!["main".to_string(), "function".to_string(), "entry point".to_string()],
                    },
                    title: "Entry Point".to_string(),
                    content: "What is the name of the function that serves as the entry point for Rust programs?".to_string(),
                    hint: Some("Every Rust program starts execution from this function".to_string()),
                    explanation: Some("The main() function is the entry point where Rust programs begin execution.".to_string()),
                },
            ],
        };

        self.quizzes.insert(intro_quiz.id.clone(), intro_quiz);
    }
}
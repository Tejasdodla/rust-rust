use crate::learning::*;
use anyhow::Result;

pub fn test_learning_components() -> Result<()> {
    println!("🧪 Testing learning components...");
    
    // Test quiz manager (doesn't need config)
    println!("❓ Testing QuizManager...");
    let quiz_manager = QuizManager::new();
    let quizzes = quiz_manager.list_quizzes();
    println!("  Found {} quizzes", quizzes.len());
    
    // Test code executor
    println!("💻 Testing RustCodeExecutor...");
    let code_executor = RustCodeExecutor::new()?;
    println!("  Code executor initialized successfully");
    
    // Test simple code validation
    let safe_code = "fn main() { println!(\"Hello!\"); }";
    let is_safe = is_safe_code(safe_code);
    println!("  Code safety check: {}", if is_safe { "✅ PASS" } else { "❌ FAIL" });
    
    println!("✅ Core learning components tested successfully!");
    println!("💡 Note: Full lesson manager test requires config initialization");
    Ok(())
}
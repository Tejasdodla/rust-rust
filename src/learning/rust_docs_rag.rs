use crate::{config::GlobalConfig, rag::Rag, utils::*};
use anyhow::{anyhow, Result};
use std::path::PathBuf;

pub async fn setup_rust_docs_rag(config: &GlobalConfig) -> Result<()> {
    println!("📚 Setting up Rust documentation RAG...");
    
    // Check if Rust docs RAG already exists
    let rag_name = "rust-docs";
    let rag_path = config.read().rag_file(rag_name);
    
    if rag_path.exists() {
        println!("✅ Rust docs RAG already exists at {}", rag_path.display());
        return Ok(());
    }
    
    // Define Rust documentation sources
    let rust_doc_sources = vec![
        // Core Rust documentation
        "https://doc.rust-lang.org/book/**".to_string(),
        "https://doc.rust-lang.org/std/**".to_string(),
        "https://doc.rust-lang.org/reference/**".to_string(),
        "https://doc.rust-lang.org/rust-by-example/**".to_string(),
        
        // Additional learning resources
        "https://doc.rust-lang.org/rustc/**".to_string(),
        "https://doc.rust-lang.org/cargo/**".to_string(),
    ];
    
    println!("🔄 Creating Rust docs RAG with sources:");
    for source in &rust_doc_sources {
        println!("  - {}", source);
    }
    
    // Create abort signal for the operation
    let abort_signal = create_abort_signal();
    
    // Initialize the RAG with Rust documentation
    match Rag::init(config, rag_name, &rag_path, &rust_doc_sources, abort_signal).await {
        Ok(_) => {
            println!("✅ Successfully created Rust docs RAG!");
            println!("💡 You can now use RAG-powered Rust documentation lookup in lessons");
        }
        Err(e) => {
            println!("⚠️ Failed to create Rust docs RAG: {}", e);
            println!("💡 You can manually add Rust documentation later using the --rag flag");
        }
    }
    
    Ok(())
}

pub async fn search_rust_docs(config: &GlobalConfig, query: &str) -> Result<String> {
    let rag_name = "rust-docs";
    let rag_path = config.read().rag_file(rag_name);
    
    if !rag_path.exists() {
        return Err(anyhow!("Rust docs RAG not found. Run setup first."));
    }
    
    let rag = Rag::load(config, rag_name, &rag_path)?;
    let (reranker_model, top_k) = rag.get_config();
    let abort_signal = create_abort_signal();
    
    let (results, _) = rag.search(query, top_k, reranker_model.as_deref(), abort_signal).await?;
    
    Ok(results)
}
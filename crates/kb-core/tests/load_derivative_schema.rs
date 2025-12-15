use kb_core::Schema;
use std::path::PathBuf;

#[test]
#[ignore = "Example schema file not yet created - see schemas/examples/"]
fn test_load_derivative_schema() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.pop(); // Go to crates/
    path.pop(); // Go to root
    path.push("schemas/examples/derivative.yaml");

    let schema = Schema::load_from_file(&path)
        .expect("Failed to load derivative.yaml");

    // Validate core fields
    assert_eq!(schema.topic, "calculus.derivative");
    assert_eq!(schema.title, "Symbolic Differentiation");
    assert!(!schema.description.is_empty());

    // Validate code references
    let code_refs = schema.code_refs.as_ref().expect("Expected code_refs");
    assert_eq!(code_refs.rust, "mathhook_core::calculus::derivative");
    assert_eq!(code_refs.python, "mathhook.calculus.derivative");
    assert_eq!(code_refs.nodejs, "mathhook.calculus.derivative");

    // Validate examples
    assert_eq!(schema.examples.len(), 3);
    assert_eq!(schema.examples[0].title, "Power Rule");
    assert_eq!(schema.examples[1].title, "Chain Rule");
    assert_eq!(schema.examples[2].title, "Product Rule");

    // All examples should have code in all three languages
    for example in &schema.examples {
        assert!(!example.code.rust.is_empty());
        assert!(!example.code.python.is_empty());
        assert!(!example.code.nodejs.is_empty());
    }

    // Validate metadata
    assert!(!schema.use_cases.is_empty());
    assert!(!schema.related_topics.is_empty());
    assert!(schema.performance.is_some());
    assert!(schema.interactive_playground.is_some());

    // Validate output hints
    assert!(schema.outputs.jupyter.is_some());
    assert!(schema.outputs.mdbook.is_some());
    assert!(schema.outputs.vue_site.is_some());
    assert!(schema.outputs.api_docs.is_some());
    assert!(schema.outputs.llm_rag.is_some());

    println!("✅ Successfully loaded and validated derivative.yaml schema");
}

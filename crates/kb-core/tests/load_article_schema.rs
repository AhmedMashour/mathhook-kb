use kb_core::Schema;
use std::path::PathBuf;

#[test]
fn test_load_derivative_with_article() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.pop(); // Go to crates/
    path.pop(); // Go to root
    path.push("schemas/examples/derivative-with-article.yaml");

    let schema = Schema::load_from_file(&path)
        .expect("Failed to load derivative-with-article.yaml");

    // Validate core fields
    assert_eq!(schema.topic, "calculus.derivative");
    assert_eq!(schema.title, "Symbolic Differentiation");

    // Validate article content exists
    assert!(schema.article.is_some());
    let article = schema.article.as_ref().unwrap();

    // Check introduction
    assert!(!article.introduction.hook.is_empty());
    assert_eq!(article.introduction.learning_objectives.len(), 4);
    assert_eq!(article.introduction.prerequisites.len(), 3);
    assert_eq!(article.introduction.estimated_time, Some("20-25 minutes".to_string()));

    // Check sections
    assert_eq!(article.sections.len(), 4);
    assert_eq!(article.sections[0].title, "What is a Derivative?");
    assert_eq!(article.sections[1].title, "The Power Rule");
    assert_eq!(article.sections[2].title, "The Chain Rule");
    assert_eq!(article.sections[3].title, "The Product Rule");

    // Check sidebars
    assert_eq!(article.sidebars.len(), 4);

    // Check conclusion
    assert!(article.conclusion.is_some());
    let conclusion = article.conclusion.as_ref().unwrap();
    assert!(!conclusion.summary.is_empty());
    assert_eq!(conclusion.next_steps.len(), 4);
    assert_eq!(conclusion.further_reading.len(), 3);
    assert_eq!(conclusion.exercises.len(), 3);

    // Check format variations
    assert!(article.variations.is_some());
    let variations = article.variations.as_ref().unwrap();
    assert!(variations.jupyter.is_some());
    assert!(variations.mdbook.is_some());
    assert!(variations.vue_site.is_some());

    // Check Jupyter variations
    let jupyter = variations.jupyter.as_ref().unwrap();
    assert!(jupyter.tutorial_intro.is_some());
    assert_eq!(jupyter.interactive_prompts.len(), 3);
    assert_eq!(jupyter.checkpoint_questions.len(), 3);

    // Check mdBook variations
    let mdbook = variations.mdbook.as_ref().unwrap();
    assert_eq!(mdbook.deep_dives.len(), 2);
    assert!(mdbook.implementation_notes.is_some());
    assert!(mdbook.complexity_analysis.is_some());

    // Check Vue site variations
    let vue = variations.vue_site.as_ref().unwrap();
    assert!(vue.marketing_hook.is_some());
    assert_eq!(vue.use_case_stories.len(), 2);
    assert!(vue.call_to_action.is_some());

    println!("✅ Successfully loaded and validated derivative-with-article.yaml");
    println!("   Article has {} sections, {} sidebars, {} exercises",
             article.sections.len(),
             article.sidebars.len(),
             article.conclusion.as_ref().unwrap().exercises.len());
}

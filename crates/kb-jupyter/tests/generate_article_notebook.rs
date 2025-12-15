use kb_core::generator::OutputGenerator;
/// Integration test for Jupyter generator with article content
///
/// This test validates that the Jupyter generator correctly handles schemas
/// with rich article content, including all sections, sidebars, exercises, and
/// format-specific variations.
use kb_core::Schema;
use kb_jupyter::JupyterGenerator;
use std::path::PathBuf;

#[test]
#[ignore = "Example schema file not yet created - see schemas/examples/"]
fn test_generate_from_derivative_with_article() {
    // Load the comprehensive derivative-with-article schema
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.pop(); // Go to crates/
    path.pop(); // Go to root
    path.push("schemas/examples/derivative-with-article.yaml");

    let schema =
        Schema::load_from_file(&path).expect("Failed to load derivative-with-article.yaml");

    // Generate Jupyter notebook
    let generator = JupyterGenerator::new();
    let output = generator
        .generate(&schema)
        .expect("Failed to generate Jupyter notebook");

    // Validate output structure
    generator
        .validate_output(&output)
        .expect("Generated output failed validation");

    // Parse as JSON to verify structure
    let parsed: serde_json::Value =
        serde_json::from_str(&output).expect("Failed to parse generated JSON");

    // Validate notebook structure
    assert!(parsed.get("nbformat").is_some(), "Missing nbformat field");
    assert_eq!(parsed["nbformat"], 4, "Incorrect nbformat version");
    assert!(parsed.get("cells").is_some(), "Missing cells field");

    let cells = parsed["cells"]
        .as_array()
        .expect("cells should be an array");

    // Should have multiple cells (header + intro + sections + examples + conclusion + exercises)
    assert!(
        cells.len() > 10,
        "Expected many cells for comprehensive article, got {}",
        cells.len()
    );

    // Check that first cell is markdown (header)
    assert_eq!(
        cells[0]["cell_type"], "markdown",
        "First cell should be markdown"
    );
    let first_cell_source = cells[0]["source"]
        .as_array()
        .expect("source should be array");
    let first_cell_text: String = first_cell_source
        .iter()
        .filter_map(|v| v.as_str())
        .collect();
    assert!(
        first_cell_text.contains("Symbolic Differentiation"),
        "Header should contain title"
    );

    // Check that we have both markdown and code cells
    let markdown_count = cells
        .iter()
        .filter(|c| c["cell_type"] == "markdown")
        .count();
    let code_count = cells.iter().filter(|c| c["cell_type"] == "code").count();

    assert!(markdown_count > 5, "Should have multiple markdown cells");
    assert!(code_count > 0, "Should have code cells for examples");

    // Check for key content sections
    let all_content = cells
        .iter()
        .filter(|c| c["cell_type"] == "markdown")
        .filter_map(|c| c["source"].as_array())
        .map(|arr| arr.iter().filter_map(|v| v.as_str()).collect::<String>())
        .collect::<Vec<String>>()
        .join("\n");

    assert!(
        all_content.contains("Learning Objectives"),
        "Should have learning objectives"
    );
    assert!(
        all_content.contains("Prerequisites"),
        "Should have prerequisites"
    );
    assert!(
        all_content.contains("What is a Derivative"),
        "Should have section titles"
    );
    assert!(
        all_content.contains("Power Rule"),
        "Should have Power Rule section"
    );
    assert!(
        all_content.contains("Conclusion"),
        "Should have conclusion section"
    );
    assert!(all_content.contains("Exercise"), "Should have exercises");
    assert!(
        all_content.contains("💡") || all_content.contains("⚠️"),
        "Should have sidebars with icons"
    );

    println!("✅ Successfully generated Jupyter notebook from article schema");
    println!(
        "   Generated {} cells ({} markdown, {} code)",
        cells.len(),
        markdown_count,
        code_count
    );
}

#[test]
#[ignore = "Example schema file not yet created - see schemas/examples/"]
fn test_generate_from_simple_schema() {
    // Load the simple derivative schema (without article)
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.pop(); // Go to crates/
    path.pop(); // Go to root
    path.push("schemas/examples/derivative.yaml");

    let schema = Schema::load_from_file(&path).expect("Failed to load derivative.yaml");

    // Generate Jupyter notebook
    let generator = JupyterGenerator::new();
    let output = generator
        .generate(&schema)
        .expect("Failed to generate Jupyter notebook");

    // Validate output
    generator
        .validate_output(&output)
        .expect("Generated output failed validation");

    // Parse and check structure
    let parsed: serde_json::Value =
        serde_json::from_str(&output).expect("Failed to parse generated JSON");

    assert!(parsed.get("nbformat").is_some());
    assert!(parsed.get("cells").is_some());

    let cells = parsed["cells"]
        .as_array()
        .expect("cells should be an array");

    // Simple schema should have fewer cells
    assert!(cells.len() > 3, "Should have at least header + examples");
    assert!(
        cells.len() < 20,
        "Simple schema should have moderate cell count"
    );

    println!("✅ Successfully generated Jupyter notebook from simple schema");
    println!("   Generated {} cells", cells.len());
}

#[test]
#[ignore = "Example schema file not yet created - see schemas/examples/"]
fn test_jupyter_specific_variations() {
    // Load article schema
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.pop();
    path.pop();
    path.push("schemas/examples/derivative-with-article.yaml");

    let schema = Schema::load_from_file(&path).expect("Failed to load schema");

    let generator = JupyterGenerator::new();
    let output = generator.generate(&schema).expect("Failed to generate");

    let parsed: serde_json::Value = serde_json::from_str(&output).expect("Failed to parse");

    let cells = parsed["cells"].as_array().expect("cells array");
    let all_content = cells
        .iter()
        .filter(|c| c["cell_type"] == "markdown")
        .filter_map(|c| c["source"].as_array())
        .map(|arr| arr.iter().filter_map(|v| v.as_str()).collect::<String>())
        .collect::<Vec<String>>()
        .join("\n");

    // Check for Jupyter-specific tutorial intro
    assert!(
        all_content.contains("Welcome") || all_content.contains("tutorial"),
        "Should have Jupyter-specific tutorial intro"
    );

    // Check for interactive prompts (checkpoints)
    assert!(
        all_content.contains("Checkpoint") || all_content.contains("Interactive"),
        "Should have interactive checkpoint questions"
    );

    println!("✅ Jupyter-specific variations rendered correctly");
}

#[test]
#[ignore = "Example schema file not yet created - see schemas/examples/"]
fn test_code_cells_executable() {
    // Verify code cells are properly formatted for execution
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.pop();
    path.pop();
    path.push("schemas/examples/derivative-with-article.yaml");

    let schema = Schema::load_from_file(&path).expect("Failed to load schema");

    let generator = JupyterGenerator::new();
    let output = generator.generate(&schema).expect("Failed to generate");

    let parsed: serde_json::Value = serde_json::from_str(&output).expect("Failed to parse");

    let cells = parsed["cells"].as_array().expect("cells array");

    // Find code cells
    let code_cells: Vec<_> = cells.iter().filter(|c| c["cell_type"] == "code").collect();

    assert!(!code_cells.is_empty(), "Should have code cells");

    // Check each code cell structure
    for cell in code_cells {
        assert!(cell.get("source").is_some(), "Code cell should have source");
        assert!(
            cell.get("outputs").is_some(),
            "Code cell should have outputs field"
        );

        // execution_count field should exist (can be null for unexecuted cells)
        assert!(
            cell.as_object().unwrap().contains_key("execution_count"),
            "Code cell should have execution_count field (even if null)"
        );

        // Verify source is an array of strings
        let source = cell["source"].as_array().expect("source should be array");
        assert!(!source.is_empty(), "Code cell source should not be empty");
    }

    println!("✅ Code cells properly formatted for execution");
}

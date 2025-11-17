use kb_core::generator::OutputGenerator;
/// Example: Generate a Jupyter notebook from the derivative-with-article schema
///
/// Run with: cargo run --example generate_notebook
///
/// This will create `output/derivative-with-article.ipynb` which can be opened in Jupyter.
use kb_core::Schema;
use kb_jupyter::JupyterGenerator;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load the comprehensive derivative-with-article schema
    let mut schema_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    schema_path.pop(); // Go to crates/
    schema_path.pop(); // Go to root
    schema_path.push("schemas/examples/derivative-with-article.yaml");

    println!("Loading schema from: {}", schema_path.display());
    let schema = Schema::load_from_file(&schema_path)?;

    println!("Schema loaded:");
    println!("  Topic: {}", schema.topic);
    println!("  Title: {}", schema.title);
    if let Some(article) = &schema.article {
        println!("  Article sections: {}", article.sections.len());
        println!("  Sidebars: {}", article.sidebars.len());
        if let Some(conclusion) = &article.conclusion {
            println!("  Exercises: {}", conclusion.exercises.len());
        }
    }

    // Generate Jupyter notebook
    println!("\nGenerating Jupyter notebook...");
    let generator = JupyterGenerator::new();
    let output = generator.generate(&schema)?;

    // Validate the output
    println!("Validating generated notebook...");
    generator.validate_output(&output)?;

    // Parse to get cell count
    let parsed: serde_json::Value = serde_json::from_str(&output)?;
    let cells = parsed["cells"].as_array().expect("cells should be array");
    println!("  Generated {} cells", cells.len());

    let markdown_count = cells
        .iter()
        .filter(|c| c["cell_type"] == "markdown")
        .count();
    let code_count = cells.iter().filter(|c| c["cell_type"] == "code").count();

    println!("  {} markdown cells", markdown_count);
    println!("  {} code cells", code_count);

    // Create output directory
    let mut output_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    output_dir.pop();
    output_dir.pop();
    output_dir.push("output");
    std::fs::create_dir_all(&output_dir)?;

    // Write to file
    let output_path = output_dir.join("derivative-with-article.ipynb");
    std::fs::write(&output_path, output)?;

    println!("\n✅ Successfully generated notebook:");
    println!("   {}", output_path.display());
    println!("\nYou can now open this file in:");
    println!(
        "   - Jupyter Notebook: jupyter notebook {}",
        output_path.display()
    );
    println!("   - JupyterLab: jupyter lab");
    println!("   - VS Code: Open with Jupyter extension");
    println!("   - Google Colab: Upload the .ipynb file");

    Ok(())
}

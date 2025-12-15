use anyhow::{Context, Result};
/// MathHook Knowledge Base CLI
///
/// Command-line interface for building, validating, and generating documentation
/// from KB schemas.
use clap::{Parser, Subcommand};
use kb_apidocs::ApiDocsGenerator;
use kb_colab::ColabGenerator;
use kb_core::{generator::OutputGenerator, Schema};
use kb_json::JsonGenerator;
use kb_jupyter::JupyterGenerator;
use kb_latex::LatexGenerator;
use kb_llm_rag::LlmRagGenerator;
use kb_mdbook::MdBookGenerator;
use kb_vue::VueGenerator;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "kb")]
#[command(about = "MathHook Knowledge Base CLI", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Build documentation from a schema
    Build {
        /// Path to schema file or directory
        #[arg(value_name = "SCHEMA")]
        schema_path: PathBuf,

        /// Output directory
        #[arg(short, long, default_value = "output")]
        output: PathBuf,

        /// Generators to run (comma-separated: jupyter,mdbook,llm-rag,vue,api-docs,colab,latex,json,all)
        #[arg(short, long, default_value = "all")]
        generators: String,
    },

    /// Validate a schema file
    Validate {
        /// Path to schema file
        #[arg(value_name = "SCHEMA")]
        schema_path: PathBuf,
    },

    /// List available generators
    List,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Build {
            schema_path,
            output,
            generators,
        } => build_command(schema_path, output, generators),
        Commands::Validate { schema_path } => validate_command(schema_path),
        Commands::List => {
            list_command();
            Ok(())
        }
    }
}

fn build_command(schema_path: PathBuf, output_dir: PathBuf, generators_str: String) -> Result<()> {
    println!("🔨 Building documentation...\n");

    // Load schema
    println!("📄 Loading schema: {}", schema_path.display());
    let schema = Schema::load_from_file(&schema_path).context("Failed to load schema")?;

    println!("   Topic: {}", schema.topic);
    println!("   Title: {}\n", schema.title);

    // Create output directory
    std::fs::create_dir_all(&output_dir).context("Failed to create output directory")?;

    // Parse generators to run
    let generators_list: Vec<&str> = generators_str.split(',').collect();
    let run_all = generators_list.contains(&"all");

    let mut generated_count = 0;

    // Run Jupyter generator
    if run_all || generators_list.contains(&"jupyter") {
        println!("📓 Generating Jupyter notebook...");
        let generator = JupyterGenerator::new();
        let filename = generator.get_output_filename(&schema);
        let output_path = output_dir.join(&filename);

        generator
            .generate_to_file(&schema, &output_path)
            .context("Failed to generate Jupyter notebook")?;

        println!("   ✅ {}", output_path.display());
        generated_count += 1;
    }

    // Run mdBook generator
    if run_all || generators_list.contains(&"mdbook") {
        println!("📚 Generating mdBook markdown...");
        let generator = MdBookGenerator::new()?;
        let filename = generator.get_output_filename(&schema);
        let output_path = output_dir.join(&filename);

        generator
            .generate_to_file(&schema, &output_path)
            .context("Failed to generate mdBook markdown")?;

        println!("   ✅ {}", output_path.display());
        generated_count += 1;
    }

    // Run LLM-RAG generator
    if run_all || generators_list.contains(&"llm-rag") {
        println!("🤖 Generating LLM-RAG markdown...");
        let generator = LlmRagGenerator::from_schema(&schema);
        let filename = format!("{}.rag.md", schema.topic.replace('.', "-"));
        let output_path = output_dir.join(&filename);

        generator
            .generate_to_file(&schema, &output_path)
            .context("Failed to generate LLM-RAG markdown")?;

        println!("   ✅ {}", output_path.display());
        generated_count += 1;
    }

    // Run Vue generator
    if run_all || generators_list.contains(&"vue") {
        println!("🎨 Generating Vue SSR component...");
        let generator = VueGenerator::new()?;
        let filename = generator.get_output_filename(&schema);
        let output_path = output_dir.join(&filename);

        generator
            .generate_to_file(&schema, &output_path)
            .context("Failed to generate Vue component")?;

        println!("   ✅ {}", output_path.display());
        generated_count += 1;
    }

    // Run API docs generator
    if run_all || generators_list.contains(&"api-docs") {
        println!("📡 Generating API documentation...");
        let generator = ApiDocsGenerator::new()?;
        let filename = generator.get_output_filename(&schema);
        let output_path = output_dir.join(&filename);

        generator
            .generate_to_file(&schema, &output_path)
            .context("Failed to generate API documentation")?;

        println!("   ✅ {}", output_path.display());
        generated_count += 1;
    }

    // Run Google Colab generator
    if run_all || generators_list.contains(&"colab") {
        println!("📊 Generating Google Colab notebook...");
        let generator = ColabGenerator::new()?;
        let filename = generator.get_output_filename(&schema);
        let output_path = output_dir.join(&filename);

        generator
            .generate_to_file(&schema, &output_path)
            .context("Failed to generate Google Colab notebook")?;

        println!("   ✅ {}", output_path.display());
        generated_count += 1;
    }

    // Run LaTeX generator
    if run_all || generators_list.contains(&"latex") {
        println!("📄 Generating LaTeX documentation...");
        let generator = LatexGenerator::new()?;
        let filename = generator.get_output_filename(&schema);
        let output_path = output_dir.join(&filename);

        generator
            .generate_to_file(&schema, &output_path)
            .context("Failed to generate LaTeX documentation")?;

        println!("   ✅ {}", output_path.display());
        generated_count += 1;
    }

    // Run JSON schema data generator
    if run_all || generators_list.contains(&"json") {
        println!("📋 Generating JSON schema data...");
        let generator = JsonGenerator::new();
        let filename = generator.get_output_filename(&schema);
        let output_path = output_dir.join(&filename);

        generator
            .generate_to_file(&schema, &output_path)
            .context("Failed to generate JSON schema data")?;

        println!("   ✅ {}", output_path.display());
        generated_count += 1;
    }

    println!(
        "\n🎉 Successfully generated {} output file(s) in {}",
        generated_count,
        output_dir.display()
    );
    Ok(())
}

fn validate_command(schema_path: PathBuf) -> Result<()> {
    println!("🔍 Validating schema: {}\n", schema_path.display());

    let schema = Schema::load_from_file(&schema_path).context("Failed to load schema")?;

    println!("✅ Schema is valid!");
    println!("\n📋 Schema Summary:");
    println!("   Topic: {}", schema.topic);
    println!("   Title: {}", schema.title);
    println!("   Examples: {}", schema.examples.len());

    if let Some(article) = &schema.article {
        use kb_core::schema::Article;
        match article {
            Article::Simple(_) => {
                println!("   Article: Simple content");
            }
            Article::Structured(structured) => {
                println!("   Article sections: {}", structured.sections.len());
                println!("   Sidebars: {}", structured.sidebars.len());
                if let Some(conclusion) = &structured.conclusion {
                    println!("   Exercises: {}", conclusion.exercises.len());
                }
            }
        }
    }

    if !schema.related_topics.is_empty() {
        println!("   Related topics: {}", schema.related_topics.len());
    }

    Ok(())
}

fn list_command() {
    println!("📦 Available Generators:\n");
    println!("   jupyter    - Interactive Jupyter notebooks (.ipynb)");
    println!("   mdbook     - mdBook markdown documentation (.md)");
    println!("   llm-rag    - LLM-optimized RAG markdown (.rag.md)");
    println!("   vue        - Vue SSR site components (.vue)");
    println!("   api-docs   - OpenAPI 3.0 specifications (.openapi.json)");
    println!("   colab      - Google Colab notebooks (.ipynb)");
    println!("   latex      - LaTeX documentation (.tex)");
    println!("   json       - Schema data for Vue site (.json)");
    println!("\nUse 'all' to run all available generators.");
}

#[cfg(test)]
mod tests {
    // CLI tests would go here
}

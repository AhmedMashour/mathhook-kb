use anyhow::{Context, Result};
/// MathHook Knowledge Base CLI
///
/// Command-line interface for building, validating, and generating documentation
/// from KB schemas.
use clap::{Parser, Subcommand};
use kb_apidocs::ApiDocsGenerator;
use kb_colab::{ColabConfig, ColabGenerator, ColabManifest};
use kb_core::{generator::OutputGenerator, Schema};
use kb_json::JsonGenerator;
use kb_jupyter::JupyterGenerator;
use kb_latex::LatexGenerator;
use kb_llm_rag::LlmRagGenerator;
use kb_mdbook::MdBookGenerator;
use kb_sitemap::{
    generate_robots_txt_snippet, verify_robots_txt, BingSubmitter, IndexNowSubmitter,
    SitemapConfig, SitemapGenerator,
};
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

        /// Output directory (default: mathhook-docs-site/public/outputs)
        #[arg(short, long, default_value = "mathhook-docs-site/public/outputs")]
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

    /// Google Colab configuration and info
    #[command(subcommand)]
    Colab(ColabCommands),

    /// Generate sitemap and optionally submit to search engines
    #[command(subcommand)]
    Sitemap(SitemapCommands),
}

#[derive(Subcommand)]
enum ColabCommands {
    /// Show current Colab/GitHub configuration
    Config,

    /// Show info about generated notebooks and their Colab URLs
    Info {
        /// Path to manifest file
        #[arg(short, long, default_value = "mathhook-docs-site/public/outputs/colab/manifest.json")]
        manifest: PathBuf,
    },
}

#[derive(Subcommand)]
enum SitemapCommands {
    /// Generate sitemap.xml from schemas
    Generate {
        /// Path to schema file or directory
        #[arg(value_name = "SCHEMA")]
        schema_path: PathBuf,

        /// Output directory for sitemap files
        #[arg(short, long, default_value = "mathhook-docs-site/public")]
        output: PathBuf,

        /// Base URL for the site (e.g., https://mathhook.org)
        #[arg(short, long, default_value = "https://mathhook.org")]
        base_url: String,

        /// URL path prefix for docs (e.g., /docs)
        #[arg(long, default_value = "/docs")]
        docs_prefix: String,

        /// Also generate robots.txt snippet
        #[arg(long)]
        robots_txt: bool,
    },

    /// Submit URLs to IndexNow (Bing, Yandex, Seznam, Naver)
    IndexNow {
        /// IndexNow API key (get one at https://www.bing.com/indexnow)
        #[arg(short, long, env = "INDEXNOW_API_KEY")]
        api_key: String,

        /// Host URL (e.g., https://mathhook.org)
        #[arg(short = 'H', long, default_value = "https://mathhook.org")]
        host: String,

        /// URLs to submit (or use --from-sitemap)
        #[arg(value_name = "URL")]
        urls: Vec<String>,

        /// Read URLs from generated sitemap
        #[arg(long)]
        from_sitemap: Option<PathBuf>,
    },

    /// Check if robots.txt contains sitemap directive
    Verify {
        /// Base URL for the site
        #[arg(short, long, default_value = "https://mathhook.org")]
        base_url: String,

        /// Sitemap URL to verify
        #[arg(short, long, default_value = "https://mathhook.org/sitemap.xml")]
        sitemap_url: String,
    },

    /// Show sitemap configuration and setup instructions
    Config {
        /// Base URL for the site
        #[arg(short, long, default_value = "https://mathhook.org")]
        base_url: String,
    },

    /// Submit URLs via Bing Webmaster API (verified sites only)
    Bing {
        /// Bing Webmaster API key (from Settings → API Access)
        #[arg(short, long, env = "BING_WEBMASTER_API_KEY")]
        api_key: String,

        /// Site URL (must be verified in Bing Webmaster Tools)
        #[arg(short, long, default_value = "https://mathhook.org")]
        site_url: String,

        /// Submit URLs from sitemap file
        #[arg(long)]
        from_sitemap: Option<PathBuf>,

        /// Submit sitemap URL directly
        #[arg(long)]
        submit_sitemap: Option<String>,

        /// Individual URLs to submit
        #[arg(value_name = "URL")]
        urls: Vec<String>,

        /// Check remaining quota
        #[arg(long)]
        quota: bool,
    },
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
        Commands::Colab(colab_cmd) => handle_colab_command(colab_cmd),
        Commands::Sitemap(sitemap_cmd) => handle_sitemap_command(sitemap_cmd),
    }
}

fn build_command(
    schema_path: PathBuf,
    output_dir: PathBuf,
    generators_str: String,
) -> Result<()> {
    println!("🔨 Building documentation...\n");

    // Collect all schemas to process
    let schemas = collect_schemas(&schema_path)?;

    // Create output subdirectories for each type
    let jupyter_dir = output_dir.join("jupyter");
    let mdbook_dir = output_dir.join("mdbook");
    let llm_rag_dir = output_dir.join("llm-rag");
    let vue_dir = output_dir.join("vue");
    let api_docs_dir = output_dir.join("api-docs");
    let colab_dir = output_dir.join("colab");
    let latex_dir = output_dir.join("latex");
    let json_dir = output_dir.join("json");

    // Parse generators to run
    let generators_list: Vec<&str> = generators_str.split(',').collect();
    let run_all = generators_list.contains(&"all");
    let run_jupyter = run_all || generators_list.contains(&"jupyter");
    let run_mdbook = run_all || generators_list.contains(&"mdbook");
    let run_llm_rag = run_all || generators_list.contains(&"llm-rag");
    let run_vue = run_all || generators_list.contains(&"vue");
    let run_api_docs = run_all || generators_list.contains(&"api-docs");
    let run_colab = run_all || generators_list.contains(&"colab");
    let run_latex = run_all || generators_list.contains(&"latex");
    let run_json = run_all || generators_list.contains(&"json");

    // Create directories for enabled generators
    if run_jupyter {
        std::fs::create_dir_all(&jupyter_dir).context("Failed to create jupyter directory")?;
    }
    if run_mdbook {
        std::fs::create_dir_all(&mdbook_dir).context("Failed to create mdbook directory")?;
    }
    if run_llm_rag {
        std::fs::create_dir_all(&llm_rag_dir).context("Failed to create llm-rag directory")?;
    }
    if run_vue {
        std::fs::create_dir_all(&vue_dir).context("Failed to create vue directory")?;
    }
    if run_api_docs {
        std::fs::create_dir_all(&api_docs_dir).context("Failed to create api-docs directory")?;
    }
    if run_colab {
        std::fs::create_dir_all(&colab_dir).context("Failed to create colab directory")?;
    }
    if run_latex {
        std::fs::create_dir_all(&latex_dir).context("Failed to create latex directory")?;
    }
    if run_json {
        std::fs::create_dir_all(&json_dir).context("Failed to create json directory")?;
    }

    let mut total_generated = 0;

    // Setup Colab manifest if we're generating Colab notebooks
    let colab_config = ColabConfig::default();
    let mut colab_manifest = ColabManifest::new(colab_config.clone());

    for schema_path in &schemas {
        println!("📄 Loading schema: {}", schema_path.display());
        let schema = Schema::load_from_file(schema_path).context("Failed to load schema")?;

        println!("   Topic: {}", schema.topic);
        println!("   Title: {}\n", schema.title);

        let mut generated_count = 0;

        // Extract category from topic for organization
        let category = schema.topic.split('.').next().unwrap_or("misc");

        // Run Jupyter generator
        if run_jupyter {
            println!("📓 Generating Jupyter notebook...");
            let generator = JupyterGenerator::new();
            let filename = generator.get_output_filename(&schema);

            // Organize by category
            let category_dir = jupyter_dir.join(category);
            std::fs::create_dir_all(&category_dir)?;
            let output_path = category_dir.join(&filename);

            generator
                .generate_to_file(&schema, &output_path)
                .context("Failed to generate Jupyter notebook")?;

            println!("   ✅ {}", output_path.display());
            generated_count += 1;
        }

        // Run mdBook generator
        if run_mdbook {
            println!("📚 Generating mdBook markdown...");
            let generator = MdBookGenerator::new()?;
            let filename = generator.get_output_filename(&schema);

            // Organize by category
            let category_dir = mdbook_dir.join(category);
            std::fs::create_dir_all(&category_dir)?;
            let output_path = category_dir.join(&filename);

            generator
                .generate_to_file(&schema, &output_path)
                .context("Failed to generate mdBook markdown")?;

            println!("   ✅ {}", output_path.display());
            generated_count += 1;
        }

        // Run LLM-RAG generator
        if run_llm_rag {
            println!("🤖 Generating LLM-RAG markdown...");
            let generator = LlmRagGenerator::from_schema(&schema);
            let filename = format!("{}.rag.md", schema.topic.replace('.', "-"));

            // Organize by category
            let category_dir = llm_rag_dir.join(category);
            std::fs::create_dir_all(&category_dir)?;
            let output_path = category_dir.join(&filename);

            generator
                .generate_to_file(&schema, &output_path)
                .context("Failed to generate LLM-RAG markdown")?;

            println!("   ✅ {}", output_path.display());
            generated_count += 1;
        }

        // Run Vue generator
        if run_vue {
            println!("🎨 Generating Vue SSR component...");
            let generator = VueGenerator::new()?;
            let filename = generator.get_output_filename(&schema);

            // Organize by category
            let category_dir = vue_dir.join(category);
            std::fs::create_dir_all(&category_dir)?;
            let output_path = category_dir.join(&filename);

            generator
                .generate_to_file(&schema, &output_path)
                .context("Failed to generate Vue component")?;

            println!("   ✅ {}", output_path.display());
            generated_count += 1;
        }

        // Run API docs generator
        if run_api_docs {
            println!("📡 Generating API documentation...");
            let generator = ApiDocsGenerator::new()?;
            let filename = generator.get_output_filename(&schema);

            // Organize by category
            let category_dir = api_docs_dir.join(category);
            std::fs::create_dir_all(&category_dir)?;
            let output_path = category_dir.join(&filename);

            generator
                .generate_to_file(&schema, &output_path)
                .context("Failed to generate API documentation")?;

            println!("   ✅ {}", output_path.display());
            generated_count += 1;
        }

        // Run Google Colab generator (organized by category)
        if run_colab {
            println!("📊 Generating Google Colab notebook...");
            let generator = ColabGenerator::with_config(colab_config.clone());
            let filename = generator.get_output_filename(&schema);

            // Create category directory inside colab
            let category_dir = colab_dir.join(category);
            std::fs::create_dir_all(&category_dir)?;

            let output_path = category_dir.join(&filename);

            generator
                .generate_to_file(&schema, &output_path)
                .context("Failed to generate Google Colab notebook")?;

            println!("   ✅ {}", output_path.display());
            generated_count += 1;

            // Add to manifest
            colab_manifest.add(&schema.topic, &schema.title, &filename);
        }

        // Run LaTeX generator
        if run_latex {
            println!("📄 Generating LaTeX documentation...");
            let generator = LatexGenerator::new()?;
            let filename = generator.get_output_filename(&schema);

            // Organize by category
            let category_dir = latex_dir.join(category);
            std::fs::create_dir_all(&category_dir)?;
            let output_path = category_dir.join(&filename);

            generator
                .generate_to_file(&schema, &output_path)
                .context("Failed to generate LaTeX documentation")?;

            println!("   ✅ {}", output_path.display());
            generated_count += 1;
        }

        // Run JSON schema data generator
        if run_json {
            println!("📋 Generating JSON schema data...");
            let generator = JsonGenerator::new();
            let filename = generator.get_output_filename(&schema);

            // Organize by category
            let category_dir = json_dir.join(category);
            std::fs::create_dir_all(&category_dir)?;
            let output_path = category_dir.join(&filename);

            generator
                .generate_to_file(&schema, &output_path)
                .context("Failed to generate JSON schema data")?;

            println!("   ✅ {}", output_path.display());
            generated_count += 1;
        }

        total_generated += generated_count;
        println!();
    }

    // Save Colab manifest and generate READMEs if we generated Colab notebooks
    if run_colab && colab_manifest.total_notebooks > 0 {
        println!("📋 Generating Colab manifest and READMEs...");

        // Save manifest
        let manifest_path = colab_dir.join("manifest.json");
        colab_manifest
            .save(&manifest_path)
            .context("Failed to save Colab manifest")?;
        println!("   ✅ {}", manifest_path.display());

        // Generate main README
        let readme_content = colab_manifest.generate_readme();
        let readme_path = colab_dir.join("README.md");
        std::fs::write(&readme_path, readme_content)?;
        println!("   ✅ {}", readme_path.display());

        // Generate category READMEs
        for category in colab_manifest.sorted_categories() {
            if let Some(category_readme) = colab_manifest.generate_category_readme(category) {
                let category_readme_path = colab_dir.join(category).join("README.md");
                std::fs::write(&category_readme_path, category_readme)?;
                println!("   ✅ {}", category_readme_path.display());
            }
        }

        println!();
        println!("🔗 Colab notebooks available locally at: {}", colab_dir.display());
        println!("   (Also hosted on GitHub for direct Colab access)");
        println!();
    }

    println!(
        "🎉 Successfully generated {} output file(s) from {} schema(s)",
        total_generated,
        schemas.len()
    );
    println!("   📁 Output directory: {}", output_dir.display());

    Ok(())
}

/// Collect all schema files from path (file or directory, recursive)
fn collect_schemas(path: &PathBuf) -> Result<Vec<PathBuf>> {
    let mut schemas = Vec::new();

    if path.is_file() {
        schemas.push(path.clone());
    } else if path.is_dir() {
        collect_schemas_recursive(path, &mut schemas)?;
        schemas.sort();
    }

    if schemas.is_empty() {
        anyhow::bail!("No schema files found at {:?}", path);
    }

    println!("📂 Found {} schema files\n", schemas.len());
    Ok(schemas)
}

/// Recursively collect schema files from directory
fn collect_schemas_recursive(dir: &PathBuf, schemas: &mut Vec<PathBuf>) -> Result<()> {
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext == "yaml" || ext == "yml" {
                    schemas.push(path);
                }
            }
        } else if path.is_dir() {
            collect_schemas_recursive(&path, schemas)?;
        }
    }
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
    println!("   colab      - Google Colab notebooks (.colab.ipynb)");
    println!("   latex      - LaTeX documentation (.tex)");
    println!("   json       - Schema data for Vue site (.json)");
    println!("\nUse 'all' to run all available generators.");
    println!("\n📓 Colab Notebooks:\n");
    println!("   Colab notebooks are organized by category and hosted on GitHub.");
    println!("   Users can open them directly in Google Colab via the badge links.");
    println!("\n   Configure via environment variables:");
    println!("     COLAB_GITHUB_USER    - GitHub username (default: AhmedMashour)");
    println!("     COLAB_GITHUB_REPO    - Repository name (default: mathhook)");
    println!("     COLAB_GITHUB_BRANCH  - Branch name (default: main)");
    println!("     COLAB_NOTEBOOKS_PATH - Notebooks path (default: colab-notebooks)");
    println!("\n🔧 Colab Commands:\n");
    println!("   kb colab config   Show current Colab/GitHub configuration");
    println!("   kb colab info     Show info about generated notebooks");
    println!("\n🗺️  Sitemap Commands:\n");
    println!("   kb sitemap generate <SCHEMA>  Generate sitemap.xml from schemas");
    println!("   kb sitemap indexnow           Submit URLs via IndexNow (Bing, Yandex)");
    println!("   kb sitemap verify             Verify robots.txt contains sitemap");
    println!("   kb sitemap config             Show setup guide and configuration");
    println!("\n   Full workflow:");
    println!("     kb sitemap generate schemas/ --robots-txt");
    println!("     kb sitemap indexnow --from-sitemap sitemap.xml -a YOUR_KEY");
}

fn handle_colab_command(cmd: ColabCommands) -> Result<()> {
    match cmd {
        ColabCommands::Config => {
            let config = ColabConfig::default();

            println!("📊 Google Colab Configuration\n");
            println!("GitHub Settings:");
            println!("   User/Org:      {}", config.github_user);
            println!("   Repository:    {}", config.github_repo);
            println!("   Branch:        {}", config.github_branch);
            println!("   Notebooks Dir: {}", config.notebooks_path);
            println!();
            println!("Example Colab URL:");
            println!(
                "   {}",
                config.get_colab_url("calculus", "calculus-derivative.colab.ipynb")
            );
            println!();
            println!("To customize, set environment variables:");
            println!("   export COLAB_GITHUB_USER=YourUser");
            println!("   export COLAB_GITHUB_REPO=your-repo");
            println!("   export COLAB_GITHUB_BRANCH=main");
            println!("   export COLAB_NOTEBOOKS_PATH=notebooks");
        }

        ColabCommands::Info { manifest } => {
            if !manifest.exists() {
                println!("❌ No manifest found at: {}", manifest.display());
                println!("\nRun `kb build` with the colab generator first.");
                return Ok(());
            }

            let manifest = ColabManifest::load(&manifest)?;

            println!("📊 Colab Notebooks Info\n");
            println!("Total notebooks: {}\n", manifest.total_notebooks);

            println!("GitHub Repository:");
            println!(
                "   https://github.com/{}/{}",
                manifest.config.github_user, manifest.config.github_repo
            );
            println!();

            println!("Categories:");
            for category in manifest.sorted_categories() {
                let notebooks = &manifest.categories[category];
                println!("\n  📁 {} ({} notebooks)", category, notebooks.len());
                for nb in notebooks {
                    println!("      • {}", nb.title);
                    println!("        Colab: {}", nb.colab_url);
                }
            }
        }
    }

    Ok(())
}

fn handle_sitemap_command(cmd: SitemapCommands) -> Result<()> {
    match cmd {
        SitemapCommands::Generate {
            schema_path,
            output,
            base_url,
            docs_prefix,
            robots_txt,
        } => {
            println!("🗺️  Generating sitemap...\n");

            // Collect all schemas
            let schemas = collect_schemas(&schema_path)?;

            // Configure sitemap generator
            let config = SitemapConfig::new(&base_url).with_docs_prefix(&docs_prefix);
            let generator = SitemapGenerator::new(config);

            // Load schemas
            let mut loaded_schemas = Vec::new();
            for path in &schemas {
                let schema = Schema::load_from_file(path).context("Failed to load schema")?;
                loaded_schemas.push(schema);
            }

            // Generate and write sitemap(s)
            std::fs::create_dir_all(&output)?;
            let written_files = generator
                .write_to_directory(&loaded_schemas, &output)
                .map_err(|e| anyhow::anyhow!("{}", e))?;

            for file in &written_files {
                println!("   ✅ {}", output.join(file).display());
            }

            let sitemap_count = loaded_schemas.len() + 2; // +2 for homepage and docs index
            println!(
                "\n🎉 Generated sitemap with {} URLs from {} schema(s)",
                sitemap_count,
                loaded_schemas.len()
            );

            // Generate robots.txt snippet if requested
            if robots_txt {
                let sitemap_url = format!("{}/sitemap.xml", base_url.trim_end_matches('/'));
                let robots_content = generate_robots_txt_snippet(&sitemap_url);
                let robots_path = output.join("robots.txt");
                std::fs::write(&robots_path, &robots_content)?;
                println!("\n📄 Generated robots.txt:");
                println!("   ✅ {}", robots_path.display());
            }

            println!("\n📋 Next steps to notify search engines:");
            println!("   1. Deploy sitemap.xml to your site root");
            println!("   2. Add to robots.txt: Sitemap: {}/sitemap.xml", base_url);
            println!("   3. Submit to Google Search Console: https://search.google.com/search-console");
            println!("   4. (Optional) Use IndexNow for instant indexing: kb sitemap indexnow");

            Ok(())
        }

        SitemapCommands::IndexNow {
            api_key,
            host,
            urls,
            from_sitemap,
        } => {
            println!("🚀 Submitting to IndexNow...\n");

            let urls_to_submit = if let Some(sitemap_path) = from_sitemap {
                // Parse URLs from sitemap
                let content = std::fs::read_to_string(&sitemap_path)
                    .context("Failed to read sitemap file")?;
                extract_urls_from_sitemap(&content)
            } else if !urls.is_empty() {
                urls
            } else {
                anyhow::bail!("Please provide URLs or use --from-sitemap");
            };

            if urls_to_submit.is_empty() {
                println!("   No URLs to submit.");
                return Ok(());
            }

            println!("   Host: {}", host);
            println!("   API Key: {}...", &api_key[..8.min(api_key.len())]);
            println!("   URLs to submit: {}\n", urls_to_submit.len());

            let submitter = IndexNowSubmitter::new(&api_key, &host);
            match submitter.submit_urls(&urls_to_submit) {
                Ok(_) => {
                    println!("   ✅ Successfully submitted {} URLs to IndexNow!", urls_to_submit.len());
                    println!("\n   IndexNow shares submissions with: Bing, Yandex, Seznam, Naver");
                }
                Err(e) => {
                    println!("   ❌ Failed to submit: {}", e);
                    println!("\n   Make sure you have:");
                    println!("   1. Created verification file: {}/{}.txt", host, api_key);
                    println!("   2. The file contains only: {}", api_key);
                }
            }

            Ok(())
        }

        SitemapCommands::Verify { base_url, sitemap_url } => {
            println!("🔍 Verifying sitemap configuration...\n");

            match verify_robots_txt(&base_url, &sitemap_url) {
                Ok(true) => {
                    println!("   ✅ robots.txt contains sitemap directive");
                    println!("   Sitemap URL: {}", sitemap_url);
                }
                Ok(false) => {
                    println!("   ❌ robots.txt does NOT contain sitemap directive");
                    println!("\n   Add this to your robots.txt:");
                    println!("   Sitemap: {}", sitemap_url);
                }
                Err(e) => {
                    println!("   ❌ Could not fetch robots.txt: {}", e);
                }
            }

            Ok(())
        }

        SitemapCommands::Config { base_url } => {
            println!("🗺️  Sitemap Configuration & Setup Guide\n");
            println!("═══════════════════════════════════════════════════════════════\n");

            println!("📍 Site Configuration:");
            println!("   Base URL:     {}", base_url);
            println!("   Docs Prefix:  /docs");
            println!("   Sitemap URL:  {}/sitemap.xml", base_url);
            println!();

            println!("📄 Generated sitemap includes:");
            println!("   • Homepage:   {} (priority: 1.0, weekly)", base_url);
            println!("   • Docs index: {}/docs (priority: 0.9, weekly)", base_url);
            println!("   • Doc pages:  {}/docs/{{topic}} (from schema SEO)", base_url);
            println!();

            println!("═══════════════════════════════════════════════════════════════\n");
            println!("🔧 SETUP STEPS:\n");

            println!("1️⃣  GENERATE SITEMAP:");
            println!("    kb sitemap generate schemas/ --robots-txt\n");

            println!("2️⃣  DEPLOY FILES:");
            println!("    • sitemap.xml → {}/sitemap.xml", base_url);
            println!("    • robots.txt  → {}/robots.txt", base_url);
            println!();

            println!("3️⃣  GOOGLE (manual submission required since June 2023):");
            println!("    • Go to: https://search.google.com/search-console");
            println!("    • Add property for {}", base_url);
            println!("    • Go to Sitemaps → Add sitemap: sitemap.xml");
            println!();

            println!("4️⃣  BING WEBMASTER (recommended for verified sites):");
            println!("    • Go to: https://www.bing.com/webmasters");
            println!("    • Verify site ownership");
            println!("    • Go to Settings → API Access → Generate API key");
            println!("    • Submit via: kb sitemap bing --submit-sitemap {}/sitemap.xml -a YOUR_KEY", base_url);
            println!();

            println!("5️⃣  INDEXNOW (instant indexing, requires verification):");
            println!("    • After Bing Webmaster verification, use:");
            println!("       kb sitemap bing --from-sitemap sitemap.xml -a YOUR_KEY");
            println!();

            println!("═══════════════════════════════════════════════════════════════\n");
            println!("💡 TIP: Run 'kb sitemap verify' to check your robots.txt setup");

            Ok(())
        }

        SitemapCommands::Bing {
            api_key,
            site_url,
            from_sitemap,
            submit_sitemap,
            urls,
            quota,
        } => {
            println!("🔵 Bing Webmaster API\n");

            let submitter = BingSubmitter::new(&api_key, &site_url);

            // Check quota if requested
            if quota {
                println!("📊 Checking submission quota...\n");
                match submitter.get_quota() {
                    Ok(q) => {
                        println!("   Daily remaining:   {} URLs", q.daily_remaining);
                        if q.monthly_remaining > 0 {
                            println!("   Monthly remaining: {} URLs", q.monthly_remaining);
                        } else {
                            println!("   Monthly remaining: Unlimited");
                        }
                    }
                    Err(e) => {
                        println!("   ❌ Failed to check quota: {}", e);
                        println!("   Make sure your API key is correct.");
                    }
                }
                println!();
            }

            // Submit sitemap directly
            if let Some(sitemap_url) = submit_sitemap {
                println!("📤 Submitting sitemap: {}\n", sitemap_url);
                match submitter.submit_sitemap(&sitemap_url) {
                    Ok(_) => {
                        println!("   ✅ Sitemap submitted successfully!");
                        println!("   Bing will process and index your URLs.");
                    }
                    Err(e) => {
                        println!("   ❌ Failed to submit sitemap: {}", e);
                    }
                }
                return Ok(());
            }

            // Collect URLs to submit
            let urls_to_submit = if let Some(sitemap_path) = from_sitemap {
                let content = std::fs::read_to_string(&sitemap_path)
                    .context("Failed to read sitemap file")?;
                extract_urls_from_sitemap(&content)
            } else if !urls.is_empty() {
                urls
            } else if !quota {
                anyhow::bail!("Please provide URLs, --from-sitemap, --submit-sitemap, or --quota");
            } else {
                return Ok(());
            };

            if urls_to_submit.is_empty() {
                println!("   No URLs to submit.");
                return Ok(());
            }

            println!("📤 Submitting {} URLs to Bing...\n", urls_to_submit.len());
            println!("   Site: {}", site_url);
            println!("   API Key: {}...", &api_key[..8.min(api_key.len())]);
            println!();

            match submitter.submit_urls(&urls_to_submit) {
                Ok(_) => {
                    println!("   ✅ Successfully submitted {} URLs to Bing!", urls_to_submit.len());
                    println!("\n   URLs will be crawled and indexed by Bing.");
                    println!("   Check status at: https://www.bing.com/webmasters");
                }
                Err(e) => {
                    println!("   ❌ Failed to submit URLs: {}", e);
                    println!("\n   Troubleshooting:");
                    println!("   1. Verify your API key in Bing Webmaster → Settings → API Access");
                    println!("   2. Make sure {} is verified in Bing Webmaster", site_url);
                    println!("   3. Check your daily quota: kb sitemap bing --quota -a YOUR_KEY");
                }
            }

            Ok(())
        }
    }
}

/// Extract URLs from sitemap XML
fn extract_urls_from_sitemap(content: &str) -> Vec<String> {
    let mut urls = Vec::new();
    // Simple regex-free extraction
    for line in content.lines() {
        if let Some(start) = line.find("<loc>") {
            if let Some(end) = line.find("</loc>") {
                let url = &line[start + 5..end];
                urls.push(url.to_string());
            }
        }
    }
    urls
}

#[cfg(test)]
mod tests {
    // CLI tests would go here
}

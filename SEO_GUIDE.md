# SEO Optimization Guide for MathHook Knowledge Base

## Overview

The MathHook Knowledge Base system includes comprehensive SEO (Search Engine Optimization) features to maximize documentation discoverability on search engines and improve social media sharing.

## SEO Schema Fields

All SEO metadata is optional but highly recommended for production documentation.

### Primary Keywords (3-5 most important)
```yaml
seo:
  keywords:
    - "symbolic differentiation"
    - "derivative calculator"
    - "automatic differentiation"
```

**Best Practices**:
- Focus on terms your target audience searches for
- Include your main topic + variations
- Use keyword research tools (Google Keyword Planner, Ahrefs, SEMrush)

### Secondary Keywords (5-10 related terms)
```yaml
  secondary_keywords:
    - "chain rule calculator"
    - "product rule differentiation"
    - "power rule derivative"
```

**Purpose**:
- Capture long-tail search queries
- Related topics and alternative phrasing
- Technical variations and synonyms

### Meta Description (150-160 characters)
```yaml
  meta_description: "Compute symbolic derivatives with MathHook's powerful CAS. Supports chain rule, product rule, power rule. Fast, accurate, and easy to use for Rust, Python, and JavaScript."
```

**Critical for SEO**:
- Shown in Google search results (affects click-through rate)
- Must be compelling and informative
- Include primary keyword
- Stay within 150-160 character limit

### Canonical URL
```yaml
  canonical_url: "https://docs.mathhook.org/calculus/derivative"
```

**Prevents Duplicate Content Issues**:
- Tells search engines which URL is the "main" version
- Critical when content appears on multiple domains
- Helps consolidate page rank

### Open Graph Tags (Social Media)
```yaml
  og_title: "Symbolic Differentiation - MathHook Computer Algebra System"
  og_description: "Professional-grade symbolic differentiation library..."
  og_image: "https://docs.mathhook.org/images/derivatives-og.png"
```

**Social Media Optimization**:
- Controls how links appear when shared on Facebook, LinkedIn, Twitter
- `og_image` should be 1200x630px for best results
- Preview your cards: https://www.opengraph.xyz/

### Twitter Card
```yaml
  twitter_card: "summary_large_image"  # or "summary"
```

**Options**:
- `summary`: Small image on left, text on right
- `summary_large_image`: Large image above text (recommended)

### Schema.org Structured Data
```yaml
  schema_org_type: "TechArticle"  # or "HowTo", "Article", "SoftwareApplication"
```

**Rich Search Results**:
- Enables Google rich snippets (star ratings, breadcrumbs, etc.)
- Improves search result appearance
- Types: https://schema.org/docs/schemas.html

### Sitemap Configuration
```yaml
  priority: 0.9  # 0.0 to 1.0
  change_frequency: "monthly"  # always, hourly, daily, weekly, monthly, yearly, never
```

**Sitemap Generation**:
- `priority`: Relative importance (1.0 = most important)
- `change_frequency`: How often content updates
- Used by search engines for crawl prioritization

### Language and Internationalization
```yaml
  language: "en"
  alternate_languages:
    es: "https://docs.mathhook.org/es/calculus/derivative"
    fr: "https://docs.mathhook.org/fr/calculus/derivative"
```

**Multilingual SEO**:
- Helps search engines serve correct language version
- `alternate_languages` creates hreflang tags
- Improves international search performance

## SEO Integration in Generators

### mdBook Generator

The mdBook generator emits SEO metadata in YAML frontmatter:

```markdown
---
description: "Meta description here"
keywords: keyword1, keyword2, keyword3
canonical_url: "https://..."
og:title: "Open Graph title"
og:description: "OG description"
og:image: "https://..."
twitter:card: "summary_large_image"
schema_type: "TechArticle"
language: "en"
---

# Page Title
...
```

**mdBook Configuration** (`book.toml`):
```toml
[output.html]
additional-css = ["seo.css"]
additional-js = ["seo.js"]

[output.html.head]
# SEO meta tags will be injected from frontmatter
```

### LLM-RAG Generator

Chunks include SEO metadata for better vector database retrieval:

```markdown
---
chunk_id: calculus_derivative::0
topic: calculus.derivative
title: Power Rule
description: "Meta description for semantic search"
seo_keywords:
  - symbolic differentiation
  - derivative calculator
canonical_url: "https://..."
---
```

**Benefits**:
- Semantic search matches on SEO keywords
- Description provides context for embeddings
- Canonical URL for source attribution

## SEO Checklist

### Before Publishing

- [ ] **Primary keywords** (3-5) selected and validated
- [ ] **Meta description** written (150-160 chars, compelling)
- [ ] **Canonical URL** set to production URL
- [ ] **Open Graph tags** complete (title, description, image)
- [ ] **OG image** created (1200x630px, shows well on social)
- [ ] **Schema.org type** selected appropriately
- [ ] **Priority** set (0.9 for important, 0.5 for average, 0.3 for less important)
- [ ] **Change frequency** set realistically

### After Publishing

- [ ] **Test Open Graph**: Use https://www.opengraph.xyz/
- [ ] **Test Twitter Card**: Use https://cards-dev.twitter.com/validator
- [ ] **Test Rich Snippets**: Use https://search.google.com/test/rich-results
- [ ] **Submit sitemap** to Google Search Console
- [ ] **Monitor rankings** for primary keywords
- [ ] **Track click-through rate** in Search Console

## Best Practices by Content Type

### API Documentation
```yaml
seo:
  keywords: ["API reference", "function name", "library name"]
  schema_org_type: "TechArticle"
  priority: 0.8
  change_frequency: "weekly"
```

### Tutorials
```yaml
seo:
  keywords: ["how to [task]", "tutorial", "step by step"]
  schema_org_type: "HowTo"
  priority: 0.9
  change_frequency: "monthly"
```

### Conceptual Guides
```yaml
seo:
  keywords: ["concept name", "explanation", "guide"]
  schema_org_type: "Article"
  priority: 0.7
  change_frequency: "monthly"
```

### Examples/Recipes
```yaml
seo:
  keywords: ["example", "use case", "recipe", "pattern"]
  schema_org_type: "HowTo"
  priority: 0.6
  change_frequency: "yearly"
```

## Keyword Research Tips

1. **Use Google Autocomplete**: Type your topic, see what Google suggests
2. **Check "People Also Ask"**: Great source of related queries
3. **Analyze Competitors**: See what they rank for
4. **Use Keyword Tools**:
   - Google Keyword Planner (free, needs Google Ads account)
   - Ahrefs Keywords Explorer (paid)
   - SEMrush (paid)
   - Ubersuggest (freemium)

## Common SEO Mistakes to Avoid

1. **Keyword Stuffing**: Don't repeat keywords unnaturally
2. **Duplicate Meta Descriptions**: Each page needs unique description
3. **Missing Canonical URLs**: Always set for production docs
4. **Wrong Priority Values**: Don't set everything to 1.0
5. **Outdated Change Frequency**: Match reality (not wishful thinking)
6. **Missing OG Images**: Social shares need images for engagement
7. **Too-Long Meta Descriptions**: Google truncates at ~160 chars
8. **Generic Descriptions**: Be specific about content value

## Measuring SEO Success

### Key Metrics

1. **Organic Traffic**: Users from search engines
2. **Keyword Rankings**: Position in search results for target keywords
3. **Click-Through Rate (CTR)**: % of impressions that result in clicks
4. **Time on Page**: Indicates content quality
5. **Bounce Rate**: % of users who leave immediately
6. **Backlinks**: Other sites linking to your docs

### Tools

- **Google Search Console**: Free, essential for SEO
- **Google Analytics**: Track traffic and user behavior
- **Ahrefs/SEMrush**: Comprehensive SEO analytics (paid)
- **Screaming Frog**: Technical SEO auditing (freemium)

## Advanced SEO Features (Future)

### Automatic Sitemap Generation
```rust
// Generate sitemap.xml from all schemas
let schemas = Schema::load_from_directory("schemas/")?;
let sitemap = SitemapGenerator::from_schemas(&schemas);
sitemap.write_to_file("docs/sitemap.xml")?;
```

### Structured Data JSON-LD
```html
<script type="application/ld+json">
{
  "@context": "https://schema.org",
  "@type": "TechArticle",
  "headline": "Symbolic Differentiation",
  "author": {
    "@type": "Organization",
    "name": "MathHook Team"
  },
  "datePublished": "2024-11-17",
  "description": "..."
}
</script>
```

### Automatic Keyword Extraction
```rust
// Extract keywords from content automatically
let seo_analyzer = SeoAnalyzer::new();
let suggested_keywords = seo_analyzer.extract_keywords(&schema.description);
```

## Example: Complete SEO-Optimized Schema

See `schemas/examples/derivative-with-seo.yaml` for a complete example with all SEO fields populated.

## Resources

- [Google SEO Starter Guide](https://developers.google.com/search/docs/beginner/seo-starter-guide)
- [Schema.org Documentation](https://schema.org/)
- [Open Graph Protocol](https://ogp.me/)
- [Twitter Card Documentation](https://developer.twitter.com/en/docs/twitter-for-websites/cards)
- [Google Rich Results Test](https://search.google.com/test/rich-results)
- [mdBook Documentation](https://rust-lang.github.io/mdBook/)

## Questions?

For SEO support, consult with an SEO specialist or use tools like:
- Moz (https://moz.com/)
- Ahrefs Academy (https://ahrefs.com/academy)
- Google Search Central (https://developers.google.com/search)

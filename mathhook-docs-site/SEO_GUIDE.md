# MathHook Documentation - SEO Guide

## How Dynamic Routes Work with SEO

### The Problem (Misunderstanding)
Some people think dynamic routes like `[topic].vue` are bad for SEO because the URL contains a variable. **This is wrong!**

### The Reality
Dynamic routes with **Server-Side Rendering (SSR)** are **EXCELLENT** for SEO because:

1. **Each URL is unique and crawlable**:
   - `/docs/ode-first_order-linear` is a real, unique URL
   - `/docs/getting-started-quick-start` is another real, unique URL
   - Google sees these as separate pages, not variables

2. **SSR delivers complete HTML immediately**:
   ```html
   <!-- What Googlebot sees when crawling /docs/ode-first_order-linear -->
   <!DOCTYPE html>
   <html>
   <head>
     <title>Linear First-Order Differential Equations - MathHook KB</title>
     <meta name="description" content="Solve linear first-order ODEs using...">
   </head>
   <body>
     <h1>Linear First-Order Differential Equations</h1>
     <!-- ALL CONTENT IS HERE, FULLY RENDERED -->
   </body>
   </html>
   ```

3. **No JavaScript required for indexing**:
   - Content is in the HTML source, not loaded by JavaScript
   - Googlebot doesn't need to execute JavaScript to see content
   - Faster indexing, better ranking

## Our SEO Implementation

### 1. Server-Side Rendering (SSR) ✅
**File**: `nuxt.config.ts`
```typescript
export default defineNuxtConfig({
  ssr: true  // ← This is the magic
})
```

**What it does**:
- Nuxt runs Vue on the server
- Generates complete HTML for each request
- Sends fully-rendered page to browser/crawler
- JavaScript is for interactivity, not content loading

### 2. Dynamic Meta Tags ✅
**File**: `pages/docs/[topic].vue`

Every page has unique SEO metadata:

```javascript
useHead(() => ({
  title: `${schema.value.title} - MathHook KB`,
  meta: [
    {
      name: 'description',
      content: schema.value.description
    },
    {
      name: 'keywords',
      content: `mathhook, ${schema.value.topic}, mathematics, ...`
    },
    // Open Graph for social media
    {
      property: 'og:title',
      content: schema.value.title
    },
    // Twitter cards
    {
      name: 'twitter:card',
      content: 'summary_large_image'
    }
  ]
}))
```

**Result**:
- Each page has unique title (critical for Google)
- Each page has unique description (shows in search results)
- Social media previews look professional
- Keywords help with discovery

### 3. Structured Data (Schema.org) ✅
**File**: `pages/docs/[topic].vue`

We inject JSON-LD structured data:

```javascript
script: [
  {
    type: 'application/ld+json',
    children: JSON.stringify({
      '@context': 'https://schema.org',
      '@type': 'TechArticle',
      headline: schema.value.title,
      description: schema.value.description,
      author: { '@type': 'Organization', name: 'MathHook Team' },
      // ... more structured data
    })
  }
]
```

**What this does**:
- Google understands the page is an educational article
- Can show rich snippets in search results
- Better categorization in Google's knowledge graph
- Potentially eligible for featured snippets

### 4. Canonical URLs ✅
```javascript
link: [
  {
    rel: 'canonical',
    href: `https://mathhook.dev/docs/${topic}`
  }
]
```

**Why important**:
- Prevents duplicate content issues
- Tells Google this is the authoritative version
- Important if you have multiple domains (e.g., mathhook.dev and www.mathhook.dev)

### 5. Sitemap Generation 🟡 (Partially Complete)
**File**: `server/routes/sitemap.xml.ts`

Auto-generates sitemap with all documentation URLs:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
  <url>
    <loc>https://mathhook.dev/</loc>
    <priority>1.0</priority>
  </url>
  <url>
    <loc>https://mathhook.dev/docs/ode-first_order-linear</loc>
    <priority>0.8</priority>
  </url>
  <!-- All other docs pages... -->
</urlset>
```

**How to use**:
1. Visit: `http://localhost:3000/sitemap.xml` (once server is running)
2. Submit to Google Search Console: `https://search.google.com/search-console`
3. Google will crawl all pages automatically

## SEO Best Practices - Implemented

### ✅ Unique Titles
Every page has a descriptive, unique `<title>` tag:
- Good: "Linear First-Order Differential Equations - MathHook KB"
- Bad: "MathHook Documentation" (too generic, same on all pages)

### ✅ Unique Descriptions
Every page has a compelling meta description:
- Good: "Solve linear first-order ODEs using the integrating factor method with MathHook"
- Bad: "Documentation page" (not helpful)

### ✅ Semantic HTML
We use proper HTML5 structure:
```html
<article>
  <header>
    <h1>Main Title</h1>
  </header>
  <section>
    <h2>Subsection</h2>
  </section>
</article>
```

### ✅ Mobile-Friendly
Tailwind CSS ensures responsive design:
- Mobile viewport meta tag
- Responsive breakpoints
- Touch-friendly buttons

### ✅ Fast Loading
- SSR reduces initial load time
- Nuxt auto code-splitting
- Prism.js only loads needed languages

### ✅ Clean URLs
- Good: `/docs/ode-first_order-linear`
- Bad: `/docs?id=123&topic=ode`

## SEO To-Do List (Future Enhancements)

### 🔲 Robots.txt
**File**: `public/robots.txt`
```
User-agent: *
Allow: /
Sitemap: https://mathhook.dev/sitemap.xml
```

### 🔲 Open Graph Images
Auto-generate preview images for social sharing:
- `public/og-images/ode-first_order-linear.png`
- Include formula, title, branding
- Tools: Puppeteer, Playwright, Satori

### 🔲 Breadcrumb Structured Data
Add breadcrumb navigation for Google:
```json
{
  "@type": "BreadcrumbList",
  "itemListElement": [
    {"@type": "ListItem", "position": 1, "name": "Home", "item": "https://mathhook.dev"},
    {"@type": "ListItem", "position": 2, "name": "Docs", "item": "https://mathhook.dev/docs"},
    {"@type": "ListItem", "position": 3, "name": "Linear ODEs"}
  ]
}
```

### 🔲 FAQ Structured Data
For pages with Q&A sections:
```json
{
  "@type": "FAQPage",
  "mainEntity": [
    {
      "@type": "Question",
      "name": "What is a linear first-order ODE?",
      "acceptedAnswer": {
        "@type": "Answer",
        "text": "..."
      }
    }
  ]
}
```

### 🔲 Internal Linking
Add "Related Topics" with actual links:
```vue
<div class="related-topics">
  <h2>Related Topics</h2>
  <NuxtLink to="/docs/ode-first_order-separable">Separable ODEs</NuxtLink>
  <NuxtLink to="/docs/ode-first_order-bernoulli">Bernoulli ODEs</NuxtLink>
</div>
```

### 🔲 Page Speed Optimization
- Lazy load images
- Preload critical fonts
- Minimize JavaScript bundles
- Use Nuxt image optimization

### 🔲 Analytics Integration
Track SEO performance:
```javascript
// Google Analytics 4
export default defineNuxtConfig({
  modules: ['@nuxtjs/google-analytics'],
  googleAnalytics: {
    id: 'G-XXXXXXXXXX'
  }
})
```

## How Google Discovers Your Pages

### 1. **Sitemap Submission** (Recommended)
- Generate sitemap: `/sitemap.xml`
- Submit to Google Search Console
- Google crawls all listed pages

### 2. **Internal Links**
- Homepage → Docs index → Individual pages
- Google follows links automatically
- More internal links = better crawlability

### 3. **External Links** (Backlinks)
- Share on social media
- Link from GitHub README
- Technical blog posts
- Stack Overflow answers

### 4. **Direct Submission**
- Google Search Console: "Request Indexing"
- Bing Webmaster Tools
- Manual submission for important pages

## Testing Your SEO

### View Rendered HTML (What Google Sees)
```bash
curl https://mathhook.dev/docs/ode-first_order-linear
```

Should return complete HTML with all content.

### Google Rich Results Test
https://search.google.com/test/rich-results

Paste URL or HTML to test structured data.

### PageSpeed Insights
https://pagespeed.web.dev/

Tests mobile-friendliness and performance.

### Mobile-Friendly Test
https://search.google.com/test/mobile-friendly

Ensures responsive design.

## Common SEO Myths Debunked

### ❌ "Dynamic routes are bad for SEO"
**FALSE!** With SSR, dynamic routes are perfectly fine. `/docs/[topic]` renders unique HTML for each topic.

### ❌ "JavaScript sites can't rank"
**FALSE!** Google executes JavaScript, but SSR makes it unnecessary. We provide complete HTML immediately.

### ❌ "You need static HTML files"
**FALSE!** Server-generated HTML is just as good as static files. SSR provides flexibility with SEO benefits.

### ❌ "Meta keywords matter"
**FALSE!** Google ignores the `keywords` meta tag (we include it for other search engines that still use it).

## Summary

✅ **Yes, your dynamic `[topic].vue` is SEO-friendly!**

Why it works:
1. **SSR** renders complete HTML for each URL
2. **Unique URLs** for every documentation page
3. **Unique metadata** (title, description, structured data)
4. **Semantic HTML** with proper heading hierarchy
5. **Mobile-friendly** responsive design
6. **Fast loading** with code splitting
7. **Canonical URLs** prevent duplicates
8. **Sitemap** helps Google discover all pages

Your documentation will rank well in Google search results!

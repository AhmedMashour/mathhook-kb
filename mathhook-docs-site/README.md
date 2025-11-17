# MathHook Documentation Website

Modern, interactive documentation website built with Nuxt 3, Vue.js, and Tailwind CSS.

## Features

- **Server-Side Rendering (SSR)**: Fast initial page loads and SEO optimization
- **Modern UI**: Built with Tailwind CSS for a clean, responsive design
- **Interactive Examples**: Code tabs for Rust, Python, and JavaScript
- **Math Rendering**: LaTeX formulas rendered with KaTeX
- **Dark Mode Ready**: Full dark mode support
- **Type-Safe**: Built with TypeScript

## Development

### Prerequisites

- Node.js 18+ or 22+
- npm or pnpm

### Setup

```bash
# Install dependencies
npm install

# Start development server
npm run dev
```

The site will be available at `http://localhost:3000`

### Build for Production

```bash
# Build the application
npm run build

# Preview production build
npm run preview
```

## Project Structure

```
mathhook-docs-site/
├── pages/                  # Route pages
│   ├── index.vue          # Homepage
│   └── docs/
│       ├── index.vue      # Documentation listing
│       └── [topic].vue    # Dynamic document pages
├── components/            # Vue components
├── public/
│   └── data/             # JSON schema files
├── app.vue               # Root layout
└── nuxt.config.ts        # Nuxt configuration
```

## Adding New Documents

1. Generate schema JSON from YAML:
   ```bash
   python3 ../convert_schema.py ../schemas/examples/your-topic.yaml public/data/your-topic.json
   ```

2. Add link to `pages/docs/index.vue`

3. Document will be automatically available at `/docs/your-topic`

## Deployment

This is a standard Nuxt 3 application and can be deployed to:

- **Vercel**: `vercel deploy`
- **Netlify**: Connect git repository
- **Node.js Server**: Use `npm run build` and serve `.output` directory
- **Static Hosting**: Use `nuxt generate` for static site generation

## Technology Stack

- **Nuxt 3**: Vue.js meta-framework with SSR
- **Vue 3**: Progressive JavaScript framework
- **Tailwind CSS**: Utility-first CSS framework
- **TypeScript**: Type safety and better DX
- **KaTeX**: Fast math rendering library

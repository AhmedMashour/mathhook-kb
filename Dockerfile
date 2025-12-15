# Use Node.js 22.13 (latest LTS that meets Nuxt 4 requirements)
FROM node:22.13-slim AS builder

WORKDIR /app

# Copy package files first for better caching
COPY mathhook-docs-site/package*.json ./

# Install dependencies
RUN npm ci

# Copy the rest of the app
COPY mathhook-docs-site/ ./

# Build the Nuxt app
RUN npm run build

# Production stage
FROM node:22.13-slim AS runner

WORKDIR /app

# Copy built output
COPY --from=builder /app/.output ./.output

# Expose port
EXPOSE 3000

# Start the server
CMD ["node", ".output/server/index.mjs"]

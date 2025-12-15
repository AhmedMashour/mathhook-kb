// Server API route for docs index - enables SSR for SEO
// GET /api/docs-index
import { readFileSync } from 'fs'
import { join } from 'path'

export default defineEventHandler((event) => {
  try {
    // Read from public directory
    const filePath = join(process.cwd(), 'public', 'data', '_index.json')
    const data = readFileSync(filePath, 'utf-8')
    return JSON.parse(data)
  } catch (error) {
    console.error('Failed to read docs index:', error)
    throw createError({
      statusCode: 500,
      message: 'Failed to load documentation index'
    })
  }
})

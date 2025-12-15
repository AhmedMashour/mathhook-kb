// Server API route for docs index - enables SSR for SEO
// GET /api/docs-index
import { readFileSync, existsSync } from 'fs'
import { join } from 'path'

// Cache for parsed JSON (in-memory, cleared on server restart)
let indexCache: { data: unknown; timestamp: number } | null = null
const CACHE_TTL_MS = 60 * 1000 // 1 minute in dev, increase for prod

export default defineEventHandler((event) => {
  // Check cache first
  if (indexCache && Date.now() - indexCache.timestamp < CACHE_TTL_MS) {
    setResponseHeaders(event, {
      'Cache-Control': 'public, max-age=60, stale-while-revalidate=300',
      'X-Cache': 'HIT'
    })
    return indexCache.data
  }

  try {
    // Build file path
    const filePath = join(process.cwd(), 'public', 'data', '_index.json')

    // Check file exists
    if (!existsSync(filePath)) {
      console.error('Documentation index not found at:', filePath)
      throw createError({
        statusCode: 503,
        statusMessage: 'Service Unavailable',
        message: 'Documentation index is not available'
      })
    }

    // Read file
    const fileContent = readFileSync(filePath, 'utf-8')
    let data: unknown

    // Parse JSON
    try {
      data = JSON.parse(fileContent)
    } catch (parseError) {
      console.error('Failed to parse documentation index:', parseError)
      throw createError({
        statusCode: 500,
        statusMessage: 'Internal Server Error',
        message: 'Failed to parse documentation index'
      })
    }

    // Validate structure (basic check)
    if (!data || typeof data !== 'object') {
      console.error('Documentation index has invalid structure')
      throw createError({
        statusCode: 500,
        statusMessage: 'Internal Server Error',
        message: 'Documentation index has invalid format'
      })
    }

    // Update cache
    indexCache = { data, timestamp: Date.now() }

    // Set cache headers
    setResponseHeaders(event, {
      'Cache-Control': 'public, max-age=60, stale-while-revalidate=300',
      'X-Cache': 'MISS'
    })

    return data
  } catch (error: unknown) {
    // Re-throw H3 errors (createError)
    if (error && typeof error === 'object' && 'statusCode' in error) {
      throw error
    }

    // Log unexpected errors
    console.error('Unexpected error loading documentation index:', error)

    throw createError({
      statusCode: 500,
      statusMessage: 'Internal Server Error',
      message: 'An unexpected error occurred'
    })
  }
})

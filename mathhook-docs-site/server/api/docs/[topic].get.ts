// Server API route for individual topic data - enables SSR for SEO
// GET /api/docs/:topic
import { readFileSync, existsSync } from 'fs'
import { join, normalize } from 'path'

// Valid topic pattern: alphanumeric, hyphens, dots (no path traversal)
const VALID_TOPIC_PATTERN = /^[a-z0-9][a-z0-9\-\.]*[a-z0-9]$|^[a-z0-9]$/i

// Cache for parsed JSON (in-memory, cleared on server restart)
const cache = new Map<string, { data: unknown; timestamp: number }>()
const CACHE_TTL_MS = 60 * 1000 // 1 minute in dev, increase for prod

function isValidTopic(topic: string): boolean {
  // Length check
  if (topic.length < 1 || topic.length > 100) return false

  // Pattern check
  if (!VALID_TOPIC_PATTERN.test(topic)) return false

  // No path traversal sequences
  if (topic.includes('..') || topic.includes('/') || topic.includes('\\')) return false

  return true
}

export default defineEventHandler((event) => {
  const topic = getRouterParam(event, 'topic')

  // Validate topic parameter exists
  if (!topic) {
    throw createError({
      statusCode: 400,
      statusMessage: 'Bad Request',
      message: 'Topic parameter is required'
    })
  }

  // Validate topic format (security: prevent path traversal)
  if (!isValidTopic(topic)) {
    throw createError({
      statusCode: 400,
      statusMessage: 'Bad Request',
      message: 'Invalid topic format'
    })
  }

  // Check cache first
  const cached = cache.get(topic)
  if (cached && Date.now() - cached.timestamp < CACHE_TTL_MS) {
    // Set cache headers
    setResponseHeaders(event, {
      'Cache-Control': 'public, max-age=60, stale-while-revalidate=300',
      'X-Cache': 'HIT'
    })
    return cached.data
  }

  try {
    // Build safe file path
    const dataDir = join(process.cwd(), 'public', 'data')
    const filePath = normalize(join(dataDir, `${topic}.json`))

    // Security: Ensure resolved path is within data directory
    if (!filePath.startsWith(dataDir)) {
      throw createError({
        statusCode: 400,
        statusMessage: 'Bad Request',
        message: 'Invalid topic path'
      })
    }

    // Check file exists
    if (!existsSync(filePath)) {
      throw createError({
        statusCode: 404,
        statusMessage: 'Not Found',
        message: `Documentation for '${topic}' not found`
      })
    }

    // Read and parse file
    const fileContent = readFileSync(filePath, 'utf-8')
    let data: unknown

    try {
      data = JSON.parse(fileContent)
    } catch (parseError) {
      console.error(`JSON parse error for topic '${topic}':`, parseError)
      throw createError({
        statusCode: 500,
        statusMessage: 'Internal Server Error',
        message: 'Failed to parse topic data'
      })
    }

    // Update cache
    cache.set(topic, { data, timestamp: Date.now() })

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
    console.error(`Unexpected error loading topic '${topic}':`, error)

    throw createError({
      statusCode: 500,
      statusMessage: 'Internal Server Error',
      message: 'An unexpected error occurred'
    })
  }
})

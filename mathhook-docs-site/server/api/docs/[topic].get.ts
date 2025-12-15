// Server API route for individual topic data - enables SSR for SEO
// GET /api/docs/:topic
import { readFileSync, existsSync } from 'fs'
import { join } from 'path'

export default defineEventHandler((event) => {
  const topic = getRouterParam(event, 'topic')

  if (!topic) {
    throw createError({
      statusCode: 400,
      message: 'Topic parameter is required'
    })
  }

  try {
    // Read from public/data directory
    const filePath = join(process.cwd(), 'public', 'data', `${topic}.json`)

    if (!existsSync(filePath)) {
      throw createError({
        statusCode: 404,
        message: `Topic '${topic}' not found`
      })
    }

    const data = readFileSync(filePath, 'utf-8')
    return JSON.parse(data)
  } catch (error: any) {
    // Re-throw createError errors
    if (error.statusCode) {
      throw error
    }

    console.error(`Failed to read topic '${topic}':`, error)
    throw createError({
      statusCode: 500,
      message: `Failed to load topic '${topic}'`
    })
  }
})

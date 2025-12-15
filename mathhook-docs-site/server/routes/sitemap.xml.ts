// Sitemap generator for MathHook documentation
// This helps search engines discover all documentation pages

import { SitemapStream, streamToPromise } from "sitemap";
import { Readable } from "stream";
import fs from "fs";
import path from "path";

export default defineEventHandler(async (event) => {
  const baseUrl = "https://mathook.org";

  // Get all JSON files from public/data directory
  const dataDir = path.join(process.cwd(), "public/data");
  const jsonFiles = fs
    .readdirSync(dataDir)
    .filter((file) => file.endsWith(".json"))
    .map((file) => file.replace(".json", ""));

  // Create sitemap entries
  const links = [
    // Homepage
    {
      url: "/",
      changefreq: "weekly",
      priority: 1.0,
    },
    // Docs index
    {
      url: "/docs",
      changefreq: "weekly",
      priority: 0.9,
    },
    // All documentation pages
    ...jsonFiles.map((topic) => ({
      url: `/docs/${topic}`,
      changefreq: "monthly",
      priority: 0.8,
    })),
  ];

  // Generate sitemap XML
  const stream = new SitemapStream({ hostname: baseUrl });

  const xml = await streamToPromise(Readable.from(links).pipe(stream)).then(
    (data) => data.toString()
  );

  event.node.res.setHeader("Content-Type", "application/xml");
  return xml;
});

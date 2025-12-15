# Internal Documentation Schemas

This directory contains YAML schemas for MathHook internal documentation converted from markdown.

## Generated: 2025-12-15T19:00:00Z

## Schema Files

1. **overview_2025-12-15T1900.yaml** - Single entry point for all investigation findings
   - Source: `internal/overview.md`
   - Topic: MathHook Complete Status

2. **error-handling_2025-12-15T1900.yaml** - Error handling architecture analysis
   - Source: `internal/error-handling.md`
   - Topic: Error Handling & Fallback Architecture
   - Score: 6/10, Risk: MEDIUM-HIGH

3. **educational-plan_2025-12-15T1900.yaml** - Implementation plan for educational integration
   - Source: `internal/educational-plan.md`
   - Topic: Educational Integration Implementation Plan
   - Goal: 12% → 95% coverage in 100-120 hours

4. **investigation_2025-12-15T1900.yaml** - Master consolidated investigation report
   - Source: `internal/investigation.md`
   - Topic: Type Dispatch Architecture + Educational Integration
   - Files Consolidated: 7 documents

5. **type-dispatch-review_2025-12-15T1900.yaml** - Module-by-module review
   - Source: `internal/type-dispatch-review.md`
   - Topic: Comprehensive Type Dispatch & Educational Integration Review
   - Specialized Agents: 6

6. **complexity-findings_2025-12-15T1900.yaml** - Code complexity analysis
   - Source: `internal/COMPLEXITY_FINDINGS_2025-11-28.md`
   - Topic: Code Complexity Findings for Educational Plan

7. **educational-effort-estimation_2025-12-15T1900.yaml** - Effort estimation review
   - Source: `internal/EDUCATIONAL_EFFORT_ESTIMATION_REVIEW_2025-11-28.md`
   - Topic: Educational Integration Plan - Effort Estimation Review

8. **feature-gap-tests_2025-12-15T1900.yaml** - Archived test cases for unimplemented features
   - Source: `internal/feature-gap-tests-2025-12-15T0430.md`
   - Topic: Feature Gap Tests Archive
   - Tests Archived: 46

## Schema Structure

Each YAML schema follows this structure:

```yaml
topic: "internal.{name}"
title: "{Title}"
description: |
  {Brief description}

document_type: "internal"
audience: ["developers", "maintainers"]

article:
  content: |
    {Full markdown content preserved}

key_findings:
  - "{finding 1}"
  - "{finding 2}"

action_items:
  - "{action 1}"
  - "{action 2}"

related_topics:
  - "internal.{related topic}"

metadata:
  schema_version: "1.0"
  source_file: "internal/{filename}.md"
  generated_timestamp: "2025-12-15T19:00:00Z"
  validation_status: "pending"
  document_category: "internal"
```

## Usage

These schemas are designed for the MathHook Knowledge Base Engine to generate:
- Jupyter Notebooks
- mdBook documentation
- Vue SSR site content
- API documentation
- LLM-optimized RAG markdown

## Validation

All schemas follow the internal documentation format and preserve the complete content from source markdown files.

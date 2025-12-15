# Step-by-Step Explanations

Educational explanations transform mathematical operations from black boxes into transparent learning experiences. The step-by-step system generates detailed walkthroughs showing exactly how MathHook arrives at solutions.


---
chunk_id: educational_step_by_step::0
topic: educational.step_by_step
title: Content
priority: high
languages: [rust, python, javascript]
chunk: 1/1
---

## Content

# Step-by-Step Explanations

> 📍 **Navigation:** [Educational API](./api.md) | [Message Registry](./messages.md) | [Operations](../operations/simplification.md)

Educational explanations transform mathematical operations from black boxes into transparent learning experiences. The step-by-step system generates detailed walkthroughs showing exactly how MathHook arrives at solutions.

## What Are Step-by-Step Explanations?

Step-by-step explanations provide detailed walkthroughs of mathematical operations, breaking down complex procedures into digestible steps. Each step includes:

- **Human-readable description** - Natural language explanation
- **Mathematical notation** - LaTeX and symbolic expressions
- **Rule applied** - The mathematical principle used
- **Current state** - Expression at this stage of solving

**Learning Journey:** This is your entry point for understanding MathHook's educational features. Once you master basic explanations, explore [message customization](./messages.md) and [programmatic integration](./api.md).

## Core Architecture

### StepByStepExplanation Structure

The core explanation type contains the complete journey from problem to solution:

```rust
pub struct StepByStepExplanation {
    pub initial_expression: Expression,
    pub final_expression: Expression,
    pub steps: Vec<Step>,
    pub total_steps: usize,
    pub rules_used: Vec<String>,
}
```

**Mathematical Formula for Steps:**

Each transformation follows the pattern:

$$
\text{Expression}_i \xrightarrow{\text{rule}} \text{Expression}_{i+1}
$$

Where the complete journey is:

$$
E_0 \xrightarrow{r_1} E_1 \xrightarrow{r_2} E_2 \xrightarrow{r_3} \cdots \xrightarrow{r_n} E_n
$$

### Step Structure

Each individual step captures one transformation:

```rust
pub struct Step {
    pub title: String,              // Brief step title
    pub description: String,        // Detailed explanation
    pub expression: Expression,     // Result after this step
    pub rule_applied: String,       // Mathematical rule name
    pub latex: Option<String>,      // LaTeX representation
}
```

### EnhancedStep: Dual-Format System

Enhanced steps provide both human and machine-consumable content:

```rust
pub struct EnhancedStep {
    pub step_id: String,
    pub title: String,
    pub human_message: String,      // Student-friendly text
    pub api_data: StepApiData,      // Machine-readable data
    pub message_key: MessageKey,    // Lookup for customization
    pub math_context: MathContext,  // Variables, progress, state
    pub presentation: PresentationHints,
}
```

**Design Philosophy:** Human messages teach students; API data enables external applications (LMS, mobile apps, assessment tools).





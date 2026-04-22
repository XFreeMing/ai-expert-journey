# ADR-001: Code Reviewer Architecture

| Field | Value |
|-------|-------|
| **Status** | Proposed |
| **Date** | 2026-04-22 |
| **Project** | 01-code-reviewer |
| **Author** | @you |

## Context

Need a practical code review tool that combines classic algorithms (AST, edit distance, cyclomatic complexity) with rule-based analysis.

## Decision

TBD

## Consequences

TBD

## Alternatives Considered

| Option | Pros | Cons | Verdict |
|--------|------|------|---------|
| SonarQube | Mature, enterprise-ready | Heavy, hard to customize | Rejected |
| Custom AST parser | Lightweight, domain-specific | More dev work | Selected |

## References

- tree-sitter: https://tree-sitter.github.io/
- pylint architecture

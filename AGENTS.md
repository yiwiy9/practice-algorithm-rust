# AGENTS

## Scope

This file governs work under `src/contest` only.
Do not introduce repo-root documentation or repo-root policy files unless explicitly requested.

## Execution Environment

Assume `cargo`, `cargo compete`, `rustfmt`, and related helper scripts are meant to run inside the devcontainer for this repository.
Do not assume the host machine or an external agent session has the Rust toolchain on `PATH`, even if the workspace files are visible.
If a command is needed for verification, prefer the devcontainer terminal/context and avoid treating missing host-side `cargo` as a project issue.

## Objective

The goal is to improve the user's AtCoder performance, especially on recent ABC `E/F`, not to maximize raw solved-count on old problems.
Prioritize reusable insight, faster self-solve ability, and lower implementation friction.

## Evaluation Policy

Judge current strength mainly by recent recorded ABC contests.
Use the most recent 25 recorded ABC contests as the primary window unless a different window is explicitly requested.
If the user identifies a recent out-of-contest study arena for the current analysis, include it as a temporary supplemental dataset with lower weight than real contest data.
Do not assume any specific directory remains the main out-of-contest arena across time.
If that study arena mixes heterogeneous sources such as ABC, JOI, JOIG, JOISC, and textbooks, do not collapse everything into one numeric score.
Use two views instead:

- contest performance signal: directly comparable contest data
- training capability signal: broader off-contest growth themes, state design, composition skill, and implementation maturity

Only blend numerically on clearly comparable subsets.
Code with substantial documentation comments is treated as a proxy for editorial-assisted AC, not a fully self-solved success.

## Documentation Policy

When work produces durable insight, update the contest-local docs.

- `wiki`: long-lived knowledge, checklists, strategy, snippet guidance
- `knowledge`: dated analyses, findings, and investigation snapshots
- `predictions`: contest prediction playbooks and contest-specific prediction logs
- `adr`: changes in policy, evaluation method, or operating rules
- `plans`: current and near-term training focus

If you modify one document, check the other relevant documents and fix inconsistent references, operating rules, or duplicated policy in the same change.
Do not add documentation that merely restates facts obvious from directory layout or file names.

## Problem Review Policy

When reviewing a problem for growth purposes, capture only the highest-signal items:

- the first wrong viewpoint or fixation
- the key transformation that unlocked the solution
- the main implementation bottleneck
- whether a reusable snippet or checklist item should be created

## Snippet Promotion Policy

Promote code to `src/lib` only when all of the following are true:

- it is useful for at least two problems
- it can be expressed with a natural problem-agnostic API
- it is testable in isolation

If an insight is important but not naturally reusable as code, record it in docs instead of forcing a library abstraction.

## Guardrails

Do not rewrite or reorganize existing AC solutions unless that work is explicitly requested.
Do not move this documentation system to the repository root unless the user explicitly asks for that tradeoff.

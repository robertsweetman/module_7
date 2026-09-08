# Design Evaluation and Architectural Decisions <!-- 1100 words -->

The application was created using an AI agent harness to significantly reduce time to deployment from about 4 months to 8 weeks.

A component of the harness creates Architectural Decision Records (ADR's) as it proceeds through feature delivery.

## Application and architectural analysis

The app is a TypeScript monorepo comprising a Next.js web client frontend and a modular NestJS API for connecting to a Postgresql database.

The browser accesses the backend through an API wrapper and NestJS controllers delegate to domain services for jobs, schemes, drainage, verges, knowledge retrieval and AI-assisted querying.

NestJS dependency injection (DI) composes these services and a global database module provides separate owner and read-only connection pools. 

This is basically a layered, modular-monolith architecture rather than microservices: deployment is simple and cross-feature calls remain local, but the API modules still establish boundaries from which services could later be separated if needed.

```mermaid
flowchart LR
	U[Councillor] --> W[Next.js web application]
	W -->|Bearer token and JSON| P[NestJS request pipeline]
	P --> J[Entra JWT guard]
	J --> D[Division guard]
	D --> C[Controllers]
	C --> S[Domain services]
	S --> O[Azure OpenAI adapter]
	S --> G[Microsoft Graph]
	S --> F[FixMyStreet Open311]
	S --> DB[(PostgreSQL, PostGIS and pgvector)]
	O --> Q[SQL AST guard]
	Q --> R[(Read-only pool)]
```

*Figure 4: Principal components and trust boundaries.*

The primary coupling is to abstractions supplied by NestJS and to locally defined service APIs, not directly between pages and data stores. 

This improves replaceability and permits tests to inject pool and credential doubles. However,`AiQueryService` coordinates prompt construction, retrieval, SQL execution and response formatting, making it a high-coupling orchestration point. 

`FixMyStreetController` similarly contains HTTP integration, cache state, geographic calculation and request handling. These choices worked for the proof of concept creation stage could cause issues later when more features or tests might need to be added.

## Design-pattern evaluation

The three pattern categories are distinguished by the design problem they solve, not merely by pattern names. 

Creational patterns control **how objects are made**; structural patterns control **how objects are composed**; and behavioural patterns control **how objects communicate and allocate responsibility** (Gamma *et al.*, 1994). 

The classification below applies these categories to the app code.

```mermaid
flowchart TB
	X{What design problem is being solved?}
	X -->|How is an object created?| CR[Creational]
	X -->|How are objects composed?| ST[Structural]
	X -->|How do objects collaborate?| BH[Behavioural]
	CR --> CR1[Factory-like pool and OpenAI client construction]
	CR --> CR2[Container-managed provider lifetimes]
	ST --> ST1[Azure OpenAI adapter]
	ST --> ST2[AI-query facade]
	ST --> ST3[FixMyStreet caching proxy-like boundary]
	BH --> BH1[Authentication and authorisation chain]
	BH --> BH2[Response rendering strategy-like dispatch]
	BH --> BH3[SQL AST interpretation and rejection]
```

*Figure 5: Pattern categories distinguished by concern and mapped to the application.*

The database module's `buildPoolConfig()` and the OpenAI service's `buildAzureClient()` both centralise construction in a single function, but neither is a true Factory Method — no subclass overrides the creation step. 

The clearest structural pattern is **Adapter**. `AzureOpenAiService` translates the application's existing `invoke` contract and Anthropic-style tool definitions into Azure OpenAI function calls. Callers therefore remained stable when AWS Bedrock was replaced in refactoring the application to run using Azure Foundry.

`AiQueryService` acts as a **Facade**, presenting controllers with one operation while coordinating LLM, knowledge-base and spatial services. 

## Quality implications

These patterns aid security and testability. Centralised authentication prevents controllers from inconsistently enforcing identity, while DI allows guards and services to be tested using fake dependencies.

The OpenAI adapter localises vendor-specific APIs, and the database factory enforces bounded pools and fail-fast configuration. 
 
Together these support ISO/IEC 25010 characteristics of maintainability, security and reliability (ISO, 2023). 

The main trade-offs are concentration of responsibility and runtime dependency. `AiQueryService` is easier for controllers to consume but has a broad test surface. 

Ward authorisation depends on Microsoft Graph; caching improves performance and availability but permits entitlement revocation to remain stale for up to 15 minutes on each application instance. 

Failing closed on an empty cache favours confidentiality over availability, consistent with zero-trust guidance (Rose *et al.*, 2020). 

## Architectural Decision Record recommendations

### ADR 1: Retain Entra-issued JWT authentication and attribute-based ward authorisation

**Status:** Recommended, reflecting implemented ADR-004 and ADR-013. **Context:** Ward data must not be anonymous or visible across councillor boundaries. Entra does not replace JWT: Entra is the identity provider that issues JWT access tokens, which the global guard validates by issuer, audience and signature. Because Custom Security Attributes are not token claims, the API retrieves ward assignments from Graph using the stable object ID and applies deny-by-default authorisation.

**Decision:** Retain global authentication followed by a separate division guard; use the configured ward attribute as the single access source, cache it for 15 minutes and fail closed when an entitlement cannot be established. **Alternatives:** per-controller checks risk inconsistent enforcement; email identity is mutable; and fail-open caching could disclose data during a Graph outage. **Consequences and tests:** Graph is an availability dependency and revocation is not immediate. Unit tests should verify invalid issuer/audience, absent attributes, cache expiry and 401/403/503 distinctions; integration tests should prove one ward cannot read another. This supports councillor reuse of managed accounts while reducing both administration and data-protection risk.

### ADR 2: Retain defence-in-depth for LLM-generated PostgreSQL queries

**Status:** Recommended, reflecting implemented ADR-009. **Context:** A language model can be manipulated through prompt injection, so generated SQL cannot be trusted merely because the user is authenticated. A previous string-prefix `SELECT` check would not reliably detect stacked or disguised statements.

**Decision:** Parse generated SQL into an abstract syntax tree, permit exactly one `SELECT`, execute it through a separately provisioned read-only pool and preserve row level ward scoping. **Alternatives:** prompt instructions alone provide no enforceable boundary; a parser alone can contain defects; and removing natural-language querying would discard its business value. **Consequences and tests:** the parser and second pool add dependency and configuration cost, but independent controls limit impact when one fails. Parameterised application queries should remain the default outside this exception (OWASP, 2025). Tests should include comments, malformed SQL, multiple statements, DDL/DML, CTEs and cross-division attempts, plus a database integration test proving the read-only role cannot write. This is proportionate because the threatened data and AI-generated execution boundary justify stronger controls than ordinary fixed-query endpoints.

<!--
LO2 | K22, K23 | 1,100 words

CONTENT TO COVER:
- Analysis of the design and architectural characteristics of the chosen system
- Evaluation of design patterns present or absent - creational, structural, behavioural
- Assessment of coupling, structural decisions, and quality implications
- Architectural Decision Record (ADR) documenting at least two recommended improvements with testability implications and business justification

TO REACH B:
- Analyse how design decisions (pattern selection, coupling, structural choices) interact to affect software quality and testability
- Justify recommendations with context-specific evidence
- Demonstrate proportionality - show awareness of when simpler solutions are preferable to complex pattern implementations
- Support evaluation with credible external evidence (peer-reviewed research, white/green papers, standards/frameworks) and explain how it strengthens judgement about proportionality

TO REACH A:
- Critically evaluate the system's design against organisational quality requirements
- Synthesise an ADR demonstrating sophisticated professional judgement: e.g. where a pattern introduces unnecessary complexity, where deferred decisions are strategically sound, or where competing design forces require explicit trade-off reasoning
- Synthesise evidence from multiple credible sources to justify architectural trade-offs
-->

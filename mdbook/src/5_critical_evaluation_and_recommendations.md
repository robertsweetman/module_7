# Critical Evaluation and Recommendations <!-- 600 words -->



## Downsides to the introduction of AI

Non-technical stakeholders can develop an overly optimistic view of delivery timescales after watching a working proof of concept appear in a day. 

The prototype performed without the real-world constraints of cloud deployment, security access, GDPR obligations around data, or even a connection to live data.

This means expectation-management conversations are challenging because they contradict a lived experience of something working in a day.

## What the accelerated process changed

```mermaid
flowchart LR
	subgraph PRE[Pre-AI delivery]
		direction TB
		P1[Discover needs] --> P2[Write requirements and SOW]
		P2 --> P3[Produce architecture and design]
		P3 --> P4[Governance review]
		P4 --> P5[Build application]
		P5 --> P6[Test and gain user acceptance]
		P6 --> P7[Release and support handover]
	end

	subgraph AI[AI-assisted delivery]
		direction TB
		A1[Export data] --> A2[Build local proof of concept]
		A2 --> A3[Live stakeholder feedback]
		A3 --> A4[Write SOW from demonstrated product]
		A4 --> A5[Retrofit cloud, identity and deployment]
		A5 --> A6[AI implementation with developer and QA oversight]
		A6 --> A7[Technical review and go/no-go]
	end

	P2 -. compressed and reordered .-> A4
	P3 -. deferred until prototype exists .-> A5
	P4 -. moved towards release .-> A7
	P6 -. retained with earlier user access .-> A6
```

*Figure 10: Comparison of the pre-AI sequence and the AI-assisted lifecycle.*

| Pre-AI activity | Change in the AI-assisted process | Quality implication |
| --- | --- | --- |
| Requirements elicitation before implementation | **Compressed and reordered:** the prototype preceded the formal SOW | Faster discovery and more tangible feedback, but prototype choices may become requirements without sufficient challenge |
| High- and low-level design before construction | **Deferred:** cloud architecture was addressed after local code existed | Rapid validation of value, but later rework was required for PostgreSQL, Azure AI Foundry and Entra ID |
| Architecture, security and governance review before build | **Initially bypassed, later restored:** technical oversight occurs after development deployment | Decisions use working evidence, but material risks may be discovered after sunk effort |
| Cloud and deployment constraints | **Missed from the initial prototype:** it ran locally against static data | Low-cost experimentation, but the proof of concept did not establish deployability, scalability or live-data behaviour |
| Test design alongside requirements | **Deferred to Stage 3:** AI generated tests under developer and QA oversight | Automation can accelerate coverage, but late test design may preserve ambiguous or untestable requirements |
| End-user acceptance near the end | **Brought forward:** a test group accesses the development instance | Earlier usability evidence reduces the risk of technically correct but unsuitable features |
| Monitoring, support and cost planning | **Deferred to go/no-go:** assessed before wider release | Avoids over-engineering an unproven idea, but late operational findings can delay or prevent release |

*Table 1: Activities compressed, deferred, omitted initially or moved earlier.*

The process shortened discovery-to-demonstration time but did not eliminate the work a supportable service needs; it changed when that work occurred and who carried its risks. 

Continuous access to a tangible product for stakeholder feedback was the strongest feature. 

The principal weakness was that architecture, security, testing and operational questions followed the prototype, creating pressure to retrofit quality into code that stakeholders already perceived as nearly complete.

## AI & Quality trade offs

While AI does increase iteration speed and can increase collaboration (Stage 4) then it _may_ result in a product that only continued use of costly AI usage can maintain. 

Code quality still matters (Yu N, 2026) but to a certain extent, _how the result is achieved_ may become secondary to _whether it works or not_ given that you could throw a large amount of LLM tokens at a problem long enough to achieve a shippable product.

It may not be objectively high quality code but it would achieve a business goal.

## Summary

Software development has moved from 'Big Design up Front' (then months of development) to 'Agile' (two week iterations) and now AI has further reduced the apparent time needed for changes to a day or less.

Changes happen faster at a lower apparent cost but without a parallel increase in testing & security checking this will lead to an increase in bugs precisely because of this higher rate of change. (CodeRabbit, 2026)
 
## Recommendations for the future

If the AI accellerated app development process is going to be re-used we should keep these points in mind.

- Rapid code changes have knock on effects for the developers (stackoverflow, 2026) in terms of stress and psychological safety
- Complex applications don't suddenly become faster to deliver just because AI is being used and this shouldn't result in abdicating responsibility for managing client expectations around delivery dates
- AI use must be accompanied by an _increase_ in testing to compensate for the evidence that developers heavy use of AI doesn't guarantee they have paid attention to changes

<!--
LO3, LO4 | S21, K8, B1 | 600 words

CONTENT TO COVER:
- Critical evaluation of design decisions made - were they the right choices?
- Critical evaluation of the testing strategy - was it effective, what did it miss?
- Assessment of quality improvements achieved and gaps that remain
- Recommendations for next steps and future iterations
- Lessons learnt

NOTE: This section draws on both LO3 and LO4 - it should synthesise the whole project,
not introduce new evidence. Use it to show depth of reflection.

TO REACH B (LO3):
- Connect testing findings explicitly to the quality improvements sought
- Show determination, refinement, and adaptation in your approach

TO REACH B (LO4):
- Show sustained commitment to quality throughout the project lifecycle
- Apply professional standards with clear awareness of their purpose

TO REACH A:
- Produce original, professionally convincing conclusions that connect design decisions, testing outcomes, and quality impact across the project as a whole
- Synthesise evidence from multiple credible sources to justify trade-offs
- Reflect on how your professional practice shaped outcomes and what you would do differently
-->

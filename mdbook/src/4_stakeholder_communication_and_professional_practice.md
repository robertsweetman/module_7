# Stakeholder Communication and Professional Practice <!-- 700 words -->

## How AI changed the software delivery process

As mentioned previously, the application was created through an AI-assisted design and delivery process. Historically, requirements would be gathered and a specification refined between a Product Owner and stakeholders before being handed to a development team. Having stakeholders directly involved in product definition via AI introduces new dynamics, not all of them positive.

### Positive impacts

- Stakeholders are more involved in development, increasing their sense of ownership
- Requirements are communicated to developers through working code rather than documents
- Initial discovery can take as little as a day, rather than the weeks of conceptual discussion previously needed to produce a specification

## Four-stage AI-assisted lifecycle

### Stage 1: Stakeholder co-creation and proof of concept

- The client data team exports a point-in-time CSV dataset for the AI context.
- Leadership works with AI co-creation specialists to build a local proof of concept.
- OpenAI API calls demonstrate conversational interrogation of the data.
- Stakeholders make design decisions and give live feedback as the interface evolves.
- The commercial team writes the Statement of Work (SOW), including functional and
	non-functional requirements, once the proof of concept has established the concept.

> **Human-in-the-loop checkpoint 1:** Leadership and client stakeholders decide which
> prototype behaviour is valuable enough to formalise in the SOW; people, not the AI,
> remain accountable for product intent.

### Stage 2: Engineering the prototype for Azure

- An application platform is established in the client's Azure tenant.
- The prototype code is imported into a repository and deployment automated.
- Engineers adapt code that was not cloud-native: PostgreSQL becomes hosted, OpenAI
	endpoints are replaced by Azure AI Foundry and Entra ID authentication is introduced.
- A development instance is deployed with controlled team access.

> **Human-in-the-loop checkpoint 2:** Engineers decide how the prototype must change to
> meet identity, data, hosting and deployment constraints; security and feasibility
> remain human accountabilities.

### Stage 3: AI-assisted implementation and testing

- The SOW is converted into Azure DevOps user stories and supplied to the AI harness, which implements requirements and produces unit and functional tests.
- Developers and QA supervise the implementation loop; QA is the principal human-in-the-loop for test evidence (see [Testing Strategy and Implementation](3_testing_strategy_and_implementation.md)).
- Product questions return to the client Product Owner rather than being inferred.
- A test group of end users receives early access and supplies usability feedback.
- Testing, security and networking evidence is presented to the technical oversight group.

> **Human-in-the-loop checkpoint 3:** QA assesses generated test evidence, developers
> review implementation quality, the Product Owner resolves product ambiguity and end
> users validate whether the application works in its real context.

Early on it became clear that non-desktop browser testing was missing so this was rolled into the manual test cases due to a lack of time to extend the automation into mobile and tablet screen based tests.

### Stage 4: In-flight Development Collaboration

- Client product owners feature and full application weekly review
- Feature requests or application changes fed directly to the development team to be implemented by AI
- Technical oversight rely on AI generated High Level Design docs for checking and approval

> **Human-in-the-loop checkpoint 4:** Stakeholders become an adjunct to the development
> team and are given a chance to steer the ship as it goes with client technical 
> architects and security team members also able ask questions all leading to increased 
> confidence in the eventual deliverable

This falls firmly into the key agile manifesto principal:

> Working software is the primary measure of progress. (Beck, 2001)

A key piece of feedback here was that in the UI several different types of client jobs appeared with different colour codes. When the data team and product owner saw this they felt it was too confusing and decided to roll all the jobs into one color. 

### Stage 5: Go/no-go and operational handover

- Stakeholders assess security, monitoring, support requirements, ongoing cost, high-
	and low-level design, and testing evidence.
- A human go/no-go decision controls wider enrolment and handover to the support team.

> **Human-in-the-loop checkpoint 5:** Accountable stakeholders accept or reject the
> residual business, technical and operational risk. AI provides delivery artefacts and
> evidence but does not authorise production use.

## Critical evaluation of the communication strategy

### What I would do differently

The primary communication challenge came from non-technical stakeholders experiencing the power of vibe coding the app in the co-creation P.O.C session (Stage 1) and then expecting the same speed of delivery to translate directly into a deployable, cloud based and secure application. 

This lack of realistic understanding of the challenges of production quality, database connected and maintainable software resulted in an extremely un-realistic high pressure deadline being set by the organisation's CTO to the point where the project plan was being shoe-horned into the pre-set deadline.



<!--
LO4 | K8, B1 | 700 words

CONTENT TO COVER:
- Communication of design decisions and quality outcomes to technical and non-technical stakeholders
- Evidence of at least two collaborative practices (e.g. peer review, design critique, test result presentation) with reference to their impact on the project
- Application of professional standards throughout the project
- Impact of the work on the team or organisation

TO REACH B:
- Analyse the effectiveness of stakeholder communication and collaborative practice
- Provide specific examples of how feedback received influenced design or testing decisions
- Demonstrate sustained commitment to quality throughout the project lifecycle
- Support approach with credible external evidence (peer-reviewed research, professional standards/frameworks) and explain how it informed audience adaptation and collaborative practices

TO REACH A:
- Critically evaluate the communication and collaboration strategy as a whole: what worked, what you would do differently, and how your approach reflects the professional standards expected of a senior software engineering practitioner
- Synthesise evidence from multiple credible sources to justify trade-offs in communication and collaboration strategy
-->

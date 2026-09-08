# Testing Strategy and Implementation <!-- 1100 words -->

Using an agentic AI harness to write automated tests, adding checks to deployment pipelines and even running a 'smoke test' post deploy leads to a high confidence that if application changes pass through this pipeline it will function as intended.

## Testing strategy

![Deployment Pipeline Overview](images/app_deploy_pipeline.png)

*Figure 6: App Deployment Pipeline*

Code changes must pass through several stages before deployment: -

1. E2E - phase 1 - Runs OWASP (ZAP) against the app code
2. E2E - phase 2 - checks the OpenAI (Azure Foundry) endpoint
3. Build - creates zips for deployment & runs unit tests
4. Trivy - infra security scan
5. SonarQube quality gate - runs checks on previous tests and decides whether deployment should continue
6. Deploy to dev
7. Grant access to DB from Azure DevOps
8. Smoke test checks the deployed API and Web endpoints are healthy

This pipeline directly exercises the ADR safeguards from Section 2: unit tests target the SQL AST guard with malformed and multi-statement SQL, and the JWT/ward authorisation chain is tested for cache expiry and cross-division access.

Additionally there's a QA person who checks the AI generated test cases for relevance or issues and is responsible for the overall test process.

Importantly they are _still_ carrying out manual test, investigative (exploratory) tests and in this case are spending significant time making sure that non-standard client devices (mobiles, tablets) can render the frontend UI properly.

### Additional deployment pipeline based checks

Deploying the Azure resources (infrastructure) and application using a CI/CD pipeline means that automated checks can be added to mitigate various issues that might not be uncovered through the normal code review process.

Trivy (Trivy, n.d.) is a Terraform code scanning tool used to look for configuration lapses in security. This checks all the infrastructure-as-code (IaC) for vulnerabilities. The various resources that are deployed to host the application may of course work but it could contain holes that a DevOps engineer might miss. 

![Trivy security scan results](images/trivy_security_results.png)

*Figure 7: Trivy IaC security scan results.*

Where Trivy checks the infrastructure, OWASP ZAP (Severns, 2025) probes the running application itself. It acts as a proxy between a browser and the deployed web app, attacking it the way an external user would — spidering pages, fuzzing inputs and flagging common weaknesses such as missing security headers, cross-site scripting and injection points. This matters because an application can pass code review and still ship an exploitable flaw: a missing header or an unvalidated input only becomes visible once the app is actually running and responding to hostile requests.

![OWASP (ZAP) security scan results](images/owasp_scanning_report.png)

*Figure 8: OWASP ZAP dynamic scan report.*

![OWASP code coverage scan](images/code_coverage_scan.png)

*Figure 9: OWASP code coverage scan*

## Limitations in coverage

### Client mobile devices

One area which ended up being extensively covered by manual tests was how the application rendered on a mobile device or tablet. This was because even with the assist of AI to generate test scripts the team writing prompts were not able to deliver a satisfactory solution to check HTML page rendering on mobile or non-desktop formats. QA members ended up checking page rendering on their phone or their own tablet(s). 

One could assume that since the app used a react framework (Next.js) that page rendering on different resolutions should 'just work' but non-technical team members seemed to insist that somehow real devices were used to check this.

With a little more time, this could probably be refined to include Chrome Dev Tools ability to render non-desktop devices (Chrome for Developers, n.d.)

## Testing Strategy Evaluation

The additional pre-deployment test stages (see *Figure 6*), with the exception of Trivy, were entirely new to the organisation so clearly an improvement from the previous Development to QA and back loop.

Manual testing for different device sizes were added after the Product team pointed out that most users would be accessing the application from their phone.

While the original Statement of Work (SOW) contained commitments around page rendering and AI query response time, this wasn't included in the original test strategy. So application performance checking remains a real gap. Engineers did point out the challenge with making performance commitments about a phone based internet connection but it wasn't removed from the SOW.

In summary the pre-existing test approach was such a low bar that any additional application, scanning, smoke or end-to-end testing meant increased confidence in making changes. 

To re-state an earlier point. As AI driven development increases code change frequency, having all these guard rails in place becomes effectively mandatory, especially when developers may not intimately understand all the code since they haven't written it themselves.

<!-- connect design decisions, testing outcomes, and quality impact in an original and professionally convincing way -->

<!--
LO3 | S21 | 1,100 words

CONTENT TO COVER:
- Testing strategy mapped to relevant SDLC stages
- How design decisions from Section 2 shaped the testing approach
- Test suite implementation: automated tests, black-box and white-box techniques, boundary analysis
- CI/CD integration and tooling
- Test results and coverage
- Refinement of approach based on outcomes and evaluation of effectiveness

TO REACH B:
- Analyse testing outcomes and explicitly refine the approach based on results
- Evaluate the effectiveness of techniques used and identify limitations in coverage
- Connect testing findings explicitly to the quality improvements sought
- Support strategy evaluation with credible external evidence (peer-reviewed research white/green papers, standards/frameworks) and explain how it informed technique selection, refinement, and coverage decisions

TO REACH A:
- Critically evaluate the testing strategy as a whole: what it proved, what it missed,and what the results mean for software quality in the organisational context
- Synthesise evidence from multiple credible sources, compare testing approaches, and justify trade-offs
- Connect design decisions, testing outcomes, and quality impact in an original and professionally convincing way
-->

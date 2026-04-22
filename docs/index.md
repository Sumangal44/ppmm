---
# https://vitepress.dev/reference/default-theme-home-page
layout: home

hero:
  name: "PPMM"
  text: "Fast, clean Python project management"
  tagline: "Build, manage, and ship Python workspaces with one focused CLI."
  actions:
    - theme: brand
      text: Get Started
      link: /markdown-examples
    - theme: alt
      text: Runtime API
      link: /api-examples
    - theme: alt
      text: GitHub
      link: https://github.com/Sumangal44/ppmm

features:
  - title: Cross-platform builds
    details: Release targets for macOS, Linux, and Windows from a single workflow.
  - title: Python-first tooling
    details: Designed around Python project realities, dependency resolution, and lockfile workflows.
  - title: Packaging ready
    details: Integrates with release scripts and packaging targets for practical shipping.
---

<section class="home-showcase">
  <div class="home-stat-grid">
    <article class="home-stat-card">
      <p class="home-stat-label">Focus</p>
      <h3>Project lifecycle</h3>
      <p>From local setup to reproducible release artifacts.</p>
    </article>
    <article class="home-stat-card">
      <p class="home-stat-label">Workflow</p>
      <h3>CLI automation</h3>
      <p>Use direct commands and scripts without juggling multiple tools.</p>
    </article>
    <article class="home-stat-card">
      <p class="home-stat-label">Docs</p>
      <h3>Actionable reference</h3>
      <p>Examples and runtime references built for fast onboarding.</p>
    </article>
  </div>

  <div class="home-command-panel">
    <p class="home-command-title">Quick start</p>

```bash
# clone the repository
git clone https://github.com/Sumangal44/ppmm

# run documentation locally
npm install
npm run docs:dev
```

  </div>
</section>


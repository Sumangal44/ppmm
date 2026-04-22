import { defineConfig } from 'vitepress'

// https://vitepress.dev/reference/site-config
export default defineConfig({
  title: 'ppmm docs',
  description: 'Modern documentation for ppmm, the project manager for Python workspaces',
  themeConfig: {
    // https://vitepress.dev/reference/default-theme-config
    nav: [
      { text: 'Home', link: '/' },
      { text: 'Getting Started', link: '/markdown-examples' },
      { text: 'CLI Reference', link: '/api-examples' }
    ],

    sidebar: [
      {
        text: 'Documentation',
        items: [
          { text: 'Getting Started', link: '/markdown-examples' },
          { text: 'CLI Reference', link: '/api-examples' }
        ]
      }
    ],

    socialLinks: [
      { icon: 'github', link: 'https://github.com/Sumangal44/ppmm' }
    ]
  }
})

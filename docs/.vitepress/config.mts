// .vitepress/config.ts
import { withMermaid } from 'vitepress-plugin-mermaid'

export default withMermaid({
  title: "Eadkp Documentation",
  description: "Official documentation for Eadkp.",
  head: [
    ['link', { rel: 'icon', href: '/eadkp/favicon.ico' }] 
  ],
  base: '/eadkp/',
  
  themeConfig: {
    nav: [
      { text: 'Home', link: '/' },
      { text: 'Examples', link: '/markdown-examples' }
    ],

    sidebar: [
      {
        text: 'Examples',
        items: [
          { text: 'Markdown Examples', link: '/markdown-examples' },
          { text: 'Runtime API Examples', link: '/api-examples' }
        ]
      }
    ],

    socialLinks: [
      { icon: 'github', link: 'https://github.com/Oignontom8283/eadkp' }
    ]
  },
  markdown: {
    theme: 'dark-plus',
    lineNumbers: true
  },
  mermaid: {
  }
})
// .vitepress/config.ts
import { withMermaid } from 'vitepress-plugin-mermaid'

export default withMermaid({
  title: "Eadkp Documentation",
  description: "Official documentation for Eadkp.",
  head: [
    ['link', { rel: 'icon', type: 'image/svg+xml', href: '/eadkp/favicon.svg' }] 
  ],
  base: '/eadkp/',
  
  themeConfig: {
    nav: [
      { text: 'Home', link: '/' },
      { text: 'Examples', link: '/markdown-examples' }
    ],

    sidebar: [
      {
        text: 'Guide',
        
        items: [
          { text: 'Getting Started ', link: '/guide/' },
          { text: 'Configuration', link: '/guide/configuration' },
          { text: 'Base of the application', link: '/guide/base' },
          { text: 'Images', link: '/guide/images' },
          { text: 'Filesystem', link: '/guide/filesystem' },
          { text: "Simulator", link: '/guide/simulator' },
          { text: "Commands & Export", link: '/guide/commands' }
        ]
      }
    ],

    socialLinks: [
      { icon: 'github', link: 'https://github.com/Oignontom8283/eadkp/tree/dev/docs' }
    ]
  },
  markdown: {
    theme: 'dark-plus',
    lineNumbers: true
  },
  mermaid: {
  }
})
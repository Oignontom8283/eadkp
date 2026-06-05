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
      { text: 'Links', link: '/documentation/links' }
    ],

    sidebar: [
      {
        text: 'Guide',
        items: [
          { text: 'Getting Started ', link: '/guide/' },
          { text: 'Configuration', link: '/guide/configuration' },
          { text: 'Application Structure', link: '/guide/structure' },
          { text: "Simulator", link: '/guide/simulator' },
          { text: "Commands & Export", link: '/guide/commands' },
          // { text: 'Images', link: '/guide/images' },
          // { text: 'Filesystem', link: '/guide/filesystem' },
        ]
      },
      {
        text: 'Documentation',
        items: [
          { text: 'Doc main', link: '/documentation/' },
          { text: 'Links', link: '/documentation/links' },
        ]
      }
    ],

    socialLinks: [
      { icon: 'github', link: 'https://github.com/Oignontom8283/eadkp/tree/dev/docs' },
      { icon: 'rust', link: 'https://crates.io/crates/eadkp' }
    ]
  },
  markdown: {
    theme: 'dark-plus',
    lineNumbers: true
  },
  mermaid: {
  }
})
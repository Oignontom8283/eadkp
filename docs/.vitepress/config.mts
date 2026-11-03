// .vitepress/config.ts
import { withMermaid } from 'vitepress-plugin-mermaid'

export default withMermaid({
  title: "eadkp Documentation",
  description: "Official documentation for eadkp.",
  head: [
    ['link', { rel: 'icon', type: 'image/svg+xml', href: '/eadkp/favicon.svg' }] 
  ],
  base: '/eadkp/',
  
  themeConfig: {
    nav: [
      { text: 'Home', link: '/' },
      { text: 'Guide', link: '/guide/' },
      { text: 'Référence', link: '/reference/links' },
      { text: 'Outils', link: '/tool/object-picker' }
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
        ]
      },
      {
        text: 'Référence',
        items: [
          { text: 'Liens', link: '/reference/links' },
        ]
      },
      {
        text: 'Formats',
        items: [
          { text: 'Format EIF', link: '/reference/formats/eif' },
          { text: 'EIF1', link: '/reference/formats/eif-v1' },
          { text: 'RGB 565', link: '/reference/formats/rgb565' },
        ]
      },
      {
        text: 'Outils',
        items: [
          { text: 'Object Picker', link: '/tool/object-picker' },
          { text: 'Redimensionner une image', link: '/tool/image-resizing' },
        ]
      },
    ],

    socialLinks: [
      { icon: 'github', link: 'https://github.com/Oignontom8283/eadkp/tree/dev/docs' },
      { icon: 'rust', link: 'https://crates.io/crates/eadkp' }
    ],

    footer: {
      message: 'Documentation licensed under <a href="./LICENSE-DOC">CC BY-SA 4.0</a>.',
      copyright: 'Copyright © 2026-present <a href="https://github.com/Oignontom8283">Oignontom8283</a>.'
    },
  },
  markdown: {
    theme: 'dark-plus',
    lineNumbers: true
  },
  mermaid: {
  }
})
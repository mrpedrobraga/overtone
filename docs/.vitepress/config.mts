import { defineConfig } from 'vitepress'
import { generateSidebar } from 'vitepress-sidebar'

// https://vitepress.dev/reference/site-config



export default defineConfig({
    title: "Overtone Documentation",
    description: "Documentation for 'overtone' and its 'music-std' standards.",
    srcDir: 'src',
    themeConfig: {
        logo: "assets/Toyvox.png",
        search: {
            provider: 'local',
        },
        outline: {
            level: 'deep',
            label: 'On this page'
        },
        nav: [
            {
                text: "Getting Started",
                items: [
                    { text: "As a User", link: "/guides/getting-started" },
                    { text: "As a Developer", link: "/guides/dev" }
                ]
            },
        ],
        socialLinks: [
            { icon: 'github', link: 'https://github.com/mrpedrobraga/overtone' }
        ],
        footer: {
            message: "Made with <3 by mrpedrobraga",
            copyright: "Copyright © 2025 Pedro Braga"
        },
        sidebar: generateSidebar({
            documentRootPath: '/src',
            useTitleFromFileHeading: true,
            capitalizeEachWords: true,
            hyphenToSpace: true,
            useFolderLinkFromIndexFile: true,
            sortMenusOrderNumericallyFromTitle: true,
            removePrefixAfterOrdering: true,
            prefixSeparator: "."
        })
    }
})

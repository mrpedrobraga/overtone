import { defineConfig } from 'vitepress'

// https://vitepress.dev/reference/site-config
export default defineConfig({
    title: "Overtone Documentation",
    description: "Documentation for 'overtone' and its 'music-std' standards.",
    srcDir: 'src',
    themeConfig: {
        logo: "assets/Iris.svg",
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
                    { text: "As a User", link: "/guides/getting_started" },
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
        sidebar: [
            {
                text: "Getting Started",
                link: "/guides/getting_started",
                collapsed: false,
                items: [
                    { text: "For Users", link: "/guides/getting_started" },
                    { text: "For Developers", link: "/guides/dev" }
                ]
            },
            {
                text: 'Reference',
                link: '/reference',
                collapsed: false,
                items: [
                    {
                        text: 'Projects', link: '/reference/project', items: [{ text: "`Overtone.toml`", link: '/reference/project/manifest' }]
                    },
                    {
                        text: 'Arrangements', link: 'reference/arrangements'
                    }
                ]
            },
            {
                text: 'Developing Plugins',
                collapsed: true,
                items: [
                    {
                        text: 'Developing for Overtone',
                        link: '/guides/dev',
                        items: [
                            { text: 'Your First Plugin', link: '/guides/dev/creating_a_plugin' }
                        ]
                    },
                ]
            }
        ]
    }
})

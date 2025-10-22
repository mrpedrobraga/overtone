import { defineConfig } from 'vitepress'

// https://vitepress.dev/reference/site-config

export default defineConfig({
    title: "Overtone",
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
            {
                text: "Docs",
                items: [
                    {
                        text: "Learn",
                        items: [
                            { text: "Projects & Albums", link: "/reference/1. Project" },
                            { text: "Songs & Compositions", link: "/reference/2. Arrangements" },
                            { text: "Effects & Setups", link: "/reference/3. Production Setups" },
                            { text: "Developing with Overtone", link: "/guides/dev" }
                        ]
                    },
                    {
                        text: "API & Standards Reference",
                        items: [
                            { text: "The MUS standard", link: "/reference/6. The MUS Format" },
                            { text: "The Editor API", link: "/reference/api/editor" },
                            { text: "UI Composer", link: "/reference/gui" },
                            { text: "Builtin Plugins", link: "/reference/plugins" },
                        ]
                    }
                ]
            }
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
                text: "Guides",
                base: "/guides",
                link: "/",
                collapsed: false,
                items: [
                    {
                        text: "for Users",
                        link: "/user"
                    },
                    {
                        text: "for Developers",
                        link: "/dev"
                    }
                ]
            },
            {
                text: "Basic Concepts",
                base: "/reference",
                link: "/",
                collapsed: false,
                items: [
                    {
                        text: "Projects",
                        link: "/project"
                    },
                    {
                        text: "Arrangements",
                        link: "/arrangement"
                    },
                    {
                        text: "Productions",
                        link: "/productions"
                    }
                ]
            },
            {
                text: "Reference",
                base: "/reference",
                link: "/",
                collapsed: false,
                items: [
                    {
                        text: "The MUS Standard",
                        base: "/reference/the-mus-standard",
                        link: "/",
                        items: [
                            {
                                text: "MUSX (Music Excerpts)",
                                link: "/musx"
                            },
                            {
                                text: "MUSI (Music Instruments)",
                                link: "/musi"
                            },
                            {
                                text: "Overtone Standard Notation",
                                link: "/standard-markers"
                            },
                        ]
                    }
                ]
            }
        ]
    }
})
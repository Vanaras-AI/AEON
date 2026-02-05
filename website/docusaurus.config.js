// @ts-check
import {themes as prismThemes} from 'prism-react-renderer';

/** @type {import('@docusaurus/types').Config} */
const config = {
  title: 'AEON',
  tagline: 'Governance-First AI Runtime',
  favicon: 'img/favicon.ico',

  future: {
    v4: true,
  },

  url: 'https://vanaras-ai.github.io',
  baseUrl: '/AEON/',

  organizationName: 'Vanaras-AI',
  projectName: 'AEON',

  onBrokenLinks: 'throw',

  i18n: {
    defaultLocale: 'en',
    locales: ['en'],
  },

  presets: [
    [
      'classic',
      /** @type {import('@docusaurus/preset-classic').Options} */
      ({
        docs: {
          sidebarPath: './sidebars.js',
          editUrl: 'https://github.com/Vanaras-AI/AEON/tree/main/website/',
        },
        blog: false, // Disable blog for now
        theme: {
          customCss: './src/css/custom.css',
        },
      }),
    ],
  ],

  themeConfig:
    /** @type {import('@docusaurus/preset-classic').ThemeConfig} */
    ({
      image: 'img/aeon-social-card.png',
      colorMode: {
        defaultMode: 'dark',
        respectPrefersColorScheme: true,
      },
      navbar: {
        title: 'AEON',
        logo: {
          alt: 'AEON Logo',
          src: 'img/logo.svg',
        },
        items: [
          {
            type: 'docSidebar',
            sidebarId: 'docs',
            position: 'left',
            label: 'Documentation',
          },
          {
            to: '/docs/a2g-protocol/overview',
            label: 'A2G Protocol',
            position: 'left',
          },
          {
            to: '/docs/api-reference',
            label: 'API',
            position: 'left',
          },
          {
            href: 'https://github.com/Vanaras-AI/AEON',
            label: 'GitHub',
            position: 'right',
          },
        ],
      },
      footer: {
        style: 'dark',
        links: [
          {
            title: 'Documentation',
            items: [
              {
                label: 'Getting Started',
                to: '/docs/intro',
              },
              {
                label: 'A2G Protocol',
                to: '/docs/a2g-protocol/overview',
              },
              {
                label: 'API Reference',
                to: '/docs/api-reference',
              },
            ],
          },
          {
            title: 'SDKs',
            items: [
              {
                label: 'Rust SDK',
                to: '/docs/sdk/rust',
              },
              {
                label: 'Python SDK',
                to: '/docs/sdk/python',
              },
            ],
          },
          {
            title: 'More',
            items: [
              {
                label: 'GitHub',
                href: 'https://github.com/Vanaras-AI/AEON',
              },
              {
                label: 'Security',
                to: '/docs/security',
              },
            ],
          },
        ],
        copyright: `Copyright © ${new Date().getFullYear()} Vanaras AI. Built with Docusaurus.`,
      },
      prism: {
        theme: prismThemes.github,
        darkTheme: prismThemes.dracula,
        additionalLanguages: ['rust', 'python', 'bash', 'json', 'toml'],
      },
    }),
};

export default config;

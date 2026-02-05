/** @type {import('@docusaurus/plugin-content-docs').SidebarsConfig} */
const sidebars = {
  docs: [
    'intro',
    {
      type: 'category',
      label: 'Getting Started',
      items: [
        'getting-started/installation',
        'getting-started/quickstart',
        'getting-started/configuration',
      ],
    },
    {
      type: 'category',
      label: 'A2G Protocol',
      items: [
        'a2g-protocol/overview',
        'a2g-protocol/message-types',
        'a2g-protocol/governance-flow',
        'a2g-protocol/risk-scoring',
      ],
    },
    {
      type: 'category',
      label: 'SDKs',
      items: [
        'sdk/rust',
        'sdk/python',
      ],
    },
    'api-reference',
    'security',
  ],
};

export default sidebars;

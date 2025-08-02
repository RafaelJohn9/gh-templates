import type { SidebarsConfig } from '@docusaurus/plugin-content-docs';

const sidebars: SidebarsConfig = {
  tutorialSidebar: [
    {
      type: 'doc',
      id: 'intro',
      label: 'Introduction',
    },
    {
      type: 'doc',
      id: 'installation',
      label: 'Installation',
    },
    {
      type: 'doc',
      id: 'usage',
      label: 'Usage Guide',
    },
    {
      type: 'category',
      label: 'Commands',
      items: [
        {
          type: 'category',
          label: 'Gitignore',
          items: [
            'commands/gitignore/gitignore',
            'commands/gitignore/gitignore-add',
            'commands/gitignore/gitignore-list',
            'commands/gitignore/gitignore-preview',
          ],
        },
        {
          type: 'category',
          label: 'Issue Templates',
          items: [
            'commands/issue/issue',
            'commands/issue/issue-add',
            'commands/issue/issue-list',
            'commands/issue/issue-preview',
          ],
        },
        {
          type: 'category',
          label: 'PR Templates',
          items: [
            'commands/pr/pr',
            'commands/pr/pr-add',
            'commands/pr/pr-list',
            'commands/pr/pr-preview',
          ],
        },
        {
          type: 'category',
          label: 'License',
          items: [
            'commands/license/license',
            'commands/license/license-add',
            'commands/license/license-list',
            'commands/license/license-preview',
          ],
        },
        {
          type: 'doc',
          id: 'commands/overview',
          label: 'All Commands Overview',
        },
      ],
    },
  ],
};

export default sidebars;
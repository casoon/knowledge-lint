// @ts-check
import casoonPages from '@casoon/pages-theme';
import { defineConfig } from 'astro/config';

// Project page: https://casoon.github.io/knowledge-lint/ — `base` is the GitHub Pages path.
export default defineConfig({
  site: 'https://casoon.github.io/knowledge-lint',
  base: '/knowledge-lint/',
  integrations: [
    casoonPages({
      name: 'knowledge-lint',
      description:
        'Validates a Markdown + YAML-frontmatter knowledge base against a declarative _types.yml config.',
      repo: 'casoon/knowledge-lint',
      version: '0.1.0',
      license: 'MIT',
      branch: 'master',
      packages: [{ label: 'crates.io', href: 'https://crates.io/crates/knowledge-lint' }],
      docsGroups: {
        'getting-started': 'Getting started',
        guides: 'Guides',
        reference: 'Reference',
      },
    }),
  ],
});

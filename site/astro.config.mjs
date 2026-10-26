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
      tagline: 'Validate Markdown knowledge bases',
      repo: 'casoon/knowledge-lint',
      license: 'MIT',
      branch: 'master',
      packages: [{ label: 'crates.io', href: 'https://crates.io/crates/knowledge-lint' }],
      mdxComponents: './src/mdx-components.ts',
      docsGroups: {
        'getting-started': 'Getting started',
        guides: 'Guides',
        reference: 'Reference',
      },
    }),
  ],
});

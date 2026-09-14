import { ansiToHtml } from '@casoon/pages-theme/ansi';
import type { ShowcaseExample } from '@casoon/pages-theme/showcase';

// Output captured by examples/capture.sh from the release binary, run against the
// sample knowledge bases in examples/.
const outputs = import.meta.glob<string>('../../examples/output/*.txt', {
  query: '?raw',
  import: 'default',
  eager: true,
});
const configs = import.meta.glob<string>('../../examples/*/_types.yml', {
  query: '?raw',
  import: 'default',
  eager: true,
});

const examples_ = [
  {
    slug: 'passing-knowledge-base',
    title: 'Passing knowledge base',
    output: 'lint-knowledge-base.txt',
    input: { code: configs['../../examples/knowledge-base/_types.yml'] ?? '', lang: 'yaml' },
    tags: ['lint', 'warning', 'sops'],
    description:
      'knowledge-lint lint knowledge-base: three entry categories, a SOPS-encrypted secret and an asset folder. One glossary entry is past its 365-day review interval, which is a warning, so the run exits 0. The input shows the _types.yml.',
  },
  {
    slug: 'broken-knowledge-base',
    title: 'Broken knowledge base',
    output: 'lint-broken-knowledge-base.txt',
    input: {
      code: configs['../../examples/broken-knowledge-base/_types.yml'] ?? '',
      lang: 'yaml',
    },
    tags: ['lint', 'errors', 'status vocabulary'],
    description:
      'knowledge-lint lint broken-knowledge-base: an undeclared folder, a wrong type, a status outside the custom vocabulary, an invalid date, a broken related link, an unencrypted secret and a missing attachment. Errors make the run exit 1. The input shows the _types.yml.',
  },
  {
    slug: 'clean-demo-content',
    title: 'Strip demo content',
    output: 'clean-knowledge-base.txt',
    input: { code: 'knowledge-lint clean knowledge-base', lang: 'shell' },
    tags: ['clean'],
    description:
      'Run on a copy of the passing knowledge base: every entry is removed, the category folders stay and _attachments.yml is reset to an empty list.',
  },
];

export const examples: ShowcaseExample[] = examples_.map(({ output, ...meta }) => ({
  ...meta,
  file: `examples/output/${output}`,
  output: { html: ansiToHtml(outputs[`../../examples/output/${output}`] ?? ''), kind: 'terminal' },
}));

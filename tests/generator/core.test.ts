import { formatRust } from '../../tools/generator/src/emit/core.ts';

describe('generator prerequisites', () => {
  it('fails generation when rustfmt is unavailable', () => {
    const path = process.env.PATH;
    try {
      process.env.PATH = '';
      expect(() => formatRust('pub fn generated() {}\n', 'fixture.rs')).toThrow(
        'Required generator tool rustfmt was not found in PATH.',
      );
    } finally {
      if (path === undefined) delete process.env.PATH;
      else process.env.PATH = path;
    }
  });
});

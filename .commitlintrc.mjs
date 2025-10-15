export default {
    extends: ['@commitlint/config-conventional'],
    rules: {
        'body-max-line-length': [1, 'always', 72],
        'header-max-length': [2, 'always', 52],
        'scope-enum': [2, 'always', []],
        'type-enum': [2, 'always', [
            'build',
            'change',
            'chore',
            'ci',
            'deprecate',
            'docs',
            'feat',
            'fix',
            'perf',
            'refactor',
            'remove',
            'revert',
            'security',
            'spike',
            'style',
            'test'
        ]]
    }
};
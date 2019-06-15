'use strict';

module.exports = {
  extends: ['airbnb-base', 'prettier', 'plugin:prettier/recommended'],
  plugins: ['prettier', 'react'],
  parserOptions: {
    // sourceType: "module" is only for ES6 modules (export, etc). Service and component
    // config modules are Node.js modules (module.exports, etc) and we ant to keep the
    // strict-mode directives in each of these components
    sourceType: 'script',
  },
  env: {
    node: true,
  },
  rules: {
    'prettier/prettier': ['error', { singleQuote: true, trailingComma: 'all', printWidth: 132 }],
  },
};

/* Generate by @shikijs/codegen */
import type {
  DynamicImportLanguageRegistration,
  DynamicImportThemeRegistration,
  HighlighterGeneric,
} from '@shikijs/types'
import {
  createSingletonShorthands,
  createdBundledHighlighter,
} from '@shikijs/core'
import { createJavaScriptRegexEngine } from '@shikijs/engine-javascript'

type BundledLanguage =
  | 'toml'
  | 'shellscript'
  | 'bash'
  | 'sh'
  | 'shell'
  | 'zsh'
  | 'shellscript'
  | 'bash'
  | 'sh'
  | 'shell'
  | 'zsh'
  | 'javascript'
  | 'js'
  | 'jsx'
  | 'tsx'
  | 'javascript'
  | 'js'
  | 'typescript'
  | 'ts'
  | 'html'
  | 'docker'
  | 'dockerfile'
  | 'json'
  | 'http'
  | 'yaml'
  | 'yml'
  | 'markdown'
  | 'md'
type BundledTheme = 'catppuccin-frappe'
type Highlighter = HighlighterGeneric<BundledLanguage, BundledTheme>

const bundledLanguages = {
  toml: () => import('@shikijs/langs/toml'),
  shellscript: () => import('@shikijs/langs/shellscript'),
  bash: () => import('@shikijs/langs/shellscript'),
  sh: () => import('@shikijs/langs/shellscript'),
  shell: () => import('@shikijs/langs/shellscript'),
  zsh: () => import('@shikijs/langs/shellscript'),
  shellscript: () => import('@shikijs/langs/shellscript'),
  bash: () => import('@shikijs/langs/shellscript'),
  sh: () => import('@shikijs/langs/shellscript'),
  shell: () => import('@shikijs/langs/shellscript'),
  zsh: () => import('@shikijs/langs/shellscript'),
  javascript: () => import('@shikijs/langs/javascript'),
  js: () => import('@shikijs/langs/javascript'),
  jsx: () => import('@shikijs/langs/jsx'),
  tsx: () => import('@shikijs/langs/tsx'),
  javascript: () => import('@shikijs/langs/javascript'),
  js: () => import('@shikijs/langs/javascript'),
  typescript: () => import('@shikijs/langs/typescript'),
  ts: () => import('@shikijs/langs/typescript'),
  html: () => import('@shikijs/langs/html'),
  docker: () => import('@shikijs/langs/docker'),
  dockerfile: () => import('@shikijs/langs/docker'),
  json: () => import('@shikijs/langs/json'),
  http: () => import('@shikijs/langs/http'),
  yaml: () => import('@shikijs/langs/yaml'),
  yml: () => import('@shikijs/langs/yaml'),
  markdown: () => import('@shikijs/langs/markdown'),
  md: () => import('@shikijs/langs/markdown'),
} as Record<BundledLanguage, DynamicImportLanguageRegistration>

const bundledThemes = {
  'catppuccin-frappe': () => import('@shikijs/themes/catppuccin-frappe'),
} as Record<BundledTheme, DynamicImportThemeRegistration>

const createHighlighter = /* @__PURE__ */ createdBundledHighlighter<
  BundledLanguage,
  BundledTheme
>({
  langs: bundledLanguages,
  themes: bundledThemes,
  engine: () => createJavaScriptRegexEngine(),
})

const {
  codeToHtml,
  codeToHast,
  codeToTokensBase,
  codeToTokens,
  codeToTokensWithThemes,
  getSingletonHighlighter,
  getLastGrammarState,
} = /* @__PURE__ */ createSingletonShorthands<BundledLanguage, BundledTheme>(
  createHighlighter,
)

export {
  bundledLanguages,
  bundledThemes,
  codeToHast,
  codeToHtml,
  codeToTokens,
  codeToTokensBase,
  codeToTokensWithThemes,
  createHighlighter,
  getLastGrammarState,
  getSingletonHighlighter,
}
export type { BundledLanguage, BundledTheme, Highlighter }

package edu.shch.mochi.plugin

import com.intellij.lang.Language

/*
 * Language Docs:
 * https://plugins.jetbrains.com/docs/intellij/language-and-filetype.html#define-the-language
 * https://plugins.jetbrains.com/docs/intellij/grammar-and-parser.html#generate-a-parser
 * https://plugins.jetbrains.com/docs/intellij/psi-helper-and-utilities.html#define-helper-methods-for-generated-psi-elements
 * https://plugins.jetbrains.com/docs/intellij/line-marker-provider.html
 * GitHub Example for a Simple Language:
 * https://github.com/JetBrains/intellij-sdk-code-samples/tree/main/simple_language_plugin/src/main/java/org/intellij/sdk/language
 */
object MochiLanguage: Language("Mochi") {
  private fun readResolve(): Any = MochiLanguage
}
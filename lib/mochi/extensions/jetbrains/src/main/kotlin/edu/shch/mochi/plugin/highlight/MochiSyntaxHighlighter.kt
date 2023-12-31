package edu.shch.mochi.plugin.highlight

import com.intellij.openapi.editor.DefaultLanguageHighlighterColors
import com.intellij.openapi.editor.colors.TextAttributesKey
import com.intellij.openapi.fileTypes.SyntaxHighlighterBase
import com.intellij.openapi.options.colors.AttributesDescriptor
import com.intellij.psi.TokenType
import com.intellij.psi.tree.IElementType
import edu.shch.mochi.plugin.MochiLexerAdapter
import edu.shch.mochi.plugin.psi.MochiTypes

object MochiSyntaxHighlighter: SyntaxHighlighterBase() {
  override fun getHighlightingLexer() = MochiLexerAdapter

  override fun getTokenHighlights(tokenType: IElementType): Array<TextAttributesKey> {
    return when (tokenType) {
      MochiTypes.COMMENT -> arrayOf(Colors.COMMENT())
      MochiTypes.STRING -> arrayOf(Colors.STRING())
      MochiTypes.INTEGER -> arrayOf(Colors.NUMBER())
      MochiTypes.ENVIRONMENT_TYPE -> arrayOf(Colors.ENVIRONMENT_TYPE())
      MochiTypes.ENUM_VALUE -> arrayOf(Colors.CONSTANT())
      MochiTypes.IDENTIFIER -> arrayOf(Colors.IDENTIFIER())
      MochiTypes.ASSIGNMENT_OP -> arrayOf(Colors.OPERATOR())
      MochiTypes.ARRAY_SEP -> arrayOf(Colors.SEPARATOR())
      TokenType.BAD_CHARACTER -> arrayOf(Colors.BAD_CHARACTER())
      else -> emptyArray()
    }
  }

  enum class Colors(private var id: String, private var descriptor: String, private var colors: TextAttributesKey) {
    COMMENT("MOCHI_COMMENT", "Comment", DefaultLanguageHighlighterColors.LINE_COMMENT),
    STRING("MOCHI_STRING", "String", DefaultLanguageHighlighterColors.STRING),
    NUMBER("MOCHI_NUMBER", "Number", DefaultLanguageHighlighterColors.NUMBER),
    ENVIRONMENT_TYPE("MOCHI_ENVIRONMENT_TYPE", "Environment Type", DefaultLanguageHighlighterColors.CONSTANT),
    IDENTIFIER("MOCHI_IDENTIFIER", "Identifier", DefaultLanguageHighlighterColors.LABEL),
    CONSTANT("MOCHI_CONSTANT", "Constant", DefaultLanguageHighlighterColors.CLASS_REFERENCE),
    OPERATOR("MOCHI_OPERATOR", "Operator", DefaultLanguageHighlighterColors.OPERATION_SIGN),
    SEPARATOR("MOCHI_SEPARATOR", "Separator", DefaultLanguageHighlighterColors.OPERATION_SIGN),
    BAD_CHARACTER("MOCHI_BAD_CHARACTER", "Bad Value", DefaultLanguageHighlighterColors.INVALID_STRING_ESCAPE);
    fun asAttributesDescriptor() = AttributesDescriptor(this.descriptor, this.colors)
    operator fun invoke() = TextAttributesKey.createTextAttributesKey(this.id, this.colors)
  }
}
package edu.shch.mochi.plugin

import com.intellij.lang.ASTNode
import com.intellij.lang.ParserDefinition
import com.intellij.openapi.project.Project
import com.intellij.psi.FileViewProvider
import com.intellij.psi.PsiElement
import com.intellij.psi.tree.IFileElementType
import com.intellij.psi.tree.TokenSet
import edu.shch.mochi.plugin.parser.MochiParser
import edu.shch.mochi.plugin.psi.MochiFile
import edu.shch.mochi.plugin.psi.MochiTokenSets
import edu.shch.mochi.plugin.psi.MochiTypes

class MochiParserDefinition: ParserDefinition {
  private val fileNodeType = IFileElementType(MochiLanguage)
  override fun createLexer(project: Project) = MochiLexerAdapter
  override fun createParser(project: Project) = MochiParser()
  override fun getFileNodeType() = fileNodeType
  override fun getCommentTokens() = MochiTokenSets.COMMENTS
  override fun getStringLiteralElements(): TokenSet = MochiTokenSets.STRINGS
  override fun getWhitespaceTokens(): TokenSet = TokenSet.WHITE_SPACE
  override fun createElement(node: ASTNode): PsiElement = MochiTypes.Factory.createElement(node)
  override fun createFile(viewProvider: FileViewProvider) = MochiFile(viewProvider)
}
package edu.shch.mochi.plugin.highlight

import com.intellij.openapi.fileTypes.SyntaxHighlighterFactory
import com.intellij.openapi.project.Project
import com.intellij.openapi.vfs.VirtualFile

class MochiSyntaxHighlightFactory: SyntaxHighlighterFactory() {
  override fun getSyntaxHighlighter(p0: Project?, p1: VirtualFile?) = MochiSyntaxHighlighter
}
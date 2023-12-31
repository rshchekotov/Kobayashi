package edu.shch.mochi.plugin

import com.intellij.lexer.FlexAdapter

object MochiLexerAdapter: FlexAdapter(MochiLexer(null))
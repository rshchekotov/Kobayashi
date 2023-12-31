package edu.shch.mochi.plugin.psi

import com.intellij.psi.tree.TokenSet

object MochiTokenSets {
  val COMMENTS = TokenSet.create(MochiTypes.COMMENT)
  val STRINGS = TokenSet.create(MochiTypes.STRING)
}
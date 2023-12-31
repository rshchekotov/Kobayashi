package edu.shch.mochi.plugin.psi

import com.intellij.psi.tree.IElementType
import edu.shch.mochi.plugin.MochiLanguage

class MochiTokenType(debugName: String): IElementType(debugName, MochiLanguage) {
  override fun toString(): String {
    return "MochiTokenType.${super.toString()}"
  }
}
package edu.shch.mochi.plugin.psi

import com.intellij.psi.tree.IElementType
import edu.shch.mochi.plugin.MochiLanguage

class MochiElementType(debugName: String): IElementType(debugName, MochiLanguage)
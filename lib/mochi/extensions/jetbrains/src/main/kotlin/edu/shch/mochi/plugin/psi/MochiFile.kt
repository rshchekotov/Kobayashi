package edu.shch.mochi.plugin.psi

import com.intellij.extapi.psi.PsiFileBase
import com.intellij.psi.FileViewProvider
import edu.shch.mochi.plugin.MochiFileType
import edu.shch.mochi.plugin.MochiLanguage

class MochiFile(viewProvider: FileViewProvider): PsiFileBase(viewProvider, MochiLanguage) {
  override fun getFileType() = MochiFileType.INSTANCE
  override fun toString() = "Mochi File"
}
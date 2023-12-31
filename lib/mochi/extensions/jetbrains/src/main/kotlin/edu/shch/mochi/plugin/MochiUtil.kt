package edu.shch.mochi.plugin

import com.intellij.openapi.project.Project
import com.intellij.psi.PsiManager
import com.intellij.psi.search.FileTypeIndex
import com.intellij.psi.search.GlobalSearchScope
import com.intellij.psi.util.PsiTreeUtil
import edu.shch.mochi.plugin.psi.MochiFile
import edu.shch.mochi.plugin.psi.MochiProperty

/* These utilities will be used later, as part of auto-complete and whatnot. */
@Suppress("unused")
object MochiUtil {
  fun findProperties(project: Project, key: String): List<MochiProperty> {
    val result: MutableList<MochiProperty> = mutableListOf()

    val virtualFiles = FileTypeIndex.getFiles(MochiFileType.INSTANCE, GlobalSearchScope.allScope(project))
    for (virtualFile in virtualFiles) {
      val mochiFile = PsiManager.getInstance(project).findFile(virtualFile) as MochiFile
      val properties = PsiTreeUtil.getChildrenOfTypeAsList(mochiFile, MochiProperty::class.java)
      for (property in properties) {
        if (key == property.getKey()) {
          result.add(property)
        }
      }
    }
    return result
  }

  fun findProperties(project: Project) {
    val result: MutableList<MochiProperty> = mutableListOf()

    val virtualFiles = FileTypeIndex.getFiles(MochiFileType.INSTANCE, GlobalSearchScope.allScope(project))
    for (virtualFile in virtualFiles) {
      val mochiFile = PsiManager.getInstance(project).findFile(virtualFile) as MochiFile
      val properties = PsiTreeUtil.getChildrenOfTypeAsList(mochiFile, MochiProperty::class.java)
      result.addAll(properties)
    }
  }
}
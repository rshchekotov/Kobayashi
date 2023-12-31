package edu.shch.mochi.plugin.highlight

import com.intellij.lang.annotation.AnnotationHolder
import com.intellij.lang.annotation.Annotator
import com.intellij.lang.annotation.HighlightSeverity
import com.intellij.psi.PsiElement
import edu.shch.mochi.plugin.MochiLanguage
import edu.shch.mochi.plugin.psi.MochiEnvironmentType

class MochiAnnotator: Annotator {
  override fun annotate(element: PsiElement, holder: AnnotationHolder) {
    /* For now, only annotate Mochi within Mochi */
    if (element.language != MochiLanguage) return

    if (element is MochiEnvironmentType) {
      /* Patch Highlights using Annotations */
      holder.newSilentAnnotation(HighlightSeverity.INFORMATION)
        .range(element.textRange).textAttributes(MochiSyntaxHighlighter.Colors.ENVIRONMENT_TYPE()).create()
    }
  }
}
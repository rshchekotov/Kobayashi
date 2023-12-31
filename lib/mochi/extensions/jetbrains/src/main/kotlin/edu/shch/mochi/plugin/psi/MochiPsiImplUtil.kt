package edu.shch.mochi.plugin.psi

class MochiPsiImplUtil {
  companion object {
    @JvmStatic
    fun getKey(element: MochiProperty): String = element.firstChild.node.text
  }
}
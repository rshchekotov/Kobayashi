package edu.shch.mochi.plugin

import com.intellij.openapi.fileTypes.LanguageFileType

class MochiFileType: LanguageFileType(MochiLanguage) {
  override fun getName() = MochiLanguage.displayName
  override fun getDescription() = "Mochi file"
  override fun getDefaultExtension() = "mochi"
  override fun getIcon() = MochiIcons.FILE

  @Suppress("CompanionObjectInExtension")
  companion object {
    @Suppress("unused")
    val INSTANCE = MochiFileType()
  }
}
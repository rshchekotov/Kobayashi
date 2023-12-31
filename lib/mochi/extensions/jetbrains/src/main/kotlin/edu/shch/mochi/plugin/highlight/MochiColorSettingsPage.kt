package edu.shch.mochi.plugin.highlight

import com.intellij.openapi.fileTypes.SyntaxHighlighter
import com.intellij.openapi.options.colors.ColorDescriptor
import com.intellij.openapi.options.colors.ColorSettingsPage
import edu.shch.mochi.plugin.MochiIcons

class MochiColorSettingsPage: ColorSettingsPage {
  override fun getAttributeDescriptors() = MochiSyntaxHighlighter.Colors.values()
    .map { it.asAttributesDescriptor() }.toTypedArray()

  override fun getColorDescriptors(): Array<ColorDescriptor> = ColorDescriptor.EMPTY_ARRAY

  override fun getDisplayName() = "Mochi"

  override fun getIcon() = MochiIcons.FILE

  override fun getHighlighter(): SyntaxHighlighter = MochiSyntaxHighlighter

  override fun getDemoText() = """
    // This describes how the default Discord Server after creation looks like.

    category "Text Channels" {
        channel "general" {
            type -> Text
        }
    }

    category "Voice Channels" {
        channel "General" {
            type -> Voice
        }
    }
  """.trimIndent()

  override fun getAdditionalHighlightingTagToDescriptorMap() = null
}
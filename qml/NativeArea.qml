import QtQuick
import QtQuick.Controls
import qs.Commons as Commons
import qs.Ui as Ui
import "Theme.js" as Theme
import "Typography.js" as Typography

TextArea {
    id: root
    implicitWidth: Commons.Style.spacing.dropdownWidth
    Keys.priority: Keys.BeforeItem
    activeFocusOnTab: true
    function submits(event) {
        return (event.key === Qt.Key_Return || event.key === Qt.Key_Enter)
            && !(event.modifiers & Qt.ShiftModifier) && !inputMethodComposing
    }
    textFormat: TextEdit.PlainText
    color: Commons.Color.popups.text
    font.family: Typography.family
    font.pixelSize: Commons.Style.font.body
    wrapMode: TextEdit.Wrap
    selectByMouse: true
    selectionColor: Commons.Style.selectionFillFor(Commons.Color.popups.text, Commons.Color.accent)
    selectedTextColor: Commons.Color.popups.text
    placeholderTextColor: Theme.secondary(Commons.Color.muted, Commons.Color.popups.text,
        Theme.over(Commons.Style.controlFill(root.activeFocus, root.hovered, Commons.Color.popups.text, Commons.Color.accent), Commons.Color.popups.background))
    padding: Commons.Style.spacing.controlPaddingX + Commons.Style.normalBorderWidth
    background: Ui.BorderSurface {
        color: Commons.Style.controlFill(root.activeFocus, root.hovered, Commons.Color.popups.text, Commons.Color.accent)
        radius: Commons.Style.cornerRadius
        borderSpec: Commons.Border.controlSpec(root.activeFocus ? "focus" : (root.hovered ? "hover-cursor" : "normal"), Commons.Color.popups.text, Commons.Color.accent)
    }
}

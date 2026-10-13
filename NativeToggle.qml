import QtQuick
import qs.Commons as Commons
import qs.Ui as Ui
import "Typography.js" as Typography

Ui.Toggle {
    fontFamily: Typography.family
    id: root
    property bool rowHovered: false
    foreground: Commons.Color.popups.text
    titleSize: Commons.Style.font.body
    implicitHeight: Commons.Style.spacing.controlHeight + Commons.Style.spacing.md
    color: Commons.Style.controlFill(activeFocus, rowHovered, foreground, accent)
    borderSpec: activeFocus || rowHovered
        ? Commons.Border.controlSpec(activeFocus ? "focus" : "hover-cursor", foreground, accent) : Commons.Border.none()
    onHovered: function(hovered) { rowHovered = hovered }
}

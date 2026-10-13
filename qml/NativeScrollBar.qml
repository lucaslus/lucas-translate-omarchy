import QtQuick
import QtQuick.Controls
import qs.Commons as Commons

ScrollBar {
    id: root
    padding: Commons.Style.spacing.xxs
    minimumSize: 0.08
    readonly property color handleColor: pressed ? Commons.Color.accent : hovered ? Commons.Color.popups.text : Commons.Color.muted
    background: null
    contentItem: Rectangle {
        implicitWidth: Commons.Style.space(6)
        implicitHeight: Commons.Style.space(6)
        radius: Commons.Style.cornerRadius
        color: root.handleColor
        opacity: root.active || root.hovered || root.pressed ? 1 : 0
    }
}

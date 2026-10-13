import QtQuick
import qs.Commons as Commons

NativeButton {
    id: root
    property string name: ""
    property bool filled: false
    foreground: Commons.Color.popups.text
    focusable: true
    implicitHeight: Math.max(Commons.Style.spacing.controlHeight, Commons.Style.font.icon + Commons.Style.spacing.xs * 2)
    implicitWidth: implicitHeight
    Accessible.name: tooltipText
    NativeIcon {
        objectName: "toolbarIcon." + root.name
        anchors.centerIn: parent
        width: Commons.Style.font.icon + Commons.Style.space(2); height: width
        name: root.name; filled: root.filled; foreground: root.foreground
    }
}

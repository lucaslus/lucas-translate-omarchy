import QtQuick
import qs.Commons as Commons
import "Theme.js" as Theme
import "Typography.js" as Typography

Text {
    textFormat: Text.PlainText
    property bool secondary: false
    color: secondary ? Theme.secondary(Commons.Color.muted, Commons.Color.popups.text, Commons.Color.popups.background) : Commons.Color.popups.text
    font.family: Typography.family
    font.pixelSize: Commons.Style.font.body
    wrapMode: Text.Wrap
}

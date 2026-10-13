import QtQuick
import qs.Commons as Commons
import qs.Ui as Ui
import "Theme.js" as Theme
import "Typography.js" as Typography

Ui.TextField {
    font.family: Typography.family
    foreground: Commons.Color.popups.text
    placeholderTextColor: Theme.secondary(Commons.Color.muted, foreground,
        Theme.over(Commons.Style.controlFill(activeFocus, hovered, foreground, accent), Commons.Color.popups.background))
}

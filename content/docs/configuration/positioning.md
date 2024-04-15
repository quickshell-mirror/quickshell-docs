+++
title = "Positioning"
+++

QtQuick has multiple ways to position components. This page has instructions for where and how
to use them.

## Anchors
Anchors can be used to position components relative to another neighboring component.
It is faster than [manual positioning](#manual-positioning) and covers a lot of simple
use cases.

The [Qt Documentation: Positioning with Anchors](https://doc.qt.io/qt-6/qtquick-positioning-anchors.html)
page has comprehensive documentation of anchors.

## Layouts
Layouts are useful when you have many components that need to be positioned relative to
eachother such as a list.

The [Qt Documentation: Layouts Overview](https://doc.qt.io/qt-6/qtquicklayouts-overview.html)
page has good documentation of the basic layout types and how to use them.

Note: layouts by default have a nonzero spacing.

## Manual Positioning
If layouts and anchors can't easily fulfill your usecase, you can also manually position and size
components by setting their `x`, `y`, `width` and `height` properties, which are relative to
the parent component.

This example puts a 100x100px blue rectangle at x=20,y=40 in the parent item. Ensure the size
of the parent is large enough for its content or positioning based on them will break.
```qml
Item {
  // make sure the component is large enough to fit its children
  implicitWidth: childrenRect.width
  implicitHeight: childrenRect.height
  
  Rectangle {
    color: "blue"
    x: 20
    y: 40
    width: 100
    height: 100
  }
}
```

## Notes
### Component Size
The [Item.implicitHeight] and [Item.implicitWidth] properties control the *base size* of a
component, before layouts are applied. These properties are *not* the same as
[Item.height] and [Item.width] which are the final size of the component.
You should nearly always use the implicit size properties when creating a component,
however using the normal width and height properties is fine if you know an
item will never go in a layout.

[Item.height]: https://doc.qt.io/qt-6/qml-qtquick-item.html#height-prop
[Item.width]: https://doc.qt.io/qt-6/qml-qtquick-item.html#width-prop
[Item.implicitHeight]: https://doc.qt.io/qt-6/qml-qtquick-item.html#implicitHeight-prop
[Item.implicitWidth]: https://doc.qt.io/qt-6/qml-qtquick-item.html#implicitWidth-prop

This example component puts a colored rectangle behind some text, and will act the same
way in a layout as the text by itself.
```qml {filename="TextWithBkgColor.qml"}
Rectangle {
  implicitWidth: text.implicitWidth
  implicitHeight: text.implicitHeight
  
  Text {
    id: text
    text: "hello!"
  }
}
```

If you want to size your component based on multiple others or use any other math you can.
```qml {filename="PaddedTexts.qml"}
Item {
  // width of both texts plus 5
  implicitWidth: text1.implicitWidth + text2.implicitWidth + 5
  // max height of both texts plus 5
  implicitHeight: Math.min(text1.implicitHeight, text2.implicitHeight) + 5

  Text {
    id: text1
    text: "text1"
  }
  
  Text {
    id: text2
    anchors.left: text1.left
    text: "text2"
  }
}
```

### Coordinate space
You should always position or size components relative to the closest possible
parent. Often this is just the `parent` property.

Refrain from using things like the size of your screen to size a component,
as this will break as soon as anything up the component hierarchy changes, such
as adding padding to a bar.

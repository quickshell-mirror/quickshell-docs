+++
title = "Introduction"
+++

This page will walk you through the process of creating a simple bar/panel, and
introduce you to all the basic concepts involved.

There are many links to the [QML Overview](../qml-overview)
and [Type Reference](/docs/types) which you should follow if you don't
fully understand the concepts involved.

## Shell Files

Every quickshell instance starts from a shell root file, conventionally named `shell.qml`.
The default path is `~/.config/quickshell/shell.qml`.
(where `~/.config` can be substituted with `$XDG_CONFIG_HOME` if present.)

Each shell file starts with the shell root object. Only one may exist per configuration.

```qml {filename="~/.config/quickshell/shell.qml"}
import Quickshell

ShellRoot {
  // ...
}
```

The shell root is not a visual element but instead contains all of the visual
and non visual objects in your shell. You can have multiple different shells
with shared components and different shell roots.

{{% details title="Shell search paths and manifests" closed="true" %}}

Quickshell can be launched with configurations in locations other than the default one.

The `-p` or `--path` option will launch the shell root at the given path.
It will also accept folders with a `shell.qml` file in them.
It can also be specified via the `QS_CONFIG_PATH` environment variable.

The `-c` or `--config` option will launch a configuration from the current manifest,
or if no manifest is specified, a subfolder of quickshell's base path.
It can also be specified via the `QS_CONFIG_NAME` environment variable.

The base path defaults to `~/.config/quickshell`, but can be changed using
the `QS_BASE_PATH` environment variable.

The `-m` or `--manifest` option specifies the quickshell manifest to read configs
from. When used with `-c`, the config will be chosen by name from the manifest.
It can also be specified via the `QS_MANIFEST` environment variable.

The manifest path defaults to `~/.config/quickshell/manifest.conf` and is a list
of `name = path` pairs where path can be relative or absolute.
Lines starting with `#` are comments.

```properties
# ~/.config/quickshell/manifest.conf
myconf1 = myconf
myconf2 = ./myconf
myconf3 = myconf/shell.nix
myconf4 = ~/.config/quickshell/myconf
```

You can use `quickshell --current` to print the current values of any of these
options and what set them.

{{% /details %}}

## Creating Windows

Quickshell has two main window types available,
[PanelWindow](/docs/types/quickshell/panelwindow) for bars and widgets, and
[FloatingWindow](/docs/types/quickshell/floatingwindow) for standard desktop windows.

We'll start with an example:
```qml
import Quickshell // for ShellRoot and PanelWindow
import QtQuick // for Text

ShellRoot {
  PanelWindow {
    anchors {
      top: true
      left: true
      right: true
    }

    height: 30

    Text {
      // center the bar in its parent component (the window)
      anchors.centerIn: parent

      text: "hello world"
    }
  }
}
```

The above example creates a bar/panel on your currently focused monitor with
a centered piece of [text](https://doc.qt.io/qt-6/qml-qtquick-text.html). It will also reserve space for itself on your monitor.

More information about available properties is available in the [type reference](/docs/types/quickshell/panelwindow).

## Running a process

Now that we have a piece of text, what if it did something useful?
To start with lets make a clock. To get the time we'll use the `date` command.

We can use a [Process](/docs/types/quickshell.io/process) object to run commands
and return their results.

We'll listen to the [DataStreamParser.read](/docs/types/quickshell.io/datastreamparser/#signal.read)
[signal](/docs/configuration/qml-overview/#signals) emitted by
[SplitParser](/docs/types/quickshell.io/splitparser/) using a
[signal handler](/docs/configuration/qml-overview/#signal-handlers)
to update the text on the clock.

{{< callout type="info" >}}
Note: Quickshell live-reloads your code. You can leave it open and edit the
original file. The panel will reload when you save it.
{{< /callout >}}

```qml
import Quickshell
import Quickshell.Io // for Process
import QtQuick

ShellRoot {
  PanelWindow {
    anchors {
      top: true
      left: true
      right: true
    }

    height: 30

    Text {
      // give the text an ID we can refer to elsewhere in the file
      id: clock

      anchors.centerIn: parent

      // create a process management object
      Process {
        // the command it will run, every argument is its own string
        command: ["date"]

        // run the command immediately
        running: true

        // process the stdout stream using a SplitParser
        // which returns chunks of output after a delimiter
        stdout: SplitParser {
          // listen for the read signal, which returns the data that was read
          // from stdout, then write that data to the clock's text property
          onRead: data => clock.text = data
        }
      }
    }
  }
}
```

## Running code at an interval
With the above example, your bar should now display the time, but it isn't updating!
Let's use a [Timer](https://doc.qt.io/qt-6/qml-qtqml-timer.html) fix that.

```qml
import Quickshell
import Quickshell.Io
import QtQuick

ShellRoot {
  PanelWindow {
    anchors {
      top: true
      left: true
      right: true
    }

    height: 30

    Text {
      id: clock
      anchors.centerIn: parent

      Process {
        // give the process object an id so we can talk
        // about it from the timer
        id: dateProc

        command: ["date"]
        running: true

        stdout: SplitParser {
          onRead: data => clock.text = data
        }
      }

      // use a timer to rerun the process at an interval
      Timer {
        // 1000 milliseconds is 1 second
        interval: 1000

        // start the timer immediately
        running: true

        // run the timer again when it ends
        repeat: true

        // when the timer is triggered, set the running property of the
        // process to true, which reruns it if stopped.
        onTriggered: dateProc.running = true
      }
    }
  }
}
```

## Reuseable components

If you have multiple monitors you might have noticed that your bar
is only on one of them. If not, you'll still want to **follow this section
to make sure your bar dosen't disappear if your monitor disconnects**.

We can use a [Variants](http://localhost:1313/docs/types/quickshell/variants/)
object to create instances of *non widget items*.
(See [Repeater](https://doc.qt.io/qt-6/qml-qtquick-repeater.html) for doing
something similar with visual items.)

The `Variants` type creates instances of a
[Component](https://doc.qt.io/qt-6/qml-qtqml-component.html) based on a data model
you supply. (A component is a re-usable tree of objects.)

The most common use of `Variants` in a shell is to create instances of
a window (your bar) based on your monitor list (the data model).

Variants will inject the properties in the data model directly into the component,
meaning we can easily set the screen property of our bar
(See [Window.screen](/docs/types/quickshell/qswindow/#prop.screen).)

```qml
import Quickshell
import Quickshell.Io
import QtQuick

ShellRoot {
  Variants {
    variants: {
      // get the list of screens from the Quickshell singleton
      const screens = Quickshell.screens;

      // transform the screen list into a list of objects with
      // screen variables, which will be set for each created object
      const variants = screens.map(screen => {
        return { screen: screen };
      });

      return variants;
    }

    component: Component {
      PanelWindow {
        // the screen property will be injected into the window,
        // so each bar displays on the right monitor

        anchors {
          top: true
          left: true
          right: true
        }

        height: 30

        Text {
          id: clock
          anchors.centerIn: parent

          Process {
            id: dateProc
            command: ["date"]
            running: true

            stdout: SplitParser {
              onRead: data => clock.text = data
            }
          }

          Timer {
            interval: 1000
            running: true
            repeat: true
            onTriggered: dateProc.running = true
          }
        }
      }
    }
  }
}
```

<span class="small">See also:
[Property Bindings](/docs/configuration/qml-overview/#property-bindings),
[Variants.component](/docs/types/quickshell/variants/#prop.component),
[Quickshell.screens](/docs/types/quickshell/quickshell/#prop.screens),
[Array.map](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/map)
</span>

With this example, bars will be created and destroyed as you plug and unplug them,
due to the reactive nature of the
[Quickshell.screens](/docs/types/quickshell/quickshell/#prop.screens) property.
(See: [Reactive Bindings](/docs/configuration/qml-overview/#reactive-bindings).)

Now there's an important problem you might have noticed: when the window
is created multiple times we also make a new Process and Timer. We can fix
this by moving the Process and Timer outside of the window.

{{< callout type="error" >}}
This code will not work correctly.
{{< /callout >}}

```qml
import Quickshell
import Quickshell.Io
import QtQuick

ShellRoot {
  Variants {
    variants: Quickshell.screens.map(screen => ({ screen }))

    component: Component {
      PanelWindow {
        anchors {
          top: true
          left: true
          right: true
        }

        height: 30

        Text {
          id: clock
          anchors.centerIn: parent
        }
      }
    }
  }

  Process {
    id: dateProc
    command: ["date"]
    running: true

    stdout: SplitParser {
      onRead: data => clock.text = data
    }
  }

  Timer {
    interval: 1000
    running: true
    repeat: true
    onTriggered: dateProc.running = true
  }
}
```

However there is a problem with naively moving the Process and Timer
out of the component.
*What about the `clock` that the process references?*

If you run the above example you'll see something like this in the console every second:

```
file:///home/name/.config/quickshell/shell.qml:33: ReferenceError: clock is not defined
file:///home/name/.config/quickshell/shell.qml:33: ReferenceError: clock is not defined
file:///home/name/.config/quickshell/shell.qml:33: ReferenceError: clock is not defined
file:///home/name/.config/quickshell/shell.qml:33: ReferenceError: clock is not defined
file:///home/name/.config/quickshell/shell.qml:33: ReferenceError: clock is not defined
```

This is because the `clock` object, even though it has an ID, cannot be referenced
outside of its component. Remember, components can be created *any number of times*,
including zero, so `clock` may not exist or there may be more than one, meaning
there isn't an object to refer to from here.

We can fix it with a [Property Definition](/docs/configuration/qml-overview/#property-definitions).

We can define a property inside of the ShellRoot and reference it from the clock
text instead. Due to QML's [Reactive Bindings](/docs/configuration/qml-overview/#reactive-bindings),
the clock text will be updated when we update the property for every clock that
currently exists.

```qml
import Quickshell
import Quickshell.Io
import QtQuick

ShellRoot {
  id: root

  // add a property in the root
  property string time;

  Variants {
    variants: Quickshell.screens.map(screen => ({ screen }))

    component: Component {
      PanelWindow {
        anchors {
          top: true
          left: true
          right: true
        }

        height: 30

        Text {
          // remove the id as we don't need it anymore

          anchors.centerIn: parent

          // bind the text to the root's time property
          text: root.time
        }
      }
    }
  }

  Process {
    id: dateProc
    command: ["date"]
    running: true

    stdout: SplitParser {
      // update the property instead of the clock directly
      onRead: data => root.time = data
    }
  }

  Timer {
    interval: 1000
    running: true
    repeat: true
    onTriggered: dateProc.running = true
  }
}
```

Now we've fixed the problem so there's nothing actually wrong with the
above code, but we can make it more concise:

1. `Component`s can be defined implicitly, meaning we can remove the
component wrapping the window and place the window directly into the
`component` property.
2. The [Variants.component](/docs/types/quickshell/variants/#prop.component)
property is a [Default Property](/docs/configuration/qml-overview/#the-default-property),
which means we can skip the `component: ` part of the assignment.
We're already using [ShellRoot](/docs/types/quickshell/shellroot/)'s
default property to store our Variants, Process, and Timer components
among other things.
3. The ShellRoot dosen't actually need an `id` property to talk about
the time property, as it is the outermost object in the file which
has [special scoping rules](/docs/configuration/qml-overview/#property-access-scopes).

This is what our shell looks like with the above (optional) cleanup:

```qml
import Quickshell
import Quickshell.Io
import QtQuick

ShellRoot {
  property string time;

  Variants {
    variants: Quickshell.screens.map(screen => ({ screen }))

    PanelWindow {
      anchors {
        top: true
        left: true
        right: true
      }

      height: 30

      Text {
        anchors.centerIn: parent

        // now just time instead of root.time
        text: time
      }
    }
  }

  Process {
    id: dateProc
    command: ["date"]
    running: true

    stdout: SplitParser {
      // now just time instead of root.time
      onRead: data => time = data
    }
  }

  Timer {
    interval: 1000
    running: true
    repeat: true
    onTriggered: dateProc.running = true
  }
}
```

## Multiple files

In an example as small as this, it isn't a problem, but as the shell
grows it might be prefferable to separate it into multiple files.

To start with, let's move the entire bar into a new file.
```qml {filename="shell.qml"}
import Quickshell

ShellRoot {
  Bar {}
}
```

```qml {filename="Bar.qml"}
import Quickshell
import Quickshell.Io
import QtQuick

Scope {
  property string time;

  Variants {
    variants: Quickshell.screens.map(screen => ({ screen }))

    PanelWindow {
      anchors {
        top: true
        left: true
        right: true
      }

      height: 30

      Text {
        anchors.centerIn: parent

        // now just time instead of root.time
        text: time
      }
    }
  }

  Process {
    id: dateProc
    command: ["date"]
    running: true

    stdout: SplitParser {
      // now just time instead of root.time
      onRead: data => time = data
    }
  }

  Timer {
    interval: 1000
    running: true
    repeat: true
    onTriggered: dateProc.running = true
  }
}
```
<span class="small">See also: [Scope](/docs/types/quickshell/scope/)</span>

Any qml file that starts with an uppercase letter can be referenced this way.
We can bring in other folders as well using
[import statements](/docs/configuration/qml-overview/#explicit-imports).

Now what about breaking out the clock? This is a bit more complex because
the clock component in the bar, as well as the process and timer that
make up the actual clock, need to be dealt with.

To start with, we can move the clock widget to a new file. For now it's just a
single `Text` object but the same concepts apply regardless of complexity.

```qml {filename="ClockWidget.qml"}
import QtQuick

// Item is a common base type for visual components
Item {
  // make a property the creator of this type is required to set
  required property string time

  // size the item to its children
  width: childrenRect.width
  height: childrenRect.height

  // use the default property to contain the clock
  Text {
    text: time
  }
}
```

```qml {filename="Bar.qml"}
import Quickshell
import Quickshell.Io
import QtQuick

Scope {
  id: root
  property string time;

  Variants {
    variants: Quickshell.screens.map(screen => ({ screen }))

    PanelWindow {
      anchors {
        top: true
        left: true
        right: true
      }

      height: 30

      // the ClockWidget type we just created
      ClockWidget {
        anchors.centerIn: parent
        // Warning: setting `time: time` will bind time to itself which is not what we want
        time: root.time
      }
    }
  }

  Process {
    id: dateProc
    command: ["date"]
    running: true

    stdout: SplitParser {
      onRead: data => time = data
    }
  }

  Timer {
    interval: 1000
    running: true
    repeat: true
    onTriggered: dateProc.running = true
  }
}
```

While this example is larger than what we had before, we can now expand
on the clock widget without cluttering the bar file.

Let's deal with the clock's update logic now:

```qml {filename="Time.qml"}
import Quickshell
import Quickshell.Io
import QtQuick

Scope {
  property string time;

  Process {
    id: dateProc
    command: ["date"]
    running: true

    stdout: SplitParser {
      onRead: data => time = data
    }
  }

  Timer {
    interval: 1000
    running: true
    repeat: true
    onTriggered: dateProc.running = true
  }
}
```

```qml {filename="Bar.qml"}
import Quickshell

Scope {
  // the Time type we just created
  Time { id: timeSource }

  Variants {
    variants: Quickshell.screens.map(screen => ({ screen }))

    PanelWindow {
      anchors {
        top: true
        left: true
        right: true
      }

      height: 30

      ClockWidget {
        anchors.centerIn: parent
        // now using the time from timeSource
        time: timeSource.time
      }
    }
  }
}
```

## Singletons

Now you might be thinking, why do we need the `Time` type in
our bar file, and the answer is we don't. We can make `Time`
a [Singleton](/docs/configuration/qml-overview/#singletons).

A singleton object has only one instance, and is accessible from
any scope.

```qml {filename="Time.qml"}
// with this line our type becomes a singleton
pragma Singleton

import Quickshell
import Quickshell.Io
import QtQuick

// your singletons should always have Singleton as the type
Singleton {
  property string time;

  Process {
    id: dateProc
    command: ["date"]
    running: true

    stdout: SplitParser {
      onRead: data => time = data
    }
  }

  Timer {
    interval: 1000
    running: true
    repeat: true
    onTriggered: dateProc.running = true
  }
}
```

```qml {filename="ClockWidget.qml"}
import QtQuick

Item {
  // we no longer need time as an input

  width: childrenRect.width
  height: childrenRect.height

  Text {
    // directly access the time property from the Time singleton
    text: Time.time
  }
}
```

```qml {filename="Bar.qml"}
import Quickshell

Scope {
  // no more time object

  Variants {
    variants: Quickshell.screens.map(screen => ({ screen }))

    PanelWindow {
      anchors {
        top: true
        left: true
        right: true
      }

      height: 30

      ClockWidget {
        anchors.centerIn: parent

        // no more time binding
      }
    }
  }
}
```

# Gpui-Component - Llms-Txt

**Pages:** 61

---

## Optional, for default bundled assets

**URL:** llms-txt#optional,-for-default-bundled-assets

**Contents:**
- Quick Start
- Basic Concepts
  - Stateless Elements
  - Stateful Components
  - Theming
  - Sizing
  - Variants
- Icons
- Next Steps
- Development

gpui-component-assets = "{{ VERSION }}"
anyhow = "1.0"
rust
use gpui::*;
use gpui_component::{button::*, *};

pub struct HelloWorld;

impl Render for HelloWorld {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .v_flex()
            .gap_2()
            .size_full()
            .items_center()
            .justify_center()
            .child("Hello, World!")
            .child(
                Button::new("ok")
                    .primary()
                    .label("Let's Go!")
                    .on_click(|_, _, _| println!("Clicked!")),
            )
    }
}

fn main() {
    let app = Application::new().with_assets(gpui_component_assets::Assets);

app.run(move |cx| {
        // This must be called before using any GPUI Component features.
        gpui_component::init(cx);

cx.spawn(async move |cx| {
            cx.open_window(WindowOptions::default(), |window, cx| {
                let view = cx.new(|_| HelloWorld);
                // This first level on the window, should be a Root.
                cx.new(|cx| Root::new(view, window, cx))
            })?;

Ok::<_, anyhow::Error>(())
        })
        .detach();
    });
}
rs
struct MyView;

impl Render for MyView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .child(Button::new("btn").label("Click Me"))
            .child(Tag::secondary().child("Secondary"))
    }
}
rs
struct MyView {
    input: Entity<InputState>,
}

impl MyView {
    fn new(window: &Window, cx: &mut Context<Self>) -> Self {
        let input = cx.new(|cx| InputState::new(window, cx).default_value("Hello 世界"));
        Self { input }
    }
}

impl Render for MyView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.input.clone()
    }
}
rust
use gpui_component::{ActiveTheme, Theme};

// Access theme colors in your components
cx.theme().primary
cx.theme().background
cx.theme().foreground
rust
Button::new("btn").small()
Button::new("btn").medium() // default
Button::new("btn").large()
Button::new("btn").xsmall()
rust
Button::new("btn").primary()
Button::new("btn").danger()
Button::new("btn").warning()
Button::new("btn").success()
Button::new("btn").ghost()
Button::new("btn").outline()
rust
use gpui_component::{Icon, IconName};

Icon::new(IconName::Check)
Icon::new(IconName::Search).small()
bash
cargo run
bash
cargo run --example <example_name>
```

[RenderOnce]: https://docs.rs/gpui/latest/gpui/trait.RenderOnce.html

[IntoElement]: https://docs.rs/gpui/latest/gpui/trait.IntoElement.html

[Render]: https://docs.rs/gpui/latest/gpui/trait.Render.html

---
url: /gpui-component/README.md
---

**Examples:**

Example 1 (unknown):
```unknown
:::tip
The `gpui-component-assets` crate is optional.

It provides a default set of icon assets. If you want to manage your own assets, you can skip adding this dependency.

See [Icons & Assets](./assets.md) for more details.
:::

## Quick Start

Here's a simple example to get you started:
```

Example 2 (unknown):
```unknown
:::info
Make sure to call `gpui_component::init(cx);` at first line inside the `app.run` closure. This initializes the GPUI Component system.

This is required for theming and other global settings to work correctly.
:::

## Basic Concepts

### Stateless Elements

GPUI Component uses stateless [RenderOnce] elements, making them simple and predictable. State management is handled at the view level, not in individual components.

The are all implemented [IntoElement] types.

For example:
```

Example 3 (unknown):
```unknown
### Stateful Components

There are some stateful components like `Dropdown`, `List`, and `Table` that manage their own internal state for convenience, these components implement the [Render] trait.

Those components to use are a bit different, we need create the \[Entity] and hold it in the view struct.
```

Example 4 (unknown):
```unknown
### Theming

All components support theming through the built-in `Theme` system:
```

---

## Alert

**URL:** llms-txt#alert

**Contents:**
- Import
- Usage
  - Basic Alert
  - Alert with Title
  - Alert Variants
  - Alert Sizes
  - Closable Alerts
  - Banner Mode
  - Custom Icons
  - With Markdown Content

A versatile alert component for displaying important messages to users. Supports multiple variants (info, success, warning, error), custom icons, optional titles, closable functionality, and banner mode. Perfect for notifications, status messages, and user feedback.

When you add an `on_close` handler, a close button appears on the alert:

Banner alerts take full width and don't display titles:

### With Markdown Content

We can use `TextView` to render formatted (Markdown or HTML) text within the alert,
for displaying lists, bold text, links, etc.

### Conditional Visibility

### Form Validation Errors

### Success Notification

### System Status Banner

### Interactive Alert with Custom Action

### Multi-line Content with Formatting

[Alert]: https://docs.rs/gpui-component/latest/gpui_component/alert/struct.Alert.html

---
url: /gpui-component/docs/components/avatar.md
description: Displays a user avatar image with fallback options.
---

**Examples:**

Example 1 (rust):
```rust
use gpui_component::alert::Alert;
```

Example 2 (rust):
```rust
Alert::new("alert-id", "This is a basic alert message.")
```

Example 3 (rust):
```rust
Alert::new("alert-with-title", "Your changes have been saved successfully.")
    .title("Success!")
```

Example 4 (rust):
```rust
// Info alert (blue)
Alert::info("info-alert", "This is an informational message.")
    .title("Information")

// Success alert (green)
Alert::success("success-alert", "Your operation completed successfully.")
    .title("Success!")

// Warning alert (yellow/orange)
Alert::warning("warning-alert", "Please review your settings before proceeding.")
    .title("Warning")

// Error alert (red)
Alert::error("error-alert", "An error occurred while processing your request.")
    .title("Error")
```

---

## Tag

**URL:** llms-txt#tag

**Contents:**
- Import
- Usage
  - Basic Tags
  - Tag Variants
  - Outline Tags
  - Tag Sizes
  - Custom Colors
  - Custom HSLA Colors
  - Rounded Corners
  - Combined Styles

A versatile tag component for categorizing and labeling content. Tags are compact visual indicators that help organize information and display metadata like categories, status, or properties.

### Custom HSLA Colors

## Tag Categories and Use Cases

### Priority Indicators

### Tag Creation Methods

| Method                      | Description                                |
| --------------------------- | ------------------------------------------ |
| `primary()`                 | Create a primary tag (blue theme)          |
| `secondary()`               | Create a secondary tag (gray theme)        |
| `danger()`                  | Create a danger tag (red theme)            |
| `success()`                 | Create a success tag (green theme)         |
| `warning()`                 | Create a warning tag (yellow/orange theme) |
| `info()`                    | Create an info tag (blue theme)            |
| `color(ColorName)`          | Create a tag with predefined color         |
| `custom(color, fg, border)` | Create a tag with custom HSLA colors       |

| Method            | Description                                  |
| ----------------- | -------------------------------------------- |
| `outline()`       | Apply outline style (transparent background) |
| `rounded(radius)` | Set custom border radius                     |
| `rounded_full()`  | Apply full rounding (pill shape)             |

### Size Methods (from Sizable trait)

| Method            | Description                      |
| ----------------- | -------------------------------- |
| `small()`         | Small tag size (reduced padding) |
| `with_size(size)` | Set custom size                  |

### Content Methods (from ParentElement trait)

| Method           | Description                  |
| ---------------- | ---------------------------- |
| `child(element)` | Add child content to the tag |

### Status Dashboard Tags

### Interactive Tag Lists

### Color-Coded Categories

* Tags automatically adjust their appearance based on the current theme
* Outline tags maintain border visibility across different backgrounds
* Small tags use reduced padding and border radius for compact layouts
* Custom colors support both light and dark theme adaptations
* Tags are display components and don't include built-in interaction handlers
* Multiple tags can be combined in flex layouts for tag clouds or lists
* Border radius automatically scales based on tag size unless explicitly overridden

* **Categorization**: Group content by type, topic, or theme
* **Status Indication**: Show state, progress, or health status
* **Metadata Display**: Present attributes, properties, or classifications
* **Filtering**: Visual indicators for active filters or selections
* **Feature Flags**: Highlight new, beta, or special features

* **Semantic Colors**: Use danger (red) for errors, success (green) for completion, warning (yellow) for caution, info (blue) for information
* **Category Colors**: Use the ColorName variants for content categorization where color coding helps with recognition
* **Custom Colors**: Reserve for brand colors or specific design system requirements

* **Small Tags**: Use for compact layouts, metadata, or when space is limited
* **Medium Tags**: Default size for most use cases, provides good readability and click targets
* **Rounding**: Use `rounded_full()` for pill-style tags, custom `rounded()` for specific design requirements

---
url: /gpui-component/docs/theme.md
---

**Examples:**

Example 1 (rust):
```rust
use gpui_component::tag::Tag;
```

Example 2 (rust):
```rust
// Primary tag (default filled style)
Tag::primary().child("Primary")

// Secondary tag
Tag::secondary().child("Secondary")

// Status tags
Tag::danger().child("Danger")
Tag::success().child("Success")
Tag::warning().child("Warning")
Tag::info().child("Info")
```

Example 3 (rust):
```rust
// Semantic variants
Tag::primary().child("Featured")
Tag::secondary().child("Category")
Tag::danger().child("Critical")
Tag::success().child("Completed")
Tag::warning().child("Pending")
Tag::info().child("Information")
```

Example 4 (rust):
```rust
// Outline style variants
Tag::primary().outline().child("Primary Outline")
Tag::secondary().outline().child("Secondary Outline")
Tag::danger().outline().child("Error Outline")
Tag::success().outline().child("Success Outline")
Tag::warning().outline().child("Warning Outline")
Tag::info().outline().child("Info Outline")
```

---

## Radio

**URL:** llms-txt#radio

**Contents:**
- Import
- Usage
  - Basic Radio Button
  - Controlled Radio Button
  - Radio Group (Recommended)
  - Different Sizes
  - Disabled State
  - Multi-line Label with Custom Content
  - Custom Tab Order
- Radio Group Usage

Radio buttons allow users to select a single option from a set of mutually exclusive choices. Use radio buttons when you want to give users a choice between multiple options and only one selection is allowed.

### Basic Radio Button

### Controlled Radio Button

### Radio Group (Recommended)

### Multi-line Label with Custom Content

### Horizontal Layout

### Styled Radio Group

### Disabled Radio Group

| Method             | Description                                                 |
| ------------------ | ----------------------------------------------------------- |
| `new(id)`          | Create a new radio button with the given ID                 |
| `label(text)`      | Set label text                                              |
| `checked(bool)`    | Set checked state                                           |
| `disabled(bool)`   | Set disabled state                                          |
| `on_click(fn)`     | Callback when clicked, receives `&bool` (new checked state) |
| `tab_stop(bool)`   | Enable/disable tab navigation (default: true)               |
| `tab_index(isize)` | Set tab order index (default: 0)                            |

| Method                          | Description                                                         |
| ------------------------------- | ------------------------------------------------------------------- |
| `horizontal(id)`                | Create a new horizontal radio group                                 |
| `vertical(id)`                  | Create a new vertical radio group                                   |
| `layout(Axis)`                  | Set layout direction (Vertical or Horizontal)                       |
| `child(Radio)`                  | Add a single radio button to the group                              |
| `children(items)`               | Add multiple radio buttons from an iterator                         |
| `selected_index(Option<usize>)` | Set the selected option by index                                    |
| `disabled(bool)`                | Disable all radio buttons in the group                              |
| `on_change(fn)`                 | Callback when selection changes, receives `&usize` (selected index) |

Both Radio and RadioGroup implement `Styled` trait for custom styling:

Radio also implements `Sizable` trait:

* `xsmall()` - Extra small size
* `small()` - Small size
* `medium()` - Medium size (default)
* `large()` - Large size

### Payment Method Selection

1. **Use RadioGroup**: Always prefer `RadioGroup` over individual `Radio` components for mutually exclusive choices
2. **Clear Labels**: Provide descriptive labels that clearly indicate what each option represents
3. **Default Selection**: Consider providing a sensible default selection, especially for required fields
4. **Logical Order**: Arrange options in a logical order (alphabetical, frequency of use, or importance)
5. **Limit Options**: Keep the number of radio options reasonable (typically 2-7 options)
6. **Group Related Options**: Use visual grouping and clear headings for multiple radio groups
7. **Responsive Design**: Consider using horizontal layout for fewer options and vertical for more options

---
url: /gpui-component/docs/components/resizable.md
description: >-
  A flexible panel layout system with draggable resize handles and adjustable
  panels.
---

**Examples:**

Example 1 (rust):
```rust
use gpui_component::radio::{Radio, RadioGroup};
```

Example 2 (rust):
```rust
Radio::new("radio-option-1")
    .label("Option 1")
    .checked(false)
    .on_click(|checked, _, _| {
        println!("Radio is now: {}", checked);
    })
```

Example 3 (rust):
```rust
struct MyView {
    radio_checked: bool,
}

impl Render for MyView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        Radio::new("radio")
            .label("Select this option")
            .checked(self.radio_checked)
            .on_click(cx.listener(|view, checked, _, cx| {
                view.radio_checked = *checked;
                cx.notify();
            }))
    }
}
```

Example 4 (rust):
```rust
struct MyView {
    selected_option: Option<usize>,
}

impl Render for MyView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        RadioGroup::horizontal("options")
            .children(["Option 1", "Option 2", "Option 3"])
            .selected_index(self.selected_option)
            .on_change(cx.listener(|view, selected_index: &usize, _, cx| {
                view.selected_option = Some(*selected_index);
                cx.notify();
            }))
    }
}
```

---

## Skeleton

**URL:** llms-txt#skeleton

**Contents:**
- Import
- Usage
  - Basic Skeleton
  - Text Line Skeleton
  - Circle Skeleton
  - Rectangle Skeleton
  - Different Shapes
  - Secondary Variant
- Animation
- Sizes

The Skeleton component displays animated placeholder content while actual content is loading. It provides visual feedback to users that content is being loaded and helps maintain layout structure during loading states.

### Text Line Skeleton

### Rectangle Skeleton

### Secondary Variant

The Skeleton component includes a built-in pulse animation that:

* Runs continuously with a 2-second duration
* Uses a bounce easing function with ease-in-out
* Animates opacity from 100% to 50% and back
* Automatically repeats to indicate loading state

The animation cannot be disabled as it's essential for indicating loading state.

The Skeleton component doesn't have predefined size variants. Instead, use gpui's sizing utilities:

### Loading Profile Card

### Loading Article List

### Loading Table Rows

### Loading Button States

### Loading Form Fields

### Conditional Loading

The Skeleton component uses the theme's `skeleton` color, which defaults to the `secondary` color if not specified. You can customize it in your theme:

The `secondary(true)` variant applies 50% opacity to the skeleton color for more subtle loading indicators.

---
url: /gpui-component/docs/components/slider.md
description: >-
  A control that allows the user to select values from a range using a draggable
  thumb.
---

**Examples:**

Example 1 (rust):
```rust
use gpui_component::skeleton::Skeleton;
```

Example 2 (rust):
```rust
Skeleton::new()
```

Example 3 (rust):
```rust
// Single line of text
Skeleton::new()
    .w(px(250.))
    .h_4()
    .rounded_md()

// Multiple text lines
v_flex()
    .gap_2()
    .child(Skeleton::new().w(px(250.)).h_4().rounded_md())
    .child(Skeleton::new().w(px(200.)).h_4().rounded_md())
    .child(Skeleton::new().w(px(180.)).h_4().rounded_md())
```

Example 4 (rust):
```rust
// Avatar placeholder
Skeleton::new()
    .size_12()
    .rounded_full()

// Profile picture placeholder
Skeleton::new()
    .w(px(64.))
    .h(px(64.))
    .rounded_full()
```

---

## Stepper

**URL:** llms-txt#stepper

**Contents:**
- Import
- Usage
  - Basic Stepper
  - With Icons
  - Vertical Layout
  - Text Center
  - Different Sizes
  - Disabled State
  - Handle Click Events
- API Reference

A step-by-step progress component that guides users through a series of steps or stages. Supports horizontal and vertical layouts, custom icons, and different sizes.

Use `selected_index` method to set current active step by index (0-based), default is `0`.

The `text_center` method centers the text within each step item.

### Handle Click Events

* [Stepper]
* [StepperItem]

Implements [Sizable] trait:

* `xsmall()` - Extra small size
* `small()` - Small size
* `medium()` - Medium size (default)
* `large()` - Large size

### Disabled Individual Steps

[Stepper]: https://docs.rs/gpui-component/latest/gpui_component/stepper/struct.Stepper.html

[StepperItem]: https://docs.rs/gpui-component/latest/gpui_component/stepper/struct.StepperItem.html

[Sizable]: https://docs.rs/gpui-component/latest/gpui_component/trait.Sizable.html

---
url: /gpui-component/docs/components/switch.md
description: A control that allows the user to toggle between checked and not checked.
---

**Examples:**

Example 1 (rust):
```rust
use gpui_component::stepper::{Stepper, StepperItem};
```

Example 2 (rust):
```rust
Stepper::new("my-stepper")
    .selected_index(0)
    .items([
        StepperItem::new().child("Step 1"),
        StepperItem::new().child("Step 2"),
        StepperItem::new().child("Step 3"),
    ])
    .on_click(|step, _, _| {
        println!("Clicked step: {}", step);
    })
```

Example 3 (rust):
```rust
use gpui_component::IconName;

Stepper::new("icon-stepper")
    .selected_index(0)
    .items([
        StepperItem::new()
            .icon(IconName::Calendar)
            .child("Order Details"),
        StepperItem::new()
            .icon(IconName::Inbox)
            .child("Shipping"),
        StepperItem::new()
            .icon(IconName::Frame)
            .child("Preview"),
        StepperItem::new()
            .icon(IconName::Info)
            .child("Finish"),
    ])
```

Example 4 (rust):
```rust
Stepper::new("vertical-stepper")
    .vertical()
    .selected_index(2)
    .items_center()
    .items([
        StepperItem::new()
            .pb_8()
            .icon(IconName::Building2)
            .child(v_flex().child("Step 1").child("Description for step 1.")),
        StepperItem::new()
            .pb_8()
            .icon(IconName::Asterisk)
            .child(v_flex().child("Step 2").child("Description for step 2.")),
        StepperItem::new()
            .pb_8()
            .icon(IconName::Folder)
            .child(v_flex().child("Step 3").child("Description for step 3.")),
        StepperItem::new()
            .icon(IconName::CircleCheck)
            .child(v_flex().child("Step 4").child("Description for step 4.")),
    ])
```

---

## Accordion

**URL:** llms-txt#accordion

**Contents:**
- Import
- Usage
  - Basic Accordion
  - Multiple Open Items
  - With Borders
  - Different Sizes
  - Handle Toggle Events
  - Disabled State
- API Reference
  - Sizing

An accordion component that allows users to show and hide sections of content. It uses collapse functionality internally to create collapsible panels.

### Multiple Open Items

By default, only one accordion item can be open at a time. Use `multiple()` to allow multiple items to be open:

### Handle Toggle Events

* [Accordion]
* [AccordionItem]

Implements [Sizable] trait:

* `small()` - Small size
* `medium()` - Medium size (default)
* `large()` - Large size
* `xsmall()` - Extra small size

### With Custom Icons

### Nested Accordions

[Accordion]: https://docs.rs/gpui-component/latest/gpui_component/accordion/struct.Accordion.html

[AccordionItem]: https://docs.rs/gpui-component/latest/gpui_component/accordion/struct.AccordionItem.html

[Sizable]: https://docs.rs/gpui-component/latest/gpui_component/trait.Sizable.html

---
url: /gpui-component/docs/components/alert.md
description: Displays a callout for user attention.
---

**Examples:**

Example 1 (rust):
```rust
use gpui_component::accordion::Accordion;
```

Example 2 (rust):
```rust
Accordion::new("my-accordion")
    .item(|item| {
        item.title("Section 1")
            .child("Content for section 1")
    })
    .item(|item| {
        item.title("Section 2")
            .child("Content for section 2")
    })
    .item(|item| {
        item.title("Section 3")
            .child("Content for section 3")
    })
```

Example 3 (rust):
```rust
Accordion::new("my-accordion")
    .multiple(true)
    .item(|item| item.title("Section 1").child("Content 1"))
    .item(|item| item.title("Section 2").child("Content 2"))
```

Example 4 (rust):
```rust
Accordion::new("my-accordion")
    .bordered(true)
    .item(|item| item.title("Section 1").child("Content 1"))
```

---

## Root View

**URL:** llms-txt#root-view

**Contents:**
- Overlays

The [Root] component for as the root provider of GPUI Component features in a window. We must to use [Root] as the **first level child** of a window to enable GPUI Component features.

This is important, if we don't use [Root] as the first level child of a window, there will have some unexpected behaviors.

We have dialogs, sheets, notifications, we need placement for them to show, so [Root] provides methods to render these overlays:

* [Root::render\_dialog\_layer](https://docs.rs/gpui-component/latest/gpui_component/struct.Root.html#method.render_dialog_layer) - Render the current opened modals.
* [Root::render\_sheet\_layer](https://docs.rs/gpui-component/latest/gpui_component/struct.Root.html#method.render_sheet_layer) - Render the current opened drawers.
* [Root::render\_notification\_layer](https://docs.rs/gpui-component/latest/gpui_component/struct.Root.html#method.render_notification_layer) - Render the notification list.

We can put these layers in the `render` method your first level view (Root > YourFirstView):

:::tip
Here the example we used `children` method, it because if there is no opened dialogs, sheets, notifications, these methods will return `None`, so GPUI will not render anything.
:::

[Root]: https://docs.rs/gpui-component/latest/gpui_component/root/struct.Root.html

---
url: /gpui-component/docs/components/scrollable.md
description: >-
  Scrollable container with custom scrollbars, scroll tracking, and
  virtualization support.
---

**Examples:**

Example 1 (rs):
```rs
fn main() {
    let app = Application::new();

    app.run(move |cx| {
        // This must be called before using any GPUI Component features.
        gpui_component::init(cx);

        cx.spawn(async move |cx| {
            cx.open_window(WindowOptions::default(), |window, cx| {
                let view = cx.new(|_| Example);
                // This first level on the window, should be a Root.
                cx.new(|cx| Root::new(view, window, cx))
            })?;

            Ok::<_, anyhow::Error>(())
        })
        .detach();
    });
}
```

Example 2 (rs):
```rs
struct MyApp;

impl Render for MyApp {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .child("My App Content")
            .children(Root::render_dialog_layer(cx))
            .children(Root::render_sheet_layer(cx))
            .children(Root::render_notification_layer(cx))
    }
}
```

---

## Form

**URL:** llms-txt#form

**Contents:**
- Import
- Usage
  - Basic Form
  - Horizontal Form Layout
  - Multi-Column Form
- Form Container and Layout
  - Vertical Layout (Default)
  - Horizontal Layout
  - Custom Sizing
- Form Validation

A comprehensive form component that provides structured layout for form fields with support for vertical/horizontal layouts, validation, field groups, and responsive multi-column layouts.

### Horizontal Form Layout

### Multi-Column Form

## Form Container and Layout

### Vertical Layout (Default)

### Horizontal Layout

### Field Descriptions

### Dynamic Descriptions

### Basic Submit Pattern

### Form with Action Buttons

### Custom Field Components

### Conditional Fields

## Grid Layout and Positioning

### Column Positioning

### Responsive Layout

### User Registration Form

### Settings Form with Sections

---
url: /gpui-component/docs/getting-started.md
description: Learn how to set up and use GPUI Component in your project
---

**Examples:**

Example 1 (rust):
```rust
use gpui_component::form::{field, v_form, h_form, Form, Field};
```

Example 2 (rust):
```rust
v_form()
    .child(
        field()
            .label("Name")
            .child(Input::new(&name_input))
    )
    .child(
        field()
            .label("Email")
            .child(Input::new(&email_input))
            .required(true)
    )
```

Example 3 (rust):
```rust
h_form()
    .label_width(px(120.))
    .child(
        field()
            .label("First Name")
            .child(Input::new(&first_name))
    )
    .child(
        field()
            .label("Last Name")
            .child(Input::new(&last_name))
    )
```

Example 4 (rust):
```rust
v_form()
    .columns(2) // Two-column layout
    .child(
        field()
            .label("First Name")
            .child(Input::new(&first_name))
    )
    .child(
        field()
            .label("Last Name")
            .child(Input::new(&last_name))
    )
    .child(
        field()
            .label("Bio")
            .col_span(2) // Span across both columns
            .child(Input::new(&bio_input))
    )
```

---

## PopupMenu

**URL:** llms-txt#popupmenu

**Contents:**
- Import
- Usage
  - ContextMenu
  - DropdownMenu
  - Anchor Position
  - Icons
  - Disabled State
  - Check state
  - Separators
  - Labels

The Menu component provides both context menus (right-click menus) and popup menus with comprehensive features including icons, keyboard shortcuts, submenus, separators, checkable items, and custom elements. Built with accessibility and keyboard navigation in mind.

Context menus appear when right-clicking on an element:

Dropdown menus are triggered by buttons or other interactive elements:

:::tip
As you see, the each menu item is associated with an [Action],
we choice this design to better integrate with GPUI's action and key binding system,
allowing menu items to automatically display keyboard shortcuts when applicable.

So, the [Action] is the recommended way to define menu item behaviors.

However, if you prefer not to use [Action]s, you can create custom menu items using the `item` method along with [PopupMenuItem].
There have a `on_click` callback to handle the click event directly.
:::

Control where the dropdown menu appears relative to the trigger:

Add icons to menu items for better visual clarity:

Create disabled menu items that cannot be activated:

Create menu items that show a check state:

By default, the check icon will be shown on the left side of the menu item, if this menu item has an icon, the check icon will replace the icon on the left side.

There also have a `check_side` option for you to config the check icon to be shown on the right side:

Use separators to group related menu items:

Add non-interactive labels to organize menu sections:

Create menu items that open external links:

Create custom menu items with complex content:

### Keyboard Shortcuts

Menu items automatically display keyboard shortcuts if they're bound to actions:

Create nested menus with submenu support:

### Submenus with Icons

Add icons to submenu headers:

:::warning
When you have enabled `scrollable()` on a menu, avoid using submenus within it, as this can lead to usability issues.
:::

For menus with many items, enable scrolling:

Control menu dimensions:

Set the focus context for handling menu actions:

* [PopupMenu]
* [context\_menu][context_menu]
* [PopupMenuItem]

### File Manager Context Menu

### Add MenuItem without action

Sometimes you may not like to define an action for a menu item, you just want add a `on_click` handler, in this case, the `item` and [PopupMenuItem] can help you:

### Editor Menu with Shortcuts

### Settings Menu with Custom Elements

## Keyboard Shortcuts

| Key               | Action                            |
| ----------------- | --------------------------------- |
| `↑` / `↓`         | Navigate menu items               |
| `←` / `→`         | Navigate submenus                 |
| `Enter` / `Space` | Activate menu item                |
| `Escape`          | Close menu                        |
| `Tab`             | Close menu and focus next element |

1. **Group Related Items**: Use separators to group related functionality
2. **Consistent Icons**: Use consistent iconography across your application
3. **Logical Order**: Place most common actions at the top
4. **Keyboard Shortcuts**: Provide shortcuts for frequently used actions
5. **Context Awareness**: Show only relevant items for the current context
6. **Progressive Disclosure**: Use submenus for complex hierarchies
7. **Clear Labels**: Use descriptive, action-oriented labels
8. **Reasonable Limits**: Use scrollable menus for more than 10-15 items

[PopupMenu]: https://docs.rs/gpui-component/latest/gpui_component/menu/struct.PopupMenu.html

[PopupMenuItem]: https://docs.rs/gpui-component/latest/gpui_component/menu/struct.PopupMenuItem.html

[context_menu]: https://docs.rs/gpui-component/latest/gpui_component/menu/trait.ContextMenuExt.html#method.context_menu

[Action]: https://docs.rs/gpui/latest/gpui/trait.Action.html

---
url: /gpui-component/docs/components/notification.md
description: >-
  Display toast notifications that appear at the top right of the window with
  auto-dismiss functionality.
---

**Examples:**

Example 1 (rust):
```rust
use gpui_component::{
    menu::{PopupMenu, PopupMenuItem, ContextMenuExt, DropdownMenu},
    Button
};
use gpui::{actions, Action};
```

Example 2 (rust):
```rust
use gpui_component::menu::ContextMenuExt;

div()
    .child("Right click me")
    .context_menu(|menu, window, cx| {
        menu.menu("Copy", Box::new(Copy))
            .menu("Paste", Box::new(Paste))
            .separator()
            .menu("Delete", Box::new(Delete))
    })
```

Example 3 (rust):
```rust
use gpui_component::popup_menu::{PopupMenuExt as _, PopupMenuItem};

let view = cx.entity();
Button::new("menu-btn")
    .label("Open Menu")
    .dropdown_menu(|menu, window, cx| {
        menu.menu("New File", Box::new(NewFile))
            .menu("Open File", Box::new(OpenFile))
            .link("Documentation", "https://longbridge.github.io/gpui-component/")
            .separator()
            .item(PopupMenuItem::new("Custom Action")
                .on_click(window.listener_for(&view, |this, _, window, cx| {
                    // Custom action logic here
                    this.
                })
            )
            .separator()
            .menu("Exit", Box::new(Exit))
    })
```

Example 4 (rust):
```rust
use gpui::Corner;

Button::new("menu-btn")
    .label("Options")
    .dropdown_menu_with_anchor(Corner::TopRight, |menu, window, cx| {
        menu.menu("Option 1", Box::new(Action1))
            .menu("Option 2", Box::new(Action2))
    })
```

---

## Select

**URL:** llms-txt#select

**Contents:**
- Import
- Usage
  - Basic Select
  - Placeholder
  - Searchable
  - Impl SelectItem
  - Group Items
  - Sizes
  - Disabled State
  - Cleanable

:::info
This component was named `Dropdown` in `<= 0.3.x`.

It has been renamed to `Select` to better reflect its purpose.
:::

A select component that allows users to choose from a list of options.

Supports search functionality, grouped items, custom rendering, and various states. Built with keyboard navigation and accessibility in mind.

You can create a basic select dropdown by initializing a `SelectState` with a list of items.

The first type parameter of `SelectState` is the items for the state, which must implement the [SelectItem] trait.

The built-in implementations of `SelectItem` include common types like `String`, `SharedString`, and `&'static str`.

Use `searchable(true)` to enable search functionality within the dropdown.

By default, we have implmemented `SelectItem` for common types like `String`, `SharedString` and `&'static str`. You can also create your own item types by implementing the `SelectItem` trait.

This is useful when you want to display complex data structures, and also want get that data type from `select_value` method.

You can also customize the search logic by overriding the `matches` method.

### Custom Appearance

### Language Selector

### Country/Region Selector

### Integrated with Input Field

### Multi-level Grouped Select

## Keyboard Shortcuts

| Key       | Action                                  |
| --------- | --------------------------------------- |
| `Tab`     | Focus dropdown                          |
| `Enter`   | Open menu or select current item        |
| `Up/Down` | Navigate options (opens menu if closed) |
| `Escape`  | Close menu                              |
| `Space`   | Open menu                               |

The dropdown respects the current theme and uses the following theme tokens:

* `background` - Dropdown input background
* `input` - Border color
* `foreground` - Text color
* `muted_foreground` - Placeholder and disabled text
* `accent` - Selected item background
* `accent_foreground` - Placeholder text color
* `border` - Menu border
* `radius` - Border radius

[SelectItem]: https://docs.rs/gpui-component/latest/gpui_component/select/trait.SelectItem.html

---
url: /gpui-component/docs/components/settings.md
description: A settings UI with grouped setting items and pages.
---

**Examples:**

Example 1 (rust):
```rust
use gpui_component::select::{
    Select, SelectState, SelectItem, SelectDelegate,
    SelectEvent, SearchableVec, SelectGroup
};
```

Example 2 (rust):
```rust
let state = cx.new(|cx| {
    SelectState::new(
        vec!["Apple", "Orange", "Banana"],
        Some(IndexPath::default()), // Select first item
        window,
        cx,
    )
});

Select::new(&state)
```

Example 3 (rust):
```rust
let state = cx.new(|cx| {
    SelectState::new(
        vec!["Rust", "Go", "JavaScript"],
        None, // No initial selection
        window,
        cx,
    )
});

Select::new(&state)
    .placeholder("Select a language...")
```

Example 4 (rust):
```rust
let fruits = SearchableVec::new(vec![
    "Apple", "Orange", "Banana", "Grape", "Pineapple",
]);

let state = cx.new(|cx| {
    SelectState::new(fruits, None, window, cx).searchable(true)
});

Select::new(&state)
    .icon(IconName::Search) // Shows search icon
```

---

## Settings

**URL:** llms-txt#settings

**Contents:**
- Import
- Usage
  - Build a settings
  - Basic Settings
  - With Multiple Pages
  - Group Variants
- Setting Page
  - Basic Page
  - Multiple Groups
  - Default Open

The Settings component provides a UI for managing application settings. It includes grouped setting items and pages.
We can search by title and description to filter the settings to display only relevant settings (Like this macOS, iOS Settings).

Here we have components that can be used to build a settings page.

* \[Settings] - The main settings component that holds multiple setting pages.
* \[SettingPage] - A page of related setting groups.
* \[SettingGroup] - A group of related setting items based on \[GroupBox] style.
* \[SettingItem] - A single setting item with title, description, and field.
* \[SettingField] - Provide different field types like Input, Dropdown, Switch, etc.

The layout of the settings is like this:

### With Multiple Pages

:::info
When you want default expland a page, you can use `default_open(true)` on the \[SettingPage].
:::

Enable reset functionality for a page:

### Custom Item with a render closure

You can create a fully custom setting item using `SettingItem::render`:

By default, setting items use horizontal layout. Use `layout(Axis::Vertical)` for vertical layout:

### With Markdown Description

The \[SettingField] enum provides different field types for various input needs.

The switch field represents a `boolean` on/off state.

Like the switch, but uses a checkbox UI.

Display a single line text input.

A dropdown with a list of options.

### Custom Field by Render Closure

The `SettingField::render` method allows you to create a custom field using a closure that returns an element.

### Custom Field Element

You may have a complex field that you want to reuse, you may want split the element into a separate struct to do the complex logic.

In this case, the \[SettingFieldElement] trait can help you to create a custom field element.

Then use it in the setting item:

- [Settings]
- [SettingPage]
- [SettingGroup]
- [SettingItem]
- [SettingField]
- [NumberFieldOptions]

Implements [Sizable] trait:

- `xsmall()` - Extra small size
- `small()` - Small size
- `medium()` - Medium size (default)
- `large()` - Large size
- `with_size(Size)` - Set specific size

### Complete Settings Example

[Settings]: https://docs.rs/gpui-component/latest/gpui_component/setting/struct.Settings.html
[SettingPage]: https://docs.rs/gpui-component/latest/gpui_component/setting/struct.SettingPage.html
[SettingGroup]: https://docs.rs/gpui-component/latest/gpui_component/setting/struct.SettingGroup.html
[SettingItem]: https://docs.rs/gpui-component/latest/gpui_component/setting/struct.SettingItem.html
[SettingField]: https://docs.rs/gpui-component/latest/gpui_component/setting/enum.SettingField.html
[SettingFieldElement]: https://docs.rs/gpui-component/latest/gpui_component/setting/trait.SettingFieldElement.html
[NumberFieldOptions]: https://docs.rs/gpui-component/latest/gpui_component/setting/struct.NumberFieldOptions.html
[GroupBox]: ./group-box.md
[Sizable]: https://docs.rs/gpui-component/latest/gpui_component/trait.Sizable.html
````

---
url: /gpui-component/docs/components/sheet.md
description: >-
  A sliding panel that appears from the edges of the screen for displaying
  content.
---

**Examples:**

Example 1 (rust):
```rust
use gpui_component::setting::{Settings, SettingPage, SettingGroup, SettingItem, SettingField};
```

Example 2 (unknown):
```unknown
Settings
  SettingPage
    SettingGroup
      SettingItem
        Title
        Description (optional)
        SettingField
```

Example 3 (rust):
```rust
use gpui_component::setting::{Settings, SettingPage, SettingGroup, SettingItem, SettingField};

Settings::new("my-settings")
    .pages(vec![
        SettingPage::new("General")
            .group(
                SettingGroup::new()
                    .title("Basic Options")
                    .item(
                        SettingItem::new(
                            "Enable Feature",
                            SettingField::switch(
                                |cx: &App| true,
                                |val: bool, cx: &mut App| {
                                    println!("Feature enabled: {}", val);
                                },
                            )
                        )
                    )
            )
    ])
```

Example 4 (rust):
```rust
Settings::new("app-settings")
    .pages(vec![
        SettingPage::new("General")
            .default_open(true)
            .group(SettingGroup::new().title("Appearance").items(vec![...])),
        SettingPage::new("Software Update")
            .group(SettingGroup::new().title("Updates").items(vec![...])),
        SettingPage::new("About")
            .group(SettingGroup::new().items(vec![...])),
    ])
```

---

## Badge

**URL:** llms-txt#badge

**Contents:**
- Import
- Usage
  - Badge with Count
  - Variants
  - Badge Sizes
  - Badge Colors
  - Badge on Icons
  - Badge on Avatars
  - Complex Nested Badges
- API Reference

A versatile badge component that can display counts, dots, or icons on elements. Perfect for indicating notifications, status, or other contextual information on avatars, icons, or other UI elements.

Use `count` to display a numeric badge, if the count is greater than zero (`> 0`) the badge will be shown, otherwise it will be hidden.

There is a default maximum count of `99`, any count above this will be displayed as `99+`. You can customize this maximum using the [max](https://docs.rs/gpui-component/latest/gpui_component/badge/struct.Badge.html#method.max) method.

* Default: Displays a numeric count.
* Dot: A small dot indicator, typically used for status.
* Icon: Displays an icon instead of a number.

The Badge is also implemented with the [Sizable] trait, allowing you to set small, medium (default), or large sizes.

### Complex Nested Badges

### Notification Indicators

### Status Indicators

### Different Badge Positions

[Badge]: https://docs.rs/gpui_component/latest/gpui_component/badge/struct.Badge.html

[Sizable]: https://docs.rs/gpui-component/latest/gpui_component/trait.Sizable.html

---
url: /gpui-component/docs/components/button.md
description: Displays a button or a component that looks like a button.
---

**Examples:**

Example 1 (rust):
```rust
use gpui_component::badge::Badge;
```

Example 2 (rust):
```rust
Badge::new()
    .count(3)
    .child(Icon::new(IconName::Bell))
```

Example 3 (rust):
```rust
// Number badge (default)
Badge::new()
    .count(5)
    .child(Avatar::new().src("https://example.com/avatar.jpg"))

// Dot badge
Badge::new()
    .dot()
    .child(Icon::new(IconName::Inbox))

// Icon badge
Badge::new()
    .icon(IconName::Check)
    .child(Avatar::new().src("https://example.com/avatar.jpg"))
```

Example 4 (rust):
```rust
// Small badge
Badge::new()
    .small()
    .count(1)
    .child(Avatar::new().small())

// Medium badge (default)
Badge::new()
    .count(5)
    .child(Avatar::new())

// Large badge
Badge::new()
    .large()
    .count(10)
    .child(Avatar::new().large())
```

---

## NumberInput

**URL:** llms-txt#numberinput

**Contents:**
- Import
- Usage
  - Basic Number Input
  - With Min/Max Validation
  - With Number Formatting
  - Different Sizes
  - With Prefix and Suffix
  - Disabled State
  - Without Default Styling
  - Handle Number Input Events

A specialized input component for numeric values with built-in increment/decrement buttons and support for min/max values, step values, and number formatting with thousands separators.

### Basic Number Input

### With Min/Max Validation

### With Number Formatting

### With Prefix and Suffix

### Without Default Styling

### Handle Number Input Events

### Programmatic Control

| Method                         | Description                                |
| ------------------------------ | ------------------------------------------ |
| `new(state)`                   | Create number input with InputState entity |
| `placeholder(str)`             | Set placeholder text                       |
| `size(size)`                   | Set input size (small, medium, large)      |
| `prefix(el)`                   | Add prefix element                         |
| `suffix(el)`                   | Add suffix element                         |
| `appearance(bool)`             | Enable/disable default styling             |
| `disabled(bool)`               | Set disabled state                         |
| `increment(state, window, cx)` | Increment value programmatically           |
| `decrement(state, window, cx)` | Decrement value programmatically           |

| Event              | Description                        |
| ------------------ | ---------------------------------- |
| `Step(StepAction)` | Increment/decrement button pressed |

| Action      | Description               |
| ----------- | ------------------------- |
| `Increment` | Value should be increased |
| `Decrement` | Value should be decreased |

### InputState (Number-specific methods)

| Method                              | Description                                             |
| ----------------------------------- | ------------------------------------------------------- |
| `pattern(regex)`                    | Set regex pattern for validation (e.g., digits only)    |
| `mask_pattern(MaskPattern::Number)` | Set number formatting with separator and decimal places |
| `value()`                           | Get current display value (formatted)                   |
| `unmask_value()`                    | Get actual numeric value (unformatted)                  |

### MaskPattern::Number

| Field       | Type            | Description                            |
| ----------- | --------------- | -------------------------------------- |
| `separator` | `Option<char>`  | Thousands separator (e.g., ',' or ' ') |
| `fraction`  | `Option<usize>` | Number of decimal places               |

## Keyboard Navigation

| Key         | Action                     |
| ----------- | -------------------------- |
| `↑`         | Increment value            |
| `↓`         | Decrement value            |
| `Tab`       | Navigate to next field     |
| `Shift+Tab` | Navigate to previous field |
| `Enter`     | Submit/confirm value       |
| `Escape`    | Clear input (if enabled)   |

### Quantity Selector with Limits

### Floating Point Input

1. **Validation**: Always validate numeric input on both client and server side
2. **Range Limits**: Implement min/max validation for user safety
3. **Step Size**: Choose appropriate step values for your use case
4. **Error Handling**: Provide clear feedback for invalid input
5. **Formatting**: Use consistent number formatting across your application
6. **Performance**: Debounce rapid increment/decrement actions if needed
7. **Accessibility**: Always provide proper labels and descriptions

---
url: /gpui-component/docs/components/otp-input.md
description: >-
  One-time password input component with multiple fields, auto-focus, and paste
  handling.
---

**Examples:**

Example 1 (rust):
```rust
use gpui_component::input::{InputState, NumberInput, NumberInputEvent, StepAction};
```

Example 2 (rust):
```rust
let number_input = cx.new(|cx|
    InputState::new(window, cx)
        .placeholder("Enter number")
        .default_value("1")
);

NumberInput::new(&number_input)
```

Example 3 (rust):
```rust
// Integer input with validation
let integer_input = cx.new(|cx|
    InputState::new(window, cx)
        .placeholder("Integer value")
        .pattern(Regex::new(r"^\d+$").unwrap()) // Only positive integers
);

NumberInput::new(&integer_input)
```

Example 4 (rust):
```rust
use gpui_component::input::MaskPattern;

// Currency input with thousands separator
let currency_input = cx.new(|cx|
    InputState::new(window, cx)
        .placeholder("Amount")
        .mask_pattern(MaskPattern::Number {
            separator: Some(','),
            fraction: Some(2), // 2 decimal places
        })
);

NumberInput::new(&currency_input)
```

---

## Collapsible

**URL:** llms-txt#collapsible

**Contents:**
- Import
- Usage
  - Basic Use

An interactive element which expands/collapses.

We can use `open` method to control the collapsed state. If false, the `content` method added child elements will be hidden.

[Collapsible]: https://docs.rs/gpui-component/latest/gpui_component/collapsible/struct.Collapsible.html

---
url: /gpui-component/docs/components/color-picker.md
description: >-
  A comprehensive color selection interface with support for multiple color
  formats, presets, and alpha channel.
---

**Examples:**

Example 1 (rust):
```rust
use gpui_component::collapsible::Collapsible;
```

Example 2 (rust):
```rust
Collapsible::new()
    .max_w_128()
    .gap_1()
    .open(self.open)
    .child(
        "This is a collapsible component. \
        Click the header to expand or collapse the content.",
    )
    .content(
        "This is the full content of the Collapsible component. \
        It is only visible when the component is expanded. \n\
        You can put any content you like here, including text, images, \
        or other UI elements.",
    )
    .child(
        h_flex().justify_center().child(
            Button::new("toggle1")
                .icon(IconName::ChevronDown)
                .label("Show more")
                .when(open, |this| {
                    this.icon(IconName::ChevronUp).label("Show less")
                })
                .xsmall()
                .link()
                .on_click({
                    cx.listener(move |this, _, _, cx| {
                        this.open = !this.open;
                        cx.notify();
                    })
                }),
        ),
    )
```

---

## Installation

**URL:** llms-txt#installation

**Contents:**
- System Requirements
  - macOS
- Windows
- Linux
- Rust and Cargo

Before you start to build your application with `gpui-component`, you need to install the library.

## System Requirements

We can development application on macOS, Windows or Linux.

* macOS 15 or later
* Xcode command line tools

* Windows 10 or later

There have a bootstrap script to help install the required toolchain and dependencies.

You can run the script in PowerShell:

Run `./script/bootstrap` to install system dependencies.

We use Rust programming language to build the `gpui-component` library. Make sure you have Rust and Cargo installed on your system.

* Rust 1.90 or later
* Cargo (comes with Rust)

To install the `gpui-component` library, you can use Cargo, the Rust package manager. Add the following line to your `Cargo.toml` file under the `[dependencies]` section:

---
url: /gpui-component/docs.md
description: >-
  Rust GUI components for building fantastic cross-platform desktop application
  by using GPUI.
---

**Examples:**

Example 1 (ps):
```ps
.\script\install-window.ps1
```

---

## OtpInput

**URL:** llms-txt#otpinput

**Contents:**
- Import
- Usage
  - Basic OTP Input
  - With Default Value
  - Masked OTP Input
  - Different Sizes
  - Grouped Layout
  - Disabled State
  - Different Length Codes
  - Handle OTP Events

A specialized input component for one-time passwords (OTP) that displays multiple input fields in a grid layout. Perfect for SMS verification codes, authenticator app codes, and other numeric verification scenarios.

### With Default Value

### Different Length Codes

### Handle OTP Events

### Programmatic Control

| Method                         | Description                                  |
| ------------------------------ | -------------------------------------------- |
| `new(length, window, cx)`      | Create a new OTP state with specified length |
| `default_value(str)`           | Set initial value                            |
| `masked(bool)`                 | Enable masked display (shows asterisks)      |
| `set_value(str, window, cx)`   | Set OTP value programmatically               |
| `value()`                      | Get current OTP value                        |
| `set_masked(bool, window, cx)` | Toggle masked display                        |
| `focus(window, cx)`            | Focus the OTP input                          |
| `focus_handle(cx)`             | Get focus handle                             |

| Method           | Description                              |
| ---------------- | ---------------------------------------- |
| `new(state)`     | Create OTP input with state entity       |
| `groups(n)`      | Set number of visual groups (default: 2) |
| `disabled(bool)` | Set disabled state                       |
| `small()`        | Small size (6x6 px fields)               |
| `large()`        | Large size (11x11 px fields)             |
| `with_size(px)`  | Custom field size                        |

| Event    | Description                                       |
| -------- | ------------------------------------------------- |
| `Change` | Emitted when OTP is complete (all digits entered) |
| `Focus`  | Input received focus                              |
| `Blur`   | Input lost focus                                  |

### Two-Factor Authentication

* **Numeric Only**: Accepts only digits (0-9)
* **Auto-Focus**: Automatically moves to next field when digit is entered
* **Backspace**: Removes current digit and moves to previous field
* **Length Limit**: Prevents input beyond specified length
* **Auto-Complete**: Emits `Change` event when all fields are filled

* **Focus Indicator**: Blue border and blinking cursor on active field
* **Masking**: Shows asterisk icons instead of numbers when enabled
* **Grouping**: Visual separation of fields into groups for better readability
* **Disabled State**: Grayed out appearance when disabled

### Keyboard Navigation

* **Arrow Keys**: Navigate between fields
* **Tab**: Move to next focusable element
* **Shift+Tab**: Move to previous focusable element
* **Backspace**: Delete current digit and move backward
* **Delete**: Clear current field

### Auto-Submit on Complete

### Resend Code Timer

---
url: /gpui-component/docs/components/plot.md
description: >-
  A low-level plotting library for creating custom charts and data
  visualizations.
---

**Examples:**

Example 1 (rust):
```rust
use gpui_component::input::{OtpInput, OtpState};
```

Example 2 (rust):
```rust
let otp_state = cx.new(|cx| OtpState::new(6, window, cx));

OtpInput::new(&otp_state)
```

Example 3 (rust):
```rust
let otp_state = cx.new(|cx|
    OtpState::new(6, window, cx)
        .default_value("123456")
);

OtpInput::new(&otp_state)
```

Example 4 (rust):
```rust
let otp_state = cx.new(|cx|
    OtpState::new(6, window, cx)
        .masked(true)
        .default_value("123456")
);

OtpInput::new(&otp_state)
```

---

## Components

**URL:** llms-txt#components

**Contents:**
  - Basic Components
  - Form Components
  - Layout Components
  - Advanced Components

* [Accordion](accordion) - Collapsible content panels
* [Alert](alert) - Alert messages with different variants
* [Avatar](avatar) - User avatars with fallback text
* [Badge](badge) - Count badges and indicators
* [Button](button) - Interactive buttons with multiple variants
* [Checkbox](checkbox) - Binary selection control
* [Collapsible](collapsible) - Expandable/collapsible content
* [DropdownButton](dropdown_button) - Button with dropdown menu
* [Icon](icon) - Icon display component
* [Image](image) - Image display with fallbacks
* [Kbd](kbd) - Keyboard shortcut display
* [Label](label) - Text labels for form elements
* [Progress](progress) - Progress bars
* [Radio](radio) - Single selection from multiple options
* [Skeleton](skeleton) - Loading placeholders
* [Slider](slider) - Value selection from a range
* [Spinner](spinner) - Loading and status spinners
* [Stepper](stepper) - Step-by-step progress indicator
* [Switch](switch) - Toggle on/off control
* [Tag](tag) - Labels and categories
* [Toggle](toggle) - Toggle button states
* [Tooltip](tooltip) - Helpful hints on hover

* [Input](input) - An input field or a component that looks like an input field.
* [Select](select) - A list of options for the user to pick.
* [NumberInput](number-input) - Numeric input with increment/decrement
* [DatePicker](date-picker) - Date selection with calendar
* [OtpInput](otp-input) - One-time password input
* [ColorPicker](color-picker) - Color selection interface
* [Editor](editor) - Multi-line text editor and code editor
* [Form](form) - Form container and layout

### Layout Components

* [DescriptionList](description-list) - Key-value pair display
* [GroupBox](group-box) - Grouped content with borders
* [Dialog](dialog) - Dialog and modal windows
* [Notification](notification) - Toast notifications
* [Popover](popover) - Floating content display
* [Resizable](resizable) - Resizable panels and containers
* [Scrollable](scrollable) - Scrollable containers
* [Sheet](sheet) - Slide-in panel from edges
* [Sidebar](sidebar) - Navigation sidebar

### Advanced Components

* [Calendar](calendar) - Calendar display and navigation
* [Chart](chart) - Data visualization charts (Line, Bar, Area, Pie, Candlestick)
* [List](list) - List display with items
* [Menu](menu) - Menu and context menu and dropdown menu.
* [Settings](settings) - Settings UI
* [Table](table) - High-performance data tables
* [Tabs](tabs) - Tabbed interface
* [Tree](tree) - Hierarchical tree data display
* [VirtualList](virtual-list) - Virtualized list for large datasets

---
url: /gpui-component/docs/context.md
description: Learn about the Window and Context in GPUI.
---

The [Window], [App], [Context] and [Entity] are most important things in GPUI, it appears everywhere.

* [Window] - The current window instance, which for handle the **Window Level** things.
* [App] - The current application instance, which for handle the **Application Level** things.
* [Context] - The Entity Context instance, which for handle the **Context Level** things.
* [Entity] - The Entity instance, which for handle the **Entity Level** things.

:::info
As you can see, we always use `cx` to represent `App` and `Context<Self>`,
which is the standard naming convention for GPUI,
we can follow this convention to make our code more readable and maintainable.
:::

[Window]: https://docs.rs/gpui/latest/gpui/struct.Window.html

[App]: https://docs.rs/gpui/latest/gpui/struct.App.html

[Context]: https://docs.rs/gpui/latest/gpui/struct.Context.html

[Entity]: https://docs.rs/gpui/latest/gpui/struct.Entity.html

---
url: /gpui-component/docs/components/date-picker.md
description: >-
  A date picker component for selecting single dates or date ranges with
  calendar interface.
---

**Examples:**

Example 1 (rs):
```rs
fn new(window: &mut Window, cx: &mut App) {}

impl RenderOnce for MyElement {
    fn render(self, window: &mut Window, cx: &mut App) {}
}

impl Render for MyView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) {}
}
```

---

## Dialog

**URL:** llms-txt#dialog

**Contents:**
- Import
- Usage
  - Setup application root view for display of dialogs
  - Basic Dialog
  - Form Dialog
  - Confirm Dialog
  - Alert Dialog
  - Custom Button Labels
  - Dialog with Icon
  - Scrollable Dialog

Dialog component for creating dialogs, confirmations, and alerts. Supports overlay, keyboard shortcuts, and various customizations.

### Setup application root view for display of dialogs

You need to set up your application's root view to render the dialog layer. This is typically done in your main application struct's render method.

The [Root::render\_dialog\_layer](https://docs.rs/gpui-component/latest/gpui_component/struct.Root.html#method.render_dialog_layer) function handles rendering any active dialogs on top of your app content.

### Custom Button Labels

### Scrollable Dialog

### Close Dialog Programmatically

The `close_dialog` method can be used to close the active dialog from anywhere within the window context.

### Delete Confirmation

---
url: /gpui-component/docs/components/dropdown_button.md
description: >-
  A DropdownButton is a combination of a button and a trigger button. It allows
  us to display a dropdown menu when the trigger is clicked, but the left Button
  can still respond to independent events.
---

**Examples:**

Example 1 (rust):
```rust
use gpui_component::dialog::DialogButtonProps;
use gpui_component::WindowExt;
```

Example 2 (rust):
```rust
use gpui_component::TitleBar;

struct MyApp {
    view: AnyView,
}

impl Render for MyApp {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let dialog_layer = Root::render_dialog_layer(window, cx);

        div()
            .size_full()
            .child(
                v_flex()
                    .size_full()
                    .child(TitleBar::new())
                    .child(div().flex_1().overflow_hidden().child(self.view.clone())),
            )
            // Render the dialog layer on top of the app content
            .children(dialog_layer)
    }
}
```

Example 3 (rust):
```rust
window.open_dialog(cx, |dialog, _, _| {
    dialog
        .title("Welcome")
        .child("This is a dialog dialog.")
})
```

Example 4 (rust):
```rust
let input = cx.new(|cx| InputState::new(window, cx));

window.open_dialog(cx, |dialog, _, _| {
    dialog
        .title("User Information")
        .child(
            v_flex()
                .gap_3()
                .child("Please enter your details:")
                .child(Input::new(&input))
        )
        .footer(|_, _, _, _| {
            vec![
                Button::new("ok")
                    .primary()
                    .label("Submit")
                    .on_click(|_, window, cx| {
                        window.close_dialog(cx);
                    }),
                Button::new("cancel")
                    .label("Cancel")
                    .on_click(|_, window, cx| {
                        window.close_dialog(cx);
                    }),
            ]
        })
})
```

---

## Plot

**URL:** llms-txt#plot

**Contents:**
- Import
- Scales
  - ScaleLinear
  - ScaleBand
  - ScalePoint
  - ScaleOrdinal
- Shapes
  - Bar
  - Line
  - Area

The `plot` module provides low-level building blocks for creating custom charts. It includes scales, shapes, and utilities that power the high-level `Chart` components.

Scales map a dimension of abstract data to a visual representation.

Maps a continuous quantitative domain to a continuous range.

Maps a discrete domain to a continuous range, useful for bar charts.

Maps a discrete domain to a set of points in a continuous range, useful for scatter plots or line charts with categorical axes.

Maps a discrete domain to a discrete range.

Renders a bar shape, commonly used in bar charts.

Renders a line shape, commonly used in line charts.

Supports rendering dots at data points.

Renders an area shape, commonly used in area charts.

Renders pie charts and donut charts using `Pie` layout and `Arc` shape.

Computes stacked layout for data series.

Renders chart axes with labels and ticks.

### Custom Bar Chart Implementation

Here's how to implement a custom stacked bar chart using low-level plot primitives:

---
url: /gpui-component/docs/components/popover.md
description: A floating overlay that displays rich content relative to a trigger element.
---

**Examples:**

Example 1 (rust):
```rust
use gpui_component::plot::{
    scale::{Scale, ScaleLinear, ScaleBand, ScalePoint, ScaleOrdinal},
    shape::{Bar, Stack, Line, Area, Pie, Arc},
    PlotAxis, AxisText
};
```

Example 2 (rust):
```rust
let scale = ScaleLinear::new(
    vec![0., 100.],   // Domain (data values)
    vec![0., 500.]    // Range (pixel coordinates)
);

scale.tick(&50.); // Returns pixel position
```

Example 3 (rust):
```rust
let scale = ScaleBand::new(
    vec!["A", "B", "C"], // Domain
    vec![0., 300.]       // Range
)
.padding_inner(0.1)
.padding_outer(0.1);

scale.band_width(); // Returns width of each band
scale.tick(&"A");   // Returns start position of band "A"
```

Example 4 (rust):
```rust
let scale = ScalePoint::new(
    vec!["A", "B", "C"], // Domain
    vec![0., 300.]       // Range
);

scale.tick(&"A"); // Returns position of point "A"
```

---

## Avatar

**URL:** llms-txt#avatar

**Contents:**
- Import
- Usage
  - Basic Avatar
  - Avatar with Fallback Text
  - Avatar Placeholder
  - Avatar Sizes
  - Custom Styling
- AvatarGroup
  - Basic Group
  - Group with Limit

The Avatar component displays user profile images with intelligent fallbacks. When no image is provided, it shows user initials or a placeholder icon. The component supports various sizes and can be grouped together for team displays.

You can create an [Avatar] by providing an image source URL and a user name:

### Avatar with Fallback Text

When no image source is provided, the Avatar displays user initials with an automatically generated color background:

### Avatar Placeholder

For anonymous users or when no name is provided:

The [AvatarGroup] component allows you to display multiple avatars in a compact, overlapping layout:

### Group with Ellipsis

Show an ellipsis indicator when avatars are hidden due to the limit.

In this example, only 3 avatars are shown, and "..." indicates there are more:

The \[Sizeable] trait can also be applied to AvatarGroup, and it will set the size for all contained avatars.

### Adding Multiple Avatars

* [Avatar]
* [AvatarGroup]

### User Profile Header

### Avatar with Custom Colors

[Avatar]: https://docs.rs/gpui-component/latest/gpui_component/avatar/struct.Avatar.html

[AvatarGroup]: https://docs.rs/gpui-component/latest/gpui_component/avatar/struct.AvatarGroup.html

[Sizable]: https://docs.rs/gpui-component/latest/gpui_component/trait.Sizable.html

---
url: /gpui-component/docs/components/badge.md
description: >-
  A red dot that indicates the number of unread messages, status, or other
  notifications.
---

**Examples:**

Example 1 (rust):
```rust
use gpui_component::avatar::{Avatar, AvatarGroup};
```

Example 2 (rust):
```rust
Avatar::new()
    .name("John Doe")
    .src("https://example.com/avatar.jpg")
```

Example 3 (rust):
```rust
// Shows "JD" initials with colored background
Avatar::new()
    .name("John Doe")

// Shows "JS" initials
Avatar::new()
    .name("Jane Smith")
```

Example 4 (rust):
```rust
use gpui_component::IconName;

// Default user icon placeholder
Avatar::new()

// Custom placeholder icon
Avatar::new()
    .placeholder(IconName::Building2)
```

---

## VirtualList

**URL:** llms-txt#virtuallist

**Contents:**
- Import
- Usage
  - Basic Vertical Virtual List
  - Horizontal Virtual List
  - Variable Item Sizes
  - Table-like Layout with Multiple Columns
- Scroll Handling
  - Basic Scroll Control
  - Programmatic Scrolling
  - Both Axis Scrolling

VirtualList is a high-performance component designed for efficiently rendering large datasets by only rendering visible items. Unlike uniform lists, VirtualList supports variable item sizes, making it perfect for complex layouts like tables with different row heights or dynamic content.

### Basic Vertical Virtual List

### Horizontal Virtual List

### Variable Item Sizes

VirtualList excels at handling items with different sizes:

### Table-like Layout with Multiple Columns

VirtualList can render complex layouts like tables:

### Basic Scroll Control

### Programmatic Scrolling

### Both Axis Scrolling

For content that scrolls in both directions:

## Performance Optimization

### Efficient Item Rendering

Only visible items are rendered, making VirtualList highly performant:

### Memory Management

VirtualList automatically manages memory by:

* Only rendering visible items
* Reusing rendered elements when scrolling
* Calculating precise visible ranges

### Variable Heights with Caching

For dynamic content with calculated heights:

### File Explorer with Virtual Scrolling

### Chat Messages with Auto-scroll

### Data Grid with Fixed Headers

1. **Item Sizing**: Pre-calculate item sizes when possible for best performance
2. **Memory Management**: Use VirtualList for any list with >50 items
3. **Scroll Performance**: Avoid heavy computations in render functions
4. **State Management**: Keep item state separate from rendering logic
5. **Error Handling**: Handle edge cases like empty lists gracefully
6. **Testing**: Test with various data sizes and scroll positions

1. **Pre-calculate Sizes**: Calculate item sizes upfront rather than during render
2. **Minimize Re-renders**: Use stable item keys and avoid recreating render functions
3. **Batch Updates**: Group multiple data changes together
4. **Efficient Rendering**: Keep item render functions lightweight
5. **Memory Monitoring**: Monitor memory usage with very large datasets

**Examples:**

Example 1 (rust):
```rust
use gpui_component::{
    v_virtual_list, h_virtual_list, VirtualListScrollHandle,
    scroll::{Scrollbar, ScrollbarState, ScrollbarAxis},
};
use std::rc::Rc;
use gpui::{px, size, ScrollStrategy, Size, Pixels};
```

Example 2 (rust):
```rust
use std::rc::Rc;
use gpui::{px, size, Size, Pixels};

pub struct ListViewExample {
    items: Vec<String>,
    item_sizes: Rc<Vec<Size<Pixels>>>,
    scroll_handle: VirtualListScrollHandle,
}

impl ListViewExample {
    fn new(cx: &mut Context<Self>) -> Self {
        let items = (0..5000).map(|i| format!("Item {}", i)).collect::<Vec<_>>();
        let item_sizes = Rc::new(items.iter().map(|_| size(px(200.), px(30.))).collect());

        Self {
            items,
            item_sizes,
            scroll_handle: VirtualListScrollHandle::new(),
        }
    }
}

impl Render for ListViewExample {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_virtual_list(
            cx.entity().clone(),
            "my-list",
            self.item_sizes.clone(),
            |view, visible_range, _, cx| {
                visible_range
                    .map(|ix| {
                        div()
                            .h(px(30.))
                            .w_full()
                            .bg(cx.theme().secondary)
                            .child(format!("Item {}", ix))
                    })
                    .collect()
            },
        )
        .track_scroll(&self.scroll_handle)
    }
}
```

Example 3 (rust):
```rust
h_virtual_list(
    cx.entity().clone(),
    "horizontal-list",
    item_sizes.clone(),
    |view, visible_range, _, cx| {
        visible_range
            .map(|ix| {
                div()
                    .w(px(120.))  // Width is used for horizontal lists
                    .h_full()
                    .bg(cx.theme().accent)
                    .child(format!("Card {}", ix))
            })
            .collect()
    },
)
.track_scroll(&scroll_handle)
```

Example 4 (rust):
```rust
let item_sizes = Rc::new(
    (0..1000)
        .map(|i| {
            // Different heights based on index
            let height = if i % 5 == 0 {
                px(60.)  // Header items are taller
            } else if i % 3 == 0 {
                px(45.)  // Some items are medium
            } else {
                px(30.)  // Regular items
            };
            size(px(300.), height)
        })
        .collect::<Vec<_>>()
);

v_virtual_list(
    cx.entity().clone(),
    "variable-list",
    item_sizes.clone(),
    |view, visible_range, _, cx| {
        visible_range
            .map(|ix| {
                let content = if ix % 5 == 0 {
                    format!("Header {}", ix / 5)
                } else {
                    format!("Item {}", ix)
                };

                let bg_color = if ix % 5 == 0 {
                    cx.theme().accent
                } else {
                    cx.theme().secondary
                };

                div()
                    .w_full()
                    .h(item_sizes[ix].height)
                    .bg(bg_color)
                    .flex()
                    .items_center()
                    .px_4()
                    .child(content)
            })
            .collect()
    },
)
```

---

## Checkbox

**URL:** llms-txt#checkbox

**Contents:**
- Import
- Usage
  - Basic Checkbox
  - Controlled Checkbox
  - Different Sizes
  - Disabled State
  - Without Label
  - Custom Tab Order
- API Reference
  - Styling

A checkbox component for binary choices. Supports labels, disabled state, and different sizes.

The `on_click` callback is triggered when the user toggles the checkbox, receiving the **new checked state**.

### Controlled Checkbox

Implements `Sizable` and `Disableable` traits:

* `text_xs()` - Extra small text
* `text_sm()` - Small text
* `text_base()` - Base text (default)
* `text_lg()` - Large text
* `disabled(bool)` - Disabled state

[Checkbox]: https://docs.rs/gpui-component/latest/gpui_component/checkbox/struct.Checkbox.html

---
url: /gpui-component/docs/components/clipboard.md
description: >-
  A button component that helps you copy text or other content to your
  clipboard.
---

**Examples:**

Example 1 (rust):
```rust
use gpui_component::checkbox::Checkbox;
```

Example 2 (rust):
```rust
Checkbox::new("my-checkbox")
    .label("Accept terms and conditions")
    .checked(false)
    .on_click(|checked, _, _| {
        println!("Checkbox is now: {}", checked);
    })
```

Example 3 (rust):
```rust
struct MyView {
    is_checked: bool,
}

impl Render for MyView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        Checkbox::new("checkbox")
            .label("Option")
            .checked(self.is_checked)
            .on_click(cx.listener(|view, checked, _, cx| {
                view.is_checked = *checked;
                cx.notify();
            }))
    }
}
```

Example 4 (rust):
```rust
Checkbox::new("cb").text_xs().label("Extra Small")
Checkbox::new("cb").text_sm().label("Small")
Checkbox::new("cb").label("Medium") // default
Checkbox::new("cb").text_lg().label("Large")
```

---

## DatePicker

**URL:** llms-txt#datepicker

**Contents:**
- Import
- Usage
  - Basic Date Picker
  - With Initial Date
  - Date Range Picker
  - With Custom Date Format
  - With Placeholder
  - Cleanable Date Picker
  - Different Sizes
  - Disabled State

A flexible date picker component with calendar interface that supports single date selection, date range selection, custom date formatting, disabled dates, and preset ranges.

### Basic Date Picker

### With Initial Date

### Date Range Picker

### With Custom Date Format

### Cleanable Date Picker

### Custom Appearance

### Disabled Weekends

### Disabled Date Range

### Disabled Date Interval

### Custom Disabled Dates

### Single Date Presets

### Date Range Presets

## Handle Date Selection Events

## Multiple Months Display

### Business Days Only

### Date Range with Max Duration

### Event Date Picker

### Booking System Date Range

### Financial Period Selector

---
url: /gpui-component/docs/components/description-list.md
description: Use to display details with a tidy layout for key-value pairs.
---

**Examples:**

Example 1 (rust):
```rust
use gpui_component::{
    date_picker::{DatePicker, DatePickerState, DateRangePreset, DatePickerEvent},
    calendar::{Date, Matcher},
};
```

Example 2 (rust):
```rust
let date_picker = cx.new(|cx| DatePickerState::new(window, cx));

DatePicker::new(&date_picker)
```

Example 3 (rust):
```rust
use chrono::Local;

let date_picker = cx.new(|cx| {
    let mut picker = DatePickerState::new(window, cx);
    picker.set_date(Local::now().naive_local().date(), window, cx);
    picker
});

DatePicker::new(&date_picker)
```

Example 4 (rust):
```rust
use chrono::{Local, Days};

// Range mode picker
let range_picker = cx.new(|cx| DatePickerState::range(window, cx));

DatePicker::new(&range_picker)
    .number_of_months(2) // Show 2 months for easier range selection

// With initial range
let range_picker = cx.new(|cx| {
    let now = Local::now().naive_local().date();
    let mut picker = DatePickerState::new(window, cx);
    picker.set_date(
        (now, now.checked_add_days(Days::new(7)).unwrap()),
        window,
        cx,
    );
    picker
});

DatePicker::new(&range_picker)
    .number_of_months(2)
```

---

## gpui-component-docs

**URL:** llms-txt#gpui-component-docs

To install dependencies:

This project was created using `bun init` in bun v1.2.23. [Bun](https://bun.com) is a fast all-in-one JavaScript runtime.

---
url: /gpui-component/docs/components/group-box.md
description: >-
  A styled container element with an optional title to group related content
  together.
---

**Examples:**

Example 1 (bash):
```bash
bun install
```

Example 2 (bash):
```bash
bun run dev
```

---

## Scrollable

**URL:** llms-txt#scrollable

**Contents:**
- Import
- Usage
  - Basic Scrollable Container
  - Vertical Scrolling
  - Horizontal Scrolling
  - Both Directions
- Custom Scrollbars
  - Manual Scrollbar Creation
- Virtualization
  - VirtualList for Large Datasets

A comprehensive scrollable container component that provides custom scrollbars, scroll tracking, and virtualization capabilities. Supports both vertical and horizontal scrolling with customizable appearance and behavior.

### Basic Scrollable Container

The simplest way to make any element scrollable is using the `overflow_scrollbar()` method from `ScrollableElement` trait.

This method is almost like the `overflow_scroll()` method, but it adds scrollbars.

* `overflow_scrollbar()` - Adds scrollbars for both axes as needed.
* `overflow_x_scrollbar()` - Adds horizontal scrollbar as needed.
* `overflow_y_scrollbar()` - Adds vertical scrollbar as needed.

### Vertical Scrolling

### Horizontal Scrolling

### Manual Scrollbar Creation

For more control, you can create scrollbars manually:

### VirtualList for Large Datasets

For rendering large lists efficiently, use `VirtualList`:

### Scrolling to Specific Items

### Variable Item Sizes

## Theme Customization

### Scrollbar Appearance

Customize scrollbar appearance through theme configuration:

### Scrollbar Show Modes

Control when scrollbars are visible:

### System Integration

Sync scrollbar behavior with system preferences:

### File Browser with Scrolling

### Chat Messages with Auto-scroll

### Data Table with Virtual Scrolling

---
url: /gpui-component/docs/components/select.md
description: Displays a list of options for the user to pick from—triggered by a button.
---

**Examples:**

Example 1 (rust):
```rust
use gpui_component::{
    scroll::{ScrollableElement, ScrollbarAxis, ScrollbarShow},
    StyledExt as _,
};
```

Example 2 (rust):
```rust
use gpui::{div, Axis};
use gpui_component::ScrollableElement;

div()
    .id("scrollable-container")
    .size_full()
    .child("Your content here")
    .overflow_scrollbar()
```

Example 3 (rust):
```rust
v_flex()
    .id("scrollable-container")
    .overflow_y_scrollbar()
    .gap_2()
    .p_4()
    .child("Scrollable Content")
    .children((0..100).map(|i| {
        div()
            .h(px(40.))
            .w_full()
            .bg(cx.theme().secondary)
            .child(format!("Item {}", i))
    }))
```

Example 4 (rust):
```rust
h_flex()
    .id("scrollable-container")
    .overflow_x_scrollbar()
    .gap_2()
    .p_4()
    .children((0..50).map(|i| {
        div()
            .min_w(px(120.))
            .h(px(80.))
            .bg(cx.theme().accent)
            .child(format!("Card {}", i))
    }))
```

---

## Progress

**URL:** llms-txt#progress

**Contents:**
- Import
- Usage
  - Basic Progress Bar
  - Different Progress Values
  - Indeterminate State
  - Dynamic Progress Updates
  - File Upload Progress
  - Loading States
  - Multi-Step Process
- Examples

A linear progress bar component that visually represents the completion percentage of a task. The progress bar features smooth animations, customizable colors, and automatic styling that adapts to the current theme.

### Basic Progress Bar

### Different Progress Values

### Indeterminate State

### Dynamic Progress Updates

### File Upload Progress

### Multi-Step Process

### Task Progress with Status

### Download Progress with Speed

### Installation Progress

## Styling and Theming

The Progress component automatically adapts to the current theme:

### Visual Properties

* **Height**: 8px by default
* **Border Radius**: Matches theme radius (up to half the height)
* **Background**: Semi-transparent theme progress bar color (20% opacity)
* **Fill**: Full opacity theme progress bar color
* **Animation**: Smooth transitions when value changes
* **Corners**: Rounded on completion, left-rounded during progress

* Values less than 0 are clamped to 0%
* Values greater than 100 are clamped to 100%
* Progress bar fills from left to right
* Border radius adjusts based on completion state:
  * Partial progress: Left side rounded only
  * Complete progress: Both sides rounded
* Background color is always a semi-transparent version of the fill color
* Height and radius adapt to theme settings automatically

1. **Always provide text indicators** alongside the visual progress bar
2. **Use meaningful labels** to describe what is progressing
3. **Update progress regularly** but not too frequently to avoid performance issues
4. **Consider showing ETA or completion time** for long-running tasks
5. **Provide cancel/pause options** for lengthy operations
6. **Show final status** when progress reaches 100%
7. **Handle error states** gracefully with appropriate messaging

---
url: /gpui-component/docs/components/radio.md
description: >-
  A set of checkable buttons—known as radio buttons—where no more than one of
  the buttons can be checked at a time.
---

**Examples:**

Example 1 (rust):
```rust
use gpui_component::progress::Progress;
```

Example 2 (rust):
```rust
Progress::new()
    .value(50.0) // 50% complete
```

Example 3 (rust):
```rust
// Starting state (0%)
Progress::new()
    .value(0.0)

// Partially complete (25%)
Progress::new()
    .value(25.0)

// Nearly complete (75%)
Progress::new()
    .value(75.0)

// Complete (100%)
Progress::new()
    .value(100.0)
```

Example 4 (rust):
```rust
// For unknown progress duration
Progress::new()
    .value(-1.0) // Any negative value shows as 0%

// Or explicitly set to 0 for starting state
Progress::new()
    .value(0.0)
```

---

## Image

**URL:** llms-txt#image

**Contents:**
- Import
- Usage
  - Basic Image
  - Image with Sizing
  - Object Fit Options
  - Image with Fallback Handling
  - Loading States
  - Responsive Images
  - Image Gallery
  - SVG Images

The Image component provides a robust way to display images with comprehensive fallback handling, loading states, and responsive sizing. Built on GPUI's native image support, it handles various image sources including URLs, local files, and SVG graphics with proper error handling and accessibility features.

### Image with Sizing

### Object Fit Options

Control how images are scaled and positioned within their containers:

### Image with Fallback Handling

### Responsive Images

### Core Image Function

| Function      | Description                           |
| ------------- | ------------------------------------- |
| `img(source)` | Create image element from ImageSource |

### Image Sources (ImageSource)

| Type        | Description            | Example                           |
| ----------- | ---------------------- | --------------------------------- |
| String/\&str | URL or file path       | `"https://example.com/image.jpg"` |
| SharedUri   | Shared URI reference   | `SharedUri::from("file://path")`  |
| Local Path  | Local file system path | `"assets/logo.png"`               |
| Data URI    | Base64 encoded image   | `"data:image/png;base64,..."`     |

| Method          | Description               |
| --------------- | ------------------------- |
| `w(length)`     | Set width                 |
| `h(length)`     | Set height                |
| `size(length)`  | Set both width and height |
| `w_full()`      | Full width of container   |
| `h_full()`      | Full height of container  |
| `size_full()`   | Full size of container    |
| `max_w(length)` | Maximum width             |
| `max_h(length)` | Maximum height            |
| `min_w(length)` | Minimum width             |
| `min_h(length)` | Minimum height            |

### Object Fit Options

| Value                  | Description                       |
| ---------------------- | --------------------------------- |
| `ObjectFit::Cover`     | Scale to fill container, may crop |
| `ObjectFit::Contain`   | Scale to fit within container     |
| `ObjectFit::Fill`      | Stretch to fill container         |
| `ObjectFit::ScaleDown` | Like contain, but never scale up  |
| `ObjectFit::None`      | Original size                     |

| Method                | Description             |
| --------------------- | ----------------------- |
| `rounded(radius)`     | Border radius           |
| `border_1()`          | 1px border              |
| `border_color(color)` | Border color            |
| `opacity(value)`      | Image opacity (0.0-1.0) |
| `shadow_sm()`         | Small shadow            |
| `shadow_lg()`         | Large shadow            |

### Product Image Card

### Avatar with Image

### Image Comparison Slider

### Error Handling Pattern

### Image Optimization

* Use appropriate image dimensions for display size
* Compress images without sacrificing quality
* Consider using modern image formats (WebP, AVIF)
* Implement responsive images for different screen sizes

* Always provide meaningful fallbacks for failed image loads
* Use skeleton loading states to maintain layout stability
* Implement retry mechanisms for temporary network failures
* Provide user feedback for permanent load failures

* Use lazy loading for images not immediately visible
* Implement proper caching strategies
* Consider using placeholder images during loading
* Optimize image sizes for their display context

* Maintain consistent aspect ratios in image grids
* Provide smooth loading transitions
* Use appropriate object-fit values for content type
* Consider providing zoom functionality for detailed images

## Implementation Notes

* Built on GPUI's native image rendering capabilities
* Supports all GPUI ImageSource types automatically
* Inherits GPUI's styling and layout system
* Compatible with GPUI's animation and interaction systems

* Full support for SVG graphics with proper scaling
* SVG images can be styled with text colors for theming
* Vector graphics maintain sharpness at all sizes
* Supports both external SVG files and inline data URIs

### Memory Management

* GPUI handles image caching and memory management automatically
* Large images are efficiently managed by the graphics backend
* No manual memory cleanup required for image components

### Cross-Platform Compatibility

* Consistent behavior across Windows, macOS, and Linux
* Native image format support varies by platform
* Uses platform-optimized rendering where available

---
url: /gpui-component/docs/components/input.md
description: 'Text input component with validation, masking, and various features.'
---

**Examples:**

Example 1 (rust):
```rust
use gpui::{img, ImageSource, ObjectFit};
use gpui_component::{v_flex, h_flex, div, Icon, IconName};
```

Example 2 (rust):
```rust
// Simple image from URL
img("https://example.com/image.jpg")

// Local image file
img("assets/logo.png")

// SVG image
img("icons/star.svg")
```

Example 3 (rust):
```rust
// Fixed dimensions
img("https://example.com/photo.jpg")
    .w(px(300.))
    .h(px(200.))

// Responsive width with aspect ratio
img("https://example.com/banner.jpg")
    .w(relative(1.))  // Full width
    .max_w(px(800.))
    .h(px(400.))

// Square image
img("https://example.com/avatar.jpg")
    .size(px(100.))  // 100x100px
```

Example 4 (rust):
```rust
// Cover - scales to fill container, may crop
img("https://example.com/photo.jpg")
    .w(px(300.))
    .h(px(200.))
    .object_fit(ObjectFit::Cover)

// Contain - scales to fit within container, preserves aspect ratio
img("https://example.com/photo.jpg")
    .w(px(300.))
    .h(px(200.))
    .object_fit(ObjectFit::Contain)

// Fill - stretches to fill container, may distort
img("https://example.com/photo.jpg")
    .w(px(300.))
    .h(px(200.))
    .object_fit(ObjectFit::Fill)

// Scale Down - acts like contain, but never scales up
img("https://example.com/photo.jpg")
    .w(px(300.))
    .h(px(200.))
    .object_fit(ObjectFit::ScaleDown)

// None - original size, may overflow or be smaller than container
img("https://example.com/photo.jpg")
    .w(px(300.))
    .h(px(200.))
    .object_fit(ObjectFit::None)
```

---

## List

**URL:** llms-txt#list

**Contents:**
- Import
- Usage
  - Basic List
  - List with Sections
  - List Items with Icons and Actions
  - List with Search
  - List with Loading State
  - Infinite Scrolling
  - List Events
  - Different Item Styles

A powerful List component that provides a virtualized, searchable list interface with support for sections, headers, footers, selection states, and infinite scrolling. The component is built on a delegate pattern that allows for flexible data management and custom item rendering.

To create a list, you need to implement the `ListDelegate` trait for your data:

Now use \[List] to render list:

### List with Sections

### List Items with Icons and Actions

The list automatically includes a search input by default. Implement `perform_search` to handle queries:

And you should use `searchable(true)` when creating the `ListState` to show search input.

### List with Loading State

### Infinite Scrolling

### Different Item Styles

### Custom Empty State

## Configuration Options

### List Configuration

### Scrolling Control

### File Browser List

### Contact List with Sections

---
url: /gpui-component/docs/components/menu.md
description: >-
  Context menus and popup menus with support for icons, shortcuts, submenus, and
  various menu item types.
---

**Examples:**

Example 1 (rust):
```rust
use gpui_component::list::{List, ListState, ListDelegate, ListItem, ListEvent, ListSeparatorItem};
use gpui_component::IndexPath;
```

Example 2 (rust):
```rust
struct MyListDelegate {
    items: Vec<String>,
    selected_index: Option<IndexPath>,
}

impl ListDelegate for MyListDelegate {
    type Item = ListItem;

    fn items_count(&self, _section: usize, _cx: &App) -> usize {
        self.items.len()
    }

    fn render_item(
        &mut self,
        ix: IndexPath,
        _window: &mut Window,
        _cx: &mut Context<TableState<Self>>,
    ) -> Option<Self::Item> {
        self.items.get(ix.row).map(|item| {
            ListItem::new(ix)
                .child(Label::new(item.clone()))
                .selected(Some(ix) == self.selected_index)
        })
    }

    fn set_selected_index(
        &mut self,
        ix: Option<IndexPath>,
        _window: &mut Window,
        cx: &mut Context<ListState<Self>>,
    ) {
        self.selected_index = ix;
        cx.notify();
    }
}

// Create the list
let delegate = MyListDelegate {
    items: vec!["Item 1".into(), "Item 2".into(), "Item 3".into()],
    selected_index: None,
};

/// Create a list state.
let state = cx.new(|cx| ListState::new(delegate, window, cx));
```

Example 3 (rs):
```rs
div().child(List::new(&state))
```

Example 4 (rust):
```rust
impl ListDelegate for MyListDelegate {
    type Item = ListItem;

    fn sections_count(&self, _cx: &App) -> usize {
        3 // Number of sections
    }

    fn items_count(&self, section: usize, _cx: &App) -> usize {
        match section {
            0 => 5,
            1 => 3,
            2 => 7,
            _ => 0,
        }
    }

    fn render_section_header(
        &mut self,
        section: usize,
        _window: &mut Window,
        cx: &mut Context<TableState<Self>>,
    ) -> Option<impl IntoElement> {
        let title = match section {
            0 => "Section 1",
            1 => "Section 2",
            2 => "Section 3",
            _ => return None,
        };

        Some(
            h_flex()
                .px_2()
                .py_1()
                .gap_2()
                .text_sm()
                .text_color(cx.theme().muted_foreground)
                .child(Icon::new(IconName::Folder))
                .child(title)
        )
    }

    fn render_section_footer(
        &mut self,
        section: usize,
        _window: &mut Window,
        cx: &mut Context<TableState<Self>>,
    ) -> Option<impl IntoElement> {
        Some(
            div()
                .px_2()
                .py_1()
                .text_xs()
                .text_color(cx.theme().muted_foreground)
                .child(format!("End of section {}", section + 1))
        )
    }
}
```

---

## Spinner

**URL:** llms-txt#spinner

**Contents:**
- Import
- Usage
  - Basic
  - Spinner with Custom Color
  - Spinner Sizes
  - Spinner with Custom Icon
- Available Icons
  - Loading Icons
  - Other Compatible Icons
- Animation

Spinner element displays an animated loading. Perfect for showing loading states, progress spinners, and other visual feedback during asynchronous operations. Features customizable icons, colors, sizes, and rotation animations.

### Spinner with Custom Color

### Spinner with Custom Icon

The Spinner component supports various loading and progress icons:

* `Loader` (default) - Rotating line spinner
* `LoaderCircle` - Circular loading spinner

### Other Compatible Icons

* Any icon from the `IconName` enum can be used, though loading-specific icons work best with the rotation animation

The Spinner component features a built-in rotation animation:

* **Duration**: 0.8 seconds (configurable via speed parameter)
* **Easing**: Ease-in-out transition
* **Repeat**: Infinite loop
* **Transform**: 360-degree rotation

| Size        | Method              | Approximate Pixels |
| ----------- | ------------------- | ------------------ |
| Extra Small | `.xsmall()`         | ~12px              |
| Small       | `.small()`          | ~14px              |
| Medium      | (default)           | ~16px              |
| Large       | `.large()`          | ~24px              |
| Custom      | `.with_size(px(n))` | n px               |

### Different Loading Icons

## Performance Considerations

* The animation uses CSS transforms for optimal performance
* Multiple spinners on the same page share the same animation timing
* The component is lightweight and suitable for frequent updates
* Consider using smaller sizes for better performance with many spinners

### Conditional Loading

### Loading with Text

---
url: /gpui-component/docs/components/stepper.md
description: >-
  A step-by-step progress for users to navigate through a series of steps or
  stages.
---

**Examples:**

Example 1 (rust):
```rust
use gpui_component::spinner::Spinner;
```

Example 2 (rust):
```rust
// Default loader icon
Spinner::new()
```

Example 3 (rust):
```rust
use gpui_component::ActiveTheme;

// Blue spinner
Spinner::new()
    .color(cx.theme().blue)

// Green spinner for success states
Spinner::new()
    .color(cx.theme().green)

// Custom color
Spinner::new()
    .color(cx.theme().cyan)
```

Example 4 (rust):
```rust
// Extra small spinner
Spinner::new().xsmall()

// Small spinner
Spinner::new().small()

// Medium spinner (default)
Spinner::new()

// Large spinner
Spinner::new().large()

// Custom size
Spinner::new().with_size(px(64.))
```

---

## DescriptionList

**URL:** llms-txt#descriptionlist

**Contents:**
- Import
- Usage
  - Basic Description List
  - Using DescriptionItem Builder
  - Different Layouts
  - Multiple Columns with Spans
  - With Dividers
  - Different Sizes
  - Without Borders
  - Custom Label Width (Horizontal Layout)

A versatile component for displaying key-value pairs in a structured, organized layout. Supports both horizontal and vertical layouts, multiple columns, borders, and different sizes. Perfect for showing detailed information like metadata, specifications, or summary data.

### Basic Description List

### Using DescriptionItem Builder

### Different Layouts

### Multiple Columns with Spans

### Custom Label Width (Horizontal Layout)

### Rich Content with Custom Elements

### Complex Example with Mixed Content

### User Profile Information

### System Information

### Product Specifications

### Configuration Settings

* Use horizontal layout for simple key-value pairs
* Use vertical layout when values are lengthy or complex
* Limit columns to 3-4 for optimal readability
* Use dividers to group related information
* Keep labels concise and descriptive
* Use consistent spacing with the size prop
* Consider removing borders for embedded contexts

---
url: /gpui-component/docs/components/dialog.md
description: A dialog dialog for displaying content in a layer above the app.
---

**Examples:**

Example 1 (rust):
```rust
use gpui_component::description_list::{DescriptionList, DescriptionItem, DescriptionText};
```

Example 2 (rust):
```rust
DescriptionList::new()
    .child("Name", "GPUI Component", 1)
    .child("Version", "0.1.0", 1)
    .child("License", "Apache-2.0", 1)
```

Example 3 (rust):
```rust
DescriptionList::new()
    .children([
        DescriptionItem::new("Name").value("GPUI Component"),
        DescriptionItem::new("Description").value("UI components for building desktop applications"),
        DescriptionItem::new("Version").value("0.1.0"),
    ])
```

Example 4 (rust):
```rust
// Horizontal layout (default)
DescriptionList::horizontal()
    .item("Platform", "macOS, Windows, Linux", 1)
    .item("Repository", "https://github.com/longbridge/gpui-component", 1)

// Vertical layout
DescriptionList::vertical()
    .item("Name", "GPUI Component", 1)
    .item("Description", "A comprehensive UI component library", 1)
```

---

## DropdownButton

**URL:** llms-txt#dropdownbutton

**Contents:**
- Import
- Usage
  - Variants
  - With custom anchor

A [DropdownButton] is a combination of a button and a trigger button. It allows us to display a dropdown menu when the trigger is clicked, but the left Button can still respond to independent events.

And more option methods of [Button] are also available for the DropdownButton, such as setting different variants using [ButtonCustomVariant], sizes using [Sizable], adding icons, loading states.

Same as [Button], DropdownButton supports different variants.

### With custom anchor

[Button]: https://docs.rs/gpui-component/latest/gpui_component/button/struct.Button.html

[DropdownButton]: https://docs.rs/gpui-component/latest/gpui_component/button/struct.DropdownButton.html

[ButtonCustomVariant]: https://docs.rs/gpui-component/latest/gpui_component/button/struct.ButtonCustomVariant.html

[Sizable]: https://docs.rs/gpui-component/latest/gpui_component/trait.Sizable.html

---
url: /gpui-component/docs/components/editor.md
description: >-
  Multi-line text input component with auto-resize, validation, and advanced
  editing features.
---

**Examples:**

Example 1 (rust):
```rust
use gpui_component::button::{Button, DropdownButton};
```

Example 2 (rust):
```rust
use gpui::Corner;

DropdownButton::new("dropdown")
    .button(Button::new("btn").label("Click Me"))
    .dropdown_menu(|menu, _, _| {
        menu.menu("Option 1", Box::new(MyAction))
            .menu("Option 2", Box::new(MyAction))
            .separator()
            .menu("Option 3", Box::new(MyAction))
    })
```

Example 3 (rust):
```rust
DropdownButton::new("dropdown")
    .primary()
    .button(Button::new("btn").label("Primary"))
    .dropdown_menu(|menu, _, _| {
        menu.menu("Option 1", Box::new(MyAction))
    })
```

Example 4 (rust):
```rust
// With custom anchor
DropdownButton::new("dropdown")
    .button(Button::new("btn").label("Click Me"))
    .dropdown_menu_with_anchor(Corner::BottomRight, |menu, _, _| {
        menu.menu("Option 1", Box::new(MyAction))
    })
```

---

## GPUI Component Introduction

**URL:** llms-txt#gpui-component-introduction

**Contents:**
- Features
- Quick Example
- Community & Support
- License

GPUI Component is a Rust UI component library for building fantastic desktop applications using [GPUI](https://gpui.rs).

GPUI Component is a comprehensive UI component library for building fantastic desktop applications using [GPUI](https://gpui.rs). It provides 60+ cross-platform components with modern design, theming support, and high performance.

* **Richness**: 60+ cross-platform desktop UI components
* **Native**: Inspired by macOS and Windows controls, combined with shadcn/ui design
* **Ease of Use**: Stateless `RenderOnce` components, simple and user-friendly
* **Customizable**: Built-in `Theme` and `ThemeColor`, supporting multi-theme
* **Versatile**: Supports sizes like `xs`, `sm`, `md`, and `lg`
* **Flexible Layout**: Dock layout for panel arrangements, resizing, and freeform (Tiles) layouts
* **High Performance**: Virtualized Table and List components for smooth large-data rendering
* **Content Rendering**: Native support for Markdown and simple HTML
* **Charting**: Built-in charts for visualization
* **Editor**: High performance code editor with LSP support
* **Syntax Highlighting**: Using Tree Sitter

Add `gpui` and `gpui-component` to your `Cargo.toml`:

Then create a simple "Hello, World!" application with a button:

## Community & Support

* [GitHub Repository](https://github.com/longbridge/gpui-component)
* [Issue Tracker](https://github.com/longbridge/gpui-component/issues)
* [Contributing Guide](https://github.com/longbridge/gpui-component/blob/main/CONTRIBUTING.md)

---
url: /gpui-component/docs/components/kbd.md
description: Displays keyboard shortcuts with platform-specific formatting.
---

**Examples:**

Example 1 (unknown):
```unknown
Then create a simple "Hello, World!" application with a button:
```

---

## Toggle

**URL:** llms-txt#toggle

**Contents:**
- Import
- Usage
  - Basic Toggle
  - Icon Toggle
  - Controlled Toggle
  - Toggle Variants
  - Different Sizes
  - Disabled State
- Toggle vs Switch
- Integration with ToggleGroup

A button-style toggle component that represents on/off or selected states. Unlike a traditional switch, toggles appear as buttons that can be pressed in or out. They're perfect for toolbar buttons, filter options, or any binary choice that benefits from a button-like appearance.

Here, we can use `on_click` to handle toggle state changes. The callback receives the **new checked state** as a `bool`.

### Controlled Toggle

| Feature                | Toggle                                      | Switch                                    |
| ---------------------- | ------------------------------------------- | ----------------------------------------- |
| **Appearance**         | Button-like, can be pressed in/out          | Traditional switch with sliding indicator |
| **Use Cases**          | Toolbar buttons, filters, binary options    | Settings, preferences, on/off states      |
| **Visual Style**       | Rectangular button shape                    | Rounded switch track with thumb           |
| **State Indication**   | Background color change, pressed appearance | Position of sliding thumb                 |
| **Multiple Selection** | Supports groups with multiple selection     | Individual switches only                  |

**Use Toggle when you want:**

* Button-like appearance for binary states
* Grouping multiple related options
* Toolbar or filter interfaces
* Options that feel like "selections" rather than "settings"

**Use Switch when you want:**

* Traditional on/off control appearance
* Settings or preferences interface
* Clear visual indication of state with sliding animation
* Individual boolean controls

## Integration with ToggleGroup

Toggle buttons can be grouped together using `ToggleGroup` for related options:

### Basic Toggle Group

The `on_click` callback receives a `Vec<bool>` representing the **new checked state** of each toggle in the group.

### Toggle Group with Controlled State

### Toggle Group Variants and Sizes

### Individual Toggle Events

### Toolbar with Toggle Buttons

### Settings with Individual Toggles

### Multi-select Options

1. **Use meaningful labels**: Choose clear, descriptive text for toggle labels
2. **Group related options**: Use ToggleGroup for logically related binary choices
3. **Provide visual feedback**: The checked state should be clearly distinguishable
4. **Consider context**: Use toggles for options that feel like "selections" rather than "settings"
5. **Maintain state consistency**: Ensure toggle state reflects the actual application state
6. **Accessible labels**: Provide tooltips or ARIA labels for icon-only toggles

---
url: /gpui-component/docs/components/tooltip.md
description: >-
  Display helpful information on hover or focus, with support for keyboard
  shortcuts and custom content.
---

**Examples:**

Example 1 (rust):
```rust
use gpui_component::button::{Toggle, ToggleGroup};
```

Example 2 (rust):
```rust
Toggle::new("toggle1").
    .label("Toggle me")
    .checked(false)
    .on_click(|checked, _, _| {
        println!("Toggle is now: {}", checked);
    })
```

Example 3 (rust):
```rust
use gpui_component::IconName;

Toggle::new("toggle2")
    .icon(IconName::Eye)
    .checked(true)
    .on_click(|checked, _, _| {
        println!("Visibility: {}", if *checked { "shown" } else { "hidden" });
    })
```

Example 4 (rust):
```rust
struct MyView {
    is_active: bool,
}

impl Render for MyView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        Toggle::new("active")
            .label("Active")
            .checked(self.is_active)
            .on_click(cx.listener(|view, checked, _, cx| {
                view.is_active = *checked;
                cx.notify();
            }))
    }
}
```

---

## Theme

**URL:** llms-txt#theme

**Contents:**
- Theme Registry

All components support theming through the built-in Theme system, the [ActiveTheme] trait provides access to the current theme colors:

So if you want use the colors from the current theme, you should keep your component or view have [App] context.

There have more than 20 built-in themes available in [themes](https://github.com/longbridge/gpui-component/tree/main/themes) folder.

https://github.com/longbridge/gpui-component/tree/main/themes

And we have a [ThemeRegistry] to help us to load themes.

[ActiveTheme]: https://docs.rs/gpui-component/latest/gpui_component/theme/trait.ActiveTheme.html

[ThemeRegistry]: https://docs.rs/gpui-component/latest/gpui_component/theme/struct.ThemeRegistry.html

[App]: https://docs.rs/gpui/latest/gpui/struct.App.html

---
url: /gpui-component/docs/components/title-bar.md
description: >-
  A custom window title bar component with window controls and custom content
  support.
---

**Examples:**

Example 1 (rs):
```rs
use gpui_component::{ActiveTheme as _};

// Access theme colors in your components
cx.theme().primary
cx.theme().background
cx.theme().foreground
```

Example 2 (rs):
```rs
use std::path::PathBuf;
use gpui::{App, SharedString};
use gpui_component::{Theme, ThemeRegistry};

pub fn init(cx: &mut App) {
    let theme_name = SharedString::from("Ayu Light");
    // Load and watch themes from ./themes directory
    if let Err(err) = ThemeRegistry::watch_dir(PathBuf::from("./themes"), cx, move |cx| {
        if let Some(theme) = ThemeRegistry::global(cx)
            .themes()
            .get(&theme_name)
            .cloned()
        {
            Theme::global_mut(cx).apply_config(&theme);
        }
    }) {
        tracing::error!("Failed to watch themes directory: {}", err);
    }
}
```

---

## Tooltip

**URL:** llms-txt#tooltip

**Contents:**
- Import
- Usage
  - Basic Tooltip with Text
  - Button with Tooltip
  - Tooltip with Action/Keybinding
  - Custom Element Tooltip
  - Tooltip with Manual Keybinding
- Advanced Usage
  - Components with Built-in Tooltip Support
  - Complex Tooltip Content

A versatile tooltip component that displays helpful information when hovering over or focusing on elements. Supports text content, custom elements, keyboard shortcuts, different trigger methods, and positioning options.

### Basic Tooltip with Text

### Button with Tooltip

### Tooltip with Action/Keybinding

### Custom Element Tooltip

### Tooltip with Manual Keybinding

### Components with Built-in Tooltip Support

Many components have built-in tooltip methods:

### Complex Tooltip Content

### Tooltip in Form Elements

| Method                    | Description                                  |
| ------------------------- | -------------------------------------------- |
| `new(text)`               | Create a tooltip with text content           |
| `element(builder)`        | Create a tooltip with custom element content |
| `action(action, context)` | Set action to display keybinding information |
| `key_binding(kbd)`        | Set manual keybinding information            |
| `build(window, cx)`       | Build and return the tooltip as AnyView      |

### Built-in Tooltip Methods

Components with tooltip support typically provide these methods:

| Method                                       | Description                             |
| -------------------------------------------- | --------------------------------------- |
| `tooltip(text)`                              | Add simple text tooltip                 |
| `tooltip_with_action(text, action, context)` | Add tooltip with action keybinding      |
| `tooltip(closure)`                           | Add custom tooltip with builder closure |

The tooltip automatically applies theme-appropriate styling:

* Background: `theme.popover`
* Text color: `theme.popover_foreground`
* Border: `theme.border`
* Shadow: Medium drop shadow
* Border radius: 6px
* Font: System UI font

You can apply additional styling using the `Styled` trait:

### Toolbar with Tooltips

### Status Indicators with Tooltips

### Interactive Elements with Rich Tooltips

### Form Validation with Tooltips

### Content Guidelines

* **Be concise**: Keep tooltip text short and to the point
* **Be helpful**: Provide additional context, not redundant information
* **Use proper tone**: Match your application's voice and tone
* **Avoid critical info**: Don't put essential information only in tooltips

* **Progressive disclosure**: Use tooltips for additional context, not primary information
* **Consistency**: Use consistent tooltip patterns throughout your application
* **Performance**: Avoid complex content in frequently triggered tooltips
* **Testing**: Test tooltips with both mouse and keyboard interaction

### Examples of Good Tooltip Content

### Examples to Avoid

---
url: /gpui-component/docs/components/tree.md
description: >-
  A hierarchical tree view component for displaying and navigating
  tree-structured data.
---

**Examples:**

Example 1 (rust):
```rust
use gpui_component::tooltip::Tooltip;
```

Example 2 (rust):
```rust
// Simple text tooltip
div()
    .child("Hover me")
    .id("basic-tooltip")
    .tooltip(|window, cx| {
        Tooltip::new("This is a helpful tooltip").build(window, cx)
    })
```

Example 3 (rust):
```rust
Button::new("save-btn")
    .label("Save")
    .tooltip("Save the current document")
```

Example 4 (rust):
```rust
actions!(my_actions, [SaveDocument]);

Button::new("save-btn")
    .label("Save")
    .tooltip_with_action(
        "Save the current document",
        &SaveDocument,
        Some("MyContext")
    )
```

---

## Sheet

**URL:** llms-txt#sheet

**Contents:**
- Import
- Usage
  - Setup application root view for display of sheets
  - Basic Sheet
  - Sheet with Placement
  - Sheet with Custom Size
  - Sheet with Form Content
  - Overlay Options
  - Resizable Sheet
  - Custom Margin and Positioning

A Sheet (also known as a sidebar or slide-out panel) is a navigation component that slides in from the edges of the screen. It provides additional space for content without taking up the main view, and can be used for navigation menus, forms, or any supplementary content.

### Setup application root view for display of sheets

You need to set up your application's root view to render the sheet layer. This is typically done in your main application struct's render method.

The [Root::render\_sheet\_layer](https://docs.rs/gpui-component/latest/gpui_component/struct.Root.html#method.render_sheet_layer) function handles rendering any active modals on top of your app content.

### Sheet with Placement

### Sheet with Custom Size

### Sheet with Form Content

### Custom Margin and Positioning

### Close Event Handling

### Programmatic Close

### Window Extensions

| Method                             | Description                               |
| ---------------------------------- | ----------------------------------------- |
| `open_sheet(cx, fn)`               | Open sheet with default placement (Right) |
| `open_sheet_at(placement, cx, fn)` | Open sheet at specific placement          |
| `close_sheet(cx)`                  | Close current sheet                       |

| Method                   | Description                             |
| ------------------------ | --------------------------------------- |
| `title(str)`             | Set sheet title                         |
| `child(el)`              | Add content to sheet body               |
| `footer(el)`             | Set footer content                      |
| `size(px)`               | Set sheet size (width or height)        |
| `margin_top(px)`         | Set top margin (for title bars)         |
| `resizable(bool)`        | Allow resizing (default: true)          |
| `overlay(bool)`          | Show overlay background (default: true) |
| `overlay_closable(bool)` | Click overlay to close (default: true)  |
| `on_close(fn)`           | Close event callback                    |

### Placement Options

| Value               | Description                         |
| ------------------- | ----------------------------------- |
| `Placement::Left`   | Slides in from left edge            |
| `Placement::Right`  | Slides in from right edge (default) |
| `Placement::Top`    | Slides in from top edge             |
| `Placement::Bottom` | Slides in from bottom edge          |

| Method                | Description              |
| --------------------- | ------------------------ |
| `bg(color)`           | Set background color     |
| `text_color(color)`   | Set text color           |
| `border_color(color)` | Set border color         |
| `px_*()/py_*()`       | Custom padding           |
| `gap_*()`             | Spacing between children |

1. **Appropriate Placement**: Use left/right for navigation, top/bottom for temporary content
2. **Consistent Sizing**: Maintain consistent sheet sizes across your application
3. **Clear Headers**: Always provide descriptive titles
4. **Close Options**: Provide multiple ways to close (ESC, overlay click, close button)
5. **Content Organization**: Use proper spacing and grouping for sheet content
6. **Responsive Design**: Consider sheet behavior on smaller screens
7. **Performance**: Lazy load sheet content when possible for better performance

---
url: /gpui-component/docs/components/sidebar.md
description: >-
  A composable, themeable and customizable sidebar component for navigation and
  content organization.
---

**Examples:**

Example 1 (rust):
```rust
use gpui_component::WindowExt;
use gpui_component::Placement;
```

Example 2 (rust):
```rust
use gpui_component::TitleBar;

struct MyApp {
    view: AnyView,
}

impl Render for MyApp {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let sheet_layer = Root::render_sheet_layer(window, cx);

        div()
            .size_full()
            .child(
                v_flex()
                    .size_full()
                    .child(TitleBar::new())
                    .child(div().flex_1().overflow_hidden().child(self.view.clone())),
            )
            // Render the sheet layer on top of the app content
            .children(sheet_layer)
    }
}
```

Example 3 (rust):
```rust
window.open_sheet(cx, |sheet, _, _| {
    sheet
        .title("Navigation")
        .child("Sheet content goes here")
})
```

Example 4 (rust):
```rust
// Left sheet (default)
window.open_sheet_at(Placement::Left, cx, |sheet, _, _| {
    sheet.title("Left Sheet")
})

// Right sheet
window.open_sheet_at(Placement::Right, cx, |sheet, _, _| {
    sheet.title("Right Sheet")
})

// Top sheet
window.open_sheet_at(Placement::Top, cx, |sheet, _, _| {
    sheet.title("Top Sheet")
})

// Bottom sheet
window.open_sheet_at(Placement::Bottom, cx, |sheet, _, _| {
    sheet.title("Bottom Sheet")
})
```

---

## Calendar

**URL:** llms-txt#calendar

**Contents:**
- Import
- Usage
  - Basic Calendar
  - Calendar with Initial Date
  - Date Range Calendar
  - Multiple Months Display
  - Calendar Sizes
- Date Restrictions
  - Disabled Weekends
  - Disabled Specific Weekdays

A standalone calendar component that provides a rich interface for date selection and navigation. The Calendar component supports single date selection, date range selection, multiple month views, custom disabled dates, and comprehensive keyboard navigation.

* [CalendarState]: For managing calendar state and selection.
* [Calendar]: For rendering the calendar UI.

### Calendar with Initial Date

### Date Range Calendar

### Multiple Months Display

### Disabled Weekends

### Disabled Specific Weekdays

### Disabled Date Range

### Disabled Date Interval

### Custom Disabled Dates

## Month/Year Navigation

The Calendar automatically provides navigation controls:

* **Previous/Next Month**: Arrow buttons in the header
* **Month Selection**: Click on month name to open month picker
* **Year Selection**: Click on year to open year picker
* **Year Pages**: Navigate through 20-year pages in year view

### Custom Year Range

## Handle Selection Events

### Business Days Only Calendar

### Multi-Month Range Selector

### Quarterly View Calendar

* [Calendar]
* [CalendarState]
* [RangeMatcher]

### Event Planning Calendar

### Vacation Booking Calendar

### Report Date Range Selector

### Availability Calendar

The Calendar component provides a foundation for any date-related UI requirements, from simple date pickers to complex scheduling interfaces.

[Calendar]: https://docs.rs/gpui-component/latest/gpui_component/calendar/struct.Calendar.html

[CalendarState]: https://docs.rs/gpui-component/latest/gpui_component/calendar/struct.CalendarState.html

[RangeMatcher]: https://docs.rs/gpui-component/latest/gpui_component/calendar/struct.RangeMatcher.html

---
url: /gpui-component/docs/components/chart.md
description: >-
  Beautiful charts and graphs for data visualization including line, bar, area,
  pie, and candlestick charts.
---

**Examples:**

Example 1 (rust):
```rust
use gpui_component::{
    calendar::{Calendar, CalendarState, CalendarEvent, Date, Matcher},
};
```

Example 2 (rust):
```rust
let state = cx.new(|cx| CalendarState::new(window, cx));
Calendar::new(&state)
```

Example 3 (rust):
```rust
use chrono::Local;

let state = cx.new(|cx| {
    let mut state = CalendarState::new(window, cx);
    state.set_date(Local::now().naive_local().date(), window, cx);
    state
});

Calendar::new(&state)
```

Example 4 (rust):
```rust
use chrono::{Local, Days};

let state = cx.new(|cx| {
    let mut state = CalendarState::new(window, cx);
    let now = Local::now().naive_local().date();
    state.set_date(
        Date::Range(Some(now), now.checked_add_days(Days::new(7))),
        window,
        cx
    );
    state
});

Calendar::new(&state)
```

---

## Switch

**URL:** llms-txt#switch

**Contents:**
- Import
- Usage
  - Basic Switch
  - Controlled Switch
  - With Label
  - Different Sizes
  - Disabled State
  - With Tooltip
- API Reference
  - Switch

A toggle switch component for binary on/off states. Features smooth animations, different sizes, labels, disabled state, and customizable positioning.

### Controlled Switch

| Method             | Description                                                 |
| ------------------ | ----------------------------------------------------------- |
| `new(id)`          | Create a new switch with the given ID                       |
| `checked(bool)`    | Set the checked/toggled state                               |
| `label(text)`      | Set label text for the switch                               |
| `label_side(side)` | Position label (Side::Left or Side::Right)                  |
| `disabled(bool)`   | Set disabled state                                          |
| `tooltip(text)`    | Add tooltip text                                            |
| `on_click(fn)`     | Callback when clicked, receives `&bool` (new checked state) |

Implements `Sizable` and `Disableable` traits:

* `small()` - Small switch size (28x16px toggle area)
* `medium()` - Medium switch size (36x20px toggle area, default)
* `with_size(size)` - Set explicit size
* `disabled(bool)` - Disabled state

### Styling Properties

The switch can also be styled using GPUI's styling methods:

* `w(width)` - Custom width
* `h(height)` - Custom height
* Standard margin, padding, and positioning methods

### Compact Settings List

The switch features smooth animations:

* **Toggle animation**: 150ms duration when switching states
* **Background color transition**: Changes from switch color to primary color
* **Position animation**: Smooth movement of the toggle indicator
* **Disabled state**: Animations are disabled when the switch is disabled

---
url: /gpui-component/docs/components/table.md
description: >-
  High-performance data table with virtual scrolling, sorting, filtering, and
  column management.
---

**Examples:**

Example 1 (rust):
```rust
use gpui_component::switch::Switch;
```

Example 2 (rust):
```rust
Switch::new("my-switch")
    .checked(false)
    .on_click(|checked, _, _| {
        println!("Switch is now: {}", checked);
    })
```

Example 3 (rust):
```rust
struct MyView {
    is_enabled: bool,
}

impl Render for MyView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        Switch::new("switch")
            .checked(self.is_enabled)
            .on_click(cx.listener(|view, checked, _, cx| {
                view.is_enabled = *checked;
                cx.notify();
            }))
    }
}
```

Example 4 (rust):
```rust
Switch::new("notifications")
    .label("Enable notifications")
    .checked(true)
    .on_click(|checked, _, _| {
        println!("Notifications: {}", if *checked { "enabled" } else { "disabled" });
    })
```

---

## Icons & Assets

**URL:** llms-txt#icons-&-assets

**Contents:**
- Use default bundled assets
- Build you own assets
- Use the icons
- Resources

The [IconName] and [Icon] in GPUI Component provide a comprehensive set of icons and assets that can be easily integrated into your GPUI applications.

But for minimal size applications, **we have not embedded any icon assets by default** in `gpui-component` crate.

We split the icon assets into a separate crate [gpui-component-assets] to allow developers to choose whether to include the icon assets in their applications or if you don't need the icons at all, you can build your own assets.

## Use default bundled assets

The [gpui-component-assets] crate provides a default bundled assets implementation that includes all the icon files in the `assets/icons` folder.

To use the default bundled assets, you need to add the `gpui-component-assets` crate as a dependency in your `Cargo.toml`:

Then we need call the `with_assets` method when creating the GPUI application to register the asset source:

Now, we can use `IconName` and `Icon` in our application as usual, the all icon assets are loaded from the default bundled assets.

Continue [Use the icons](#use-the-icons) section to see how to use the icons in your application.

## Build you own assets

You may have a specific set of icons that you want to use in your application, or you may want to reduce the size of your application binary by including only the icons you need.

In this case, you can build your own assets by following these steps.

The [assets](https://github.com/longbridge/gpui-component/tree/main/crates/assets/assets/) folder in source code contains all the available icons in SVG format, every file is that GPUI Component support, it matched with the [IconName] enum.

You can download the SVG files you need from the [assets] folder, or you can use your own SVG files by following the [IconName] naming convention.

In GPUI application, we can use the [rust-embed] crate to embed the SVG files into the application binary.

And GPUI Application providers an `AssetSource` trait to load the assets.

We need call the `with_assets` method when creating the GPUI application to register the asset source:

Now we can use the icons in our application:

* [Lucide Icons](https://lucide.dev/) - The icon set used in GPUI Component is based on the open-source Lucide Icons library, which provides a wide range of customizable SVG icons.

[rust-embed]: https://docs.rs/rust-embed/latest/rust_embed/

[IconName]: https://docs.rs/gpui_component/latest/gpui_component/icon/enum.IconName.html

[Icon]: https://docs.rs/gpui_component/latest/gpui_component/icon/struct.Icon.html

[assets]: https://github.com/longbridge/gpui-component/tree/main/crates/assets/assets/

[gpui-component-assets]: https://crates.io/crates/gpui-component-assets

---
url: /gpui-component/docs/components/image.md
description: >-
  A flexible image display component with loading states, fallbacks, and
  responsive sizing options.
---

**Examples:**

Example 1 (unknown):
```unknown
Then we need call the `with_assets` method when creating the GPUI application to register the asset source:
```

Example 2 (unknown):
```unknown
Now, we can use `IconName` and `Icon` in our application as usual, the all icon assets are loaded from the default bundled assets.

Continue [Use the icons](#use-the-icons) section to see how to use the icons in your application.

## Build you own assets

You may have a specific set of icons that you want to use in your application, or you may want to reduce the size of your application binary by including only the icons you need.

In this case, you can build your own assets by following these steps.

The [assets](https://github.com/longbridge/gpui-component/tree/main/crates/assets/assets/) folder in source code contains all the available icons in SVG format, every file is that GPUI Component support, it matched with the [IconName] enum.

You can download the SVG files you need from the [assets] folder, or you can use your own SVG files by following the [IconName] naming convention.

In GPUI application, we can use the [rust-embed] crate to embed the SVG files into the application binary.

And GPUI Application providers an `AssetSource` trait to load the assets.
```

Example 3 (unknown):
```unknown
We need call the `with_assets` method when creating the GPUI application to register the asset source:
```

Example 4 (unknown):
```unknown
## Use the icons

Now we can use the icons in our application:
```

---

## Kbd

**URL:** llms-txt#kbd

**Contents:**
- Import
- Usage
  - Basic Keyboard Shortcut
  - Common Shortcuts
  - Multiple Modifiers
  - Arrow Keys and Function Keys
  - Without Visual Styling
  - From Action Bindings
- Platform Differences
  - macOS

A component for displaying keyboard shortcuts and key combinations with proper platform-specific formatting. Automatically adapts the display to match the conventions of macOS (using symbols) or Windows/Linux (using text labels).

### Basic Keyboard Shortcut

### Multiple Modifiers

### Arrow Keys and Function Keys

### Without Visual Styling

### From Action Bindings

## Platform Differences

The Kbd component automatically formats shortcuts according to platform conventions:

* Uses symbols: ⌃ ⌥ ⇧ ⌘
* No separators between modifiers
* Order: Control, Option, Shift, Command
* Special keys: ⌫ (backspace), ⎋ (escape), ⏎ (enter), ← → ↑ ↓ (arrows)

* Uses text labels: Ctrl, Alt, Shift, Win
* Plus sign (+) separators
* Order: Ctrl, Alt, Shift, Win
* Special keys: Backspace, Esc, Enter, Left, Right, Up, Down

### Examples by Platform

| Input               | macOS | Windows/Linux     |
| ------------------- | ----- | ----------------- |
| `cmd-a`             | ⌘A    | Win+A             |
| `ctrl-shift-a`      | ⌃⇧A   | Ctrl+Shift+A      |
| `cmd-alt-backspace` | ⌥⌘⌫   | Win+Alt+Backspace |
| `escape`            | ⎋     | Esc               |
| `enter`             | ⏎     | Enter             |
| `left`              | ←     | Left              |

### Keyboard Shortcut Help

### Menu Item with Shortcut

### Inline Documentation

The Kbd component uses the following default styles:

* Border with theme border color
* Muted foreground text color
* Background with theme background color
* Small rounded corners
* Centered text alignment
* Extra small font size
* Minimal padding (0.5px vertical, 1px horizontal)
* Minimum width of 5 units
* Flex shrink disabled to maintain size

All styles can be customized using the `Styled` trait methods.

---
url: /gpui-component/docs/components/label.md
description: Text labels for form elements with highlighting and styling options.
---

**Examples:**

Example 1 (rust):
```rust
use gpui_component::kbd::Kbd;
use gpui::Keystroke;
```

Example 2 (rust):
```rust
// Create from a keystroke
let kbd = Kbd::new(Keystroke::parse("cmd-shift-p").unwrap());

// Or convert directly from keystroke
let kbd: Kbd = Keystroke::parse("escape").unwrap().into();
```

Example 3 (rust):
```rust
// Command palette
Kbd::new(Keystroke::parse("cmd-shift-p").unwrap())

// New tab
Kbd::new(Keystroke::parse("cmd-t").unwrap())

// Zoom controls
Kbd::new(Keystroke::parse("cmd--").unwrap())  // Zoom out
Kbd::new(Keystroke::parse("cmd-+").unwrap())  // Zoom in

// Navigation
Kbd::new(Keystroke::parse("escape").unwrap())
Kbd::new(Keystroke::parse("enter").unwrap())
Kbd::new(Keystroke::parse("backspace").unwrap())
```

Example 4 (rust):
```rust
// Complex combinations
Kbd::new(Keystroke::parse("cmd-ctrl-shift-a").unwrap())
Kbd::new(Keystroke::parse("cmd-alt-backspace").unwrap())
Kbd::new(Keystroke::parse("ctrl-alt-shift-a").unwrap())
```

---

## Sidebar

**URL:** llms-txt#sidebar

**Contents:**
- Import
- Usage
  - Basic Sidebar
  - Collapsible Sidebar
  - Nested Menu Items
  - Multiple Groups
  - With Badges and Suffixes
  - Right-Side Placement
  - Custom Width and Styling
  - Interactive Header with Popup Menu

A flexible sidebar component that provides navigation structure for applications. Features collapsible states, nested menu items, header and footer sections, and responsive design. Perfect for creating application navigation panels, admin dashboards, and complex hierarchical interfaces.

### Collapsible Sidebar

### Nested Menu Items

### With Badges and Suffixes

### Right-Side Placement

### Custom Width and Styling

### Interactive Header with Popup Menu

### Footer with User Information

### Responsive Sidebar

The sidebar uses dedicated theme colors:

### File Explorer Sidebar

### Admin Dashboard Sidebar

---
url: /gpui-component/docs/components/skeleton.md
description: Use to show a placeholder while content is loading.
---

**Examples:**

Example 1 (rust):
```rust
use gpui_component::sidebar::{
    Sidebar, SidebarHeader, SidebarFooter, SidebarGroup,
    SidebarMenu, SidebarMenuItem, SidebarToggleButton
};
```

Example 2 (rust):
```rust
use gpui_component::{sidebar::*, Side};

Sidebar::new(Side::Left)
    .header(
        SidebarHeader::new()
            .child("My Application")
    )
    .child(
        SidebarGroup::new("Navigation")
            .child(
                SidebarMenu::new()
                    .child(
                        SidebarMenuItem::new("Dashboard")
                            .icon(IconName::LayoutDashboard)
                            .on_click(|_, _, _| println!("Dashboard clicked"))
                    )
                    .child(
                        SidebarMenuItem::new("Settings")
                            .icon(IconName::Settings)
                            .on_click(|_, _, _| println!("Settings clicked"))
                    )
            )
    )
    .footer(
        SidebarFooter::new()
            .child("User Profile")
    )
```

Example 3 (rust):
```rust
let mut collapsed = false;

Sidebar::new(Side::Left)
    .collapsed(collapsed)
    .collapsible(true)
    .header(
        SidebarHeader::new()
            .child(
                h_flex()
                    .child(Icon::new(IconName::Home))
                    .when(!collapsed, |this| this.child("Home"))
            )
    )
    .child(
        SidebarGroup::new("Menu")
            .child(
                SidebarMenu::new()
                    .child(
                        SidebarMenuItem::new("Files")
                            .icon(IconName::Folder)
                    )
            )
    )

// Toggle button
SidebarToggleButton::left()
    .collapsed(collapsed)
    .on_click(|_, _, _| {
        collapsed = !collapsed;
    })
```

Example 4 (rust):
```rust
SidebarMenuItem::new("Projects")
    .icon(IconName::FolderOpen)
    .active(true)
    .children([
        SidebarMenuItem::new("Web App")
            .active(false)
            .on_click(|_, _, _| println!("Web App selected")),
        SidebarMenuItem::new("Mobile App")
            .active(true)
            .on_click(|_, _, _| println!("Mobile App selected")),
        SidebarMenuItem::new("Desktop App")
            .on_click(|_, _, _| println!("Desktop App selected")),
    ])
    .on_click(|_, _, _| {
        // Toggle project group
    })
```

---

## Icon

**URL:** llms-txt#icon

**Contents:**
- Import
- Usage
  - Basic Icon
  - Icon with Custom Size
  - Icon with Custom Color
  - Rotated Icons
  - Custom SVG Path
- Available Icons
  - Navigation
  - Actions

A flexible icon component that renders SVG icons from the built-in icon library. Icons are based on Lucide.dev and support customization of size, color, and rotation. The component requires SVG files to be provided by the user in the assets bundle.

Before you start, please make sure you have read: [Icons & Assets](../assets.md) to understand how use SVG in GPUI & GPUI Component application.

### Icon with Custom Size

### Icon with Custom Color

The `IconName` enum provides access to a curated set of icons. Here are some commonly used ones:

* `ArrowUp`, `ArrowDown`, `ArrowLeft`, `ArrowRight`
* `ChevronUp`, `ChevronDown`, `ChevronLeft`, `ChevronRight`
* `ChevronsUpDown`

* `Check`, `Close`, `Plus`, `Minus`
* `Copy`, `Delete`, `Search`, `Replace`
* `Maximize`, `Minimize`, `WindowRestore`

* `File`, `Folder`, `FolderOpen`, `FolderClosed`
* `BookOpen`, `Inbox`

* `Menu`, `Settings`, `Settings2`, `Ellipsis`, `EllipsisVertical`
* `Eye`, `EyeOff`, `Bell`, `Info`

### Social & External

* `GitHub`, `Globe`, `ExternalLink`
* `Heart`, `HeartOff`, `Star`, `StarOff`
* `ThumbsUp`, `ThumbsDown`

* `CircleCheck`, `CircleX`, `TriangleAlert`
* `Loader`, `LoaderCircle`

* `PanelLeft`, `PanelRight`, `PanelBottom`
* `PanelLeftOpen`, `PanelRightOpen`, `PanelBottomOpen`
* `LayoutDashboard`, `Frame`

* `User`, `CircleUser`, `Bot`

* `Calendar`, `Map`, `Palette`, `Inspector`
* `Sun`, `Moon`, `Building2`

The Icon component supports several predefined sizes:

| Size        | Method                | CSS Class    | Pixels |
| ----------- | --------------------- | ------------ | ------ |
| Extra Small | `.xsmall()`           | `size_3()`   | 12px   |
| Small       | `.small()`            | `size_3p5()` | 14px   |
| Medium      | `.medium()` (default) | `size_4()`   | 16px   |
| Large       | `.large()`            | `size_6()`   | 24px   |
| Custom      | `.with_size(px(n))`   | -            | n px   |

## Build you own `IconName`.

You can define your own `IconName` to have more specific icons for your application. We have `IconNamed` trait for you to implement for your.

If you want to directly `render` a custom `IconName` you must implement the `RenderOnce` trait and derive `IntoElement` on the `IconName`.

### Animated Loading Icon

### Custom Icon from Assets

* Icons are rendered as SVG elements and support full CSS styling
* The default size matches the current text size if no explicit size is set
* Icons are flex-shrink-0 by default to prevent unwanted shrinking in flex layouts
* All icon paths are relative to the assets bundle root
* Icons from Lucide.dev are designed to work well at 16px and scale nicely to other sizes

---
url: /gpui-component/docs/assets.md
---

**Examples:**

Example 1 (rust):
```rust
use gpui_component::{Icon, IconName};
```

Example 2 (rust):
```rust
// Using IconName enum directly
IconName::Heart

// Or creating an Icon explicitly
Icon::new(IconName::Heart)
```

Example 3 (rust):
```rust
// Predefined sizes
Icon::new(IconName::Search).xsmall()   // size_3()
Icon::new(IconName::Search).small()    // size_3p5()
Icon::new(IconName::Search).medium()   // size_4() (default)
Icon::new(IconName::Search).large()    // size_6()

// Custom pixel size
Icon::new(IconName::Search).with_size(px(20.))
```

Example 4 (rust):
```rust
// Using theme colors
Icon::new(IconName::Heart)
    .text_color(cx.theme().red)

// Using custom colors
Icon::new(IconName::Star)
    .text_color(gpui::red())
```

---

## Label

**URL:** llms-txt#label

**Contents:**
- Import
- Usage
  - Basic Label
  - Label with Secondary Text
  - Text Alignment
  - Text Highlighting
  - Color and Styling
  - Masked Labels
  - Multi-line Text
  - Different Sizes

A versatile label component for displaying text with support for secondary text, highlighting, masking, and customizable styling. Perfect for form labels, captions, and general text display with optional/required indicators.

### Label with Secondary Text

### Text Highlighting

### Color and Styling

| Method              | Description                                                   |
| ------------------- | ------------------------------------------------------------- |
| `new(text)`         | Create a new label with text                                  |
| `secondary(text)`   | Add secondary text (usually for optional/required indicators) |
| `masked(bool)`      | Show/hide text with bullet characters                         |
| `highlights(match)` | Highlight matching text                                       |

| Variant        | Description                                      |
| -------------- | ------------------------------------------------ |
| `Full(text)`   | Highlights all occurrences of the text           |
| `Prefix(text)` | Highlights only if text appears at the beginning |

| Method        | Description                     |
| ------------- | ------------------------------- |
| `as_str()`    | Get the search text as string   |
| `is_prefix()` | Check if this is a prefix match |

### Styling Methods (via Styled trait)

| Method                | Description                 |
| --------------------- | --------------------------- |
| `text_color(color)`   | Set text color              |
| `text_size(size)`     | Set font size               |
| `text_center()`       | Center align text           |
| `text_right()`        | Right align text            |
| `font_semibold()`     | Set font weight to semibold |
| `font_bold()`         | Set font weight to bold     |
| `line_height(height)` | Set line height             |
| `text_xs()`           | Extra small text size       |
| `text_sm()`           | Small text size             |
| `text_base()`         | Base text size (default)    |
| `text_lg()`           | Large text size             |
| `text_xl()`           | Extra large text size       |
| `text_2xl()`          | 2x large text size          |

### Search Highlighting

### Sensitive Information

### Multi-language Support

### Status Indicators

---
url: /gpui-component/docs/components/list.md
description: >-
  A flexible list component that displays a series of items with support for
  sections, search, selection, and infinite scrolling.
---

**Examples:**

Example 1 (rust):
```rust
use gpui_component::label::{Label, HighlightsMatch};
```

Example 2 (rust):
```rust
Label::new("This is a label")
```

Example 3 (rust):
```rust
// Label with optional indicator
Label::new("Company Address")
    .secondary("(optional)")

// Label with required indicator
Label::new("Email Address")
    .secondary("(required)")
```

Example 4 (rust):
```rust
// Left aligned (default)
Label::new("Text align left")

// Center aligned
Label::new("Text align center")
    .text_center()

// Right aligned
Label::new("Text align right")
    .text_right()
```

---

## Notification

**URL:** llms-txt#notification

**Contents:**
- Import
- Usage
  - Setup application root view for display of notifications
  - Basic Notification
  - Notification Types
  - Notification with Title
  - Auto-hide Control
  - With Action Button
  - Clickable Notifications
  - Custom Content

A toast notification system for displaying temporary messages to users. Notifications appear at the top right of the window and can auto-dismiss after a timeout. Supports multiple variants (info, success, warning, error), custom content, titles, and action buttons. Perfect for status updates, confirmations, and user feedback.

### Setup application root view for display of notifications

You need to set up your application's root view to render the notification layer. This is typically done in your main application struct's render method.

The [Root::render\_notification\_layer](https://docs.rs/gpui-component/latest/gpui_component/struct.Root.html#method.render_notification_layer) function handles rendering any active modals on top of your app content.

### Basic Notification

### Notification Types

### Notification with Title

### Auto-hide Control

### With Action Button

### Clickable Notifications

### Unique Notifications

When you need to manage notifications manually, such as for long-running processes or persistent alerts, you can use unique IDs to push and remove notifications as needed.

In this case, you can create a special `struct` in local scope, and use `id` methods with this struct to identify the notification.

Then you can push the notification when needed, and later remove it using the same ID.

Then remove the notification with `window.remove_notification::<UpdateNotification>`, like this:

### Form Validation Error

### File Upload Progress

### System Status Updates

### Batch Operation Results

### Interactive Confirmation

---
url: /gpui-component/docs/components/number-input.md
description: >-
  Number input component with increment/decrement controls and numeric
  formatting.
---

**Examples:**

Example 1 (rust):
```rust
use gpui_component::{
    notification::{Notification, NotificationType},
    WindowExt
};
```

Example 2 (rust):
```rust
use gpui_component::{TitleBar, Root};

struct Example {}

impl Render for Example {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let notification_layer = Root::render_notification_layer(window, cx);

        div()
            .size_full()
            .child(
                v_flex()
                    .size_full()
                    .child(TitleBar::new())
                    .child(div().flex_1().child("Hello world!")),
            )
            // Render the notification layer on top of the app content
            .children(notification_layer)
    }
}
```

Example 3 (rust):
```rust
// Simple string notification
window.push_notification("This is a notification.", cx);

// Using Notification builder
Notification::new()
    .message("Your changes have been saved.")
```

Example 4 (rust):
```rust
// Info notification (blue)
window.push_notification(
    (NotificationType::Info, "File saved successfully."),
    cx,
);

// Success notification (green)
window.push_notification(
    (NotificationType::Success, "Payment processed successfully."),
    cx,
);

// Warning notification (yellow/orange)
window.push_notification(
    (NotificationType::Warning, "Network connection is unstable."),
    cx,
);

// Error notification (red)
window.push_notification(
    (NotificationType::Error, "Failed to save file. Please try again."),
    cx,
);
```

---

## ColorPicker

**URL:** llms-txt#colorpicker

**Contents:**
- Import
- Usage
  - Basic Color Picker
  - With Event Handling
  - Setting Default Color
  - Different Sizes
  - With Custom Featured Colors
  - With Icon Instead of Color Square
  - With Label
  - Custom Anchor Position

A versatile color picker component that provides an intuitive interface for color selection. Features include color palettes, hex input, featured colors, and support for various color formats including RGB, HSL, and hex values with alpha channel support.

### Basic Color Picker

### With Event Handling

### Setting Default Color

### With Custom Featured Colors

### With Icon Instead of Color Square

### Custom Anchor Position

## Color Selection Interface

The color picker includes predefined color palettes organized by color family:

* **Stone**: Neutral grays and stone colors
* **Red**: Red color variations from light to dark
* **Orange**: Orange color variations
* **Yellow**: Yellow color variations
* **Green**: Green color variations
* **Cyan**: Cyan color variations
* **Blue**: Blue color variations
* **Purple**: Purple color variations
* **Pink**: Pink color variations

Each palette provides multiple shades and tints of the base color, allowing for precise color selection.

### Featured Colors Section

A customizable section at the top of the picker that displays frequently used or brand colors. If not specified, defaults to theme colors:

* Primary colors from the current theme
* Light variants of theme colors
* Essential UI colors (red, blue, green, yellow, cyan, magenta)

A text input field that allows direct entry of hex color values:

* Supports standard 6-digit hex format (#RRGGBB)
* Real-time validation and preview
* Updates color picker state automatically
* Press Enter to confirm selection

### RGB (Red, Green, Blue)

Colors are internally represented using GPUI's `Hsla` format but can be converted to RGB:

### HSL (Hue, Saturation, Lightness)

Native format used by the color picker:

Standard web hex format with # prefix:

Full alpha channel support for transparency:

The color picker preserves alpha values when selecting colors and allows modification through the alpha component of HSLA colors.

* [ColorPicker]
* [ColorPickerState]
* [ColorPickerEvent]

### Color Theme Editor

### Brand Color Selector

### Toolbar Color Picker

### Color Palette Builder

### With Color Validation

[ColorPicker]: https://docs.rs/gpui-component/latest/gpui_component/color_picker/struct.ColorPicker.html

[ColorPickerState]: https://docs.rs/gpui-component/latest/gpui_component/color_picker/struct.ColorPickerState.html

[ColorPickerEvent]: https://docs.rs/gpui-component/latest/gpui_component/color_picker/enum.ColorPickerEvent.html

---
url: /gpui-component/docs/components.md
---

**Examples:**

Example 1 (rust):
```rust
use gpui_component::color_picker::{ColorPicker, ColorPickerState, ColorPickerEvent};
```

Example 2 (rust):
```rust
use gpui::{Entity, Window, Context};

// Create color picker state
let color_picker = cx.new(|cx|
    ColorPickerState::new(window, cx)
        .default_value(cx.theme().primary)
);

// Create the color picker component
ColorPicker::new(&color_picker)
```

Example 3 (rust):
```rust
use gpui::{Subscription, Entity};

let color_picker = cx.new(|cx| ColorPickerState::new(window, cx));

let _subscription = cx.subscribe(&color_picker, |this, _, ev, _| match ev {
    ColorPickerEvent::Change(color) => {
        if let Some(color) = color {
            println!("Selected color: {}", color.to_hex());
            // Handle color change
        }
    }
});

ColorPicker::new(&color_picker)
```

Example 4 (rust):
```rust
use gpui::Hsla;

let color_picker = cx.new(|cx|
    ColorPickerState::new(window, cx)
        .default_value(cx.theme().blue) // Set default color
);
```

---

## GroupBox

**URL:** llms-txt#groupbox

**Contents:**
- Import
- Usage
  - Basic GroupBox
  - GroupBox Variants
  - With Title
  - Custom ID
  - Custom Title Styling
  - Custom Content Styling
  - Complex Example
- Examples

The GroupBox component is a versatile container that groups related content together with optional borders, backgrounds, and titles. It provides visual organization and semantic grouping for form controls, settings panels, and other related UI elements.

### GroupBox Variants

### Custom Title Styling

### Custom Content Styling

### Subscription Management

The GroupBox component supports extensive customization through both built-in variants and custom styling:

### Theme Integration

### Custom Appearance

1. **Use titles for clarity** - Always include a descriptive title when grouping form controls
2. **Choose appropriate variants** - Use `fill()` for primary content groups, `outline()` for secondary groupings
3. **Maintain visual hierarchy** - Use GroupBox to create clear sections without overwhelming the interface
4. **Group related content** - Only group logically related controls and information
5. **Consider spacing** - The component automatically handles internal spacing, but consider external margins
6. **Responsive design** - GroupBox adapts well to different screen sizes and container widths

## Related Components

* **Form**: Use GroupBox within forms to organize sections
* **Dialog**: GroupBox works well within dialogs for organizing content
* **Accordion**: For collapsible grouped content, consider using Accordion instead
* **Card**: For elevated content containers with more visual weight

---
url: /gpui-component/docs/components/icon.md
description: 'Display SVG icons with various sizes, colors, and transformations.'
---

**Examples:**

Example 1 (rust):
```rust
use gpui_component::group_box::{GroupBox, GroupBoxVariant, GroupBoxVariants as _};
```

Example 2 (rust):
```rust
GroupBox::new()
    .child("Subscriptions")
    .child(Checkbox::new("all").label("All"))
    .child(Checkbox::new("newsletter").label("Newsletter"))
    .child(Button::new("save").primary().label("Save"))
```

Example 3 (rust):
```rust
// Normal variant (default) - no background or border
GroupBox::new()
    .child("Content without visual container")

// Fill variant - with background color
GroupBox::new()
    .fill()
    .title("Settings")
    .child("Content with background")

// Outline variant - with border, no background
GroupBox::new()
    .outline()
    .title("Preferences")
    .child("Content with border")
```

Example 4 (rust):
```rust
GroupBox::new()
    .fill()
    .title("Account Settings")
    .child(
        h_flex()
            .justify_between()
            .child("Make profile private")
            .child(Switch::new("privacy").checked(false))
    )
    .child(Button::new("save").primary().label("Save Changes"))
```

---

## Slider

**URL:** llms-txt#slider

**Contents:**
- Import
- Usage
  - Basic Slider
  - Slider with Event Handling
  - Range Slider
  - Vertical Slider
  - Custom Step Intervals
  - Min/Max Configuration
  - Disabled State
  - Custom Styling

A slider component for selecting numeric values within a specified range. Supports both single value and range selection modes, horizontal and vertical orientations, custom styling, and step intervals.

### Slider with Event Handling

### Custom Step Intervals

### Min/Max Configuration

There have 2 types of scale for the slider:

* `Linear` (default)
* `Logarithmic`

The logarithmic scale is useful when the range of values is large and you want to give more precision to smaller values.

:::info
$$ v = min \times (max/min)^p $$

The value `v` is calculated using the formula above, where `p` is the slider percentage (0 to 1).
:::

* If slider at 25%, value will be approximately `5.62`.
* If slider at 50%, value will be approximately `31.62`.
* If slider at 75%, value will be approximately `177.83`.
* If slider at 100%, value will be `1000.0`.

| Event                 | Description                       |
| --------------------- | --------------------------------- |
| `Change(SliderValue)` | Emitted when slider value changes |

The slider component implements `Styled` trait and supports:

* Background color for track and thumb
* Text color for thumb
* Border radius
* Size customization

### Price Range Filter

### Temperature Slider with Custom Styling

## Keyboard Shortcuts

| Key           | Action                         |
| ------------- | ------------------------------ |
| `←` / `↓`     | Decrease value by step         |
| `→` / `↑`     | Increase value by step         |
| `Page Down`   | Decrease by larger amount      |
| `Page Up`     | Increase by larger amount      |
| `Home`        | Set to minimum value           |
| `End`         | Set to maximum value           |
| `Tab`         | Move focus to next element     |
| `Shift + Tab` | Move focus to previous element |

---
url: /gpui-component/docs/components/spinner.md
description: Displays an animated loading showing the completion progress of a task.
---

**Examples:**

Example 1 (rust):
```rust
use gpui_component::slider::{Slider, SliderState, SliderEvent, SliderValue};
```

Example 2 (rust):
```rust
let slider_state = cx.new(|_| {
    SliderState::new()
        .min(0.0)
        .max(100.0)
        .default_value(50.0)
        .step(1.0)
});

Slider::new(&slider_state)
```

Example 3 (rust):
```rust
struct MyView {
    slider_state: Entity<SliderState>,
    current_value: f32,
}

impl MyView {
    fn new(cx: &mut Context<Self>) -> Self {
        let slider_state = cx.new(|_| {
            SliderState::new()
                .min(0.0)
                .max(100.0)
                .default_value(25.0)
                .step(5.0)
        });

        let subscription = cx.subscribe(&slider_state, |this, _, event: &SliderEvent, cx| {
            match event {
                SliderEvent::Change(value) => {
                    this.current_value = value.start();
                    cx.notify();
                }
            }
        });

        Self {
            slider_state,
            current_value: 25.0,
        }
    }
}

impl Render for MyView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_2()
            .child(Slider::new(&self.slider_state))
            .child(format!("Value: {}", self.current_value))
    }
}
```

Example 4 (rust):
```rust
let range_slider = cx.new(|_| {
    SliderState::new()
        .min(0.0)
        .max(100.0)
        .default_value(20.0..80.0)  // Range from 20 to 80
        .step(1.0)
});

Slider::new(&range_slider)
```

---

## Getting Started

**URL:** llms-txt#getting-started

**Contents:**
- Installation

Add dependencies to your `Cargo.toml`:

```toml-vue
[dependencies]
gpui = "{{ GPUI_VERSION }}"
gpui-component = "{{ VERSION }}"

---

## Tabs

**URL:** llms-txt#tabs

**Contents:**
- Import
- Usage
  - Basic Tabs
  - Tab Variants
  - Tab Sizes
  - Tabs with Icons
  - Tabs with Prefix and Suffix
  - Disabled Tabs
  - Dynamic Tabs
  - Tabs with Menu

A tabbed interface component for organizing content into separate sections. Supports multiple variants, sizes, navigation controls, and interactive features like reordering and prefix/suffix elements.

### Tabs with Prefix and Suffix

Use `menu` option to enable a dropdown menu for tab selection when there are many tabs,
this is default `false`.

If enable, the will have a dropdown button at the end of the tab bar to show all tabs in a menu.

### Individual Tab Configuration

| Method                      | Description                                        |
| --------------------------- | -------------------------------------------------- |
| `new(id)`                   | Create a new tab bar with the given ID             |
| `child(tab)`                | Add a tab to the bar                               |
| `children(tabs)`            | Add multiple tabs to the bar                       |
| `selected_index(index)`     | Set the active tab index                           |
| `on_click(fn)`              | Callback when a tab is clicked, receives tab index |
| `prefix(element)`           | Add element before the tabs                        |
| `suffix(element)`           | Add element after the tabs                         |
| `last_empty_space(element)` | Custom element for empty space at the end          |
| `track_scroll(handle)`      | Enable scrolling with a scroll handle              |
| `with_menu(bool)`           | Enable dropdown menu for tab selection             |

| Method                  | Description                          |
| ----------------------- | ------------------------------------ |
| `with_variant(variant)` | Set the tab variant for all children |
| `underline()`           | Use underline variant                |
| `pill()`                | Use pill variant                     |
| `outline()`             | Use outline variant                  |
| `segmented()`           | Use segmented variant                |

| Method                  | Description                                    |
| ----------------------- | ---------------------------------------------- |
| `new(label)`            | Create a new tab with a label                  |
| `empty()`               | Create an empty tab                            |
| `icon(icon)`            | Create a tab with only an icon                 |
| `id(id)`                | Set custom ID for the tab                      |
| `with_variant(variant)` | Set the tab variant                            |
| `pill()`                | Use pill variant                               |
| `outline()`             | Use outline variant                            |
| `segmented()`           | Use segmented variant                          |
| `underline()`           | Use underline variant                          |
| `prefix(element)`       | Add element before tab content                 |
| `suffix(element)`       | Add element after tab content                  |
| `disabled(bool)`        | Set disabled state                             |
| `selected(bool)`        | Set selected state (usually handled by TabBar) |
| `on_click(fn)`          | Custom click handler for individual tab        |

Both `TabBar` and `Tab` implement `Sizable` trait:

* `xsmall()` - Extra small size
* `small()` - Small size
* `medium()` - Medium size (default)
* `large()` - Large size

### Custom Tab Content

### Tabs with State Management

### Tabs with Close Buttons

While the basic Tab component doesn't include closeable functionality, you can create closeable tabs using suffix elements:

* The `TabBar` manages the selection state of all child tabs
* Individual tab `on_click` handlers are ignored when `TabBar.on_click` is set
* Tabs automatically inherit the variant and size from their parent `TabBar`
* The `with_menu` option adds a dropdown for tab selection when there are many tabs
* Scrolling is automatically enabled when tabs overflow the container width
* The dock system provides advanced closeable tab functionality for complex layouts

---
url: /gpui-component/docs/components/tag.md
description: A short item that can be used to categorize or label content.
---

**Examples:**

Example 1 (rust):
```rust
use gpui_component::tab::{Tab, TabBar};
```

Example 2 (rust):
```rust
TabBar::new("tabs")
    .selected_index(0)
    .on_click(|selected_index, _, _| {
        println!("Tab {} selected", selected_index);
    })
    .child(Tab::new().label("Account"))
    .child(Tab::new().label("Profile"))
    .child(Tab::new().label("Settings"))
```

Example 3 (rust):
```rust
TabBar::new("default-tabs")
    .selected_index(0)
    .child(Tab::new().label("Account"))
    .child(Tab::new().label("Profile"))
    .child(Tab::new().label("Documents"))
```

Example 4 (rust):
```rust
TabBar::new("underline-tabs")
    .underline()
    .selected_index(0)
    .child(Tab::new().label("Account"))
    .child(Tab::new().label("Profile"))
    .child(Tab::new().label("Documents"))
```

---

## Editor

**URL:** llms-txt#editor

**Contents:**
- Import
- Usage
  - Textarea
  - AutoGrow
  - CodeEditor
  - TabSize
  - Searchable
  - SoftWrap
  - Text Manipulation
  - Validation

A powerful multi-line text input component that extends the basic input functionality with support for multiple lines, auto-resizing, syntax highlighting, line numbers, and code editing features. Perfect for forms, code editors, and content editing.

With fixed height Textarea:

GPUI Component's `InputState` supports a code editor mode with syntax highlighting, line numbers, and search functionality.

It design for high performance and can handle large files efficiently. We
used [tree-sitter](https://tree-sitter.github.io/tree-sitter/) for syntax highlighting, and [ropey](https://github.com/cessen/ropey) for text storage and manipulation.

#### Single Line Mode

Sometimes you may want to use the code editor features but restrict input to a single line, for example for code snippets or commands.

The search feature allows for all multi-line inputs to support searching through the content using `Ctrl+F` (or `Cmd+F` on Mac).

It provides a search bar with options to navigate between matches and highlight them.

Use `searchable` method to enable:

By default multi-line inputs have soft wrapping enabled, meaning long lines will wrap to fit the width of the textarea.

You can disable soft wrapping to allow horizontal scrolling instead:

### Text Manipulation

### Code Editor with Language Selection

### Text Editor with Toolbar

---
url: /gpui-component/docs/element_id.md
description: To introduce the ElementId concept in GPUI.
---

The [ElementId] is a unique identifier for a GPUI element. It is used to reference elements in the GPUI component tree.

Before you start using GPUI and GPUI Component, you need to understand the [ElementId].

In this case, the `div` element has an `id` of `"my-element"`. The add `id` is used for GPUI for binding events, for example `on_click` or `on_mouse_move`, the `element` with `id` in GPUI we call [Stateful\<E>][Stateful<E>].

We also use `id` (actually, it uses [GlobalElementId] internally in GPUI) to manage the `state` in some elements, by using `window.use_keyed_state`, so it is important to keep the `id` unique.

The `id` should be unique within the layout scope (In a same [Stateful\<E>][Stateful<E>] parent).

For example we have a list with multiple items:

In this case, we can named the child items with a very simple id, because they are have a parent `list1` element with an `id`.

GPUI internal will generate [GlobalElementId] with the parent elements's `id`, in this example, the `Item 1` will have global\_id:

And the `Item 1` in `list2` will have global\_id:

So we can named the child items with a very simple id.

[ElementId]: https://docs.rs/gpui/latest/gpui/enum.ElementId.html

[GlobalElementId]: https://docs.rs/gpui/latest/gpui/struct.GlobalElementId.html

[Stateful]: https://docs.rs/gpui/latest/gpui/struct.Stateful.html

[Stateful<E>]: https://docs.rs/gpui/latest/gpui/struct.Stateful.html

---
url: /gpui-component/docs/components/form.md
description: >-
  Flexible form container with support for field layout, validation, and
  multi-column layouts.
---

**Examples:**

Example 1 (rust):
```rust
use gpui_component::input::{InputState, Input};
```

Example 2 (rust):
```rust
let state = cx.new(|cx|
    InputState::new(window, cx)
        .multi_line(true)
        .placeholder("Enter your message...")
);

Input::new(&state)
```

Example 3 (rust):
```rust
let state = cx.new(|cx|
    InputState::new(window, cx)
        .multi_line(true)
        .rows(10) // Set number of rows
        .placeholder("Enter text here...")
);

Input::new(&state)
    .h(px(320.)) // Set explicit height
```

Example 4 (rust):
```rust
let state = cx.new(|cx|
    InputState::new(window, cx)
        .auto_grow(1, 5) // min_rows: 1, max_rows: 5
        .placeholder("Type here and watch it grow...")
);

Input::new(&state)
```

---

## Clipboard

**URL:** llms-txt#clipboard

**Contents:**
- Import
- Usage
  - Basic Clipboard
  - Using Dynamic Values
  - With Custom Content
  - In Input Fields
- API Reference
- Examples
  - Simple Text Copy
  - With User Feedback

The Clipboard component provides an easy way to copy text or other data to the user's clipboard. It renders as a button with a copy icon that changes to a checkmark when content is successfully copied. The component supports both static values and dynamic content through callback functions.

### Using Dynamic Values

The `value_fn` method allows you to provide a closure that generates the content to be copied at the time of the copy action.

* This is useful when the content to be copied depends on the current state of the application.
* And in some cases, it may have a larger overhead to compute, so you only want to do it when the user actually clicks the copy button.

### With Custom Content

The Clipboard component is commonly used as a suffix in input fields:

### With User Feedback

### Form Field Integration

### Dynamic Content Copy

The Clipboard component currently supports copying text strings to the clipboard. It uses GPUI's `ClipboardItem::new_string()` method, which handles:

* Plain text strings
* UTF-8 encoded content
* Cross-platform clipboard integration

[Clipboard]: https://docs.rs/gpui-component/latest/gpui_component/clipboard/struct.Clipboard.html

---
url: /gpui-component/docs/components/collapsible.md
description: An interactive element which expands/collapses.
---

**Examples:**

Example 1 (rust):
```rust
use gpui_component::clipboard::Clipboard;
```

Example 2 (rust):
```rust
Clipboard::new("my-clipboard")
    .value("Text to copy")
    .on_copied(|value, window, cx| {
        window.push_notification(format!("Copied: {}", value), cx)
    })
```

Example 3 (rust):
```rust
let state = some_state.clone();
Clipboard::new("dynamic-clipboard")
    .value_fn(move |_, cx| {
        state.read(cx).get_current_value()
    })
    .on_copied(|value, window, cx| {
        window.push_notification(format!("Copied: {}", value), cx)
    })
```

Example 4 (rust):
```rust
use gpui_component::label::Label;

 h_flex()
     .gap_2()
     .child(Label::new("Share URL"))
     .child(Icon::new(IconName::Share))
     .child(
        Clipboard::new("custom-clipboard")
        .value("https://example.com")
     )
```

---

## Popover

**URL:** llms-txt#popover

**Contents:**
- Import
- Usage
  - Basic Popover
  - Popover with Custom Positioning
  - View in Popover
  - Add content by `content` method
  - Right-Click Popover
  - Dismiss Popover manually
  - Styling Popover
  - Control Open State

Popover component for displaying floating content that appears when interacting with a trigger element. Supports multiple positioning options, custom content, different trigger methods, and automatic dismissal behaviors. Perfect for tooltips, menus, forms, and other contextual information.

:::info
Any element that implements [Selectable] can be used as a trigger, for example, a [Button].

Any element that implements [RenderOnce] or [Render] can be used as popover content, use `.child(...)` to add children directly.
:::

### Popover with Custom Positioning

The `anchor` method allows you to specify where the popover appears relative to the trigger element, this anchor point can be one of the four corners: TopLeft, TopRight, BottomLeft, BottomRight.

You can add any `Entity<T>` that implemented [Render] as the popover content.

### Add content by `content` method

The `content` method allows you to create more complex popover content using a closure. This is useful when
you need to build dynamic content or need access to the popover's context.

This method will let us to have `&mut PopoverState`, `&mut Window` and `&mut Context<PopoverState>` parameters in the
closure is to allow you to interact with the popover's state and the overall application context if needed.

:::warning
This `content` callback will called every time on render the popover.
So, you should avoid creating new elements or entities in the content closure
or other heavy operations that may impact performance.
:::

And `content` will works with `child`, `children` methods together.

### Right-Click Popover

Sometimes you may want to show a popover on right-click, for example, to create a special your ownen context menu. The `mouse_button` method allows you to specify which mouse button triggers the popover.

### Dismiss Popover manually

If you want to dismiss the popover programmatically from within the content, you can emit a `DismissEvent`. In this case, you should use `content` method to create the popover content so you have access to the `cx: &mut Context<PopoverState>`.

Like the others components in GPUI Component, the `appearance(false)` method can be used to disable the default styling of the popover, allowing you to fully customize its appearance.

And the `Popover` has implemented the [Styled] trait, so you can use all the styling methods provided by GPUI to style the popover content as you like.

### Control Open State

There have `open` and `on_open_change` methods to control the open state of the popover programmatically.

This is useful when you want to synchronize the popover's open state with other UI elements or application state.

:::tip
When you use `open` to control the popover's open state, that means you have take full control of it,
so you need to update the state in `on_open_change` callback to keep the popover working correctly.
:::

The `default_open` method allows you to set the initial open state of the popover when it is first rendered.

Please note that if you use the `open` method to control the popover's open state, the `default_open` setting will be ignored.

[Button]: https://docs.rs/gpui-component/latest/gpui_component/button/struct.Button.html

[Selectable]: https://docs.rs/gpui-component/latest/gpui_component/trait.Selectable.html

[Render]: https://docs.rs/gpui/latest/gpui/trait.Render.html

[RenderOnce]: https://docs.rs/gpui/latest/gpui/trait.RenderOnce.html

[Styled]: https://docs.rs/gpui/latest/gpui/trait.Styled.html

---
url: /gpui-component/docs/components/progress.md
description: >-
  Displays an indicator showing the completion progress of a task, typically
  displayed as a progress bar.
---

**Examples:**

Example 1 (rust):
```rust
use gpui_component::popover::{Popover};
```

Example 2 (rust):
```rust
use gpui::ParentElement as _;
use gpui_component::{button::Button, popover::Popover};

Popover::new("basic-popover")
    .trigger(Button::new("trigger").label("Click me").outline())
    .child("Hello, this is a popover!")
    .child("It appears when you click the button.")
```

Example 3 (rust):
```rust
use gpui::Corner;

Popover::new("positioned-popover")
    .anchor(Corner::TopRight)
    .trigger(Button::new("top-right").label("Top Right").outline())
    .child("This popover appears at the top right")
```

Example 4 (rust):
```rust
let view = cx.new(|_| MyView::new());

Popover::new("form-popover")
    .anchor(Corner::BottomLeft)
    .trigger(Button::new("show-form").label("Open Form").outline())
    .child(view.clone())
```

---

## Chart

**URL:** llms-txt#chart

**Contents:**
- Import
- Chart Types
  - LineChart
  - BarChart
  - AreaChart
  - PieChart
  - CandlestickChart
- Data Structures
  - Example Data Types
- Chart Configuration

A comprehensive charting library providing Line, Bar, Area, Pie, and Candlestick charts for data visualization. The charts feature smooth animations, customizable styling, tooltips, legends, and automatic theming that adapts to your application's theme.

A line chart displays data points connected by straight line segments, perfect for showing trends over time.

#### Basic Line Chart

#### Line Chart Variants

A bar chart uses rectangular bars to show comparisons among categories.

#### Bar Chart Customization

An area chart displays quantitative data visually, similar to a line chart but with the area below the line filled.

#### Basic Area Chart

#### Stacked Area Charts

#### Area Chart Styling

A pie chart displays data as slices of a circular chart, ideal for showing proportions.

#### Pie Chart Customization

A candlestick chart displays financial data using OHLC (Open, High, Low, Close) values, perfect for visualizing stock prices and market trends.

#### Basic Candlestick Chart

#### Candlestick Chart Customization

#### Candlestick Chart Colors

The candlestick chart automatically uses theme colors:

* **Bullish** (close > open): `bullish` color (green)
* **Bearish** (close < open): `bearish` color (red)

### Example Data Types

## Chart Configuration

### Theme Integration

* [LineChart]
* [BarChart]
* [AreaChart]
* [PieChart]
* [CandlestickChart]

### Multi-Series Time Chart

## Customization Options

### Responsive Design

### Grid and Axis Styling

Charts automatically include:

* Grid lines with dashed appearance
* X-axis labels with smart positioning
* Y-axis scaling starting from zero
* Responsive tick spacing based on `tick_margin`

## Performance Considerations

### Memory Optimization

## Integration Examples

### With State Management

### Real-time Updates

[LineChart]: https://docs.rs/gpui-component/latest/gpui_component/chart/struct.LineChart.html

[BarChart]: https://docs.rs/gpui-component/latest/gpui_component/chart/struct.BarChart.html

[AreaChart]: https://docs.rs/gpui-component/latest/gpui_component/chart/struct.AreaChart.html

[PieChart]: https://docs.rs/gpui-component/latest/gpui_component/chart/struct.PieChart.html

[CandlestickChart]: https://docs.rs/gpui-component/latest/gpui_component/chart/struct.CandlestickChart.html

---
url: /gpui-component/docs/components/checkbox.md
description: A control that allows the user to toggle between checked and not checked.
---

**Examples:**

Example 1 (rust):
```rust
use gpui_component::chart::{LineChart, BarChart, AreaChart, PieChart, CandlestickChart};
```

Example 2 (rust):
```rust
#[derive(Clone)]
struct DataPoint {
    x: String,
    y: f64,
}

let data = vec![
    DataPoint { x: "Jan".to_string(), y: 100.0 },
    DataPoint { x: "Feb".to_string(), y: 150.0 },
    DataPoint { x: "Mar".to_string(), y: 120.0 },
];

LineChart::new(data)
    .x(|d| d.x.clone())
    .y(|d| d.y)
```

Example 3 (rust):
```rust
// Basic curved line (default)
LineChart::new(data)
    .x(|d| d.month.clone())
    .y(|d| d.value)

// Linear interpolation
LineChart::new(data)
    .x(|d| d.month.clone())
    .y(|d| d.value)
    .linear()

// Step after interpolation
LineChart::new(data)
    .x(|d| d.month.clone())
    .y(|d| d.value)
    .step_after()

// With dots at data points
LineChart::new(data)
    .x(|d| d.month.clone())
    .y(|d| d.value)
    .dot()

// Custom stroke color
LineChart::new(data)
    .x(|d| d.month.clone())
    .y(|d| d.value)
    .stroke(cx.theme().success)
```

Example 4 (rust):
```rust
// Show every tick
LineChart::new(data)
    .x(|d| d.month.clone())
    .y(|d| d.value)
    .tick_margin(1)

// Show every 2nd tick
LineChart::new(data)
    .x(|d| d.month.clone())
    .y(|d| d.value)
    .tick_margin(2)
```

---

## TitleBar

**URL:** llms-txt#titlebar

**Contents:**
- Import
- Usage
  - Basic Title Bar
  - Title Bar with Custom Content
  - Title Bar with Menu Bar
  - Title Bar with Window Controls (Linux only)
  - Styled Title Bar
  - Title Bar Options for Window
- Platform Differences
  - macOS

TitleBar provides a customizable window title bar that can replace the default OS title bar. It includes platform-specific window controls (minimize, maximize, close) and supports custom content and styling. The component automatically adapts to different operating systems (macOS, Windows, Linux) with appropriate behaviors and visual styles.

### Title Bar with Custom Content

### Title Bar with Menu Bar

### Title Bar with Window Controls (Linux only)

### Title Bar Options for Window

## Platform Differences

* Uses native traffic light buttons (minimize, maximize, close)
* Traffic light position is automatically set to `(9px, 9px)`
* Double-click behavior calls `window.titlebar_double_click()`
* Left padding accounts for traffic light buttons (80px)
* Appears transparent by default

* Custom window control buttons with system integration
* Uses `WindowControlArea` for proper window management
* Control buttons have hover and active states
* Fixed button width of 34px each
* Left padding is 12px

* Custom window control buttons with manual event handling
* Supports custom close window callback via `on_close_window()`
* Double-click to maximize/restore window
* Right-click shows window context menu
* Window dragging supported in title bar area

| Method                | Description                              |
| --------------------- | ---------------------------------------- |
| `new()`               | Create a new title bar                   |
| `child(element)`      | Add child element to the title bar       |
| `on_close_window(fn)` | Custom close window handler (Linux only) |
| `title_bar_options()` | Get default titlebar options for window  |

### Window Configuration

| Property                 | Description                                         |
| ------------------------ | --------------------------------------------------- |
| `appears_transparent`    | Make title bar transparent (default: true)          |
| `traffic_light_position` | Position of macOS traffic lights                    |
| `title`                  | Window title (optional when using custom title bar) |

### Title Bar Element (Internal)

The `TitleBarElement` provides window dragging functionality on Linux platforms.

| Constant                 | Value                           | Description               |
| ------------------------ | ------------------------------- | ------------------------- |
| `TITLE_BAR_HEIGHT`       | `34px`                          | Standard title bar height |
| `TITLE_BAR_LEFT_PADDING` | `80px` (macOS), `12px` (others) | Left padding for content  |

### Application Title Bar

### Title Bar with Breadcrumbs

### Custom Themed Title Bar

### Title Bar with Status

### Minimal Title Bar

### Title Bar with Search

* The title bar automatically handles platform-specific styling and behavior
* Window controls are only rendered on Windows and Linux platforms
* The component integrates with GPUI's window management system
* Custom styling should consider platform conventions
* Window dragging is handled automatically in appropriate areas

---
url: /gpui-component/docs/components/toggle.md
description: A button-style toggle component for binary on/off or selected states.
---

**Examples:**

Example 1 (rust):
```rust
use gpui_component::TitleBar;
```

Example 2 (rust):
```rust
TitleBar::new()
    .child(div().child("My Application"))
```

Example 3 (rust):
```rust
TitleBar::new()
    .child(
        div()
            .flex()
            .items_center()
            .gap_3()
            .child("App Name")
            .child(Badge::new().count(5))
    )
    .child(
        div()
            .flex()
            .items_center()
            .gap_2()
            .child(Button::new("settings").icon(IconName::Settings))
            .child(Button::new("profile").icon(IconName::User))
    )
```

Example 4 (rust):
```rust
TitleBar::new()
    .child(
        div()
            .flex()
            .items_center()
            .child(AppMenuBar::new(window, cx))
    )
    .child(
        div()
            .flex()
            .items_center()
            .justify_end()
            .gap_2()
            .child(Button::new("github").icon(IconName::GitHub))
            .child(Button::new("notifications").icon(IconName::Bell))
    )
```

---

## ---

**URL:** llms-txt#---

url: /gpui-component/docs/components/accordion.md
description: The accordion uses collapse internally to make it collapsible.
---

---

## Tree

**URL:** llms-txt#tree

**Contents:**
- Import
- Usage
  - Basic Tree
  - File Tree with Icons
  - Dynamic Tree Loading
  - Tree with Selection Handling
  - Disabled Items
  - Programmatic Tree Control
- API Reference
  - TreeState

A versatile tree component for displaying hierarchical data with expand/collapse functionality, keyboard navigation, and custom item rendering. Perfect for file explorers, navigation menus, or any nested data structure.

### File Tree with Icons

### Dynamic Tree Loading

### Tree with Selection Handling

### Programmatic Tree Control

| Method                         | Description                  |
| ------------------------------ | ---------------------------- |
| `new(cx)`                      | Create a new tree state      |
| `items(items)`                 | Set initial tree items       |
| `set_items(items, cx)`         | Update tree items and notify |
| `selected_index()`             | Get currently selected index |
| `set_selected_index(ix, cx)`   | Set selected index           |
| `selected_entry()`             | Get currently selected entry |
| `scroll_to_item(ix, strategy)` | Scroll to specific item      |

| Method            | Description                            |
| ----------------- | -------------------------------------- |
| `new(id, label)`  | Create new tree item with ID and label |
| `child(item)`     | Add single child item                  |
| `children(items)` | Add multiple child items               |
| `expanded(bool)`  | Set expanded state                     |
| `disabled(bool)`  | Set disabled state                     |
| `is_folder()`     | Check if item has children             |
| `is_expanded()`   | Check if item is expanded              |
| `is_disabled()`   | Check if item is disabled              |

| Method          | Description                 |
| --------------- | --------------------------- |
| `item()`        | Get the source TreeItem     |
| `depth()`       | Get item depth in tree      |
| `is_folder()`   | Check if entry has children |
| `is_expanded()` | Check if entry is expanded  |
| `is_disabled()` | Check if entry is disabled  |

| Parameter     | Description                           |
| ------------- | ------------------------------------- |
| `state`       | `Entity<TreeState>` for managing tree |
| `render_item` | Closure for rendering each item       |

#### Render Item Closure

* `usize`: Item index in flattened tree
* `&TreeEntry`: Tree entry with item and metadata
* `bool`: Whether item is currently selected
* `&mut Window`: Current window context
* `&mut App`: Application context
* Returns: `ListItem` for rendering

### Lazy Loading Tree

### Search and Filter

### Multi-Select Tree

## Keyboard Navigation

The Tree component supports comprehensive keyboard navigation:

| Key     | Action                                    |
| ------- | ----------------------------------------- |
| `↑`     | Select previous item                      |
| `↓`     | Select next item                          |
| `←`     | Collapse current folder or move to parent |
| `→`     | Expand current folder                     |
| `Enter` | Toggle expand/collapse for folders        |
| `Space` | Custom action (configurable)              |

---
url: /gpui-component/contributors.md
---

---
url: /gpui-component/docs/components/virtual-list.md
description: >-
  High-performance virtualized list component for rendering large datasets with
  variable item sizes.
---

**Examples:**

Example 1 (rust):
```rust
use gpui_component::tree::{tree, TreeState, TreeItem, TreeEntry};
```

Example 2 (rust):
```rust
// Create tree state
let tree_state = cx.new(|cx| {
    TreeState::new(cx).items(vec![
        TreeItem::new("src", "src")
            .expanded(true)
            .child(TreeItem::new("src/lib.rs", "lib.rs"))
            .child(TreeItem::new("src/main.rs", "main.rs")),
        TreeItem::new("Cargo.toml", "Cargo.toml"),
        TreeItem::new("README.md", "README.md"),
    ])
});

// Render tree
tree(&tree_state, |ix, entry, selected, window, cx| {
    ListItem::new(ix)
        .child(
            h_flex()
                .gap_2()
                .child(entry.item().label.clone())
        )
})
```

Example 3 (rust):
```rust
use gpui_component::{ListItem, IconName, h_flex};

tree(&tree_state, |ix, entry, selected, window, cx| {
    let item = entry.item();
    let icon = if !entry.is_folder() {
        IconName::File
    } else if entry.is_expanded() {
        IconName::FolderOpen
    } else {
        IconName::Folder
    };

    ListItem::new(ix)
        .selected(selected)
        .pl(px(16.) * entry.depth() + px(12.)) // Indent based on depth
        .child(
            h_flex()
                .gap_2()
                .child(icon)
                .child(item.label.clone())
        )
        .on_click(cx.listener(move |_, _, _, _| {
            // Handle item click
        }))
})
```

Example 4 (rust):
```rust
impl MyView {
    fn load_files(&mut self, path: PathBuf, cx: &mut Context<Self>) {
        let tree_state = self.tree_state.clone();
        cx.spawn(async move |cx| {
            let items = build_file_items(&path).await;
            tree_state.update(cx, |state, cx| {
                state.set_items(items, cx);
            })
        }).detach();
    }
}

fn build_file_items(path: &Path) -> Vec<TreeItem> {
    let mut items = Vec::new();
    if let Ok(entries) = std::fs::read_dir(path) {
        for entry in entries.flatten() {
            let path = entry.path();
            let name = path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("Unknown")
                .to_string();

            if path.is_dir() {
                let children = build_file_items(&path);
                items.push(TreeItem::new(path.to_string_lossy(), name)
                    .children(children));
            } else {
                items.push(TreeItem::new(path.to_string_lossy(), name));
            }
        }
    }
    items
}
```

---

## Input

**URL:** llms-txt#input

**Contents:**
- Import
- Usage
  - Basic Input
  - With Placeholder
  - With Default Value
  - Cleanable Input
  - With Prefix and Suffix
  - Password Input (Masked)
  - Input Sizes
  - Disabled Input

A flexible text input component with support for validation, masking, prefix/suffix elements, and different states.

### With Default Value

### With Prefix and Suffix

### Password Input (Masked)

### Handle Input Events

### Custom Appearance

### Form with Multiple Inputs

---
url: /gpui-component/docs/installation.md
---

**Examples:**

Example 1 (rust):
```rust
use gpui_component::input::{InputState, Input};
```

Example 2 (rust):
```rust
let input = cx.new(|cx| InputState::new(window, cx));

Input::new(&input)
```

Example 3 (rust):
```rust
let input = cx.new(|cx|
    InputState::new(window, cx)
        .placeholder("Enter your name...")
);

Input::new(&input)
```

Example 4 (rust):
```rust
let input = cx.new(|cx|
    InputState::new(window, cx)
        .default_value("John Doe")
);

Input::new(&input)
```

---

## Resizable

**URL:** llms-txt#resizable

**Contents:**
- Import
- Usage
  - Panel Size Constraints
  - Multiple Panels
  - Nested Layouts
  - Nested Panel Groups
  - Conditional Panel Visibility
  - Panel with Size Limits
- Examples
  - File Explorer Layout

The resizable component system provides a flexible way to create layouts with resizable panels. It supports both horizontal and vertical resizing, nested layouts, size constraints, and drag handles. Perfect for creating paned interfaces, split views, and adjustable dashboards.

Use `h_resizable` to create a horizontal layout, `v_resizable` to create a vertical layout.

The first argument is the `id` for this \[ResizablePanelGroup].

:::tip
In GPUI, the `id` must be unique within the layout scope (The nearest parent has presents `id`).
:::

The `v_resizable` component is used to create a vertical layout.

### Panel Size Constraints

### Nested Panel Groups

### Conditional Panel Visibility

### Panel with Size Limits

### File Explorer Layout

### Dashboard with Widgets

1. **State Management**: Use separate ResizableState for independent layouts
2. **Size Constraints**: Always set reasonable min/max sizes for panels
3. **Event Handling**: Subscribe to ResizablePanelEvent for layout persistence
4. **Nested Layouts**: Use `.group()` method for clean nested structures
5. **Performance**: Avoid excessive nesting for better performance
6. **User Experience**: Provide adequate handle padding for easier interaction

---
url: /gpui-component/docs/root.md
---

**Examples:**

Example 1 (rust):
```rust
use gpui_component::resizable::{
    h_resizable, v_resizable, resizable_panel,
    ResizablePanelGroup, ResizablePanel, ResizableState, ResizablePanelEvent
};
```

Example 2 (rust):
```rust
h_resizable("my-layout")
    .on_resize(|state, window, cx| {
        // Handle resize event
        // You can read the panel sizes from the state.
        let state = state.read(cx);
        let sizes = state.sizes();
    })
    .child(
        // Use resizable_panel() to create a sized panel.
        resizable_panel()
            .size(px(200.))
            .child("Left Panel")
    )
    .child(
        // Or you can just add AnyElement without a size.
        div()
            .child("Right Panel")
            .into_any_element()
    )
```

Example 3 (rust):
```rust
v_resizable("vertical-layout")
    .child(
        resizable_panel()
            .size(px(100.))
            .child("Top Panel")
    )
    .child(
        div()
            .child("Bottom Panel")
            .into_any_element()
    )
```

Example 4 (rust):
```rust
resizable_panel()
    .size(px(200.))                    // Initial size
    .size_range(px(150.)..px(400.))    // Min and max size
    .child("Constrained Panel")
```

---

## Table

**URL:** llms-txt#table

**Contents:**
- Import
- Usage
  - Basic Table
  - Column Configuration
  - Virtual Scrolling for Large Datasets
  - Sorting Implementation
  - ContextMenu
  - Cell Rendering
  - Column Resizing and Moving
  - Infinite Loading / Pagination

A comprehensive data table component designed for handling large datasets with high performance. Features virtual scrolling, column configuration, sorting, filtering, row selection, and custom cell rendering. Perfect for displaying tabular data with thousands of rows while maintaining smooth performance.

To create a table, you need to implement the `TableDelegate` trait and provide column definitions, and use `TableState` to manage the table state.

### Column Configuration

Columns provide extensive configuration options:

### Virtual Scrolling for Large Datasets

The table automatically handles virtual scrolling for optimal performance:

### Sorting Implementation

Implement sorting in your delegate:

Create rich cell content with custom rendering:

### Column Resizing and Moving

Enable dynamic column management:

### Infinite Loading / Pagination

Implement loading more data as user scrolls:

Customize table appearance:

### Financial Data Table

### User Management Table

## Keyboard shortcuts

* `↑/↓` - Navigate rows
* `←/→` - Navigate columns
* `Enter/Space` - Select row/column
* `Escape` - Clear selection

---
url: /gpui-component/docs/components/tabs.md
description: >-
  A set of layered sections of content—known as tab panels—that are displayed
  one at a time.
---

**Examples:**

Example 1 (rust):
```rust
use gpui_component::table::{Table, TableState, TableDelegate, Column, ColumnSort, ColumnFixed, TableEvent};
```

Example 2 (rust):
```rust
use std::ops::Range;
use gpui::{App, Context, Window, IntoElement};
use gpui_component::table::{Table, TableDelegate, Column, ColumnSort};

struct MyData {
    id: usize,
    name: String,
    age: u32,
    email: String,
}

struct MyTableDelegate {
    data: Vec<MyData>,
    columns: Vec<Column>,
}

impl MyTableDelegate {
    fn new() -> Self {
        Self {
            data: vec![
                MyData { id: 1, name: "John".to_string(), age: 30, email: "john@example.com".to_string() },
                MyData { id: 2, name: "Jane".to_string(), age: 25, email: "jane@example.com".to_string() },
            ],
            columns: vec![
                Column::new("id", "ID").width(60.),
                Column::new("name", "Name").width(150.).sortable(),
                Column::new("age", "Age").width(80.).sortable(),
                Column::new("email", "Email").width(200.),
            ],
        }
    }
}

impl TableDelegate for MyTableDelegate {
    fn columns_count(&self, _: &App) -> usize {
        self.columns.len()
    }

    fn rows_count(&self, _: &App) -> usize {
        self.data.len()
    }

    fn column(&self, col_ix: usize, _: &App) -> &Column {
        &self.columns[col_ix]
    }

    fn render_td(&mut self, row_ix: usize, col_ix: usize, _: &mut Window, _: &mut Context<TableState<Self>>) -> impl IntoElement {
        let row = &self.data[row_ix];
        let col = &self.columns[col_ix];

        match col.key.as_ref() {
            "id" => row.id.to_string(),
            "name" => row.name.clone(),
            "age" => row.age.to_string(),
            "email" => row.email.clone(),
            _ => "".to_string(),
        }
    }
}

// Create the table
let delegate = MyTableDelegate::new();
let state = cx.new(|cx| TableState::new(delegate, window, cx));
```

Example 3 (rust):
```rust
// Basic column
Column::new("id", "ID")

// Sortable column
Column::new("name", "Name")
    .sortable()
    .width(150.)

// Right-aligned column
Column::new("price", "Price")
    .text_right()
    .sortable()

// Fixed column (pinned to left)
Column::new("actions", "Actions")
    .fixed(ColumnFixed::Left)
    .resizable(false)
    .movable(false)

// Column with custom padding
Column::new("description", "Description")
    .width(200.)
    .paddings(px(8.))

// Non-resizable column
Column::new("status", "Status")
    .width(100.)
    .resizable(false)

// Custom sort orders
Column::new("created", "Created")
    .ascending() // Default ascending
// or
Column::new("modified", "Modified")
    .descending() // Default descending
```

Example 4 (rust):
```rust
struct LargeDataDelegate {
    data: Vec<Record>, // Could be 10,000+ items
    columns: Vec<Column>,
}

impl TableDelegate for LargeDataDelegate {
    fn rows_count(&self, _: &App) -> usize {
        self.data.len() // No performance impact regardless of size
    }

    // Only visible rows are rendered
    fn render_td(&mut self, row_ix: usize, col_ix: usize, _: &mut Window, _: &mut Context<TableState<Self>>) -> impl IntoElement {
        // This is only called for visible rows
        // Efficiently render cell content
        let row = &self.data[row_ix];
        format_cell_data(row, col_ix)
    }

    // Track visible range for optimizations
    fn visible_rows_changed(&mut self, visible_range: Range<usize>, _: &mut Window, _: &mut Context<TableState<Self>>) {
        // Only update data for visible rows if needed
        // This is called when user scrolls
    }
}
```

---

## Button

**URL:** llms-txt#button

**Contents:**
- Import
- Usage
  - Basic Button
  - Variants
  - Outline Buttons
  - Compact Button
  - Sizeable
  - With Icons
  - With a dropdown caret icon
  - Button States

The [Button] element with multiple variants, sizes, and states. Supports icons, loading states, and can be grouped together.

Outline style is not a variant itself, but can be combined with other variants.

The `compact` method reduces the padding of the button for a more condensed appearance.

The Button supports the [Sizable] trait for different sizes.

### With a dropdown caret icon

The `.dropdown_caret` method can allows adding a dropdown caret icon to end of the button.

There have `disabled`, `loading`, `selected` state for buttons to indicate different statuses.

### Toggle Button Group

* [Button]
* [ButtonGroup]
* [ButtonCustomVariant]

[Button]: https://docs.rs/gpui-component/latest/gpui_component/button/struct.Button.html

[ButtonGroup]: https://docs.rs/gpui-component/latest/gpui_component/button/struct.ButtonGroup.html

[ButtonCustomVariant]: https://docs.rs/gpui-component/latest/gpui_component/button/struct.ButtonCustomVariant.html

[Sizable]: https://docs.rs/gpui-component/latest/gpui_component/trait.Sizable.html

---
url: /gpui-component/docs/components/calendar.md
description: >-
  A flexible calendar component for displaying months, navigating dates, and
  selecting single dates or date ranges.
---

**Examples:**

Example 1 (rust):
```rust
use gpui_component::button::{Button, ButtonGroup};
```

Example 2 (rust):
```rust
Button::new("my-button")
    .label("Click me")
    .on_click(|_, _, _| {
        println!("Button clicked!");
    })
```

Example 3 (rust):
```rust
// Primary button
Button::new("btn-primary").primary().label("Primary")

// Secondary button (default)
Button::new("btn-secondary").label("Secondary")

// Danger button
Button::new("btn-danger").danger().label("Delete")

// Warning button
Button::new("btn-warning").warning().label("Warning")

// Success button
Button::new("btn-success").success().label("Success")

// Info button
Button::new("btn-info").info().label("Info")

// Ghost button
Button::new("btn-ghost").ghost().label("Ghost")

// Link button
Button::new("btn-link").link().label("Link")

// Text button
Button::new("btn-text").text().label("Text")
```

Example 4 (rust):
```rust
Button::new("btn").primary().outline().label("Primary Outline")
Button::new("btn").danger().outline().label("Danger Outline")
```

---

---
url: /gpui-component/docs/components/accordion.md
description: The accordion uses collapse internally to make it collapsible.
---

# Accordion

An accordion component that allows users to show and hide sections of content. It uses collapse functionality internally to create collapsible panels.

## Import

```rust
use gpui_component::accordion::Accordion;
```

## Usage

### Basic Accordion

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

### Multiple Open Items

By default, only one accordion item can be open at a time. Use `multiple()` to allow multiple items to be open:

```rust
Accordion::new("my-accordion")
    .multiple(true)
    .item(|item| item.title("Section 1").child("Content 1"))
    .item(|item| item.title("Section 2").child("Content 2"))
```

### With Borders

```rust
Accordion::new("my-accordion")
    .bordered(true)
    .item(|item| item.title("Section 1").child("Content 1"))
```

### Different Sizes

```rust
use gpui_component::{Sizable as _, Size};

Accordion::new("my-accordion")
    .small()
    .item(|item| item.title("Small Section").child("Content"))

Accordion::new("my-accordion")
    .large()
    .item(|item| item.title("Large Section").child("Content"))
```

### Handle Toggle Events

```rust
Accordion::new("my-accordion")
    .on_toggle_click(|open_indices, window, cx| {
        println!("Open items: {:?}", open_indices);
    })
    .item(|item| item.title("Section 1").child("Content 1"))
```

### Disabled State

```rust
Accordion::new("my-accordion")
    .disabled(true)
    .item(|item| item.title("Disabled Section").child("Content"))
```

## API Reference

* [Accordion]
* [AccordionItem]

### Sizing

Implements [Sizable] trait:

* `small()` - Small size
* `medium()` - Medium size (default)
* `large()` - Large size
* `xsmall()` - Extra small size

## Examples

### With Custom Icons

```rust
Accordion::new("my-accordion")
    .item(|item| {
        item.title(
            h_flex()
                .gap_2()
                .child(Icon::new(IconName::Settings))
                .child("Settings")
        )
        .child("Settings content here")
    })
```

### Nested Accordions

```rust
Accordion::new("outer")
    .item(|item| {
        item.title("Parent Section")
            .content(
                Accordion::new("inner")
                    .item(|item| item.title("Child 1").child("Content"))
                    .item(|item| item.title("Child 2").child("Content"))
            )
    })
```

[Accordion]: https://docs.rs/gpui-component/latest/gpui_component/accordion/struct.Accordion.html

[AccordionItem]: https://docs.rs/gpui-component/latest/gpui_component/accordion/struct.AccordionItem.html

[Sizable]: https://docs.rs/gpui-component/latest/gpui_component/trait.Sizable.html

---

---
url: /gpui-component/docs/components/alert.md
description: Displays a callout for user attention.
---

# Alert

A versatile alert component for displaying important messages to users. Supports multiple variants (info, success, warning, error), custom icons, optional titles, closable functionality, and banner mode. Perfect for notifications, status messages, and user feedback.

## Import

```rust
use gpui_component::alert::Alert;
```

## Usage

### Basic Alert

```rust
Alert::new("alert-id", "This is a basic alert message.")
```

### Alert with Title

```rust
Alert::new("alert-with-title", "Your changes have been saved successfully.")
    .title("Success!")
```

### Alert Variants

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

### Alert Sizes

```rust
use gpui_component::{alert::Alert, Sizable as _};

Alert::info("alert", "Message content")
    .xsmall()
    .title("XSmall Alert")
Alert::info("alert", "Message content")
    .small()
    .title("Small Alert")

Alert::info("alert", "Message content")
    .title("Medium Alert")

Alert::info("alert", "Message content")
    .large()
    .title("Large Alert")
```

### Closable Alerts

When you add an `on_close` handler, a close button appears on the alert:

```rust
Alert::info("closable-alert", "This alert can be dismissed.")
    .title("Dismissible")
    .on_close(|_event, _window, _cx| {
        println!("Alert was closed");
        // Handle alert dismissal
    })
```

### Banner Mode

Banner alerts take full width and don't display titles:

```rust
Alert::info("banner-alert", "This is a banner alert that spans the full width.")
    .banner()

Alert::success("banner-success", "Operation completed successfully!")
    .banner()

Alert::warning("banner-warning", "System maintenance scheduled for tonight.")
    .banner()

Alert::error("banner-error", "Service temporarily unavailable.")
    .banner()
```

### Custom Icons

```rust
use gpui_component::IconName;

Alert::new("custom-icon", "Meeting scheduled for tomorrow at 3 PM.")
    .title("Calendar Reminder")
    .icon(IconName::Calendar)
```

### With Markdown Content

We can use `TextView` to render formatted (Markdown or HTML) text within the alert,
for displaying lists, bold text, links, etc.

```rust
use gpui_component::text::markdown;

Alert::error(
    "error-with-markdown",
    markdown(
        "Please verify your billing information and try again.\n\
        - Check your card details\n\
        - Ensure sufficient funds\n\
        - Verify billing address"
    ),
)
.title("Payment Failed")
```

### Conditional Visibility

```rust
Alert::info("conditional-alert", "This alert may be hidden.")
    .title("Conditional")
    .visible(should_show_alert) // boolean condition
```

## API Reference

* [Alert]

## Examples

### Form Validation Errors

```rust
Alert::error(
    "validation-error",
    "Please correct the following errors before submitting:\n\
    - Email address is required\n\
    - Password must be at least 8 characters\n\
    - Terms of service must be accepted"
)
.title("Validation Failed")
```

### Success Notification

```rust
Alert::success("save-success", "Your profile has been updated successfully.")
    .title("Changes Saved")
    .on_close(|_, _, _| {
        // Auto-dismiss after showing
    })
```

### System Status Banner

```rust
Alert::warning(
    "maintenance-banner",
    "Scheduled maintenance will occur tonight from 2:00 AM to 4:00 AM EST. \
    Some services may be temporarily unavailable."
)
.banner()
.large()
```

### Interactive Alert with Custom Action

```rust
Alert::info("update-available", "A new version of the application is available.")
    .title("Update Available")
    .icon(IconName::Download)
    .on_close(cx.listener(|this, _, _, cx| {
        // Handle update or dismiss
        this.handle_update_notification(cx);
    }))
```

### Multi-line Content with Formatting

```rust
use gpui_component::text::markdown;

Alert::warning(
    "security-alert",
    markdown(
        "**Security Notice**: Unusual activity detected on your account.\n\n\
        Recent activity:\n\
        - Login from new device (Chrome on Windows)\n\
        - Location: San Francisco, CA\n\
        - Time: Today at 2:30 PM\n\n\
        If this wasn't you, please [change your password](/) immediately."
    )
)
.title("Security Alert")
.icon(IconName::Shield)
```

[Alert]: https://docs.rs/gpui-component/latest/gpui_component/alert/struct.Alert.html

---

---
url: /gpui-component/docs/components/avatar.md
description: Displays a user avatar image with fallback options.
---

# Avatar

The Avatar component displays user profile images with intelligent fallbacks. When no image is provided, it shows user initials or a placeholder icon. The component supports various sizes and can be grouped together for team displays.

## Import

```rust
use gpui_component::avatar::{Avatar, AvatarGroup};
```

## Usage

### Basic Avatar

You can create an [Avatar] by providing an image source URL and a user name:

```rust
Avatar::new()
    .name("John Doe")
    .src("https://example.com/avatar.jpg")
```

### Avatar with Fallback Text

When no image source is provided, the Avatar displays user initials with an automatically generated color background:

```rust
// Shows "JD" initials with colored background
Avatar::new()
    .name("John Doe")

// Shows "JS" initials
Avatar::new()
    .name("Jane Smith")
```

### Avatar Placeholder

For anonymous users or when no name is provided:

```rust
use gpui_component::IconName;

// Default user icon placeholder
Avatar::new()

// Custom placeholder icon
Avatar::new()
    .placeholder(IconName::Building2)
```

### Avatar Sizes

```rust
Avatar::new()
    .name("John Doe")
    .xsmall()

Avatar::new()
    .name("John Doe")
    .small()

Avatar::new()
    .name("John Doe")   // 48px (default medium)

Avatar::new()
    .name("John Doe")
    .large()

// Custom size
Avatar::new()
    .name("John Doe")
    .with_size(px(100.))
```

### Custom Styling

```rust
Avatar::new()
    .src("https://example.com/avatar.jpg")
    .with_size(px(100.))
    .border_3()
    .border_color(cx.theme().foreground)
    .shadow_sm()
    .rounded(px(20.))  // Custom border radius
```

## AvatarGroup

The [AvatarGroup] component allows you to display multiple avatars in a compact, overlapping layout:

### Basic Group

```rust
AvatarGroup::new()
    .child(Avatar::new().src("https://example.com/user1.jpg"))
    .child(Avatar::new().src("https://example.com/user2.jpg"))
    .child(Avatar::new().src("https://example.com/user3.jpg"))
    .child(Avatar::new().name("John Doe"))
```

### Group with Limit

```rust
AvatarGroup::new()
    .limit(3)  // Show maximum 3 avatars
    .child(Avatar::new().src("https://example.com/user1.jpg"))
    .child(Avatar::new().src("https://example.com/user2.jpg"))
    .child(Avatar::new().src("https://example.com/user3.jpg"))
    .child(Avatar::new().src("https://example.com/user4.jpg"))  // Hidden
    .child(Avatar::new().src("https://example.com/user5.jpg"))  // Hidden
```

### Group with Ellipsis

Show an ellipsis indicator when avatars are hidden due to the limit.

In this example, only 3 avatars are shown, and "..." indicates there are more:

```rust
AvatarGroup::new()
    .limit(3)
    .ellipsis()  // Shows "..." when limit is exceeded
    .child(Avatar::new().src("https://example.com/user1.jpg"))
    .child(Avatar::new().src("https://example.com/user2.jpg"))
    .child(Avatar::new().src("https://example.com/user3.jpg"))
    .child(Avatar::new().src("https://example.com/user4.jpg"))
    .child(Avatar::new().src("https://example.com/user5.jpg"))
```

### Group Sizes

The \[Sizeable] trait can also be applied to AvatarGroup, and it will set the size for all contained avatars.

```rust
// Extra small group
AvatarGroup::new()
    .xsmall()
    .child(Avatar::new().name("A"))
    .child(Avatar::new().name("B"))
    .child(Avatar::new().name("C"))

// Small group
AvatarGroup::new()
    .small()
    .child(Avatar::new().name("A"))
    .child(Avatar::new().name("B"))

// Medium group (default)
AvatarGroup::new()
    .child(Avatar::new().name("A"))
    .child(Avatar::new().name("B"))

// Large group
AvatarGroup::new()
    .large()
    .child(Avatar::new().name("A"))
    .child(Avatar::new().name("B"))
```

### Adding Multiple Avatars

```rust
let avatars = vec![
    Avatar::new().src("https://example.com/user1.jpg"),
    Avatar::new().src("https://example.com/user2.jpg"),
    Avatar::new().name("John Doe"),
];

AvatarGroup::new()
    .children(avatars)
    .limit(5)
    .ellipsis()
```

## API Reference

* [Avatar]
* [AvatarGroup]

## Examples

### Team Display

```rust
use gpui_component::{h_flex, v_flex};

v_flex()
    .gap_4()
    .child("Development Team")
    .child(
        AvatarGroup::new()
            .limit(4)
            .ellipsis()
            .child(Avatar::new().name("Alice Johnson").src("https://example.com/alice.jpg"))
            .child(Avatar::new().name("Bob Smith").src("https://example.com/bob.jpg"))
            .child(Avatar::new().name("Charlie Brown"))
            .child(Avatar::new().name("Diana Prince"))
            .child(Avatar::new().name("Eve Wilson"))
    )
```

### User Profile Header

```rust
h_flex()
    .items_center()
    .gap_4()
    .child(
        Avatar::new()
            .src("https://example.com/profile.jpg")
            .name("John Doe")
            .large()
            .border_2()
            .border_color(cx.theme().primary)
    )
    .child(
        v_flex()
            .child("John Doe")
            .child("Software Engineer")
    )
```

### Anonymous User

```rust
use gpui_component::IconName;

Avatar::new()
    .placeholder(IconName::UserCircle)
    .medium()
```

### Avatar with Custom Colors

```rust
// The avatar automatically generates colors based on the name
// Different names will get different colors from the color palette
Avatar::new().name("Alice")    // Gets one color
Avatar::new().name("Bob")      // Gets a different color
Avatar::new().name("Charlie")  // Gets another color
```

[Avatar]: https://docs.rs/gpui-component/latest/gpui_component/avatar/struct.Avatar.html

[AvatarGroup]: https://docs.rs/gpui-component/latest/gpui_component/avatar/struct.AvatarGroup.html

[Sizable]: https://docs.rs/gpui-component/latest/gpui_component/trait.Sizable.html

---

---
url: /gpui-component/docs/components/badge.md
description: >-
  A red dot that indicates the number of unread messages, status, or other
  notifications.
---

# Badge

A versatile badge component that can display counts, dots, or icons on elements. Perfect for indicating notifications, status, or other contextual information on avatars, icons, or other UI elements.

## Import

```rust
use gpui_component::badge::Badge;
```

## Usage

### Badge with Count

Use `count` to display a numeric badge, if the count is greater than zero (`> 0`) the badge will be shown, otherwise it will be hidden.

There is a default maximum count of `99`, any count above this will be displayed as `99+`. You can customize this maximum using the [max](https://docs.rs/gpui-component/latest/gpui_component/badge/struct.Badge.html#method.max) method.

```rust
Badge::new()
    .count(3)
    .child(Icon::new(IconName::Bell))
```

### Variants

* Default: Displays a numeric count.
* Dot: A small dot indicator, typically used for status.
* Icon: Displays an icon instead of a number.

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

### Badge Sizes

The Badge is also implemented with the [Sizable] trait, allowing you to set small, medium (default), or large sizes.

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

### Badge Colors

```rust
use gpui_component::ActiveTheme;

// Custom colors
Badge::new()
    .count(3)
    .color(cx.theme().blue)
    .child(Avatar::new())

Badge::new()
    .icon(IconName::Star)
    .color(cx.theme().yellow)
    .child(Avatar::new())

Badge::new()
    .dot()
    .color(cx.theme().green)
    .child(Icon::new(IconName::Bell))
```

### Badge on Icons

```rust
use gpui_component::{Icon, IconName};

// Badge with count on icon
Badge::new()
    .count(3)
    .child(Icon::new(IconName::Bell).large())

// Badge with high count (shows max)
Badge::new()
    .count(103)
    .child(Icon::new(IconName::Inbox).large())

// Custom max count
Badge::new()
    .count(150)
    .max(999)
    .child(Icon::new(IconName::Mail))
```

### Badge on Avatars

```rust
use gpui_component::avatar::Avatar;

// Basic count badge
Badge::new()
    .count(5)
    .child(Avatar::new().src("https://example.com/avatar.jpg"))

// Status badge with icon
Badge::new()
    .icon(IconName::Check)
    .color(cx.theme().green)
    .child(Avatar::new().src("https://example.com/avatar.jpg"))

// Online indicator with dot
Badge::new()
    .dot()
    .color(cx.theme().green)
    .child(Avatar::new().src("https://example.com/avatar.jpg"))
```

### Complex Nested Badges

```rust
// Badge on badge for complex status
Badge::new()
    .count(212)
    .large()
    .child(
        Badge::new()
            .icon(IconName::Check)
            .large()
            .color(cx.theme().cyan)
            .child(Avatar::new().large().src("https://example.com/avatar.jpg"))
    )

// Multiple status indicators
Badge::new()
    .count(2)
    .color(cx.theme().green)
    .large()
    .child(
        Badge::new()
            .icon(IconName::Star)
            .large()
            .color(cx.theme().yellow)
            .child(Avatar::new().large().src("https://example.com/avatar.jpg"))
    )
```

## API Reference

* [Badge]

## Examples

### Notification Indicators

```rust
// Unread messages
Badge::new()
    .count(12)
    .child(Icon::new(IconName::Mail).large())

// New notifications
Badge::new()
    .count(3)
    .color(cx.theme().red)
    .child(Icon::new(IconName::Bell).large())

// High priority with custom max
Badge::new()
    .count(1234)
    .max(999)
    .color(cx.theme().orange)
    .child(Icon::new(IconName::AlertTriangle))
```

### Status Indicators

```rust
// Online status
Badge::new()
    .dot()
    .color(cx.theme().green)
    .child(Avatar::new().src("https://example.com/user.jpg"))

// Verified status
Badge::new()
    .icon(IconName::CheckCircle)
    .color(cx.theme().blue)
    .child(Avatar::new().src("https://example.com/verified-user.jpg"))

// Warning status
Badge::new()
    .icon(IconName::AlertTriangle)
    .color(cx.theme().yellow)
    .child(Avatar::new().src("https://example.com/user.jpg"))
```

### Different Badge Positions

```rust
// The badge automatically positions itself based on variant:
// - Dot: top-right corner (small dot)
// - Number: top-right with dynamic sizing
// - Icon: bottom-right corner with border
```

### Count Formatting

```rust
// Numbers 1-99 show as-is
Badge::new().count(5)    // Shows "5"
Badge::new().count(99)   // Shows "99"

// Numbers above max show with "+"
Badge::new().count(100)  // Shows "99+" (default max)
Badge::new().count(1000).max(999) // Shows "999+"

// Zero count hides the badge
Badge::new().count(0)    // Badge not visible
```

[Badge]: https://docs.rs/gpui_component/latest/gpui_component/badge/struct.Badge.html

[Sizable]: https://docs.rs/gpui-component/latest/gpui_component/trait.Sizable.html

---

---
url: /gpui-component/docs/components/button.md
description: Displays a button or a component that looks like a button.
---

# Button

The [Button] element with multiple variants, sizes, and states. Supports icons, loading states, and can be grouped together.

## Import

```rust
use gpui_component::button::{Button, ButtonGroup};
```

## Usage

### Basic Button

```rust
Button::new("my-button")
    .label("Click me")
    .on_click(|_, _, _| {
        println!("Button clicked!");
    })
```

### Variants

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

### Outline Buttons

Outline style is not a variant itself, but can be combined with other variants.

```rust
Button::new("btn").primary().outline().label("Primary Outline")
Button::new("btn").danger().outline().label("Danger Outline")
```

### Compact Button

The `compact` method reduces the padding of the button for a more condensed appearance.

```rust
// Compact (reduced padding)
Button::new("btn")
    .label("Compact")
    .compact()
```

### Sizeable

The Button supports the [Sizable] trait for different sizes.

```rust
Button::new("btn").xsmall().label("Extra Small")
Button::new("btn").small().label("Small")
Button::new("btn").label("Medium") // default
Button::new("btn").large().label("Large")
```

### With Icons

```rust
use gpui_component::{Icon, IconName};

// Icon before label
Button::new("btn")
    .icon(IconName::Check)
    .label("Confirm")

// Icon only
Button::new("btn")
    .icon(IconName::Search)

// Custom icon size
Button::new("btn")
    .icon(Icon::new(IconName::Heart))
    .label("Like")
```

### With a dropdown caret icon

The `.dropdown_caret` method can allows adding a dropdown caret icon to end of the button.

```rust
Button::new("btn")
    .label("Options")
    .dropdown_caret(true)
```

### Button States

There have `disabled`, `loading`, `selected` state for buttons to indicate different statuses.

```rust
// Disabled
Button::new("btn")
    .label("Disabled")
    .disabled(true)

// Loading
Button::new("btn")
    .label("Loading")
    .loading(true)

// Selected
Button::new("btn")
    .label("Selected")
    .selected(true)
```

## Button Group

```rust
ButtonGroup::new("btn-group")
    .child(Button::new("btn1").label("One"))
    .child(Button::new("btn2").label("Two"))
    .child(Button::new("btn3").label("Three"))
```

### Toggle Button Group

```rust
ButtonGroup::new("toggle-group")
    .multiple(true) // Allow multiple selections
    .child(Button::new("btn1").label("Option 1").selected(true))
    .child(Button::new("btn2").label("Option 2"))
    .child(Button::new("btn3").label("Option 3"))
    .on_click(|selected_indices, _, _| {
        println!("Selected: {:?}", selected_indices);
    })
```

## Custom Variant

```rust
use gpui_component::button::ButtonCustomVariant;

let custom = ButtonCustomVariant::new(cx)
    .color(cx.theme().magenta)
    .foreground(cx.theme().primary_foreground)
    .border(cx.theme().magenta)
    .hover(cx.theme().magenta.opacity(0.1))
    .active(cx.theme().magenta);

Button::new("custom-btn")
    .custom(custom)
    .label("Custom Button")
```

## API Reference

* [Button]
* [ButtonGroup]
* [ButtonCustomVariant]

## Examples

### With Tooltip

```rust
Button::new("btn")
    .label("Hover me")
    .tooltip("This is a helpful tooltip")
```

### Custom Children

```rust
Button::new("btn")
    .child(
        h_flex()
            .items_center()
            .gap_2()
            .child("Custom Content")
            .child(IconName::ChevronDown)
            .child(IconName::Eye)
    )
```

[Button]: https://docs.rs/gpui-component/latest/gpui_component/button/struct.Button.html

[ButtonGroup]: https://docs.rs/gpui-component/latest/gpui_component/button/struct.ButtonGroup.html

[ButtonCustomVariant]: https://docs.rs/gpui-component/latest/gpui_component/button/struct.ButtonCustomVariant.html

[Sizable]: https://docs.rs/gpui-component/latest/gpui_component/trait.Sizable.html

---

---
url: /gpui-component/docs/components/calendar.md
description: >-
  A flexible calendar component for displaying months, navigating dates, and
  selecting single dates or date ranges.
---

# Calendar

A standalone calendar component that provides a rich interface for date selection and navigation. The Calendar component supports single date selection, date range selection, multiple month views, custom disabled dates, and comprehensive keyboard navigation.

* [CalendarState]: For managing calendar state and selection.
* [Calendar]: For rendering the calendar UI.

## Import

```rust
use gpui_component::{
    calendar::{Calendar, CalendarState, CalendarEvent, Date, Matcher},
};
```

## Usage

### Basic Calendar

```rust
let state = cx.new(|cx| CalendarState::new(window, cx));
Calendar::new(&state)
```

### Calendar with Initial Date

```rust
use chrono::Local;

let state = cx.new(|cx| {
    let mut state = CalendarState::new(window, cx);
    state.set_date(Local::now().naive_local().date(), window, cx);
    state
});

Calendar::new(&state)
```

### Date Range Calendar

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

### Multiple Months Display

```rust
// Show 2 months side by side
Calendar::new(&state)
    .number_of_months(2)

// Show 3 months
Calendar::new(&state)
    .number_of_months(3)
```

### Calendar Sizes

```rust
Calendar::new(&state).large()
Calendar::new(&state) // medium (default)
Calendar::new(&state).small()
```

## Date Restrictions

### Disabled Weekends

```rust
let state = cx.new(|cx| {
    CalendarState::new(window, cx)
        .disabled_matcher(vec![0, 6]) // Sunday=0, Saturday=6
});

Calendar::new(&state)
```

### Disabled Specific Weekdays

```rust
// Disable Sundays, Wednesdays, and Saturdays
let state = cx.new(|cx| {
    CalendarState::new(window, cx)
        .disabled_matcher(vec![0, 3, 6])
});

Calendar::new(&state)
```

### Disabled Date Range

```rust
use chrono::{Local, Days};

let now = Local::now().naive_local().date();

let state = cx.new(|cx| {
    CalendarState::new(window, cx)
        .disabled_matcher(Matcher::range(
            Some(now),
            now.checked_add_days(Days::new(7)),
        ))
});

Calendar::new(&state)
```

### Disabled Date Interval

```rust
// Disable dates outside the interval (before/after specified dates)
let state = cx.new(|cx| {
    CalendarState::new(window, cx)
        .disabled_matcher(Matcher::interval(
            Some(now.checked_sub_days(Days::new(30)).unwrap()),
            now.checked_add_days(Days::new(30))
        ))
});

Calendar::new(&state)
```

### Custom Disabled Dates

```rust
// Disable first 5 days of each month
let state = cx.new(|cx| {
    CalendarState::new(window, cx)
        .disabled_matcher(Matcher::custom(|date| {
            date.day0() < 5 // day0() returns 0-based day
        }))
});

Calendar::new(&state)

// Disable all Mondays
let state = cx.new(|cx| {
    CalendarState::new(window, cx)
        .disabled_matcher(Matcher::custom(|date| {
            date.weekday() == chrono::Weekday::Mon
        }))
});

Calendar::new(&state)

// Disable past dates
let state = cx.new(|cx| {
    CalendarState::new(window, cx)
        .disabled_matcher(Matcher::custom(|date| {
            *date < Local::now().naive_local().date()
        }))
});

Calendar::new(&state)
```

## Month/Year Navigation

The Calendar automatically provides navigation controls:

* **Previous/Next Month**: Arrow buttons in the header
* **Month Selection**: Click on month name to open month picker
* **Year Selection**: Click on year to open year picker
* **Year Pages**: Navigate through 20-year pages in year view

### Custom Year Range

```rust
let state = cx.new(|cx| {
    CalendarState::new(window, cx)
        .year_range((2020, 2030)) // Limit to specific year range
});

Calendar::new(&state)
```

## Handle Selection Events

```rust
let state = cx.new(|cx| CalendarState::new(window, cx));

cx.subscribe(&state, |view, _, event, _| {
    match event {
        CalendarEvent::Selected(date) => {
            match date {
                Date::Single(Some(selected_date)) => {
                    println!("Date selected: {}", selected_date);
                }
                Date::Range(Some(start), Some(end)) => {
                    println!("Range selected: {} to {}", start, end);
                }
                Date::Range(Some(start), None) => {
                    println!("Range start: {}", start);
                }
                _ => {
                    println!("Selection cleared");
                }
            }
        }
    }
});

Calendar::new(&state)
```

## Advanced Examples

### Business Days Only Calendar

```rust
use chrono::Weekday;

let state = cx.new(|cx| {
    CalendarState::new(window, cx)
        .disabled_matcher(Matcher::custom(|date| {
            matches!(date.weekday(), Weekday::Sat | Weekday::Sun)
        }))
});

Calendar::new(&state)
```

### Holiday Calendar

```rust
use chrono::NaiveDate;
use std::collections::HashSet;

// Define holidays
let holidays: HashSet<NaiveDate> = [
    NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(), // New Year
    NaiveDate::from_ymd_opt(2024, 7, 4).unwrap(), // Independence Day
    NaiveDate::from_ymd_opt(2024, 12, 25).unwrap(), // Christmas
].into_iter().collect();

let state = cx.new(|cx| {
    CalendarState::new(window, cx)
        .disabled_matcher(Matcher::custom(move |date| {
            holidays.contains(date)
        }))
});

Calendar::new(&state)
```

### Multi-Month Range Selector

```rust
let state = cx.new(|cx| {
    let mut state = CalendarState::new(window, cx);
    state.set_date(Date::Range(None, None), window, cx); // Range mode
    state
});

Calendar::new(&state)
    .number_of_months(3) // Show 3 months for easier range selection
```

### Quarterly View Calendar

```rust
let state = cx.new(|cx| CalendarState::new(window, cx));

// Update to show current quarter's months
Calendar::new(&state)
    .number_of_months(3)
```

## Custom Styling

```rust
use gpui::{px, relative};

Calendar::new(&calendar)
    .p_4() // Custom padding
    .bg(cx.theme().secondary) // Custom background
    .border_2() // Custom border
    .border_color(cx.theme().primary) // Custom border color
    .rounded(px(12.)) // Custom border radius
    .w(px(400.)) // Custom width
    .h(px(350.)) // Custom height
```

## API Reference

* [Calendar]
* [CalendarState]
* [RangeMatcher]

## Examples

### Event Planning Calendar

```rust
let event_calendar = cx.new(|cx| {
    let mut state = CalendarState::new(window, cx);
    // Disable past dates and weekends
    state = state.disabled_matcher(Matcher::custom(|date| {
        let now = Local::now().naive_local().date();
        *date < now || matches!(date.weekday(), Weekday::Sat | Weekday::Sun)
    }));
    state
});

Calendar::new(&event_calendar)
    .large() // Easier to see and interact with
```

### Vacation Booking Calendar

```rust
let vacation_calendar = cx.new(|cx| {
    let mut state = CalendarState::new(window, cx);
    state.set_date(Date::Range(None, None), window, cx); // Range mode
    state
});

Calendar::new(&vacation_calendar)
    .number_of_months(2) // Show 2 months for range selection
```

### Report Date Range Selector

```rust
let report_calendar = cx.new(|cx| {
    let mut state = CalendarState::new(window, cx)
        .year_range((2020, 2025)); // Limit to business years

    state.set_date(Date::Range(None, None), window, cx);
    state
});

Calendar::new(&report_calendar)
    .number_of_months(3)
    .small() // Compact for dashboard use
```

### Availability Calendar

```rust
use std::collections::HashSet;

let unavailable_dates: HashSet<NaiveDate> = get_unavailable_dates();

let availability_calendar = cx.new(|cx| {
    CalendarState::new(window, cx)
        .disabled_matcher(Matcher::custom(move |date| {
            unavailable_dates.contains(date)
        }))
});

Calendar::new(&availability_calendar)
    .number_of_months(2)
```

The Calendar component provides a foundation for any date-related UI requirements, from simple date pickers to complex scheduling interfaces.

[Calendar]: https://docs.rs/gpui-component/latest/gpui_component/calendar/struct.Calendar.html

[CalendarState]: https://docs.rs/gpui-component/latest/gpui_component/calendar/struct.CalendarState.html

[RangeMatcher]: https://docs.rs/gpui-component/latest/gpui_component/calendar/struct.RangeMatcher.html

---

---
url: /gpui-component/docs/components/chart.md
description: >-
  Beautiful charts and graphs for data visualization including line, bar, area,
  pie, and candlestick charts.
---

# Chart

A comprehensive charting library providing Line, Bar, Area, Pie, and Candlestick charts for data visualization. The charts feature smooth animations, customizable styling, tooltips, legends, and automatic theming that adapts to your application's theme.

## Import

```rust
use gpui_component::chart::{LineChart, BarChart, AreaChart, PieChart, CandlestickChart};
```

## Chart Types

### LineChart

A line chart displays data points connected by straight line segments, perfect for showing trends over time.

#### Basic Line Chart

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

#### Line Chart Variants

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

#### Tick Control

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

### BarChart

A bar chart uses rectangular bars to show comparisons among categories.

#### Basic Bar Chart

```rust
BarChart::new(data)
    .x(|d| d.category.clone())
    .y(|d| d.value)
```

#### Bar Chart Customization

```rust
// Custom fill colors
BarChart::new(data)
    .x(|d| d.category.clone())
    .y(|d| d.value)
    .fill(|d| d.color)

// With labels on bars
BarChart::new(data)
    .x(|d| d.category.clone())
    .y(|d| d.value)
    .label(|d| format!("{}", d.value))

// Custom tick spacing
BarChart::new(data)
    .x(|d| d.category.clone())
    .y(|d| d.value)
    .tick_margin(2)
```

### AreaChart

An area chart displays quantitative data visually, similar to a line chart but with the area below the line filled.

#### Basic Area Chart

```rust
AreaChart::new(data)
    .x(|d| d.time.clone())
    .y(|d| d.value)
```

#### Stacked Area Charts

```rust
// Multi-series area chart
AreaChart::new(data)
    .x(|d| d.date.clone())
    .y(|d| d.desktop)  // First series
    .stroke(cx.theme().chart_1)
    .fill(cx.theme().chart_1.opacity(0.4))
    .y(|d| d.mobile)   // Second series
    .stroke(cx.theme().chart_2)
    .fill(cx.theme().chart_2.opacity(0.4))
```

#### Area Chart Styling

```rust
use gpui::{linear_gradient, linear_color_stop};

// With gradient fill
AreaChart::new(data)
    .x(|d| d.month.clone())
    .y(|d| d.value)
    .fill(linear_gradient(
        0.,
        linear_color_stop(cx.theme().chart_1.opacity(0.4), 1.),
        linear_color_stop(cx.theme().background.opacity(0.3), 0.),
    ))

// Different interpolation styles
AreaChart::new(data)
    .x(|d| d.month.clone())
    .y(|d| d.value)
    .linear()  // or .step_after()
```

### PieChart

A pie chart displays data as slices of a circular chart, ideal for showing proportions.

#### Basic Pie Chart

```rust
PieChart::new(data)
    .value(|d| d.amount as f32)
    .outer_radius(100.)
```

#### Donut Chart

```rust
PieChart::new(data)
    .value(|d| d.amount as f32)
    .outer_radius(100.)
    .inner_radius(60.) // Creates donut effect
```

#### Pie Chart Customization

```rust
// Custom colors
PieChart::new(data)
    .value(|d| d.amount as f32)
    .outer_radius(100.)
    .color(|d| d.color)

// With padding between slices
PieChart::new(data)
    .value(|d| d.amount as f32)
    .outer_radius(100.)
    .inner_radius(60.)
    .pad_angle(4. / 100.) // 4% padding
```

### CandlestickChart

A candlestick chart displays financial data using OHLC (Open, High, Low, Close) values, perfect for visualizing stock prices and market trends.

#### Basic Candlestick Chart

```rust
#[derive(Clone)]
struct StockPrice {
    pub date: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
}

let data = vec![
    StockPrice { date: "Jan".to_string(), open: 100.0, high: 110.0, low: 95.0, close: 105.0 },
    StockPrice { date: "Feb".to_string(), open: 105.0, high: 115.0, low: 100.0, close: 112.0 },
    StockPrice { date: "Mar".to_string(), open: 112.0, high: 120.0, low: 108.0, close: 115.0 },
];

CandlestickChart::new(data)
    .x(|d| d.date.clone())
    .open(|d| d.open)
    .high(|d| d.high)
    .low(|d| d.low)
    .close(|d| d.close)
```

#### Candlestick Chart Customization

```rust
// Adjust body width ratio (default: 0.6)
CandlestickChart::new(data)
    .x(|d| d.date.clone())
    .open(|d| d.open)
    .high(|d| d.high)
    .low(|d| d.low)
    .close(|d| d.close)
    .body_width_ratio(0.4) // Narrower bodies

// Custom tick spacing
CandlestickChart::new(data)
    .x(|d| d.date.clone())
    .open(|d| d.open)
    .high(|d| d.high)
    .low(|d| d.low)
    .close(|d| d.close)
    .tick_margin(2) // Show every 2nd tick
```

#### Candlestick Chart Colors

The candlestick chart automatically uses theme colors:

* **Bullish** (close > open): `bullish` color (green)
* **Bearish** (close < open): `bearish` color (red)

## Data Structures

### Example Data Types

```rust
// Time series data
#[derive(Clone)]
struct DailyDevice {
    pub date: String,
    pub desktop: f64,
    pub mobile: f64,
}

// Category data with styling
#[derive(Clone)]
struct MonthlyDevice {
    pub month: String,
    pub desktop: f64,
    pub color_alpha: f32,
}

impl MonthlyDevice {
    pub fn color(&self, base_color: Hsla) -> Hsla {
        base_color.alpha(self.color_alpha)
    }
}

// Financial data
#[derive(Clone)]
struct StockPrice {
    pub date: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: u64,
}
```

## Chart Configuration

### Container Setup

```rust
fn chart_container(
    title: &str,
    chart: impl IntoElement,
    center: bool,
    cx: &mut Context<ChartStory>,
) -> impl IntoElement {
    v_flex()
        .flex_1()
        .h_full()
        .border_1()
        .border_color(cx.theme().border)
        .rounded_lg()
        .p_4()
        .child(
            div()
                .when(center, |this| this.text_center())
                .font_semibold()
                .child(title.to_string()),
        )
        .child(
            div()
                .when(center, |this| this.text_center())
                .text_color(cx.theme().muted_foreground)
                .text_sm()
                .child("Data period label"),
        )
        .child(div().flex_1().py_4().child(chart))
        .child(
            div()
                .when(center, |this| this.text_center())
                .font_semibold()
                .text_sm()
                .child("Summary statistic"),
        )
        .child(
            div()
                .when(center, |this| this.text_center())
                .text_color(cx.theme().muted_foreground)
                .text_sm()
                .child("Additional context"),
        )
}
```

### Theme Integration

```rust
// Charts automatically use theme colors
let chart = LineChart::new(data)
    .x(|d| d.date.clone())
    .y(|d| d.value)
    .stroke(cx.theme().chart_1); // Uses theme chart colors

// Available theme chart colors:
// cx.theme().chart_1
// cx.theme().chart_2
// cx.theme().chart_3
// ... up to chart_5
```

## API Reference

* [LineChart]
* [BarChart]
* [AreaChart]
* [PieChart]
* [CandlestickChart]

## Examples

### Sales Dashboard

```rust
#[derive(Clone)]
struct SalesData {
    month: String,
    revenue: f64,
    profit: f64,
    region: String,
}

fn sales_dashboard(data: Vec<SalesData>, cx: &mut Context<Self>) -> impl IntoElement {
    v_flex()
        .gap_4()
        .child(
            h_flex()
                .gap_4()
                .child(
                    chart_container(
                        "Monthly Revenue",
                        LineChart::new(data.clone())
                            .x(|d| d.month.clone())
                            .y(|d| d.revenue)
                            .stroke(cx.theme().chart_1)
                            .dot(),
                        false,
                        cx,
                    )
                )
                .child(
                    chart_container(
                        "Profit Breakdown",
                        PieChart::new(data.clone())
                            .value(|d| d.profit as f32)
                            .outer_radius(80.)
                            .color(|d| match d.region.as_str() {
                                "North" => cx.theme().chart_1,
                                "South" => cx.theme().chart_2,
                                "East" => cx.theme().chart_3,
                                "West" => cx.theme().chart_4,
                                _ => cx.theme().chart_5,
                            }),
                        true,
                        cx,
                    )
                )
        )
        .child(
            chart_container(
                "Regional Performance",
                BarChart::new(data)
                    .x(|d| d.region.clone())
                    .y(|d| d.revenue)
                    .fill(|d| match d.region.as_str() {
                        "North" => cx.theme().chart_1,
                        "South" => cx.theme().chart_2,
                        "East" => cx.theme().chart_3,
                        "West" => cx.theme().chart_4,
                        _ => cx.theme().chart_5,
                    })
                    .label(|d| format!("${:.0}k", d.revenue / 1000.)),
                false,
                cx,
            )
        )
}
```

### Multi-Series Time Chart

```rust
#[derive(Clone)]
struct DeviceUsage {
    date: String,
    desktop: f64,
    mobile: f64,
    tablet: f64,
}

fn device_usage_chart(data: Vec<DeviceUsage>, cx: &mut Context<Self>) -> impl IntoElement {
    chart_container(
        "Device Usage Over Time",
        AreaChart::new(data)
            .x(|d| d.date.clone())
            .y(|d| d.desktop)
            .stroke(cx.theme().chart_1)
            .fill(linear_gradient(
                0.,
                linear_color_stop(cx.theme().chart_1.opacity(0.4), 1.),
                linear_color_stop(cx.theme().background.opacity(0.3), 0.),
            ))
            .y(|d| d.mobile)
            .stroke(cx.theme().chart_2)
            .fill(linear_gradient(
                0.,
                linear_color_stop(cx.theme().chart_2.opacity(0.4), 1.),
                linear_color_stop(cx.theme().background.opacity(0.3), 0.),
            ))
            .y(|d| d.tablet)
            .stroke(cx.theme().chart_3)
            .fill(linear_gradient(
                0.,
                linear_color_stop(cx.theme().chart_3.opacity(0.4), 1.),
                linear_color_stop(cx.theme().background.opacity(0.3), 0.),
            ))
            .tick_margin(3),
        false,
        cx,
    )
}
```

### Financial Chart

```rust
#[derive(Clone)]
struct StockData {
    date: String,
    price: f64,
    volume: u64,
}

#[derive(Clone)]
struct StockOHLC {
    date: String,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
}

fn stock_chart(ohlc_data: Vec<StockOHLC>, price_data: Vec<StockData>, cx: &mut Context<Self>) -> impl IntoElement {
    v_flex()
        .gap_4()
        .child(
            chart_container(
                "Stock Price - Candlestick",
                CandlestickChart::new(ohlc_data.clone())
                    .x(|d| d.date.clone())
                    .open(|d| d.open)
                    .high(|d| d.high)
                    .low(|d| d.low)
                    .close(|d| d.close)
                    .tick_margin(3),
                false,
                cx,
            )
        )
        .child(
            chart_container(
                "Stock Price - Line",
                LineChart::new(price_data.clone())
                    .x(|d| d.date.clone())
                    .y(|d| d.price)
                    .stroke(cx.theme().chart_1)
                    .linear()
                    .tick_margin(5),
                false,
                cx,
            )
        )
        .child(
            chart_container(
                "Trading Volume",
                BarChart::new(price_data)
                    .x(|d| d.date.clone())
                    .y(|d| d.volume as f64)
                    .fill(|d| {
                        if d.volume > 1000000 {
                            cx.theme().chart_1
                        } else {
                            cx.theme().muted_foreground.opacity(0.6)
                        }
                    })
                    .tick_margin(5),
                false,
                cx,
            )
        )
}
```

## Customization Options

### Color Schemes

```rust
// Theme-based colors (recommended)
LineChart::new(data)
    .x(|d| d.x.clone())
    .y(|d| d.y)
    .stroke(cx.theme().chart_1)

// Custom color palette
let colors = [
    cx.theme().success,
    cx.theme().warning,
    cx.theme().destructive,
    cx.theme().info,
    cx.theme().chart_1,
];

BarChart::new(data)
    .x(|d| d.category.clone())
    .y(|d| d.value)
    .fill(|d| colors[d.category_index % colors.len()])
```

### Responsive Design

```rust
// Container with responsive sizing
div()
    .flex_1()
    .min_h(px(300.))
    .max_h(px(600.))
    .w_full()
    .child(
        LineChart::new(data)
            .x(|d| d.x.clone())
            .y(|d| d.y)
    )
```

### Grid and Axis Styling

Charts automatically include:

* Grid lines with dashed appearance
* X-axis labels with smart positioning
* Y-axis scaling starting from zero
* Responsive tick spacing based on `tick_margin`

## Performance Considerations

### Large Datasets

```rust
// For large datasets, consider data sampling
let sampled_data: Vec<_> = data
    .iter()
    .step_by(5) // Show every 5th point
    .cloned()
    .collect();

LineChart::new(sampled_data)
    .x(|d| d.date.clone())
    .y(|d| d.value)
    .tick_margin(3) // Reduce tick density
```

### Memory Optimization

```rust
// Use efficient data accessors
LineChart::new(data)
    .x(|d| d.date.clone()) // Clone only when necessary
    .y(|d| d.value)        // Direct field access
```

## Integration Examples

### With State Management

```rust
struct ChartComponent {
    data: Vec<DataPoint>,
    chart_type: ChartType,
    time_range: TimeRange,
}

impl ChartComponent {
    fn render_chart(&self, cx: &mut Context<Self>) -> impl IntoElement {
        match self.chart_type {
            ChartType::Line => LineChart::new(self.filtered_data())
                .x(|d| d.date.clone())
                .y(|d| d.value)
                .into_any_element(),
            ChartType::Bar => BarChart::new(self.filtered_data())
                .x(|d| d.date.clone())
                .y(|d| d.value)
                .into_any_element(),
            ChartType::Area => AreaChart::new(self.filtered_data())
                .x(|d| d.date.clone())
                .y(|d| d.value)
                .into_any_element(),
        }
    }

    fn filtered_data(&self) -> Vec<DataPoint> {
        self.data
            .iter()
            .filter(|d| self.time_range.contains(&d.date))
            .cloned()
            .collect()
    }
}
```

### Real-time Updates

```rust
struct LiveChart {
    data: Vec<DataPoint>,
    max_points: usize,
}

impl LiveChart {
    fn add_data_point(&mut self, point: DataPoint) {
        self.data.push(point);
        if self.data.len() > self.max_points {
            self.data.remove(0); // Remove oldest point
        }
    }

    fn render(&self, cx: &mut Context<Self>) -> impl IntoElement {
        LineChart::new(self.data.clone())
            .x(|d| d.timestamp.clone())
            .y(|d| d.value)
            .linear()
            .dot()
    }
}
```

[LineChart]: https://docs.rs/gpui-component/latest/gpui_component/chart/struct.LineChart.html

[BarChart]: https://docs.rs/gpui-component/latest/gpui_component/chart/struct.BarChart.html

[AreaChart]: https://docs.rs/gpui-component/latest/gpui_component/chart/struct.AreaChart.html

[PieChart]: https://docs.rs/gpui-component/latest/gpui_component/chart/struct.PieChart.html

[CandlestickChart]: https://docs.rs/gpui-component/latest/gpui_component/chart/struct.CandlestickChart.html

---

---
url: /gpui-component/docs/components/checkbox.md
description: A control that allows the user to toggle between checked and not checked.
---

# Checkbox

A checkbox component for binary choices. Supports labels, disabled state, and different sizes.

## Import

```rust
use gpui_component::checkbox::Checkbox;
```

## Usage

### Basic Checkbox

```rust
Checkbox::new("my-checkbox")
    .label("Accept terms and conditions")
    .checked(false)
    .on_click(|checked, _, _| {
        println!("Checkbox is now: {}", checked);
    })
```

The `on_click` callback is triggered when the user toggles the checkbox, receiving the **new checked state**.

### Controlled Checkbox

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

### Different Sizes

```rust
Checkbox::new("cb").text_xs().label("Extra Small")
Checkbox::new("cb").text_sm().label("Small")
Checkbox::new("cb").label("Medium") // default
Checkbox::new("cb").text_lg().label("Large")
```

### Disabled State

```rust
Checkbox::new("checkbox")
    .label("Disabled checkbox")
    .disabled(true)
    .checked(false)
```

### Without Label

```rust
Checkbox::new("checkbox")
    .checked(true)
```

### Custom Tab Order

```rust
Checkbox::new("checkbox")
    .label("Custom tab order")
    .tab_index(2)
    .tab_stop(true)
```

## API Reference

* [Checkbox]

### Styling

Implements `Sizable` and `Disableable` traits:

* `text_xs()` - Extra small text
* `text_sm()` - Small text
* `text_base()` - Base text (default)
* `text_lg()` - Large text
* `disabled(bool)` - Disabled state

## Examples

### Checkbox List

```rust
v_flex()
    .gap_2()
    .child(Checkbox::new("cb1").label("Option 1").checked(true))
    .child(Checkbox::new("cb2").label("Option 2").checked(false))
    .child(Checkbox::new("cb3").label("Option 3").checked(false))
```

### Form Integration

```rust
struct FormView {
    agree_terms: bool,
    subscribe: bool,
}

v_flex()
    .gap_3()
    .child(
        Checkbox::new("terms")
            .label("I agree to the terms and conditions")
            .checked(self.agree_terms)
            .on_click(cx.listener(|view, checked, _, cx| {
                view.agree_terms = *checked;
                cx.notify();
            }))
    )
    .child(
        Checkbox::new("subscribe")
            .label("Subscribe to newsletter")
            .checked(self.subscribe)
            .on_click(cx.listener(|view, checked, _, cx| {
                view.subscribe = *checked;
                cx.notify();
            }))
    )
```

[Checkbox]: https://docs.rs/gpui-component/latest/gpui_component/checkbox/struct.Checkbox.html

---

---
url: /gpui-component/docs/components/clipboard.md
description: >-
  A button component that helps you copy text or other content to your
  clipboard.
---

# Clipboard

The Clipboard component provides an easy way to copy text or other data to the user's clipboard. It renders as a button with a copy icon that changes to a checkmark when content is successfully copied. The component supports both static values and dynamic content through callback functions.

## Import

```rust
use gpui_component::clipboard::Clipboard;
```

## Usage

### Basic Clipboard

```rust
Clipboard::new("my-clipboard")
    .value("Text to copy")
    .on_copied(|value, window, cx| {
        window.push_notification(format!("Copied: {}", value), cx)
    })
```

### Using Dynamic Values

The `value_fn` method allows you to provide a closure that generates the content to be copied at the time of the copy action.

* This is useful when the content to be copied depends on the current state of the application.
* And in some cases, it may have a larger overhead to compute, so you only want to do it when the user actually clicks the copy button.

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

### With Custom Content

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

### In Input Fields

The Clipboard component is commonly used as a suffix in input fields:

```rust
use gpui_component::input::{InputState, Input};

let url_state = cx.new(|cx| InputState::new(window, cx).default_value("https://github.com"));

Input::new(&url_state)
    .suffix(
        Clipboard::new("url-clipboard")
            .value_fn({
                let state = url_state.clone();
                move |_, cx| state.read(cx).value()
            })
            .on_copied(|value, window, cx| {
                window.push_notification(format!("URL copied: {}", value), cx)
            })
    )
```

## API Reference

* [Clipboard]

## Examples

### Simple Text Copy

```rust
Clipboard::new("simple")
    .value("Hello, World!")
```

### With User Feedback

```rust
h_flex()
    .gap_2()
    .child(Label::new("Your API Key:"))
    .child(
        Clipboard::new("feedback")
            .value("sk-1234567890abcdef")
            .on_copied(|_, window, cx| {
                window.push_notification("API key copied to clipboard", cx)
            })
    )
```

### Form Field Integration

```rust
use gpui_component::{
    input::{InputState, Input},
    h_flex, label::Label
};

let api_key = "sk-1234567890abcdef";

h_flex()
    .gap_2()
    .items_center()
    .child(Label::new("API Key:"))
    .child(
        Input::new(&input_state)
            .value(api_key)
            .readonly(true)
            .suffix(
                Clipboard::new("api-key-copy")
                    .value(api_key)
                    .on_copied(|_, window, cx| {
                        window.push_notification("API key copied!", cx)
                    })
            )
    )
```

### Dynamic Content Copy

```rust
struct AppState {
    current_url: String,
}

let app_state = cx.new(|_| AppState {
    current_url: "https://example.com".to_string()
});

Clipboard::new("current-url")
    .value_fn({
        let state = app_state.clone();
        move |_, cx| {
            SharedString::from(state.read(cx).current_url.clone())
        }
    })
    .on_copied(|url, window, cx| {
        window.push_notification(format!("Shared: {}", url), cx)
    })
```

## Data Types

The Clipboard component currently supports copying text strings to the clipboard. It uses GPUI's `ClipboardItem::new_string()` method, which handles:

* Plain text strings
* UTF-8 encoded content
* Cross-platform clipboard integration

[Clipboard]: https://docs.rs/gpui-component/latest/gpui_component/clipboard/struct.Clipboard.html

---

---
url: /gpui-component/docs/components/collapsible.md
description: An interactive element which expands/collapses.
---

# Collapsible

An interactive element which expands/collapses.

## Import

```rust
use gpui_component::collapsible::Collapsible;
```

## Usage

### Basic Use

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

We can use `open` method to control the collapsed state. If false, the `content` method added child elements will be hidden.

[Collapsible]: https://docs.rs/gpui-component/latest/gpui_component/collapsible/struct.Collapsible.html

---

---
url: /gpui-component/docs/components/color-picker.md
description: >-
  A comprehensive color selection interface with support for multiple color
  formats, presets, and alpha channel.
---

# ColorPicker

A versatile color picker component that provides an intuitive interface for color selection. Features include color palettes, hex input, featured colors, and support for various color formats including RGB, HSL, and hex values with alpha channel support.

## Import

```rust
use gpui_component::color_picker::{ColorPicker, ColorPickerState, ColorPickerEvent};
```

## Usage

### Basic Color Picker

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

### With Event Handling

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

### Setting Default Color

```rust
use gpui::Hsla;

let color_picker = cx.new(|cx|
    ColorPickerState::new(window, cx)
        .default_value(cx.theme().blue) // Set default color
);
```

### Different Sizes

```rust
// Small color picker
ColorPicker::new(&color_picker).small()

// Medium color picker (default)
ColorPicker::new(&color_picker)

// Large color picker
ColorPicker::new(&color_picker).large()

// Extra small color picker
ColorPicker::new(&color_picker).xsmall()
```

### With Custom Featured Colors

```rust
use gpui::Hsla;

let featured_colors = vec![
    cx.theme().red,
    cx.theme().green,
    cx.theme().blue,
    cx.theme().yellow,
    // Add your custom colors
];

ColorPicker::new(&color_picker)
    .featured_colors(featured_colors)
```

### With Icon Instead of Color Square

```rust
use gpui_component::IconName;

ColorPicker::new(&color_picker)
    .icon(IconName::Palette)
```

### With Label

```rust
ColorPicker::new(&color_picker)
    .label("Background Color")
```

### Custom Anchor Position

```rust
use gpui::Corner;

ColorPicker::new(&color_picker)
    .anchor(Corner::TopRight) // Dropdown opens to top-right
```

## Color Selection Interface

### Color Palettes

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

### Hex Input Field

A text input field that allows direct entry of hex color values:

* Supports standard 6-digit hex format (#RRGGBB)
* Real-time validation and preview
* Updates color picker state automatically
* Press Enter to confirm selection

## Color Formats

### RGB (Red, Green, Blue)

Colors are internally represented using GPUI's `Hsla` format but can be converted to RGB:

```rust
let color = cx.theme().blue;
// Access RGB components through Hsla methods
```

### HSL (Hue, Saturation, Lightness)

Native format used by the color picker:

```rust
use gpui::Hsla;

// Create HSL color
let color = Hsla::hsl(240.0, 100.0, 50.0); // Blue color

// Access components
let hue = color.h;
let saturation = color.s;
let lightness = color.l;
```

### Hex Format

Standard web hex format with # prefix:

```rust
// Convert color to hex
let hex_string = color.to_hex(); // Returns "#3366FF"

// Parse hex string to color
if let Ok(color) = Hsla::parse_hex("#3366FF") {
    // Use parsed color
}
```

## Alpha Channel

Full alpha channel support for transparency:

```rust
use gpui::hsla;

// Create color with alpha
let semi_transparent = hsla(0.5, 0.8, 0.6, 0.7); // 70% opacity

// Modify existing color opacity
let transparent_blue = cx.theme().blue.opacity(0.5);
```

The color picker preserves alpha values when selecting colors and allows modification through the alpha component of HSLA colors.

## API Reference

* [ColorPicker]
* [ColorPickerState]
* [ColorPickerEvent]

## Examples

### Color Theme Editor

```rust
struct ThemeEditor {
    primary_color: Entity<ColorPickerState>,
    secondary_color: Entity<ColorPickerState>,
    accent_color: Entity<ColorPickerState>,
}

impl ThemeEditor {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let primary_color = cx.new(|cx|
            ColorPickerState::new(window, cx)
                .default_value(cx.theme().primary)
        );

        let secondary_color = cx.new(|cx|
            ColorPickerState::new(window, cx)
                .default_value(cx.theme().secondary)
        );

        let accent_color = cx.new(|cx|
            ColorPickerState::new(window, cx)
                .default_value(cx.theme().accent)
        );

        Self {
            primary_color,
            secondary_color,
            accent_color,
        }
    }

    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_4()
            .child(
                h_flex()
                    .gap_2()
                    .items_center()
                    .child("Primary Color:")
                    .child(ColorPicker::new(&self.primary_color))
            )
            .child(
                h_flex()
                    .gap_2()
                    .items_center()
                    .child("Secondary Color:")
                    .child(ColorPicker::new(&self.secondary_color))
            )
            .child(
                h_flex()
                    .gap_2()
                    .items_center()
                    .child("Accent Color:")
                    .child(ColorPicker::new(&self.accent_color))
            )
    }
}
```

### Brand Color Selector

```rust
use gpui_component::{Sizable as _};

let brand_colors = vec![
    Hsla::parse_hex("#FF6B6B").unwrap(), // Brand Red
    Hsla::parse_hex("#4ECDC4").unwrap(), // Brand Teal
    Hsla::parse_hex("#45B7D1").unwrap(), // Brand Blue
    Hsla::parse_hex("#96CEB4").unwrap(), // Brand Green
    Hsla::parse_hex("#FFEAA7").unwrap(), // Brand Yellow
];

ColorPicker::new(&color_picker)
    .featured_colors(brand_colors)
    .label("Brand Color")
    .large()
```

### Toolbar Color Picker

```rust
use gpui_component::{Sizable as _, IconName);

ColorPicker::new(&text_color_picker)
    .icon(IconName::Type)
    .small()
    .anchor(Corner::BottomLeft)
```

### Color Palette Builder

```rust
struct ColorPalette {
    colors: Vec<Entity<ColorPickerState>>,
}

impl ColorPalette {
    fn add_color(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let color_picker = cx.new(|cx| ColorPickerState::new(window, cx));

        // Subscribe to color changes
        cx.subscribe(&color_picker, |this, _, ev, _| match ev {
            ColorPickerEvent::Change(color) => {
                if let Some(color) = color {
                    this.update_palette_preview();
                }
            }
        });

        self.colors.push(color_picker);
        cx.notify();
    }

    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .gap_2()
            .children(
                self.colors.iter().map(|color_picker| {
                    ColorPicker::new(color_picker).small()
                })
            )
            .child(
                Button::new("add-color")
                    .icon(IconName::Plus)
                    .ghost()
                    .on_click(cx.listener(|this, _, window, cx| {
                        this.add_color(window, cx);
                    }))
            )
    }
}
```

### With Color Validation

```rust
let color_picker = cx.new(|cx| ColorPickerState::new(window, cx));

let _subscription = cx.subscribe(&color_picker, |this, _, ev, _| match ev {
    ColorPickerEvent::Change(color) => {
        if let Some(color) = color {
            // Validate color accessibility
            if this.validate_contrast(color) {
                this.apply_color(color);
            } else {
                this.show_contrast_warning();
            }
        }
    }
});
```

[ColorPicker]: https://docs.rs/gpui-component/latest/gpui_component/color_picker/struct.ColorPicker.html

[ColorPickerState]: https://docs.rs/gpui-component/latest/gpui_component/color_picker/struct.ColorPickerState.html

[ColorPickerEvent]: https://docs.rs/gpui-component/latest/gpui_component/color_picker/enum.ColorPickerEvent.html

---

---
url: /gpui-component/docs/components.md
---

# Components

### Basic Components

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

### Form Components

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

---
url: /gpui-component/docs/context.md
description: Learn about the Window and Context in GPUI.
---

The [Window], [App], [Context] and [Entity] are most important things in GPUI, it appears everywhere.

* [Window] - The current window instance, which for handle the **Window Level** things.
* [App] - The current application instance, which for handle the **Application Level** things.
* [Context] - The Entity Context instance, which for handle the **Context Level** things.
* [Entity] - The Entity instance, which for handle the **Entity Level** things.

For example:

```rs
fn new(window: &mut Window, cx: &mut App) {}

impl RenderOnce for MyElement {
    fn render(self, window: &mut Window, cx: &mut App) {}
}

impl Render for MyView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) {}
}
```

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

---
url: /gpui-component/docs/components/date-picker.md
description: >-
  A date picker component for selecting single dates or date ranges with
  calendar interface.
---

# DatePicker

A flexible date picker component with calendar interface that supports single date selection, date range selection, custom date formatting, disabled dates, and preset ranges.

## Import

```rust
use gpui_component::{
    date_picker::{DatePicker, DatePickerState, DateRangePreset, DatePickerEvent},
    calendar::{Date, Matcher},
};
```

## Usage

### Basic Date Picker

```rust
let date_picker = cx.new(|cx| DatePickerState::new(window, cx));

DatePicker::new(&date_picker)
```

### With Initial Date

```rust
use chrono::Local;

let date_picker = cx.new(|cx| {
    let mut picker = DatePickerState::new(window, cx);
    picker.set_date(Local::now().naive_local().date(), window, cx);
    picker
});

DatePicker::new(&date_picker)
```

### Date Range Picker

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

### With Custom Date Format

```rust
let date_picker = cx.new(|cx| {
    DatePickerState::new(window, cx)
        .date_format("%Y-%m-%d") // ISO format
});

DatePicker::new(&date_picker)

// Other format examples:
// "%m/%d/%Y" -> 12/25/2023
// "%B %d, %Y" -> December 25, 2023
// "%d %b %Y" -> 25 Dec 2023
```

### With Placeholder

```rust
DatePicker::new(&date_picker)
    .placeholder("Select a date...")
```

### Cleanable Date Picker

```rust
DatePicker::new(&date_picker)
    .cleanable(true) // Show clear button when date is selected
```

### Different Sizes

```rust
DatePicker::new(&date_picker).large()
DatePicker::new(&date_picker) // medium (default)
DatePicker::new(&date_picker).small()
```

### Disabled State

```rust
DatePicker::new(&date_picker).disabled(true)
```

### Custom Appearance

```rust
// Without default styling
DatePicker::new(&date_picker).appearance(false)

// Use in custom container
div()
    .border_b_2()
    .px_6()
    .py_3()
    .border_color(cx.theme().border)
    .bg(cx.theme().secondary)
    .child(DatePicker::new(&date_picker).appearance(false))
```

## Date Restrictions

### Disabled Weekends

```rust
use gpui_component::calendar;

let date_picker = cx.new(|cx| {
    DatePickerState::new(window, cx)
        .disabled_matcher(vec![0, 6]) // Sunday=0, Saturday=6
});

DatePicker::new(&date_picker)
```

### Disabled Date Range

```rust
use chrono::{Local, Days};

let now = Local::now().naive_local().date();

let date_picker = cx.new(|cx| {
    DatePickerState::new(window, cx)
        .disabled_matcher(calendar::Matcher::range(
            Some(now),
            now.checked_add_days(Days::new(7)),
        ))
});

DatePicker::new(&date_picker)
```

### Disabled Date Interval

```rust
let date_picker = cx.new(|cx| {
    DatePickerState::new(window, cx)
        .disabled_matcher(calendar::Matcher::interval(
            Some(now),
            now.checked_add_days(Days::new(5))
        ))
});

DatePicker::new(&date_picker)
```

### Custom Disabled Dates

```rust
// Disable first 5 days of each month
let date_picker = cx.new(|cx| {
    DatePickerState::new(window, cx)
        .disabled_matcher(calendar::Matcher::custom(|date| {
            date.day0() < 5
        }))
});

DatePicker::new(&date_picker)

// Disable all Mondays
let date_picker = cx.new(|cx| {
    DatePickerState::new(window, cx)
        .disabled_matcher(calendar::Matcher::custom(|date| {
            date.weekday() == chrono::Weekday::Mon
        }))
});
```

## Preset Ranges

### Single Date Presets

```rust
use chrono::{Utc, Duration};

let presets = vec![
    DateRangePreset::single(
        "Yesterday",
        (Utc::now() - Duration::days(1)).naive_local().date(),
    ),
    DateRangePreset::single(
        "Last Week",
        (Utc::now() - Duration::weeks(1)).naive_local().date(),
    ),
    DateRangePreset::single(
        "Last Month",
        (Utc::now() - Duration::days(30)).naive_local().date(),
    ),
];

DatePicker::new(&date_picker)
    .presets(presets)
```

### Date Range Presets

```rust
let range_presets = vec![
    DateRangePreset::range(
        "Last 7 Days",
        (Utc::now() - Duration::days(7)).naive_local().date(),
        Utc::now().naive_local().date(),
    ),
    DateRangePreset::range(
        "Last 30 Days",
        (Utc::now() - Duration::days(30)).naive_local().date(),
        Utc::now().naive_local().date(),
    ),
    DateRangePreset::range(
        "Last 90 Days",
        (Utc::now() - Duration::days(90)).naive_local().date(),
        Utc::now().naive_local().date(),
    ),
];

DatePicker::new(&date_picker)
    .number_of_months(2)
    .presets(range_presets)
```

## Handle Date Selection Events

```rust
let date_picker = cx.new(|cx| DatePickerState::new(window, cx));

cx.subscribe(&date_picker, |view, _, event, _| {
    match event {
        DatePickerEvent::Change(date) => {
            match date {
                Date::Single(Some(selected_date)) => {
                    println!("Single date selected: {}", selected_date);
                }
                Date::Range(Some(start), Some(end)) => {
                    println!("Date range selected: {} to {}", start, end);
                }
                Date::Range(Some(start), None) => {
                    println!("Range start selected: {}", start);
                }
                _ => {
                    println!("Date cleared");
                }
            }
        }
    }
});
```

## Multiple Months Display

```rust
// Show 2 months side by side (useful for date ranges)
DatePicker::new(&date_picker)
    .number_of_months(2)

// Show 3 months
DatePicker::new(&date_picker)
    .number_of_months(3)
```

## Advanced Examples

### Business Days Only

```rust
use chrono::Weekday;

let business_days_picker = cx.new(|cx| {
    DatePickerState::new(window, cx)
        .disabled_matcher(calendar::Matcher::custom(|date| {
            matches!(date.weekday(), Weekday::Sat | Weekday::Sun)
        }))
});

DatePicker::new(&business_days_picker)
    .placeholder("Select business day")
```

### Date Range with Max Duration

```rust
use chrono::Days;

let max_30_days_picker = cx.new(|cx| DatePickerState::range(window, cx));

cx.subscribe(&max_30_days_picker, |view, picker, event, _| {
    match event {
        DatePickerEvent::Change(Date::Range(Some(start), Some(end))) => {
            let duration = end.signed_duration_since(*start).num_days();
            if duration > 30 {
                // Reset to start date only if range exceeds 30 days
                picker.update(cx, |state, cx| {
                    state.set_date(Date::Range(Some(*start), None), window, cx);
                });
            }
        }
        _ => {}
    }
});

DatePicker::new(&max_30_days_picker)
    .number_of_months(2)
    .placeholder("Select up to 30 days")
```

### Quarter Presets

```rust
use chrono::{NaiveDate, Datelike};

fn quarter_start(year: i32, quarter: u32) -> NaiveDate {
    let month = (quarter - 1) * 3 + 1;
    NaiveDate::from_ymd_opt(year, month, 1).unwrap()
}

fn quarter_end(year: i32, quarter: u32) -> NaiveDate {
    let month = quarter * 3;
    let start = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
    NaiveDate::from_ymd_opt(year, month, start.days_in_month()).unwrap()
}

let year = Local::now().year();
let quarterly_presets = vec![
    DateRangePreset::range("Q1", quarter_start(year, 1), quarter_end(year, 1)),
    DateRangePreset::range("Q2", quarter_start(year, 2), quarter_end(year, 2)),
    DateRangePreset::range("Q3", quarter_start(year, 3), quarter_end(year, 3)),
    DateRangePreset::range("Q4", quarter_start(year, 4), quarter_end(year, 4)),
];

DatePicker::new(&date_picker)
    .presets(quarterly_presets)
```

## Examples

### Event Date Picker

```rust
let event_date = cx.new(|cx| {
    let mut picker = DatePickerState::new(window, cx)
        .date_format("%B %d, %Y")
        .disabled_matcher(calendar::Matcher::custom(|date| {
            // Disable past dates
            *date < Local::now().naive_local().date()
        }));
    picker
});

DatePicker::new(&event_date)
    .placeholder("Choose event date")
    .cleanable(true)
```

### Booking System Date Range

```rust
let booking_range = cx.new(|cx| DatePickerState::range(window, cx));

let booking_presets = vec![
    DateRangePreset::range("This Weekend", /* weekend dates */),
    DateRangePreset::range("Next Week", /* next week dates */),
    DateRangePreset::range("This Month", /* this month dates */),
];

DatePicker::new(&booking_range)
    .number_of_months(2)
    .presets(booking_presets)
    .placeholder("Select check-in and check-out dates")
```

### Financial Period Selector

```rust
let financial_period = cx.new(|cx| {
    DatePickerState::range(window, cx)
        .date_format("%Y-%m-%d")
});

DatePicker::new(&financial_period)
    .number_of_months(3)
    .presets(quarterly_presets)
    .placeholder("Select reporting period")
```

---

---
url: /gpui-component/docs/components/description-list.md
description: Use to display details with a tidy layout for key-value pairs.
---

# DescriptionList

A versatile component for displaying key-value pairs in a structured, organized layout. Supports both horizontal and vertical layouts, multiple columns, borders, and different sizes. Perfect for showing detailed information like metadata, specifications, or summary data.

## Import

```rust
use gpui_component::description_list::{DescriptionList, DescriptionItem, DescriptionText};
```

## Usage

### Basic Description List

```rust
DescriptionList::new()
    .child("Name", "GPUI Component", 1)
    .child("Version", "0.1.0", 1)
    .child("License", "Apache-2.0", 1)
```

### Using DescriptionItem Builder

```rust
DescriptionList::new()
    .children([
        DescriptionItem::new("Name").value("GPUI Component"),
        DescriptionItem::new("Description").value("UI components for building desktop applications"),
        DescriptionItem::new("Version").value("0.1.0"),
    ])
```

### Different Layouts

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

### Multiple Columns with Spans

```rust
DescriptionList::new()
    .columns(3)
    .child(DescriptionItem::new("Name").value("GPUI Component").span(1))
    .children([
        DescriptionItem::new("Version").value("0.1.0").span(1),
        DescriptionItem::new("License").value("Apache-2.0").span(1),
        DescriptionItem::new("Description")
            .value("Full-featured UI components for desktop applications")
            .span(3), // Spans all 3 columns
        DescriptionItem::new("Repository")
            .value("https://github.com/longbridge/gpui-component")
            .span(2), // Spans 2 columns
    ])
```

### With Dividers

```rust
DescriptionList::new()
    .item("Name", "GPUI Component", 1)
    .item("Version", "0.1.0", 1)
    .divider() // Add a visual separator
    .item("Author", "Longbridge", 1)
    .item("License", "Apache-2.0", 1)
```

### Different Sizes

```rust
// Large size
DescriptionList::new()
    .large()
    .item("Title", "Large Description List", 1)

// Medium size (default)
DescriptionList::new()
    .item("Title", "Medium Description List", 1)

// Small size
DescriptionList::new()
    .small()
    .item("Title", "Small Description List", 1)
```

### Without Borders

```rust
DescriptionList::new()
    .bordered(false) // Remove borders for a cleaner look
    .item("Name", "GPUI Component", 1)
    .item("Type", "UI Library", 1)
```

### Custom Label Width (Horizontal Layout)

```rust
use gpui::px;

DescriptionList::horizontal()
    .label_width(px(200.0)) // Set custom label width
    .item("Very Long Label Name", "Short Value", 1)
    .item("Short", "Very long value that needs more space", 1)
```

### Rich Content with Custom Elements

```rust
use gpui_component::text::markdown;

DescriptionList::new()
    .columns(2)
    .children([
        DescriptionItem::new("Name").value("GPUI Component"),
        DescriptionItem::new("Description").value(
            markdown(
                "UI components for building **fantastic** desktop applications.",
            ).into_any_element()
        ),
    ])
```

### Complex Example with Mixed Content

```rust
DescriptionList::new()
    .columns(3)
    .label_width(px(150.0))
    .children([
        DescriptionItem::new("Project Name").value("GPUI Component").span(1),
        DescriptionItem::new("Version").value("0.1.0").span(1),
        DescriptionItem::new("Status").value("Active").span(1),

        DescriptionItem::Divider, // Full-width divider

        DescriptionItem::new("Description").value(
            "A comprehensive UI component library for building desktop applications with GPUI"
        ).span(3),

        DescriptionItem::new("Repository").value(
            "https://github.com/longbridge/gpui-component"
        ).span(2),
        DescriptionItem::new("License").value("Apache-2.0").span(1),

        DescriptionItem::new("Platforms").value("macOS, Windows, Linux").span(2),
        DescriptionItem::new("Language").value("Rust").span(1),
    ])
```

## Examples

### User Profile Information

```rust
DescriptionList::new()
    .columns(2)
    .bordered(true)
    .children([
        DescriptionItem::new("Full Name").value("John Doe"),
        DescriptionItem::new("Email").value("john@example.com"),
        DescriptionItem::new("Phone").value("+1 (555) 123-4567"),
        DescriptionItem::new("Department").value("Engineering"),
        DescriptionItem::Divider,
        DescriptionItem::new("Bio").value(
            "Senior software engineer with 10+ years of experience in Rust and system programming."
        ).span(2),
    ])
```

### System Information

```rust
DescriptionList::vertical()
    .small()
    .bordered(false)
    .children([
        DescriptionItem::new("Operating System").value("macOS 14.0"),
        DescriptionItem::new("Architecture").value("Apple Silicon (M2)"),
        DescriptionItem::new("Memory").value("16 GB"),
        DescriptionItem::new("Storage").value("512 GB SSD"),
        DescriptionItem::new("GPU").value("Apple M2 10-core GPU"),
    ])
```

### Product Specifications

```rust
DescriptionList::new()
    .columns(3)
    .large()
    .children([
        DescriptionItem::new("Model").value("MacBook Pro").span(1),
        DescriptionItem::new("Year").value("2023").span(1),
        DescriptionItem::new("Screen Size").value("14-inch").span(1),

        DescriptionItem::new("Processor").value("Apple M2 Pro").span(2),
        DescriptionItem::new("Base Price").value("$1,999").span(1),

        DescriptionItem::Divider,

        DescriptionItem::new("Key Features").value(
            "Liquid Retina XDR display, ProMotion technology, P3 wide color gamut"
        ).span(3),
    ])
```

### Configuration Settings

```rust
DescriptionList::horizontal()
    .label_width(px(180.0))
    .bordered(false)
    .children([
        DescriptionItem::new("Theme").value("Dark Mode"),
        DescriptionItem::new("Font Size").value("14px"),
        DescriptionItem::new("Auto Save").value("Enabled"),
        DescriptionItem::new("Backup Frequency").value("Every 30 minutes"),
        DescriptionItem::new("Language").value("English (US)"),
    ])
```

## Design Guidelines

* Use horizontal layout for simple key-value pairs
* Use vertical layout when values are lengthy or complex
* Limit columns to 3-4 for optimal readability
* Use dividers to group related information
* Keep labels concise and descriptive
* Use consistent spacing with the size prop
* Consider removing borders for embedded contexts

---

---
url: /gpui-component/docs/components/dialog.md
description: A dialog dialog for displaying content in a layer above the app.
---

# Dialog

Dialog component for creating dialogs, confirmations, and alerts. Supports overlay, keyboard shortcuts, and various customizations.

## Import

```rust
use gpui_component::dialog::DialogButtonProps;
use gpui_component::WindowExt;
```

## Usage

### Setup application root view for display of dialogs

You need to set up your application's root view to render the dialog layer. This is typically done in your main application struct's render method.

The [Root::render\_dialog\_layer](https://docs.rs/gpui-component/latest/gpui_component/struct.Root.html#method.render_dialog_layer) function handles rendering any active dialogs on top of your app content.

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

### Basic Dialog

```rust
window.open_dialog(cx, |dialog, _, _| {
    dialog
        .title("Welcome")
        .child("This is a dialog dialog.")
})
```

### Form Dialog

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

### Confirm Dialog

```rust
window.open_dialog(cx, |dialog, _, _| {
    dialog
        .confirm()
        .child("Are you sure you want to delete this item?")
        .on_ok(|_, window, cx| {
            window.push_notification("Item deleted", cx);
            true // Return true to close dialog
        })
        .on_cancel(|_, window, cx| {
            window.push_notification("Cancelled", cx);
            true
        })
})
```

### Alert Dialog

```rust
window.open_dialog(cx, |dialog, _, _| {
    dialog
        .alert()
        .child("Operation completed successfully!")
        .on_close(|_, window, cx| {
            window.push_notification("Alert closed", cx);
        })
})
```

### Custom Button Labels

```rust
use gpui_component::button::ButtonVariant;

window.open_dialog(cx, |dialog, _, _| {
    dialog
        .confirm()
        .child("Update available. Restart now?")
        .button_props(
            DialogButtonProps::default()
                .cancel_text("Later")
                .cancel_variant(ButtonVariant::Secondary)
                .ok_text("Restart Now")
                .ok_variant(ButtonVariant::Danger)
        )
        .on_ok(|_, window, cx| {
            window.push_notification("Restarting...", cx);
            true
        })
})
```

### Dialog with Icon

```rust
window.open_dialog(cx, |dialog, _, cx| {
    dialog
        .confirm()
        .child(
            h_flex()
                .gap_3()
                .child(Icon::new(IconName::TriangleAlert)
                    .size_6()
                    .text_color(cx.theme().warning))
                .child("This action cannot be undone.")
        )
})
```

### Scrollable Dialog

```rust
use gpui_component::text::markdown;

window.open_dialog(cx, |dialog, window, cx| {
    dialog
        .h(px(450.))
        .title("Long Content")
        .child(markdown(long_markdown_text))
})
```

### Dialog Options

```rust
window.open_dialog(cx, |dialog, _, _| {
    dialog
        .title("Custom Dialog")
        .overlay(true)              // Show overlay (default: true)
        .overlay_closable(true)     // Click overlay to close (default: true)
        .keyboard(true)             // ESC to close (default: true)
        .close_button(false)        // Show close button (default: true)
        .child("Dialog content")
})
```

### Nested Dialogs

```rust
window.open_dialog(cx, |dialog, _, _| {
    dialog
        .title("First Dialog")
        .child("This is the first dialog")
        .footer(|_, _, _, _| {
            vec![
                Button::new("open-another")
                    .label("Open Another Dialog")
                    .on_click(|_, window, cx| {
                        window.open_dialog(cx, |dialog, _, _| {
                            dialog
                                .title("Second Dialog")
                                .child("This is nested")
                        });
                    }),
            ]
        })
})
```

### Custom Styling

```rust
window.open_dialog(cx, |dialog, _, cx| {
    dialog
        .rounded_lg()
        .bg(cx.theme().cyan)
        .text_color(cx.theme().info_foreground)
        .title("Custom Style")
        .child("Styled dialog content")
})
```

### Custom Padding

```rust
window.open_dialog(cx, |dialog, _, _| {
    dialog
        .p_3()                      // Custom padding
        .title("Custom Padding")
        .child("Dialog with custom spacing")
})
```

### Close Dialog Programmatically

The `close_dialog` method can be used to close the active dialog from anywhere within the window context.

```rust
// Close top level active dialog.
window.close_dialog(cx);

// Close and perform action
Button::new("submit")
    .primary()
    .label("Submit")
    .on_click(|_, window, cx| {
        // Do something
        window.close_dialog(cx);
    })
```

## Examples

### Delete Confirmation

```rust
Button::new("delete")
    .danger()
    .label("Delete")
    .on_click(|_, window, cx| {
        window.open_dialog(cx, |dialog, _, _| {
            dialog
                .confirm()
                .child("Are you sure you want to delete this item?")
                .on_ok(|_, window, cx| {
                    // Perform delete
                    window.push_notification("Deleted", cx);
                    true
                })
        });
    })
```

### Success Alert

```rust
window.open_dialog(cx, |dialog, _, _| {
    dialog
        .confirm()
        .alert()
        .child("Your changes have been saved successfully!")
        .on_close(|_, _, _| {
            // Optional close handler
        })
})
```

---

---
url: /gpui-component/docs/components/dropdown_button.md
description: >-
  A DropdownButton is a combination of a button and a trigger button. It allows
  us to display a dropdown menu when the trigger is clicked, but the left Button
  can still respond to independent events.
---

# DropdownButton

A [DropdownButton] is a combination of a button and a trigger button. It allows us to display a dropdown menu when the trigger is clicked, but the left Button can still respond to independent events.

And more option methods of [Button] are also available for the DropdownButton, such as setting different variants using [ButtonCustomVariant], sizes using [Sizable], adding icons, loading states.

## Import

```rust
use gpui_component::button::{Button, DropdownButton};
```

## Usage

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

### Variants

Same as [Button], DropdownButton supports different variants.

````rust
DropdownButton::new("dropdown")
    .primary()
    .button(Button::new("btn").label("Primary"))
    .dropdown_menu(|menu, _, _| {
        menu.menu("Option 1", Box::new(MyAction))
    })
```

### With custom anchor

```rust
// With custom anchor
DropdownButton::new("dropdown")
    .button(Button::new("btn").label("Click Me"))
    .dropdown_menu_with_anchor(Corner::BottomRight, |menu, _, _| {
        menu.menu("Option 1", Box::new(MyAction))
    })
````

[Button]: https://docs.rs/gpui-component/latest/gpui_component/button/struct.Button.html

[DropdownButton]: https://docs.rs/gpui-component/latest/gpui_component/button/struct.DropdownButton.html

[ButtonCustomVariant]: https://docs.rs/gpui-component/latest/gpui_component/button/struct.ButtonCustomVariant.html

[Sizable]: https://docs.rs/gpui-component/latest/gpui_component/trait.Sizable.html

---

---
url: /gpui-component/docs/components/editor.md
description: >-
  Multi-line text input component with auto-resize, validation, and advanced
  editing features.
---

# Editor

A powerful multi-line text input component that extends the basic input functionality with support for multiple lines, auto-resizing, syntax highlighting, line numbers, and code editing features. Perfect for forms, code editors, and content editing.

## Import

```rust
use gpui_component::input::{InputState, Input};
```

## Usage

### Textarea

```rust
let state = cx.new(|cx|
    InputState::new(window, cx)
        .multi_line(true)
        .placeholder("Enter your message...")
);

Input::new(&state)
```

With fixed height Textarea:

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

### AutoGrow

```rust
let state = cx.new(|cx|
    InputState::new(window, cx)
        .auto_grow(1, 5) // min_rows: 1, max_rows: 5
        .placeholder("Type here and watch it grow...")
);

Input::new(&state)
```

### CodeEditor

GPUI Component's `InputState` supports a code editor mode with syntax highlighting, line numbers, and search functionality.

It design for high performance and can handle large files efficiently. We
used [tree-sitter](https://tree-sitter.github.io/tree-sitter/) for syntax highlighting, and [ropey](https://github.com/cessen/ropey) for text storage and manipulation.

```rust
let state = cx.new(|cx|
    InputState::new(window, cx)
        .code_editor("rust") // Language for syntax highlighting
        .line_number(true) // Show line numbers
        .searchable(true) // Enable search functionality
        .default_value("fn main() {\n    println!(\"Hello, world!\");\n}")
);

Input::new(&state)
    .h_full() // Full height
```

#### Single Line Mode

Sometimes you may want to use the code editor features but restrict input to a single line, for example for code snippets or commands.

```rust
let state = cx.new(|cx|
    InputState::new(window, cx)
        .code_editor("rust")
        .multi_line(false) // Single line
        .default_value("println!(\"Hello, world!\");")
);

Input::new(&state)
```

### TabSize

```rust
use gpui_component::input::TabSize;

let state = cx.new(|cx|
    InputState::new(window, cx)
        .multi_line(true)
        .tab_size(TabSize {
            tab_size: 4,
            hard_tabs: false, // Use spaces instead of tabs
        })
);

Input::new(&state)
```

### Searchable

The search feature allows for all multi-line inputs to support searching through the content using `Ctrl+F` (or `Cmd+F` on Mac).

It provides a search bar with options to navigate between matches and highlight them.

Use `searchable` method to enable:

```rust
let state = cx.new(|cx|
    InputState::new(window, cx)
        .multi_line(true)
        .searchable(true) // Enable Ctrl+F search
        .rows(15)
        .default_value("Search through this content...")
);

Input::new(&state)
```

### SoftWrap

By default multi-line inputs have soft wrapping enabled, meaning long lines will wrap to fit the width of the textarea.

You can disable soft wrapping to allow horizontal scrolling instead:

```rust
// With soft wrap (default)
let state = cx.new(|cx|
    InputState::new(window, cx)
        .multi_line(true)
        .soft_wrap(true)
        .rows(6)
);

// Without soft wrap (horizontal scrolling)
let state = cx.new(|cx|
    InputState::new(window, cx)
        .multi_line(true)
        .soft_wrap(false)
        .rows(6)
        .default_value("This is a very long line that will not wrap automatically but will show horizontal scrollbar instead.")
);
```

### Text Manipulation

```rust
// Insert text at cursor position
state.update(cx, |state, cx| {
    state.insert("inserted text", window, cx);
});

// Replace all content
state.update(cx, |state, cx| {
    state.replace("new content", window, cx);
});

// Set cursor position
state.update(cx, |state, cx| {
    state.set_cursor_position(Position { line: 2, character: 5 }, window, cx);
});

// Get cursor position
let position = state.read(cx).cursor_position();
println!("Line: {}, Column: {}", position.line, position.character);
```

### Validation

```rust
let state = cx.new(|cx|
    InputState::new(window, cx)
        .multi_line(true)
        .validate(|text, _| {
            // Validate that content is not empty and under 1000 chars
            !text.trim().is_empty() && text.len() <= 1000
        })
);

Input::new(&state)
```

### Handle Events

```rust
cx.subscribe_in(&state, window, |view, state, event, window, cx| {
    match event {
        InputEvent::Change => {
            let content = state.read(cx).value();
            println!("Content changed: {} characters", content.len());
        }
        InputEvent::PressEnter { secondary } => {
            if secondary {
                println!("Shift+Enter pressed - insert line break");
            } else {
                println!("Enter pressed - could submit form");
            }
        }
        InputEvent::Focus => println!("Textarea focused"),
        InputEvent::Blur => println!("Textarea blurred"),
    }
});
```

### Disabled State

```rust
Input::new(&state)
    .disabled(true)
    .h(px(200.))
```

### Custom Styling

```rust
// Without default appearance
Input::new(&state)
    .appearance(false)
    .h(px(200.))

// Custom container styling
div()
    .bg(cx.theme().background)
    .border_2()
    .border_color(cx.theme().input)
    .rounded_lg()
    .p_4()
    .child(
        Input::new(&state)
            .appearance(false)
            .h(px(150.))
    )
```

## Examples

### Comment Box

```rust
struct CommentBox {
    state: Entity<InputState>,
    char_limit: usize,
}

impl CommentBox {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let state = cx.new(|cx|
            InputState::new(window, cx)
                .auto_grow(3, 8)
                .placeholder("Write your comment...")
                .validate(|text, _| text.len() <= 500)
        );

        Self {
            state,
            char_limit: 500,
        }
    }
}

impl Render for CommentBox {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let content = self.state.read(cx).value();
        let char_count = content.len();
        let remaining = self.char_limit.saturating_sub(char_count);

        v_flex()
            .gap_2()
            .child(Input::new(&self.state))
            .child(
                h_flex()
                    .justify_between()
                    .child(
                        div()
                            .text_xs()
                            .text_color(cx.theme().muted_foreground)
                            .child(format!("{} characters remaining", remaining))
                    )
                    .child(
                        Button::new("submit")
                            .primary()
                            .disabled(char_count == 0 || char_count > self.char_limit)
                            .label("Post Comment")
                    )
            )
    }
}
```

### Code Editor with Language Selection

```rust
struct CodeEditor {
    editor: Entity<InputState>,
    language: String,
}

impl CodeEditor {
    fn set_language(&mut self, language: String, window: &mut Window, cx: &mut Context<Self>) {
        self.language = language.clone();
        self.editor.update(cx, |editor, cx| {
            editor.set_highlighter(language, cx);
        });
    }
}

impl Render for CodeEditor {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_3()
            .child(
                h_flex()
                    .gap_2()
                    .child("Language:")
                    .child(
                        // Language selector dropdown would go here
                        div().child(self.language.clone())
                    )
            )
            .child(
                Input::new(&self.editor)
                    .h(px(400.))
                    .bordered(true)
            )
    }
}
```

### Text Editor with Toolbar

```rust
struct TextEditor {
    editor: Entity<InputState>,
}

impl TextEditor {
    fn format_bold(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.editor.update(cx, |editor, cx| {
            if !editor.selected_range.is_empty() {
                let selected = editor.selected_text().to_string();
                editor.replace(&format!("**{}**", selected), window, cx);
            }
        });
    }
}

impl Render for TextEditor {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_2()
            .child(
                h_flex()
                    .gap_1()
                    .p_2()
                    .border_b_1()
                    .border_color(cx.theme().border)
                    .child(
                        Button::new("bold")
                            .ghost()
                            .icon(IconName::Bold)
                            .on_click(cx.listener(Self::format_bold))
                    )
                    .child(
                        Button::new("italic")
                            .ghost()
                            .icon(IconName::Italic)
                    )
            )
            .child(
                Input::new(&self.editor)
                    .h(px(300.))
            )
    }
}
```

---

---
url: /gpui-component/docs/element_id.md
description: To introduce the ElementId concept in GPUI.
---

The [ElementId] is a unique identifier for a GPUI element. It is used to reference elements in the GPUI component tree.

Before you start using GPUI and GPUI Component, you need to understand the [ElementId].

For example:

```rs
div().id("my-element").child("Hello, World!")
```

In this case, the `div` element has an `id` of `"my-element"`. The add `id` is used for GPUI for binding events, for example `on_click` or `on_mouse_move`, the `element` with `id` in GPUI we call [Stateful\<E>][Stateful<E>].

We also use `id` (actually, it uses [GlobalElementId] internally in GPUI) to manage the `state` in some elements, by using `window.use_keyed_state`, so it is important to keep the `id` unique.

## Unique

The `id` should be unique within the layout scope (In a same [Stateful\<E>][Stateful<E>] parent).

For example we have a list with multiple items:

```rs
div().id("app").child(
    div().id("list1").child(vec![
        div().id(1).child("Item 1"),
        div().id(2).child("Item 2"),
        div().id(3).child("Item 3"),
    ])
).child(
    div().id("list2").child(vec![
        div().id(1).child("Item 1"),
    ])
)
```

In this case, we can named the child items with a very simple id, because they are have a parent `list1` element with an `id`.

GPUI internal will generate [GlobalElementId] with the parent elements's `id`, in this example, the `Item 1` will have global\_id:

```rs
["app", "list1", 1]
```

And the `Item 1` in `list2` will have global\_id:

```rs
["app", "list2", 1]
```

So we can named the child items with a very simple id.

[ElementId]: https://docs.rs/gpui/latest/gpui/enum.ElementId.html

[GlobalElementId]: https://docs.rs/gpui/latest/gpui/struct.GlobalElementId.html

[Stateful]: https://docs.rs/gpui/latest/gpui/struct.Stateful.html

[Stateful<E>]: https://docs.rs/gpui/latest/gpui/struct.Stateful.html

---

---
url: /gpui-component/docs/components/form.md
description: >-
  Flexible form container with support for field layout, validation, and
  multi-column layouts.
---

# Form

A comprehensive form component that provides structured layout for form fields with support for vertical/horizontal layouts, validation, field groups, and responsive multi-column layouts.

## Import

```rust
use gpui_component::form::{field, v_form, h_form, Form, Field};
```

## Usage

### Basic Form

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

### Horizontal Form Layout

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

### Multi-Column Form

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

## Form Container and Layout

### Vertical Layout (Default)

```rust
v_form()
    .gap(px(12.))
    .child(field().label("Name").child(input))
    .child(field().label("Email").child(email_input))
```

### Horizontal Layout

```rust
h_form()
    .label_width(px(100.))
    .child(field().label("Name").child(input))
    .child(field().label("Email").child(email_input))
```

### Custom Sizing

```rust
v_form()
    .large() // Large form size
    .label_text_size(rems(1.2))
    .child(field().label("Title").child(input))

v_form()
    .small() // Small form size
    .child(field().label("Code").child(input))
```

## Form Validation

### Required Fields

```rust
field()
    .label("Email")
    .required(true) // Shows asterisk (*) next to label
    .child(Input::new(&email_input))
```

### Field Descriptions

```rust
field()
    .label("Password")
    .description("Must be at least 8 characters long")
    .child(Input::new(&password_input))
```

### Dynamic Descriptions

```rust
field()
    .label("Bio")
    .description_fn(|_, _| {
        div().child("Use at most 100 words to describe yourself.")
    })
    .child(Input::new(&bio_input))
```

### Field Visibility

```rust
field()
    .label("Admin Settings")
    .visible(user.is_admin()) // Conditionally show field
    .child(Switch::new("admin-mode"))
```

## Submit Handling

### Basic Submit Pattern

```rust
struct FormView {
    name_input: Entity<InputState>,
    email_input: Entity<InputState>,
}

impl FormView {
    fn submit(&mut self, cx: &mut Context<Self>) {
        let name = self.name_input.read(cx).value();
        let email = self.email_input.read(cx).value();

        // Validate inputs
        if name.is_empty() || email.is_empty() {
            // Show validation error
            return;
        }

        // Submit form data
        self.handle_submit(name, email, cx);
    }
}

// Form with submit button
v_form()
    .child(field().label("Name").child(Input::new(&self.name_input)))
    .child(field().label("Email").child(Input::new(&self.email_input)))
    .child(
        field()
            .label_indent(false)
            .child(
                Button::new("submit")
                    .primary()
                    .child("Submit")
                    .on_click(cx.listener(|this, _, _, cx| this.submit(cx)))
            )
    )
```

### Form with Action Buttons

```rust
v_form()
    .child(field().label("Title").child(Input::new(&title)))
    .child(field().label("Content").child(Input::new(&content)))
    .child(
        field()
            .label_indent(false)
            .child(
                h_flex()
                    .gap_2()
                    .child(Button::new("save").primary().child("Save"))
                    .child(Button::new("cancel").child("Cancel"))
                    .child(Button::new("preview").outline().child("Preview"))
            )
    )
```

## Field Groups

### Related Fields

```rust
v_form()
    .child(
        field()
            .label("Name")
            .child(
                h_flex()
                    .gap_2()
                    .child(div().flex_1().child(Input::new(&first_name)))
                    .child(div().flex_1().child(Input::new(&last_name)))
            )
    )
    .child(
        field()
            .label("Address")
            .items_start() // Align to start for multi-line content
            .child(
                v_flex()
                    .gap_2()
                    .child(Input::new(&street))
                    .child(
                        h_flex()
                            .gap_2()
                            .child(div().flex_1().child(Input::new(&city)))
                            .child(div().w(px(100.)).child(Input::new(&zip)))
                    )
            )
    )
```

### Custom Field Components

```rust
field()
    .label("Theme Color")
    .child(ColorPicker::new(&color_state).small())

field()
    .label("Birth Date")
    .description("We'll send you a birthday gift!")
    .child(DatePicker::new(&date_state))

field()
    .label("Notifications")
    .child(
        v_flex()
            .gap_2()
            .child(Switch::new("email").label("Email notifications"))
            .child(Switch::new("push").label("Push notifications"))
            .child(Switch::new("sms").label("SMS notifications"))
    )
```

### Conditional Fields

```rust
v_form()
    .child(
        field()
            .label("Account Type")
            .child(Select::new(&account_type))
    )
    .child(
        field()
            .label("Company Name")
            .visible(is_business_account) // Show only for business accounts
            .child(Input::new(&company_name))
    )
    .child(
        field()
            .label("Tax ID")
            .visible(is_business_account)
            .required(is_business_account)
            .child(Input::new(&tax_id))
    )
```

## Grid Layout and Positioning

### Column Spanning

```rust
v_form()
    .columns(3) // Three-column grid
    .child(field().label("First").child(input1))
    .child(field().label("Second").child(input2))
    .child(field().label("Third").child(input3))
    .child(
        field()
            .label("Full Width")
            .col_span(3) // Spans all three columns
            .child(Input::new(&full_width))
    )
```

### Column Positioning

```rust
v_form()
    .columns(4)
    .child(field().label("A").child(input_a))
    .child(field().label("B").child(input_b))
    .child(
        field()
            .label("Positioned")
            .col_start(1) // Start at column 1
            .col_span(2)  // Span 2 columns
            .child(input_positioned)
    )
```

### Responsive Layout

```rust
v_form()
    .columns(if is_mobile { 1 } else { 2 })
    .child(field().label("Name").child(name_input))
    .child(field().label("Email").child(email_input))
    .child(
        field()
            .label("Bio")
            .when(!is_mobile, |field| field.col_span(2))
            .child(bio_input)
    )
```

## Examples

### User Registration Form

```rust
struct RegistrationForm {
    first_name: Entity<InputState>,
    last_name: Entity<InputState>,
    email: Entity<InputState>,
    password: Entity<InputState>,
    confirm_password: Entity<InputState>,
    terms_accepted: bool,
}

impl Render for RegistrationForm {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_form()
            .large()
            .child(
                field()
                    .label("Personal Information")
                    .label_indent(false)
                    .child(
                        h_flex()
                            .gap_3()
                            .child(
                                div().flex_1().child(
                                    Input::new(&self.first_name)
                                        .placeholder("First name")
                                )
                            )
                            .child(
                                div().flex_1().child(
                                    Input::new(&self.last_name)
                                        .placeholder("Last name")
                                )
                            )
                    )
            )
            .child(
                field()
                    .label("Email")
                    .required(true)
                    .child(Input::new(&self.email))
            )
            .child(
                field()
                    .label("Password")
                    .required(true)
                    .description("Must be at least 8 characters")
                    .child(Input::new(&self.password))
            )
            .child(
                field()
                    .label("Confirm Password")
                    .required(true)
                    .child(Input::new(&self.confirm_password))
            )
            .child(
                field()
                    .label_indent(false)
                    .child(
                        Checkbox::new("terms")
                            .label("I agree to the Terms of Service")
                            .checked(self.terms_accepted)
                            .on_click(cx.listener(|this, checked, _, cx| {
                                this.terms_accepted = *checked;
                                cx.notify();
                            }))
                    )
            )
            .child(
                field()
                    .label_indent(false)
                    .child(
                        Button::new("register")
                            .primary()
                            .large()
                            .w_full()
                            .child("Create Account")
                    )
            )
    }
}
```

### Settings Form with Sections

```rust
v_form()
    .column(2)
    .child(
        field()
            .label("Profile")
            .label_indent(false)
            .col_span(2)
            .child(Divider::horizontal())
    )
    .child(
        field()
            .label("Display Name")
            .child(Input::new(&display_name))
    )
    .child(
        field()
            .label("Email")
            .child(Input::new(&email))
    )
    .child(
        field()
            .label("Bio")
            .col_span(2)
            .items_start()
            .child(Input::new(&bio))
    )
    .child(
        field()
            .label("Preferences")
            .label_indent(false)
            .col_span(2)
            .child(Divider::horizontal())
    )
    .child(
        field()
            .label("Theme")
            .child(Select::new(&theme_state))
    )
    .child(
        field()
            .label("Language")
            .child(Select::new(&language_state))
    )
    .child(
        field()
            .label_indent(false)
            .child(Switch::new("notifications").label("Enable notifications"))
    )
    .child(
        field()
            .label_indent(false)
            .child(Switch::new("marketing").label("Marketing emails"))
    )
```

### Contact Form

```rust
v_form()
    .child(
        field()
            .label("Contact Information")
            .child(
                h_flex()
                    .gap_2()
                    .child(
                        Select::new(&prefix_state)
                            .w(px(80.))
                    )
                    .child(
                        div().flex_1().child(
                            Input::new(&name_input)
                                .placeholder("Your name")
                        )
                    )
            )
    )
    .child(
        field()
            .label("Email")
            .required(true)
            .child(Input::new(&email_input))
    )
    .child(
        field()
            .label("Subject")
            .child(Select::new(&subject_state))
    )
    .child(
        field()
            .label("Message")
            .required(true)
            .items_start()
            .description("Please describe your inquiry in detail")
            .child(Input::new(&message_input))
    )
    .child(
        field()
            .label_indent(false)
            .child(
                h_flex()
                    .gap_2()
                    .justify_between()
                    .child(
                        Checkbox::new("copy")
                            .label("Send me a copy")
                    )
                    .child(
                        h_flex()
                            .gap_2()
                            .child(Button::new("cancel").child("Cancel"))
                            .child(Button::new("send").primary().child("Send Message"))
                    )
            )
    )
```

---

---
url: /gpui-component/docs/getting-started.md
description: Learn how to set up and use GPUI Component in your project
---

# Getting Started

## Installation

Add dependencies to your `Cargo.toml`:

```toml-vue
[dependencies]
gpui = "{{ GPUI_VERSION }}"
gpui-component = "{{ VERSION }}"
# Optional, for default bundled assets
gpui-component-assets = "{{ VERSION }}"
anyhow = "1.0"
```

:::tip
The `gpui-component-assets` crate is optional.

It provides a default set of icon assets. If you want to manage your own assets, you can skip adding this dependency.

See [Icons & Assets](./assets.md) for more details.
:::

## Quick Start

Here's a simple example to get you started:

```rust
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
```

:::info
Make sure to call `gpui_component::init(cx);` at first line inside the `app.run` closure. This initializes the GPUI Component system.

This is required for theming and other global settings to work correctly.
:::

## Basic Concepts

### Stateless Elements

GPUI Component uses stateless [RenderOnce] elements, making them simple and predictable. State management is handled at the view level, not in individual components.

The are all implemented [IntoElement] types.

For example:

```rs
struct MyView;

impl Render for MyView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .child(Button::new("btn").label("Click Me"))
            .child(Tag::secondary().child("Secondary"))
    }
}
```

### Stateful Components

There are some stateful components like `Dropdown`, `List`, and `Table` that manage their own internal state for convenience, these components implement the [Render] trait.

Those components to use are a bit different, we need create the \[Entity] and hold it in the view struct.

```rs
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
```

### Theming

All components support theming through the built-in `Theme` system:

```rust
use gpui_component::{ActiveTheme, Theme};

// Access theme colors in your components
cx.theme().primary
cx.theme().background
cx.theme().foreground
```

### Sizing

Most components support multiple sizes:

```rust
Button::new("btn").small()
Button::new("btn").medium() // default
Button::new("btn").large()
Button::new("btn").xsmall()
```

### Variants

Components offer different visual variants:

```rust
Button::new("btn").primary()
Button::new("btn").danger()
Button::new("btn").warning()
Button::new("btn").success()
Button::new("btn").ghost()
Button::new("btn").outline()
```

## Icons

:::info
Icons are not bundled with GPUI Component to keep the library lightweight.

Continue read [Icons & Assets](./assets.md) to learn how to add icons to your project.
:::

GPUI Component has an `Icon` element, but does not include SVG files by default.

The examples use [Lucide](https://lucide.dev) icons. You can use any icons you like by naming the SVG files as defined in `IconName`. Add the icons you need to your project.

```rust
use gpui_component::{Icon, IconName};

Icon::new(IconName::Check)
Icon::new(IconName::Search).small()
```

## Next Steps

Explore the component documentation to learn more about each component:

* [Button](./components/button) - Interactive button component
* [Input](./components/input) - Text input with validation
* [Dialog](./components/dialog) - Dialog and modal windows
* [Table](./components/table) - High-performance data tables
* [More components...](./components/index)

## Development

To run the component gallery:

```bash
cargo run
```

More examples can be found in the `examples` directory:

```bash
cargo run --example <example_name>
```

[RenderOnce]: https://docs.rs/gpui/latest/gpui/trait.RenderOnce.html

[IntoElement]: https://docs.rs/gpui/latest/gpui/trait.IntoElement.html

[Render]: https://docs.rs/gpui/latest/gpui/trait.Render.html

---

---
url: /gpui-component/README.md
---
# gpui-component-docs

To install dependencies:

```bash
bun install
```

To run:

```bash
bun run dev
```

This project was created using `bun init` in bun v1.2.23. [Bun](https://bun.com) is a fast all-in-one JavaScript runtime.

---

---
url: /gpui-component/docs/components/group-box.md
description: >-
  A styled container element with an optional title to group related content
  together.
---

# GroupBox

The GroupBox component is a versatile container that groups related content together with optional borders, backgrounds, and titles. It provides visual organization and semantic grouping for form controls, settings panels, and other related UI elements.

## Import

```rust
use gpui_component::group_box::{GroupBox, GroupBoxVariant, GroupBoxVariants as _};
```

## Usage

### Basic GroupBox

```rust
GroupBox::new()
    .child("Subscriptions")
    .child(Checkbox::new("all").label("All"))
    .child(Checkbox::new("newsletter").label("Newsletter"))
    .child(Button::new("save").primary().label("Save"))
```

### GroupBox Variants

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

### With Title

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

### Custom ID

```rust
GroupBox::new()
    .id("user-preferences")
    .outline()
    .title("User Preferences")
    .child("Preference controls...")
```

### Custom Title Styling

```rust
use gpui::{StyleRefinement, relative};

GroupBox::new()
    .outline()
    .title("Custom Title")
    .title_style(
        StyleRefinement::default()
            .font_semibold()
            .line_height(relative(1.0))
            .px_3()
            .text_color(cx.theme().accent)
    )
    .child("Content with custom title styling")
```

### Custom Content Styling

```rust
GroupBox::new()
    .fill()
    .title("Custom Content Area")
    .content_style(
        StyleRefinement::default()
            .rounded_xl()
            .py_3()
            .px_4()
            .border_2()
            .border_color(cx.theme().accent)
    )
    .child("Content with custom styling")
```

### Complex Example

```rust
GroupBox::new()
    .id("notification-settings")
    .outline()
    .bg(cx.theme().group_box)
    .rounded_xl()
    .p_5()
    .title("Notification Preferences")
    .title_style(
        StyleRefinement::default()
            .font_semibold()
            .line_height(relative(1.0))
            .px_3()
    )
    .content_style(
        StyleRefinement::default()
            .rounded_xl()
            .py_3()
            .px_4()
            .border_2()
    )
    .child(
        v_flex()
            .gap_3()
            .child(
                h_flex()
                    .justify_between()
                    .child("Email notifications")
                    .child(Switch::new("email").checked(true))
            )
            .child(
                h_flex()
                    .justify_between()
                    .child("Push notifications")
                    .child(Switch::new("push").checked(false))
            )
            .child(
                h_flex()
                    .justify_between()
                    .child("SMS notifications")
                    .child(Switch::new("sms").checked(false))
            )
    )
    .child(
        h_flex()
            .justify_end()
            .gap_2()
            .child(Button::new("cancel").label("Cancel"))
            .child(Button::new("save").primary().label("Save Settings"))
    )
```

## Examples

### Form Section

```rust
GroupBox::new()
    .fill()
    .title("Personal Information")
    .child(
        v_flex()
            .gap_4()
            .child(
                h_flex()
                    .gap_2()
                    .child(Input::new("first-name").placeholder("First Name"))
                    .child(Input::new("last-name").placeholder("Last Name"))
            )
            .child(Input::new("email").placeholder("Email Address"))
            .child(
                h_flex()
                    .justify_end()
                    .child(Button::new("update").primary().label("Update Profile"))
            )
    )
```

### Settings Panel

```rust
GroupBox::new()
    .outline()
    .title("Display Settings")
    .child(
        v_flex()
            .gap_3()
            .child(
                h_flex()
                    .justify_between()
                    .child(Label::new("Theme"))
                    .child(
                        RadioGroup::horizontal("theme")
                            .child(Radio::new("light").label("Light"))
                            .child(Radio::new("dark").label("Dark"))
                            .child(Radio::new("auto").label("Auto"))
                    )
            )
            .child(
                h_flex()
                    .justify_between()
                    .child(Label::new("Font Size"))
                    .child(
                        Select::new("font-size")
                            .option("small", "Small")
                            .option("medium", "Medium")
                            .option("large", "Large")
                    )
            )
    )
```

### Subscription Management

```rust
GroupBox::new()
    .title("Email Subscriptions")
    .child(
        v_flex()
            .gap_2()
            .child(Checkbox::new("newsletter").label("Weekly Newsletter"))
            .child(Checkbox::new("updates").label("Product Updates"))
            .child(Checkbox::new("security").label("Security Alerts"))
            .child(Checkbox::new("marketing").label("Marketing Communications"))
    )
    .child(
        h_flex()
            .justify_between()
            .mt_4()
            .child(Button::new("unsubscribe-all").link().label("Unsubscribe All"))
            .child(Button::new("save").primary().label("Update Preferences"))
    )
```

### Without Title

```rust
GroupBox::new()
    .outline()
    .child(
        h_flex()
            .justify_between()
            .items_center()
            .child("Enable two-factor authentication")
            .child(Switch::new("2fa").checked(false))
    )
```

## Styling

The GroupBox component supports extensive customization through both built-in variants and custom styling:

### Theme Integration

```rust
// Using theme colors
GroupBox::new()
    .fill()
    .bg(cx.theme().group_box)
    .title("Themed Group Box")
```

### Custom Appearance

```rust
GroupBox::new()
    .outline()
    .border_2()
    .border_color(cx.theme().accent)
    .rounded_lg()
    .title("Custom Styled Group Box")
    .title_style(
        StyleRefinement::default()
            .text_color(cx.theme().accent)
            .font_bold()
    )
```

## Best Practices

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

---
url: /gpui-component/docs/components/icon.md
description: 'Display SVG icons with various sizes, colors, and transformations.'
---

# Icon

A flexible icon component that renders SVG icons from the built-in icon library. Icons are based on Lucide.dev and support customization of size, color, and rotation. The component requires SVG files to be provided by the user in the assets bundle.

Before you start, please make sure you have read: [Icons & Assets](../assets.md) to understand how use SVG in GPUI & GPUI Component application.

## Import

```rust
use gpui_component::{Icon, IconName};
```

## Usage

### Basic Icon

```rust
// Using IconName enum directly
IconName::Heart

// Or creating an Icon explicitly
Icon::new(IconName::Heart)
```

### Icon with Custom Size

```rust
// Predefined sizes
Icon::new(IconName::Search).xsmall()   // size_3()
Icon::new(IconName::Search).small()    // size_3p5()
Icon::new(IconName::Search).medium()   // size_4() (default)
Icon::new(IconName::Search).large()    // size_6()

// Custom pixel size
Icon::new(IconName::Search).with_size(px(20.))
```

### Icon with Custom Color

```rust
// Using theme colors
Icon::new(IconName::Heart)
    .text_color(cx.theme().red)

// Using custom colors
Icon::new(IconName::Star)
    .text_color(gpui::red())
```

### Rotated Icons

```rust
use gpui::Radians;

// Rotate by radians
Icon::new(IconName::ArrowUp)
    .rotate(Radians::from_degrees(90.))

// Transform with custom transformation
Icon::new(IconName::ChevronRight)
    .transform(Transformation::rotate(Radians::PI))
```

### Custom SVG Path

```rust
// Using a custom SVG file from assets
Icon::new(Icon::empty())
    .path("icons/my-custom-icon.svg")
```

## Available Icons

The `IconName` enum provides access to a curated set of icons. Here are some commonly used ones:

### Navigation

* `ArrowUp`, `ArrowDown`, `ArrowLeft`, `ArrowRight`
* `ChevronUp`, `ChevronDown`, `ChevronLeft`, `ChevronRight`
* `ChevronsUpDown`

### Actions

* `Check`, `Close`, `Plus`, `Minus`
* `Copy`, `Delete`, `Search`, `Replace`
* `Maximize`, `Minimize`, `WindowRestore`

### Files & Folders

* `File`, `Folder`, `FolderOpen`, `FolderClosed`
* `BookOpen`, `Inbox`

### UI Elements

* `Menu`, `Settings`, `Settings2`, `Ellipsis`, `EllipsisVertical`
* `Eye`, `EyeOff`, `Bell`, `Info`

### Social & External

* `GitHub`, `Globe`, `ExternalLink`
* `Heart`, `HeartOff`, `Star`, `StarOff`
* `ThumbsUp`, `ThumbsDown`

### Status & Alerts

* `CircleCheck`, `CircleX`, `TriangleAlert`
* `Loader`, `LoaderCircle`

### Panels & Layout

* `PanelLeft`, `PanelRight`, `PanelBottom`
* `PanelLeftOpen`, `PanelRightOpen`, `PanelBottomOpen`
* `LayoutDashboard`, `Frame`

### Users & Profile

* `User`, `CircleUser`, `Bot`

### Other

* `Calendar`, `Map`, `Palette`, `Inspector`
* `Sun`, `Moon`, `Building2`

## Icon Sizes

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

```rust
use gpui_component::IconNamed;

pub enum IconName {
    Encounters,
    Monsters,
    Spells,
}

impl IconNamed for IconName {
    fn path(self) -> gpui::SharedString {
        match self {
            IconName::Encounters => "icons/encounters.svg",
            IconName::Monsters => "icons/monsters.svg",
            IconName::Spells => "icons/spells.svg",
        }
        .into()
    }
}

// This allows for the following interactions (works with anything that has the `.icon(icon)` method.
Button::new("my-button").icon(IconName::Spells);
Icon::new(IconName::Monsters);
```

If you want to directly `render` a custom `IconName` you must implement the `RenderOnce` trait and derive `IntoElement` on the `IconName`.

```rust
impl RenderOnce for IconName {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        Icon::empty().path(self.path())
    }
}

// Now you can use it directly in your element tree:
div()
    .child(IconName::Monsters)
```

## Examples

### Icon in Button

```rust
use gpui_component::button::Button;

Button::new("like-btn")
    .icon(
        Icon::new(IconName::Heart)
            .text_color(cx.theme().red)
            .large()
    )
    .label("Like")
```

### Animated Loading Icon

```rust
Icon::new(IconName::LoaderCircle)
    .text_color(cx.theme().muted_foreground)
    .medium()
    // Add rotation animation in your render logic
```

### Status Icons

```rust
// Success
Icon::new(IconName::CircleCheck)
    .text_color(cx.theme().green)

// Error
Icon::new(IconName::CircleX)
    .text_color(cx.theme().red)

// Warning
Icon::new(IconName::TriangleAlert)
    .text_color(cx.theme().yellow)
```

### Navigation Icons

```rust
// Back button
Icon::new(IconName::ArrowLeft)
    .medium()
    .text_color(cx.theme().foreground)

// Dropdown indicator
Icon::new(IconName::ChevronDown)
    .small()
    .text_color(cx.theme().muted_foreground)
```

### Custom Icon from Assets

```rust
// Using a custom SVG file
Icon::empty()
    .path("icons/my-brand-logo.svg")
    .large()
    .text_color(cx.theme().primary)
```

## Notes

* Icons are rendered as SVG elements and support full CSS styling
* The default size matches the current text size if no explicit size is set
* Icons are flex-shrink-0 by default to prevent unwanted shrinking in flex layouts
* All icon paths are relative to the assets bundle root
* Icons from Lucide.dev are designed to work well at 16px and scale nicely to other sizes

---

---
url: /gpui-component/docs/assets.md
---

# Icons & Assets

The [IconName] and [Icon] in GPUI Component provide a comprehensive set of icons and assets that can be easily integrated into your GPUI applications.

But for minimal size applications, **we have not embedded any icon assets by default** in `gpui-component` crate.

We split the icon assets into a separate crate [gpui-component-assets] to allow developers to choose whether to include the icon assets in their applications or if you don't need the icons at all, you can build your own assets.

## Use default bundled assets

The [gpui-component-assets] crate provides a default bundled assets implementation that includes all the icon files in the `assets/icons` folder.

To use the default bundled assets, you need to add the `gpui-component-assets` crate as a dependency in your `Cargo.toml`:

```toml-vue
[dependencies]
gpui-component = "{{ VERSION }}"
gpui-component-assets = "{{ VERSION }}"
```

Then we need call the `with_assets` method when creating the GPUI application to register the asset source:

```rs
use gpui::*;
use gpui_component_assets::Assets;

let app = Application::new().with_assets(Assets);
```

Now, we can use `IconName` and `Icon` in our application as usual, the all icon assets are loaded from the default bundled assets.

Continue [Use the icons](#use-the-icons) section to see how to use the icons in your application.

## Build you own assets

You may have a specific set of icons that you want to use in your application, or you may want to reduce the size of your application binary by including only the icons you need.

In this case, you can build your own assets by following these steps.

The [assets](https://github.com/longbridge/gpui-component/tree/main/crates/assets/assets/) folder in source code contains all the available icons in SVG format, every file is that GPUI Component support, it matched with the [IconName] enum.

You can download the SVG files you need from the [assets] folder, or you can use your own SVG files by following the [IconName] naming convention.

In GPUI application, we can use the [rust-embed] crate to embed the SVG files into the application binary.

And GPUI Application providers an `AssetSource` trait to load the assets.

```rs
use anyhow::anyhow;
use gpui::*;
use gpui_component::{v_flex, IconName, Root};
use rust_embed::RustEmbed;
use std::borrow::Cow;

/// An asset source that loads assets from the `./assets` folder.
#[derive(RustEmbed)]
#[folder = "./assets"]
#[include = "icons/**/*.svg"]
pub struct Assets;

impl AssetSource for Assets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        if path.is_empty() {
            return Ok(None);
        }

        Self::get(path)
            .map(|f| Some(f.data))
            .ok_or_else(|| anyhow!("could not find asset at path \"{path}\""))
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        Ok(Self::iter()
            .filter_map(|p| p.starts_with(path).then(|| p.into()))
            .collect())
    }
}
```

We need call the `with_assets` method when creating the GPUI application to register the asset source:

```rs
fn main() {
    // Register Assets to GPUI application.
    let app = Application::new().with_assets(Assets);

    app.run(move |cx| {
        // We must initialize gpui_component before using it.
        gpui_component::init(cx);

        cx.spawn(async move |cx| {
            cx.open_window(WindowOptions::default(), |window, cx| {
                let view = cx.new(|_| Example);
                // The first level on the window must be Root.
                cx.new(|cx| Root::new(view, window, cx))
            })?;

            Ok::<_, anyhow::Error>(())
        })
        .detach();
    });
}
```

## Use the icons

Now we can use the icons in our application:

```rs
pub struct Example;

impl Render for Example {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_2()
            .size_full()
            .items_center()
            .justify_center()
            .text_center()
            .child(IconName::Inbox)
            .child(IconName::Bot)
    }
}
```

## Resources

* [Lucide Icons](https://lucide.dev/) - The icon set used in GPUI Component is based on the open-source Lucide Icons library, which provides a wide range of customizable SVG icons.

[rust-embed]: https://docs.rs/rust-embed/latest/rust_embed/

[IconName]: https://docs.rs/gpui_component/latest/gpui_component/icon/enum.IconName.html

[Icon]: https://docs.rs/gpui_component/latest/gpui_component/icon/struct.Icon.html

[assets]: https://github.com/longbridge/gpui-component/tree/main/crates/assets/assets/

[gpui-component-assets]: https://crates.io/crates/gpui-component-assets

---

---
url: /gpui-component/docs/components/image.md
description: >-
  A flexible image display component with loading states, fallbacks, and
  responsive sizing options.
---

# Image

The Image component provides a robust way to display images with comprehensive fallback handling, loading states, and responsive sizing. Built on GPUI's native image support, it handles various image sources including URLs, local files, and SVG graphics with proper error handling and accessibility features.

## Import

```rust
use gpui::{img, ImageSource, ObjectFit};
use gpui_component::{v_flex, h_flex, div, Icon, IconName};
```

## Usage

### Basic Image

```rust
// Simple image from URL
img("https://example.com/image.jpg")

// Local image file
img("assets/logo.png")

// SVG image
img("icons/star.svg")
```

### Image with Sizing

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

### Object Fit Options

Control how images are scaled and positioned within their containers:

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

### Image with Fallback Handling

```rust
// Basic fallback with placeholder
fn image_with_fallback(src: &str, alt_text: &str) -> impl IntoElement {
    div()
        .w(px(300.))
        .h(px(200.))
        .bg(cx.theme().surface)
        .border_1()
        .border_color(cx.theme().border)
        .rounded(px(8.))
        .overflow_hidden()
        .child(
            img(src)
                .w_full()
                .h_full()
                .object_fit(ObjectFit::Cover)
                // Add error handling in practice
        )
}

// Fallback with icon placeholder
fn image_with_icon_fallback(src: &str) -> impl IntoElement {
    div()
        .size(px(200.))
        .bg(cx.theme().surface)
        .border_1()
        .border_color(cx.theme().border)
        .rounded(px(8.))
        .flex()
        .items_center()
        .justify_center()
        .child(
            img(src)
                .size_full()
                .object_fit(ObjectFit::Cover)
                // On error, show icon:
                // Icon::new(IconName::Image)
                //     .size(px(48.))
                //     .text_color(cx.theme().muted_foreground)
        )
}
```

### Loading States

```rust
// Image with loading skeleton
fn image_with_loading(src: &str, is_loading: bool) -> impl IntoElement {
    div()
        .w(px(400.))
        .h(px(300.))
        .rounded(px(8.))
        .overflow_hidden()
        .map(|this| {
            if is_loading {
                this.bg(cx.theme().muted)
                    .flex()
                    .items_center()
                    .justify_center()
                    .child("Loading...")
            } else {
                this.child(
                    img(src)
                        .w_full()
                        .h_full()
                        .object_fit(ObjectFit::Cover)
                )
            }
        })
}

// Progressive loading with placeholder
fn progressive_image(src: &str, placeholder_src: &str) -> impl IntoElement {
    div()
        .relative()
        .w(px(400.))
        .h(px(300.))
        .rounded(px(8.))
        .overflow_hidden()
        .child(
            // Low-quality placeholder
            img(placeholder_src)
                .absolute()
                .inset_0()
                .w_full()
                .h_full()
                .object_fit(ObjectFit::Cover)
                .opacity(0.5)
        )
        .child(
            // High-quality image
            img(src)
                .absolute()
                .inset_0()
                .w_full()
                .h_full()
                .object_fit(ObjectFit::Cover)
        )
}
```

### Responsive Images

```rust
// Responsive grid images
fn responsive_image_grid() -> impl IntoElement {
    div()
        .grid()
        .grid_cols(3)
        .gap_4()
        .child(
            img("https://example.com/photo1.jpg")
                .w_full()
                .aspect_ratio(1.0)  // Square aspect ratio
                .object_fit(ObjectFit::Cover)
                .rounded(px(8.))
        )
        .child(
            img("https://example.com/photo2.jpg")
                .w_full()
                .aspect_ratio(1.0)
                .object_fit(ObjectFit::Cover)
                .rounded(px(8.))
        )
        .child(
            img("https://example.com/photo3.jpg")
                .w_full()
                .aspect_ratio(1.0)
                .object_fit(ObjectFit::Cover)
                .rounded(px(8.))
        )
}

// Hero image with text overlay
fn hero_image() -> impl IntoElement {
    div()
        .relative()
        .w_full()
        .h(px(500.))
        .rounded(px(12.))
        .overflow_hidden()
        .child(
            img("https://example.com/hero-image.jpg")
                .absolute()
                .inset_0()
                .w_full()
                .h_full()
                .object_fit(ObjectFit::Cover)
        )
        .child(
            div()
                .absolute()
                .inset_0()
                .bg(rgba(0, 0, 0, 0.4))  // Dark overlay
                .flex()
                .items_center()
                .justify_center()
                .child(
                    v_flex()
                        .items_center()
                        .gap_4()
                        .child("Hero Title")
                        .child("Subtitle text here")
                )
        )
}
```

### Image Gallery

```rust
// Simple image gallery
fn image_gallery(images: Vec<&str>) -> impl IntoElement {
    v_flex()
        .gap_6()
        .child(
            // Main image
            div()
                .w_full()
                .h(px(400.))
                .rounded(px(12.))
                .overflow_hidden()
                .child(
                    img(images[0])
                        .w_full()
                        .h_full()
                        .object_fit(ObjectFit::Cover)
                )
        )
        .child(
            // Thumbnail row
            h_flex()
                .gap_3()
                .children(
                    images.iter().map(|src| {
                        div()
                            .size(px(80.))
                            .rounded(px(6.))
                            .overflow_hidden()
                            .border_2()
                            .border_color(cx.theme().border)
                            .cursor_pointer()
                            .hover(|this| this.border_color(cx.theme().primary))
                            .child(
                                img(*src)
                                    .size_full()
                                    .object_fit(ObjectFit::Cover)
                            )
                    })
                )
        )
}
```

### SVG Images

```rust
// SVG icon with custom styling
img("assets/icons/logo.svg")
    .size(px(64.))
    .text_color(cx.theme().primary)  // SVG color

// Inline SVG handling
img("data:image/svg+xml;base64,...")
    .w(px(32.))
    .h(px(32.))

// SVG with animation-friendly setup
img("assets/spinner.svg")
    .size(px(24.))
    .text_color(cx.theme().primary)
    // Add rotation animation in practice
```

## API Reference

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

### Sizing Methods

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

### Styling Methods

| Method                | Description             |
| --------------------- | ----------------------- |
| `rounded(radius)`     | Border radius           |
| `border_1()`          | 1px border              |
| `border_color(color)` | Border color            |
| `opacity(value)`      | Image opacity (0.0-1.0) |
| `shadow_sm()`         | Small shadow            |
| `shadow_lg()`         | Large shadow            |

## Examples

### Product Image Card

```rust
use gpui_component::{v_flex, div, Icon, IconName};

fn product_card(image_src: &str, title: &str, price: &str) -> impl IntoElement {
    v_flex()
        .gap_3()
        .p_4()
        .bg(cx.theme().card)
        .rounded(px(12.))
        .shadow_sm()
        .child(
            div()
                .relative()
                .w_full()
                .h(px(200.))
                .rounded(px(8.))
                .overflow_hidden()
                .bg(cx.theme().muted)
                .child(
                    img(image_src)
                        .w_full()
                        .h_full()
                        .object_fit(ObjectFit::Cover)
                )
                .child(
                    // Wishlist button
                    div()
                        .absolute()
                        .top_2()
                        .right_2()
                        .size(px(32.))
                        .bg(rgba(255, 255, 255, 0.9))
                        .rounded_full()
                        .flex()
                        .items_center()
                        .justify_center()
                        .cursor_pointer()
                        .child(Icon::new(IconName::Heart).size(px(16.)))
                )
        )
        .child(title)
        .child(price)
}
```

### Avatar with Image

```rust
fn custom_avatar(src: &str, name: &str, size: f32) -> impl IntoElement {
    div()
        .size(px(size))
        .rounded_full()
        .overflow_hidden()
        .border_2()
        .border_color(cx.theme().background)
        .shadow_sm()
        .child(
            img(src)
                .size_full()
                .object_fit(ObjectFit::Cover)
        )
}
```

### Image Comparison Slider

```rust
fn image_comparison(before_src: &str, after_src: &str) -> impl IntoElement {
    div()
        .relative()
        .w_full()
        .h(px(400.))
        .rounded(px(12.))
        .overflow_hidden()
        .child(
            // Before image
            img(before_src)
                .absolute()
                .inset_0()
                .w_full()
                .h_full()
                .object_fit(ObjectFit::Cover)
        )
        .child(
            // After image with clip
            div()
                .absolute()
                .top_0()
                .left_0()
                .w(relative(0.5))  // Show 50% initially
                .h_full()
                .overflow_hidden()
                .child(
                    img(after_src)
                        .w(px(800.))  // Full width of container
                        .h_full()
                        .object_fit(ObjectFit::Cover)
                )
        )
        .child(
            // Divider line
            div()
                .absolute()
                .top_0()
                .left(relative(0.5))
                .w(px(2.))
                .h_full()
                .bg(cx.theme().primary)
        )
}
```

### Error Handling Pattern

```rust
enum ImageState {
    Loading,
    Loaded,
    Error,
}

fn robust_image(src: &str, state: ImageState) -> impl IntoElement {
    div()
        .w(px(300.))
        .h(px(200.))
        .bg(cx.theme().muted)
        .rounded(px(8.))
        .border_1()
        .border_color(cx.theme().border)
        .flex()
        .items_center()
        .justify_center()
        .map(|this| {
            match state {
                ImageState::Loading => {
                    this.child(
                        v_flex()
                            .items_center()
                            .gap_2()
                            .child(Icon::new(IconName::Loader2).size(px(24.)))
                            .child("Loading...")
                    )
                }
                ImageState::Loaded => {
                    this.p_0()
                        .overflow_hidden()
                        .child(
                            img(src)
                                .w_full()
                                .h_full()
                                .object_fit(ObjectFit::Cover)
                        )
                }
                ImageState::Error => {
                    this.child(
                        v_flex()
                            .items_center()
                            .gap_2()
                            .child(
                                Icon::new(IconName::ImageOff)
                                    .size(px(32.))
                                    .text_color(cx.theme().muted_foreground)
                            )
                            .child("Failed to load image")
                    )
                }
            }
        })
}
```

## Best Practices

### Image Optimization

* Use appropriate image dimensions for display size
* Compress images without sacrificing quality
* Consider using modern image formats (WebP, AVIF)
* Implement responsive images for different screen sizes

### Error Handling

* Always provide meaningful fallbacks for failed image loads
* Use skeleton loading states to maintain layout stability
* Implement retry mechanisms for temporary network failures
* Provide user feedback for permanent load failures

### Performance

* Use lazy loading for images not immediately visible
* Implement proper caching strategies
* Consider using placeholder images during loading
* Optimize image sizes for their display context

### User Experience

* Maintain consistent aspect ratios in image grids
* Provide smooth loading transitions
* Use appropriate object-fit values for content type
* Consider providing zoom functionality for detailed images

## Implementation Notes

### GPUI Integration

* Built on GPUI's native image rendering capabilities
* Supports all GPUI ImageSource types automatically
* Inherits GPUI's styling and layout system
* Compatible with GPUI's animation and interaction systems

### SVG Support

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

---
url: /gpui-component/docs/components/input.md
description: 'Text input component with validation, masking, and various features.'
---

# Input

A flexible text input component with support for validation, masking, prefix/suffix elements, and different states.

## Import

```rust
use gpui_component::input::{InputState, Input};
```

## Usage

### Basic Input

```rust
let input = cx.new(|cx| InputState::new(window, cx));

Input::new(&input)
```

### With Placeholder

```rust
let input = cx.new(|cx|
    InputState::new(window, cx)
        .placeholder("Enter your name...")
);

Input::new(&input)
```

### With Default Value

```rust
let input = cx.new(|cx|
    InputState::new(window, cx)
        .default_value("John Doe")
);

Input::new(&input)
```

### Cleanable Input

```rust
Input::new(&input)
    .cleanable(true) // Show clear button when input has value
```

### With Prefix and Suffix

```rust
use gpui_component::{Icon, IconName};

// With prefix icon
Input::new(&input)
    .prefix(Icon::new(IconName::Search).small())

// With suffix button
Input::new(&input)
    .suffix(
        Button::new("info")
            .ghost()
            .icon(IconName::Info)
            .xsmall()
    )

// With both
Input::new(&input)
    .prefix(Icon::new(IconName::Search).small())
    .suffix(Button::new("btn").ghost().icon(IconName::Info).xsmall())
```

### Password Input (Masked)

```rust
let input = cx.new(|cx|
    InputState::new(window, cx)
        .masked(true)
        .default_value("password123")
);

Input::new(&input)
    .mask_toggle() // Shows toggle button to reveal password
```

### Input Sizes

```rust
Input::new(&input).large()
Input::new(&input) // medium (default)
Input::new(&input).small()
```

### Disabled Input

```rust
Input::new(&input).disabled(true)
```

### Clean on ESC

```rust
let input = cx.new(|cx|
    InputState::new(window, cx)
        .clean_on_escape() // Clear input when ESC is pressed
);

Input::new(&input)
```

### Input Validation

```rust
// Validate float numbers
let input = cx.new(|cx|
    InputState::new(window, cx)
        .validate(|s, _| s.parse::<f32>().is_ok())
);

// Regex pattern validation
let input = cx.new(|cx|
    InputState::new(window, cx)
        .pattern(regex::Regex::new(r"^[a-zA-Z0-9]*$").unwrap())
);
```

### Input Masking

```rust
// Phone number mask
let input = cx.new(|cx|
    InputState::new(window, cx)
        .mask_pattern("(999)-999-9999")
);

// Custom pattern: AAA-###-AAA (A=letter, #=digit, 9=digit optional)
let input = cx.new(|cx|
    InputState::new(window, cx)
        .mask_pattern("AAA-###-AAA")
);

// Number with thousands separator
use gpui_component::input::MaskPattern;

let input = cx.new(|cx|
    InputState::new(window, cx)
        .mask_pattern(MaskPattern::Number {
            separator: Some(','),
            fraction: Some(3),
        })
);
```

### Handle Input Events

```rust
let input = cx.new(|cx| InputState::new(window, cx));

cx.subscribe_in(&input, window, |view, state, event, window, cx| {
    match event {
        InputEvent::Change => {
            let text = state.read(cx).value();
            println!("Input changed: {}", text);
        }
        InputEvent::PressEnter { secondary } => {
            println!("Enter pressed, secondary: {}", secondary);
        }
        InputEvent::Focus => println!("Input focused"),
        InputEvent::Blur => println!("Input blurred"),
    }
});
```

### Custom Appearance

```rust
// Without default styling
Input::new(&input).appearance(false)

// Use in custom container
div()
    .border_b_2()
    .px_6()
    .py_3()
    .border_color(cx.theme().border)
    .bg(cx.theme().secondary)
    .child(Input::new(&input).appearance(false))
```

## Examples

### Search Input

```rust
let search = cx.new(|cx|
    InputState::new(window, cx)
        .placeholder("Search...")
);

Input::new(&search)
    .prefix(Icon::new(IconName::Search).small())
```

### Currency Input

```rust
let amount = cx.new(|cx|
    InputState::new(window, cx)
        .mask_pattern(MaskPattern::Number {
            separator: Some(','),
            fraction: Some(2),
        })
);

div()
    .child(Input::new(&amount))
    .child(format!("Value: {}", amount.read(cx).value()))
```

### Form with Multiple Inputs

```rust
struct FormView {
    name_input: Entity<InputState>,
    email_input: Entity<InputState>,
}

v_flex()
    .gap_3()
    .child(Input::new(&self.name_input))
    .child(Input::new(&self.email_input))
```

---

---
url: /gpui-component/docs/installation.md
---

# Installation

Before you start to build your application with `gpui-component`, you need to install the library.

## System Requirements

We can development application on macOS, Windows or Linux.

### macOS

* macOS 15 or later
* Xcode command line tools

## Windows

* Windows 10 or later

There have a bootstrap script to help install the required toolchain and dependencies.

You can run the script in PowerShell:

```ps
.\script\install-window.ps1
```

## Linux

Run `./script/bootstrap` to install system dependencies.

## Rust and Cargo

We use Rust programming language to build the `gpui-component` library. Make sure you have Rust and Cargo installed on your system.

* Rust 1.90 or later
* Cargo (comes with Rust)

To install the `gpui-component` library, you can use Cargo, the Rust package manager. Add the following line to your `Cargo.toml` file under the `[dependencies]` section:

```toml-vue
gpui = "{{ GPUI_VERSION }}"
gpui-component = "{{ VERSION }}"
```

---

---
url: /gpui-component/docs.md
description: >-
  Rust GUI components for building fantastic cross-platform desktop application
  by using GPUI.
---

# GPUI Component Introduction

GPUI Component is a Rust UI component library for building fantastic desktop applications using [GPUI](https://gpui.rs).

GPUI Component is a comprehensive UI component library for building fantastic desktop applications using [GPUI](https://gpui.rs). It provides 60+ cross-platform components with modern design, theming support, and high performance.

## Features

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

## Quick Example

Add `gpui` and `gpui-component` to your `Cargo.toml`:

```toml-vue
[dependencies]
gpui = "{{ VERSION }}"
gpui-component = "{{ VERSION }}"
```

Then create a simple "Hello, World!" application with a button:

```rust
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
    let app = Application::new();

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
```

## Community & Support

* [GitHub Repository](https://github.com/longbridge/gpui-component)
* [Issue Tracker](https://github.com/longbridge/gpui-component/issues)
* [Contributing Guide](https://github.com/longbridge/gpui-component/blob/main/CONTRIBUTING.md)

## License

Apache-2.0

---

---
url: /gpui-component/docs/components/kbd.md
description: Displays keyboard shortcuts with platform-specific formatting.
---

# Kbd

A component for displaying keyboard shortcuts and key combinations with proper platform-specific formatting. Automatically adapts the display to match the conventions of macOS (using symbols) or Windows/Linux (using text labels).

## Import

```rust
use gpui_component::kbd::Kbd;
use gpui::Keystroke;
```

## Usage

### Basic Keyboard Shortcut

```rust
// Create from a keystroke
let kbd = Kbd::new(Keystroke::parse("cmd-shift-p").unwrap());

// Or convert directly from keystroke
let kbd: Kbd = Keystroke::parse("escape").unwrap().into();
```

### Common Shortcuts

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

### Multiple Modifiers

```rust
// Complex combinations
Kbd::new(Keystroke::parse("cmd-ctrl-shift-a").unwrap())
Kbd::new(Keystroke::parse("cmd-alt-backspace").unwrap())
Kbd::new(Keystroke::parse("ctrl-alt-shift-a").unwrap())
```

### Arrow Keys and Function Keys

```rust
// Arrow keys
Kbd::new(Keystroke::parse("left").unwrap())
Kbd::new(Keystroke::parse("right").unwrap())
Kbd::new(Keystroke::parse("up").unwrap())
Kbd::new(Keystroke::parse("down").unwrap())

// Function keys
Kbd::new(Keystroke::parse("f12").unwrap())
Kbd::new(Keystroke::parse("secondary-f12").unwrap())

// Page navigation
Kbd::new(Keystroke::parse("pageup").unwrap())
Kbd::new(Keystroke::parse("pagedown").unwrap())
```

### Without Visual Styling

```rust
// Display only the key text without the styled background
Kbd::new(Keystroke::parse("cmd-s").unwrap())
    .appearance(false)
```

### From Action Bindings

```rust
use gpui::{Action, Window, FocusHandle};

// Get first keybinding for an action
if let Some(kbd) = Kbd::binding_for_action(&MyAction {}, None, window) {
    // Display the bound shortcut
}

// Get keybinding for action within a specific context
if let Some(kbd) = Kbd::binding_for_action(&MyAction {}, Some("Editor"), window) {
    // Display context-specific shortcut
}

// Get keybinding for action within a focus handle
if let Some(kbd) = Kbd::binding_for_action_in(&MyAction {}, &focus_handle, window) {
    // Display shortcut for focused element
}
```

## Platform Differences

The Kbd component automatically formats shortcuts according to platform conventions:

### macOS

* Uses symbols: ⌃ ⌥ ⇧ ⌘
* No separators between modifiers
* Order: Control, Option, Shift, Command
* Special keys: ⌫ (backspace), ⎋ (escape), ⏎ (enter), ← → ↑ ↓ (arrows)

### Windows/Linux

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

## Examples

### Keyboard Shortcut Help

```rust
use gpui::{div, h_flex, v_flex};

// Display common shortcuts
v_flex()
    .gap_2()
    .child(
        h_flex()
            .gap_2()
            .items_center()
            .child("Open command palette:")
            .child(Kbd::new(Keystroke::parse("cmd-shift-p").unwrap()))
    )
    .child(
        h_flex()
            .gap_2()
            .items_center()
            .child("Save file:")
            .child(Kbd::new(Keystroke::parse("cmd-s").unwrap()))
    )
    .child(
        h_flex()
            .gap_2()
            .items_center()
            .child("Find in files:")
            .child(Kbd::new(Keystroke::parse("cmd-shift-f").unwrap()))
    )
```

### Menu Item with Shortcut

```rust
h_flex()
    .justify_between()
    .items_center()
    .child("New File")
    .child(Kbd::new(Keystroke::parse("cmd-n").unwrap()))
```

### Inline Documentation

```rust
div()
    .child("Press ")
    .child(Kbd::new(Keystroke::parse("escape").unwrap()))
    .child(" to cancel or ")
    .child(Kbd::new(Keystroke::parse("enter").unwrap()))
    .child(" to confirm.")
```

### Custom Styling

```rust
Kbd::new(Keystroke::parse("cmd-k").unwrap())
    .text_color(cx.theme().accent)
    .border_color(cx.theme().accent)
    .bg(cx.theme().accent.opacity(0.1))
```

### Text-Only Format

```rust
// Get formatted text without styling
let shortcut_text = Kbd::format(&Keystroke::parse("cmd-shift-p").unwrap());
div().child(format!("Shortcut: {}", shortcut_text))
```

## Styling

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

---
url: /gpui-component/docs/components/label.md
description: Text labels for form elements with highlighting and styling options.
---

# Label

A versatile label component for displaying text with support for secondary text, highlighting, masking, and customizable styling. Perfect for form labels, captions, and general text display with optional/required indicators.

## Import

```rust
use gpui_component::label::{Label, HighlightsMatch};
```

## Usage

### Basic Label

```rust
Label::new("This is a label")
```

### Label with Secondary Text

```rust
// Label with optional indicator
Label::new("Company Address")
    .secondary("(optional)")

// Label with required indicator
Label::new("Email Address")
    .secondary("(required)")
```

### Text Alignment

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

### Text Highlighting

```rust
// Full text highlighting (finds all matches)
Label::new("Hello World Hello")
    .highlights("Hello")

// Prefix highlighting (only matches at start)
Label::new("Hello World")
    .highlights(HighlightsMatch::Prefix("Hello".into()))

// Highlight with secondary text
Label::new("Company Name")
    .secondary("(optional)")
    .highlights("Company")
```

### Color and Styling

```rust
use gpui_component::green_500;

// Custom text color
Label::new("Color Label")
    .text_color(green_500())

// Font styling
Label::new("Font Size Label")
    .text_size(px(20.))
    .font_semibold()
    .line_height(rems(1.8))
```

### Masked Labels

```rust
// For sensitive information
Label::new("9,182,1 USD")
    .text_2xl()
    .masked(true) // Shows as "•••••••••••"

// Toggle masking programmatically
Label::new("500 USD")
    .text_xl()
    .masked(self.masked)
```

### Multi-line Text

```rust
// Text wrapping with line height
div().w(px(200.)).child(
    Label::new(
        "Label should support text wrap in default, \
        if the text is too long, it should wrap to the next line."
    )
    .line_height(rems(1.8))
)
```

### Different Sizes

```rust
// Using text size utilities
Label::new("Extra Large").text_2xl()
Label::new("Large").text_xl()
Label::new("Medium").text_base() // default
Label::new("Small").text_sm()
Label::new("Extra Small").text_xs()
```

## API Reference

### Label

| Method              | Description                                                   |
| ------------------- | ------------------------------------------------------------- |
| `new(text)`         | Create a new label with text                                  |
| `secondary(text)`   | Add secondary text (usually for optional/required indicators) |
| `masked(bool)`      | Show/hide text with bullet characters                         |
| `highlights(match)` | Highlight matching text                                       |

### HighlightsMatch

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

## Examples

### Form Labels

```rust
// Required field
Label::new("Email Address")
    .secondary("*")
    .text_color(cx.theme().destructive)

// Optional field
Label::new("Phone Number")
    .secondary("(optional)")

// Field with description
Label::new("Password")
    .secondary("(minimum 8 characters)")
```

### Search Highlighting

```rust
// Interactive search highlighting
let search_term = "Hello";
Label::new("Hello World Hello Universe")
    .highlights(search_term) // Highlights all "Hello" occurrences
```

### Sensitive Information

```rust
// Financial data with toggle
h_flex()
    .child(
        Label::new("$9,182.50 USD")
            .text_2xl()
            .masked(self.is_masked)
    )
    .child(
        Button::new("toggle-mask")
            .ghost()
            .icon(if self.is_masked { IconName::EyeOff } else { IconName::Eye })
            .on_click(|this, _, _, _| {
                this.is_masked = !this.is_masked;
            })
    )
```

### Multi-language Support

```rust
// Supports Unicode text
Label::new("这是一个标签") // Chinese text
Label::new("こんにちは世界") // Japanese text
Label::new("🌍 Hello World 🚀") // Emojis
```

### Status Indicators

```rust
// Success status
Label::new("✓ Verified")
    .text_color(cx.theme().success)

// Warning status
Label::new("⚠ Pending Review")
    .text_color(cx.theme().warning)

// Error status
Label::new("✗ Failed")
    .text_color(cx.theme().destructive)
```

### Custom Layouts

```rust
// Flex layout with labels
h_flex()
    .justify_between()
    .child(Label::new("Total Amount"))
    .child(Label::new("$1,234.56").font_semibold())

// Grid layout
v_flex()
    .gap_2()
    .child(Label::new("Name:").font_semibold())
    .child(Label::new("John Doe"))
    .child(Label::new("Email:").font_semibold())
    .child(Label::new("john@example.com"))
```

---

---
url: /gpui-component/docs/components/list.md
description: >-
  A flexible list component that displays a series of items with support for
  sections, search, selection, and infinite scrolling.
---

# List

A powerful List component that provides a virtualized, searchable list interface with support for sections, headers, footers, selection states, and infinite scrolling. The component is built on a delegate pattern that allows for flexible data management and custom item rendering.

## Import

```rust
use gpui_component::list::{List, ListState, ListDelegate, ListItem, ListEvent, ListSeparatorItem};
use gpui_component::IndexPath;
```

## Usage

### Basic List

To create a list, you need to implement the `ListDelegate` trait for your data:

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

Now use \[List] to render list:

```rs
div().child(List::new(&state))
```

### List with Sections

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

### List Items with Icons and Actions

```rust
fn render_item(
    &mut self,
    ix: IndexPath,
    _window: &mut Window,
    cx: &mut Context<TableState<Self>>,
) -> Option<Self::Item> {
    self.items.get(ix.row).map(|item| {
        ListItem::new(ix)
            .child(
                h_flex()
                    .items_center()
                    .gap_2()
                    .child(Icon::new(IconName::File))
                    .child(Label::new(item.title.clone()))
            )
            .suffix(|_, _| {
                Button::new("action")
                    .ghost()
                    .small()
                    .icon(IconName::MoreHorizontal)
            })
            .selected(Some(ix) == self.selected_index)
            .on_click(cx.listener(move |this, _, window, cx| {
                this.delegate_mut().select_item(ix, window, cx);
            }))
    })
}
```

### List with Search

The list automatically includes a search input by default. Implement `perform_search` to handle queries:

And you should use `searchable(true)` when creating the `ListState` to show search input.

```rust
impl ListDelegate for MyListDelegate {
    fn perform_search(
        &mut self,
        query: &str,
        _window: &mut Window,
        _cx: &mut Context<ListState<Self>>,
    ) -> Task<()> {
        // Filter items based on query
        self.filtered_items = self.all_items
            .iter()
            .filter(|item| item.to_lowercase().contains(&query.to_lowercase()))
            .cloned()
            .collect();

        Task::ready(())
    }
}

let state = cx.new(|cx| ListState::new(delegate, window, cx).searchable(true));
List::new(&state)
```

### List with Loading State

```rust
impl ListDelegate for MyListDelegate {
    fn loading(&self, _cx: &App) -> bool {
        self.is_loading
    }

    fn render_loading(
        &mut self,
        _window: &mut Window,
        _cx: &mut Context<TableState<Self>>,
    ) -> impl IntoElement {
        // Custom loading view
        v_flex()
            .justify_center()
            .items_center()
            .py_4()
            .child(Skeleton::new().h_4().w_full())
            .child(Skeleton::new().h_4().w_3_4())
    }
}
```

### Infinite Scrolling

```rust
impl ListDelegate for MyListDelegate {
    fn has_more(&self, _cx: &App) -> bool {
        self.has_more_data
    }

    fn load_more_threshold(&self) -> usize {
        20 // Trigger when 20 items from bottom
    }

    fn load_more(&mut self, window: &mut Window, cx: &mut Context<ListState<Self>>) {
        if self.is_loading {
            return;
        }

        self.is_loading = true;
        cx.spawn_in(window, async move |view, window| {
            // Simulate API call
            Timer::after(Duration::from_secs(1)).await;

            view.update_in(window, |view, _, cx| {
                // Add more items
                view.delegate_mut().load_more_items();
                view.delegate_mut().is_loading = false;
                cx.notify();
            });
        }).detach();
    }
}
```

### List Events

```rust
// Subscribe to list events
let _subscription = cx.subscribe(&state, |_, _, event: &ListEvent, _| {
    match event {
        ListEvent::Select(ix) => {
            println!("Item selected at: {:?}", ix);
        }
        ListEvent::Confirm(ix) => {
            println!("Item confirmed at: {:?}", ix);
        }
        ListEvent::Cancel => {
            println!("Selection cancelled");
        }
    }
});
```

### Different Item Styles

```rust
// Basic item with hover effect
ListItem::new(ix)
    .child(Label::new("Basic Item"))
    .selected(is_selected)

// Item with check icon
ListItem::new(ix)
    .child(Label::new("Checkable Item"))
    .check_icon(IconName::Check)
    .confirmed(is_confirmed)

// Disabled item
ListItem::new(ix)
    .child(Label::new("Disabled Item"))
    .disabled(true)

// Separator item
ListSeparatorItem::new()
    .child(
        div()
            .h_px()
            .w_full()
            .bg(cx.theme().border)
    )
```

### Custom Empty State

```rust
impl ListDelegate for MyListDelegate {
    fn render_empty(&mut self, _window: &mut Window, cx: &mut Context<TableState<Self>>) -> impl IntoElement {
        v_flex()
            .size_full()
            .justify_center()
            .items_center()
            .gap_2()
            .child(Icon::new(IconName::Search).size_16().text_color(cx.theme().muted_foreground))
            .child(
                Label::new("No items found")
                    .text_color(cx.theme().muted_foreground)
            )
            .child(
                Label::new("Try adjusting your search terms")
                    .text_sm()
                    .text_color(cx.theme().muted_foreground.opacity(0.7))
            )
    }
}
```

## Configuration Options

### List Configuration

```rust
List::new(&state)
    .max_h(px(400.))                    // Set maximum height
    .scrollbar_visible(false)           // Hide scrollbar
    .paddings(Edges::all(px(8.)))       // Set internal padding
```

### Scrolling Control

```rust
// Scroll to specific item
state.update(cx, |state, cx| {
    state.scroll_to_item(
        IndexPath::new(0).section(1),  // Row 0 of section 1
        ScrollStrategy::Center,
        window,
        cx,
    );
});

// Scroll to selected item
state.update(cx, |state, cx| {
    state.scroll_to_selected_item(window, cx);
});

// Set selected index without scrolling
state.update(cx, |state, cx| {
    state.set_selected_index(Some(IndexPath::new(5)), window, cx);
});
```

## Examples

### File Browser List

```rust
struct FileBrowserDelegate {
    files: Vec<FileInfo>,
    selected: Option<IndexPath>,
}

#[derive(Clone)]
struct FileInfo {
    name: String,
    is_directory: bool,
    size: Option<u64>,
}

impl ListDelegate for FileBrowserDelegate {
    type Item = ListItem;

    fn render_item(&mut self, ix: IndexPath, window: &mut Window, cx: &mut Context<TableState<Self>>) -> Option<Self::Item> {
        self.files.get(ix.row).map(|file| {
            let icon = if file.is_directory {
                IconName::Folder
            } else {
                IconName::File
            };

            ListItem::new(ix)
                .child(
                    h_flex()
                        .items_center()
                        .justify_between()
                        .w_full()
                        .child(
                            h_flex()
                                .items_center()
                                .gap_2()
                                .child(Icon::new(icon))
                                .child(Label::new(file.name.clone()))
                        )
                        .when_some(file.size, |this, size| {
                            this.child(
                                Label::new(format_size(size))
                                    .text_sm()
                                    .text_color(cx.theme().muted_foreground)
                            )
                        })
                )
                .selected(Some(ix) == self.selected)
        })
    }
}
```

### Contact List with Sections

```rust
struct ContactListDelegate {
    contacts_by_letter: BTreeMap<char, Vec<Contact>>,
    selected: Option<IndexPath>,
}

impl ListDelegate for ContactListDelegate {
    type Item = ListItem;

    fn sections_count(&self, _cx: &App) -> usize {
        self.contacts_by_letter.len()
    }

    fn render_section_header(&mut self, section: usize, _window: &mut Window, cx: &mut Context<TableState<Self>>) -> Option<impl IntoElement> {
        let letter = self.contacts_by_letter.keys().nth(section)?;

        Some(
            div()
                .px_3()
                .py_2()
                .bg(cx.theme().background)
                .border_b_1()
                .border_color(cx.theme().border)
                .child(
                    Label::new(letter.to_string())
                        .text_lg()
                        .text_color(cx.theme().accent_foreground)
                        .font_weight(FontWeight::BOLD)
                )
        )
    }
}
```

---

---
url: /gpui-component/docs/components/menu.md
description: >-
  Context menus and popup menus with support for icons, shortcuts, submenus, and
  various menu item types.
---

# PopupMenu

The Menu component provides both context menus (right-click menus) and popup menus with comprehensive features including icons, keyboard shortcuts, submenus, separators, checkable items, and custom elements. Built with accessibility and keyboard navigation in mind.

## Import

```rust
use gpui_component::{
    menu::{PopupMenu, PopupMenuItem, ContextMenuExt, DropdownMenu},
    Button
};
use gpui::{actions, Action};
```

## Usage

### ContextMenu

Context menus appear when right-clicking on an element:

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

### DropdownMenu

Dropdown menus are triggered by buttons or other interactive elements:

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

:::tip
As you see, the each menu item is associated with an [Action],
we choice this design to better integrate with GPUI's action and key binding system,
allowing menu items to automatically display keyboard shortcuts when applicable.

So, the [Action] is the recommended way to define menu item behaviors.

However, if you prefer not to use [Action]s, you can create custom menu items using the `item` method along with [PopupMenuItem].
There have a `on_click` callback to handle the click event directly.
:::

### Anchor Position

Control where the dropdown menu appears relative to the trigger:

```rust
use gpui::Corner;

Button::new("menu-btn")
    .label("Options")
    .dropdown_menu_with_anchor(Corner::TopRight, |menu, window, cx| {
        menu.menu("Option 1", Box::new(Action1))
            .menu("Option 2", Box::new(Action2))
    })
```

### Icons

Add icons to menu items for better visual clarity:

```rust
use gpui_component::IconName;

menu.menu_with_icon("Search", IconName::Search, Box::new(Search))
    .menu_with_icon("Settings", IconName::Settings, Box::new(OpenSettings))
    .separator()
    .menu_with_icon("Help", IconName::Help, Box::new(ShowHelp))
```

### Disabled State

Create disabled menu items that cannot be activated:

```rust
menu.menu("Available Action", Box::new(Action1))
    .menu_with_disabled("Disabled Action", Box::new(Action2), true)
    .menu_with_icon_and_disabled(
        "Unavailable",
        IconName::Lock,
        Box::new(Action3),
        true
    )
```

### Check state

Create menu items that show a check state:

```rust
let is_enabled = true;

menu.menu_with_check("Enable Feature", is_enabled, Box::new(ToggleFeature))
    .menu_with_check("Show Sidebar", sidebar_visible, Box::new(ToggleSidebar))
```

By default, the check icon will be shown on the left side of the menu item, if this menu item has an icon, the check icon will replace the icon on the left side.

There also have a `check_side` option for you to config the check icon to be shown on the right side:

```rust
menu.check_size(Side::Right)
    .menu_with_check("Enable Feature", is_enabled, Box::new(ToggleFeature))
```

### Separators

Use separators to group related menu items:

```rust
menu.menu("New", Box::new(NewFile))
    .menu("Open", Box::new(OpenFile))
    .separator()  // Groups file operations
    .menu("Copy", Box::new(Copy))
    .menu("Paste", Box::new(Paste))
    .separator()  // Groups edit operations
    .menu("Exit", Box::new(Exit))
```

### Labels

Add non-interactive labels to organize menu sections:

```rust
menu.label("File Operations")
    .menu("New", Box::new(NewFile))
    .menu("Open", Box::new(OpenFile))
    .separator()
    .label("Edit Operations")
    .menu("Copy", Box::new(Copy))
    .menu("Paste", Box::new(Paste))
```

### Link MenuItem

Create menu items that open external links:

```rust
menu.link("Documentation", "https://docs.example.com")
    .link_with_icon(
        "GitHub",
        IconName::GitHub,
        "https://github.com/example/repo"
    )
    .separator()
    .external_link_icon(false) // Hide external link icons
    .link("Support", "https://support.example.com")
```

### Custom Element

Create custom menu items with complex content:

```rust
use gpui_component::{h_flex, v_flex};

menu.menu_element(Box::new(CustomAction), |window, cx| {
        v_flex()
            .child("Custom Element")
            .child(
                div()
                    .text_xs()
                    .text_color(cx.theme().muted_foreground)
                    .child("This is a subtitle")
            )
    })
    .menu_element_with_icon(
        IconName::Info,
        Box::new(InfoAction),
        |window, cx| {
            h_flex()
                .gap_1()
                .child("Status")
                .child(
                    div()
                        .text_sm()
                        .text_color(cx.theme().success)
                        .child("✓ Connected")
                )
        }
    )
```

### Keyboard Shortcuts

Menu items automatically display keyboard shortcuts if they're bound to actions:

```rust
// First define your actions and key bindings
actions!(my_app, [Copy, Paste, Cut]);

// In your app initialization
cx.bind_keys([
    KeyBinding::new("ctrl-c", Copy, Some("editor")),
    KeyBinding::new("ctrl-v", Paste, Some("editor")),
    KeyBinding::new("ctrl-x", Cut, Some("editor")),
]);

// The menu will automatically show shortcuts
menu.action_context(focus_handle) // Set context for shortcuts
    .menu("Copy", Box::new(Copy))     // Will show "Ctrl+C"
    .menu("Paste", Box::new(Paste))   // Will show "Ctrl+V"
    .menu("Cut", Box::new(Cut))       // Will show "Ctrl+X"
```

### Submenus

Create nested menus with submenu support:

```rust
menu.submenu("File", window, cx, |submenu, window, cx| {
        submenu.menu("New", Box::new(NewFile))
            .menu("Open", Box::new(OpenFile))
            .separator()
            .menu("Recent", Box::new(ShowRecent))
    })
    .submenu("Edit", window, cx, |submenu, window, cx| {
        submenu.menu("Undo", Box::new(Undo))
            .menu("Redo", Box::new(Redo))
    })
```

### Submenus with Icons

Add icons to submenu headers:

```rust
menu.submenu_with_icon(
        Some(IconName::Folder.into()),
        "Project",
        window,
        cx,
        |submenu, window, cx| {
            submenu.menu("Open Project", Box::new(OpenProject))
                .menu("Close Project", Box::new(CloseProject))
        }
    )
```

### Scrollable Menus

:::warning
When you have enabled `scrollable()` on a menu, avoid using submenus within it, as this can lead to usability issues.
:::

For menus with many items, enable scrolling:

```rust
Button::new("large-menu")
    .label("Many Options")
    .dropdown_menu(|menu, window, cx| {
        let mut menu = menu
            .scrollable(true)
            .max_h(px(300.))
            .label("Select an option");

        for i in 0..100 {
            menu = menu.menu(
                format!("Option {}", i),
                Box::new(SelectOption(i))
            );
        }
        menu
    })
```

### Menu Sizing

Control menu dimensions:

```rust
menu.min_w(px(200.))      // Minimum width
    .max_w(px(400.))      // Maximum width
    .max_h(px(300.))      // Maximum height
    .scrollable(true)         // Enable scrolling when content exceeds max height
```

### Action Context

Set the focus context for handling menu actions:

```rust
let focus_handle = cx.focus_handle();

menu.action_context(focus_handle)
    .menu("Copy", Box::new(Copy))
    .menu("Paste", Box::new(Paste))
```

## API Reference

* [PopupMenu]
* [context\_menu][context_menu]
* [PopupMenuItem]

## Examples

### File Manager Context Menu

```rust
div()
    .child("Right-click for options")
    .context_menu(|menu, window, cx| {
        menu.menu_with_icon("Open", IconName::FolderOpen, Box::new(Open))
            .separator()
            .menu_with_icon("Copy", IconName::Copy, Box::new(Copy))
            .menu_with_icon("Cut", IconName::Scissors, Box::new(Cut))
            .menu_with_icon("Paste", IconName::Clipboard, Box::new(Paste))
            .separator()
            .submenu("New", window, cx, |submenu, window, cx| {
                submenu.menu_with_icon("File", IconName::File, Box::new(NewFile))
                    .menu_with_icon("Folder", IconName::Folder, Box::new(NewFolder))
            })
            .separator()
            .menu_with_icon("Delete", IconName::Trash, Box::new(Delete))
            .separator()
            .menu("Properties", Box::new(ShowProperties))
    })
```

### Add MenuItem without action

Sometimes you may not like to define an action for a menu item, you just want add a `on_click` handler, in this case, the `item` and [PopupMenuItem] can help you:

```rust
use gpui_component::{menu::PopupMenuItem, Button};

Button::new("custom-item-menu")
    .label("Options")
    .dropdown_menu(|menu, window, cx| {
        menu.item(
            PopupMenuItem::new("Custom Action")
                .disabled(false)
                .icon(IconName::Star)
                .on_click(|window, cx| {
                    // Custom click handler logic
                    println!("Custom Action Clicked!");
                })
        )
        .separator()
        .menu("Standard Action", Box::new(StandardAction))
    })
```

### Editor Menu with Shortcuts

```rust
// Define actions with keyboard shortcuts
actions!(editor, [Save, SaveAs, Find, Replace, ToggleWordWrap]);

// Set up key bindings
cx.bind_keys([
    KeyBinding::new("ctrl-s", Save, Some("editor")),
    KeyBinding::new("ctrl-shift-s", SaveAs, Some("editor")),
    KeyBinding::new("ctrl-f", Find, Some("editor")),
    KeyBinding::new("ctrl-h", Replace, Some("editor")),
]);

// Create menu with automatic shortcuts
let editor_focus = cx.focus_handle();

Button::new("editor-menu")
    .label("Edit")
    .dropdown_menu(|menu, window, cx| {
        menu.action_context(editor_focus)
            .menu("Save", Box::new(Save))           // Shows "Ctrl+S"
            .menu("Save As...", Box::new(SaveAs))   // Shows "Ctrl+Shift+S"
            .separator()
            .menu("Find", Box::new(Find))           // Shows "Ctrl+F"
            .menu("Replace", Box::new(Replace))     // Shows "Ctrl+H"
            .separator()
            .menu_with_check("Word Wrap", true, Box::new(ToggleWordWrap))
    })
```

### Settings Menu with Custom Elements

```rust
Button::new("settings")
    .label("Settings")
    .dropdown_menu(|menu, window, cx| {
        menu.label("Display")
            .menu_element_with_check(dark_mode, Box::new(ToggleDarkMode), |window, cx| {
                h_flex()
                    .gap_2()
                    .child("Dark Mode")
                    .child(
                        div()
                            .text_xs()
                            .text_color(cx.theme().muted_foreground)
                            .child(if dark_mode { "On" } else { "Off" })
                    )
            })
            .separator()
            .label("Account")
            .menu_element_with_icon(
                IconName::User,
                Box::new(ShowProfile),
                |window, cx| {
                    v_flex()
                        .child("John Doe")
                        .child(
                            div()
                                .text_xs()
                                .text_color(cx.theme().muted_foreground)
                                .child("john@example.com")
                        )
                }
            )
            .separator()
            .link_with_icon("Help Center", IconName::Help, "https://help.example.com")
            .menu("Sign Out", Box::new(SignOut))
    })
```

## Keyboard Shortcuts

| Key               | Action                            |
| ----------------- | --------------------------------- |
| `↑` / `↓`         | Navigate menu items               |
| `←` / `→`         | Navigate submenus                 |
| `Enter` / `Space` | Activate menu item                |
| `Escape`          | Close menu                        |
| `Tab`             | Close menu and focus next element |

## Best Practices

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

---
url: /gpui-component/docs/components/notification.md
description: >-
  Display toast notifications that appear at the top right of the window with
  auto-dismiss functionality.
---

# Notification

A toast notification system for displaying temporary messages to users. Notifications appear at the top right of the window and can auto-dismiss after a timeout. Supports multiple variants (info, success, warning, error), custom content, titles, and action buttons. Perfect for status updates, confirmations, and user feedback.

## Import

```rust
use gpui_component::{
    notification::{Notification, NotificationType},
    WindowExt
};
```

## Usage

### Setup application root view for display of notifications

You need to set up your application's root view to render the notification layer. This is typically done in your main application struct's render method.

The [Root::render\_notification\_layer](https://docs.rs/gpui-component/latest/gpui_component/struct.Root.html#method.render_notification_layer) function handles rendering any active modals on top of your app content.

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

### Basic Notification

```rust
// Simple string notification
window.push_notification("This is a notification.", cx);

// Using Notification builder
Notification::new()
    .message("Your changes have been saved.")
```

### Notification Types

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

### Notification with Title

```rust
Notification::new()
    .title("Update Available")
    .message("A new version of the application is ready to install.")
    .with_type(NotificationType::Info)
```

### Auto-hide Control

```rust
// Disable auto-hide (manual dismiss only)
Notification::new()
    .message("This notification stays until manually closed.")
    .autohide(false)

// Default auto-hide after 5 seconds
Notification::new()
    .message("This will disappear automatically.")
    .autohide(true) // default
```

### With Action Button

```rust
Notification::new()
    .title("Connection Lost")
    .message("Unable to connect to server.")
    .with_type(NotificationType::Error)
    .autohide(false)
    .action(|_, cx| {
        Button::new("retry")
            .primary()
            .label("Retry")
            .on_click(cx.listener(|this, _, window, cx| {
                // Perform retry action
                println!("Retrying connection...");
                this.dismiss(window, cx);
            }))
    })
```

### Clickable Notifications

```rust
Notification::new()
    .message("Click to view details")
    .on_click(cx.listener(|_, _, _, cx| {
        println!("Notification clicked");
        // Handle notification click
        cx.notify();
    }))
```

### Custom Content

```rust
use gpui_component::text::markdown;

let markdown_content = r#"
## Custom Notification
- **Feature**: New dashboard available
- **Status**: Ready to use
- [Learn more](https://example.com)
"#;

Notification::new()
    .content(|_, window, cx| {
        markdown(markdown_content).into_any_element()
    })
```

### Unique Notifications

When you need to manage notifications manually, such as for long-running processes or persistent alerts, you can use unique IDs to push and remove notifications as needed.

In this case, you can create a special `struct` in local scope, and use `id` methods with this struct to identify the notification.

Then you can push the notification when needed, and later remove it using the same ID.

Like this:

```rust
// Using type-based ID for uniqueness
struct UpdateNotification;

Notification::new()
    .id::<UpdateNotification>()
    .message("System update available")
    .autohide(false)

// Using type + element ID for multiple unique notifications
struct TaskNotification;

Notification::warning("Task failed to complete")
    .id1::<TaskNotification>("task-123")
    .title("Task Failed")
```

Then remove the notification with `window.remove_notification::<UpdateNotification>`, like this:

```rust
// Later, dismiss the notification
window.remove_notification::<UpdateNotification>(cx);
```

## Examples

### Form Validation Error

```rust
Notification::error("Please correct the following errors before submitting.")
    .title("Validation Failed")
    .autohide(false)
    .action(|_, _, cx| {
        Button::new("review")
            .outline()
            .label("Review Form")
            .on_click(cx.listener(|this, _, window, cx| {
                // Navigate to form
                this.dismiss(window, cx);
            }))
    })
```

### File Upload Progress

```rust
struct UploadNotification;

// Start upload notification
window.push_notification(
    Notification::info("Uploading file...")
        .id::<UploadNotification>()
        .title("File Upload")
        .autohide(false),
    cx,
);

// Update to success when complete
window.push_notification(
    Notification::success("File uploaded successfully!")
        .id::<UploadNotification>()
        .title("Upload Complete"),
    cx,
);
```

### System Status Updates

```rust
// Warning about maintenance
Notification::warning("System maintenance will begin in 30 minutes.")
    .title("Scheduled Maintenance")
    .autohide(false)
    .action(|_, cx| {
        Button::new("details")
            .link()
            .label("View Details")
            .on_click(cx.listener(|this, _, window, cx| {
                // Show maintenance details
                this.dismiss(window, cx);
            }))
    })
```

### Batch Operation Results

```rust
use gpui_component::text::markdown;

let results_content = r#"
## Batch Operation Complete

**Processed**: 150 items
**Success**: 147 items
**Failed**: 3 items

[View failed items](/)
"#;

Notification::success("Batch operation completed with some failures.")
    .title("Operation Results")
    .content(|window, cx| {
        markdown(results_content).into_any_element()
    })
    .autohide(false)
```

### Interactive Confirmation

```rust
struct SaveConfirmation;

Notification::new()
    .id::<SaveConfirmation>()
    .title("Unsaved Changes")
    .message("You have unsaved changes. Save before leaving?")
    .autohide(false)
    .action(|_, cx| {
        Button::new("save")
            .primary()
            .label("Save")
            .on_click(cx.listener(|this, _, window, cx| {
                // Perform save
                println!("Saving changes...");
                this.dismiss(window, cx);
            }))
    })
    .on_click(cx.listener(|_, _, _, cx| {
        println!("Save reminder clicked");
        cx.notify();
    }))
```

---

---
url: /gpui-component/docs/components/number-input.md
description: >-
  Number input component with increment/decrement controls and numeric
  formatting.
---

# NumberInput

A specialized input component for numeric values with built-in increment/decrement buttons and support for min/max values, step values, and number formatting with thousands separators.

## Import

```rust
use gpui_component::input::{InputState, NumberInput, NumberInputEvent, StepAction};
```

## Usage

### Basic Number Input

```rust
let number_input = cx.new(|cx|
    InputState::new(window, cx)
        .placeholder("Enter number")
        .default_value("1")
);

NumberInput::new(&number_input)
```

### With Min/Max Validation

```rust
// Integer input with validation
let integer_input = cx.new(|cx|
    InputState::new(window, cx)
        .placeholder("Integer value")
        .pattern(Regex::new(r"^\d+$").unwrap()) // Only positive integers
);

NumberInput::new(&integer_input)
```

### With Number Formatting

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

### Different Sizes

```rust
// Large size
NumberInput::new(&input).large()

// Medium size (default)
NumberInput::new(&input)

// Small size
NumberInput::new(&input).small()
```

### With Prefix and Suffix

```rust
use gpui_component::{button::{Button, ButtonVariants}, IconName};

// With currency prefix
NumberInput::new(&input)
    .prefix(div().child("$"))

// With info button suffix
NumberInput::new(&input)
    .suffix(
        Button::new("info")
            .ghost()
            .icon(IconName::Info)
            .xsmall()
    )
```

### Disabled State

```rust
NumberInput::new(&input).disabled(true)
```

### Without Default Styling

```rust
// For custom container styling
div()
    .w_full()
    .bg(cx.theme().secondary)
    .rounded_md()
    .child(NumberInput::new(&input).appearance(false))
```

### Handle Number Input Events

```rust
let number_input = cx.new(|cx| InputState::new(window, cx));
let mut value: i64 = 0;

// Subscribe to input changes
cx.subscribe_in(&number_input, window, |view, state, event, window, cx| {
    match event {
        InputEvent::Change => {
            let text = state.read(cx).value();
            if let Ok(new_value) = text.parse::<i64>() {
                view.value = new_value;
            }
        }
        _ => {}
    }
});

// Subscribe to increment/decrement actions
cx.subscribe_in(&number_input, window, |view, state, event, window, cx| {
    match event {
        NumberInputEvent::Step(step_action) => {
            match step_action {
                StepAction::Increment => {
                    view.value += 1;
                    state.update(cx, |input, cx| {
                        input.set_value(view.value.to_string(), window, cx);
                    });
                }
                StepAction::Decrement => {
                    view.value -= 1;
                    state.update(cx, |input, cx| {
                        input.set_value(view.value.to_string(), window, cx);
                    });
                }
            }
        }
    }
});
```

### Programmatic Control

```rust
// Increment programmatically
NumberInput::increment(&number_input, window, cx);

// Decrement programmatically
NumberInput::decrement(&number_input, window, cx);
```

## API Reference

### NumberInput

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

### NumberInputEvent

| Event              | Description                        |
| ------------------ | ---------------------------------- |
| `Step(StepAction)` | Increment/decrement button pressed |

### StepAction

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

## Examples

### Integer Counter

```rust
struct CounterView {
    counter_input: Entity<InputState>,
    counter_value: i32,
}

impl CounterView {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let counter_input = cx.new(|cx|
            InputState::new(window, cx)
                .placeholder("Count")
                .default_value("0")
                .pattern(Regex::new(r"^-?\d+$").unwrap()) // Allow negative integers
        );

        let _subscription = cx.subscribe_in(&counter_input, window, Self::on_number_event);

        Self {
            counter_input,
            counter_value: 0,
        }
    }

    fn on_number_event(
        &mut self,
        state: &Entity<InputState>,
        event: &NumberInputEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match event {
            NumberInputEvent::Step(StepAction::Increment) => {
                self.counter_value += 1;
                state.update(cx, |input, cx| {
                    input.set_value(self.counter_value.to_string(), window, cx);
                });
            }
            NumberInputEvent::Step(StepAction::Decrement) => {
                self.counter_value -= 1;
                state.update(cx, |input, cx| {
                    input.set_value(self.counter_value.to_string(), window, cx);
                });
            }
        }
    }
}

// Usage
NumberInput::new(&self.counter_input)
```

### Currency Input

```rust
struct PriceInput {
    price_input: Entity<InputState>,
    price_value: f64,
}

impl PriceInput {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let price_input = cx.new(|cx|
            InputState::new(window, cx)
                .placeholder("0.00")
                .mask_pattern(MaskPattern::Number {
                    separator: Some(','),
                    fraction: Some(2),
                })
        );

        Self {
            price_input,
            price_value: 0.0,
        }
    }
}

// Usage with currency prefix
h_flex()
    .gap_2()
    .child(div().child("$"))
    .child(NumberInput::new(&self.price_input))
```

### Quantity Selector with Limits

```rust
struct QuantitySelector {
    quantity_input: Entity<InputState>,
    quantity: u32,
    min_quantity: u32,
    max_quantity: u32,
}

impl QuantitySelector {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let min_quantity = 1;
        let max_quantity = 99;

        let quantity_input = cx.new(|cx|
            InputState::new(window, cx)
                .default_value(min_quantity.to_string())
                .pattern(Regex::new(&format!(r"^[{}-{}]\d*$", min_quantity, max_quantity)).unwrap())
        );

        let _subscription = cx.subscribe_in(&quantity_input, window, Self::on_quantity_event);

        Self {
            quantity_input,
            quantity: min_quantity,
            min_quantity,
            max_quantity,
        }
    }

    fn on_quantity_event(
        &mut self,
        state: &Entity<InputState>,
        event: &NumberInputEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match event {
            NumberInputEvent::Step(StepAction::Increment) => {
                if self.quantity < self.max_quantity {
                    self.quantity += 1;
                    state.update(cx, |input, cx| {
                        input.set_value(self.quantity.to_string(), window, cx);
                    });
                }
            }
            NumberInputEvent::Step(StepAction::Decrement) => {
                if self.quantity > self.min_quantity {
                    self.quantity -= 1;
                    state.update(cx, |input, cx| {
                        input.set_value(self.quantity.to_string(), window, cx);
                    });
                }
            }
        }
    }
}

// Usage
NumberInput::new(&self.quantity_input).small()
```

### Floating Point Input

```rust
struct FloatInput {
    float_input: Entity<InputState>,
    float_value: f64,
    step: f64,
}

impl FloatInput {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let float_input = cx.new(|cx|
            InputState::new(window, cx)
                .placeholder("0.0")
                .pattern(Regex::new(r"^-?\d*\.?\d*$").unwrap()) // Allow decimal numbers
        );

        Self {
            float_input,
            float_value: 0.0,
            step: 0.1,
        }
    }

    fn on_float_event(
        &mut self,
        state: &Entity<InputState>,
        event: &NumberInputEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match event {
            NumberInputEvent::Step(StepAction::Increment) => {
                self.float_value += self.step;
                state.update(cx, |input, cx| {
                    input.set_value(format!("{:.1}", self.float_value), window, cx);
                });
            }
            NumberInputEvent::Step(StepAction::Decrement) => {
                self.float_value -= self.step;
                state.update(cx, |input, cx| {
                    input.set_value(format!("{:.1}", self.float_value), window, cx);
                });
            }
        }
    }
}
```

## Best Practices

1. **Validation**: Always validate numeric input on both client and server side
2. **Range Limits**: Implement min/max validation for user safety
3. **Step Size**: Choose appropriate step values for your use case
4. **Error Handling**: Provide clear feedback for invalid input
5. **Formatting**: Use consistent number formatting across your application
6. **Performance**: Debounce rapid increment/decrement actions if needed
7. **Accessibility**: Always provide proper labels and descriptions

---

---
url: /gpui-component/docs/components/otp-input.md
description: >-
  One-time password input component with multiple fields, auto-focus, and paste
  handling.
---

# OtpInput

A specialized input component for one-time passwords (OTP) that displays multiple input fields in a grid layout. Perfect for SMS verification codes, authenticator app codes, and other numeric verification scenarios.

## Import

```rust
use gpui_component::input::{OtpInput, OtpState};
```

## Usage

### Basic OTP Input

```rust
let otp_state = cx.new(|cx| OtpState::new(6, window, cx));

OtpInput::new(&otp_state)
```

### With Default Value

```rust
let otp_state = cx.new(|cx|
    OtpState::new(6, window, cx)
        .default_value("123456")
);

OtpInput::new(&otp_state)
```

### Masked OTP Input

```rust
let otp_state = cx.new(|cx|
    OtpState::new(6, window, cx)
        .masked(true)
        .default_value("123456")
);

OtpInput::new(&otp_state)
```

### Different Sizes

```rust
// Small size
OtpInput::new(&otp_state).small()

// Medium size (default)
OtpInput::new(&otp_state)

// Large size
OtpInput::new(&otp_state).large()

// Custom size
OtpInput::new(&otp_state).with_size(px(55.))
```

### Grouped Layout

```rust
// Single group (all fields together)
OtpInput::new(&otp_state).groups(1)

// Two groups (default) - splits fields in half
OtpInput::new(&otp_state).groups(2)

// Three groups - splits fields into thirds
OtpInput::new(&otp_state).groups(3)
```

### Disabled State

```rust
OtpInput::new(&otp_state).disabled(true)
```

### Different Length Codes

```rust
// 4-digit PIN
let pin_state = cx.new(|cx| OtpState::new(4, window, cx));
OtpInput::new(&pin_state).groups(1)

// 6-digit SMS code (most common)
let sms_state = cx.new(|cx| OtpState::new(6, window, cx));
OtpInput::new(&sms_state)

// 8-digit authenticator code
let auth_state = cx.new(|cx| OtpState::new(8, window, cx));
OtpInput::new(&auth_state).groups(2)
```

### Handle OTP Events

```rust
let otp_state = cx.new(|cx| OtpState::new(6, window, cx));

cx.subscribe(&otp_state, |this, state, event: &InputEvent, cx| {
    match event {
        InputEvent::Change => {
            let code = state.read(cx).value();
            if code.len() == 6 {
                println!("Complete OTP: {}", code);
                // Automatically submit when complete
                this.verify_otp(&code, cx);
            }
        }
        InputEvent::Focus => println!("OTP input focused"),
        InputEvent::Blur => println!("OTP input lost focus"),
        _ => {}
    }
});
```

### Programmatic Control

```rust
// Set value programmatically
otp_state.update(cx, |state, cx| {
    state.set_value("123456", window, cx);
});

// Toggle masking
otp_state.update(cx, |state, cx| {
    state.set_masked(true, window, cx);
});

// Focus the input
otp_state.update(cx, |state, cx| {
    state.focus(window, cx);
});

// Get current value
let current_value = otp_state.read(cx).value();
```

## API Reference

### OtpState

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

### OtpInput

| Method           | Description                              |
| ---------------- | ---------------------------------------- |
| `new(state)`     | Create OTP input with state entity       |
| `groups(n)`      | Set number of visual groups (default: 2) |
| `disabled(bool)` | Set disabled state                       |
| `small()`        | Small size (6x6 px fields)               |
| `large()`        | Large size (11x11 px fields)             |
| `with_size(px)`  | Custom field size                        |

### InputEvent

| Event    | Description                                       |
| -------- | ------------------------------------------------- |
| `Change` | Emitted when OTP is complete (all digits entered) |
| `Focus`  | Input received focus                              |
| `Blur`   | Input lost focus                                  |

## Examples

### SMS Verification

```rust
struct SmsVerification {
    otp_state: Entity<OtpState>,
    phone_number: String,
    is_verifying: bool,
}

impl SmsVerification {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let otp_state = cx.new(|cx| OtpState::new(6, window, cx));

        cx.subscribe(&otp_state, |this, state, event: &InputEvent, cx| {
            if let InputEvent::Change = event {
                let code = state.read(cx).value();
                this.verify_sms_code(&code, cx);
            }
        });

        Self {
            otp_state,
            phone_number: "+1234567890".to_string(),
            is_verifying: false,
        }
    }

    fn verify_sms_code(&mut self, code: &str, cx: &mut Context<Self>) {
        self.is_verifying = true;
        // API call to verify SMS code
        println!("Verifying SMS code: {}", code);
        cx.notify();
    }
}

impl Render for SmsVerification {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_4()
            .child(format!("Enter the 6-digit code sent to {}", self.phone_number))
            .child(OtpInput::new(&self.otp_state))
            .when(self.is_verifying, |this| {
                this.child("Verifying...")
            })
    }
}
```

### Two-Factor Authentication

```rust
struct TwoFactorAuth {
    otp_state: Entity<OtpState>,
    is_masked: bool,
}

impl TwoFactorAuth {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let otp_state = cx.new(|cx|
            OtpState::new(6, window, cx)
                .masked(true)
        );

        Self {
            otp_state,
            is_masked: true,
        }
    }

    fn toggle_visibility(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.is_masked = !self.is_masked;
        self.otp_state.update(cx, |state, cx| {
            state.set_masked(self.is_masked, window, cx);
        });
    }
}

impl Render for TwoFactorAuth {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_4()
            .child("Enter your authenticator code")
            .child(OtpInput::new(&self.otp_state))
            .child(
                Button::new("toggle-visibility")
                    .label(if self.is_masked { "Show" } else { "Hide" })
                    .on_click(cx.listener(Self::toggle_visibility))
            )
    }
}
```

### PIN Entry

```rust
struct PinEntry {
    pin_state: Entity<OtpState>,
    attempts: usize,
    max_attempts: usize,
}

impl PinEntry {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let pin_state = cx.new(|cx|
            OtpState::new(4, window, cx)
                .masked(true)
        );

        cx.subscribe(&pin_state, |this, state, event: &InputEvent, cx| {
            if let InputEvent::Change = event {
                let pin = state.read(cx).value();
                this.verify_pin(&pin, cx);
            }
        });

        Self {
            pin_state,
            attempts: 0,
            max_attempts: 3,
        }
    }

    fn verify_pin(&mut self, pin: &str, cx: &mut Context<Self>) {
        self.attempts += 1;

        // Simulate PIN verification
        if pin == "1234" {
            println!("PIN verified successfully!");
        } else {
            println!("Incorrect PIN. Attempts: {}/{}", self.attempts, self.max_attempts);

            // Clear PIN on incorrect attempt
            self.pin_state.update(cx, |state, cx| {
                state.set_value("", window, cx);
            });
        }

        cx.notify();
    }
}

impl Render for PinEntry {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let is_locked = self.attempts >= self.max_attempts;

        v_flex()
            .gap_4()
            .child("Enter your 4-digit PIN")
            .child(
                OtpInput::new(&self.pin_state)
                    .groups(1)
                    .disabled(is_locked)
            )
            .when(is_locked, |this| {
                this.child("Too many attempts. Please try again later.")
            })
            .when(self.attempts > 0 && !is_locked, |this| {
                this.child(format!(
                    "Incorrect PIN. {} attempts remaining.",
                    self.max_attempts - self.attempts
                ))
            })
    }
}
```

## Behavior

### Input Handling

* **Numeric Only**: Accepts only digits (0-9)
* **Auto-Focus**: Automatically moves to next field when digit is entered
* **Backspace**: Removes current digit and moves to previous field
* **Length Limit**: Prevents input beyond specified length
* **Auto-Complete**: Emits `Change` event when all fields are filled

### Visual Feedback

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

## Common Patterns

### Auto-Submit on Complete

```rust
cx.subscribe(&otp_state, |this, state, event: &InputEvent, cx| {
    if let InputEvent::Change = event {
        let code = state.read(cx).value();
        if code.len() == 6 {
            // Auto-submit when complete
            this.submit_verification_code(&code, cx);
        }
    }
});
```

### Clear on Focus

```rust
cx.subscribe(&otp_state, |this, state, event: &InputEvent, cx| {
    if let InputEvent::Focus = event {
        // Clear previous value when user starts entering new code
        state.update(cx, |state, cx| {
            state.set_value("", window, cx);
        });
    }
});
```

### Resend Code Timer

```rust
struct OtpWithResend {
    otp_state: Entity<OtpState>,
    resend_timer: Option<Timer>,
    can_resend: bool,
}

// Implementation would include timer logic for resend functionality
```

---

---
url: /gpui-component/docs/components/plot.md
description: >-
  A low-level plotting library for creating custom charts and data
  visualizations.
---

# Plot

The `plot` module provides low-level building blocks for creating custom charts. It includes scales, shapes, and utilities that power the high-level `Chart` components.

## Import

```rust
use gpui_component::plot::{
    scale::{Scale, ScaleLinear, ScaleBand, ScalePoint, ScaleOrdinal},
    shape::{Bar, Stack, Line, Area, Pie, Arc},
    PlotAxis, AxisText
};
```

## Scales

Scales map a dimension of abstract data to a visual representation.

### ScaleLinear

Maps a continuous quantitative domain to a continuous range.

```rust
let scale = ScaleLinear::new(
    vec![0., 100.],   // Domain (data values)
    vec![0., 500.]    // Range (pixel coordinates)
);

scale.tick(&50.); // Returns pixel position
```

### ScaleBand

Maps a discrete domain to a continuous range, useful for bar charts.

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

### ScalePoint

Maps a discrete domain to a set of points in a continuous range, useful for scatter plots or line charts with categorical axes.

```rust
let scale = ScalePoint::new(
    vec!["A", "B", "C"], // Domain
    vec![0., 300.]       // Range
);

scale.tick(&"A"); // Returns position of point "A"
```

### ScaleOrdinal

Maps a discrete domain to a discrete range.

```rust
let scale = ScaleOrdinal::new(
    vec!["A", "B", "C"], // Domain
    vec![color1, color2, color3] // Range
);

scale.map(&"A"); // Returns color1
```

## Shapes

### Bar

Renders a bar shape, commonly used in bar charts.

```rust
Bar::new()
    .data(data)
    .band_width(30.)
    .x(|d| x_scale.tick(&d.category))
    .y0(|d| y_scale.tick(&0.).unwrap())
    .y1(|d| y_scale.tick(&d.value))
    .fill(|d| color_scale.map(&d.category))
    .paint(&bounds, window, cx);
```

### Line

Renders a line shape, commonly used in line charts.

```rust
Line::new()
    .data(data)
    .x(|d| x_scale.tick(&d.date))
    .y(|d| y_scale.tick(&d.value))
    .stroke(cx.theme().chart_1)
    .stroke_width(px(2.))
    .paint(&bounds, window);
```

#### Line with Dots

Supports rendering dots at data points.

```rust
Line::new()
    .data(data)
    .x(|d| x_scale.tick(&d.date))
    .y(|d| y_scale.tick(&d.value))
    .dot()
    .dot_size(px(4.))
    .paint(&bounds, window);
```

### Area

Renders an area shape, commonly used in area charts.

```rust
Area::new()
    .data(data)
    .x(|d| x_scale.tick(&d.date))
    .y0(height) // Baseline
    .y1(|d| y_scale.tick(&d.value))
    .fill(cx.theme().chart_1.opacity(0.5))
    .stroke(cx.theme().chart_1)
    .paint(&bounds, window);
```

### Pie & Arc

Renders pie charts and donut charts using `Pie` layout and `Arc` shape.

```rust
// 1. Compute pie layout
let pie = Pie::new()
    .value(|d| Some(d.value))
    .pad_angle(0.05);

let arcs = pie.arcs(&data);

// 2. Render arcs
let arc_shape = Arc::new()
    .inner_radius(0.)
    .outer_radius(100.);

for arc_data in arcs {
    arc_shape.paint(
        &arc_data,
        color_scale.map(&arc_data.data.category), // Color
        None, // Override inner radius
        None, // Override outer radius
        &bounds,
        window
    );
}
```

### Stack

Computes stacked layout for data series.

```rust
let stack = Stack::new()
    .data(data)
    .keys(vec!["series1", "series2"])
    .value(|d, key| match key {
        "series1" => Some(d.val1),
        "series2" => Some(d.val2),
        _ => None
    });

let series = stack.series(); // Returns Vec<StackSeries<T>>
```

## Components

### PlotAxis

Renders chart axes with labels and ticks.

```rust
PlotAxis::new()
    .x(height) // Y position for X axis
    .x_label(labels) // Iterator of AxisText
    .stroke(cx.theme().border)
    .paint(&bounds, window, cx);
```

## Examples

### Custom Bar Chart Implementation

Here's how to implement a custom stacked bar chart using low-level plot primitives:

```rust
struct StackedBarChart {
    data: Vec<DailyDevice>,
    series: Vec<StackSeries<DailyDevice>>,
}

impl StackedBarChart {
    pub fn new(data: Vec<DailyDevice>) -> Self {
        let series = Stack::new()
            .data(data.clone())
            .keys(vec!["desktop", "mobile"])
            .value(|d, key| match key {
                "desktop" => Some(d.desktop),
                "mobile" => Some(d.mobile),
                _ => None,
            })
            .series();

        Self { data, series }
    }
}

impl Plot for StackedBarChart {
    fn paint(&mut self, bounds: Bounds<Pixels>, window: &mut Window, cx: &mut App) {
        // 1. Setup Scales
        let x = ScaleBand::new(
            self.data.iter().map(|v| v.date.clone()).collect(),
            vec![0., width],
        );
        
        let y = ScaleLinear::new(vec![0., max_value], vec![height, 0.]);

        // 2. Draw Axis
        // ... (axis rendering logic)

        // 3. Draw Stacked Bars
        let bar = Bar::new()
            .stack_data(&self.series)
            .band_width(x.band_width())
            .x(move |d| x.tick(&d.data.date))
            .fill(move |_| cx.theme().chart_1);

        bar.paint(&bounds, window, cx);
    }
}
```

---

---
url: /gpui-component/docs/components/popover.md
description: A floating overlay that displays rich content relative to a trigger element.
---

# Popover

Popover component for displaying floating content that appears when interacting with a trigger element. Supports multiple positioning options, custom content, different trigger methods, and automatic dismissal behaviors. Perfect for tooltips, menus, forms, and other contextual information.

## Import

```rust
use gpui_component::popover::{Popover};
```

## Usage

### Basic Popover

:::info
Any element that implements [Selectable] can be used as a trigger, for example, a [Button].

Any element that implements [RenderOnce] or [Render] can be used as popover content, use `.child(...)` to add children directly.
:::

```rust
use gpui::ParentElement as _;
use gpui_component::{button::Button, popover::Popover};

Popover::new("basic-popover")
    .trigger(Button::new("trigger").label("Click me").outline())
    .child("Hello, this is a popover!")
    .child("It appears when you click the button.")
```

### Popover with Custom Positioning

The `anchor` method allows you to specify where the popover appears relative to the trigger element, this anchor point can be one of the four corners: TopLeft, TopRight, BottomLeft, BottomRight.

```rust
use gpui::Corner;

Popover::new("positioned-popover")
    .anchor(Corner::TopRight)
    .trigger(Button::new("top-right").label("Top Right").outline())
    .child("This popover appears at the top right")
```

### View in Popover

You can add any `Entity<T>` that implemented [Render] as the popover content.

```rust
let view = cx.new(|_| MyView::new());

Popover::new("form-popover")
    .anchor(Corner::BottomLeft)
    .trigger(Button::new("show-form").label("Open Form").outline())
    .child(view.clone())
```

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

```rust
use gpui::ParentElement as _;
use gpui_component::popover::Popover;

Popover::new("complex-popover")
    .anchor(Corner::BottomLeft)
    .trigger(Button::new("complex").label("Complex Content").outline())
    .content(|_, _, _| {
        div()
            .child("This popover has complex content.")
            .child(
                Button::new("action-btn")
                    .label("Perform Action")
                    .outline()
            )
    })
```

### Right-Click Popover

Sometimes you may want to show a popover on right-click, for example, to create a special your ownen context menu. The `mouse_button` method allows you to specify which mouse button triggers the popover.

```rust
use gpui::MouseButton;

Popover::new("context-menu")
    .anchor(Corner::BottomRight)
    .mouse_button(MouseButton::Right)
    .trigger(Button::new("right-click").label("Right Click Me").outline())
    .child("Context Menu")
    .child(Divider::horizontal())
    .child("This is a custom context menu.")
```

### Dismiss Popover manually

If you want to dismiss the popover programmatically from within the content, you can emit a `DismissEvent`. In this case, you should use `content` method to create the popover content so you have access to the `cx: &mut Context<PopoverState>`.

```rust
use gpui_component::{DismissEvent, popover::Popover};

Popover::new("dismiss-popover")
    .trigger(Button::new("dismiss").label("Dismiss Popover").outline())
    .content(|_, cx| {
        div()
            .child("Click the button below to dismiss this popover.")
            .child(
                Button::new("close-btn")
                    .label("Close Popover")
                    .on_click(cx.listener(|_, _, _, cx| {
                        // NOTE: Here `cx` is `&mut Context<PopoverState>` type, so we can emit DismissEvent.
                        cx.emit(DismissEvent);
                    }))
            )
    })
```

### Styling Popover

Like the others components in GPUI Component, the `appearance(false)` method can be used to disable the default styling of the popover, allowing you to fully customize its appearance.

And the `Popover` has implemented the [Styled] trait, so you can use all the styling methods provided by GPUI to style the popover content as you like.

```rust
// For custom styled popovers or when you want full control
Popover::new("custom-popover")
    .appearance(false)
    .trigger(Button::new("custom").label("Custom Style"))
    .bg(cx.theme().accent)
    .text_color(cx.theme().accent_foreground)
    .p_6()
    .rounded_xl()
    .shadow_2xl()
    .child("Fully custom styled popover")
```

### Control Open State

There have `open` and `on_open_change` methods to control the open state of the popover programmatically.

This is useful when you want to synchronize the popover's open state with other UI elements or application state.

:::tip
When you use `open` to control the popover's open state, that means you have take full control of it,
so you need to update the state in `on_open_change` callback to keep the popover working correctly.
:::

```rust
use gpui_component::popover::Popover;

struct MyView {
    popover_open: bool,
}

Popover::new("controlled-popover")
    .open(self.open)
    .on_open_change(cx.listener(|this, open: &bool, _, cx| {
        this.popover_open = *open;
        cx.notify();
    }))
    .trigger(Button::new("control-btn").label("Control Popover").outline())
    .child("This popover's open state is controlled programmatically.")
```

### Default Open

The `default_open` method allows you to set the initial open state of the popover when it is first rendered.

Please note that if you use the `open` method to control the popover's open state, the `default_open` setting will be ignored.

```rust
use gpui_component::popover::Popover;

Popover::new("default-open-popover")
    .default_open(true)
    .trigger(Button::new("default-open-btn").label("Default Open").outline())
    .child("This popover is open by default when first rendered.")
```

[Button]: https://docs.rs/gpui-component/latest/gpui_component/button/struct.Button.html

[Selectable]: https://docs.rs/gpui-component/latest/gpui_component/trait.Selectable.html

[Render]: https://docs.rs/gpui/latest/gpui/trait.Render.html

[RenderOnce]: https://docs.rs/gpui/latest/gpui/trait.RenderOnce.html

[Styled]: https://docs.rs/gpui/latest/gpui/trait.Styled.html

---

---
url: /gpui-component/docs/components/progress.md
description: >-
  Displays an indicator showing the completion progress of a task, typically
  displayed as a progress bar.
---

# Progress

A linear progress bar component that visually represents the completion percentage of a task. The progress bar features smooth animations, customizable colors, and automatic styling that adapts to the current theme.

## Import

```rust
use gpui_component::progress::Progress;
```

## Usage

### Basic Progress Bar

```rust
Progress::new()
    .value(50.0) // 50% complete
```

### Different Progress Values

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

### Indeterminate State

```rust
// For unknown progress duration
Progress::new()
    .value(-1.0) // Any negative value shows as 0%

// Or explicitly set to 0 for starting state
Progress::new()
    .value(0.0)
```

### Dynamic Progress Updates

```rust
struct MyComponent {
    progress_value: f32,
}

impl MyComponent {
    fn update_progress(&mut self, new_value: f32) {
        self.progress_value = new_value.clamp(0.0, 100.0);
    }

    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_3()
            .child(
                h_flex()
                    .gap_2()
                    .child(Button::new("decrease").label("-").on_click(
                        cx.listener(|this, _, _, _| {
                            this.update_progress(this.progress_value - 10.0);
                        })
                    ))
                    .child(Button::new("increase").label("+").on_click(
                        cx.listener(|this, _, _, _| {
                            this.update_progress(this.progress_value + 10.0);
                        })
                    ))
            )
            .child(Progress::new().value(self.progress_value))
            .child(format!("{}%", self.progress_value as i32))
    }
}
```

### File Upload Progress

```rust
struct FileUpload {
    bytes_uploaded: u64,
    total_bytes: u64,
}

impl FileUpload {
    fn progress_percentage(&self) -> f32 {
        if self.total_bytes == 0 {
            0.0
        } else {
            (self.bytes_uploaded as f32 / self.total_bytes as f32) * 100.0
        }
    }

    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_2()
            .child("Uploading file...")
            .child(Progress::new().value(self.progress_percentage()))
            .child(format!(
                "{} / {} bytes",
                self.bytes_uploaded,
                self.total_bytes
            ))
    }
}
```

### Loading States

```rust
struct LoadingComponent {
    is_loading: bool,
    progress: f32,
}

impl LoadingComponent {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_3()
            .when(self.is_loading, |this| {
                this.child("Loading...")
                    .child(Progress::new().value(self.progress))
            })
            .when(!self.is_loading, |this| {
                this.child("Task completed!")
                    .child(Progress::new().value(100.0))
            })
    }
}
```

### Multi-Step Process

```rust
enum ProcessStep {
    Initializing,
    Processing,
    Finalizing,
    Complete,
}

struct MultiStepProcess {
    current_step: ProcessStep,
    step_progress: f32,
}

impl MultiStepProcess {
    fn overall_progress(&self) -> f32 {
        let base_progress = match self.current_step {
            ProcessStep::Initializing => 0.0,
            ProcessStep::Processing => 33.33,
            ProcessStep::Finalizing => 66.66,
            ProcessStep::Complete => 100.0,
        };

        if matches!(self.current_step, ProcessStep::Complete) {
            100.0
        } else {
            base_progress + (self.step_progress / 3.0)
        }
    }

    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_3()
            .child(match self.current_step {
                ProcessStep::Initializing => "Initializing...",
                ProcessStep::Processing => "Processing data...",
                ProcessStep::Finalizing => "Finalizing...",
                ProcessStep::Complete => "Complete!",
            })
            .child(Progress::new().value(self.overall_progress()))
            .child(format!("{:.1}% complete", self.overall_progress()))
    }
}
```

## Examples

### Task Progress with Status

```rust
struct TaskProgress {
    completed_tasks: usize,
    total_tasks: usize,
}

impl TaskProgress {
    fn progress_value(&self) -> f32 {
        if self.total_tasks == 0 {
            0.0
        } else {
            (self.completed_tasks as f32 / self.total_tasks as f32) * 100.0
        }
    }

    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_2()
            .child(
                h_flex()
                    .justify_between()
                    .child("Task Progress")
                    .child(format!("{}/{}", self.completed_tasks, self.total_tasks))
            )
            .child(Progress::new().value(self.progress_value()))
            .when(self.completed_tasks == self.total_tasks, |this| {
                this.child("All tasks completed!")
            })
    }
}
```

### Download Progress with Speed

```rust
struct DownloadProgress {
    downloaded: u64,
    total_size: u64,
    speed_mbps: f32,
}

impl DownloadProgress {
    fn eta_seconds(&self) -> u64 {
        if self.speed_mbps == 0.0 {
            0
        } else {
            let remaining_mb = (self.total_size - self.downloaded) as f32 / 1_000_000.0;
            (remaining_mb / self.speed_mbps) as u64
        }
    }

    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        let progress = (self.downloaded as f32 / self.total_size as f32) * 100.0;

        v_flex()
            .gap_2()
            .child(
                h_flex()
                    .justify_between()
                    .child("Downloading...")
                    .child(format!("{:.1}%", progress))
            )
            .child(Progress::new().value(progress))
            .child(
                h_flex()
                    .justify_between()
                    .child(format!("{:.1} MB/s", self.speed_mbps))
                    .child(format!("ETA: {}s", self.eta_seconds()))
            )
    }
}
```

### Installation Progress

```rust
struct InstallationProgress {
    current_package: String,
    package_index: usize,
    total_packages: usize,
    package_progress: f32,
}

impl InstallationProgress {
    fn overall_progress(&self) -> f32 {
        if self.total_packages == 0 {
            0.0
        } else {
            let completed_packages = self.package_index as f32;
            let current_package_contribution = self.package_progress / 100.0;
            let total_progress = (completed_packages + current_package_contribution)
                / self.total_packages as f32;
            total_progress * 100.0
        }
    }

    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_3()
            .child("Installing packages...")
            .child(
                v_flex()
                    .gap_2()
                    .child(
                        h_flex()
                            .justify_between()
                            .child(format!("Overall Progress"))
                            .child(format!("{}/{}", self.package_index + 1, self.total_packages))
                    )
                    .child(Progress::new().value(self.overall_progress()))
            )
            .child(
                v_flex()
                    .gap_2()
                    .child(format!("Installing: {}", self.current_package))
                    .child(Progress::new().value(self.package_progress))
            )
    }
}
```

## Styling and Theming

The Progress component automatically adapts to the current theme:

### Theme Colors

```rust
// The progress bar uses theme colors automatically
// Background: theme.progress_bar with 20% opacity
// Fill: theme.progress_bar at full opacity

// These colors adapt to light/dark theme automatically
Progress::new().value(75.0) // Uses theme colors
```

### Visual Properties

* **Height**: 8px by default
* **Border Radius**: Matches theme radius (up to half the height)
* **Background**: Semi-transparent theme progress bar color (20% opacity)
* **Fill**: Full opacity theme progress bar color
* **Animation**: Smooth transitions when value changes
* **Corners**: Rounded on completion, left-rounded during progress

## Behavior Notes

* Values less than 0 are clamped to 0%
* Values greater than 100 are clamped to 100%
* Progress bar fills from left to right
* Border radius adjusts based on completion state:
  * Partial progress: Left side rounded only
  * Complete progress: Both sides rounded
* Background color is always a semi-transparent version of the fill color
* Height and radius adapt to theme settings automatically

## Best Practices

1. **Always provide text indicators** alongside the visual progress bar
2. **Use meaningful labels** to describe what is progressing
3. **Update progress regularly** but not too frequently to avoid performance issues
4. **Consider showing ETA or completion time** for long-running tasks
5. **Provide cancel/pause options** for lengthy operations
6. **Show final status** when progress reaches 100%
7. **Handle error states** gracefully with appropriate messaging

---

---
url: /gpui-component/docs/components/radio.md
description: >-
  A set of checkable buttons—known as radio buttons—where no more than one of
  the buttons can be checked at a time.
---

# Radio

Radio buttons allow users to select a single option from a set of mutually exclusive choices. Use radio buttons when you want to give users a choice between multiple options and only one selection is allowed.

## Import

```rust
use gpui_component::radio::{Radio, RadioGroup};
```

## Usage

### Basic Radio Button

```rust
Radio::new("radio-option-1")
    .label("Option 1")
    .checked(false)
    .on_click(|checked, _, _| {
        println!("Radio is now: {}", checked);
    })
```

### Controlled Radio Button

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

### Radio Group (Recommended)

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

### Different Sizes

```rust
Radio::new("small").label("Small").xsmall()
Radio::new("medium").label("Medium") // default
Radio::new("large").label("Large").large()
```

### Disabled State

```rust
Radio::new("disabled")
    .label("Disabled option")
    .disabled(true)
    .checked(false)

Radio::new("disabled-checked")
    .label("Disabled and checked")
    .checked(true)
    .disabled(true)
```

### Multi-line Label with Custom Content

```rust
Radio::new("custom")
    .label("Primary option")
    .child(
        div()
            .text_color(cx.theme().muted_foreground)
            .child("This is additional descriptive text that provides more context.")
    )
    .w(px(300.))
```

### Custom Tab Order

```rust
Radio::new("radio")
    .label("Custom tab order")
    .tab_index(2)
    .tab_stop(true)
```

## Radio Group Usage

### Horizontal Layout

```rust
RadioGroup::horizontal("horizontal-group")
    .children(["First", "Second", "Third"])
    .selected_index(Some(0))
    .on_change(cx.listener(|view, index, _, cx| {
        println!("Selected index: {}", index);
        cx.notify();
    }))
```

### Vertical Layout

```rust
RadioGroup::vertical("vertical-group")
    .child(Radio::new("option1").label("United States"))
    .child(Radio::new("option2").label("Canada"))
    .child(Radio::new("option3").label("Mexico"))
    .selected_index(Some(1))
    .disabled(false)
```

### Styled Radio Group

```rust
RadioGroup::vertical("styled-group")
    .w(px(220.))
    .p_2()
    .border_1()
    .border_color(cx.theme().border)
    .rounded_md()
    .child(Radio::new("option1").label("Option 1"))
    .child(Radio::new("option2").label("Option 2"))
    .child(Radio::new("option3").label("Option 3"))
    .selected_index(Some(0))
```

### Disabled Radio Group

```rust
RadioGroup::vertical("disabled-group")
    .children(["Option A", "Option B", "Option C"])
    .selected_index(Some(1))
    .disabled(true) // Disables all radio buttons in the group
```

## API Reference

### Radio

| Method             | Description                                                 |
| ------------------ | ----------------------------------------------------------- |
| `new(id)`          | Create a new radio button with the given ID                 |
| `label(text)`      | Set label text                                              |
| `checked(bool)`    | Set checked state                                           |
| `disabled(bool)`   | Set disabled state                                          |
| `on_click(fn)`     | Callback when clicked, receives `&bool` (new checked state) |
| `tab_stop(bool)`   | Enable/disable tab navigation (default: true)               |
| `tab_index(isize)` | Set tab order index (default: 0)                            |

### RadioGroup

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

### Styling

Both Radio and RadioGroup implement `Styled` trait for custom styling:

Radio also implements `Sizable` trait:

* `xsmall()` - Extra small size
* `small()` - Small size
* `medium()` - Medium size (default)
* `large()` - Large size

## Examples

### Settings Panel

```rust
struct SettingsView {
    theme: Option<usize>, // 0: Light, 1: Dark, 2: Auto
    language: Option<usize>, // 0: English, 1: Spanish, 2: French
}

impl Render for SettingsView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_6()
            .child(
                v_flex()
                    .gap_2()
                    .child(div().text_sm().font_semibold().child("Theme"))
                    .child(
                        RadioGroup::vertical("theme")
                            .child(Radio::new("light").label("Light"))
                            .child(Radio::new("dark").label("Dark"))
                            .child(Radio::new("auto").label("Auto"))
                            .selected_index(self.theme)
                            .on_change(cx.listener(|view, index, _, cx| {
                                view.theme = Some(*index);
                                cx.notify();
                            }))
                    )
            )
            .child(
                v_flex()
                    .gap_2()
                    .child(div().text_sm().font_semibold().child("Language"))
                    .child(
                        RadioGroup::horizontal("language")
                            .children(["English", "Español", "Français"])
                            .selected_index(self.language)
                            .on_change(cx.listener(|view, index, _, cx| {
                                view.language = Some(*index);
                                cx.notify();
                            }))
                    )
            )
    }
}
```

### Survey Form

```rust
struct SurveyView {
    satisfaction: Option<usize>,
    recommendation: Option<usize>,
}

impl Render for SurveyView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_8()
            .child(
                v_flex()
                    .gap_3()
                    .child(
                        div()
                            .text_base()
                            .font_medium()
                            .child("How satisfied are you with our service?")
                    )
                    .child(
                        RadioGroup::vertical("satisfaction")
                            .child(Radio::new("very-satisfied").label("Very satisfied"))
                            .child(Radio::new("satisfied").label("Satisfied"))
                            .child(Radio::new("neutral").label("Neutral"))
                            .child(Radio::new("dissatisfied").label("Dissatisfied"))
                            .child(Radio::new("very-dissatisfied").label("Very dissatisfied"))
                            .selected_index(self.satisfaction)
                            .on_change(cx.listener(|view, index, _, cx| {
                                view.satisfaction = Some(*index);
                                cx.notify();
                            }))
                    )
            )
            .child(
                v_flex()
                    .gap_3()
                    .child(
                        div()
                            .text_base()
                            .font_medium()
                            .child("How likely are you to recommend us?")
                    )
                    .child(
                        RadioGroup::horizontal("recommendation")
                            .children((0..=10).map(|i| i.to_string()))
                            .selected_index(self.recommendation)
                            .on_change(cx.listener(|view, index, _, cx| {
                                view.recommendation = Some(*index);
                                cx.notify();
                            }))
                    )
            )
    }
}
```

### Payment Method Selection

```rust
struct PaymentView {
    payment_method: Option<usize>,
}

impl Render for PaymentView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_4()
            .child(
                div()
                    .text_lg()
                    .font_semibold()
                    .child("Select Payment Method")
            )
            .child(
                RadioGroup::vertical("payment")
                    .child(
                        Radio::new("credit-card")
                            .label("Credit Card")
                            .child(
                                div()
                                    .text_color(cx.theme().muted_foreground)
                                    .child("Visa, MasterCard, American Express")
                            )
                    )
                    .child(
                        Radio::new("paypal")
                            .label("PayPal")
                            .child(
                                div()
                                    .text_color(cx.theme().muted_foreground)
                                    .child("Pay with your PayPal account")
                            )
                    )
                    .child(
                        Radio::new("bank-transfer")
                            .label("Bank Transfer")
                            .child(
                                div()
                                    .text_color(cx.theme().muted_foreground)
                                    .child("Direct bank account transfer")
                            )
                    )
                    .selected_index(self.payment_method)
                    .on_change(cx.listener(|view, index, _, cx| {
                        view.payment_method = Some(*index);
                        cx.notify();
                    }))
            )
    }
}
```

## Best Practices

1. **Use RadioGroup**: Always prefer `RadioGroup` over individual `Radio` components for mutually exclusive choices
2. **Clear Labels**: Provide descriptive labels that clearly indicate what each option represents
3. **Default Selection**: Consider providing a sensible default selection, especially for required fields
4. **Logical Order**: Arrange options in a logical order (alphabetical, frequency of use, or importance)
5. **Limit Options**: Keep the number of radio options reasonable (typically 2-7 options)
6. **Group Related Options**: Use visual grouping and clear headings for multiple radio groups
7. **Responsive Design**: Consider using horizontal layout for fewer options and vertical for more options

---

---
url: /gpui-component/docs/components/resizable.md
description: >-
  A flexible panel layout system with draggable resize handles and adjustable
  panels.
---

# Resizable

The resizable component system provides a flexible way to create layouts with resizable panels. It supports both horizontal and vertical resizing, nested layouts, size constraints, and drag handles. Perfect for creating paned interfaces, split views, and adjustable dashboards.

## Import

```rust
use gpui_component::resizable::{
    h_resizable, v_resizable, resizable_panel,
    ResizablePanelGroup, ResizablePanel, ResizableState, ResizablePanelEvent
};
```

## Usage

Use `h_resizable` to create a horizontal layout, `v_resizable` to create a vertical layout.

The first argument is the `id` for this \[ResizablePanelGroup].

:::tip
In GPUI, the `id` must be unique within the layout scope (The nearest parent has presents `id`).
:::

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

The `v_resizable` component is used to create a vertical layout.

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

### Panel Size Constraints

```rust
resizable_panel()
    .size(px(200.))                    // Initial size
    .size_range(px(150.)..px(400.))    // Min and max size
    .child("Constrained Panel")
```

### Multiple Panels

```rust
h_resizable("multi-panel", state)
    .child(
        resizable_panel()
            .size(px(200.))
            .size_range(px(150.)..px(300.))
            .child("Left Panel")
    )
    .child(
        resizable_panel()
            .child("Center Panel")
    )
    .child(
        resizable_panel()
            .size(px(250.))
            .child("Right Panel")
    )
```

### Nested Layouts

```rust
v_resizable("main-layout", window, cx)
    .child(
        resizable_panel()
            .size(px(300.))
            .child(
                h_resizable("nested-layout", window, cx)
                    .child(
                        resizable_panel()
                            .size(px(200.))
                            .child("Top Left")
                    )
                    .child(
                        resizable_panel()
                            .child("Top Right")
                    )
            )
    )
    .child(
        resizable_panel()
            .child("Bottom Panel")
    )
```

### Nested Panel Groups

```rust
h_resizable("outer", window, cx)
    .child(
        resizable_panel()
            .size(px(200.))
            .child("Left Panel")
    )
    .group(
        v_resizable("inner", window, cx)
            .child(
                resizable_panel()
                    .size(px(150.))
                    .child("Top Right")
            )
            .child(
                resizable_panel()
                    .child("Bottom Right")
            )
    )
```

### Conditional Panel Visibility

```rust
resizable_panel()
    .visible(self.show_sidebar)
    .size(px(250.))
    .child("Sidebar Content")
```

### Panel with Size Limits

```rust
// Panel with minimum size only
resizable_panel()
    .size_range(px(100.)..Pixels::MAX)
    .child("Flexible Panel")

// Panel with both min and max
resizable_panel()
    .size_range(px(200.)..px(500.))
    .child("Constrained Panel")

// Panel with exact constraints
resizable_panel()
    .size(px(300.))
    .size_range(px(300.)..px(300.))  // Fixed size
    .child("Fixed Panel")
```

## Examples

### File Explorer Layout

```rust
struct FileExplorer {
    show_sidebar: bool,
}

impl Render for FileExplorer {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        h_resizable("file-explorer", window, cx)
            .child(
                resizable_panel()
                    .visible(self.show_sidebar)
                    .size(px(250.))
                    .size_range(px(200.)..px(400.))
                    .child(
                        v_flex()
                            .p_4()
                            .child("📁 Folders")
                            .child("• Documents")
                            .child("• Pictures")
                            .child("• Downloads")
                    )
            )
            .child(
                v_flex()
                    .p_4()
                    .child("📄 Files")
                    .child("file1.txt")
                    .child("file2.pdf")
                    .child("image.png")
                    .into_any_element()
            )
    }
}
```

### IDE Layout

```rust
struct IDELayout {
    main_state: Entity<ResizableState>,
    sidebar_state: Entity<ResizableState>,
    bottom_state: Entity<ResizableState>,
}

impl Render for IDELayout {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        h_resizable("ide-main", self.main_state.clone())
            .child(
                resizable_panel()
                    .size(px(300.))
                    .size_range(px(200.)..px(500.))
                    .child(
                        v_resizable("sidebar", self.sidebar_state.clone())
                            .child(
                                resizable_panel()
                                    .size(px(200.))
                                    .child("File Explorer")
                            )
                            .child(
                                resizable_panel()
                                    .child("Outline")
                            )
                    )
            )
            .child(
                resizable_panel()
                    .child(
                        v_resizable("editor-area", self.bottom_state.clone())
                            .child(
                                resizable_panel()
                                    .child("Code Editor")
                            )
                            .child(
                                resizable_panel()
                                    .size(px(150.))
                                    .size_range(px(100.)..px(300.))
                                    .child("Terminal / Output")
                            )
                    )
            )
    }
}
```

### Dashboard with Widgets

```rust
struct Dashboard {
    layout_state: Entity<ResizableState>,
    widget_state: Entity<ResizableState>,
}

impl Render for Dashboard {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        v_resizable("dashboard", self.layout_state.clone())
            .child(
                resizable_panel()
                    .size(px(120.))
                    .child("Header / Navigation")
            )
            .child(
                resizable_panel()
                    .child(
                        h_resizable("widgets", self.widget_state.clone())
                            .child(
                                resizable_panel()
                                    .size(px(300.))
                                    .child("Chart Widget")
                            )
                            .child(
                                resizable_panel()
                                    .child("Data Table")
                            )
                            .child(
                                resizable_panel()
                                    .size(px(250.))
                                    .child("Stats Panel")
                            )
                    )
            )
            .child(
                resizable_panel()
                    .size(px(60.))
                    .child("Footer")
            )
    }
}
```

### Settings Panel

```rust
struct SettingsPanel {
    settings_state: Entity<ResizableState>,
}

impl SettingsPanel {
    fn new(cx: &mut Context<Self>) -> Self {
        let settings_state = ResizableState::new(cx);

        // Listen for resize events to save layout preferences
        cx.subscribe(&settings_state, |this, _, event: &ResizablePanelEvent, cx| {
            match event {
                ResizablePanelEvent::Resized => {
                    this.save_layout_preferences(cx);
                }
            }
        });

        Self { settings_state }
    }

    fn save_layout_preferences(&self, cx: &mut Context<Self>) {
        let sizes = self.settings_state.read(cx).sizes();
        // Save to preferences
        println!("Saving layout: {:?}", sizes);
    }
}

impl Render for SettingsPanel {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        h_resizable("settings", self.settings_state.clone())
            .child(
                resizable_panel()
                    .size(px(200.))
                    .size_range(px(150.)..px(300.))
                    .child(
                        v_flex()
                            .gap_2()
                            .p_4()
                            .child("Categories")
                            .child("• General")
                            .child("• Appearance")
                            .child("• Advanced")
                    )
            )
            .child(
                resizable_panel()
                    .child(
                        div()
                            .p_6()
                            .child("Settings Content Area")
                    )
            )
    }
}
```

## Best Practices

1. **State Management**: Use separate ResizableState for independent layouts
2. **Size Constraints**: Always set reasonable min/max sizes for panels
3. **Event Handling**: Subscribe to ResizablePanelEvent for layout persistence
4. **Nested Layouts**: Use `.group()` method for clean nested structures
5. **Performance**: Avoid excessive nesting for better performance
6. **User Experience**: Provide adequate handle padding for easier interaction

---

---
url: /gpui-component/docs/root.md
---

# Root View

The [Root] component for as the root provider of GPUI Component features in a window. We must to use [Root] as the **first level child** of a window to enable GPUI Component features.

This is important, if we don't use [Root] as the first level child of a window, there will have some unexpected behaviors.

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

## Overlays

We have dialogs, sheets, notifications, we need placement for them to show, so [Root] provides methods to render these overlays:

* [Root::render\_dialog\_layer](https://docs.rs/gpui-component/latest/gpui_component/struct.Root.html#method.render_dialog_layer) - Render the current opened modals.
* [Root::render\_sheet\_layer](https://docs.rs/gpui-component/latest/gpui_component/struct.Root.html#method.render_sheet_layer) - Render the current opened drawers.
* [Root::render\_notification\_layer](https://docs.rs/gpui-component/latest/gpui_component/struct.Root.html#method.render_notification_layer) - Render the notification list.

We can put these layers in the `render` method your first level view (Root > YourFirstView):

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

:::tip
Here the example we used `children` method, it because if there is no opened dialogs, sheets, notifications, these methods will return `None`, so GPUI will not render anything.
:::

[Root]: https://docs.rs/gpui-component/latest/gpui_component/root/struct.Root.html

---

---
url: /gpui-component/docs/components/scrollable.md
description: >-
  Scrollable container with custom scrollbars, scroll tracking, and
  virtualization support.
---

# Scrollable

A comprehensive scrollable container component that provides custom scrollbars, scroll tracking, and virtualization capabilities. Supports both vertical and horizontal scrolling with customizable appearance and behavior.

## Import

```rust
use gpui_component::{
    scroll::{ScrollableElement, ScrollbarAxis, ScrollbarShow},
    StyledExt as _,
};
```

## Usage

### Basic Scrollable Container

The simplest way to make any element scrollable is using the `overflow_scrollbar()` method from `ScrollableElement` trait.

This method is almost like the `overflow_scroll()` method, but it adds scrollbars.

* `overflow_scrollbar()` - Adds scrollbars for both axes as needed.
* `overflow_x_scrollbar()` - Adds horizontal scrollbar as needed.
* `overflow_y_scrollbar()` - Adds vertical scrollbar as needed.

```rust
use gpui::{div, Axis};
use gpui_component::ScrollableElement;

div()
    .id("scrollable-container")
    .size_full()
    .child("Your content here")
    .overflow_scrollbar()
```

### Vertical Scrolling

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

### Horizontal Scrolling

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

### Both Directions

```rust
div()
    .id("scrollable-container")
    .size_full()
    .overflow_scrollbar()
    .child(
        div()
            .w(px(2000.))  // Wide content
            .h(px(2000.))  // Tall content
            .bg(cx.theme().background)
            .child("Large content area")
    )
```

## Custom Scrollbars

### Manual Scrollbar Creation

For more control, you can create scrollbars manually:

```rust
use gpui_component::scroll::{ScrollableElement};

pub struct ScrollableView {
    scroll_handle: ScrollHandle,
}

impl Render for ScrollableView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .relative()
            .size_full()
            .child(
                div()
                    .id("content")
                    .track_scroll(&self.scroll_handle)
                    .overflow_scroll()
                    .size_full()
                    .child("Your scrollable content")
            )
            .vertical_scrollbar(&self.scroll_handle)
    }
}
```

## Virtualization

### VirtualList for Large Datasets

For rendering large lists efficiently, use `VirtualList`:

```rust
use gpui_component::{VirtualList, VirtualListScrollHandle};

pub struct LargeListView {
    items: Vec<String>,
    scroll_handle: VirtualListScrollHandle,
}

impl Render for LargeListView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let item_count = self.items.len();

        VirtualList::new(
            self.scroll_handle.clone(),
            item_count,
            |ix, window, cx| {
                // Item sizes - can be different for each item
                size(px(300.), px(40.))
            },
            |ix, bounds, selected, window, cx| {
                // Render each item
                div()
                    .size(bounds.size)
                    .bg(if selected {
                        cx.theme().accent
                    } else {
                        cx.theme().background
                    })
                    .child(format!("Item {}: {}", ix, self.items[ix]))
                    .into_any_element()
            },
        )
    }
}
```

### Scrolling to Specific Items

```rust
impl LargeListView {
    fn scroll_to_item(&mut self, index: usize) {
        self.scroll_handle.scroll_to_item(index, ScrollStrategy::Top);
    }

    fn scroll_to_item_centered(&mut self, index: usize) {
        self.scroll_handle.scroll_to_item(index, ScrollStrategy::Center);
    }
}
```

### Variable Item Sizes

```rust
VirtualList::new(
    scroll_handle,
    items.len(),
    |ix, window, cx| {
        // Different heights based on content
        let height = if items[ix].len() > 50 {
            px(80.)  // Tall items for long content
        } else {
            px(40.)  // Normal height
        };
        size(px(300.), height)
    },
    |ix, bounds, selected, window, cx| {
        // Render logic
    },
)
```

## Theme Customization

### Scrollbar Appearance

Customize scrollbar appearance through theme configuration:

```rust
// In your theme JSON
{
    "scrollbar.background": "#ffffff20",
    "scrollbar.thumb.background": "#00000060",
    "scrollbar.thumb.hover.background": "#000000"
}
```

### Scrollbar Show Modes

Control when scrollbars are visible:

```rust
use gpui_component::scroll::ScrollbarShow;

// In theme initialization
theme.scrollbar_show = ScrollbarShow::Scrolling;  // Show only when scrolling
theme.scrollbar_show = ScrollbarShow::Hover;      // Show on hover
theme.scrollbar_show = ScrollbarShow::Always;     // Always visible
```

### System Integration

Sync scrollbar behavior with system preferences:

```rust
// Automatically sync with system settings
Theme::sync_scrollbar_appearance(cx);
```

## Examples

### File Browser with Scrolling

```rust
pub struct FileBrowser {
    files: Vec<String>,
}

impl Render for FileBrowser {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .border_1()
            .border_color(cx.theme().border)
            .size_full()
            .child(
                v_flex()
                    .gap_1()
                    .p_2()
                    .overflow_y_scrollbar()
                    .children(self.files.iter().map(|file| {
                        div()
                            .h(px(32.))
                            .w_full()
                            .px_2()
                            .flex()
                            .items_center()
                            .hover(|style| style.bg(cx.theme().secondary_hover))
                            .child(file.clone())
                    }))
            )
    }
}
```

### Chat Messages with Auto-scroll

```rust
pub struct ChatView {
    messages: Vec<String>,
    scroll_handle: ScrollHandle,
    should_auto_scroll: bool,
}

impl ChatView {
    fn add_message(&mut self, message: String) {
        self.messages.push(message);

        if self.should_auto_scroll {
            // Scroll to bottom for new messages
            let max_offset = self.scroll_handle.max_offset();
            self.scroll_handle.set_offset(point(px(0.), max_offset.y));
        }
    }
}
```

### Data Table with Virtual Scrolling

```rust
pub struct DataTable {
    data: Vec<Vec<String>>,
    scroll_handle: VirtualListScrollHandle,
}

impl Render for DataTable {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        VirtualList::new(
            self.scroll_handle.clone(),
            self.data.len(),
            |_ix, _window, _cx| size(px(800.), px(32.)), // Fixed row height
            |ix, bounds, _selected, _window, cx| {
                h_flex()
                    .size(bounds.size)
                    .border_b_1()
                    .border_color(cx.theme().border)
                    .children(self.data[ix].iter().map(|cell| {
                        div()
                            .flex_1()
                            .px_2()
                            .flex()
                            .items_center()
                            .child(cell.clone())
                    }))
                    .into_any_element()
            },
        )
    }
}
```

---

---
url: /gpui-component/docs/components/select.md
description: Displays a list of options for the user to pick from—triggered by a button.
---

# Select

:::info
This component was named `Dropdown` in `<= 0.3.x`.

It has been renamed to `Select` to better reflect its purpose.
:::

A select component that allows users to choose from a list of options.

Supports search functionality, grouped items, custom rendering, and various states. Built with keyboard navigation and accessibility in mind.

## Import

```rust
use gpui_component::select::{
    Select, SelectState, SelectItem, SelectDelegate,
    SelectEvent, SearchableVec, SelectGroup
};
```

## Usage

### Basic Select

You can create a basic select dropdown by initializing a `SelectState` with a list of items.

The first type parameter of `SelectState` is the items for the state, which must implement the [SelectItem] trait.

The built-in implementations of `SelectItem` include common types like `String`, `SharedString`, and `&'static str`.

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

### Placeholder

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

### Searchable

Use `searchable(true)` to enable search functionality within the dropdown.

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

### Impl SelectItem

By default, we have implmemented `SelectItem` for common types like `String`, `SharedString` and `&'static str`. You can also create your own item types by implementing the `SelectItem` trait.

This is useful when you want to display complex data structures, and also want get that data type from `select_value` method.

You can also customize the search logic by overriding the `matches` method.

```rust
#[derive(Debug, Clone)]
struct Country {
    name: SharedString,
    code: SharedString,
}

impl SelectItem for Country {
    type Value = SharedString;

    fn title(&self) -> SharedString {
        self.name.clone()
    }

    fn display_title(&self) -> Option<gpui::AnyElement> {
        // Custom display for selected item
        Some(format!("{} ({})", self.name, self.code).into_any_element())
    }

    fn value(&self) -> &Self::Value {
        &self.code
    }

    fn matches(&self, query: &str) -> bool {
        // Custom search logic
        self.name.to_lowercase().contains(&query.to_lowercase()) ||
        self.code.to_lowercase().contains(&query.to_lowercase())
    }
}
```

### Group Items

```rust
let mut grouped_items = SearchableVec::new(vec![]);

// Group countries by first letter
grouped_items.push(
    SelectGroup::new("A")
        .items(vec![
            Country { name: "Australia".into(), code: "AU".into() },
            Country { name: "Austria".into(), code: "AT".into() },
        ])
);
grouped_items.push(
    SelectGroup::new("B")
        .items(vec![
            Country { name: "Brazil".into(), code: "BR".into() },
            Country { name: "Belgium".into(), code: "BE".into() },
        ])
);

let state = cx.new(|cx| {
    SelectState::new(grouped_items, None, window, cx)
});

Select::new(&state)
```

### Sizes

```rust
Select::new(&state).large()
Select::new(&state) // medium (default)
Select::new(&state).small()
```

### Disabled State

```rust
Select::new(&state).disabled(true)
```

### Cleanable

```rust
Select::new(&state)
    .cleanable(true) // Show clear button when item is selected
```

### Custom Appearance

```rust
Select::new(&state)
    .w(px(320.))                    // Set dropdown width
    .menu_width(px(400.))           // Set menu popup width
    .appearance(false)              // Remove default styling
    .title_prefix("Country: ")      // Add prefix to selected title
```

### Empty State

```rust
let state = cx.new(|cx| {
    SelectState::new(Vec::<SharedString>::new(), None, window, cx)
});

Select::new(&state)
    .empty(
        h_flex()
            .h_24()
            .justify_center()
            .text_color(cx.theme().muted_foreground)
            .child("No options available")
    )
```

### Events

```rust
cx.subscribe_in(&state, window, |view, state, event, window, cx| {
    match event {
        SelectEvent::Confirm(value) => {
            if let Some(selected_value) = value {
                println!("Selected: {:?}", selected_value);
            } else {
                println!("Selection cleared");
            }
        }
    }
});
```

### Mutating

```rust
// Set by index
state.update(cx, |state, cx| {
    state.set_selected_index(Some(IndexPath::default().row(2)), window, cx);
});

// Set by value (requires PartialEq on Value type)
state.update(cx, |state, cx| {
    state.set_selected_value(&"US".into(), window, cx);
});

// Get current selection
let current_value = state.read(cx).selected_value();
```

Update items:

```rust
state.update(cx, |state, cx| {
    let new_items = vec!["New Option 1".into(), "New Option 2".into()];
    state.set_items(new_items, window, cx);
});
```

## Examples

### Language Selector

```rust
let languages = SearchableVec::new(vec![
    "Rust".into(),
    "TypeScript".into(),
    "Go".into(),
    "Python".into(),
    "JavaScript".into(),
]);

let state = cx.new(|cx| {
    SelectState::new(languages, None, window, cx)
});

Select::new(&state)
    .placeholder("Select language...")
    .title_prefix("Language: ")
```

### Country/Region Selector

```rust
#[derive(Debug, Clone)]
struct Region {
    name: SharedString,
    code: SharedString,
    flag: SharedString,
}

impl SelectItem for Region {
    type Value = SharedString;

    fn title(&self) -> SharedString {
        self.name.clone()
    }

    fn display_title(&self) -> Option<gpui::AnyElement> {
        Some(
            h_flex()
                .items_center()
                .gap_2()
                .child(self.flag.clone())
                .child(format!("{} ({})", self.name, self.code))
                .into_any_element()
        )
    }

    fn value(&self) -> &Self::Value {
        &self.code
    }
}

let regions = vec![
    Region {
        name: "United States".into(),
        code: "US".into(),
        flag: "🇺🇸".into()
    },
    Region {
        name: "Canada".into(),
        code: "CA".into(),
        flag: "🇨🇦".into()
    },
];

let state = cx.new(|cx| {
    SelectState::new(regions, None, window, cx)
});

Select::new(&state)
    .placeholder("Select country...")
```

### Integrated with Input Field

```rust
// Combined country code + phone input
h_flex()
    .border_1()
    .border_color(cx.theme().input)
    .rounded_lg()
    .w_full()
    .gap_1()
    .child(
        div().w(px(140.)).child(
            Select::new(&country_state)
                .appearance(false) // No border/background
                .py_2()
                .pl_3()
        )
    )
    .child(Divider::vertical())
    .child(
        div().flex_1().child(
            Input::new(&phone_input)
                .appearance(false)
                .placeholder("Phone number")
                .pr_3()
                .py_2()
        )
    )
```

### Multi-level Grouped Select

```rust
let mut grouped_countries = SearchableVec::new(vec![]);

for (continent, countries) in countries_by_continent {
    grouped_countries.push(
        SelectGroup::new(continent)
            .items(countries)
    );
}

let state = cx.new(|cx| {
    SelectState::new(grouped_countries, None, window, cx)
});

Select::new(&state)
    .menu_width(px(350.))
    .placeholder("Select country...")
```

## Keyboard Shortcuts

| Key       | Action                                  |
| --------- | --------------------------------------- |
| `Tab`     | Focus dropdown                          |
| `Enter`   | Open menu or select current item        |
| `Up/Down` | Navigate options (opens menu if closed) |
| `Escape`  | Close menu                              |
| `Space`   | Open menu                               |

## Theming

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

---
url: /gpui-component/docs/components/settings.md
description: A settings UI with grouped setting items and pages.
---

# Settings

> Since: v0.5.0

The Settings component provides a UI for managing application settings. It includes grouped setting items and pages.
We can search by title and description to filter the settings to display only relevant settings (Like this macOS, iOS Settings).

## Import

```rust
use gpui_component::setting::{Settings, SettingPage, SettingGroup, SettingItem, SettingField};
```

## Usage

### Build a settings

Here we have components that can be used to build a settings page.

* \[Settings] - The main settings component that holds multiple setting pages.
* \[SettingPage] - A page of related setting groups.
* \[SettingGroup] - A group of related setting items based on \[GroupBox] style.
* \[SettingItem] - A single setting item with title, description, and field.
* \[SettingField] - Provide different field types like Input, Dropdown, Switch, etc.

The layout of the settings is like this:

```
Settings
  SettingPage
    SettingGroup
      SettingItem
        Title
        Description (optional)
        SettingField
```

### Basic Settings

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

### With Multiple Pages

:::info
When you want default expland a page, you can use `default_open(true)` on the \[SettingPage].
:::

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

### Group Variants

```rust
use gpui_component::group_box::GroupBoxVariant;

Settings::new("my-settings")
    .with_group_variant(GroupBoxVariant::Outline)
    .pages(vec![...])

Settings::new("my-settings")
    .with_group_variant(GroupBoxVariant::Fill)
    .pages(vec![...])
```

## Setting Page

### Basic Page

```rust
SettingPage::new("General")
    .group(SettingGroup::new().title("Options").items(vec![...]))
```

### Multiple Groups

```rust
SettingPage::new("General")
    .groups(vec![
        SettingGroup::new().title("Appearance").items(vec![...]),
        SettingGroup::new().title("Font").items(vec![...]),
        SettingGroup::new().title("Other").items(vec![...]),
    ])
```

### Default Open

```rust
SettingPage::new("General")
    .default_open(true)
    .groups(vec![...])
```

### resettable

Enable reset functionality for a page:

```rust
SettingPage::new("General")
    .resettable(true)
    .groups(vec![...])
```

## Setting Group

### Basic Group

```rust
SettingGroup::new()
    .title("Appearance")
    .items(vec![
        SettingItem::new(...),
        SettingItem::new(...),
    ])
```

### Single Item

```rust
SettingGroup::new()
    .title("Font")
    .item(SettingItem::new(...))
```

### Without Title

```rust
SettingGroup::new()
    .items(vec![...])
```

## Setting Item

### Basic Item

```rust
SettingItem::new("Title", SettingField::switch(...))
    .description("Description text")
```

### Custom Item with a render closure

You can create a fully custom setting item using `SettingItem::render`:

```rust
SettingItem::render(|options, _, _| {
    h_flex()
        .w_full()
        .justify_between()
        .child("Custom content")
        .child(
            Button::new("action")
                .label("Action")
                .with_size(options.size)
        )
        .into_any_element()
})
```

### Vertical Layout

By default, setting items use horizontal layout. Use `layout(Axis::Vertical)` for vertical layout:

```rust
SettingItem::new(
    "CLI Path",
    SettingField::input(...)
)
.layout(Axis::Vertical)
.description("This item uses vertical layout.")
```

### With Markdown Description

```rust
use gpui_component::text::markdown;

SettingItem::new(
    "Documentation",
    SettingField::element(...)
)
.description(markdown("Rust doc for the `gpui-component` crate."))
```

## Setting Fields

The \[SettingField] enum provides different field types for various input needs.

### Switch

The switch field represents a `boolean` on/off state.

```rust
SettingItem::new(
    "Dark Mode",
    SettingField::switch(
        |cx: &App| cx.theme().mode.is_dark(),
        |val: bool, cx: &mut App| {
            // Handle value change
        },
    )
    .default_value(false)
)
```

### Checkbox

Like the switch, but uses a checkbox UI.

```rust
SettingItem::new(
    "Auto Switch Theme",
    SettingField::checkbox(
        |cx: &App| AppSettings::global(cx).auto_switch_theme,
        |val: bool, cx: &mut App| {
            AppSettings::global_mut(cx).auto_switch_theme = val;
        },
    )
    .default_value(false)
)
```

### Input

Display a single line text input.

```rust
SettingItem::new(
    "CLI Path",
    SettingField::input(
        |cx: &App| AppSettings::global(cx).cli_path.clone(),
        |val: SharedString, cx: &mut App| {
            AppSettings::global_mut(cx).cli_path = val;
        },
    )
    .default_value("/usr/local/bin/bash".into())
)
.layout(Axis::Vertical)
.description("Path to the CLI executable.")
```

### Dropdown

A dropdown with a list of options.

```rust
SettingItem::new(
    "Font Family",
    SettingField::dropdown(
        vec![
            ("Arial".into(), "Arial".into()),
            ("Helvetica".into(), "Helvetica".into()),
            ("Times New Roman".into(), "Times New Roman".into()),
        ],
        |cx: &App| AppSettings::global(cx).font_family.clone(),
        |val: SharedString, cx: &mut App| {
            AppSettings::global_mut(cx).font_family = val;
        },
    )
    .default_value("Arial".into())
)
```

### NumberInput

```rust
use gpui_component::setting::NumberFieldOptions;

SettingItem::new(
    "Font Size",
    SettingField::number_input(
        NumberFieldOptions {
            min: 8.0,
            max: 72.0,
            ..Default::default()
        },
        |cx: &App| AppSettings::global(cx).font_size,
        |val: f64, cx: &mut App| {
            AppSettings::global_mut(cx).font_size = val;
        },
    )
    .default_value(14.0)
)
```

### Custom Field by Render Closure

The `SettingField::render` method allows you to create a custom field using a closure that returns an element.

```rust
SettingItem::new(
    "GitHub Repository",
    SettingField::render(|options, _window, _cx| {
        Button::new("open-url")
            .outline()
            .label("Repository...")
            .with_size(options.size)
            .on_click(|_, _window, cx| {
                cx.open_url("https://github.com/example/repo");
            })
    })
)
```

### Custom Field Element

You may have a complex field that you want to reuse, you may want split the element into a separate struct to do the complex logic.

In this case, the \[SettingFieldElement] trait can help you to create a custom field element.

````rust
use gpui_component::setting::{SettingFieldElement, RenderOptions};

struct OpenURLSettingField {
    label: SharedString,
    url: SharedString,
}

impl SettingFieldElement for OpenURLSettingField {
    type Element = Button;

    fn render_field(&self, options: &RenderOptions, _: &mut Window, _: &mut App) -> Self::Element {
        let url = self.url.clone();
        Button::new("open-url")
            .outline()
            .label(self.label.clone())
            .with_size(options.size)
            .on_click(move |_, _window, cx| {
                cx.open_url(url.as_str());
            })
    }
}
```

Then use it in the setting item:

```rust
SettingItem::new(
    "GitHub Repository",
    SettingField::element(OpenURLSettingField {
        label: "Repository...".into(),
        url: "https://github.com/longbridge/gpui-component".into(),
    })
)
```

## API Reference

- [Settings]
- [SettingPage]
- [SettingGroup]
- [SettingItem]
- [SettingField]
- [NumberFieldOptions]

### Sizing

Implements [Sizable] trait:

- `xsmall()` - Extra small size
- `small()` - Small size
- `medium()` - Medium size (default)
- `large()` - Large size
- `with_size(Size)` - Set specific size

## Examples

### Complete Settings Example

```rust
use gpui::{App, SharedString};
use gpui_component::{
    Settings, SettingPage, SettingGroup, SettingItem, SettingField,
    setting::NumberFieldOptions,
    group_box::GroupBoxVariant,
    Size,
};

Settings::new("app-settings")
    .with_size(Size::Medium)
    .with_group_variant(GroupBoxVariant::Outline)
    .pages(vec![
        SettingPage::new("General")
            .resettable(true)
            .default_open(true)
            .groups(vec![
                SettingGroup::new()
                    .title("Appearance")
                    .items(vec![
                        SettingItem::new(
                            "Dark Mode",
                            SettingField::switch(
                                |cx: &App| cx.theme().mode.is_dark(),
                                |val: bool, cx: &mut App| {
                                    // Handle theme change
                                },
                            )
                        )
                        .description("Switch between light and dark themes."),
                    ]),
                SettingGroup::new()
                    .title("Font")
                    .items(vec![
                        SettingItem::new(
                            "Font Family",
                            SettingField::dropdown(
                                vec![
                                    ("Arial".into(), "Arial".into()),
                                    ("Helvetica".into(), "Helvetica".into()),
                                ],
                                |cx: &App| "Arial".into(),
                                |val: SharedString, cx: &mut App| {
                                    // Handle font change
                                },
                            )
                        ),
                        SettingItem::new(
                            "Font Size",
                            SettingField::number_input(
                                NumberFieldOptions {
                                    min: 8.0,
                                    max: 72.0,
                                    ..Default::default()
                                },
                                |cx: &App| 14.0,
                                |val: f64, cx: &mut App| {
                                    // Handle size change
                                },
                            )
                        ),
                    ]),
            ]),
        SettingPage::new("Software Update")
            .resettable(true)
            .group(
                SettingGroup::new()
                    .title("Updates")
                    .items(vec![
                        SettingItem::new(
                            "Auto Update",
                            SettingField::switch(
                                |cx: &App| true,
                                |val: bool, cx: &mut App| {
                                    // Handle auto update
                                },
                            )
                        )
                        .description("Automatically download and install updates."),
                    ])
            ),
    ])
```

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

---
url: /gpui-component/docs/components/sheet.md
description: >-
  A sliding panel that appears from the edges of the screen for displaying
  content.
---

# Sheet

A Sheet (also known as a sidebar or slide-out panel) is a navigation component that slides in from the edges of the screen. It provides additional space for content without taking up the main view, and can be used for navigation menus, forms, or any supplementary content.

## Import

```rust
use gpui_component::WindowExt;
use gpui_component::Placement;
```

## Usage

### Setup application root view for display of sheets

You need to set up your application's root view to render the sheet layer. This is typically done in your main application struct's render method.

The [Root::render\_sheet\_layer](https://docs.rs/gpui-component/latest/gpui_component/struct.Root.html#method.render_sheet_layer) function handles rendering any active modals on top of your app content.

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

### Basic Sheet

```rust
window.open_sheet(cx, |sheet, _, _| {
    sheet
        .title("Navigation")
        .child("Sheet content goes here")
})
```

### Sheet with Placement

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

### Sheet with Custom Size

```rust
window.open_sheet(cx, |sheet, _, _| {
    sheet
        .title("Wide Sheet")
        .size(px(500.))  // Custom width for left/right, height for top/bottom
        .child("This sheet is 500px wide")
})
```

### Sheet with Form Content

```rust
let input = cx.new(|cx| InputState::new(window, cx));
let date = cx.new(|cx| DatePickerState::new(window, cx));

window.open_sheet(cx, |sheet, _, _| {
    sheet
        .title("User Profile")
        .child(
            v_flex()
                .gap_4()
                .child("Enter your information:")
                .child(Input::new(&input).placeholder("Full Name"))
                .child(DatePicker::new(&date).placeholder("Date of Birth"))
        )
        .footer(
            h_flex()
                .gap_3()
                .child(Button::new("save").primary().label("Save"))
                .child(Button::new("cancel").label("Cancel"))
        )
})
```

### Overlay Options

```rust
window.open_sheet(cx, |sheet, _, _| {
    sheet
        .title("Settings")
        .overlay(true)              // Show overlay background (default: true)
        .overlay_closable(true)     // Click overlay to close (default: true)
        .child("Sheet settings content")
})

// No overlay
window.open_sheet(cx, |sheet, _, _| {
    sheet
        .title("Side Panel")
        .overlay(false)             // No overlay background
        .child("This sheet has no overlay")
})
```

### Resizable Sheet

```rust
window.open_sheet(cx, |sheet, _, _| {
    sheet
        .title("Resizable Panel")
        .resizable(true)            // Allow user to resize (default: true)
        .size(px(300.))
        .child("You can resize this sheet by dragging the edge")
})
```

### Custom Margin and Positioning

```rust
window.open_sheet(cx, |sheet, _, _| {
    sheet
        .title("Below Title Bar")
        .margin_top(px(32.))        // Space for window title bar
        .child("This sheet appears below the title bar")
})
```

### Sheet with List

```rust
let delegate = ListDelegate::new(items);
let list = cx.new(|cx| List::new(delegate, window, cx));

window.open_sheet_at(Placement::Left, cx, |sheet, _, _| {
    sheet
        .title("File Explorer")
        .size(px(400.))
        .child(
            div()
                .border_1()
                .border_color(cx.theme().border)
                .rounded(cx.theme().radius)
                .size_full()
                .child(list.clone())
        )
})
```

### Close Event Handling

```rust
window.open_sheet(cx, |sheet, _, _| {
    sheet
        .title("Sheet with Handler")
        .child("This sheet has a custom close handler")
        .on_close(|_, window, cx| {
            window.push_notification("Sheet was closed", cx);
        })
})
```

### Navigation Sheet

```rust
window.open_sheet_at(Placement::Left, cx, |sheet, _, _| {
    sheet
        .title("Navigation")
        .size(px(280.))
        .child(
            v_flex()
                .gap_2()
                .child(Button::new("home").ghost().label("Home").w_full())
                .child(Button::new("profile").ghost().label("Profile").w_full())
                .child(Button::new("settings").ghost().label("Settings").w_full())
                .child(Button::new("logout").ghost().label("Logout").w_full())
        )
})
```

### Custom Styling

```rust
window.open_sheet(cx, |sheet, _, cx| {
    sheet
        .title("Styled Sheet")
        .bg(cx.theme().accent)
        .text_color(cx.theme().accent_foreground)
        .border_color(cx.theme().primary)
        .child("Custom styled sheet content")
})
```

### Programmatic Close

```rust
// Close sheet from inside
Button::new("close")
    .label("Close Sheet")
    .on_click(|_, window, cx| {
        window.close_sheet(cx);
    })

// Close sheet from outside
window.close_sheet(cx);
```

## API Reference

### Window Extensions

| Method                             | Description                               |
| ---------------------------------- | ----------------------------------------- |
| `open_sheet(cx, fn)`               | Open sheet with default placement (Right) |
| `open_sheet_at(placement, cx, fn)` | Open sheet at specific placement          |
| `close_sheet(cx)`                  | Close current sheet                       |

### Sheet Builder

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

### Styling Methods

| Method                | Description              |
| --------------------- | ------------------------ |
| `bg(color)`           | Set background color     |
| `text_color(color)`   | Set text color           |
| `border_color(color)` | Set border color         |
| `px_*()/py_*()`       | Custom padding           |
| `gap_*()`             | Spacing between children |

## Examples

### Settings Panel

```rust
window.open_sheet_at(Placement::Right, cx, |sheet, _, _| {
    sheet
        .title("Settings")
        .size(px(350.))
        .child(
            v_flex()
                .gap_4()
                .child("Appearance")
                .child(Checkbox::new("dark-mode").label("Dark Mode"))
                .child(Checkbox::new("animations").label("Enable Animations"))
                .child("Notifications")
                .child(Checkbox::new("push-notifications").label("Push Notifications"))
        )
        .footer(
            h_flex()
                .justify_end()
                .gap_2()
                .child(Button::new("apply").primary().label("Apply"))
                .child(Button::new("cancel").label("Cancel"))
        )
})
```

### File Browser

```rust
window.open_sheet_at(Placement::Left, cx, |sheet, _, _| {
    sheet
        .title("Files")
        .size(px(300.))
        .child(
            v_flex()
                .size_full()
                .child(
                    h_flex()
                        .gap_2()
                        .p_2()
                        .child(Button::new("new-folder").small().icon(IconName::FolderPlus))
                        .child(Button::new("upload").small().icon(IconName::Upload))
                )
                .child(
                    div()
                        .flex_1()
                        .overflow_hidden()
                        .child(file_tree_list)
                )
        )
})
```

### Help Panel

```rust
window.open_sheet_at(Placement::Bottom, cx, |sheet, _, _| {
    sheet
        .title("Help & Documentation")
        .size(px(200.))
        .child(
            h_flex()
                .gap_4()
                .child("Keyboard Shortcuts")
                .child(Kbd::new("⌘").child("K"))
                .child("Search")
                .child(Kbd::new("⌘").child("P"))
                .child("Command Palette")
        )
})
```

## Best Practices

1. **Appropriate Placement**: Use left/right for navigation, top/bottom for temporary content
2. **Consistent Sizing**: Maintain consistent sheet sizes across your application
3. **Clear Headers**: Always provide descriptive titles
4. **Close Options**: Provide multiple ways to close (ESC, overlay click, close button)
5. **Content Organization**: Use proper spacing and grouping for sheet content
6. **Responsive Design**: Consider sheet behavior on smaller screens
7. **Performance**: Lazy load sheet content when possible for better performance

---

---
url: /gpui-component/docs/components/sidebar.md
description: >-
  A composable, themeable and customizable sidebar component for navigation and
  content organization.
---

# Sidebar

A flexible sidebar component that provides navigation structure for applications. Features collapsible states, nested menu items, header and footer sections, and responsive design. Perfect for creating application navigation panels, admin dashboards, and complex hierarchical interfaces.

## Import

```rust
use gpui_component::sidebar::{
    Sidebar, SidebarHeader, SidebarFooter, SidebarGroup,
    SidebarMenu, SidebarMenuItem, SidebarToggleButton
};
```

## Usage

### Basic Sidebar

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

### Collapsible Sidebar

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

### Nested Menu Items

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

### Multiple Groups

```rust
Sidebar::new(Side::Left)
    .child(
        SidebarGroup::new("Main")
            .child(
                SidebarMenu::new()
                    .child(SidebarMenuItem::new("Dashboard").icon(IconName::Home))
                    .child(SidebarMenuItem::new("Analytics").icon(IconName::BarChart))
            )
    )
    .child(
        SidebarGroup::new("Content")
            .child(
                SidebarMenu::new()
                    .child(SidebarMenuItem::new("Posts").icon(IconName::FileText))
                    .child(SidebarMenuItem::new("Media").icon(IconName::Image))
                    .child(SidebarMenuItem::new("Comments").icon(IconName::MessageCircle))
            )
    )
    .child(
        SidebarGroup::new("Settings")
            .child(
                SidebarMenu::new()
                    .child(SidebarMenuItem::new("General").icon(IconName::Settings))
                    .child(SidebarMenuItem::new("Users").icon(IconName::Users))
            )
    )
```

### With Badges and Suffixes

```rust
use gpui_component::{Badge, Switch};

SidebarMenuItem::new("Notifications")
    .icon(IconName::Bell)
    .suffix(
        Badge::new()
            .count(5)
            .child("5")
    )

SidebarMenuItem::new("Dark Mode")
    .icon(IconName::Moon)
    .suffix(
        Switch::new("dark-mode")
            .checked(true)
            .xsmall()
    )

SidebarMenuItem::new("Settings")
    .icon(IconName::Settings)
    .suffix(IconName::ChevronRight)
```

### Right-Side Placement

```rust
Sidebar::new(Side::Right)
    .width(300)
    .header(
        SidebarHeader::new()
            .child("Right Panel")
    )
    .child(
        SidebarGroup::new("Tools")
            .child(
                SidebarMenu::new()
                    .child(SidebarMenuItem::new("Inspector").icon(IconName::Search))
                    .child(SidebarMenuItem::new("Console").icon(IconName::Terminal))
            )
    )
```

### Custom Width and Styling

```rust
Sidebar::new(Side::Left)
    .width(280)  // Custom width in pixels
    .border_width(2)  // Custom border width
    .header(
        SidebarHeader::new()
            .p_4()  // Custom padding
            .rounded(cx.theme().radius)
            .child("Custom Styled Sidebar")
    )
```

### Interactive Header with Popup Menu

```rust
use gpui_component::menu::DropdownMenu;

SidebarHeader::new()
    .child(
        h_flex()
            .gap_2()
            .child(Icon::new(IconName::Building))
            .child("Company Name")
            .child(Icon::new(IconName::ChevronsUpDown))
    )
    .dropdown_menu(|menu, _, _| {
        menu.menu("Acme Corp", Box::new(SelectCompany("acme")))
            .menu("Tech Solutions", Box::new(SelectCompany("tech")))
            .separator()
            .menu("Switch Organization", Box::new(SwitchOrg))
    })
```

### Footer with User Information

```rust
SidebarFooter::new()
    .justify_between()
    .child(
        h_flex()
            .gap_2()
            .child(Icon::new(IconName::User))
            .when(!collapsed, |this| {
                this.child(
                    v_flex()
                        .child("John Doe")
                        .child(div().text_xs().text_color(cx.theme().muted_foreground).child("john@example.com"))
                )
            })
    )
    .when(!collapsed, |this| {
        this.child(Icon::new(IconName::MoreHorizontal))
    })
```

### Responsive Sidebar

```rust
let is_mobile = window_width < 768;

Sidebar::new(Side::Left)
    .collapsed(is_mobile || manually_collapsed)
    .width(if is_mobile { 60 } else { 240 })
    .header(
        SidebarHeader::new()
            .child(
                div()
                    .when(!is_mobile, |this| this.child("Full App Name"))
                    .when(is_mobile, |this| this.child(Icon::new(IconName::Menu)))
            )
    )
```

## Theming

The sidebar uses dedicated theme colors:

```rust
// Theme colors used by sidebar
cx.theme().sidebar                    // Background
cx.theme().sidebar_foreground         // Text color
cx.theme().sidebar_border            // Border color
cx.theme().sidebar_accent            // Hover/active background
cx.theme().sidebar_accent_foreground // Hover/active text
cx.theme().sidebar_primary           // Primary elements
cx.theme().sidebar_primary_foreground // Primary text
```

## Examples

### File Explorer Sidebar

```rust
Sidebar::new(Side::Left)
    .header(
        SidebarHeader::new()
            .child(
                h_flex()
                    .gap_2()
                    .child(IconName::Folder)
                    .child("Explorer")
            )
    )
    .child(
        SidebarGroup::new("Folders")
            .child(
                SidebarMenu::new()
                    .child(
                        SidebarMenuItem::new("src")
                            .icon(IconName::FolderOpen)
                            .active(true)
                            .children([
                                SidebarMenuItem::new("components")
                                    .icon(IconName::Folder),
                                SidebarMenuItem::new("utils")
                                    .icon(IconName::Folder),
                                SidebarMenuItem::new("main.rs")
                                    .icon(IconName::FileCode)
                                    .active(true),
                            ])
                    )
                    .child(
                        SidebarMenuItem::new("tests")
                            .icon(IconName::Folder)
                    )
                    .child(
                        SidebarMenuItem::new("Cargo.toml")
                            .icon(IconName::FileText)
                    )
            )
    )
```

### Admin Dashboard Sidebar

```rust
Sidebar::new(Side::Left)
    .header(
        SidebarHeader::new()
            .child(
                h_flex()
                    .gap_2()
                    .child(
                        div()
                            .size_8()
                            .rounded_full()
                            .bg(cx.theme().primary)
                            .child(Icon::new(IconName::Crown))
                    )
                    .child("Admin Panel")
            )
    )
    .child(
        SidebarGroup::new("Overview")
            .child(
                SidebarMenu::new()
                    .child(
                        SidebarMenuItem::new("Dashboard")
                            .icon(IconName::LayoutDashboard)
                            .active(true)
                    )
                    .child(
                        SidebarMenuItem::new("Analytics")
                            .icon(IconName::TrendingUp)
                            .suffix(Badge::new().count(2))
                    )
            )
    )
    .child(
        SidebarGroup::new("Management")
            .child(
                SidebarMenu::new()
                    .child(
                        SidebarMenuItem::new("Users")
                            .icon(IconName::Users)
                            .suffix("1,234")
                    )
                    .child(
                        SidebarMenuItem::new("Orders")
                            .icon(IconName::ShoppingCart)
                            .suffix(Badge::new().dot().variant_destructive())
                    )
                    .child(
                        SidebarMenuItem::new("Products")
                            .icon(IconName::Package)
                    )
            )
    )
    .footer(
        SidebarFooter::new()
            .child(
                h_flex()
                    .gap_2()
                    .child(IconName::User)
                    .child("Administrator")
            )
            .child(IconName::LogOut)
    )
```

### Settings Sidebar

```rust
Sidebar::new(Side::Left)
    .width(300)
    .header(
        SidebarHeader::new()
            .child("Settings")
    )
    .child(
        SidebarGroup::new("General")
            .child(
                SidebarMenu::new()
                    .child(
                        SidebarMenuItem::new("Appearance")
                            .icon(IconName::Palette)
                            .active(true)
                    )
                    .child(
                        SidebarMenuItem::new("Notifications")
                            .icon(IconName::Bell)
                            .suffix(
                                Switch::new("notifications")
                                    .checked(true)
                                    .xsmall()
                            )
                    )
                    .child(
                        SidebarMenuItem::new("Privacy")
                            .icon(IconName::Shield)
                    )
            )
    )
    .child(
        SidebarGroup::new("Advanced")
            .child(
                SidebarMenu::new()
                    .child(
                        SidebarMenuItem::new("Developer")
                            .icon(IconName::Code)
                            .children([
                                SidebarMenuItem::new("Debug Mode")
                                    .suffix(
                                        Switch::new("debug")
                                            .checked(false)
                                            .xsmall()
                                    ),
                                SidebarMenuItem::new("Console")
                                    .on_click(|_, _, _| println!("Open console")),
                            ])
                    )
                    .child(
                        SidebarMenuItem::new("Performance")
                            .icon(IconName::Zap)
                    )
            )
    )
```

---

---
url: /gpui-component/docs/components/skeleton.md
description: Use to show a placeholder while content is loading.
---

# Skeleton

The Skeleton component displays animated placeholder content while actual content is loading. It provides visual feedback to users that content is being loaded and helps maintain layout structure during loading states.

## Import

```rust
use gpui_component::skeleton::Skeleton;
```

## Usage

### Basic Skeleton

```rust
Skeleton::new()
```

### Text Line Skeleton

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

### Circle Skeleton

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

### Rectangle Skeleton

```rust
// Card image placeholder
Skeleton::new()
    .w(px(250.))
    .h(px(125.))
    .rounded_md()

// Button placeholder
Skeleton::new()
    .w(px(120.))
    .h(px(40.))
    .rounded_md()
```

### Different Shapes

```rust
// Text content
Skeleton::new().w(px(200.)).h_4().rounded_sm()

// Square image
Skeleton::new().size_20().rounded_md()

// Wide banner
Skeleton::new().w_full().h(px(200.)).rounded_lg()

// Small icon
Skeleton::new().size_6().rounded_md()
```

### Secondary Variant

```rust
// Use secondary color (more subtle)
Skeleton::new()
    .secondary()
    .w(px(200.))
    .h_4()
    .rounded_md()
```

## Animation

The Skeleton component includes a built-in pulse animation that:

* Runs continuously with a 2-second duration
* Uses a bounce easing function with ease-in-out
* Animates opacity from 100% to 50% and back
* Automatically repeats to indicate loading state

The animation cannot be disabled as it's essential for indicating loading state.

## Sizes

The Skeleton component doesn't have predefined size variants. Instead, use gpui's sizing utilities:

```rust
// Height utilities
Skeleton::new().h_3()    // 12px height
Skeleton::new().h_4()    // 16px height
Skeleton::new().h_5()    // 20px height
Skeleton::new().h_6()    // 24px height

// Width utilities
Skeleton::new().w(px(100.))   // 100px width
Skeleton::new().w(px(200.))   // 200px width
Skeleton::new().w_full()      // Full width
Skeleton::new().w_1_2()       // 50% width

// Square sizes
Skeleton::new().size_4()      // 16x16px
Skeleton::new().size_8()      // 32x32px
Skeleton::new().size_12()     // 48x48px
Skeleton::new().size_16()     // 64x64px
```

## Examples

### Loading Profile Card

```rust
v_flex()
    .gap_4()
    .p_4()
    .border_1()
    .border_color(cx.theme().border)
    .rounded_lg()
    .child(
        h_flex()
            .gap_3()
            .items_center()
            .child(Skeleton::new().size_12().rounded_full()) // Avatar
            .child(
                v_flex()
                    .gap_2()
                    .child(Skeleton::new().w(px(120.)).h_4().rounded_md()) // Name
                    .child(Skeleton::new().w(px(100.)).h_3().rounded_md()) // Email
            )
    )
    .child(
        v_flex()
            .gap_2()
            .child(Skeleton::new().w_full().h_4().rounded_md()) // Bio line 1
            .child(Skeleton::new().w(px(200.)).h_4().rounded_md()) // Bio line 2
    )
```

### Loading Article List

```rust
v_flex()
    .gap_6()
    .children((0..3).map(|_| {
        h_flex()
            .gap_4()
            .child(Skeleton::new().w(px(120.)).h(px(80.)).rounded_md()) // Thumbnail
            .child(
                v_flex()
                    .gap_2()
                    .flex_1()
                    .child(Skeleton::new().w_full().h_5().rounded_md()) // Title
                    .child(Skeleton::new().w(px(300.)).h_4().rounded_md()) // Excerpt line 1
                    .child(Skeleton::new().w(px(250.)).h_4().rounded_md()) // Excerpt line 2
                    .child(Skeleton::new().w(px(100.)).h_3().rounded_md()) // Date
            )
    }))
```

### Loading Table Rows

```rust
v_flex()
    .gap_2()
    .children((0..5).map(|_| {
        h_flex()
            .gap_4()
            .p_3()
            .border_b_1()
            .border_color(cx.theme().border)
            .child(Skeleton::new().size_8().rounded_full()) // Status indicator
            .child(Skeleton::new().w(px(150.)).h_4().rounded_md()) // Name
            .child(Skeleton::new().w(px(200.)).h_4().rounded_md()) // Email
            .child(Skeleton::new().w(px(80.)).h_4().rounded_md()) // Role
            .child(Skeleton::new().w(px(60.)).h_4().rounded_md()) // Actions
    }))
```

### Loading Button States

```rust
h_flex()
    .gap_3()
    .child(Skeleton::new().w(px(80.)).h(px(36.)).rounded_md()) // Primary button
    .child(Skeleton::new().w(px(70.)).h(px(36.)).rounded_md()) // Secondary button
    .child(Skeleton::new().size_9().rounded_md()) // Icon button
```

### Loading Form Fields

```rust
v_flex()
    .gap_4()
    .child(
        v_flex()
            .gap_1()
            .child(Skeleton::new().w(px(60.)).h_4().rounded_md()) // Label
            .child(Skeleton::new().w_full().h(px(40.)).rounded_md()) // Input
    )
    .child(
        v_flex()
            .gap_1()
            .child(Skeleton::new().w(px(80.)).h_4().rounded_md()) // Label
            .child(Skeleton::new().w_full().h(px(120.)).rounded_md()) // Textarea
    )
```

### Conditional Loading

```rust
if loading {
    Skeleton::new().w(px(200.)).h_4().rounded_md()
} else {
    div().child("Actual content here")
}
```

## Theming

The Skeleton component uses the theme's `skeleton` color, which defaults to the `secondary` color if not specified. You can customize it in your theme:

```json
{
  "skeleton.background": "#e2e8f0"
}
```

The `secondary(true)` variant applies 50% opacity to the skeleton color for more subtle loading indicators.

---

---
url: /gpui-component/docs/components/slider.md
description: >-
  A control that allows the user to select values from a range using a draggable
  thumb.
---

# Slider

A slider component for selecting numeric values within a specified range. Supports both single value and range selection modes, horizontal and vertical orientations, custom styling, and step intervals.

## Import

```rust
use gpui_component::slider::{Slider, SliderState, SliderEvent, SliderValue};
```

## Usage

### Basic Slider

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

### Slider with Event Handling

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

### Range Slider

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

### Vertical Slider

```rust
Slider::new(&slider_state)
    .vertical()
    .h(px(200.))
```

### Custom Step Intervals

```rust
// Integer steps
let integer_slider = cx.new(|_| {
    SliderState::new()
        .min(0.0)
        .max(10.0)
        .step(1.0)
        .default_value(5.0)
});

// Decimal steps
let decimal_slider = cx.new(|_| {
    SliderState::new()
        .min(0.0)
        .max(1.0)
        .step(0.01)
        .default_value(0.5)
});
```

### Min/Max Configuration

```rust
// Temperature slider
let temp_slider = cx.new(|_| {
    SliderState::new()
        .min(-10.0)
        .max(40.0)
        .default_value(20.0)
        .step(0.5)
});

// Percentage slider
let percent_slider = cx.new(|_| {
    SliderState::new()
        .min(0.0)
        .max(100.0)
        .default_value(75.0)
        .step(5.0)
});
```

### Disabled State

```rust
Slider::new(&slider_state)
    .disabled(true)
```

### Custom Styling

```rust
Slider::new(&slider_state)
    .bg(cx.theme().success)
    .text_color(cx.theme().success_foreground)
    .rounded(px(4.))
```

### Scale

There have 2 types of scale for the slider:

* `Linear` (default)
* `Logarithmic`

The logarithmic scale is useful when the range of values is large and you want to give more precision to smaller values.

```rust
let log_slider = cx.new(|_| {
    SliderState::new()
        .min(1.0) // min must be greater than 0 for log scale
        .max(1000.0)
        .default_value(10.0)
        .step(1.0)
        .scale(SliderScale::Logarithmic)
});
```

In this case:

:::info
$$ v = min \times (max/min)^p $$

The value `v` is calculated using the formula above, where `p` is the slider percentage (0 to 1).
:::

* If slider at 25%, value will be approximately `5.62`.
* If slider at 50%, value will be approximately `31.62`.
* If slider at 75%, value will be approximately `177.83`.
* If slider at 100%, value will be `1000.0`.

#### Conversions

```rust
// From f32
let single_value: SliderValue = 42.0.into();

// From tuple
let range_value: SliderValue = (10.0, 90.0).into();

// From Range
let range_value: SliderValue = (10.0..90.0).into();
```

### SliderEvent

| Event                 | Description                       |
| --------------------- | --------------------------------- |
| `Change(SliderValue)` | Emitted when slider value changes |

### Styling

The slider component implements `Styled` trait and supports:

* Background color for track and thumb
* Text color for thumb
* Border radius
* Size customization

## Examples

### Color Picker

```rust
struct ColorPicker {
    hue_slider: Entity<SliderState>,
    saturation_slider: Entity<SliderState>,
    lightness_slider: Entity<SliderState>,
    alpha_slider: Entity<SliderState>,
    current_color: Hsla,
}

impl ColorPicker {
    fn new(cx: &mut Context<Self>) -> Self {
        let hue_slider = cx.new(|_| {
            SliderState::new()
                .min(0.0)
                .max(1.0)
                .step(0.01)
                .default_value(0.5)
        });

        let saturation_slider = cx.new(|_| {
            SliderState::new()
                .min(0.0)
                .max(1.0)
                .step(0.01)
                .default_value(1.0)
        });

        // Subscribe to all sliders to update color
        let subscriptions = [&hue_slider, &saturation_slider /* ... */]
            .iter()
            .map(|slider| {
                cx.subscribe(slider, |this, _, event: &SliderEvent, cx| {
                    match event {
                        SliderEvent::Change(_) => {
                            this.update_color(cx);
                        }
                    }
                })
            })
            .collect::<Vec<_>>();

        Self {
            hue_slider,
            saturation_slider,
            // ... other fields
        }
    }

    fn update_color(&mut self, cx: &mut Context<Self>) {
        let h = self.hue_slider.read(cx).value().start();
        let s = self.saturation_slider.read(cx).value().start();
        // ... calculate color
        self.current_color = hsla(h, s, l, a);
        cx.notify();
    }
}

impl Render for ColorPicker {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .gap_4()
            .child(
                v_flex()
                    .gap_2()
                    .child("Hue")
                    .child(Slider::new(&self.hue_slider).vertical().h(px(120.)))
            )
            .child(
                v_flex()
                    .gap_2()
                    .child("Saturation")
                    .child(Slider::new(&self.saturation_slider).vertical().h(px(120.)))
            )
            // ... other sliders
    }
}
```

### Volume Control

```rust
struct VolumeControl {
    volume_slider: Entity<SliderState>,
    volume: f32,
}

impl VolumeControl {
    fn new(cx: &mut Context<Self>) -> Self {
        let volume_slider = cx.new(|_| {
            SliderState::new()
                .min(0.0)
                .max(100.0)
                .step(1.0)
                .default_value(50.0)
        });

        let subscription = cx.subscribe(&volume_slider, |this, _, event: &SliderEvent, cx| {
            match event {
                SliderEvent::Change(value) => {
                    this.volume = value.start();
                    this.apply_volume_change();
                    cx.notify();
                }
            }
        });

        Self {
            volume_slider,
            volume: 50.0,
        }
    }

    fn apply_volume_change(&self) {
        // Apply volume change to audio system
        println!("Volume changed to: {}%", self.volume);
    }
}

impl Render for VolumeControl {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .items_center()
            .gap_3()
            .child("🔊")
            .child(Slider::new(&self.volume_slider).flex_1())
            .child(format!("{}%", self.volume as i32))
    }
}
```

### Price Range Filter

```rust
struct PriceFilter {
    price_range: Entity<SliderState>,
    min_price: f32,
    max_price: f32,
}

impl PriceFilter {
    fn new(cx: &mut Context<Self>) -> Self {
        let price_range = cx.new(|_| {
            SliderState::new()
                .min(0.0)
                .max(1000.0)
                .step(10.0)
                .default_value(100.0..500.0)  // Range slider
        });

        let subscription = cx.subscribe(&price_range, |this, _, event: &SliderEvent, cx| {
            match event {
                SliderEvent::Change(value) => {
                    this.min_price = value.start();
                    this.max_price = value.end();
                    this.filter_products();
                    cx.notify();
                }
            }
        });

        Self {
            price_range,
            min_price: 100.0,
            max_price: 500.0,
        }
    }

    fn filter_products(&self) {
        println!("Filtering products: ${} - ${}", self.min_price, self.max_price);
    }
}

impl Render for PriceFilter {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_2()
            .child("Price Range")
            .child(Slider::new(&self.price_range))
            .child(format!("${} - ${}", self.min_price as i32, self.max_price as i32))
    }
}
```

### Temperature Slider with Custom Styling

```rust
struct TemperatureControl {
    temp_slider: Entity<SliderState>,
    temperature: f32,
}

impl Render for TemperatureControl {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let temp_color = if self.temperature < 10.0 {
            cx.theme().info  // Cold - blue
        } else if self.temperature > 25.0 {
            cx.theme().destructive  // Hot - red
        } else {
            cx.theme().success  // Comfortable - green
        };

        v_flex()
            .gap_3()
            .child("Temperature Control")
            .child(
                Slider::new(&self.temp_slider)
                    .bg(temp_color)
                    .text_color(cx.theme().background)
                    .rounded(px(8.))
            )
            .child(format!("{}°C", self.temperature as i32))
    }
}
```

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

---
url: /gpui-component/docs/components/spinner.md
description: Displays an animated loading showing the completion progress of a task.
---

# Spinner

Spinner element displays an animated loading. Perfect for showing loading states, progress spinners, and other visual feedback during asynchronous operations. Features customizable icons, colors, sizes, and rotation animations.

## Import

```rust
use gpui_component::spinner::Spinner;
```

## Usage

### Basic

```rust
// Default loader icon
Spinner::new()
```

### Spinner with Custom Color

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

### Spinner Sizes

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

### Spinner with Custom Icon

```rust
use gpui_component::IconName;

// Loading circle icon
Spinner::new()
    .icon(IconName::LoaderCircle)

// Large loading circle with custom color
Spinner::new()
    .icon(IconName::LoaderCircle)
    .large()
    .color(cx.theme().cyan)

// Different loading icons
Spinner::new()
    .icon(IconName::Loader)
    .color(cx.theme().primary)
```

## Available Icons

The Spinner component supports various loading and progress icons:

### Loading Icons

* `Loader` (default) - Rotating line spinner
* `LoaderCircle` - Circular loading spinner

### Other Compatible Icons

* Any icon from the `IconName` enum can be used, though loading-specific icons work best with the rotation animation

## Animation

The Spinner component features a built-in rotation animation:

* **Duration**: 0.8 seconds (configurable via speed parameter)
* **Easing**: Ease-in-out transition
* **Repeat**: Infinite loop
* **Transform**: 360-degree rotation

## Size Reference

| Size        | Method              | Approximate Pixels |
| ----------- | ------------------- | ------------------ |
| Extra Small | `.xsmall()`         | ~12px              |
| Small       | `.small()`          | ~14px              |
| Medium      | (default)           | ~16px              |
| Large       | `.large()`          | ~24px              |
| Custom      | `.with_size(px(n))` | n px               |

## Examples

### Loading States

```rust
// Simple loading spinner
Spinner::new()

// Loading with custom color
Spinner::new()
    .color(cx.theme().blue)

// Large loading spinner
Spinner::new()
    .large()
    .color(cx.theme().primary)
```

### Different Loading Icons

```rust
// Default loader (line spinner)
Spinner::new()
    .color(cx.theme().muted_foreground)

// Circle loader
Spinner::new()
    .icon(IconName::LoaderCircle)
    .color(cx.theme().blue)

// Large circle loader with custom color
Spinner::new()
    .icon(IconName::LoaderCircle)
    .large()
    .color(cx.theme().green)
```

### Status Spinners

```rust
// Loading state
Spinner::new()
    .small()
    .color(cx.theme().muted_foreground)

// Processing state
Spinner::new()
    .icon(IconName::LoaderCircle)
    .color(cx.theme().blue)

// Success processing (still animating)
Spinner::new()
    .icon(IconName::LoaderCircle)
    .color(cx.theme().green)
```

### Size Variations

```rust
// Extra small for inline text
Spinner::new()
    .xsmall()
    .color(cx.theme().muted_foreground)

// Small for buttons
Spinner::new()
    .small()
    .color(cx.theme().primary_foreground)

// Medium for general use (default)
Spinner::new()
    .color(cx.theme().primary)

// Large for prominent loading states
Spinner::new()
    .large()
    .color(cx.theme().blue)

// Custom size for specific requirements
Spinner::new()
    .with_size(px(32.))
    .color(cx.theme().orange)
```

### In UI Components

```rust
// In a button
Button::new("submit-btn")
    .loading(true)
    .icon(
        Spinner::new()
            .small()
            .color(cx.theme().primary_foreground)
    )
    .label("Loading...")

// In a card header
div()
    .flex()
    .items_center()
    .gap_2()
    .child("Processing...")
    .child(
        Spinner::new()
            .small()
            .color(cx.theme().muted_foreground)
    )

// Full-screen loading
div()
    .flex()
    .items_center()
    .justify_center()
    .h_full()
    .w_full()
    .child(
        Spinner::new()
            .large()
            .color(cx.theme().primary)
    )
```

## Performance Considerations

* The animation uses CSS transforms for optimal performance
* Multiple spinners on the same page share the same animation timing
* The component is lightweight and suitable for frequent updates
* Consider using smaller sizes for better performance with many spinners

## Common Patterns

### Conditional Loading

```rust
// Show spinner only when loading
.when(is_loading, |this| {
    this.child(
        Spinner::new()
            .small()
            .color(cx.theme().muted_foreground)
    )
})
```

### Loading with Text

```rust
// Loading text with spinner
h_flex()
    .items_center()
    .gap_2()
    .child(
        Spinner::new()
            .small()
            .color(cx.theme().primary)
    )
    .child("Loading data...")
```

### Overlay Loading

```rust
// Full overlay with spinner
div()
    .absolute()
    .inset_0()
    .flex()
    .items_center()
    .justify_center()
    .bg(cx.theme().background.alpha(0.8))
    .child(
        v_flex()
            .items_center()
            .gap_3()
            .child(
                Spinner::new()
                    .large()
                    .color(cx.theme().primary)
            )
            .child("Loading...")
    )
```

---

---
url: /gpui-component/docs/components/stepper.md
description: >-
  A step-by-step progress for users to navigate through a series of steps or
  stages.
---

# Stepper

A step-by-step progress component that guides users through a series of steps or stages. Supports horizontal and vertical layouts, custom icons, and different sizes.

## Import

```rust
use gpui_component::stepper::{Stepper, StepperItem};
```

## Usage

### Basic Stepper

Use `selected_index` method to set current active step by index (0-based), default is `0`.

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

### With Icons

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

### Vertical Layout

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

### Text Center

The `text_center` method centers the text within each step item.

```rust
Stepper::new("center-stepper")
    .selected_index(0)
    .text_center(true)
    .items([
        StepperItem::new().child(
            v_flex()
                .items_center()
                .child("Step 1")
                .child("Desc for step 1."),
        ),
        StepperItem::new().child(
            v_flex()
                .items_center()
                .child("Step 2")
                .child("Desc for step 2."),
        ),
        StepperItem::new().child(
            v_flex()
                .items_center()
                .child("Step 3")
                .child("Desc for step 3."),
        ),
    ])
```

### Different Sizes

```rust
use gpui_component::{Sizable as _, Size};

Stepper::new("stepper")
    .xsmall()
    .items([...])

Stepper::new("stepper")
    .small()
    .items([...])

Stepper::new("stepper")
    .large()
    .items([...])
```

### Disabled State

```rust
Stepper::new("disabled-stepper")
    .disabled(true)
    .items([
        StepperItem::new().child("Step 1"),
        StepperItem::new().child("Step 2"),
    ])
```

### Handle Click Events

```rust
Stepper::new("my-stepper")
    .selected_index(current_step)
    .items([
        StepperItem::new().child("Step 1"),
        StepperItem::new().child("Step 2"),
        StepperItem::new().child("Step 3"),
    ])
    .on_click(cx.listener(|this, step, _, cx| {
        this.current_step = *step;
        cx.notify();
    }))
```

## API Reference

* [Stepper]
* [StepperItem]

### Sizing

Implements [Sizable] trait:

* `xsmall()` - Extra small size
* `small()` - Small size
* `medium()` - Medium size (default)
* `large()` - Large size

## Examples

### Multi-step Form

```rust
Stepper::new("form-stepper")
    .w_full()
    .selected_index(form_step)
    .items([
        StepperItem::new()
            .icon(IconName::User)
            .child("Personal Info"),
        StepperItem::new()
            .icon(IconName::CreditCard)
            .child("Payment"),
        StepperItem::new()
            .icon(IconName::CircleCheck)
            .child("Confirmation"),
    ])
    .on_click(cx.listener(|this, step, _, cx| {
        this.form_step = *step;
        cx.notify();
    }))
```

### Disabled Individual Steps

```rust
Stepper::new("stepper")
    .selected_index(0)
    .items([
        StepperItem::new().child("Available"),
        StepperItem::new().disabled(true).child("Locked"),
        StepperItem::new().child("Available"),
    ])
```

[Stepper]: https://docs.rs/gpui-component/latest/gpui_component/stepper/struct.Stepper.html

[StepperItem]: https://docs.rs/gpui-component/latest/gpui_component/stepper/struct.StepperItem.html

[Sizable]: https://docs.rs/gpui-component/latest/gpui_component/trait.Sizable.html

---

---
url: /gpui-component/docs/components/switch.md
description: A control that allows the user to toggle between checked and not checked.
---

# Switch

A toggle switch component for binary on/off states. Features smooth animations, different sizes, labels, disabled state, and customizable positioning.

## Import

```rust
use gpui_component::switch::Switch;
```

## Usage

### Basic Switch

```rust
Switch::new("my-switch")
    .checked(false)
    .on_click(|checked, _, _| {
        println!("Switch is now: {}", checked);
    })
```

### Controlled Switch

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

### With Label

```rust
Switch::new("notifications")
    .label("Enable notifications")
    .checked(true)
    .on_click(|checked, _, _| {
        println!("Notifications: {}", if *checked { "enabled" } else { "disabled" });
    })
```

### Different Sizes

```rust
// Small switch
Switch::new("small-switch")
    .small()
    .label("Small switch")

// Medium switch (default)
Switch::new("medium-switch")
    .label("Medium switch")

// Using explicit size
Switch::new("custom-switch")
    .with_size(Size::Small)
    .label("Custom size")
```

### Disabled State

```rust
// Disabled unchecked
Switch::new("disabled-off")
    .label("Disabled (off)")
    .disabled(true)
    .checked(false)

// Disabled checked
Switch::new("disabled-on")
    .label("Disabled (on)")
    .disabled(true)
    .checked(true)
```

### With Tooltip

```rust
Switch::new("switch")
    .label("Airplane mode")
    .tooltip("Enable airplane mode to disable all wireless connections")
    .checked(false)
```

## API Reference

### Switch

| Method             | Description                                                 |
| ------------------ | ----------------------------------------------------------- |
| `new(id)`          | Create a new switch with the given ID                       |
| `checked(bool)`    | Set the checked/toggled state                               |
| `label(text)`      | Set label text for the switch                               |
| `label_side(side)` | Position label (Side::Left or Side::Right)                  |
| `disabled(bool)`   | Set disabled state                                          |
| `tooltip(text)`    | Add tooltip text                                            |
| `on_click(fn)`     | Callback when clicked, receives `&bool` (new checked state) |

### Styling

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

## Examples

### Settings Panel

```rust
struct SettingsView {
    marketing_emails: bool,
    security_emails: bool,
    push_notifications: bool,
}

v_flex()
    .gap_4()
    .child(
        // Setting with description
        v_flex()
            .gap_2()
            .child(
                h_flex()
                    .items_center()
                    .justify_between()
                    .child(
                        v_flex()
                            .child(Label::new("Marketing emails").text_lg())
                            .child(
                                Label::new("Receive emails about new products and features")
                                    .text_color(theme.muted_foreground)
                            )
                    )
                    .child(
                        Switch::new("marketing")
                            .checked(self.marketing_emails)
                            .on_click(cx.listener(|view, checked, _, cx| {
                                view.marketing_emails = *checked;
                                cx.notify();
                            }))
                    )
            )
    )
    .child(
        // Simple setting
        h_flex()
            .items_center()
            .justify_between()
            .child(Label::new("Push notifications"))
            .child(
                Switch::new("push")
                    .checked(self.push_notifications)
                    .on_click(cx.listener(|view, checked, _, cx| {
                        view.push_notifications = *checked;
                        cx.notify();
                    }))
            )
    )
```

### Compact Settings List

```rust
v_flex()
    .gap_3()
    .child(
        Switch::new("wifi")
            .label("Wi-Fi")
            .label_side(Side::Left)
            .checked(true)
            .small()
    )
    .child(
        Switch::new("bluetooth")
            .label("Bluetooth")
            .label_side(Side::Left)
            .checked(false)
            .small()
    )
    .child(
        Switch::new("airplane")
            .label("Airplane Mode")
            .label_side(Side::Left)
            .checked(false)
            .disabled(true)
            .small()
    )
```

### Form Integration

```rust
struct FormData {
    subscribe_newsletter: bool,
    enable_notifications: bool,
    remember_me: bool,
}

v_flex()
    .gap_4()
    .p_4()
    .border_1()
    .border_color(theme.border)
    .rounded(theme.radius)
    .child(
        Switch::new("newsletter")
            .label("Subscribe to newsletter")
            .checked(self.subscribe_newsletter)
            .tooltip("Receive monthly updates about new features")
            .on_click(cx.listener(|view, checked, _, cx| {
                view.subscribe_newsletter = *checked;
                cx.notify();
            }))
    )
    .child(
        Switch::new("notifications")
            .label("Enable notifications")
            .checked(self.enable_notifications)
            .on_click(cx.listener(|view, checked, _, cx| {
                view.enable_notifications = *checked;
                cx.notify();
            }))
    )
    .child(
        Switch::new("remember")
            .label("Remember me")
            .checked(self.remember_me)
            .small()
            .on_click(cx.listener(|view, checked, _, cx| {
                view.remember_me = *checked;
                cx.notify();
            }))
    )
```

### Custom Styling

```rust
Switch::new("custom")
    .label("Custom styled switch")
    .w(px(200.))
    .checked(true)
    .on_click(|checked, _, _| {
        println!("Custom switch: {}", checked);
    })
```

## Animation

The switch features smooth animations:

* **Toggle animation**: 150ms duration when switching states
* **Background color transition**: Changes from switch color to primary color
* **Position animation**: Smooth movement of the toggle indicator
* **Disabled state**: Animations are disabled when the switch is disabled

---

---
url: /gpui-component/docs/components/table.md
description: >-
  High-performance data table with virtual scrolling, sorting, filtering, and
  column management.
---

# Table

A comprehensive data table component designed for handling large datasets with high performance. Features virtual scrolling, column configuration, sorting, filtering, row selection, and custom cell rendering. Perfect for displaying tabular data with thousands of rows while maintaining smooth performance.

## Import

```rust
use gpui_component::table::{Table, TableState, TableDelegate, Column, ColumnSort, ColumnFixed, TableEvent};
```

## Usage

### Basic Table

To create a table, you need to implement the `TableDelegate` trait and provide column definitions, and use `TableState` to manage the table state.

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

### Column Configuration

Columns provide extensive configuration options:

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

### Virtual Scrolling for Large Datasets

The table automatically handles virtual scrolling for optimal performance:

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

### Sorting Implementation

Implement sorting in your delegate:

```rust
impl TableDelegate for MyTableDelegate {
    fn perform_sort(&mut self, col_ix: usize, sort: ColumnSort, _: &mut Window, _: &mut Context<TableState<Self>>) {
        let col = &self.columns[col_ix];

        match col.key.as_ref() {
            "name" => {
                match sort {
                    ColumnSort::Ascending => self.data.sort_by(|a, b| a.name.cmp(&b.name)),
                    ColumnSort::Descending => self.data.sort_by(|a, b| b.name.cmp(&a.name)),
                    ColumnSort::Default => {
                        // Reset to original order or default sort
                        self.data.sort_by(|a, b| a.id.cmp(&b.id));
                    }
                }
            }
            "age" => {
                match sort {
                    ColumnSort::Ascending => self.data.sort_by(|a, b| a.age.cmp(&b.age)),
                    ColumnSort::Descending => self.data.sort_by(|a, b| b.age.cmp(&a.age)),
                    ColumnSort::Default => self.data.sort_by(|a, b| a.id.cmp(&b.id)),
                }
            }
            _ => {}
        }
    }
}
```

### ContextMenu

```rust
impl TableDelegate for MyTableDelegate {
    // Context menu for right-click
    fn context_menu(&mut self, row_ix: usize, menu: PopupMenu, _: &mut Window, _: &mut Context<TableState<Self>>) -> PopupMenu {
        let row = &self.data[row_ix];
        menu.menu(format!("Edit {}", row.name), Box::new(EditRowAction(row_ix)))
            .menu("Delete", Box::new(DeleteRowAction(row_ix)))
            .separator()
            .menu("Duplicate", Box::new(DuplicateRowAction(row_ix)))
    }
}
```

### Cell Rendering

Create rich cell content with custom rendering:

```rust
impl TableDelegate for MyTableDelegate {
    fn render_td(&mut self, row_ix: usize, col_ix: usize, _: &mut Window, cx: &mut Context<TableState<Self>>) -> impl IntoElement {
        let row = &self.data[row_ix];
        let col = &self.columns[col_ix];

        match col.key.as_ref() {
            "status" => {
                // Custom status badge
                let (color, text) = match row.status {
                    Status::Active => (cx.theme().green, "Active"),
                    Status::Inactive => (cx.theme().red, "Inactive"),
                    Status::Pending => (cx.theme().yellow, "Pending"),
                };

                div()
                    .px_2()
                    .py_1()
                    .rounded(px(4.))
                    .bg(color.opacity(0.1))
                    .text_color(color)
                    .child(text)
            }
            "progress" => {
                // Progress bar
                div()
                    .w_full()
                    .h(px(8.))
                    .bg(cx.theme().muted)
                    .rounded(px(4.))
                    .child(
                        div()
                            .h_full()
                            .w(percentage(row.progress))
                            .bg(cx.theme().primary)
                            .rounded(px(4.))
                    )
            }
            "actions" => {
                // Action buttons
                h_flex()
                    .gap_1()
                    .child(Button::new(format!("edit-{}", row_ix)).text().icon(IconName::Edit))
                    .child(Button::new(format!("delete-{}", row_ix)).text().icon(IconName::Trash))
            }
            "avatar" => {
                // User avatar with image
                h_flex()
                    .items_center()
                    .gap_2()
                    .child(
                        div()
                            .w(px(32.))
                            .h(px(32.))
                            .rounded_full()
                            .bg(cx.theme().accent)
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(row.name.chars().next().unwrap_or('?').to_string())
                    )
                    .child(row.name.clone())
            }
            _ => row.get_field_value(col.key.as_ref()).into_any_element(),
        }
    }
}
```

### Column Resizing and Moving

Enable dynamic column management:

```rust
// Configure table features
let state = cx.new(|cx| {
    TableState::new(delegate, window, cx)
        .col_resizable(true)  // Allow column resizing
        .col_movable(true)    // Allow column reordering
        .sortable(true)       // Enable sorting
        .col_selectable(true) // Allow column selection
        .row_selectable(true) // Allow row selection
});

// Listen for column changes
cx.subscribe_in(&state, window, |view, table, event, _, cx| {
    match event {
        TableEvent::ColumnWidthsChanged(widths) => {
            // Save column widths to user preferences
            save_column_widths(widths);
        }
        TableEvent::MoveColumn(from_ix, to_ix) => {
            // Save column order
            save_column_order(from_ix, to_ix);
        }
        _ => {}
    }
}).detach();
```

### Infinite Loading / Pagination

Implement loading more data as user scrolls:

```rust
impl TableDelegate for MyTableDelegate {
    fn has_more(&self, _: &App) -> bool {
        self.has_more_data
    }

    fn load_more_threshold(&self) -> usize {
        50 // Load more when 50 rows from bottom
    }

    fn load_more(&mut self, _: &mut Window, cx: &mut Context<TableState<Self>>) {
        if self.loading {
            return; // Prevent multiple loads
        }

        self.loading = true;

        // Spawn async task to load data
        cx.spawn(async move |view, cx| {
            let new_data = fetch_more_data().await;

            cx.update(|cx| {
                view.update(cx, |view, _| {
                    let delegate = view.table.delegate_mut();
                    delegate.data.extend(new_data);
                    delegate.loading = false;
                    delegate.has_more_data = !new_data.is_empty();
                });
            })
        }).detach();
    }

    fn loading(&self, _: &App) -> bool {
        self.loading
    }
}
```

### Table Styling

Customize table appearance:

```rust
let state = cx.new(|cx| {
    TableState::new(delegate, window, cx)
});

// In render
Table::new(&state)
    .stripe(true)           // Alternating row colors
    .bordered(true)           // Border around table
    .scrollbar_visible(true, true) // Vertical, horizontal scrollbars
```

## Examples

### Financial Data Table

```rust
struct StockData {
    symbol: String,
    price: f64,
    change: f64,
    change_percent: f64,
    volume: u64,
}

impl TableDelegate for StockTableDelegate {
    fn render_td(&mut self, row_ix: usize, col_ix: usize, _: &mut Window, cx: &mut Context<TableState<Self>>) -> impl IntoElement {
        let stock = &self.stocks[row_ix];
        let col = &self.columns[col_ix];

        match col.key.as_ref() {
            "symbol" => div().font_weight(FontWeight::BOLD).child(stock.symbol.clone()),
            "price" => div().text_right().child(format!("${:.2}", stock.price)),
            "change" => {
                let color = if stock.change >= 0.0 { cx.theme().green } else { cx.theme().red };
                div()
                    .text_right()
                    .text_color(color)
                    .child(format!("{:+.2}", stock.change))
            }
            "change_percent" => {
                let color = if stock.change_percent >= 0.0 { cx.theme().green } else { cx.theme().red };
                div()
                    .text_right()
                    .text_color(color)
                    .child(format!("{:+.1}%", stock.change_percent * 100.0))
            }
            "volume" => div().text_right().child(format!("{:,}", stock.volume)),
            _ => div(),
        }
    }
}
```

### User Management Table

```rust
struct UserTableDelegate {
    users: Vec<User>,
    columns: Vec<Column>,
}

impl UserTableDelegate {
    fn new() -> Self {
        Self {
            users: Vec::new(),
            columns: vec![
                Column::new("avatar", "").width(50.).resizable(false).movable(false),
                Column::new("name", "Name").width(150.).sortable().fixed_left(),
                Column::new("email", "Email").width(200.).sortable(),
                Column::new("role", "Role").width(100.).sortable(),
                Column::new("status", "Status").width(100.),
                Column::new("last_login", "Last Login").width(120.).sortable(),
                Column::new("actions", "Actions").width(100.).resizable(false),
            ],
        }
    }
}
```

## Keyboard shortcuts

* `↑/↓` - Navigate rows
* `←/→` - Navigate columns
* `Enter/Space` - Select row/column
* `Escape` - Clear selection

---

---
url: /gpui-component/docs/components/tabs.md
description: >-
  A set of layered sections of content—known as tab panels—that are displayed
  one at a time.
---

# Tabs

A tabbed interface component for organizing content into separate sections. Supports multiple variants, sizes, navigation controls, and interactive features like reordering and prefix/suffix elements.

## Import

```rust
use gpui_component::tab::{Tab, TabBar};
```

## Usage

### Basic Tabs

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

### Tab Variants

#### Default Tabs

```rust
TabBar::new("default-tabs")
    .selected_index(0)
    .child(Tab::new().label("Account"))
    .child(Tab::new().label("Profile"))
    .child(Tab::new().label("Documents"))
```

#### Underline Tabs

```rust
TabBar::new("underline-tabs")
    .underline()
    .selected_index(0)
    .child(Tab::new().label("Account"))
    .child(Tab::new().label("Profile"))
    .child(Tab::new().label("Documents"))
```

#### Pill Tabs

```rust
TabBar::new("pill-tabs")
    .pill()
    .selected_index(0)
    .child(Tab::new().label("Account"))
    .child(Tab::new().label("Profile"))
    .child(Tab::new().label("Documents"))
```

#### Outline Tabs

```rust
TabBar::new("outline-tabs")
    .outline()
    .selected_index(0)
    .child(Tab::new().label("Account"))
    .child(Tab::new().label("Profile"))
    .child(Tab::new().label("Documents"))
```

#### Segmented Tabs

```rust
use gpui_component::IconName;

TabBar::new("segmented-tabs")
    .segmented()
    .selected_index(0)
    .child(IconName::Bot)
    .child(IconName::Calendar)
    .child(IconName::Map)
    .children(vec!["Settings", "About"])
```

### Tab Sizes

```rust
// Extra Small
TabBar::new("tabs").xsmall()
    .child(Tab::new().label("Small"))

// Small
TabBar::new("tabs").small()
    .child(Tab::new().label("Small"))

// Medium (default)
TabBar::new("tabs")
    .child(Tab::new().label("Medium"))

// Large
TabBar::new("tabs").large()
    .child(Tab::new().label("Large"))
```

### Tabs with Icons

```rust
use gpui_component::{Icon, IconName};

TabBar::new("icon-tabs")
    .child(Tab::default().icon(IconName::User).with_variant(TabVariant::Tab))
    .child(Tab::default().icon(IconName::Settings).with_variant(TabVariant::Tab))
    .child(Tab::default().icon(IconName::Mail).with_variant(TabVariant::Tab))
```

### Tabs with Prefix and Suffix

```rust
use gpui_component::button::Button;
use gpui_component::{h_flex, IconName};

TabBar::new("tabs-with-controls")
    .prefix(
        h_flex()
            .gap_1()
            .child(Button::new("back").ghost().xsmall().icon(IconName::ArrowLeft))
            .child(Button::new("forward").ghost().xsmall().icon(IconName::ArrowRight))
    )
    .suffix(
        h_flex()
            .gap_1()
            .child(Button::new("inbox").ghost().xsmall().icon(IconName::Inbox))
            .child(Button::new("more").ghost().xsmall().icon(IconName::Ellipsis))
    )
    .child(Tab::new().label("Account"))
    .child(Tab::new().label("Profile"))
    .child(Tab::new().label("Settings"))
```

### Disabled Tabs

```rust
TabBar::new("tabs-with-disabled")
    .child(Tab::new().label("Account"))
    .child(Tab::new().label("Profile").disabled(true))
    .child(Tab::new().label("Settings"))
```

### Dynamic Tabs

```rust
struct TabsView {
    active_tab: usize,
    tabs: Vec<String>,
}

impl Render for TabsView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        TabBar::new("dynamic-tabs")
            .selected_index(self.active_tab)
            .on_click(cx.listener(|view, index, _, cx| {
                view.active_tab = *index;
                cx.notify();
            }))
            .children(
                self.tabs
                    .iter()
                    .map(|tab_name| Tab::new().label(tab_name.clone()))
            )
    }
}
```

### Tabs with Menu

Use `menu` option to enable a dropdown menu for tab selection when there are many tabs,
this is default `false`.

If enable, the will have a dropdown button at the end of the tab bar to show all tabs in a menu.

```rust
TabBar::new("tabs-with-menu")
    .menu(true)
    .selected_index(0)
    .child(Tab::new().label("Account"))
    .child(Tab::new().label("Profile"))
    .child(Tab::new().label("Documents"))
    .child(Tab::new().label("Mail"))
    .child(Tab::new().label("Settings"))
```

### Scrollable Tabs

```rust
use gpui::ScrollHandle;

struct ScrollableTabsView {
    scroll_handle: ScrollHandle,
}

impl Render for ScrollableTabsView {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        TabBar::new("scrollable-tabs")
            .track_scroll(&self.scroll_handle)
            .child(Tab::new().label("Very Long Tab Name 1"))
            .child(Tab::new().label("Very Long Tab Name 2"))
            .child(Tab::new().label("Very Long Tab Name 3"))
            .child(Tab::new().label("Very Long Tab Name 4"))
            .child(Tab::new().label("Very Long Tab Name 5"))
    }
}
```

### Individual Tab Configuration

```rust
TabBar::new("custom-tabs")
    .child(
        Tab::new().label("Custom Tab")
            .id("custom-id")
            .prefix(IconName::Star)
            .suffix(IconName::X)
            .on_click(|_, _, _| {
                println!("Custom tab clicked");
            })
    )
```

## API Reference

### TabBar

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

### TabBar Variants

| Method                  | Description                          |
| ----------------------- | ------------------------------------ |
| `with_variant(variant)` | Set the tab variant for all children |
| `underline()`           | Use underline variant                |
| `pill()`                | Use pill variant                     |
| `outline()`             | Use outline variant                  |
| `segmented()`           | Use segmented variant                |

### Tab

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

### TabVariant

```rust
pub enum TabVariant {
    Tab,      // Default bordered tabs
    Outline,  // Rounded outline tabs
    Pill,     // Rounded pill-shaped tabs
    Segmented, // Segmented control style
    Underline, // Underline indicator tabs
}
```

### Styling

Both `TabBar` and `Tab` implement `Sizable` trait:

* `xsmall()` - Extra small size
* `small()` - Small size
* `medium()` - Medium size (default)
* `large()` - Large size

## Advanced Examples

### Custom Tab Content

```rust
Tab::empty()
    .child(
        h_flex()
            .items_center()
            .gap_2()
            .child(IconName::Folder)
            .child("Documents")
            .child(
                div()
                    .px_1()
                    .py_0p5()
                    .text_xs()
                    .bg(cx.theme().accent)
                    .text_color(cx.theme().accent_foreground)
                    .rounded_sm()
                    .child("12")
            )
    )
```

### Tabs with State Management

```rust
struct TabsWithContent {
    active_tab: usize,
    tab_contents: Vec<String>,
}

impl TabsWithContent {
    fn render_tab_content(&self, cx: &mut Context<Self>) -> impl IntoElement {
        match self.active_tab {
            0 => div().child("Account content"),
            1 => div().child("Profile content"),
            2 => div().child("Settings content"),
            _ => div().child("Unknown content"),
        }
    }
}

impl Render for TabsWithContent {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .child(
                TabBar::new("content-tabs")
                    .selected_index(self.active_tab)
                    .on_click(cx.listener(|view, index, _, cx| {
                        view.active_tab = *index;
                        cx.notify();
                    }))
                    .child(Tab::new().label("Account"))
                    .child(Tab::new().label("Profile"))
                    .child(Tab::new().label("Settings"))
            )
            .child(
                div()
                    .flex_1()
                    .p_4()
                    .child(self.render_tab_content(cx))
            )
    }
}
```

### Tabs with Close Buttons

While the basic Tab component doesn't include closeable functionality, you can create closeable tabs using suffix elements:

```rust
struct CloseableTabsView {
    tabs: Vec<String>,
    active_tab: usize,
}

impl CloseableTabsView {
    fn close_tab(&mut self, index: usize, cx: &mut Context<Self>) {
        if self.tabs.len() > 1 {
            self.tabs.remove(index);
            if self.active_tab >= index && self.active_tab > 0 {
                self.active_tab -= 1;
            }
            cx.notify();
        }
    }
}

impl Render for CloseableTabsView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        TabBar::new("closeable-tabs")
            .selected_index(self.active_tab)
            .on_click(cx.listener(|view, index, _, cx| {
                view.active_tab = *index;
                cx.notify();
            }))
            .children(
                self.tabs
                    .iter()
                    .enumerate()
                    .map(|(index, tab_name)| {
                        Tab::new().label(tab_name.clone())
                            .suffix(
                                Button::new(format!("close-{}", index))
                                    .icon(IconName::X)
                                    .ghost()
                                    .xsmall()
                                    .on_click(cx.listener(move |view, _, _, cx| {
                                        view.close_tab(index, cx);
                                    }))
                            )
                    })
            )
    }
}
```

## Notes

* The `TabBar` manages the selection state of all child tabs
* Individual tab `on_click` handlers are ignored when `TabBar.on_click` is set
* Tabs automatically inherit the variant and size from their parent `TabBar`
* The `with_menu` option adds a dropdown for tab selection when there are many tabs
* Scrolling is automatically enabled when tabs overflow the container width
* The dock system provides advanced closeable tab functionality for complex layouts

---

---
url: /gpui-component/docs/components/tag.md
description: A short item that can be used to categorize or label content.
---

# Tag

A versatile tag component for categorizing and labeling content. Tags are compact visual indicators that help organize information and display metadata like categories, status, or properties.

## Import

```rust
use gpui_component::tag::Tag;
```

## Usage

### Basic Tags

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

### Tag Variants

```rust
// Semantic variants
Tag::primary().child("Featured")
Tag::secondary().child("Category")
Tag::danger().child("Critical")
Tag::success().child("Completed")
Tag::warning().child("Pending")
Tag::info().child("Information")
```

### Outline Tags

```rust
// Outline style variants
Tag::primary().outline().child("Primary Outline")
Tag::secondary().outline().child("Secondary Outline")
Tag::danger().outline().child("Error Outline")
Tag::success().outline().child("Success Outline")
Tag::warning().outline().child("Warning Outline")
Tag::info().outline().child("Info Outline")
```

### Tag Sizes

```rust
// Small size
Tag::primary().small().child("Small Tag")

// Medium size (default)
Tag::primary().child("Medium Tag")
```

### Custom Colors

```rust
use gpui_component::ColorName;

// Using predefined color names
Tag::color(ColorName::Blue).child("Blue Tag")
Tag::color(ColorName::Green).child("Green Tag")
Tag::color(ColorName::Purple).child("Purple Tag")
Tag::color(ColorName::Pink).child("Pink Tag")
Tag::color(ColorName::Indigo).child("Indigo Tag")
Tag::color(ColorName::Yellow).child("Yellow Tag")
Tag::color(ColorName::Red).child("Red Tag")
```

### Custom HSLA Colors

```rust
use gpui::{hsla, Hsla};

// Custom colors with HSLA values
let color = hsla(220.0 / 360.0, 0.8, 0.5, 1.0);
let foreground = hsla(0.0, 0.0, 1.0, 1.0);
let border = hsla(220.0 / 360.0, 0.8, 0.4, 1.0);

Tag::custom(color, foreground, border).child("Custom Color")
```

### Rounded Corners

```rust
use gpui::px;

// Fully rounded tags
Tag::primary().rounded_full().child("Rounded Full")

// Custom border radius
Tag::primary().rounded(px(4.0)).child("Custom Radius")

// Square corners
Tag::primary().rounded(px(0.0)).child("Square Tag")
```

### Combined Styles

```rust
// Small tags with full rounding
Tag::primary().small().rounded_full().child("Small Pill")
Tag::success().small().rounded_full().child("Success Pill")

// Outline tags with custom rounding
Tag::warning().outline().rounded(px(2.0)).child("Custom Outline")

// Color tags with outline style
Tag::color(ColorName::Purple).outline().child("Purple Outline")
```

## Tag Categories and Use Cases

### Status Tags

```rust
// Task or item status
Tag::success().child("Completed")
Tag::warning().child("In Progress")
Tag::danger().child("Failed")
Tag::info().child("Pending Review")
```

### Category Labels

```rust
// Content categorization
Tag::secondary().child("Technology")
Tag::color(ColorName::Blue).child("Design")
Tag::color(ColorName::Green).child("Development")
Tag::color(ColorName::Purple).child("Marketing")
```

### Priority Indicators

```rust
// Priority levels
Tag::danger().child("High Priority")
Tag::warning().child("Medium Priority")
Tag::secondary().child("Low Priority")
```

### Feature Tags

```rust
// Feature flags or attributes
Tag::primary().small().child("New")
Tag::success().small().child("Popular")
Tag::info().small().child("Beta")
Tag::warning().small().child("Limited")
```

## API Reference

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

### Style Methods

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

## Examples

### Tag Collections

```rust
use gpui_component::{h_flex, v_flex};

// Horizontal tag group
h_flex()
    .gap_2()
    .child(Tag::primary().child("React"))
    .child(Tag::success().child("TypeScript"))
    .child(Tag::info().child("Next.js"))
    .child(Tag::warning().child("Beta"))

// Vertical tag stack
v_flex()
    .gap_1()
    .child(Tag::danger().small().child("Critical"))
    .child(Tag::warning().small().child("Important"))
    .child(Tag::secondary().small().child("Normal"))
```

### Status Dashboard Tags

```rust
// System status indicators
h_flex()
    .gap_3()
    .child(
        v_flex()
            .child("API Status:")
            .child(Tag::success().child("Operational"))
    )
    .child(
        v_flex()
            .child("Database:")
            .child(Tag::warning().child("Maintenance"))
    )
    .child(
        v_flex()
            .child("Cache:")
            .child(Tag::danger().child("Down"))
    )
```

### Interactive Tag Lists

```rust
// Note: Event handling would require additional state management
// Tags themselves are display components

// Filter tags (would need click handlers)
h_flex()
    .gap_2()
    .child(Tag::primary().small().child("All"))
    .child(Tag::secondary().outline().small().child("Active"))
    .child(Tag::secondary().outline().small().child("Completed"))
    .child(Tag::secondary().outline().small().child("Archived"))
```

### Color-Coded Categories

```rust
use gpui_component::ColorName;

// Content type tags
h_flex()
    .gap_2()
    .flex_wrap()
    .child(Tag::color(ColorName::Red).child("Bug"))
    .child(Tag::color(ColorName::Blue).child("Feature"))
    .child(Tag::color(ColorName::Green).child("Enhancement"))
    .child(Tag::color(ColorName::Purple).child("Documentation"))
    .child(Tag::color(ColorName::Yellow).child("Question"))
    .child(Tag::color(ColorName::Pink).child("Discussion"))
```

### Pill-Style Tags

```rust
// Skill tags with pill styling
h_flex()
    .gap_2()
    .flex_wrap()
    .child(Tag::color(ColorName::Blue).rounded_full().small().child("Rust"))
    .child(Tag::color(ColorName::Green).rounded_full().small().child("JavaScript"))
    .child(Tag::color(ColorName::Purple).rounded_full().small().child("Python"))
    .child(Tag::color(ColorName::Red).rounded_full().small().child("Go"))
```

## Behavior Notes

* Tags automatically adjust their appearance based on the current theme
* Outline tags maintain border visibility across different backgrounds
* Small tags use reduced padding and border radius for compact layouts
* Custom colors support both light and dark theme adaptations
* Tags are display components and don't include built-in interaction handlers
* Multiple tags can be combined in flex layouts for tag clouds or lists
* Border radius automatically scales based on tag size unless explicitly overridden

## Design Guidelines

### When to Use Tags

* **Categorization**: Group content by type, topic, or theme
* **Status Indication**: Show state, progress, or health status
* **Metadata Display**: Present attributes, properties, or classifications
* **Filtering**: Visual indicators for active filters or selections
* **Feature Flags**: Highlight new, beta, or special features

### Color Usage

* **Semantic Colors**: Use danger (red) for errors, success (green) for completion, warning (yellow) for caution, info (blue) for information
* **Category Colors**: Use the ColorName variants for content categorization where color coding helps with recognition
* **Custom Colors**: Reserve for brand colors or specific design system requirements

### Size Guidelines

* **Small Tags**: Use for compact layouts, metadata, or when space is limited
* **Medium Tags**: Default size for most use cases, provides good readability and click targets
* **Rounding**: Use `rounded_full()` for pill-style tags, custom `rounded()` for specific design requirements

---

---
url: /gpui-component/docs/theme.md
---

# Theme

All components support theming through the built-in Theme system, the [ActiveTheme] trait provides access to the current theme colors:

```rs
use gpui_component::{ActiveTheme as _};

// Access theme colors in your components
cx.theme().primary
cx.theme().background
cx.theme().foreground
```

So if you want use the colors from the current theme, you should keep your component or view have [App] context.

## Theme Registry

There have more than 20 built-in themes available in [themes](https://github.com/longbridge/gpui-component/tree/main/themes) folder.

https://github.com/longbridge/gpui-component/tree/main/themes

And we have a [ThemeRegistry] to help us to load themes.

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

[ActiveTheme]: https://docs.rs/gpui-component/latest/gpui_component/theme/trait.ActiveTheme.html

[ThemeRegistry]: https://docs.rs/gpui-component/latest/gpui_component/theme/struct.ThemeRegistry.html

[App]: https://docs.rs/gpui/latest/gpui/struct.App.html

---

---
url: /gpui-component/docs/components/title-bar.md
description: >-
  A custom window title bar component with window controls and custom content
  support.
---

# TitleBar

TitleBar provides a customizable window title bar that can replace the default OS title bar. It includes platform-specific window controls (minimize, maximize, close) and supports custom content and styling. The component automatically adapts to different operating systems (macOS, Windows, Linux) with appropriate behaviors and visual styles.

## Import

```rust
use gpui_component::TitleBar;
```

## Usage

### Basic Title Bar

```rust
TitleBar::new()
    .child(div().child("My Application"))
```

### Title Bar with Custom Content

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

### Title Bar with Menu Bar

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

### Title Bar with Window Controls (Linux only)

```rust
TitleBar::new()
    .on_close_window(|_, window, cx| {
        // Custom close behavior
        window.push_notification("Saving before close...", cx);
        // Perform cleanup
        window.remove_window();
    })
    .child(div().child("Custom Close Behavior"))
```

### Styled Title Bar

```rust
TitleBar::new()
    .bg(cx.theme().primary)
    .border_color(cx.theme().primary_border)
    .child(
        div()
            .text_color(cx.theme().primary_foreground)
            .child("Styled Title Bar")
    )
```

### Title Bar Options for Window

```rust
use gpui::{WindowOptions, TitlebarOptions};

WindowOptions {
    titlebar: Some(TitleBar::title_bar_options()),
    ..Default::default()
}
```

## Platform Differences

### macOS

* Uses native traffic light buttons (minimize, maximize, close)
* Traffic light position is automatically set to `(9px, 9px)`
* Double-click behavior calls `window.titlebar_double_click()`
* Left padding accounts for traffic light buttons (80px)
* Appears transparent by default

### Windows

* Custom window control buttons with system integration
* Uses `WindowControlArea` for proper window management
* Control buttons have hover and active states
* Fixed button width of 34px each
* Left padding is 12px

### Linux

* Custom window control buttons with manual event handling
* Supports custom close window callback via `on_close_window()`
* Double-click to maximize/restore window
* Right-click shows window context menu
* Window dragging supported in title bar area

## API Reference

### TitleBar

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

### Constants

| Constant                 | Value                           | Description               |
| ------------------------ | ------------------------------- | ------------------------- |
| `TITLE_BAR_HEIGHT`       | `34px`                          | Standard title bar height |
| `TITLE_BAR_LEFT_PADDING` | `80px` (macOS), `12px` (others) | Left padding for content  |

## Examples

### Application Title Bar

```rust
use gpui_component::{TitleBar, button::Button, menu::AppMenuBar};

struct AppTitleBar {
    app_menu_bar: Entity<AppMenuBar>,
}

impl Render for AppTitleBar {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        TitleBar::new()
            .child(
                div()
                    .flex()
                    .items_center()
                    .child(self.app_menu_bar.clone())
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_end()
                    .gap_2()
                    .child(
                        Button::new("settings")
                            .ghost()
                            .icon(IconName::Settings)
                    )
                    .child(
                        Button::new("help")
                            .ghost()
                            .icon(IconName::HelpCircle)
                    )
            )
    }
}
```

### Title Bar with Breadcrumbs

```rust
TitleBar::new()
    .child(
        div()
            .flex()
            .items_center()
            .gap_2()
            .child("Home")
            .child(IconName::ChevronRight)
            .child("Documents")
            .child(IconName::ChevronRight)
            .child("Project")
    )
    .child(
        div()
            .flex()
            .items_center()
            .gap_1()
            .child(Button::new("search").icon(IconName::Search).ghost())
            .child(Button::new("more").icon(IconName::MoreHorizontal).ghost())
    )
```

### Custom Themed Title Bar

```rust
TitleBar::new()
    .h(px(40.)) // Custom height
    .bg(cx.theme().accent)
    .border_b_2()
    .border_color(cx.theme().accent_border)
    .child(
        div()
            .flex()
            .items_center()
            .text_color(cx.theme().accent_foreground)
            .font_weight_semibold()
            .child("Custom Theme App")
    )
```

### Title Bar with Status

```rust
TitleBar::new()
    .child(
        div()
            .flex()
            .items_center()
            .gap_3()
            .child("My Editor")
            .child(
                div()
                    .text_xs()
                    .text_color(cx.theme().muted_foreground)
                    .child("● Unsaved changes")
            )
    )
    .child(
        div()
            .flex()
            .items_center()
            .gap_2()
            .child(
                div()
                    .text_xs()
                    .text_color(cx.theme().muted_foreground)
                    .child("Line 42, Col 12")
            )
            .child(
                Button::new("sync")
                    .small()
                    .ghost()
                    .icon(IconName::RotateCcw)
                    .tooltip("Sync changes")
            )
    )
```

### Minimal Title Bar

```rust
TitleBar::new()
    .child(
        div()
            .text_center()
            .flex_1()
            .child("Document.txt")
    )
```

### Title Bar with Search

```rust
TitleBar::new()
    .child(
        div()
            .flex()
            .items_center()
            .gap_3()
            .child("File Explorer")
            .child(
                Input::new("search")
                    .placeholder("Search files...")
                    .w(px(200.))
                    .small()
            )
    )
```

## Notes

* The title bar automatically handles platform-specific styling and behavior
* Window controls are only rendered on Windows and Linux platforms
* The component integrates with GPUI's window management system
* Custom styling should consider platform conventions
* Window dragging is handled automatically in appropriate areas

---

---
url: /gpui-component/docs/components/toggle.md
description: A button-style toggle component for binary on/off or selected states.
---

# Toggle

A button-style toggle component that represents on/off or selected states. Unlike a traditional switch, toggles appear as buttons that can be pressed in or out. They're perfect for toolbar buttons, filter options, or any binary choice that benefits from a button-like appearance.

## Import

```rust
use gpui_component::button::{Toggle, ToggleGroup};
```

## Usage

### Basic Toggle

```rust
Toggle::new("toggle1").
    .label("Toggle me")
    .checked(false)
    .on_click(|checked, _, _| {
        println!("Toggle is now: {}", checked);
    })
```

Here, we can use `on_click` to handle toggle state changes. The callback receives the **new checked state** as a `bool`.

### Icon Toggle

```rust
use gpui_component::IconName;

Toggle::new("toggle2")
    .icon(IconName::Eye)
    .checked(true)
    .on_click(|checked, _, _| {
        println!("Visibility: {}", if *checked { "shown" } else { "hidden" });
    })
```

### Controlled Toggle

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

### Toggle Variants

```rust
// Ghost toggle (default)
Toggle::new("ghost-toggle")
    .ghost()
    .label("Ghost")

// Outline toggle
Toggle::new("outline-toggle")
    .outline()
    .label("Outline")
```

### Different Sizes

```rust
// Extra small
Toggle::new("xs-toggle")
    .icon(IconName::Star)
    .xsmall()

// Small
Toggle::new("small-toggle")
    .label("Small")
    .small()

// Medium (default)
Toggle::new("medium-toggle")
    .label("Medium")


// Large
Toggle::new("large-toggle")
    .label("Large")
    .large()
```

### Disabled State

```rust
// Disabled unchecked
Toggle::new("disabled-toggle")
    .label("Disabled")
    .disabled(true)
    .checked(false)

// Disabled checked
Toggle::new("disabled-checked-toggle")
    .label("Selected (Disabled)")
    .disabled(true)
    .checked(true)
```

## Toggle vs Switch

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

```rust
ToggleGroup::new("filter-group")
    .child(Toggle::new(0).icon(IconName::Bell))
    .child(Toggle::new(1).icon(IconName::Bot))
    .child(Toggle::new(2).icon(IconName::Inbox))
    .child(Toggle::new(3).label("Other"))
    .on_click(|checkeds, _, _| {
        println!("Selected toggles: {:?}", checkeds);
    })
```

The `on_click` callback receives a `Vec<bool>` representing the **new checked state** of each toggle in the group.

### Toggle Group with Controlled State

```rust
struct FilterView {
    notifications: bool,
    bots: bool,
    inbox: bool,
    other: bool,
}

impl Render for FilterView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        ToggleGroup::new("filters")
            .child(Toggle::new(0).icon(IconName::Bell).checked(self.notifications))
            .child(Toggle::new(1).icon(IconName::Bot).checked(self.bots))
            .child(Toggle::new(2).icon(IconName::Inbox).checked(self.inbox))
            .child(Toggle::new(3).label("Other").checked(self.other))
            .on_click(cx.listener(|view, checkeds, _, cx| {
                view.notifications = checkeds[0];
                view.bots = checkeds[1];
                view.inbox = checkeds[2];
                view.other = checkeds[3];
                cx.notify();
            }))
    }
}
```

### Toggle Group Variants and Sizes

```rust
// Outline variant, small size
ToggleGroup::new("compact-filters")
    .outline()
    .small()
    .child(Toggle::new(0).icon(IconName::Filter))
    .child(Toggle::new(1).icon(IconName::Sort))
    .child(Toggle::new(2).icon(IconName::Search))

// Ghost variant (default), extra small
ToggleGroup::new("mini-toolbar")
    .xsmall()
    .child(Toggle::new(0).icon(IconName::Bold))
    .child(Toggle::new(1).icon(IconName::Italic))
    .child(Toggle::new(2).icon(IconName::Underline))
```

## Event Handling

### Individual Toggle Events

```rust
Toggle::new("subscribe-toggle")
    .label("Subscribe")
    .on_click(|checked, window, cx| {
        if *checked {
            // Handle subscription logic
            println!("Subscribed!");
        } else {
            // Handle unsubscription logic
            println!("Unsubscribed!");
        }
    })
```

## Examples

### Toolbar with Toggle Buttons

```rust
struct EditorToolbar {
    bold: bool,
    italic: bool,
    underline: bool,
    strikethrough: bool,
}

h_flex()
    .gap_1()
    .p_2()
    .bg(cx.theme().background)
    .border_1()
    .border_color(cx.theme().border)
    .child(
        ToggleGroup::new("formatting")
            .small()
            .child(Toggle::new(0).icon(IconName::Bold).checked(self.bold))
            .child(Toggle::new(1).icon(IconName::Italic).checked(self.italic))
            .child(Toggle::new(2).icon(IconName::Underline).checked(self.underline))
            .child(Toggle::new(3).icon(IconName::Strikethrough).checked(self.strikethrough))
            .on_click(cx.listener(|view, states, _, cx| {
                view.bold = states[0];
                view.italic = states[1];
                view.underline = states[2];
                view.strikethrough = states[3];
                cx.notify();
            }))
    )
```

### Filter Interface

```rust
struct FilterPanel {
    show_completed: bool,
    show_pending: bool,
    show_cancelled: bool,
    show_urgent: bool,
}

v_flex()
    .gap_3()
    .p_4()
    .child(Label::new("Filter by status"))
    .child(
        ToggleGroup::new("status-filters")
            .outline()
            .child(Toggle::new(0).label("Completed").checked(self.show_completed))
            .child(Toggle::new(1).label("Pending").checked(self.show_pending))
            .child(Toggle::new(2).label("Cancelled").checked(self.show_cancelled))
            .on_click(cx.listener(|view, states, _, cx| {
                view.show_completed = states[0];
                view.show_pending = states[1];
                view.show_cancelled = states[2];
                cx.notify();
            }))
    )
    .child(
        Toggle::new("urgent-filter")
            .label("Show urgent only")
            .checked(self.show_urgent)
            .on_click(cx.listener(|view, checked, _, cx| {
                view.show_urgent = *checked;
                cx.notify();
            }))
    )
```

### Settings with Individual Toggles

```rust
struct NotificationSettings {
    email_notifications: bool,
    push_notifications: bool,
    marketing_emails: bool,
}

v_flex()
    .gap_4()
    .child(
        h_flex()
            .items_center()
            .justify_between()
            .child(
                v_flex()
                    .child(Label::new("Email notifications"))
                    .child(
                        Label::new("Receive notifications via email")
                            .text_color(cx.theme().muted_foreground)
                            .text_sm()
                    )
            )
            .child(
                Toggle::new("email-notifications")
                    .icon(IconName::Mail)
                    .checked(self.email_notifications)
                    .on_click(cx.listener(|view, checked, _, cx| {
                        view.email_notifications = *checked;
                        cx.notify();
                    }))
            )
    )
    .child(
        h_flex()
            .items_center()
            .justify_between()
            .child(Label::new("Push notifications"))
            .child(
                Toggle::new("push-notifications")
                    .icon(IconName::Bell)
                    .checked(self.push_notifications)
                    .on_click(cx.listener(|view, checked, _, cx| {
                        view.push_notifications = *checked;
                        cx.notify();
                    }))
            )
    )
```

### Multi-select Options

```rust
struct SelectionView {
    selected_categories: Vec<bool>,
}

impl SelectionView {
    fn categories() -> Vec<&'static str> {
        vec!["Technology", "Design", "Business", "Science", "Art"]
    }
}

v_flex()
    .gap_3()
    .child(Label::new("Select categories of interest"))
    .child(
        ToggleGroup::new("categories")
            .children(
                Self::categories()
                    .into_iter()
                    .enumerate()
                    .map(|(i, category)| {
                        Toggle::new(i)
                            .label(category)
                            .checked(self.selected_categories.get(i).copied().unwrap_or(false))
                    })
            )
            .on_click(cx.listener(|view, states, _, cx| {
                view.selected_categories = states.clone();
                cx.notify();
            }))
    )
```

## Best Practices

1. **Use meaningful labels**: Choose clear, descriptive text for toggle labels
2. **Group related options**: Use ToggleGroup for logically related binary choices
3. **Provide visual feedback**: The checked state should be clearly distinguishable
4. **Consider context**: Use toggles for options that feel like "selections" rather than "settings"
5. **Maintain state consistency**: Ensure toggle state reflects the actual application state
6. **Accessible labels**: Provide tooltips or ARIA labels for icon-only toggles

---

---
url: /gpui-component/docs/components/tooltip.md
description: >-
  Display helpful information on hover or focus, with support for keyboard
  shortcuts and custom content.
---

# Tooltip

A versatile tooltip component that displays helpful information when hovering over or focusing on elements. Supports text content, custom elements, keyboard shortcuts, different trigger methods, and positioning options.

## Import

```rust
use gpui_component::tooltip::Tooltip;
```

## Usage

### Basic Tooltip with Text

```rust
// Simple text tooltip
div()
    .child("Hover me")
    .id("basic-tooltip")
    .tooltip(|window, cx| {
        Tooltip::new("This is a helpful tooltip").build(window, cx)
    })
```

### Button with Tooltip

```rust
Button::new("save-btn")
    .label("Save")
    .tooltip("Save the current document")
```

### Tooltip with Action/Keybinding

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

### Custom Element Tooltip

```rust
div()
    .child("Hover for rich content")
    .id("rich-tooltip")
    .tooltip(|window, cx| {
        Tooltip::element(|_, cx| {
            h_flex()
                .gap_x_1()
                .child(IconName::Info)
                .child(
                    div()
                        .child("Muted Text")
                        .text_color(cx.theme().muted_foreground)
                )
                .child(
                    div()
                        .child("Danger Text")
                        .text_color(cx.theme().danger)
                )
                .child(IconName::ArrowUp)
        })
        .build(window, cx)
    })
```

### Tooltip with Manual Keybinding

```rust
div()
    .child("Custom keybinding")
    .id("custom-kb")
    .tooltip(|window, cx| {
        Tooltip::new("Delete item")
            .key_binding(Some(Kbd::new("Delete")))
            .build(window, cx)
    })
```

## Advanced Usage

### Components with Built-in Tooltip Support

Many components have built-in tooltip methods:

```rust
// Button
Button::new("btn")
    .label("Click me")
    .tooltip("This button performs an action")

// Switch
Switch::new("toggle")
    .label("Enable notifications")
    .tooltip("Toggle push notifications on/off")

// Checkbox
Checkbox::new("check")
    .label("Remember me")
    .tooltip("Keep me logged in for 30 days")

// Radio
Radio::new("option")
    .label("Option 1")
    .tooltip("Select this option to enable feature X")
```

### Complex Tooltip Content

```rust
div()
    .child("Hover for details")
    .id("complex-tooltip")
    .tooltip(|window, cx| {
        Tooltip::element(|_, cx| {
            v_flex()
                .gap_2()
                .child(
                    h_flex()
                        .gap_1()
                        .child(IconName::User)
                        .child("User Information")
                        .text_sm()
                        .font_semibold()
                )
                .child(
                    div()
                        .child("Last login: 2 hours ago")
                        .text_xs()
                        .text_color(cx.theme().muted_foreground)
                )
                .child(
                    div()
                        .child("Status: Active")
                        .text_xs()
                        .text_color(cx.theme().success)
                )
        })
        .build(window, cx)
    })
```

### Tooltip in Form Elements

```rust
v_flex()
    .gap_4()
    .child(
        Input::new("email")
            .placeholder("Enter your email")
            .tooltip("We'll never share your email address")
    )
    .child(
        Input::new("password")
            .input_type(InputType::Password)
            .placeholder("Password")
            .tooltip("Must be at least 8 characters with special characters")
    )
```

## API Reference

### Tooltip

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

### Tooltip Styling

The tooltip automatically applies theme-appropriate styling:

* Background: `theme.popover`
* Text color: `theme.popover_foreground`
* Border: `theme.border`
* Shadow: Medium drop shadow
* Border radius: 6px
* Font: System UI font

You can apply additional styling using the `Styled` trait:

```rust
Tooltip::new("Custom styled tooltip")
    .bg(cx.theme().accent)
    .text_color(cx.theme().accent_foreground)
    .build(window, cx)
```

## Examples

### Toolbar with Tooltips

```rust
h_flex()
    .gap_1()
    .child(
        Button::new("new")
            .icon(IconName::Plus)
            .tooltip_with_action("Create new file", &NewFile, Some("Editor"))
    )
    .child(
        Button::new("open")
            .icon(IconName::FolderOpen)
            .tooltip_with_action("Open file", &OpenFile, Some("Editor"))
    )
    .child(
        Button::new("save")
            .icon(IconName::Save)
            .tooltip_with_action("Save file", &SaveFile, Some("Editor"))
    )
```

### Status Indicators with Tooltips

```rust
h_flex()
    .gap_2()
    .child(
        div()
            .size_3()
            .rounded_full()
            .bg(cx.theme().success)
            .tooltip(|window, cx| {
                Tooltip::new("Connected to server").build(window, cx)
            })
    )
    .child(
        div()
            .size_3()
            .rounded_full()
            .bg(cx.theme().warning)
            .tooltip(|window, cx| {
                Tooltip::new("Limited connectivity").build(window, cx)
            })
    )
```

### Interactive Elements with Rich Tooltips

```rust
v_flex()
    .gap_3()
    .child(
        div()
            .p_2()
            .border_1()
            .border_color(cx.theme().border)
            .rounded(cx.theme().radius)
            .child("File: document.txt")
            .id("file-item")
            .tooltip(|window, cx| {
                Tooltip::element(|_, cx| {
                    v_flex()
                        .gap_1()
                        .child(
                            h_flex()
                                .gap_2()
                                .child(IconName::File)
                                .child("document.txt")
                                .text_sm()
                                .font_medium()
                        )
                        .child(
                            div()
                                .child("Size: 2.4 KB")
                                .text_xs()
                                .text_color(cx.theme().muted_foreground)
                        )
                        .child(
                            div()
                                .child("Modified: 2 hours ago")
                                .text_xs()
                                .text_color(cx.theme().muted_foreground)
                        )
                        .child(
                            h_flex()
                                .gap_1()
                                .child(Kbd::new("Enter"))
                                .child("to open")
                                .text_xs()
                                .text_color(cx.theme().muted_foreground)
                        )
                })
                .build(window, cx)
            })
    )
```

### Form Validation with Tooltips

```rust
struct FormView {
    email_error: Option<String>,
    password_error: Option<String>,
}

v_flex()
    .gap_4()
    .child(
        Input::new("email")
            .placeholder("Email address")
            .when_some(self.email_error.clone(), |this, error| {
                this.tooltip(move |window, cx| {
                    Tooltip::element(|_, cx| {
                        h_flex()
                            .gap_1()
                            .child(IconName::AlertCircle)
                            .child(error.clone())
                            .text_color(cx.theme().destructive)
                    })
                    .build(window, cx)
                })
            })
    )
```

## Best Practices

### Content Guidelines

* **Be concise**: Keep tooltip text short and to the point
* **Be helpful**: Provide additional context, not redundant information
* **Use proper tone**: Match your application's voice and tone
* **Avoid critical info**: Don't put essential information only in tooltips

### Usage Guidelines

* **Progressive disclosure**: Use tooltips for additional context, not primary information
* **Consistency**: Use consistent tooltip patterns throughout your application
* **Performance**: Avoid complex content in frequently triggered tooltips
* **Testing**: Test tooltips with both mouse and keyboard interaction

### Examples of Good Tooltip Content

```rust
// Good: Provides helpful context
Button::new("delete")
    .icon(IconName::Trash)
    .tooltip("Delete this item permanently")

// Good: Explains abbreviation
div()
    .child("CPU: 45%")
    .tooltip("Central Processing Unit usage")

// Good: Describes action with keybinding
Button::new("undo")
    .icon(IconName::Undo)
    .tooltip_with_action("Undo last action", &Undo, Some("Editor"))
```

### Examples to Avoid

```rust
// Avoid: Redundant information
Button::new("save")
    .label("Save")
    .tooltip("Save") // Doesn't add value

// Avoid: Critical information
Button::new("delete")
    .tooltip("This will permanently delete all your files") // Too important for tooltip only
```

---

---
url: /gpui-component/docs/components/tree.md
description: >-
  A hierarchical tree view component for displaying and navigating
  tree-structured data.
---

# Tree

A versatile tree component for displaying hierarchical data with expand/collapse functionality, keyboard navigation, and custom item rendering. Perfect for file explorers, navigation menus, or any nested data structure.

## Import

```rust
use gpui_component::tree::{tree, TreeState, TreeItem, TreeEntry};
```

## Usage

### Basic Tree

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

### File Tree with Icons

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

### Dynamic Tree Loading

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

### Tree with Selection Handling

```rust
struct MyTreeView {
    tree_state: Entity<TreeState>,
    selected_item: Option<TreeItem>,
}

impl MyTreeView {
    fn handle_selection(&mut self, item: TreeItem, cx: &mut Context<Self>) {
        self.selected_item = Some(item.clone());
        println!("Selected: {} ({})", item.label, item.id);
        cx.notify();
    }
}

// In render method
tree(&self.tree_state, {
    let view = cx.entity();
    move |ix, entry, selected, window, cx| {
        view.update(cx, |this, cx| {
            ListItem::new(ix)
                .selected(selected)
                .child(entry.item().label.clone())
                .on_click(cx.listener({
                    let item = entry.item().clone();
                    move |this, _, _, cx| {
                        this.handle_selection(item.clone(), cx);
                    }
                }))
        })
    }
})
```

### Disabled Items

```rust
TreeItem::new("protected", "Protected Folder")
    .disabled(true)
    .child(TreeItem::new("secret.txt", "secret.txt"))
```

### Programmatic Tree Control

```rust
// Get current selection
if let Some(entry) = tree_state.read(cx).selected_entry() {
    println!("Current selection: {}", entry.item().label);
}

// Set selection programmatically
tree_state.update(cx, |state, cx| {
    state.set_selected_index(Some(2), cx); // Select third item
});

// Scroll to specific item
tree_state.update(cx, |state, _| {
    state.scroll_to_item(5, gpui::ScrollStrategy::Center);
});

// Clear selection
tree_state.update(cx, |state, cx| {
    state.set_selected_index(None, cx);
});
```

## API Reference

### TreeState

| Method                         | Description                  |
| ------------------------------ | ---------------------------- |
| `new(cx)`                      | Create a new tree state      |
| `items(items)`                 | Set initial tree items       |
| `set_items(items, cx)`         | Update tree items and notify |
| `selected_index()`             | Get currently selected index |
| `set_selected_index(ix, cx)`   | Set selected index           |
| `selected_entry()`             | Get currently selected entry |
| `scroll_to_item(ix, strategy)` | Scroll to specific item      |

### TreeItem

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

### TreeEntry

| Method          | Description                 |
| --------------- | --------------------------- |
| `item()`        | Get the source TreeItem     |
| `depth()`       | Get item depth in tree      |
| `is_folder()`   | Check if entry has children |
| `is_expanded()` | Check if entry is expanded  |
| `is_disabled()` | Check if entry is disabled  |

### tree() Function

| Parameter     | Description                           |
| ------------- | ------------------------------------- |
| `state`       | `Entity<TreeState>` for managing tree |
| `render_item` | Closure for rendering each item       |

#### Render Item Closure

```rust
Fn(usize, &TreeEntry, bool, &mut Window, &mut App) -> ListItem
```

* `usize`: Item index in flattened tree
* `&TreeEntry`: Tree entry with item and metadata
* `bool`: Whether item is currently selected
* `&mut Window`: Current window context
* `&mut App`: Application context
* Returns: `ListItem` for rendering

## Examples

### Lazy Loading Tree

```rust
struct LazyTreeView {
    tree_state: Entity<TreeState>,
    loaded_paths: HashSet<String>,
}

impl LazyTreeView {
    fn load_children(&mut self, item_id: &str, cx: &mut Context<Self>) {
        if self.loaded_paths.contains(item_id) {
            return;
        }

        let path = PathBuf::from(item_id);
        if path.is_dir() {
            let tree_state = self.tree_state.clone();
            let item_id = item_id.to_string();

            cx.spawn(async move |cx| {
                let children = load_directory_children(&path).await;
                tree_state.update(cx, |state, cx| {
                    // Update specific item with loaded children
                    state.update_item_children(&item_id, children, cx);
                })
            }).detach();

            self.loaded_paths.insert(item_id.to_string());
        }
    }
}
```

### Search and Filter

```rust
struct SearchableTree {
    tree_state: Entity<TreeState>,
    original_items: Vec<TreeItem>,
    search_query: String,
}

impl SearchableTree {
    fn filter_tree(&mut self, query: &str, cx: &mut Context<Self>) {
        self.search_query = query.to_string();

        let filtered_items = if query.is_empty() {
            self.original_items.clone()
        } else {
            filter_tree_items(&self.original_items, query)
        };

        self.tree_state.update(cx, |state, cx| {
            state.set_items(filtered_items, cx);
        });
    }
}

fn filter_tree_items(items: &[TreeItem], query: &str) -> Vec<TreeItem> {
    items.iter()
        .filter_map(|item| {
            if item.label.to_lowercase().contains(&query.to_lowercase()) {
                Some(item.clone().expanded(true)) // Auto-expand matches
            } else {
                // Check if any children match
                let filtered_children = filter_tree_items(&item.children, query);
                if !filtered_children.is_empty() {
                    Some(item.clone()
                        .children(filtered_children)
                        .expanded(true))
                } else {
                    None
                }
            }
        })
        .collect()
}
```

### Multi-Select Tree

```rust
struct MultiSelectTree {
    tree_state: Entity<TreeState>,
    selected_items: HashSet<String>,
}

impl MultiSelectTree {
    fn toggle_selection(&mut self, item_id: &str, cx: &mut Context<Self>) {
        if self.selected_items.contains(item_id) {
            self.selected_items.remove(item_id);
        } else {
            self.selected_items.insert(item_id.to_string());
        }
        cx.notify();
    }

    fn is_selected(&self, item_id: &str) -> bool {
        self.selected_items.contains(item_id)
    }
}

// In render method
tree(&self.tree_state, {
    let view = cx.entity();
    move |ix, entry, _selected, window, cx| {
        view.update(cx, |this, cx| {
            let item = entry.item();
            let is_multi_selected = this.is_selected(&item.id);

            ListItem::new(ix)
                .selected(is_multi_selected)
                .child(
                    h_flex()
                        .gap_2()
                        .child(checkbox().checked(is_multi_selected))
                        .child(item.label.clone())
                )
                .on_click(cx.listener({
                    let item_id = item.id.clone();
                    move |this, _, _, cx| {
                        this.toggle_selection(&item_id, cx);
                    }
                }))
        })
    }
})
```

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

```rust
// Custom keyboard handling
tree(&tree_state)
    .key_context("MyTree")
    .on_action(cx.listener(|this, action: &MyCustomAction, _, cx| {
        // Handle custom actions
    }))
```

---

---
url: /gpui-component/contributors.md
---


---

---
url: /gpui-component/docs/components/virtual-list.md
description: >-
  High-performance virtualized list component for rendering large datasets with
  variable item sizes.
---

# VirtualList

VirtualList is a high-performance component designed for efficiently rendering large datasets by only rendering visible items. Unlike uniform lists, VirtualList supports variable item sizes, making it perfect for complex layouts like tables with different row heights or dynamic content.

## Import

```rust
use gpui_component::{
    v_virtual_list, h_virtual_list, VirtualListScrollHandle,
    scroll::{Scrollbar, ScrollbarState, ScrollbarAxis},
};
use std::rc::Rc;
use gpui::{px, size, ScrollStrategy, Size, Pixels};
```

## Usage

### Basic Vertical Virtual List

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

### Horizontal Virtual List

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

### Variable Item Sizes

VirtualList excels at handling items with different sizes:

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

### Table-like Layout with Multiple Columns

VirtualList can render complex layouts like tables:

```rust
v_virtual_list(
    cx.entity().clone(),
    "table-list",
    item_sizes.clone(),
    |view, visible_range, _, cx| {
        visible_range
            .map(|row_ix| {
                h_flex()
                    .w_full()
                    .h(px(40.))
                    .border_b_1()
                    .border_color(cx.theme().border)
                    .children(
                        // Multiple columns per row
                        (0..5).map(|col_ix| {
                            div()
                                .flex_1()
                                .h_full()
                                .px_3()
                                .flex()
                                .items_center()
                                .child(format!("R{}C{}", row_ix, col_ix))
                        })
                    )
            })
            .collect()
    },
)
```

## Scroll Handling

### Basic Scroll Control

```rust
pub struct ScrollableList {
    scroll_handle: VirtualListScrollHandle,
    scroll_state: ScrollbarState,
}

impl Render for ScrollableList {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .relative()
            .size_full()
            .child(
                v_virtual_list(/* ... */)
                    .track_scroll(&self.scroll_handle)
                    .p_4()
                    .border_1()
                    .border_color(cx.theme().border)
            )
            .child(
                // Add scrollbars
                div()
                    .absolute()
                    .top_0()
                    .left_0()
                    .right_0()
                    .bottom_0()
                    .child(
                        Scrollbar::both(&self.scroll_state, &self.scroll_handle)
                            .axis(ScrollbarAxis::Vertical)
                    )
            )
    }
}
```

### Programmatic Scrolling

```rust
impl ScrollableList {
    // Scroll to specific item
    fn scroll_to_item(&self, index: usize) {
        self.scroll_handle.scroll_to_item(index, ScrollStrategy::Top);
    }

    // Center item in view
    fn center_item(&self, index: usize) {
        self.scroll_handle.scroll_to_item(index, ScrollStrategy::Center);
    }

    // Scroll to bottom
    fn scroll_to_bottom(&self) {
        self.scroll_handle.scroll_to_bottom();
    }

    // Get current scroll position
    fn get_scroll_offset(&self) -> Point<Pixels> {
        self.scroll_handle.offset()
    }

    // Set scroll position manually
    fn set_scroll_position(&self, offset: Point<Pixels>) {
        self.scroll_handle.set_offset(offset);
    }
}
```

### Both Axis Scrolling

For content that scrolls in both directions:

```rust
v_virtual_list(
    cx.entity().clone(),
    "both-axis",
    item_sizes.clone(),
    |view, visible_range, _, cx| {
        visible_range
            .map(|ix| {
                // Wide content that requires horizontal scrolling
                h_flex()
                    .gap_2()
                    .children((0..20).map(|col| {
                        div()
                            .min_w(px(100.))
                            .h(px(30.))
                            .bg(cx.theme().secondary)
                            .child(format!("R{}C{}", ix, col))
                    }))
            })
            .collect()
    },
)
.track_scroll(&scroll_handle)
.child(
    Scrollbar::both(&scroll_state, &scroll_handle)
        .axis(ScrollbarAxis::Both)
)
```

## Performance Optimization

### Efficient Item Rendering

Only visible items are rendered, making VirtualList highly performant:

```rust
// The render function is only called for visible items
v_virtual_list(
    cx.entity().clone(),
    "efficient-list",
    item_sizes.clone(),
    |view, visible_range, _, cx| {
        // visible_range contains only the items currently visible
        // This typically contains 10-20 items, not all 10,000
        println!("Rendering {} items out of {}",
                visible_range.len(),
                view.total_items);

        visible_range
            .map(|ix| {
                // Complex rendering logic here
                // Only executed for visible items
                expensive_item_renderer(ix, cx)
            })
            .collect()
    },
)
```

### Memory Management

VirtualList automatically manages memory by:

* Only rendering visible items
* Reusing rendered elements when scrolling
* Calculating precise visible ranges

```rust
// Large dataset - only visible items use memory
let large_dataset = (0..1_000_000).map(|i| format!("Item {}", i)).collect();

// Memory usage remains constant regardless of dataset size
v_virtual_list(/* render only visible items */)
```

### Variable Heights with Caching

For dynamic content with calculated heights:

```rust
struct DynamicItem {
    content: String,
    calculated_height: Option<Pixels>,
}

impl MyView {
    fn calculate_item_size(&mut self, ix: usize) -> Size<Pixels> {
        if let Some(height) = self.items[ix].calculated_height {
            return size(px(300.), height);
        }

        // Calculate height based on content
        let content_lines = self.items[ix].content.lines().count();
        let height = px(20. + content_lines as f32 * 16.);

        // Cache the calculated height
        self.items[ix].calculated_height = Some(height);

        size(px(300.), height)
    }
}
```

## Examples

### File Explorer with Virtual Scrolling

```rust
pub struct FileExplorer {
    files: Vec<FileEntry>,
    item_sizes: Rc<Vec<Size<Pixels>>>,
    scroll_handle: VirtualListScrollHandle,
    selected_index: Option<usize>,
}

impl FileExplorer {
    fn calculate_item_heights(&mut self) {
        let sizes = self.files.iter().map(|file| {
            // Different heights for different file types
            let height = match file.file_type {
                FileType::Directory => px(40.),
                FileType::Image => px(60.),  // Larger for thumbnails
                FileType::Document => px(35.),
                _ => px(30.),
            };
            size(px(400.), height)
        }).collect();

        self.item_sizes = Rc::new(sizes);
    }
}

impl Render for FileExplorer {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_virtual_list(
            cx.entity().clone(),
            "file-list",
            self.item_sizes.clone(),
            |view, visible_range, _, cx| {
                visible_range
                    .map(|ix| {
                        let file = &view.files[ix];
                        let is_selected = view.selected_index == Some(ix);

                        div()
                            .w_full()
                            .h(view.item_sizes[ix].height)
                            .px_3()
                            .py_1()
                            .flex()
                            .items_center()
                            .gap_2()
                            .bg(if is_selected {
                                cx.theme().accent
                            } else {
                                Color::transparent()
                            })
                            .hover(|style| style.bg(cx.theme().secondary_hover))
                            .child(file_icon(&file.file_type))
                            .child(file.name.clone())
                            .child(
                                div()
                                    .flex_1()
                                    .text_right()
                                    .text_xs()
                                    .text_color(cx.theme().muted_foreground)
                                    .child(format_file_size(file.size))
                            )
                            .on_click(cx.listener(move |view, _, _, cx| {
                                view.selected_index = Some(ix);
                                cx.notify();
                            }))
                    })
                    .collect()
            },
        )
        .track_scroll(&self.scroll_handle)
    }
}
```

### Chat Messages with Auto-scroll

```rust
pub struct ChatWindow {
    messages: Vec<ChatMessage>,
    scroll_handle: VirtualListScrollHandle,
    auto_scroll: bool,
}

impl ChatWindow {
    fn add_message(&mut self, message: ChatMessage, cx: &mut Context<Self>) {
        self.messages.push(message);

        // Recalculate item sizes
        self.update_item_sizes();

        if self.auto_scroll {
            // Scroll to bottom for new messages
            self.scroll_handle.scroll_to_bottom();
        }

        cx.notify();
    }

    fn update_item_sizes(&mut self) {
        let sizes = self.messages.iter().map(|msg| {
            // Calculate height based on message content
            let lines = msg.content.lines().count().max(1);
            let height = px(40. + (lines.saturating_sub(1)) as f32 * 16.);
            size(px(350.), height)
        }).collect();

        self.item_sizes = Rc::new(sizes);
    }
}

impl Render for ChatWindow {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .child(
                v_virtual_list(
                    cx.entity().clone(),
                    "chat-messages",
                    self.item_sizes.clone(),
                    |view, visible_range, _, cx| {
                        visible_range
                            .map(|ix| {
                                let msg = &view.messages[ix];

                                div()
                                    .w_full()
                                    .px_4()
                                    .py_2()
                                    .child(
                                        v_flex()
                                            .gap_1()
                                            .child(
                                                h_flex()
                                                    .justify_between()
                                                    .child(
                                                        div()
                                                            .text_sm()
                                                            .font_weight(FontWeight::SEMIBOLD)
                                                            .child(msg.author.clone())
                                                    )
                                                    .child(
                                                        div()
                                                            .text_xs()
                                                            .text_color(cx.theme().muted_foreground)
                                                            .child(format_timestamp(msg.timestamp))
                                                    )
                                            )
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .child(msg.content.clone())
                                            )
                                    )
                            })
                            .collect()
                    },
                )
                .track_scroll(&self.scroll_handle)
                .flex_1()
            )
            .child(
                // Chat input at bottom
                div()
                    .w_full()
                    .h(px(60.))
                    .border_t_1()
                    .border_color(cx.theme().border)
                    .child("Chat input here...")
            )
    }
}
```

### Data Grid with Fixed Headers

```rust
pub struct DataGrid {
    headers: Vec<String>,
    data: Vec<Vec<String>>,
    column_widths: Vec<Pixels>,
    scroll_handle: VirtualListScrollHandle,
}

impl Render for DataGrid {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .child(
                // Fixed header
                h_flex()
                    .w_full()
                    .h(px(40.))
                    .bg(cx.theme().secondary)
                    .border_b_1()
                    .border_color(cx.theme().border)
                    .children(
                        self.headers.iter().zip(&self.column_widths).map(|(header, &width)| {
                            div()
                                .w(width)
                                .h_full()
                                .px_3()
                                .flex()
                                .items_center()
                                .font_weight(FontWeight::SEMIBOLD)
                                .child(header.clone())
                        })
                    )
            )
            .child(
                // Virtual list for data rows
                v_virtual_list(
                    cx.entity().clone(),
                    "data-rows",
                    Rc::new(vec![size(px(800.), px(32.)); self.data.len()]),
                    |view, visible_range, _, cx| {
                        visible_range
                            .map(|row_ix| {
                                h_flex()
                                    .w_full()
                                    .h(px(32.))
                                    .border_b_1()
                                    .border_color(cx.theme().border.opacity(0.5))
                                    .children(
                                        view.data[row_ix].iter().zip(&view.column_widths).map(|(cell, &width)| {
                                            div()
                                                .w(width)
                                                .h_full()
                                                .px_3()
                                                .flex()
                                                .items_center()
                                                .child(cell.clone())
                                        })
                                    )
                            })
                            .collect()
                    },
                )
                .track_scroll(&self.scroll_handle)
                .flex_1()
            )
    }
}
```

## Best Practices

1. **Item Sizing**: Pre-calculate item sizes when possible for best performance
2. **Memory Management**: Use VirtualList for any list with >50 items
3. **Scroll Performance**: Avoid heavy computations in render functions
4. **State Management**: Keep item state separate from rendering logic
5. **Error Handling**: Handle edge cases like empty lists gracefully
6. **Testing**: Test with various data sizes and scroll positions

## Performance Tips

1. **Pre-calculate Sizes**: Calculate item sizes upfront rather than during render
2. **Minimize Re-renders**: Use stable item keys and avoid recreating render functions
3. **Batch Updates**: Group multiple data changes together
4. **Efficient Rendering**: Keep item render functions lightweight
5. **Memory Monitoring**: Monitor memory usage with very large datasets

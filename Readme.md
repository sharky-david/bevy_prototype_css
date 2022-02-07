# Bevy CSS (Prototype)

A project that allows Bevy UI Node Styles to be defined with CSS strings, rather than the current verbose inline code
definitions.  Mozilla's excellent cssparser & selectors crates provide the core parsing functionality.

If you like this crate and would like to help make it better, please consider submitting a pull request.

## Current Features

### Supported components

Styling/definition in CSS is supported for the following components:

- `bevy::ui::Style`
- `bevy::ui::UiColor`

### Styling with `.css` stylesheets and class / id components

> Example: `bevy_ui_stylesheet.rs`/`assets/styles/bevy_ui.css` (`cargo run --example bevy_ui_stylesheet`)

The `CssPlugin` allows UI styles to be defined in a `.css` asset file (e.g. `assets/styles/ui.css`).  By loading this
asset and tagging your styled entities with a `CssTag`, the stylesheet styles will be applied to your nodes for you.
Only entities with a `CssTag` will be styled.

#### Example

`src/main.rs`:

    use bevy_prototype_css::{CssPlugin, CssStylesheet, CssTag};                 // Required imports

    fn main() {
        App::new()
            .add_plugins(DefaultPlugins)
            .add_plugin(CssPlugin)                                              // CSSPlugin does the hard work
            .add_startup_system(setup)
            .run()
    }

    fn setup(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
    ) {
        let sheet: Handle<CssStylesheet> = asset_server.load("styles/ui.css");  // Load the .css file
    
        commands
            .spawn_bundle(NodeBundle::default())
            .insert(CssTag::from("#container-1.fill-width"));                   // Tag the entity with your id/classes
            
    }

`assets/styles/ui.css`:

    #container-1 { height: 10em; color: blue; }
    .fill-width { width: 100%; }

**Caveat**: Selector matching is currently very rudimentary.  Ids and classes can be combined (e.g.
`#id.class-1.class-2`), but there is _currently_ no hierarchical matching (e.g. `#parent>.child` doesn't work).  There
is also no pseudo-class (e.g. `:hover`), pseudo-element (e.g. `::after`), nor attribute (e.g. `[attr=value]`) matching.

### Inline css in code

> Example: `bevy_ui_inline.rs` (`cargo run --example bevy_ui_inline`)

UI styles can be created inline in your code from a css string and the appropriate (static) context.  Use `CssStyle` to
define your style, then call `.to_style(css_context)` to get a `bevy::ui::Style` component, or `.to_ui_color()` to get
a `bevy::ui::UiColor` component.

`CssStyle` is **not** a component, just a container for `&str`.  You could create common `CssStyle` structs ahead of
time, then call `.to_style` on the same `CssStyle` multiple times.

`src/main.rs`:

    use bevy_prototype_css::{CssContext, CssStyle};                              // Required imports

    fn main() {
        App::new()
            .add_plugins(DefaultPlugins)
            .add_startup_system(setup)
            .run()
    }

    fn setup(mut commands: Commands) {
        let css_context = CssContext::default();                                 // CssContext is required!
    
        commands.spawn_bundle(NodeBundle {
            style: CssStyle("width: 100%; height: 10em;")                        // Define your styles without selectors
                .to_style(css_context),                                          // Call .to_style to get your Style
            color: CssStyle("color: red;").to_ui_color(),                        // Works for colors with .to_ui_color()
            ..Default::default(),
        });
    }

## Possible Future Features

- Proper & full testing
- Hierarchical selector matching (e.g. `#parent>.child`)
- Entity components as CSS tags (e.g. `Node.class { /* ... */ }` in your stylesheet)
- `@font-face` definitions for font asset loading
- Support for the following `ui::Node` component types
  - `text::TextStyle`
  - `ui::UiImage`
- `calc()` and other css functions
- Full set of CSS spec `<length>` dimensions
- `!important` keyword
- CSS wide keywords (`initial`, `inherit`, `unset`)

## Compatible Bevy Versions

| `bevy_prototype_css` version | Minimum `bevy` version |
|:----------------------------:|:----------------------:|
|             0.1              |          0.6           |

## References

### Bevy

- `ui::Style` doc: <https://docs.rs/bevy/latest/bevy/ui/struct.Style.html>

### CSS Spec + Documentation

- Developer reference (Mozilla): <https://developer.mozilla.org/en-US/docs/Web/CSS>
- Values & Units Spec: <https://drafts.csswg.org/css-values/>
- w3 Schools tutorials & reference: <https://www.w3schools.com/css>

### Property Names

`<Bevy Type>` -> `<css-property-name>`

> NB: Not all Bevy types have exactly the same name as their CSS properties.  This is to
> stay consistent with web CSS style names.

#### Display

- `Style::Display` -> `display`
- `Style::Direction` -> `direction`
- `Style::Size` -> `width`, `height`
- `Style::MinSize` -> `min-width`, `min-height`
- `Style::MaxSize` -> `max-width`, `max-height`
- `Style::Overflow` -> `overflow`

#### Position

- `Style::PositionType` -> `position`
- `Style::Position` -> `top`, `right`, `bottom`, `left`

#### Flex Box

- `Style::FlexDirection` -> `flex-direction`
- `Style::FlexWrap` -> `flex-wrap`
- `Style::FlexGrow` -> `flex-grow`
- `Style::FlexShrink` -> `flex-shrink`
- `Style::FlexBasis` -> `flex-basis`
- `Style::AspectRatio` -> `aspect-ratio`

#### Alignment

- `Style::AlignItems` -> `align-items`
- `Style::AlignSelf` -> `align-self`
- `Style::AlignContent` -> `align-content`
- `Style::JustifyContent` -> `justify-content`

#### Margins

- `Style::Margin` -> `margin`, `margin-top`, `margin-right`, `margin-bottom`, `margin-left`

#### Padding

- `Style::Padding` -> `padding`, `padding-top`,`padding-right`,`padding-bottom`,`padding-left`

#### Borders

- `Style::Border` -> `border-width`, `border-width-top`, `border-width-right`, `border-width-bottom`, `border-width-left`

#### Color

- `UiColor` -> `color`

### Accepted Values

#### Display

- display: `flex` | `none`
- direction: `ltr` | `rtl` | `inherit`
- width: `auto` | `<length>` | `<percentage>`
- height: `auto` | `<length>` | `<percentage>`
- min-width: `auto` | `<length>` | `<percentage>`
- min-height: `auto` | `<length>` | `<percentage>`
- max-width: `auto` | `<length>` | `<percentage>`
- max-height: `auto` | `<length>` | `<percentage>`
- overflow: `visible` | `hidden`

#### Position

- position: `relative` | `absolute`
- top: `auto` | `<length>` | `<percentage>`
- right: `auto` | `<length>` | `<percentage>`
- bottom: `auto` | `<length>` | `<percentage>`
- left: `auto` | `<length>` | `<percentage>`

#### Flex Box

- flex-direction: `row` | `row-reverse` | `column` | `column-reverse`
- flex-wrap: `nowrap` | `wrap` | `wrap-reverse`
- flex-grow: `<non-negative-number>`
- flex-shrink: `<non-negative-number>`
- flex-basis: `auto` | `<length>` | `<percentage>`
- aspect-ratio: `auto` | `<ratio>`

#### Alignment

- align-items: `stretch` | `center` | `flex-start` | `flex-end` | `baseline`
- align-self: `auto` | `stretch` | `center` | `flex-start` | `flex-end` | `baseline`
- align-content: `stretch` | `center` | `flex-start` | `flex-end` | `space-between` | `space-around`
- justify-content: `flex-start` | `flex-end` | `center` | `space-between` | `space-around` | `space-evenly`

#### Margins

- margin: [`auto` | `<length>` | `<percentage>`]{1,4} (See _Shorthand_ below)
- margin-top: `auto` | `<length>` | `<percentage>`
- margin-right: `auto` | `<length>` | `<percentage>`
- margin-bottom: `auto` | `<length>` | `<percentage>`
- margin-left: `auto` | `<length>` | `<percentage>`

#### Padding

- padding: [`auto` | `<length>` | `<percentage>`]{1,4} (See _Shorthand_ below)
- padding-top: `auto` | `<length>` | `<percentage>`
- padding-right: `auto` | `<length>` | `<percentage>`
- padding-bottom: `auto` | `<length>` | `<percentage>`
- padding-left: `auto` | `<length>` | `<percentage>`

#### Borders

- border-width: [`auto` | `<length>` | `<percentage>`]{1,4} (See _Shorthand_ below)
- border-width-top: `auto` | `<length>` | `<percentage>`
- border-width-right: `auto` | `<length>` | `<percentage>`
- border-width-bottom: `auto` | `<length>` | `<percentage>`
- border-width-left: `auto` | `<length>` | `<percentage>`

#### Color

- color: `none` | `transparent` | `<rgb()>` | `<rgba()>` | `<hsl()>` | `<hsla()>` | `<hex-color>` | `<named-color>`
(See _Colors_ below)

### Value Types

#### `<number>`

- [CSS Spec](https://drafts.csswg.org/css-values/#numbers)
- [Mozilla Web Docs](https://developer.mozilla.org/en-US/docs/Web/CSS/number)

#### `<non-negative-number>`

- Same as `<numer>`, except the value has to be `>= 0`

#### `<length>`

- [CSS Spec](https://drafts.csswg.org/css-values/#lengths)
- [Mozilla Web Docs](https://developer.mozilla.org/en-US/docs/Web/CSS/length)
- 96 DPI is assumed.  This means `1in` == `96px`, regardless of your actual dpi setting.  This is part of the CSS spec.
  - See also: <https://drafts.csswg.org/css-values/#reference-pixel>
- Not all dimensions in the CSS spec are accepted by this parser.
- The following dimensions are accepted:
  - Absolute: `px`, `cm`, `mm`, `Q`, `in`, `pc`, `pt`
  - Font Relative: `em`, `rem`, `ex`, `ch`
  - Viewport Relative: `vw`, `vh`, `vmin`, `vmax`

#### `<pergentage>`

- [CSS Spec](https://drafts.csswg.org/css-values/#percentages)
- [Mozilla Web Docs](https://developer.mozilla.org/en-US/docs/Web/CSS/percentage)

#### `<ratio>`

- [CSS Spec](https://drafts.csswg.org/css-values/#ratios)
- [Mozilla Web Docs](https://developer.mozilla.org/en-US/docs/Web/CSS/ratio)

#### `<angle>`

- [CSS Spec](https://drafts.csswg.org/css-values/#angles)
- [Mozilla Web Docs](https://developer.mozilla.org/en-US/docs/Web/CSS/angle)
- Possible angle units are `deg`, `grad`, `rad`, or `turn`

### Shorthand

Allows multiple properties to be set in one declaration.

#### Syntax

[`value type` | `value type` | ... ]{`min`,`max`}

Accepts some number of `value type`s, at least `min`, and at most `max` times.  Separate each `value type` with a space.

Where a property is shorthand for the 4 sides, the order is always: `-top`, `-right`, `-bottom`, `-left`.

#### Example

margin: [`auto` | `<length>` | `<percentage>`]{1,4}

> Shorthand for `margin-top`, `margin-right`, `margin-bottom`, `margin-left`

The `margin` shorthand property accepts either **_one_**, **_two_**, **_three_**, or **_four_** (i.e. '**{1,4}**')
values.  Each value can have the type of either `auto`, `<length>`, or `<percentage>`
(i.e. '[`auto` | `<length>` | `<percentage>`]').

- If only **_one_** value is given, then **all** the full properties will use that same value.
- If **_two_** values are given, then the **first** value will be used for the `-top` and `-bottom`, while the
  **second** value will be used for the `-right` and `-left`.
- If **_three_** values are given, then the **first** value will be used for the `-top`, the **second** will be used for
  the `-right` and `-left`, and the **third** will be used for the `-bottom`.
- If all **_four_** values are given, then they will each be used for `-top`, `-right`, `-bottom`, `-left` respectively.

The following are all valid `margin` declarations:

> margin: auto;  
> margin: 10px;  
> margin: 2% 1em;  
> margin: 20mm auto 0.3%;  
> margin: 1px 0.5% auto auto;

#### Colors

- [CSS Spec](https://www.w3.org/TR/css-color-4/#color-type)
- [Mozilla Web Docs](https://developer.mozilla.org/en-US/docs/Web/CSS/color)
- The `none` keyword has been added to line up with Bevy's `Color::None` variant
  - `none` is equivalent to CSS `transparent`, which is also available
- The `currentcolor` CSS keyword will be ignored
- The `hwb()` color function is not supported
- Color definitions are quite versatile, and can be a bit complicated.  The CSS docs help a lot.

#### `<rgb()>` & `<rgba()>`

- Format is: 'rgb([`<number>` | `<percentage>`]{3} [, `<number>` | `<percentage>`]?)'
- Both versions of this function accept the same arguments, despite the names
- The first 3 values are (in order) red, green, and blue
  - If these are given as a `<number>`, they must be in the range `0 ... 255`
- The 4th value is for alpha, and is optional
  - If a `<number>` is given, it must be in the range `0.0 ... 1.0`
- See also: [W3 Schools](https://www.w3schools.com/csSref/func_rgba.asp)

#### `<hsl()>` & `<hsla()>`

- Format is: 'hsl([`<number>` | `<angle>`], `<percentage>`, `<percentage>` [, `<number>` | `<percentage>`]?)'
- Both versions of this function accept the same arguments, despite the names
- The first value (`<number>` | `<angle>`) is hue, representing the color/hue angle.
  - If no unit is given, `deg` is assumed.  i.e. the number must be in the range `0 ... 360`
- The second value is saturation
- The third value is lightness
- The 4th value is for alpha, and is optional
  - If a `<number>` is given, it must be in the range `0.0 ... 1.0`
- See also: [W3 Schools](https://www.w3schools.com/csSref/func_hsla.asp)

#### `<hex-color>`

- Format is: `#<red><green><blue>` (i.e. `#RRGGBB` | `#RGB`)
- Each of `<red>`, `<green>`, and `<blue>` is a 2 digit hexadecimal integers (between `00 ... FF`)
  - `00` is none of the respective color, while `FF` is full intensity for the respective color
- As a shorthand, each color can be specified with only 1 digit.
  - In this case the digit is repeated for the color.  Eg: `#fff` -> `#ffffff` & `#abc` -> `#aabbcc`
  - If using shorthand, all 3 colors must use only 1 digit
- Case-insensitive
- [CSS Spec](https://www.w3.org/TR/css-color-4/#typedef-hex-color)
- [W3 Schools HEX Color Helper](https://www.w3schools.com/colors/colors_hexadecimal.asp)

#### `<named-color>`

- Use the links below for a full list of the available named colors
  - In particular, I recommend reading the 'gotcha' note in the Mozilla Web Docs
- Keep in mind that some colors may look different on different monitors/color profiles
- [CSS Spec](https://www.w3.org/TR/css-color-4/#typedef-named-color)
- [Mozilla Web Docs](https://developer.mozilla.org/en-US/docs/Web/CSS/color_value#color_keywords)

## License

This crate is dual licensed under the MIT license or the Apache 2.0 license.

Copies of these licenses are available in the `docs` folder.
# Bevy CSS (Prototype)

This project allows Bevy UI Node Styles to be defined with CSS strings, rather than the current verbose code
definitions.  This crate is currently still being developed, and lacks significant test coverage.

If you like this crate and would like to help make it better, please consider submitting a pull request.

## Features

### Current Features

#### Supported `ui::Node` component types

- `ui::Style`

#### Code inline css

UI styles can be created inline in your code from a css string and the appropriate (static) context:

    use bevy_prototype_css::{CssContext, CssStyle};

    fn main() {
        App::new()
            .add_plugins(DefaultPlugins)
            .add_system(setup)
            .run()
    }

    fn setup(mut commands: Commands) {
        let css_context = CssContext::default();
    
        commands.spawn_bundle(
            style: CssStyle("width: 100%; height: 10em;").to_style(css_context),
            ..Default::default(),
        );
    }

### Planned

- Spreadsheets as assets
- Support for the following `ui::Node` component types
  - `ui::UiColor`
  - `text::TextStyle`
  - `ui::UiImage`
- `@font-face` definitions for font asset loading
- `calc()` and other css functions
- Full set of CSS spec `<length>` dimensions
- `!important` keyword
- CSS wide keywords (`initial`, `inherit`, `unset`)

## Useful References

#### Bevy

- `ui::Style` doc: <https://docs.rs/bevy/latest/bevy/ui/struct.Style.html>

#### CSS Spec + Documentation

- Developer reference (Mozilla): <https://developer.mozilla.org/en-US/docs/Web/CSS>
- Values & Units Spec: <https://drafts.csswg.org/css-values/>
- w3 Schools tutorials & reference: <https://www.w3schools.com/css>

## Property Names

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

## Accepted Values

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

- margin: [`auto` | `<length>` | `<percentage>`]{1,4} (See Shorthand below)
- margin-top: `auto` | `<length>` | `<percentage>`
- margin-right: `auto` | `<length>` | `<percentage>`
- margin-bottom: `auto` | `<length>` | `<percentage>`
- margin-left: `auto` | `<length>` | `<percentage>`

#### Padding

- padding: [`auto` | `<length>` | `<percentage>`]{1,4} (See Shorthand below)
- padding-top: `auto` | `<length>` | `<percentage>`
- padding-right: `auto` | `<length>` | `<percentage>`
- padding-bottom: `auto` | `<length>` | `<percentage>`
- padding-left: `auto` | `<length>` | `<percentage>`

#### Borders

- border-width: [`auto` | `<length>` | `<percentage>`]{1,4} (See Shorthand below)
- border-width-top: `auto` | `<length>` | `<percentage>`
- border-width-right: `auto` | `<length>` | `<percentage>`
- border-width-bottom: `auto` | `<length>` | `<percentage>`
- border-width-left: `auto` | `<length>` | `<percentage>`

## Value Types

### `<number>`

- [CSS Spec](https://drafts.csswg.org/css-values/#numbers)
- [Mozilla Web Docs](https://developer.mozilla.org/en-US/docs/Web/CSS/number)

#### `<non-negative-number>`

- Same as `<numer>`, except the value has to be `>= 0`

### `<length>`

- [CSS Spec](https://drafts.csswg.org/css-values/#lengths)
- [Mozilla Web Docs](https://developer.mozilla.org/en-US/docs/Web/CSS/length)
- 96 DPI is assumed.  This means `1in` == `96px`, regardless of your actual dpi setting.  This is part of the CSS spec.
  - See also: <https://drafts.csswg.org/css-values/#reference-pixel>
- Not all dimensions in the CSS spec are accepted by this parser.
- The following dimensions are accepted:
    - Absolute: `px`, `cm`, `mm`, `Q`, `in`, `pc`, `pt`
    - Font Relative: `em`, `rem`, `ex`, `ch`
    - Viewport Relative: `vw`, `vh`, `vmin`, `vmax`

### `<pergentage>`

- [CSS Spec](https://drafts.csswg.org/css-values/#percentages)
- [Mozilla Web Docs](https://developer.mozilla.org/en-US/docs/Web/CSS/percentage)

### `<ratio>`

- [CSS Spec](https://drafts.csswg.org/css-values/#ratios)
- [Mozilla Web Docs](https://developer.mozilla.org/en-US/docs/Web/CSS/ratio)

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
- If all **_four_** values are given, then they will each be used `-top`, `-right`, `-bottom`, `-left` respectively.

The following are all valid `margin` declarations:

> margin: auto;  
> margin: 10px;  
> margin: 2% 1em;  
> margin: 20mm auto 0.3%;  
> margin: 1px 0.5% auto auto;  

## License

This crate is dual licensed under the MIT license or the Apache 2.0 license.
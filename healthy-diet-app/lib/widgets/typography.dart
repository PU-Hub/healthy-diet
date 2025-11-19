/// Typography widgets using Material Design text styles.
///
/// Provides convenient widget wrappers around Material Design's text theme
/// styles (Display, Headline, Title, Body, Label). Each widget automatically
/// applies the appropriate theme style while allowing customization through
/// color, weight, and style parameters.
library;

import 'package:flutter/material.dart';
import 'package:healthy_diet/utils/extensions/build_context.dart';

/// Display text widget for large, short, important text.
///
/// Display styles are the largest text on screen, reserved for short,
/// important text or numerals. They work best on large screens.
///
/// Provides three size variants: small, medium, and large, corresponding
/// to [TextTheme.displaySmall], [TextTheme.displayMedium], and
/// [TextTheme.displayLarge].
class DisplayText extends StatelessWidget {
  /// The text to display.
  final String data;

  /// Custom style to merge with the theme's display style.
  final TextStyle? style;

  /// How the text should be aligned horizontally.
  final TextAlign? align;

  /// Internal getter for the appropriate display text style from theme.
  final TextStyle? Function(BuildContext) _textStyleGetter;

  /// Creates small display text.
  ///
  /// Uses [TextTheme.displaySmall] as the base style.
  DisplayText.small(this.data, {super.key, Color? color, FontWeight? weight, TextStyle? style, this.align})
    : style = TextStyle(color: color, fontWeight: weight).merge(style),
      _textStyleGetter = ((context) => context.texts.displaySmall);

  /// Creates medium display text.
  ///
  /// Uses [TextTheme.displayMedium] as the base style.
  DisplayText.medium(this.data, {super.key, Color? color, FontWeight? weight, TextStyle? style, this.align})
    : style = TextStyle(color: color, fontWeight: weight).merge(style),
      _textStyleGetter = ((context) => context.texts.displayMedium);

  /// Creates large display text.
  ///
  /// Uses [TextTheme.displayLarge] as the base style.
  DisplayText.large(this.data, {super.key, Color? color, FontWeight? weight, TextStyle? style, this.align})
    : style = TextStyle(color: color, fontWeight: weight).merge(style),
      _textStyleGetter = ((context) => context.texts.displayLarge);

  @override
  Widget build(BuildContext context) {
    return Text(data, style: _textStyleGetter(context)?.merge(style), textAlign: align);
  }
}

/// Headline text widget for high-emphasis text.
///
/// Headlines are smaller than display styles and should be used for short,
/// high-emphasis text on smaller screens. These are best-suited for short,
/// high-emphasis text on smaller screens.
///
/// Provides three size variants: small, medium, and large, corresponding
/// to [TextTheme.headlineSmall], [TextTheme.headlineMedium], and
/// [TextTheme.headlineLarge].
class HeadLineText extends StatelessWidget {
  /// The text to display.
  final String data;

  /// Custom style to merge with the theme's headline style.
  final TextStyle? style;

  /// How the text should be aligned horizontally.
  final TextAlign? align;

  /// Internal getter for the appropriate headline text style from theme.
  final TextStyle? Function(BuildContext) _textStyleGetter;

  /// Creates small headline text.
  ///
  /// Uses [TextTheme.headlineSmall] as the base style.
  HeadLineText.small(this.data, {super.key, Color? color, FontWeight? weight, TextStyle? style, this.align})
    : style = TextStyle(color: color, fontWeight: weight).merge(style),
      _textStyleGetter = ((context) => context.texts.headlineSmall);

  /// Creates medium headline text.
  ///
  /// Uses [TextTheme.headlineMedium] as the base style.
  HeadLineText.medium(this.data, {super.key, Color? color, FontWeight? weight, TextStyle? style, this.align})
    : style = TextStyle(color: color, fontWeight: weight).merge(style),
      _textStyleGetter = ((context) => context.texts.headlineMedium);

  /// Creates large headline text.
  ///
  /// Uses [TextTheme.headlineLarge] as the base style.
  HeadLineText.large(this.data, {super.key, Color? color, FontWeight? weight, TextStyle? style, this.align})
    : style = TextStyle(color: color, fontWeight: weight).merge(style),
      _textStyleGetter = ((context) => context.texts.headlineLarge);

  @override
  Widget build(BuildContext context) {
    return Text(data, style: _textStyleGetter(context)?.merge(style), textAlign: align);
  }
}

/// Title text widget for medium-emphasis text.
///
/// Titles are smaller than headline styles and should be used for medium-
/// emphasis text that remains relatively short. For example, consider using
/// title styles to divide secondary passages of text or sections of text.
///
/// Provides three size variants: small, medium, and large, corresponding
/// to [TextTheme.titleSmall], [TextTheme.titleMedium], and
/// [TextTheme.titleLarge].
class TitleText extends StatelessWidget {
  /// The text to display.
  final String data;

  /// Custom style to merge with the theme's title style.
  final TextStyle? style;

  /// How the text should be aligned horizontally.
  final TextAlign? align;

  /// Internal getter for the appropriate title text style from theme.
  final TextStyle? Function(BuildContext) _textStyleGetter;

  /// Creates small title text.
  ///
  /// Uses [TextTheme.titleSmall] as the base style. The [leading] parameter
  /// sets the line height.
  TitleText.small(
    this.data, {
    super.key,
    Color? color,
    FontWeight? weight,
    double? leading,
    TextStyle? style,
    this.align,
  }) : style = TextStyle(color: color, fontWeight: weight, height: leading).merge(style),
       _textStyleGetter = ((context) => context.texts.titleSmall);

  /// Creates medium title text.
  ///
  /// Uses [TextTheme.titleMedium] as the base style. The [leading] parameter
  /// sets the line height.
  TitleText.medium(
    this.data, {
    super.key,
    Color? color,
    FontWeight? weight,
    double? leading,
    TextStyle? style,
    this.align,
  }) : style = TextStyle(color: color, fontWeight: weight, height: leading).merge(style),
       _textStyleGetter = ((context) => context.texts.titleMedium);

  /// Creates large title text.
  ///
  /// Uses [TextTheme.titleLarge] as the base style. The [leading] parameter
  /// sets the line height.
  TitleText.large(
    this.data, {
    super.key,
    Color? color,
    FontWeight? weight,
    double? leading,
    TextStyle? style,
    this.align,
  }) : style = TextStyle(color: color, fontWeight: weight, height: leading).merge(style),
       _textStyleGetter = ((context) => context.texts.titleLarge);

  @override
  Widget build(BuildContext context) {
    return Text(
      data,
      style: _textStyleGetter(context)?.merge(style),
      textAlign: align,
    );
  }
}

/// Body text widget for regular paragraph text.
///
/// Body styles are used for longer passages of text in your app. Use body
/// styles for displaying most of the text in a UI, such as in articles,
/// blog posts, or any content that requires comfortable reading.
///
/// Provides three size variants: small, medium, and large, corresponding
/// to [TextTheme.bodySmall], [TextTheme.bodyMedium], and
/// [TextTheme.bodyLarge].
class BodyText extends StatelessWidget {
  /// The text to display.
  final String data;

  /// Custom style to merge with the theme's body style.
  final TextStyle? style;

  /// How the text should be aligned horizontally.
  final TextAlign? align;

  /// Internal getter for the appropriate body text style from theme.
  final TextStyle? Function(BuildContext) _textStyleGetter;

  /// Creates small body text.
  ///
  /// Uses [TextTheme.bodySmall] as the base style. The [leading] parameter
  /// sets the line height.
  BodyText.small(
    this.data, {
    super.key,
    Color? color,
    FontWeight? weight,
    double? leading,
    TextStyle? style,
    this.align,
  }) : style = TextStyle(color: color, fontWeight: weight, height: leading).merge(style),
       _textStyleGetter = ((context) => context.texts.bodySmall);

  /// Creates medium body text.
  ///
  /// Uses [TextTheme.bodyMedium] as the base style. The [leading] parameter
  /// sets the line height.
  BodyText.medium(
    this.data, {
    super.key,
    Color? color,
    FontWeight? weight,
    double? leading,
    TextStyle? style,
    this.align,
  }) : style = TextStyle(color: color, fontWeight: weight, height: leading).merge(style),
       _textStyleGetter = ((context) => context.texts.bodyMedium);

  /// Creates large body text.
  ///
  /// Uses [TextTheme.bodyLarge] as the base style. The [leading] parameter
  /// sets the line height.
  BodyText.large(
    this.data, {
    super.key,
    Color? color,
    FontWeight? weight,
    double? leading,
    TextStyle? style,
    this.align,
  }) : style = TextStyle(color: color, fontWeight: weight, height: leading).merge(style),
       _textStyleGetter = ((context) => context.texts.bodyLarge);

  @override
  Widget build(BuildContext context) {
    return Text(
      data,
      style: _textStyleGetter(context)?.merge(style),
      textAlign: align,
    );
  }
}

/// Label text widget for UI components and annotations.
///
/// Label styles are smaller, utilitarian styles used for things like the
/// text inside components or for very small text in the content body, such
/// as captions. Button text also uses label styles.
///
/// Provides three size variants: small, medium, and large, corresponding
/// to [TextTheme.labelSmall], [TextTheme.labelMedium], and
/// [TextTheme.labelLarge].
class LabelText extends StatelessWidget {
  /// The text to display.
  final String data;

  /// Custom style to merge with the theme's label style.
  final TextStyle? style;

  /// How the text should be aligned horizontally.
  final TextAlign? align;

  /// Internal getter for the appropriate label text style from theme.
  final TextStyle? Function(BuildContext) _textStyleGetter;

  /// Creates small label text.
  ///
  /// Uses [TextTheme.labelSmall] as the base style. The [leading] parameter
  /// sets the line height.
  LabelText.small(
    this.data, {
    super.key,
    Color? color,
    FontWeight? weight,
    double? leading,
    TextStyle? style,
    this.align,
  }) : style = TextStyle(color: color, fontWeight: weight, height: leading).merge(style),
       _textStyleGetter = ((context) => context.texts.labelSmall);

  /// Creates medium label text.
  ///
  /// Uses [TextTheme.labelMedium] as the base style. The [leading] parameter
  /// sets the line height.
  LabelText.medium(
    this.data, {
    super.key,
    Color? color,
    FontWeight? weight,
    double? leading,
    TextStyle? style,
    this.align,
  }) : style = TextStyle(color: color, fontWeight: weight, height: leading).merge(style),
       _textStyleGetter = ((context) => context.texts.labelMedium);

  /// Creates large label text.
  ///
  /// Uses [TextTheme.labelLarge] as the base style. The [leading] parameter
  /// sets the line height.
  LabelText.large(
    this.data, {
    super.key,
    Color? color,
    FontWeight? weight,
    double? leading,
    TextStyle? style,
    this.align,
  }) : style = TextStyle(color: color, fontWeight: weight, height: leading).merge(style),
       _textStyleGetter = ((context) => context.texts.labelLarge);

  @override
  Widget build(BuildContext context) {
    return Text(
      data,
      style: _textStyleGetter(context)?.merge(style),
      textAlign: align,
    );
  }
}

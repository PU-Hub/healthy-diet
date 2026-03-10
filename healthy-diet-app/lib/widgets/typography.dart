/// Typed widget wrappers for Material Design text theme styles (Display, Headline, Title, Body, Label).
library;

import 'package:flutter/material.dart';
import 'package:healthy_diet/utils/extensions/build_context.dart';

/// Display text widget for large, short, important text or numerals.
///
/// Provides [DisplayText.small], [DisplayText.medium], and [DisplayText.large] variants.
class DisplayText extends StatelessWidget {
  final String data;
  final TextStyle? style;
  final TextAlign? align;
  final TextStyle? Function(BuildContext) _textStyleGetter;

  /// Uses [TextTheme.displaySmall].
  DisplayText.small(this.data, {super.key, Color? color, FontWeight? weight, TextStyle? style, this.align})
    : style = TextStyle(color: color, fontWeight: weight).merge(style),
      _textStyleGetter = ((context) => context.texts.displaySmall);

  /// Uses [TextTheme.displayMedium].
  DisplayText.medium(this.data, {super.key, Color? color, FontWeight? weight, TextStyle? style, this.align})
    : style = TextStyle(color: color, fontWeight: weight).merge(style),
      _textStyleGetter = ((context) => context.texts.displayMedium);

  /// Uses [TextTheme.displayLarge].
  DisplayText.large(this.data, {super.key, Color? color, FontWeight? weight, TextStyle? style, this.align})
    : style = TextStyle(color: color, fontWeight: weight).merge(style),
      _textStyleGetter = ((context) => context.texts.displayLarge);

  @override
  Widget build(BuildContext context) {
    return Text(data, style: _textStyleGetter(context)?.merge(style), textAlign: align);
  }
}

/// Headline text widget for short, high-emphasis text.
///
/// Provides [HeadLineText.small], [HeadLineText.medium], and [HeadLineText.large] variants.
class HeadLineText extends StatelessWidget {
  final String data;
  final TextStyle? style;
  final TextAlign? align;
  final TextStyle? Function(BuildContext) _textStyleGetter;

  /// Uses [TextTheme.headlineSmall].
  HeadLineText.small(this.data, {super.key, Color? color, FontWeight? weight, TextStyle? style, this.align})
    : style = TextStyle(color: color, fontWeight: weight).merge(style),
      _textStyleGetter = ((context) => context.texts.headlineSmall);

  /// Uses [TextTheme.headlineMedium].
  HeadLineText.medium(this.data, {super.key, Color? color, FontWeight? weight, TextStyle? style, this.align})
    : style = TextStyle(color: color, fontWeight: weight).merge(style),
      _textStyleGetter = ((context) => context.texts.headlineMedium);

  /// Uses [TextTheme.headlineLarge].
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
/// Provides [TitleText.small], [TitleText.medium], and [TitleText.large] variants.
class TitleText extends StatelessWidget {
  final String data;
  final TextStyle? style;
  final TextAlign? align;
  final TextStyle? Function(BuildContext) _textStyleGetter;

  /// Uses [TextTheme.titleSmall]. The [leading] parameter sets the line height.
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

  /// Uses [TextTheme.titleMedium]. The [leading] parameter sets the line height.
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

  /// Uses [TextTheme.titleLarge]. The [leading] parameter sets the line height.
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

/// Body text widget for longer passages of text.
///
/// Provides [BodyText.small], [BodyText.medium], and [BodyText.large] variants.
class BodyText extends StatelessWidget {
  final String data;
  final TextStyle? style;
  final TextAlign? align;
  final TextStyle? Function(BuildContext) _textStyleGetter;

  /// Uses [TextTheme.bodySmall]. The [leading] parameter sets the line height.
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

  /// Uses [TextTheme.bodyMedium]. The [leading] parameter sets the line height.
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

  /// Uses [TextTheme.bodyLarge]. The [leading] parameter sets the line height.
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

/// Label text widget for UI components, captions, and button text.
///
/// Provides [LabelText.small], [LabelText.medium], and [LabelText.large] variants.
class LabelText extends StatelessWidget {
  final String data;
  final TextStyle? style;
  final TextAlign? align;
  final TextStyle? Function(BuildContext) _textStyleGetter;

  /// Uses [TextTheme.labelSmall]. The [leading] parameter sets the line height.
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

  /// Uses [TextTheme.labelMedium]. The [leading] parameter sets the line height.
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

  /// Uses [TextTheme.labelLarge]. The [leading] parameter sets the line height.
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

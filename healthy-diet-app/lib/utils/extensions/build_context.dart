/// Extensions on [BuildContext] for convenient access to common values.
///
/// Provides shorthand getters for frequently accessed values from Theme,
/// MediaQuery, and ScaffoldMessenger, reducing boilerplate code throughout
/// the application.
library;

import 'package:flutter/material.dart';

/// Convenience extension methods on [BuildContext].
///
/// Provides quick access to theme data, media queries, and scaffolds without
/// repeatedly calling verbose static methods. Improves code readability by
/// using concise property names.
extension BuildContextExtension on BuildContext {
  // Theme

  /// The current [ThemeData] from the nearest [Theme] ancestor.
  ThemeData get theme => Theme.of(this);

  /// The [ColorScheme] from the current theme.
  ///
  /// Shorthand for `Theme.of(context).colorScheme`.
  ColorScheme get colors => theme.colorScheme;

  /// The [TextTheme] from the current theme.
  ///
  /// Shorthand for `Theme.of(context).textTheme`.
  TextTheme get texts => theme.textTheme;

  // MediaQuery

  /// The size of the media in logical pixels.
  ///
  /// Equivalent to `MediaQuery.sizeOf(context)`.
  Size get dimension => MediaQuery.sizeOf(this);

  /// The current brightness mode (light or dark).
  ///
  /// Returns the platform's brightness setting.
  Brightness get brightness => MediaQuery.platformBrightnessOf(this);

  /// The padding for system UI intrusions.
  ///
  /// Includes areas like status bars, notches, and navigation bars.
  /// Equivalent to `MediaQuery.paddingOf(context)`.
  EdgeInsets get padding => MediaQuery.paddingOf(this);

  /// The current view insets.
  ///
  /// Represents areas obscured by system UI like the keyboard.
  /// Equivalent to `MediaQuery.viewInsetsOf(context)`.
  EdgeInsets get insets => MediaQuery.viewInsetsOf(this);

  // ScaffoldMessenger

  /// The [ScaffoldMessengerState] from the nearest [ScaffoldMessenger] ancestor.
  ///
  /// Used to show snackbars and material banners.
  ScaffoldMessengerState get scaffold => ScaffoldMessenger.of(this);
}

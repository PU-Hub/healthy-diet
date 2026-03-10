/// Convenience extension on [BuildContext] for quick access to theme, media queries, and scaffold.
library;

import 'package:flutter/material.dart';

extension BuildContextExtension on BuildContext {
  // Theme

  ThemeData get theme => Theme.of(this);

  /// Shorthand for `Theme.of(context).colorScheme`.
  ColorScheme get colors => theme.colorScheme;

  /// Shorthand for `Theme.of(context).textTheme`.
  TextTheme get texts => theme.textTheme;

  // MediaQuery

  Size get dimension => MediaQuery.sizeOf(this);

  Brightness get brightness => MediaQuery.platformBrightnessOf(this);

  /// System UI padding (status bar, notch, navigation bar).
  EdgeInsets get padding => MediaQuery.paddingOf(this);

  /// Areas obscured by system UI (e.g. keyboard).
  EdgeInsets get insets => MediaQuery.viewInsetsOf(this);

  // ScaffoldMessenger

  ScaffoldMessengerState get scaffold => ScaffoldMessenger.of(this);
}

import 'package:flutter/material.dart';

extension BuildContextExtension on BuildContext {
  // Theme
  ThemeData get theme => Theme.of(this);
  ColorScheme get colors => theme.colorScheme;
  TextTheme get texts => theme.textTheme;

  // MediaQuery
  Size get dimension => MediaQuery.sizeOf(this);
  Brightness get brightness => MediaQuery.platformBrightnessOf(this);
  EdgeInsets get padding => MediaQuery.paddingOf(this);
  EdgeInsets get insets => MediaQuery.viewInsetsOf(this);

  // ScaffoldMessenger
  ScaffoldMessengerState get scaffold => ScaffoldMessenger.of(this);
}

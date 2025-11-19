/// The root application widget for Healthy Diet.
///
/// This file defines the main application widget that configures theming,
/// routing, and Material Design for the entire app. It integrates dynamic
/// system colors to adapt to the user's device theme preferences.
library;

import 'package:flutter/material.dart';

import 'package:dynamic_system_colors/dynamic_system_colors.dart';

import 'package:healthy_diet/router.dart';

/// The root application widget.
///
/// Configures the Material Design app with dynamic theming that adapts to
/// system color preferences. Uses [MaterialApp.router] with go_router for
/// type-safe navigation throughout the application.
///
/// The app supports both light and dark themes that automatically adjust
/// based on the device's color scheme and brightness settings.
class HealthyDiet extends StatelessWidget {
  /// Creates the root application widget.
  const HealthyDiet({super.key});

  /// Builds the application with dynamic theming.
  ///
  /// Creates light and dark [ThemeData] configurations using system colors
  /// when available. Returns a [MaterialApp.router] configured with the
  /// application's routing structure and theme settings.
  @override
  Widget build(BuildContext context) {
    return DynamicColorBuilder(
      builder: (lightDynamic, darkDynamic) {
        final light = ThemeData(
          brightness: .light,
          colorScheme: lightDynamic,
          snackBarTheme: SnackBarThemeData(behavior: .floating),
          progressIndicatorTheme: ProgressIndicatorThemeData(year2023: false),
          sliderTheme: SliderThemeData(year2023: false),
        );
        final dark = ThemeData(
          brightness: .dark,
          colorScheme: darkDynamic,
          snackBarTheme: SnackBarThemeData(behavior: .floating),
          progressIndicatorTheme: ProgressIndicatorThemeData(year2023: false),
          sliderTheme: SliderThemeData(year2023: false),
        );

        return MaterialApp.router(
          title: '健康飲食',
          routerConfig: router,
          theme: light,
          darkTheme: dark,
        );
      },
    );
  }
}

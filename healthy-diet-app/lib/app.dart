library;

import 'package:dynamic_color/dynamic_color.dart';
import 'package:flutter/material.dart';
import 'package:healthy_diet/app/provider.dart';
import 'package:healthy_diet/router.dart';
import 'package:provider/provider.dart';

/// Root application widget with dynamic theming.
class HealthyDiet extends StatelessWidget {
  const HealthyDiet({super.key});

  @override
  Widget build(BuildContext context) {
    return DynamicColorBuilder(
      builder: (lightDynamic, darkDynamic) {
        final light = ThemeData(
          brightness: .light,
          colorSchemeSeed: lightDynamic?.primary,
          snackBarTheme: SnackBarThemeData(behavior: .floating),
          progressIndicatorTheme: ProgressIndicatorThemeData(year2023: false),
          sliderTheme: SliderThemeData(year2023: false),
        );
        final dark = ThemeData(
          brightness: .dark,
          colorSchemeSeed: darkDynamic?.primary,
          snackBarTheme: SnackBarThemeData(behavior: .floating),
          progressIndicatorTheme: ProgressIndicatorThemeData(year2023: false),
          sliderTheme: SliderThemeData(year2023: false),
        );

        return ChangeNotifierProvider<AppProvider>.value(
          value: appProvider,
          child: MaterialApp.router(
            title: '健康飲食',
            routerConfig: router,
            theme: light,
            darkTheme: dark,
          ),
        );
      },
    );
  }
}

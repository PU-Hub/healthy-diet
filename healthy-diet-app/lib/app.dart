import 'package:flutter/material.dart';

import 'package:dynamic_system_colors/dynamic_system_colors.dart';

import 'package:healthy_diet/router.dart';

class MyWidget extends StatelessWidget {
  const MyWidget({super.key});

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

import 'package:flutter/material.dart';
import 'package:healthy_diet/utils/extensions/build_context.dart';
import 'package:material_symbols_icons/symbols.dart';

class AppShellWidget extends StatefulWidget {
  const AppShellWidget({
    required this.navigator,
    super.key,
  });

  final Widget navigator;

  @override
  State<AppShellWidget> createState() => _AppShellWidgetState();
}

class _AppShellWidgetState extends State<AppShellWidget> {
  final GlobalKey<ScaffoldState> _key = .new();

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      key: _key,
      body: widget.navigator,
      appBar: AppBar(
        backgroundColor: Colors.transparent,
        surfaceTintColor: Colors.transparent,
        leadingWidth: 72,
        leading: IconButton.filled(
          style: IconButton.styleFrom(
            backgroundColor: context.colors.surfaceContainer,
            shape: RoundedRectangleBorder(
              side: .new(color: context.colors.surfaceContainerHigh),
              borderRadius: .circular(32),
            ),
            shadowColor: context.colors.shadow,
            elevation: 8,
          ),
          onPressed: () {
            final state = _key.currentState;
            if (state == null) return;
            state.openDrawer();
          },
          icon: Icon(Symbols.menu_rounded),
        ),
      ),
      drawer: NavigationDrawer(
        children: [NavigationDrawerDestination(icon: const Icon(Symbols.home_rounded), label: Text('首頁'))],
      ),
      extendBodyBehindAppBar: true,
    );
  }
}

/// Welcome flow shell widget providing consistent navigation UI.
///
/// This file defines the shell layout that wraps all welcome screens with
/// a unified navigation interface. The shell includes a bottom sheet with
/// back and next buttons, managed by [WelcomeProvider] state.
library;

import 'package:flutter/material.dart';

import 'package:go_router/go_router.dart';
import 'package:healthy_diet/app/welcome/provider.dart';
import 'package:healthy_diet/utils/extensions/build_context.dart';
import 'package:healthy_diet/utils/extensions/edge_insets.dart';
import 'package:provider/provider.dart';

/// Welcome shell widget providing consistent navigation UI.
///
/// Wraps all welcome-related screens with a common UI structure that includes
/// navigation controls at the bottom. This shell provides a consistent layout
/// with back and next buttons for moving through the welcome screens.
///
/// The bottom sheet navigation adapts based on the current state managed by
/// [WelcomeProvider], including handling loading states and navigation limits.
///
/// Used by the welcome shell route to provide consistent UI across all
/// welcome flow screens.
class WelcomeShellWidget extends StatelessWidget {
  /// Creates a welcome shell widget.
  ///
  /// The [navigator] parameter contains the nested route content that will be
  /// displayed in the body of the scaffold.
  const WelcomeShellWidget({
    required this.navigator,
    super.key,
  });

  /// The nested navigator containing welcome screen content.
  final Widget navigator;

  /// Builds the shell layout with navigation controls.
  ///
  /// Creates a scaffold with a bottom sheet containing navigation buttons.
  /// Provides a [ChangeNotifierProvider] with [WelcomeProvider] to manage
  /// navigation state across all welcome screens. The bottom navigation
  /// includes:
  /// - Back button: Enabled when navigation stack allows popping
  /// - Next button: Enabled based on [WelcomeProvider.canNext] state
  @override
  Widget build(BuildContext context) {
    return ChangeNotifierProvider<WelcomeProvider>(
      create: (context) => .new(),
      builder: (context, child) {
        return Scaffold(
          body: child,
          bottomSheet: Material(
            elevation: 8,
            surfaceTintColor: context.colors.surfaceTint,
            shape: RoundedRectangleBorder(
              borderRadius: .vertical(top: .circular(24)),
            ),
            child: Padding(
              padding: context.padding.onlyBottom + .fromLTRB(16, 8, 16, 8),
              child: Consumer<WelcomeProvider>(
                builder: (context, provider, child) {
                  final WelcomeProvider(
                    canNext: canNext,
                    isLast: isLast,
                    isLoading: isLoading,
                    nextRoute: nextRoute,
                  ) = provider;

                  return Row(
                    mainAxisAlignment: .spaceBetween,
                    children: [
                      PopScope(
                        canPop: !isLoading && context.canPop(),
                        child: IconButton(
                          icon: const Icon(Icons.arrow_back),
                          onPressed: !isLoading && context.canPop() ? () => context.pop() : null,
                        ),
                      ),
                      FilledButton.icon(
                        icon: const Icon(Icons.arrow_forward),
                        label: Text('繼續'),
                        iconAlignment: .end,
                        style: FilledButton.styleFrom(padding: .only(left: 16, right: 16)),
                        onPressed: !isLoading && canNext && nextRoute != null
                            ? () => (isLast ? nextRoute.go : nextRoute.push)(context)
                            : null,
                      ),
                    ],
                  );
                },
              ),
            ),
          ),
        );
      },
      child: navigator,
    );
  }
}

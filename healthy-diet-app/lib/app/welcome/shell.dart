/// Welcome flow shell widget providing consistent navigation UI.
///
/// This file defines the shell layout that wraps all welcome screens with a unified navigation interface. The shell
/// includes a bottom sheet with back and next buttons, managed by [WelcomeProvider] state.
library;

import 'package:flutter/material.dart';

import 'package:go_router/go_router.dart';
import 'package:material_symbols_icons/symbols.dart';
import 'package:provider/provider.dart';

import 'package:healthy_diet/app/welcome/provider.dart';
import 'package:healthy_diet/utils/extensions/build_context.dart';
import 'package:healthy_diet/utils/extensions/edge_insets.dart';

/// Welcome shell widget providing consistent navigation UI.
///
/// Wraps all welcome-related screens with a common UI structure that includes navigation controls at the bottom. This
/// shell provides a consistent layout with back and next buttons for moving through the welcome screens.
///
/// The bottom sheet navigation adapts based on the current state managed by [WelcomeProvider], including handling
/// loading states and navigation limits.
///
/// Used by the welcome shell route to provide consistent UI across all welcome flow screens.
class WelcomeShellWidget extends StatelessWidget {
  /// Creates a welcome shell widget.
  ///
  /// The [navigator] parameter contains the nested route content that will be displayed in the body of the scaffold.
  const WelcomeShellWidget({
    required this.navigator,
    super.key,
  });

  /// The nested navigator containing welcome screen content.
  final Widget navigator;

  /// Builds the shell layout with navigation controls.
  ///
  /// Creates a scaffold with a bottom sheet containing navigation buttons. Provides a [ChangeNotifierProvider] with
  /// [WelcomeProvider] to manage navigation state across all welcome screens.
  ///
  /// The bottom navigation includes:
  /// - **Back button**: Enabled when navigation stack allows popping and not loading
  /// - **Next button**: Shows loading indicator during async operations, enabled based on [WelcomeProvider.canNext]
  ///   state
  ///
  /// The next button executes the following sequence:
  /// 1. If [WelcomeProvider.nextRouteCallback] is set, executes it and shows loading state. Any errors are displayed
  ///    via snackbar.
  /// 2. After callback completes (or if none is set), navigates to next route:
  ///    - If `isLast` is true, uses `go()` to replace the current route
  ///    - If `isLast` is false, uses `push()` to add to the navigation stack
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
                  void next() async {
                    final nextRouteCallback = provider.nextRouteCallback;

                    if (nextRouteCallback != null) {
                      provider.setIsLoading(true);
                      try {
                        await nextRouteCallback();
                      } catch (e) {
                        if (!context.mounted) return;
                        context.scaffold.showSnackBar(SnackBar(content: Text(e.toString())));
                        return;
                      } finally {
                        provider.setIsLoading(false);
                      }
                    }

                    if (!context.mounted) return;

                    if (provider.isLast) {
                      provider.nextRoute?.go(context);
                    } else {
                      provider.nextRoute?.push(context);
                    }
                  }

                  return Row(
                    mainAxisAlignment: .spaceBetween,
                    children: [
                      PopScope(
                        canPop: !provider.isLoading && context.canPop(),
                        child: IconButton(
                          icon: const Icon(Symbols.arrow_back_rounded),
                          onPressed: !provider.isLoading && context.canPop() ? () => context.pop() : null,
                        ),
                      ),
                      FilledButton.icon(
                        icon: provider.isLoading
                            ? const SizedBox.square(dimension: 24, child: CircularProgressIndicator(strokeWidth: 2))
                            : const Icon(Symbols.arrow_forward_rounded),
                        label: Text('繼續'),
                        iconAlignment: .end,
                        style: FilledButton.styleFrom(padding: .only(left: 16, right: 16)),
                        onPressed: !provider.isLoading && provider.canNext && provider.nextRoute != null ? next : null,
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

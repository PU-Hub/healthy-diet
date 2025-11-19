/// Application routing configuration using go_router for type-safe navigation.
///
/// This file defines the app's routing structure, including shell routes and
/// page routes. The router uses code generation to provide type-safe route
/// navigation throughout the application.
library;

import 'package:flutter/material.dart';

import 'package:go_router/go_router.dart';
import 'package:healthy_diet/app/welcome/introduction/page.dart';
import 'package:healthy_diet/app/welcome/provider.dart';
import 'package:healthy_diet/utils/extensions/build_context.dart';
import 'package:healthy_diet/utils/extensions/edge_insets.dart';
import 'package:provider/provider.dart';

part 'router.g.dart';

/// The main application router instance.
///
/// Defines the navigation structure and initial route for the application.
/// Debug logging is enabled to help track navigation events during development.
final router = GoRouter(
  initialLocation: WelcomeIntroductionRoute().location,
  routes: $appRoutes,
  debugLogDiagnostics: true,
);

/// Shell route for the welcome flow.
///
/// Wraps all welcome-related screens with a common UI structure that includes
/// navigation controls at the bottom. This shell provides a consistent layout
/// with back and next buttons for moving through the welcome screens.
///
/// The bottom sheet navigation adapts based on the current state managed by
/// [WelcomeProvider], including handling loading states and navigation limits.
@TypedShellRoute<WelcomeShell>(
  routes: [
    TypedGoRoute<WelcomeIntroductionRoute>(
      path: '/welcome/introduction',
    ),
  ],
)
class WelcomeShell extends ShellRouteData {
  /// Builds the shell layout with navigation controls.
  ///
  /// Creates a scaffold with a bottom sheet containing navigation buttons.
  /// The navigator parameter contains the nested route content that will be
  /// displayed in the body of the scaffold.
  @override
  Widget builder(BuildContext context, GoRouterState state, Widget navigator) {
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

/// Route for the welcome introduction screen.
///
/// This is the entry point of the welcome flow, displayed at `/welcome/introduction`.
/// It shows the initial introduction content to new users.
class WelcomeIntroductionRoute extends GoRouteData with $WelcomeIntroductionRoute {
  /// Builds the welcome introduction screen.
  ///
  /// Returns a [WelcomeIntroductionScreen] widget that displays the introduction
  /// content within the [WelcomeShell] layout.
  @override
  Widget build(BuildContext context, GoRouterState state) => const WelcomeIntroductionScreen();
}

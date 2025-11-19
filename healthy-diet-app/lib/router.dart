/// Application routing configuration using go_router for type-safe navigation.
///
/// This file defines the app's routing structure, including shell routes and
/// page routes. The router uses code generation to provide type-safe route
/// navigation throughout the application.
library;

import 'package:flutter/material.dart';

import 'package:go_router/go_router.dart';
import 'package:healthy_diet/app/welcome/introduction/page.dart';
import 'package:healthy_diet/app/welcome/login/page.dart';
import 'package:healthy_diet/app/welcome/shell.dart';

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

/// Welcome flow shell route configuration.
///
/// Defines the shell route that wraps welcome screens with the [WelcomeShellWidget]
/// layout. The shell provides consistent navigation UI across all welcome pages.
@TypedShellRoute<WelcomeShell>(
  routes: [
    TypedGoRoute<WelcomeIntroductionRoute>(
      path: '/welcome/introduction',
    ),
    TypedGoRoute<WelcomeLoginRoute>(
      path: '/welcome/login',
    ),
  ],
)
class WelcomeShell extends ShellRouteData {
  /// Builds the shell layout using [WelcomeShellWidget].
  ///
  /// Delegates to [WelcomeShellWidget] which provides the actual UI
  /// implementation with navigation controls.
  @override
  Widget builder(BuildContext context, GoRouterState state, Widget navigator) {
    return WelcomeShellWidget(navigator: navigator);
  }
}

/// Route for the welcome introduction page.
///
/// This is the entry point of the welcome flow, displayed at `/welcome/introduction`.
/// It shows the initial introduction content to new users.
class WelcomeIntroductionRoute extends GoRouteData with $WelcomeIntroductionRoute {
  /// Builds the welcome introduction page.
  ///
  /// Returns a [WelcomeIntroductionPage] widget that displays the introduction
  /// content within the [WelcomeShell] layout.
  @override
  Widget build(BuildContext context, GoRouterState state) => const WelcomeIntroductionPage();
}

/// Route for the welcome login page.
///
/// Displays the login or authentication screen as part of the welcome flow,
/// located at `/welcome/login`. Users reach this page after completing the
/// introduction step.
class WelcomeLoginRoute extends GoRouteData with $WelcomeLoginRoute {
  /// Builds the welcome login page.
  ///
  /// Returns a [WelcomeLoginPage] widget that displays the login interface
  /// within the [WelcomeShell] layout.
  @override
  Widget build(BuildContext context, GoRouterState state) => const WelcomeLoginPage();
}

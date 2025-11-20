/// Application routing configuration using go_router for type-safe navigation.
///
/// This file defines the app's routing structure, including shell routes and page routes. The router uses code
/// generation to provide type-safe route navigation throughout the application.
library;

import 'package:flutter/material.dart';

import 'package:go_router/go_router.dart';

import 'package:healthy_diet/app/welcome/introduction/page.dart';
import 'package:healthy_diet/app/welcome/login/page.dart';
import 'package:healthy_diet/app/welcome/register/page.dart';
import 'package:healthy_diet/app/welcome/shell.dart';

part 'router.g.dart';

/// Global route observer for monitoring navigation events.
///
/// Used to track route lifecycle events across the application. Pages that need to respond to navigation events (like
/// returning from a pushed route) should subscribe to this observer using the [RouteAware] mixin.
///
/// This observer is registered with [WelcomeShell] to enable welcome pages to respond to navigation changes.
final routeObserver = RouteObserver<PageRoute>();

/// The main application router instance.
///
/// Defines the navigation structure and initial route for the application. Debug logging is enabled to help track
/// navigation events during development.
final router = GoRouter(
  initialLocation: WelcomeIntroductionRoute().location,
  routes: $appRoutes,
  debugLogDiagnostics: true,
);

/// Welcome flow shell route configuration.
///
/// Defines the shell route that wraps welcome screens with the [WelcomeShellWidget] layout. The shell provides
/// consistent navigation UI across all welcome pages.
///
/// This shell contains the following routes:
/// - `/welcome/introduction` - [WelcomeIntroductionRoute]
/// - `/welcome/login` - [WelcomeLoginRoute]
/// - `/welcome/register` - [WelcomeRegisterRoute]
@TypedShellRoute<WelcomeShell>(
  routes: [
    TypedGoRoute<WelcomeIntroductionRoute>(
      path: '/welcome/introduction',
    ),
    TypedGoRoute<WelcomeLoginRoute>(
      path: '/welcome/login',
    ),
    TypedGoRoute<WelcomeRegisterRoute>(
      path: '/welcome/register',
    ),
  ],
)
class WelcomeShell extends ShellRouteData {
  /// Navigator observers for this shell route.
  ///
  /// Registers the [routeObserver] to enable route lifecycle monitoring for welcome pages. This allows pages to use
  /// [RouteAware] mixin to respond to navigation events.
  static final List<NavigatorObserver> $observers = [routeObserver];

  /// Builds the shell layout using [WelcomeShellWidget].
  ///
  /// Delegates to [WelcomeShellWidget] which provides the actual UI implementation with navigation controls.
  @override
  Widget builder(BuildContext context, GoRouterState state, Widget navigator) {
    return WelcomeShellWidget(navigator: navigator);
  }
}

/// Route for the welcome introduction page.
///
/// This is the entry point of the welcome flow, displayed at `/welcome/introduction`. It shows the initial introduction
/// content to new users.
class WelcomeIntroductionRoute extends GoRouteData with $WelcomeIntroductionRoute {
  /// Builds the welcome introduction page.
  ///
  /// Returns a [WelcomeIntroductionPage] widget that displays the introduction content within the [WelcomeShell]
  /// layout.
  @override
  Widget build(BuildContext context, GoRouterState state) => const Material(child: WelcomeIntroductionPage());
}

/// Route for the welcome login page.
///
/// Displays the login or authentication screen as part of the welcome flow, located at `/welcome/login`. Users reach
/// this page after completing the introduction step.
class WelcomeLoginRoute extends GoRouteData with $WelcomeLoginRoute {
  /// Builds the welcome login page.
  ///
  /// Returns a [WelcomeLoginPage] widget that displays the login interface within the [WelcomeShell] layout.
  @override
  Widget build(BuildContext context, GoRouterState state) => const Material(child: WelcomeLoginPage());
}

/// Route for the welcome registration page.
///
/// Displays the registration screen as part of the welcome flow, located at `/welcome/register`. Users can create a new
/// account by providing email, password, and password confirmation.
class WelcomeRegisterRoute extends GoRouteData with $WelcomeRegisterRoute {
  /// Builds the welcome registration page.
  ///
  /// Returns a [WelcomeRegisterPage] widget that displays the registration form within the [WelcomeShell] layout.
  @override
  Widget build(BuildContext context, GoRouterState state) => const Material(child: WelcomeRegisterPage());
}

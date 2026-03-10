library;

import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';
import 'package:healthy_diet/app/chat/page.dart';
import 'package:healthy_diet/app/shell.dart';
import 'package:healthy_diet/app/home/page.dart';
import 'package:healthy_diet/app/welcome/introduction/page.dart';
import 'package:healthy_diet/app/welcome/login/page.dart';
import 'package:healthy_diet/app/welcome/register/page.dart';
import 'package:healthy_diet/app/welcome/shell.dart';

part 'router.g.dart';

/// Route observer for welcome page navigation events.
///
/// Subscribe via the [RouteAware] mixin to respond to navigation changes.
final routeObserver = RouteObserver<PageRoute>();

/// Main application router.
final router = GoRouter(
  initialLocation: WelcomeIntroductionRoute().location,
  routes: $appRoutes,
  debugLogDiagnostics: true,
);

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
  static final List<NavigatorObserver> $observers = [routeObserver];

  @override
  Widget builder(BuildContext context, GoRouterState state, Widget navigator) {
    return WelcomeShellWidget(navigator: navigator);
  }
}

/// Entry point of the welcome flow at `/welcome/introduction`.
class WelcomeIntroductionRoute extends GoRouteData with $WelcomeIntroductionRoute {
  @override
  Widget build(BuildContext context, GoRouterState state) => const Material(child: WelcomeIntroductionPage());
}

/// Login screen at `/welcome/login`.
class WelcomeLoginRoute extends GoRouteData with $WelcomeLoginRoute {
  @override
  Widget build(BuildContext context, GoRouterState state) => const Material(child: WelcomeLoginPage());
}

/// Registration screen at `/welcome/register`.
class WelcomeRegisterRoute extends GoRouteData with $WelcomeRegisterRoute {
  @override
  Widget build(BuildContext context, GoRouterState state) => const Material(child: WelcomeRegisterPage());
}

@TypedShellRoute<AppShell>(
  routes: [
    TypedGoRoute<HomeRoute>(
      path: '/',
    ),
    TypedGoRoute<ChatRoute>(
      path: '/chat',
    ),
  ],
)
class AppShell extends ShellRouteData {
  static const List<NavigatorObserver> $observers = [];

  @override
  Widget builder(BuildContext context, GoRouterState state, Widget navigator) {
    return AppShellWidget(navigator: navigator);
  }
}

/// Home screen at `/`.
class HomeRoute extends GoRouteData with $HomeRoute {
  @override
  Widget build(BuildContext context, GoRouterState state) => const Material(child: HomePage());
}

/// Chat screen at `/`.
class ChatRoute extends GoRouteData with $ChatRoute {
  @override
  Widget build(BuildContext context, GoRouterState state) => const Material(child: ChatPage());
}
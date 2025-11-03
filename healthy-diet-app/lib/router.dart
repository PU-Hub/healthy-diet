import 'package:flutter/material.dart';

import 'package:go_router/go_router.dart';

import 'package:healthy_diet/app/welcome/page.dart';

part 'router.g.dart';

final router = GoRouter(
  initialLocation: WelcomeRoute().location,
  routes: $appRoutes,
  debugLogDiagnostics: true,
);

@TypedGoRoute<WelcomeRoute>(path: '/welcome')
class WelcomeRoute extends GoRouteData with $WelcomeRoute {
  @override
  Widget build(BuildContext context, GoRouterState state) => WelcomeScreen();
}

/// Welcome introduction screen displayed at app first launch.
///
/// This screen presents the initial welcome content to new users as part of the onboarding flow. It integrates with
/// [WelcomeProvider] to manage navigation state and enable progression to subsequent welcome screens.
library;

import 'package:flutter/material.dart';

import 'package:provider/provider.dart';

import 'package:healthy_diet/app/welcome/provider.dart';
import 'package:healthy_diet/router.dart';
import 'package:healthy_diet/utils/extensions/build_context.dart';
import 'package:healthy_diet/utils/extensions/edge_insets.dart';
import 'package:healthy_diet/widgets/typography.dart';

/// The welcome introduction screen widget.
///
/// Displays introductory content to users when they first launch the app. This is the entry point of the welcome flow
/// and sets up navigation controls through the [WelcomeProvider].
class WelcomeIntroductionPage extends StatefulWidget {
  /// Creates a welcome introduction screen.
  const WelcomeIntroductionPage({super.key});

  @override
  State<WelcomeIntroductionPage> createState() => _WelcomeIntroductionPageState();
}

/// State for [WelcomeIntroductionPage].
///
/// Manages the introduction page lifecycle and navigation configuration. Implements [RouteAware] to respond to route
/// navigation events, ensuring the navigation provider is properly configured when returning to this page.
class _WelcomeIntroductionPageState extends State<WelcomeIntroductionPage> with RouteAware {
  /// Configures the next route navigation.
  ///
  /// Sets up [WelcomeProvider] to navigate to the login page when the user taps the next button. Checks if the context
  /// is mounted before accessing the provider to avoid using disposed widgets.
  void enableNextRoute() {
    if (!context.mounted) return;

    final provider = context.read<WelcomeProvider>();
    provider.setNextRoute(WelcomeLoginRoute());
    provider.setCanNext(true);
  }

  /// Initializes the state and configures navigation.
  ///
  /// Called once when the widget is first created. Sets up the initial navigation configuration for this page.
  @override
  void initState() {
    super.initState();
    enableNextRoute();
  }

  /// Called when returning to this route from a popped route.
  ///
  /// Re-configures navigation when the user navigates back to the introduction page from a subsequent screen. This
  /// ensures the navigation state is properly restored.
  @override
  void didPopNext() {
    enableNextRoute();
  }

  /// Subscribes to route changes using [RouteObserver].
  ///
  /// Called when a dependency of this state object changes. Registers this state with the [routeObserver] to receive
  /// route lifecycle callbacks like [didPopNext].
  @override
  void didChangeDependencies() {
    super.didChangeDependencies();

    final route = ModalRoute.of(context);
    if (route is PageRoute<dynamic>) {
      routeObserver.subscribe(this, route);
    }
  }

  /// Unsubscribes from route changes.
  ///
  /// Called when this state object is permanently removed. Unregisters from the [routeObserver] to prevent memory
  /// leaks.
  @override
  void dispose() {
    routeObserver.unsubscribe(this);
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return ListView(
      padding: context.padding.onlyTop + .symmetric(horizontal: 24, vertical: 16),
      children: [
        HeadLineText.medium('Welcome to the app'),
        SizedBox(height: 16),
        Text(
          "這是一個可以幫你管理飲食的APP,用來記錄你個人狀況和飲食習慣。"
          "\n"
          "APP的功能有：",
          style: TextStyle(fontSize: 16),
        ),
      ],
    );
  }
}

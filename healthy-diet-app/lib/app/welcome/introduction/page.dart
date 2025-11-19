/// Welcome introduction screen displayed at app first launch.
///
/// This screen presents the initial welcome content to new users as part of
/// the onboarding flow. It integrates with [WelcomeProvider] to manage
/// navigation state and enable progression to subsequent welcome screens.
library;

import 'package:flutter/material.dart';
import 'package:flutter/scheduler.dart';
import 'package:healthy_diet/app/welcome/provider.dart';
import 'package:healthy_diet/router.dart';
import 'package:provider/provider.dart';

/// The welcome introduction screen widget.
///
/// Displays introductory content to users when they first launch the app.
/// This is the entry point of the welcome flow and sets up navigation
/// controls through the [WelcomeProvider].
class WelcomeIntroductionScreen extends StatefulWidget {
  /// Creates a welcome introduction screen.
  const WelcomeIntroductionScreen({super.key});

  @override
  State<WelcomeIntroductionScreen> createState() => _WelcomeIntroductionScreenState();
}

/// State for [WelcomeIntroductionScreen].
///
/// Initializes the welcome flow by configuring the navigation provider
/// with the current route and enabling the next button.
class _WelcomeIntroductionScreenState extends State<WelcomeIntroductionScreen> {
  /// Initializes the state and configures navigation.
  ///
  /// Sets up the [WelcomeProvider] after the first frame to ensure the
  /// context is ready. Configures the next route and enables navigation.
  @override
  void initState() {
    super.initState();

    SchedulerBinding.instance.addPostFrameCallback((_) {
      final provider = context.read<WelcomeProvider>();
      provider.setNextRoute(WelcomeIntroductionRoute());
      provider.setCanNext(true);
    });
  }

  @override
  Widget build(BuildContext context) {
    return const Scaffold(
      body: Column(
        children: [
          Text('Welcome to the app'),
        ],
      ),
    );
  }
}

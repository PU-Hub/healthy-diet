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
import 'package:healthy_diet/utils/extensions/build_context.dart';
import 'package:healthy_diet/utils/extensions/edge_insets.dart';
import 'package:healthy_diet/widgets/typography.dart';
import 'package:provider/provider.dart';

/// The welcome introduction screen widget.
///
/// Displays introductory content to users when they first launch the app.
/// This is the entry point of the welcome flow and sets up navigation
/// controls through the [WelcomeProvider].
class WelcomeIntroductionPage extends StatefulWidget {
  /// Creates a welcome introduction screen.
  const WelcomeIntroductionPage({super.key});

  @override
  State<WelcomeIntroductionPage> createState() => _WelcomeIntroductionPageState();
}

/// State for [WelcomeIntroductionPage].
///
/// Initializes the welcome flow by configuring the navigation provider
/// with the current route and enabling the next button.
class _WelcomeIntroductionPageState extends State<WelcomeIntroductionPage> {
  /// Initializes the state and configures navigation.
  ///
  /// Sets up the [WelcomeProvider] after the first frame to ensure the
  /// context is ready. Configures the next route and enables navigation.
  @override
  void initState() {
    super.initState();

    SchedulerBinding.instance.addPostFrameCallback((_) {
      final provider = context.read<WelcomeProvider>();
      provider.setNextRoute(WelcomeLoginRoute());
      provider.setCanNext(true);
    });
  }

  @override
  Widget build(BuildContext context) {
    return ListView(
      padding: context.padding.onlyTop + .symmetric(horizontal: 24, vertical: 16),
      children: [
        HeadLineText.medium('Welcome to the app'),
      ],
    );
  }
}

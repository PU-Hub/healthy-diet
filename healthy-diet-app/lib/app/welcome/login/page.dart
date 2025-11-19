/// Welcome login page for user authentication.
///
/// This page is part of the welcome onboarding flow and handles user
/// authentication or login. It integrates with [WelcomeProvider] to manage
/// navigation state within the welcome flow.
library;

import 'package:flutter/material.dart';
import 'package:flutter/scheduler.dart';
import 'package:healthy_diet/app/welcome/provider.dart';
import 'package:healthy_diet/router.dart';
import 'package:provider/provider.dart';

/// The welcome login page widget.
///
/// Displays login or authentication interface as part of the welcome flow.
/// This page is shown after users complete the introduction screen and
/// allows them to authenticate before proceeding.
class WelcomeLoginPage extends StatefulWidget {
  /// Creates a welcome login page.
  const WelcomeLoginPage({super.key});

  @override
  State<WelcomeLoginPage> createState() => _WelcomeLoginPageState();
}

/// State for [WelcomeLoginPage].
///
/// Manages the login page state and configures the navigation provider
/// with appropriate route information.
class _WelcomeLoginPageState extends State<WelcomeLoginPage> {
  /// Initializes the state and configures navigation.
  ///
  /// Sets up the [WelcomeProvider] after the first frame to ensure the
  /// context is ready. Configures navigation options for this page.
  @override
  void initState() {
    super.initState();

    SchedulerBinding.instance.addPostFrameCallback((_) {
      final provider = context.read<WelcomeProvider>();
      provider.setNextRoute(WelcomeIntroductionRoute());
      provider.setCanNext(false);
    });
  }

  @override
  Widget build(BuildContext context) {
    return ListView(
      padding: .symmetric(horizontal: 16),
      children: [
        Text('Login'),
      ],
    );
  }
}

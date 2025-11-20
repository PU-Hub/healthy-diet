/// Welcome login page for user authentication.
///
/// This page is part of the welcome onboarding flow and handles user authentication or login. It integrates with
/// [WelcomeProvider] to manage navigation state within the welcome flow.
library;

import 'dart:async';

import 'package:flutter/gestures.dart';
import 'package:flutter/material.dart';

import 'package:form_validation/form_validation.dart';
import 'package:material_symbols_icons/symbols.dart';
import 'package:provider/provider.dart';

import 'package:healthy_diet/app/welcome/provider.dart';
import 'package:healthy_diet/router.dart';
import 'package:healthy_diet/utils/extensions/build_context.dart';
import 'package:healthy_diet/utils/extensions/edge_insets.dart';
import 'package:healthy_diet/widgets/typography.dart';

/// The welcome login page widget.
///
/// Displays login or authentication interface as part of the welcome flow. This page is shown after users complete the
/// introduction screen and allows them to authenticate before proceeding.
class WelcomeLoginPage extends StatefulWidget {
  /// Creates a welcome login page.
  const WelcomeLoginPage({super.key});

  @override
  State<WelcomeLoginPage> createState() => _WelcomeLoginPageState();
}

/// State for [WelcomeLoginPage].
///
/// Manages the login page lifecycle and navigation configuration. Implements [RouteAware] to respond to route
/// navigation events, ensuring the navigation state is properly configured when returning to this page.
class _WelcomeLoginPageState extends State<WelcomeLoginPage> with RouteAware {
  /// Form key for validation state management.
  final _formKey = GlobalKey<FormState>();

  /// Controls password field visibility.
  bool _obscurePassword = true;

  /// Text controller for email input field.
  final _emailController = TextEditingController();

  /// Text controller for password input field.
  final _passwordController = TextEditingController();

  /// Configures the navigation state for this page.
  ///
  /// Sets up the [WelcomeProvider] with login-specific navigation behavior:
  /// - Disables the next button initially (enabled via form validation)
  /// - Sets [login] as the callback to execute before navigation
  /// - Checks if the context is mounted before accessing the provider
  void configureNextRoute() {
    if (!context.mounted) return;

    final provider = context.read<WelcomeProvider>();
    provider.setNextRoute(WelcomeIntroductionRoute());
    provider.setNextRouteCallback(login);
    provider.setCanNext(false);
  }

  /// Performs user login with provided credentials.
  ///
  /// Executes when the user taps the next button with valid form data. Currently throws an exception for demonstration.
  /// Should be replaced with actual authentication logic.
  ///
  /// Throws an [Exception] if login fails.
  Future<void> login() async {
    // TODO(kamiya): implement account login logic
    await Future.delayed(const Duration(seconds: 2));
    throw Exception('登入失敗');
  }

  /// Initializes the state and configures navigation.
  ///
  /// Called once when the widget is first created. Sets up the initial navigation configuration, disabling the next
  /// button until login is complete.
  @override
  void initState() {
    super.initState();
    configureNextRoute();
  }

  /// Called when returning to this route from a popped route.
  ///
  /// Re-configures navigation when the user navigates back to the login page from a subsequent screen. This ensures the
  /// navigation state is properly restored with the next button disabled.
  @override
  void didPopNext() {
    configureNextRoute();
  }

  /// Subscribes to route changes using [RouteObserver].
  ///
  /// Called when a dependency of this state object changes. Registers this state with the [routeObserver] to receive
  /// route lifecycle callbacks like [didPopNext].
  @override
  void didChangeDependencies() {
    super.didChangeDependencies();

    final ModalRoute<dynamic>? route = ModalRoute.of(context);
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
        Padding(
          padding: .symmetric(vertical: 32),
          child: HeadLineText.medium('登入', align: .center),
        ),
        Form(
          key: _formKey,
          onChanged: () {
            final email = _emailController.text;
            final password = _passwordController.text;

            final canNext = email.isNotEmpty && password.isNotEmpty && _formKey.currentState?.validate() == true;
            context.read<WelcomeProvider>().setCanNext(canNext);
          },
          child: Selector<WelcomeProvider, bool>(
            selector: (context, provider) => provider.isLoading,
            builder: (context, isLoading, child) => Column(
              spacing: 16,
              children: [
                TextFormField(
                  controller: _emailController,
                  enabled: !isLoading,
                  decoration: InputDecoration(
                    labelText: '電子郵件*',
                    border: OutlineInputBorder(borderRadius: .circular(16)),
                  ),
                  autovalidateMode: .onUnfocus,
                  autocorrect: false,
                  keyboardType: .emailAddress,
                  validator: (value) => Validator(
                    validators: [RequiredValidator(), EmailValidator()],
                  ).validate(label: '電子郵件', value: value),
                ),
                TextFormField(
                  controller: _passwordController,
                  enabled: !isLoading,
                  decoration: InputDecoration(
                    labelText: '密碼*',
                    border: OutlineInputBorder(borderRadius: .circular(16)),
                    suffixIcon: IconButton(
                      icon: Icon(_obscurePassword ? Symbols.visibility_off_rounded : Symbols.visibility_rounded),
                      onPressed: () {
                        setState(() => _obscurePassword = !_obscurePassword);
                      },
                    ),
                  ),
                  autovalidateMode: .onUnfocus,
                  autocorrect: false,
                  keyboardType: .visiblePassword,
                  obscureText: _obscurePassword,
                  validator: (value) => Validator(
                    validators: [RequiredValidator()],
                  ).validate(label: '密碼', value: value),
                ),
              ],
            ),
          ),
        ),
        Padding(
          padding: .symmetric(vertical: 16),
          child: RichText(
            textAlign: .center,
            text: TextSpan(
              style: context.texts.bodyMedium,
              children: [
                TextSpan(text: '還沒有帳號嗎？'),
                TextSpan(
                  text: '註冊',
                  style: TextStyle(color: context.colors.primary),
                  recognizer: TapGestureRecognizer()..onTap = () => WelcomeRegisterRoute().pushReplacement(context),
                ),
              ],
            ),
          ),
        ),
      ],
    );
  }
}

/// Welcome registration page for new user account creation.
///
/// This page is part of the welcome onboarding flow and handles user registration. It integrates with [WelcomeProvider]
/// to manage navigation state and provides form validation for email, password, and password confirmation fields.
library;

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

/// The welcome registration page widget.
///
/// Displays the registration form as part of the welcome flow. Users can create a new account by providing email,
/// password, and password confirmation. The page validates input and provides navigation to the login page.
class WelcomeRegisterPage extends StatefulWidget {
  /// Creates a welcome registration page.
  const WelcomeRegisterPage({super.key});

  @override
  State<WelcomeRegisterPage> createState() => _WelcomeRegisterPageState();
}

/// State for [WelcomeRegisterPage].
///
/// Manages the registration form state, validation, and navigation configuration. Implements [RouteAware] to respond to
/// route navigation events, ensuring the navigation state is properly configured when returning to this page.
class _WelcomeRegisterPageState extends State<WelcomeRegisterPage> with RouteAware {
  /// Form key for validation state management.
  final _formKey = GlobalKey<FormState>();

  /// Controls password field visibility for both password fields.
  bool _obscurePassword = true;

  /// Text controller for email input field.
  final _emailController = TextEditingController();

  /// Text controller for password input field.
  final _passwordController = TextEditingController();

  /// Text controller for password confirmation field.
  final _confirmPasswordController = TextEditingController();

  /// Configures the navigation state for this page.
  ///
  /// Sets up the [WelcomeProvider] with registration-specific navigation behavior:
  /// - Disables the next button initially (enabled via form validation)
  /// - Sets [register] as the callback to execute before navigation
  /// - Checks if the context is mounted before accessing the provider
  void configureNextRoute() {
    if (!context.mounted) return;

    final provider = context.read<WelcomeProvider>();
    provider.setNextRoute(WelcomeIntroductionRoute());
    provider.setCanNext(false);
  }

  /// Performs user registration with provided credentials.
  ///
  /// Executes when the user taps the next button with valid form data. Currently throws an exception for demonstration.
  /// Should be replaced with actual account creation logic.
  ///
  /// Throws an [Exception] if registration fails.
  Future<void> register() async {
    // TODO(kamiya): implement account registration logic
    await Future.delayed(const Duration(seconds: 2));
    throw Exception('註冊失敗');
  }

  /// Initializes the state and configures navigation.
  ///
  /// Called once when the widget is first created. Sets up the initial navigation configuration, disabling the next
  /// button until registration form is valid.
  @override
  void initState() {
    super.initState();
    configureNextRoute();
  }

  /// Called when returning to this route from a popped route.
  ///
  /// Re-configures navigation when the user navigates back to the registration page from a subsequent screen. This
  /// ensures the navigation state is properly restored with the next button disabled.
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

  /// Unsubscribes from route changes and disposes controllers.
  ///
  /// Called when this state object is permanently removed. Unregisters from the [routeObserver] and disposes all text
  /// controllers to prevent memory leaks.
  @override
  void dispose() {
    routeObserver.unsubscribe(this);
    _emailController.dispose();
    _passwordController.dispose();
    _confirmPasswordController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return ListView(
      padding: context.padding.onlyTop + .symmetric(horizontal: 24, vertical: 16),
      children: [
        Padding(
          padding: .symmetric(vertical: 32),
          child: HeadLineText.medium('註冊', align: .center),
        ),
        Form(
          key: _formKey,
          onChanged: () {
            final email = _emailController.text;
            final password = _passwordController.text;
            final confirmPassword = _confirmPasswordController.text;

            final canNext =
                email.isNotEmpty &&
                password.isNotEmpty &&
                confirmPassword.isNotEmpty &&
                _formKey.currentState?.validate() == true;
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
                    validators: [RequiredValidator(), MinLengthValidator(length: 4)],
                  ).validate(label: '密碼', value: value),
                ),
                TextFormField(
                  controller: _confirmPasswordController,
                  enabled: !isLoading,
                  decoration: InputDecoration(
                    labelText: '確認密碼*',
                    border: OutlineInputBorder(borderRadius: .circular(16)),
                    suffixIcon: IconButton(
                      icon: Icon(_obscurePassword ? Symbols.visibility_off_rounded : Symbols.visibility_rounded),
                      onPressed: () => setState(() => _obscurePassword = !_obscurePassword),
                    ),
                  ),
                  autovalidateMode: .onUnfocus,
                  obscureText: _obscurePassword,
                  validator: (value) => Validator(
                    validators: [
                      RequiredValidator(),
                      _LiteralValidator(literal: _passwordController.text, label: '密碼'),
                    ],
                  ).validate(label: '確認密碼', value: value),
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
                TextSpan(text: '已經有帳號嗎？'),
                TextSpan(
                  text: '登入',
                  style: TextStyle(color: context.colors.primary),
                  recognizer: TapGestureRecognizer()..onTap = () => WelcomeLoginRoute().pushReplacement(context),
                ),
              ],
            ),
          ),
        ),
      ],
    );
  }
}

/// Custom validator that checks if a value matches a literal string.
///
/// Used for password confirmation validation, ensuring the confirmation field matches the original password field
/// exactly. Extends [ValueValidator] from the form_validation package.
class _LiteralValidator extends ValueValidator {
  /// The expected literal value to match against.
  final String literal;

  /// The label of the field being compared to (for error messages).
  final String label;

  /// Creates a literal validator.
  ///
  /// The [literal] is the expected value that must be matched. The [label] is used in error messages to identify the
  /// comparison field.
  _LiteralValidator({required this.literal, required this.label});

  /// Validates that the value matches the literal string.
  ///
  /// Returns `null` if the value matches [literal] exactly. Returns an error message if the value doesn't match.
  @override
  String? validate({
    required String label,
    required String? value,
  }) {
    String? error;

    if (value?.isNotEmpty == true) {
      if (value == literal) {
        return null;
      }

      error = '$label do not match with ${this.label}';
    }

    return error;
  }

  /// Serializes the validator to JSON format.
  ///
  /// Returns a map containing the validator type, label, and literal value.
  @override
  Map<String, dynamic> toJson() {
    return {
      'type': type,
      'label': label,
      'literal': literal,
    };
  }

  /// The validator type identifier.
  @override
  String get type => 'literal';
}

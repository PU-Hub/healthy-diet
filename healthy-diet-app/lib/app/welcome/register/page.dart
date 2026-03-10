library;

import 'package:flutter/gestures.dart';
import 'package:flutter/material.dart';
import 'package:form_validation/form_validation.dart';
import 'package:healthy_diet/app/welcome/provider.dart';
import 'package:healthy_diet/router.dart';
import 'package:healthy_diet/utils/extensions/build_context.dart';
import 'package:healthy_diet/utils/extensions/edge_insets.dart';
import 'package:healthy_diet/widgets/typography.dart';
import 'package:material_symbols_icons/symbols.dart';
import 'package:provider/provider.dart';

/// Registration screen for the welcome flow.
class WelcomeRegisterPage extends StatefulWidget {
  const WelcomeRegisterPage({super.key});

  @override
  State<WelcomeRegisterPage> createState() => _WelcomeRegisterPageState();
}

class _WelcomeRegisterPageState extends State<WelcomeRegisterPage> with RouteAware {
  final _formKey = GlobalKey<FormState>();
  bool _obscurePassword = true;
  final _emailController = TextEditingController();
  final _passwordController = TextEditingController();
  final _confirmPasswordController = TextEditingController();

  void configureNextRoute() => WidgetsBinding.instance.addPostFrameCallback((_) {
    if (!context.mounted) return;

    final provider = context.read<WelcomeProvider>();
    provider.setNextRoute(WelcomeIntroductionRoute());
    provider.setCanNext(false);
  });

  Future<void> register() async {
    // TODO(kamiya): implement account registration logic
    await Future.delayed(const Duration(seconds: 2));
    throw Exception('註冊失敗');
  }

  @override
  void initState() {
    super.initState();
    configureNextRoute();
  }

  @override
  void didPopNext() {
    configureNextRoute();
  }

  @override
  void didChangeDependencies() {
    super.didChangeDependencies();

    final ModalRoute<dynamic>? route = ModalRoute.of(context);
    if (route is PageRoute<dynamic>) {
      routeObserver.subscribe(this, route);
    }
  }

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

class _LiteralValidator extends ValueValidator {
  final String literal;
  final String label;

  _LiteralValidator({required this.literal, required this.label});

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

  @override
  Map<String, dynamic> toJson() {
    return {
      'type': type,
      'label': label,
      'literal': literal,
    };
  }

  @override
  String get type => 'literal';
}

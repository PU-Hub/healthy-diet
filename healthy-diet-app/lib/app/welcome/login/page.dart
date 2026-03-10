library;

import 'dart:async';

import 'package:flutter/gestures.dart';
import 'package:flutter/material.dart';
import 'package:form_validation/form_validation.dart';
import 'package:healthy_diet/api/client.dart';
import 'package:healthy_diet/api/models/authentication/login_request_body.dart';
import 'package:healthy_diet/app/provider.dart';
import 'package:healthy_diet/app/welcome/provider.dart';
import 'package:healthy_diet/router.dart';
import 'package:healthy_diet/utils/extensions/build_context.dart';
import 'package:healthy_diet/utils/extensions/edge_insets.dart';
import 'package:healthy_diet/widgets/typography.dart';
import 'package:material_symbols_icons/symbols.dart';
import 'package:provider/provider.dart';

/// Login screen for the welcome flow.
class WelcomeLoginPage extends StatefulWidget {
  const WelcomeLoginPage({super.key});

  @override
  State<WelcomeLoginPage> createState() => _WelcomeLoginPageState();
}

class _WelcomeLoginPageState extends State<WelcomeLoginPage> with RouteAware {
  final _formKey = GlobalKey<FormState>();
  bool _obscurePassword = true;
  final _emailController = TextEditingController();
  final _passwordController = TextEditingController();

  void configureNextRoute() => WidgetsBinding.instance.addPostFrameCallback((_) {
    if (!context.mounted) return;

    final provider = context.read<WelcomeProvider>();
    provider.setNextRoute(HomeRoute());
    provider.setIsLast(true);
    provider.setNextRouteCallback(login);
    provider.setCanNext(false);
  });

  Future<void> login() async {
    final result = await api.login(
      LoginRequestBody(
        email: _emailController.text,
        password: _passwordController.text,
      ),
    );

    if (!mounted) return;

    context.read<AppProvider>().setAuth(result);
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

library;

import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';
import 'package:healthy_diet/app/welcome/provider.dart';
import 'package:healthy_diet/utils/extensions/build_context.dart';
import 'package:healthy_diet/utils/extensions/edge_insets.dart';
import 'package:material_symbols_icons/symbols.dart';
import 'package:provider/provider.dart';

/// Shell layout for welcome screens with back/next navigation controls.
class WelcomeShellWidget extends StatelessWidget {
  const WelcomeShellWidget({
    required this.navigator,
    super.key,
  });

  final Widget navigator;

  @override
  Widget build(BuildContext context) {
    return ChangeNotifierProvider<WelcomeProvider>(
      create: (context) => .new(),
      builder: (context, child) {
        return Scaffold(
          body: child,
          bottomSheet: Material(
            elevation: 8,
            surfaceTintColor: context.colors.surfaceTint,
            shape: RoundedRectangleBorder(
              borderRadius: .vertical(top: .circular(24)),
            ),
            child: Padding(
              padding: context.padding.onlyBottom + .fromLTRB(16, 8, 16, 8),
              child: Consumer<WelcomeProvider>(
                builder: (context, provider, child) {
                  void next() async {
                    final nextRouteCallback = provider.nextRouteCallback;

                    if (nextRouteCallback != null) {
                      provider.setIsLoading(true);
                      try {
                        await nextRouteCallback();
                      } catch (e) {
                        if (!context.mounted) return;
                        context.scaffold.showSnackBar(SnackBar(content: Text(e.toString())));
                        return;
                      } finally {
                        provider.setIsLoading(false);
                      }
                    }

                    if (!context.mounted) return;

                    if (provider.isLast) {
                      provider.nextRoute?.go(context);
                    } else {
                      provider.nextRoute?.push(context);
                    }
                  }

                  return Row(
                    mainAxisAlignment: .spaceBetween,
                    children: [
                      PopScope(
                        canPop: !provider.isLoading && context.canPop(),
                        child: IconButton(
                          icon: const Icon(Symbols.arrow_back_rounded),
                          onPressed: !provider.isLoading && context.canPop() ? () => context.pop() : null,
                        ),
                      ),
                      FilledButton.icon(
                        icon: provider.isLoading
                            ? const SizedBox.square(dimension: 24, child: CircularProgressIndicator(strokeWidth: 2))
                            : const Icon(Symbols.arrow_forward_rounded),
                        label: Text('繼續'),
                        iconAlignment: .end,
                        style: FilledButton.styleFrom(padding: .only(left: 16, right: 16)),
                        onPressed: !provider.isLoading && provider.canNext && provider.nextRoute != null ? next : null,
                      ),
                    ],
                  );
                },
              ),
            ),
          ),
        );
      },
      child: navigator,
    );
  }
}

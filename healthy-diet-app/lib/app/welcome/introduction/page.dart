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
  void configureNextRoute() {
    if (!context.mounted) return;

    final provider = context.read<WelcomeProvider>();
    provider.setNextRoute(WelcomeLoginRoute());
    provider.setNextRouteCallback(null);
    provider.setCanNext(true);
  }

  /// Initializes the state and configures navigation.
  ///
  /// Called once when the widget is first created. Sets up the initial navigation configuration for this page.
  @override
  void initState() {
    super.initState();
    configureNextRoute();
  }

  /// Called when returning to this route from a popped route.
  ///
  /// Re-configures navigation when the user navigates back to the introduction page from a subsequent screen. This
  /// ensures the navigation state is properly restored.
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
        Text(
          "Welcome to the app",
          style: Theme.of(context).textTheme.headlineMedium,
        ),
        const SizedBox(height: 40),
        Text(
          "記錄每日飲食，享受健康生活",
          style: (Theme.of(context).textTheme.titleLarge?.copyWith(
            fontSize: 20,
            fontWeight: FontWeight.w700,
          )),
        ),
        SizedBox(height: 16),
        Text(
          """
    每天三餐是身體能量的來源，但要如何吃得健康、維持好體態卻常讓人感到困惑。
我們的智慧飲食 APP 將成為你的貼身飲食管家，幫助你輕鬆記錄每日飲食，分析熱量與營養，並提供多元功能，陪伴你一步步養成健康的生活習慣。
            """,
          style: (Theme.of(
            context,
          ).textTheme.bodyLarge?.copyWith(fontSize: 15)),
        ),
        SizedBox(height: 16),
        _buildExpandableItem(
          context,
          title: "智慧飲食建議",
          content: """
    拍下你的餐點或輸入內容，AI 立即分析並提供專屬建議，幫助你調整營養比例。
            """,
          activeColor: const Color(0xFF6678FF),
        ),
        _buildExpandableItem(
          context,
          title: "定時提醒",
          content: """
    自由設定提醒時間，指定時段通知你記錄餐點，幫助建立規律的飲食習慣。
            """,
          activeColor: const Color(0xFFFF3333),
        ),
        _buildExpandableItem(
          context,
          title: "飲食報告分析",
          content: """
    透過圖表呈現每日與每週的飲食變化，讓你一眼掌握健康趨勢。
            """,
          activeColor: const Color(0xFF199400),
        ),
        _buildExpandableItem(
          context,
          title: "個人資料管理",
          content: """
    管理你的身高、體重與飲食偏好，系統將提供更精準的健康建議。
            """,
          activeColor: const Color(0xFFAB67AD),
        ),
      ],
    );
  }

  Widget _buildExpandableItem(
    BuildContext context, {
    required String title,
    required String content,
    required Color activeColor,
    String? imagePath,
  }) {
    final isDarkMode = Theme.of(context).brightness == Brightness.dark;

    final finalBackgroundColor = isDarkMode
        ? Color.lerp(activeColor, Colors.black, 0.6)
        : Color.lerp(activeColor, Colors.black, 0.2);

    return Card(
      margin: const EdgeInsets.only(bottom: 12),
      elevation: 2,
      clipBehavior: Clip.antiAlias,
      shape: RoundedRectangleBorder(
        borderRadius: BorderRadiusGeometry.circular(12),
        side: BorderSide(color: Colors.grey.shade200),
      ),
      child: ExpansionTile(
        backgroundColor: finalBackgroundColor,
        collapsedBackgroundColor: isDarkMode ? const Color(0xFF252525) : Colors.white,

        textColor: activeColor,
        collapsedTextColor: isDarkMode ? Colors.black87 : Colors.white,

        iconColor: Colors.white,
        collapsedIconColor: isDarkMode ? Colors.grey[400] : Colors.grey,

        title: Text(
          title,
          style: Theme.of(context).textTheme.titleLarge?.copyWith(
            fontSize: 18,
            fontWeight: FontWeight.w700,
            color: isDarkMode ? const Color.fromARGB(255, 224, 209, 209) : const Color.fromARGB(255, 0, 0, 0),
          ),
        ),
        initiallyExpanded: false,

        children: [
          Padding(
            padding: const EdgeInsets.fromLTRB(16, 0, 16, 16),
            child: Text(
              content,
              style: Theme.of(context).textTheme.bodyLarge?.copyWith(
                fontSize: 15,
                height: 1.5,
                color: isDarkMode ? const Color.fromARGB(255, 255, 255, 255) : Colors.black87,
              ),
            ),
          ),
        ],
      ),
    );
  }
}

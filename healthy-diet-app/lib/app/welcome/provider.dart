/// Provider for managing welcome flow navigation state.
///
/// This provider manages the state and navigation logic for the welcome onboarding screens. It tracks loading states,
/// navigation capabilities, and the current position within the welcome flow.
library;

import 'dart:async';

import 'package:flutter/foundation.dart';

import 'package:go_router/go_router.dart';

/// Base provider implementation with mutable state.
///
/// Contains the private state fields and setter methods for managing the welcome flow. Extended by [WelcomeProvider] to
/// expose read-only getters with computed properties.
sealed class _WelcomeProvider extends ChangeNotifier {
  bool _canNext = true;

  /// Updates whether navigation to the next screen is allowed.
  ///
  /// Notifies listeners when the value changes.
  void setCanNext(bool value) {
    _canNext = value;
    notifyListeners();
  }

  bool _isLast = false;

  /// Updates whether the current screen is the last in the welcome flow.
  ///
  /// Notifies listeners when the value changes.
  void setIsLast(bool value) {
    _isLast = value;
    notifyListeners();
  }

  bool _isLoading = false;

  /// Updates the loading state.
  ///
  /// Used to disable navigation and show loading indicators during asynchronous operations. Notifies listeners when the
  /// value changes.
  void setIsLoading(bool value) {
    _isLoading = value;
    notifyListeners();
  }

  GoRouteData? _nextRoute;

  /// Sets the next route in the welcome flow.
  ///
  /// Updates the target route for the next navigation action. Notifies listeners when the value changes.
  void setNextRoute(GoRouteData value) {
    _nextRoute = value;
    notifyListeners();
  }

  FutureOr<void> Function()? _nextRouteCallback;

  /// Sets an optional callback to execute before navigation.
  ///
  /// This callback is executed when the user taps the next button, before navigating to the next route. Useful for
  /// async operations like form submission, data validation, or API calls. Notifies listeners when set.
  void setNextRouteCallback(FutureOr<void> Function()? callback) {
    _nextRouteCallback = callback;
    notifyListeners();
  }
}

/// Provider for welcome flow state management.
///
/// Manages navigation state during the welcome onboarding process. Provides computed properties that combine multiple
/// state values to determine UI behavior and navigation capabilities.
///
/// Use this provider with [ChangeNotifierProvider] to make welcome flow state available throughout the welcome screen
/// hierarchy.
class WelcomeProvider extends _WelcomeProvider {
  /// Whether navigation to the next screen is allowed.
  ///
  /// Returns `true` only if both the next action is enabled and a next route has been configured.
  bool get canNext => _canNext && _nextRoute != null;

  /// Whether the current screen is the last in the welcome flow.
  bool get isLast => _isLast;

  /// Whether an async operation is in progress.
  ///
  /// When `true`, navigation should be disabled and loading indicators shown.
  bool get isLoading => _isLoading;

  /// The next route in the welcome flow.
  ///
  /// Returns `null` if no next route has been configured.
  GoRouteData? get nextRoute => _nextRoute;

  /// The callback to execute before navigating to the next route.
  ///
  /// Returns `null` if no callback has been configured. When set, this callback is executed before navigation,
  /// typically for async operations like login, registration, or form submission.
  FutureOr<void> Function()? get nextRouteCallback => _nextRouteCallback;
}

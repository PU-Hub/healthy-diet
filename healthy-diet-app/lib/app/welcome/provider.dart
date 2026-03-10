library;

import 'dart:async';

import 'package:flutter/foundation.dart';
import 'package:go_router/go_router.dart';

sealed class _WelcomeProvider extends ChangeNotifier {
  bool _canNext = true;

  void setCanNext(bool value) {
    _canNext = value;
    notifyListeners();
  }

  bool _isLast = false;

  void setIsLast(bool value) {
    _isLast = value;
    notifyListeners();
  }

  bool _isLoading = false;

  void setIsLoading(bool value) {
    _isLoading = value;
    notifyListeners();
  }

  GoRouteData? _nextRoute;

  void setNextRoute(GoRouteData value) {
    _nextRoute = value;
    notifyListeners();
  }

  FutureOr<void> Function()? _nextRouteCallback;

  void setNextRouteCallback(FutureOr<void> Function()? callback) {
    _nextRouteCallback = callback;
    notifyListeners();
  }
}

/// Manages navigation state for the welcome onboarding flow.
///
/// Use with [ChangeNotifierProvider] to expose state to the welcome screen hierarchy.
class WelcomeProvider extends _WelcomeProvider {
  /// `true` only when navigation is enabled and a next route has been set.
  bool get canNext => _canNext && _nextRoute != null;

  bool get isLast => _isLast;

  bool get isLoading => _isLoading;

  GoRouteData? get nextRoute => _nextRoute;

  FutureOr<void> Function()? get nextRouteCallback => _nextRouteCallback;
}

library;

import 'package:flutter/foundation.dart';
import 'package:flutter_secure_storage/flutter_secure_storage.dart';
import 'package:healthy_diet/api/models/authentication/auth_response.dart';
import 'package:healthy_diet/api/models/user/user_detail_response.dart';

const _kAuthKey = 'auth';

/// Global instance shared between the router and the widget tree.
final appProvider = AppProvider();

/// Manages authenticated user state across the application.
class AppProvider extends ChangeNotifier {
  final _storage = const FlutterSecureStorage();

  bool _isLoaded = false;

  /// `false` until [load] completes. The router waits on this before redirecting.
  bool get isLoaded => _isLoaded;

  AuthResponse? _auth;

  AuthResponse? get auth => _auth;

  bool get isAuthenticated => _auth != null;

  UserDetailResponse? _profile;

  UserDetailResponse? get profile => _profile;

  /// Restores persisted auth from secure storage. Call once before [runApp].
  Future<void> load() async {
    final json = await _storage.read(key: _kAuthKey);
    if (json != null) {
      try {
        _auth = AuthResponseMapper.fromJson(json);
      } catch (_) {
        await _storage.delete(key: _kAuthKey);
      }
    }
    _isLoaded = true;
    notifyListeners();
  }

  Future<void> setAuth(AuthResponse auth) async {
    _auth = auth;
    notifyListeners();
    await _storage.write(key: _kAuthKey, value: auth.toJson());
  }


  Future<void> clearAuth() async {
    _auth = null;
    _profile = null;
    notifyListeners();
    await _storage.delete(key: _kAuthKey);
  }

  void setProfile(UserDetailResponse profile) {
    _profile = profile;
    notifyListeners();
  }
}

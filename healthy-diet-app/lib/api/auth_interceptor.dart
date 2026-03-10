library;

import 'package:dio/dio.dart';
import 'package:healthy_diet/api/models/authentication/auth_response.dart';
import 'package:healthy_diet/api/models/authentication/refresh_token_request_body.dart';
import 'package:healthy_diet/app/provider.dart';

const _retryKey = '_retry';

/// Injects the Bearer token on every request and transparently refreshes it
/// on 401 responses. If the refresh token itself is invalid (401/403) the
/// auth is cleared immediately. On network / server errors the refresh is
/// retried up to 3 times before giving up and clearing auth.
class AuthInterceptor extends Interceptor {
  AuthInterceptor(this._dio);

  final Dio _dio;
  bool _isRefreshing = false;
  final _queue = <_PendingRequest>[];

  @override
  void onRequest(RequestOptions options, RequestInterceptorHandler handler) {
    final token = appProvider.auth?.token;
    if (token != null) {
      options.headers['Authorization'] = 'Bearer $token';
    }
    handler.next(options);
  }

  @override
  void onError(DioException err, ErrorInterceptorHandler handler) async {
    // Skip non-401 errors and already-retried requests.
    if (err.response?.statusCode != 401 ||
        err.requestOptions.extra[_retryKey] == true) {
      handler.next(err);
      return;
    }

    final refreshToken = appProvider.auth?.refreshToken;
    if (refreshToken == null) {
      await appProvider.clearAuth();
      handler.next(err);
      return;
    }

    // Queue concurrent 401s while a refresh is in flight.
    if (_isRefreshing) {
      _queue.add(_PendingRequest(err.requestOptions, handler));
      return;
    }

    _isRefreshing = true;
    try {
      final newAuth = await _refreshWithRetry(refreshToken);
      await appProvider.setAuth(newAuth);

      handler.resolve(await _retry(err.requestOptions, newAuth.token));
      for (final pending in _queue) {
        try {
          pending.handler.resolve(await _retry(pending.options, newAuth.token));
        } catch (_) {
          pending.handler.next(err);
        }
      }
    } catch (_) {
      await appProvider.clearAuth();
      handler.next(err);
      for (final pending in _queue) {
        pending.handler.next(err);
      }
    } finally {
      _queue.clear();
      _isRefreshing = false;
    }
  }

  /// Retries the refresh call up to 3 times, but fails immediately on
  /// 401/403 (the refresh token itself is expired or invalid).
  Future<AuthResponse> _refreshWithRetry(String refreshToken) async {
    for (int attempt = 0; attempt < 3; attempt++) {
      try {
        final response = await _dio.post<Map<String, dynamic>>(
          '/auth/refresh',
          data: RefreshTokenRequestBody(refreshToken: refreshToken).toMap(),
          options: Options(extra: {_retryKey: true}),
        );
        return AuthResponseMapper.fromMap(response.data!);
      } on DioException catch (e) {
        final status = e.response?.statusCode;
        if (status == 401 || status == 403) rethrow;
        if (attempt == 2) rethrow;
      }
    }
    throw StateError('unreachable');
  }

  Future<Response<dynamic>> _retry(RequestOptions options, String token) {
    return _dio.request<dynamic>(
      options.path,
      data: options.data,
      queryParameters: options.queryParameters,
      options: Options(
        method: options.method,
        headers: {...options.headers, 'Authorization': 'Bearer $token'},
        extra: {...options.extra, _retryKey: true},
      ),
    );
  }
}

class _PendingRequest {
  _PendingRequest(this.options, this.handler);

  final RequestOptions options;
  final ErrorInterceptorHandler handler;
}

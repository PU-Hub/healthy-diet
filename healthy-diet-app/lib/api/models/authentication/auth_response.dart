import 'package:dart_mappable/dart_mappable.dart';

part 'auth_response.mapper.dart';

@MappableClass()
class AuthResponseUser with AuthResponseUserMappable {
  const AuthResponseUser({
    required this.id,
    required this.email,
    this.nickname,
    this.avatarUrl,
  });

  /// 唯一識別碼
  final String id;

  /// 電子郵件
  final String email;

  /// 暱稱
  final String? nickname;

  /// 頭像圖片連結
  final String? avatarUrl;
}

@MappableClass()
class AuthResponse with AuthResponseMappable {
  const AuthResponse({
    required this.token,
    required this.refreshToken,
    required this.expiresIn,
    required this.user,
  });

  /// JWT 存取權杖
  final String token;

  /// JWT 刷新權杖
  final String refreshToken;

  /// 存取權杖有效秒數
  final int expiresIn;

  /// 使用者資訊
  final AuthResponseUser user;
}

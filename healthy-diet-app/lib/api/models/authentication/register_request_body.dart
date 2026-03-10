import 'package:dart_mappable/dart_mappable.dart';

part 'register_request_body.mapper.dart';

@MappableClass()
class RegisterRequestBody with RegisterRequestBodyMappable {
  const RegisterRequestBody({
    required this.email,
    required this.password,
    this.nickname,
    this.avatarUrl,
  });

  /// 電子郵件
  final String email;

  /// 密碼
  final String password;

  /// 暱稱
  final String? nickname;

  /// 頭像圖片連結
  final String? avatarUrl;
}

import 'package:dart_mappable/dart_mappable.dart';

part 'login_request_body.mapper.dart';

@MappableClass()
class LoginRequestBody with LoginRequestBodyMappable {
  const LoginRequestBody({
    required this.email,
    required this.password,
  });

  /// 電子郵件
  final String email;

  /// 密碼
  final String password;
}

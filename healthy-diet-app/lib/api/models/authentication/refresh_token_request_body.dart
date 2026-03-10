import 'package:dart_mappable/dart_mappable.dart';

part 'refresh_token_request_body.mapper.dart';

@MappableClass()
class RefreshTokenRequestBody with RefreshTokenRequestBodyMappable {
  const RefreshTokenRequestBody({
    required this.refreshToken,
  });

  /// JWT 刷新權杖
  final String refreshToken;
}

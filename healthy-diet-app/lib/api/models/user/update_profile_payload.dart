import 'package:dart_mappable/dart_mappable.dart';

part 'update_profile_payload.mapper.dart';

@MappableClass()
class UpdateProfilePayload with UpdateProfilePayloadMappable {
  const UpdateProfilePayload({
    this.nickname,
    this.height,
    this.weight,
    this.dietaryRestrictions,
  });

  /// 暱稱
  final String? nickname;

  /// 身高（單位：公分）
  final double? height;

  /// 體重（單位：公斤）
  final double? weight;

  /// 飲食禁忌或過敏源
  @MappableField(key: 'dietary_restrictions')
  final String? dietaryRestrictions;
}

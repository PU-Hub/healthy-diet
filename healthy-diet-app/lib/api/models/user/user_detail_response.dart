import 'package:dart_mappable/dart_mappable.dart';

part 'user_detail_response.mapper.dart';

@MappableClass()
class AiConsultationRecord with AiConsultationRecordMappable {
  const AiConsultationRecord({
    required this.id,
    required this.question,
    required this.aiResponse,
    required this.createdAt,
  });

  /// 諮詢紀錄唯一識別碼
  final String id;

  /// 使用者問的問題
  final String question;

  /// AI 的回覆
  final String aiResponse;

  /// 諮詢紀錄建立時間
  final DateTime createdAt;
}

@MappableClass()
class UserDetailResponse with UserDetailResponseMappable {
  const UserDetailResponse({
    required this.id,
    required this.email,
    this.nickname,
    this.avatarUrl,
    required this.height,
    required this.weight,
    this.dietaryRestrictions,
    required this.aiConsultations,
  });

  /// 唯一識別碼
  final String id;

  /// 電子郵件
  final String email;

  /// 暱稱
  final String? nickname;

  /// 頭像圖片連結
  final String? avatarUrl;

  /// 身高（單位：公分）
  final double height;

  /// 體重（單位：公斤）
  final double weight;

  /// 飲食禁忌或過敏源
  final String? dietaryRestrictions;

  /// AI 諮詢紀錄
  final List<AiConsultationRecord> aiConsultations;
}

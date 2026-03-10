import 'package:dio/dio.dart';
import 'package:healthy_diet/api/auth_interceptor.dart';
import 'package:healthy_diet/api/models/authentication/auth_response.dart';
import 'package:healthy_diet/api/models/authentication/login_request_body.dart';
import 'package:healthy_diet/api/models/authentication/refresh_token_request_body.dart';
import 'package:healthy_diet/api/models/authentication/register_request_body.dart';
import 'package:healthy_diet/api/models/consultation/consult_request.dart';
import 'package:healthy_diet/api/models/consultation/consult_response.dart';
import 'package:healthy_diet/api/models/user/update_profile_payload.dart';
import 'package:healthy_diet/api/models/user/user_detail_response.dart';
import 'package:retrofit/retrofit.dart';

part 'client.g.dart';

@RestApi(
  baseUrl: 'http://120.110.113.111:3000/',
  parser: Parser.DartMappable,
)
abstract class RestClient {
  factory RestClient(Dio dio, {String? baseUrl}) = _RestClient;

  /// 驗證使用者身分並返回用於後續 API 請求的存取權杖及使用者資訊。
  //
  // 客戶端應提交有效的電子郵件及密碼憑證。成功驗證後，回應將包含 JWT 存取權杖及使用者基本資料。
  @POST('/auth/login')
  Future<AuthResponse> login(@Body() LoginRequestBody body);

  /// 使用提供的憑證建立新的使用者帳戶。註冊成功後，帳戶即可透過登入端點進行驗證。
  @POST('/auth/register')
  Future<AuthResponse> register(@Body() RegisterRequestBody body);

  /// 使用有效的刷新權杖產生新的存取權杖。
  @POST('/auth/refresh')
  Future<AuthResponse> refreshToken(@Body() RefreshTokenRequestBody body);

  /// 取得使用者的基本資料、身體數值以及最近的 AI 諮詢紀錄。
  @GET('/api/user/profile')
  Future<UserDetailResponse> getProfile();

  /// 更新使用者的暱稱、身高、體重或飲食禁忌。支援部分更新。
  @PUT('/api/user/profile')
  Future<UserDetailResponse> updateProfile(@Body() UpdateProfilePayload profile);

  /// 發送健康或飲食相關問題給 AI。系統會自動讀取該使用者的個人檔案 (身高、體重、飲食禁忌) 並附加在提示詞中，以獲得個人化的建議。對話紀錄會自動儲存至資料庫。
  @POST('/api/consult')
  Future<ConsultResponse> consult(@Body() ConsultRequest request);
}

Dio _createDio() {
  final dio = Dio();
  dio.interceptors.add(AuthInterceptor(dio));
  return dio;
}

final _dio = _createDio();

final api = RestClient(_dio);

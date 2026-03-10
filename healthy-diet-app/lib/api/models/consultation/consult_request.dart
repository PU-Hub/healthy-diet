import 'package:dart_mappable/dart_mappable.dart';

part 'consult_request.mapper.dart';

@MappableClass()
class ConsultRequest with ConsultRequestMappable {
  const ConsultRequest({
    required this.question,
  });

  /// 使用者想問的問題
  final String question;
}

import 'package:dart_mappable/dart_mappable.dart';

part 'consult_response.mapper.dart';

@MappableClass()
class ConsultResponse with ConsultResponseMappable {
  const ConsultResponse({
    required this.replay,
  });

  /// AI 營養師的回覆內容
  final String replay;
}

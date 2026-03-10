/// Extensions on [EdgeInsets] for extracting individual sides.
library;

import 'package:flutter/material.dart';

extension EdgeInsetsExtension on EdgeInsets {
  /// Returns a new [EdgeInsets] with only the bottom value preserved.
  ///
  /// ```dart
  /// context.padding.onlyBottom // EdgeInsets.only(bottom: <system bottom>)
  /// ```
  EdgeInsets get onlyBottom => EdgeInsets.only(bottom: bottom);

  /// Returns a new [EdgeInsets] with only the top value preserved.
  ///
  /// ```dart
  /// context.padding.onlyTop // EdgeInsets.only(top: <system top>)
  /// ```
  EdgeInsets get onlyTop => EdgeInsets.only(top: top);
}

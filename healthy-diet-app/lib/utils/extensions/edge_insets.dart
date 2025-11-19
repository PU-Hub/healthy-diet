/// Extensions on [EdgeInsets] for common transformation operations.
///
/// Provides utility methods for deriving new [EdgeInsets] from existing
/// instances, enabling more concise padding and margin calculations.
library;

import 'package:flutter/material.dart';

/// Extension methods for [EdgeInsets] transformations.
///
/// Adds convenient getters for creating modified versions of existing
/// [EdgeInsets] instances, particularly useful when working with
/// [MediaQuery] padding and combining multiple insets.
extension EdgeInsetsExtension on EdgeInsets {
  /// Creates new [EdgeInsets] with only the bottom value from this instance.
  ///
  /// Returns an [EdgeInsets] where left, top, and right are 0, and bottom
  /// retains the value from this instance. Useful for preserving only
  /// bottom padding from system UI insets.
  ///
  /// Example:
  /// ```dart
  /// final systemPadding = MediaQuery.paddingOf(context); // EdgeInsets.all(20)
  /// final bottomOnly = systemPadding.onlyBottom; // EdgeInsets.only(bottom: 20)
  /// ```
  EdgeInsets get onlyBottom => EdgeInsets.only(bottom: bottom);

  /// Creates new [EdgeInsets] with only the top value from this instance.
  ///
  /// Returns an [EdgeInsets] where left, bottom, and right are 0, and top
  /// retains the value from this instance. Useful for preserving only
  /// top padding from system UI insets.
  ///
  /// Example:
  /// ```dart
  /// final systemPadding = MediaQuery.paddingOf(context); // EdgeInsets.all(20)
  /// final topOnly = systemPadding.onlyTop; // EdgeInsets.only(top: 20)
  /// ```
  EdgeInsets get onlyTop => EdgeInsets.only(top: top);
}

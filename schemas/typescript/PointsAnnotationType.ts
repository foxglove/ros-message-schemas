// Generated by https://github.com/foxglove/schemas

/** Type of points annotation */
export const enum PointsAnnotationType {
  UNKNOWN = 0,

  /** Individual points: 0, 1, 2, ... */
  POINTS = 1,

  /** Closed polygon: 0-1, 1-2, ..., (n-1)-n, n-0 */
  LINE_LOOP = 2,

  /** Connected line segments: 0-1, 1-2, ..., (n-1)-n */
  LINE_STRIP = 3,

  /** Individual line segments: 0-1, 2-3, 4-5, ... */
  LINE_LIST = 4,
}

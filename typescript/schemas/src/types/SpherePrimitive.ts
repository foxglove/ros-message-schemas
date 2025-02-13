// Generated by https://github.com/foxglove/foxglove-sdk
// Options: {}

import { Color } from "./Color";
import { Pose } from "./Pose";
import { Vector3 } from "./Vector3";

/** A primitive representing a sphere or ellipsoid */
export type SpherePrimitive = {
  /** Position of the center of the sphere and orientation of the sphere */
  pose: Pose;

  /** Size (diameter) of the sphere along each axis */
  size: Vector3;

  /** Color of the sphere */
  color: Color;
};

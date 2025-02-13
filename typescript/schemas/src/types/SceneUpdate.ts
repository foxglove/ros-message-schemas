// Generated by https://github.com/foxglove/foxglove-sdk
// Options: {}

import { SceneEntity } from "./SceneEntity";
import { SceneEntityDeletion } from "./SceneEntityDeletion";

/** An update to the entities displayed in a 3D scene */
export type SceneUpdate = {
  /** Scene entities to delete */
  deletions: SceneEntityDeletion[];

  /** Scene entities to add or replace */
  entities: SceneEntity[];
};

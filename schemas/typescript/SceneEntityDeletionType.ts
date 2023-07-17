// Generated by https://github.com/foxglove/schemas

/** An enumeration indicating which entities should match a SceneEntityDeletion command */
export const enum SceneEntityDeletionType {
  /** Delete the existing entity on the same topic that has the provided `id` */
  MATCHING_ID = 0,

  /** Delete all existing entities on the same topic */
  ALL = 1,
}

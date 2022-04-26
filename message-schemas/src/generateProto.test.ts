import { generateProto } from "./generateProto";
import { foxgloveSchemas } from "./schemas";

describe("generateProtoFiles", () => {
  it("generates .proto files", () => {
    expect(generateProto(foxgloveSchemas["foxglove.Markers.LineMarker"]))
      .toMatchInlineSnapshot(`
      "// Generated from foxglove.Markers.LineMarker by @foxglove/message-schemas

      syntax = \\"proto3\\";

      import \\"foxglove/Color.proto\\";
      import \\"foxglove/KeyValuePair.proto\\";
      import \\"foxglove/Pose.proto\\";
      import \\"foxglove/Vector3.proto\\";
      import \\"foxglove/builtins.proto\\";

      package foxglove;

      message foxglove.Markers.LineMarker {
        // Timestamp of the marker
        foxglove.Time timestamp = 1;

        // Frame of reference
        string frame_id = 2;

        // Namespace into which the marker should be grouped. A marker will replace any prior marker on the same topic with the same \`namespace\` and \`id\`.
        string namespace = 3;

        // Identifier for the marker. A marker will replace any prior marker on the same topic with the same \`namespace\` and \`id\`.
        string id = 4;

        // Length of time (relative to \`timestamp\`) after which the marker should be automatically removed. Zero value indicates the marker should remain visible until it is replaced or deleted.
        foxglove.Duration lifetime = 5;

        // Whether the marker should keep its location in the fixed frame (false) or follow the frame specified in \`frame_id\` as it moves relative to the fixed frame (true)
        boolean frame_locked = 6;

        // Additional user-provided metadata associated with the marker. Keys must be unique.
        repeated foxglove.KeyValuePair metadata = 7;

        // Origin of lines relative to reference frame
        foxglove.Pose pose = 8;

        // Line thickness
        double thickness = 9;

        // Indicates whether \`thickness\` is a fixed size in screen pixels (true), or specified in world coordinates and scales with distance from the camera (false)
        boolean scale_invariant = 10;

        // Points along the line
        repeated foxglove.Vector3 points = 11;

        // Solid color to use for the whole line. One of \`color\` or \`colors\` must be provided.
        foxglove.Color color = 12;

        // Per-point colors (if specified, must have the same length as \`points\`). One of \`color\` or \`colors\` must be provided.
        repeated foxglove.Color colors = 13;

        // Indexes into the \`points\` and \`colors\` attribute arrays, which can be used to avoid duplicating attribute data.
        repeated int32 indices = 14;
      }
      "
    `);
  });
});

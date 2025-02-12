//! Definitions for well-known Foxglove schemas
//! Generated by https://github.com/foxglove/schemas
#![allow(clippy::too_many_arguments)]
#![allow(clippy::enum_variant_names)]
#![allow(non_snake_case)]
use pyo3::prelude::*;

/// An enumeration indicating how input points should be interpreted to create lines
#[pyclass(eq, eq_int)]
#[derive(PartialEq, Clone)]
pub(crate) enum LinePrimitiveLineType {
    LineStrip = 0,
    LineLoop = 1,
    LineList = 2,
}

/// Log level
#[pyclass(eq, eq_int)]
#[derive(PartialEq, Clone)]
pub(crate) enum LogLevel {
    Unknown = 0,
    Debug = 1,
    Info = 2,
    Warning = 3,
    Error = 4,
    Fatal = 5,
}

/// An enumeration indicating which entities should match a SceneEntityDeletion command
#[pyclass(eq, eq_int)]
#[derive(PartialEq, Clone)]
pub(crate) enum SceneEntityDeletionType {
    MatchingId = 0,
    All = 1,
}

/// Numeric type
#[pyclass(eq, eq_int)]
#[derive(PartialEq, Clone)]
pub(crate) enum PackedElementFieldNumericType {
    Unknown = 0,
    Uint8 = 1,
    Int8 = 2,
    Uint16 = 3,
    Int16 = 4,
    Uint32 = 5,
    Int32 = 6,
    Float32 = 7,
    Float64 = 8,
}

/// Type of points annotation
#[pyclass(eq, eq_int)]
#[derive(PartialEq, Clone)]
pub(crate) enum PointsAnnotationType {
    Unknown = 0,
    Points = 1,
    LineLoop = 2,
    LineStrip = 3,
    LineList = 4,
}

/// Type of position covariance
#[pyclass(eq, eq_int)]
#[derive(PartialEq, Clone)]
pub(crate) enum LocationFixPositionCovarianceType {
    Unknown = 0,
    Approximated = 1,
    DiagonalKnown = 2,
    Known = 3,
}

#[pyclass]
#[derive(Clone)]
pub struct Timestamp {
    pub seconds: i64,
    pub nanos: i32,
}

#[pymethods]
impl Timestamp {
    #[new]
    fn new(seconds: i64, nanos: i32) -> Self {
        Self { seconds, nanos }
    }
}

impl From<Timestamp> for prost_types::Timestamp {
    fn from(value: Timestamp) -> Self {
        Self {
            seconds: value.seconds,
            nanos: value.nanos,
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub struct Duration {
    pub seconds: u64,
    pub nanos: u32,
}

#[pymethods]
impl Duration {
    #[new]
    fn new(seconds: u64, nanos: u32) -> Self {
        Self { seconds, nanos }
    }
}

impl From<Duration> for prost_types::Duration {
    fn from(value: Duration) -> Self {
        Self {
            seconds: value.seconds.try_into().unwrap_or_default(),
            nanos: value.nanos.try_into().unwrap_or_default(),
        }
    }
}
/// A primitive representing an arrow
#[pyclass]
#[derive(Clone)]
pub(crate) struct ArrowPrimitive(pub(crate) foxglove::schemas::ArrowPrimitive);
#[pymethods]
impl ArrowPrimitive {
    #[new]
    fn new(
        pose: Pose,
        shaft_length: f64,
        shaft_diameter: f64,
        head_length: f64,
        head_diameter: f64,
        color: Color,
    ) -> Self {
        Self(foxglove::schemas::ArrowPrimitive {
            pose: Some(pose.into()),
            shaft_length,
            shaft_diameter,
            head_length,
            head_diameter,
            color: Some(color.into()),
        })
    }
}

impl From<ArrowPrimitive> for foxglove::schemas::ArrowPrimitive {
    fn from(value: ArrowPrimitive) -> Self {
        value.0
    }
}

/// Camera calibration parameters
#[pyclass]
#[derive(Clone)]
pub(crate) struct CameraCalibration(pub(crate) foxglove::schemas::CameraCalibration);
#[pymethods]
impl CameraCalibration {
    #[new]
    fn new(
        timestamp: Timestamp,
        frame_id: String,
        width: u32,
        height: u32,
        distortion_model: String,
        D: Vec<f64>,
        K: Vec<f64>,
        R: Vec<f64>,
        P: Vec<f64>,
    ) -> Self {
        Self(foxglove::schemas::CameraCalibration {
            timestamp: Some(timestamp.into()),
            frame_id,
            width,
            height,
            distortion_model,
            d: D,
            k: K,
            r: R,
            p: P,
        })
    }
}

impl From<CameraCalibration> for foxglove::schemas::CameraCalibration {
    fn from(value: CameraCalibration) -> Self {
        value.0
    }
}

/// A circle annotation on a 2D image
#[pyclass]
#[derive(Clone)]
pub(crate) struct CircleAnnotation(pub(crate) foxglove::schemas::CircleAnnotation);
#[pymethods]
impl CircleAnnotation {
    #[new]
    fn new(
        timestamp: Timestamp,
        position: Point2,
        diameter: f64,
        thickness: f64,
        fill_color: Color,
        outline_color: Color,
    ) -> Self {
        Self(foxglove::schemas::CircleAnnotation {
            timestamp: Some(timestamp.into()),
            position: Some(position.into()),
            diameter,
            thickness,
            fill_color: Some(fill_color.into()),
            outline_color: Some(outline_color.into()),
        })
    }
}

impl From<CircleAnnotation> for foxglove::schemas::CircleAnnotation {
    fn from(value: CircleAnnotation) -> Self {
        value.0
    }
}

/// A color in RGBA format
#[pyclass]
#[derive(Clone)]
pub(crate) struct Color(pub(crate) foxglove::schemas::Color);
#[pymethods]
impl Color {
    #[new]
    fn new(r: f64, g: f64, b: f64, a: f64) -> Self {
        Self(foxglove::schemas::Color { r, g, b, a })
    }
}

impl From<Color> for foxglove::schemas::Color {
    fn from(value: Color) -> Self {
        value.0
    }
}

/// A compressed image
#[pyclass]
#[derive(Clone)]
pub(crate) struct CompressedImage(pub(crate) foxglove::schemas::CompressedImage);
#[pymethods]
impl CompressedImage {
    #[new]
    fn new(timestamp: Timestamp, frame_id: String, data: Vec<u8>, format: String) -> Self {
        Self(foxglove::schemas::CompressedImage {
            timestamp: Some(timestamp.into()),
            frame_id,
            data,
            format,
        })
    }
}

impl From<CompressedImage> for foxglove::schemas::CompressedImage {
    fn from(value: CompressedImage) -> Self {
        value.0
    }
}

/// A single frame of a compressed video bitstream
#[pyclass]
#[derive(Clone)]
pub(crate) struct CompressedVideo(pub(crate) foxglove::schemas::CompressedVideo);
#[pymethods]
impl CompressedVideo {
    #[new]
    fn new(timestamp: Timestamp, frame_id: String, data: Vec<u8>, format: String) -> Self {
        Self(foxglove::schemas::CompressedVideo {
            timestamp: Some(timestamp.into()),
            frame_id,
            data,
            format,
        })
    }
}

impl From<CompressedVideo> for foxglove::schemas::CompressedVideo {
    fn from(value: CompressedVideo) -> Self {
        value.0
    }
}

/// A primitive representing a cylinder, elliptic cylinder, or truncated cone
#[pyclass]
#[derive(Clone)]
pub(crate) struct CylinderPrimitive(pub(crate) foxglove::schemas::CylinderPrimitive);
#[pymethods]
impl CylinderPrimitive {
    #[new]
    fn new(pose: Pose, size: Vector3, bottom_scale: f64, top_scale: f64, color: Color) -> Self {
        Self(foxglove::schemas::CylinderPrimitive {
            pose: Some(pose.into()),
            size: Some(size.into()),
            bottom_scale,
            top_scale,
            color: Some(color.into()),
        })
    }
}

impl From<CylinderPrimitive> for foxglove::schemas::CylinderPrimitive {
    fn from(value: CylinderPrimitive) -> Self {
        value.0
    }
}

/// A primitive representing a cube or rectangular prism
#[pyclass]
#[derive(Clone)]
pub(crate) struct CubePrimitive(pub(crate) foxglove::schemas::CubePrimitive);
#[pymethods]
impl CubePrimitive {
    #[new]
    fn new(pose: Pose, size: Vector3, color: Color) -> Self {
        Self(foxglove::schemas::CubePrimitive {
            pose: Some(pose.into()),
            size: Some(size.into()),
            color: Some(color.into()),
        })
    }
}

impl From<CubePrimitive> for foxglove::schemas::CubePrimitive {
    fn from(value: CubePrimitive) -> Self {
        value.0
    }
}

/// A transform between two reference frames in 3D space
#[pyclass]
#[derive(Clone)]
pub(crate) struct FrameTransform(pub(crate) foxglove::schemas::FrameTransform);
#[pymethods]
impl FrameTransform {
    #[new]
    fn new(
        timestamp: Timestamp,
        parent_frame_id: String,
        child_frame_id: String,
        translation: Vector3,
        rotation: Quaternion,
    ) -> Self {
        Self(foxglove::schemas::FrameTransform {
            timestamp: Some(timestamp.into()),
            parent_frame_id,
            child_frame_id,
            translation: Some(translation.into()),
            rotation: Some(rotation.into()),
        })
    }
}

impl From<FrameTransform> for foxglove::schemas::FrameTransform {
    fn from(value: FrameTransform) -> Self {
        value.0
    }
}

/// An array of FrameTransform messages
#[pyclass]
#[derive(Clone)]
pub(crate) struct FrameTransforms(pub(crate) foxglove::schemas::FrameTransforms);
#[pymethods]
impl FrameTransforms {
    #[new]
    fn new(transforms: Vec<FrameTransform>) -> Self {
        Self(foxglove::schemas::FrameTransforms {
            transforms: transforms.into_iter().map(|x| x.into()).collect(),
        })
    }
}

impl From<FrameTransforms> for foxglove::schemas::FrameTransforms {
    fn from(value: FrameTransforms) -> Self {
        value.0
    }
}

/// GeoJSON data for annotating maps
#[pyclass]
#[derive(Clone)]
pub(crate) struct GeoJson(pub(crate) foxglove::schemas::GeoJson);
#[pymethods]
impl GeoJson {
    #[new]
    fn new(geojson: String) -> Self {
        Self(foxglove::schemas::GeoJson { geojson })
    }
}

impl From<GeoJson> for foxglove::schemas::GeoJson {
    fn from(value: GeoJson) -> Self {
        value.0
    }
}

/// A 2D grid of data
#[pyclass]
#[derive(Clone)]
pub(crate) struct Grid(pub(crate) foxglove::schemas::Grid);
#[pymethods]
impl Grid {
    #[new]
    fn new(
        timestamp: Timestamp,
        frame_id: String,
        pose: Pose,
        column_count: u32,
        cell_size: Vector2,
        row_stride: u32,
        cell_stride: u32,
        fields: Vec<PackedElementField>,
        data: Vec<u8>,
    ) -> Self {
        Self(foxglove::schemas::Grid {
            timestamp: Some(timestamp.into()),
            frame_id,
            pose: Some(pose.into()),
            column_count,
            cell_size: Some(cell_size.into()),
            row_stride,
            cell_stride,
            fields: fields.into_iter().map(|x| x.into()).collect(),
            data,
        })
    }
}

impl From<Grid> for foxglove::schemas::Grid {
    fn from(value: Grid) -> Self {
        value.0
    }
}

/// Array of annotations for a 2D image
#[pyclass]
#[derive(Clone)]
pub(crate) struct ImageAnnotations(pub(crate) foxglove::schemas::ImageAnnotations);
#[pymethods]
impl ImageAnnotations {
    #[new]
    fn new(
        circles: Vec<CircleAnnotation>,
        points: Vec<PointsAnnotation>,
        texts: Vec<TextAnnotation>,
    ) -> Self {
        Self(foxglove::schemas::ImageAnnotations {
            circles: circles.into_iter().map(|x| x.into()).collect(),
            points: points.into_iter().map(|x| x.into()).collect(),
            texts: texts.into_iter().map(|x| x.into()).collect(),
        })
    }
}

impl From<ImageAnnotations> for foxglove::schemas::ImageAnnotations {
    fn from(value: ImageAnnotations) -> Self {
        value.0
    }
}

/// A key with its associated value
#[pyclass]
#[derive(Clone)]
pub(crate) struct KeyValuePair(pub(crate) foxglove::schemas::KeyValuePair);
#[pymethods]
impl KeyValuePair {
    #[new]
    fn new(key: String, value: String) -> Self {
        Self(foxglove::schemas::KeyValuePair { key, value })
    }
}

impl From<KeyValuePair> for foxglove::schemas::KeyValuePair {
    fn from(value: KeyValuePair) -> Self {
        value.0
    }
}

/// A single scan from a planar laser range-finder
#[pyclass]
#[derive(Clone)]
pub(crate) struct LaserScan(pub(crate) foxglove::schemas::LaserScan);
#[pymethods]
impl LaserScan {
    #[new]
    fn new(
        timestamp: Timestamp,
        frame_id: String,
        pose: Pose,
        start_angle: f64,
        end_angle: f64,
        ranges: Vec<f64>,
        intensities: Vec<f64>,
    ) -> Self {
        Self(foxglove::schemas::LaserScan {
            timestamp: Some(timestamp.into()),
            frame_id,
            pose: Some(pose.into()),
            start_angle,
            end_angle,
            ranges,
            intensities,
        })
    }
}

impl From<LaserScan> for foxglove::schemas::LaserScan {
    fn from(value: LaserScan) -> Self {
        value.0
    }
}

/// A primitive representing a series of points connected by lines
#[pyclass]
#[derive(Clone)]
pub(crate) struct LinePrimitive(pub(crate) foxglove::schemas::LinePrimitive);
#[pymethods]
impl LinePrimitive {
    #[new]
    fn new(
        r#type: LinePrimitiveLineType,
        pose: Pose,
        thickness: f64,
        scale_invariant: bool,
        points: Vec<Point3>,
        color: Color,
        colors: Vec<Color>,
        indices: Vec<u32>,
    ) -> Self {
        Self(foxglove::schemas::LinePrimitive {
            r#type: r#type as i32,
            pose: Some(pose.into()),
            thickness,
            scale_invariant,
            points: points.into_iter().map(|x| x.into()).collect(),
            color: Some(color.into()),
            colors: colors.into_iter().map(|x| x.into()).collect(),
            indices,
        })
    }
}

impl From<LinePrimitive> for foxglove::schemas::LinePrimitive {
    fn from(value: LinePrimitive) -> Self {
        value.0
    }
}

/// A navigation satellite fix for any Global Navigation Satellite System
#[pyclass]
#[derive(Clone)]
pub(crate) struct LocationFix(pub(crate) foxglove::schemas::LocationFix);
#[pymethods]
impl LocationFix {
    #[new]
    fn new(
        timestamp: Timestamp,
        frame_id: String,
        latitude: f64,
        longitude: f64,
        altitude: f64,
        position_covariance: Vec<f64>,
        position_covariance_type: LocationFixPositionCovarianceType,
    ) -> Self {
        Self(foxglove::schemas::LocationFix {
            timestamp: Some(timestamp.into()),
            frame_id,
            latitude,
            longitude,
            altitude,
            position_covariance,
            position_covariance_type: position_covariance_type as i32,
        })
    }
}

impl From<LocationFix> for foxglove::schemas::LocationFix {
    fn from(value: LocationFix) -> Self {
        value.0
    }
}

/// A log message
#[pyclass]
#[derive(Clone)]
pub(crate) struct Log(pub(crate) foxglove::schemas::Log);
#[pymethods]
impl Log {
    #[new]
    fn new(
        timestamp: Timestamp,
        level: LogLevel,
        message: String,
        name: String,
        file: String,
        line: u32,
    ) -> Self {
        Self(foxglove::schemas::Log {
            timestamp: Some(timestamp.into()),
            level: level as i32,
            message,
            name,
            file,
            line,
        })
    }
}

impl From<Log> for foxglove::schemas::Log {
    fn from(value: Log) -> Self {
        value.0
    }
}

/// Command to remove previously published entities
#[pyclass]
#[derive(Clone)]
pub(crate) struct SceneEntityDeletion(pub(crate) foxglove::schemas::SceneEntityDeletion);
#[pymethods]
impl SceneEntityDeletion {
    #[new]
    fn new(timestamp: Timestamp, r#type: SceneEntityDeletionType, id: String) -> Self {
        Self(foxglove::schemas::SceneEntityDeletion {
            timestamp: Some(timestamp.into()),
            r#type: r#type as i32,
            id,
        })
    }
}

impl From<SceneEntityDeletion> for foxglove::schemas::SceneEntityDeletion {
    fn from(value: SceneEntityDeletion) -> Self {
        value.0
    }
}

/// A visual element in a 3D scene. An entity may be composed of multiple primitives which all share the same frame of reference.
#[pyclass]
#[derive(Clone)]
pub(crate) struct SceneEntity(pub(crate) foxglove::schemas::SceneEntity);
#[pymethods]
impl SceneEntity {
    #[new]
    fn new(
        timestamp: Timestamp,
        frame_id: String,
        id: String,
        lifetime: Duration,
        frame_locked: bool,
        metadata: Vec<KeyValuePair>,
        arrows: Vec<ArrowPrimitive>,
        cubes: Vec<CubePrimitive>,
        spheres: Vec<SpherePrimitive>,
        cylinders: Vec<CylinderPrimitive>,
        lines: Vec<LinePrimitive>,
        triangles: Vec<TriangleListPrimitive>,
        texts: Vec<TextPrimitive>,
        models: Vec<ModelPrimitive>,
    ) -> Self {
        Self(foxglove::schemas::SceneEntity {
            timestamp: Some(timestamp.into()),
            frame_id,
            id,
            lifetime: Some(lifetime.into()),
            frame_locked,
            metadata: metadata.into_iter().map(|x| x.into()).collect(),
            arrows: arrows.into_iter().map(|x| x.into()).collect(),
            cubes: cubes.into_iter().map(|x| x.into()).collect(),
            spheres: spheres.into_iter().map(|x| x.into()).collect(),
            cylinders: cylinders.into_iter().map(|x| x.into()).collect(),
            lines: lines.into_iter().map(|x| x.into()).collect(),
            triangles: triangles.into_iter().map(|x| x.into()).collect(),
            texts: texts.into_iter().map(|x| x.into()).collect(),
            models: models.into_iter().map(|x| x.into()).collect(),
        })
    }
}

impl From<SceneEntity> for foxglove::schemas::SceneEntity {
    fn from(value: SceneEntity) -> Self {
        value.0
    }
}

/// An update to the entities displayed in a 3D scene
#[pyclass]
#[derive(Clone)]
pub(crate) struct SceneUpdate(pub(crate) foxglove::schemas::SceneUpdate);
#[pymethods]
impl SceneUpdate {
    #[new]
    fn new(deletions: Vec<SceneEntityDeletion>, entities: Vec<SceneEntity>) -> Self {
        Self(foxglove::schemas::SceneUpdate {
            deletions: deletions.into_iter().map(|x| x.into()).collect(),
            entities: entities.into_iter().map(|x| x.into()).collect(),
        })
    }
}

impl From<SceneUpdate> for foxglove::schemas::SceneUpdate {
    fn from(value: SceneUpdate) -> Self {
        value.0
    }
}

/// A primitive representing a 3D model file loaded from an external URL or embedded data
#[pyclass]
#[derive(Clone)]
pub(crate) struct ModelPrimitive(pub(crate) foxglove::schemas::ModelPrimitive);
#[pymethods]
impl ModelPrimitive {
    #[new]
    fn new(
        pose: Pose,
        scale: Vector3,
        color: Color,
        override_color: bool,
        url: String,
        media_type: String,
        data: Vec<u8>,
    ) -> Self {
        Self(foxglove::schemas::ModelPrimitive {
            pose: Some(pose.into()),
            scale: Some(scale.into()),
            color: Some(color.into()),
            override_color,
            url,
            media_type,
            data,
        })
    }
}

impl From<ModelPrimitive> for foxglove::schemas::ModelPrimitive {
    fn from(value: ModelPrimitive) -> Self {
        value.0
    }
}

/// A field present within each element in a byte array of packed elements.
#[pyclass]
#[derive(Clone)]
pub(crate) struct PackedElementField(pub(crate) foxglove::schemas::PackedElementField);
#[pymethods]
impl PackedElementField {
    #[new]
    fn new(name: String, offset: u32, r#type: PackedElementFieldNumericType) -> Self {
        Self(foxglove::schemas::PackedElementField {
            name,
            offset,
            r#type: r#type as i32,
        })
    }
}

impl From<PackedElementField> for foxglove::schemas::PackedElementField {
    fn from(value: PackedElementField) -> Self {
        value.0
    }
}

/// A point representing a position in 2D space
#[pyclass]
#[derive(Clone)]
pub(crate) struct Point2(pub(crate) foxglove::schemas::Point2);
#[pymethods]
impl Point2 {
    #[new]
    fn new(x: f64, y: f64) -> Self {
        Self(foxglove::schemas::Point2 { x, y })
    }
}

impl From<Point2> for foxglove::schemas::Point2 {
    fn from(value: Point2) -> Self {
        value.0
    }
}

/// A point representing a position in 3D space
#[pyclass]
#[derive(Clone)]
pub(crate) struct Point3(pub(crate) foxglove::schemas::Point3);
#[pymethods]
impl Point3 {
    #[new]
    fn new(x: f64, y: f64, z: f64) -> Self {
        Self(foxglove::schemas::Point3 { x, y, z })
    }
}

impl From<Point3> for foxglove::schemas::Point3 {
    fn from(value: Point3) -> Self {
        value.0
    }
}

/// A collection of N-dimensional points, which may contain additional fields with information like normals, intensity, etc.
#[pyclass]
#[derive(Clone)]
pub(crate) struct PointCloud(pub(crate) foxglove::schemas::PointCloud);
#[pymethods]
impl PointCloud {
    #[new]
    fn new(
        timestamp: Timestamp,
        frame_id: String,
        pose: Pose,
        point_stride: u32,
        fields: Vec<PackedElementField>,
        data: Vec<u8>,
    ) -> Self {
        Self(foxglove::schemas::PointCloud {
            timestamp: Some(timestamp.into()),
            frame_id,
            pose: Some(pose.into()),
            point_stride,
            fields: fields.into_iter().map(|x| x.into()).collect(),
            data,
        })
    }
}

impl From<PointCloud> for foxglove::schemas::PointCloud {
    fn from(value: PointCloud) -> Self {
        value.0
    }
}

/// An array of points on a 2D image
#[pyclass]
#[derive(Clone)]
pub(crate) struct PointsAnnotation(pub(crate) foxglove::schemas::PointsAnnotation);
#[pymethods]
impl PointsAnnotation {
    #[new]
    fn new(
        timestamp: Timestamp,
        r#type: PointsAnnotationType,
        points: Vec<Point2>,
        outline_color: Color,
        outline_colors: Vec<Color>,
        fill_color: Color,
        thickness: f64,
    ) -> Self {
        Self(foxglove::schemas::PointsAnnotation {
            timestamp: Some(timestamp.into()),
            r#type: r#type as i32,
            points: points.into_iter().map(|x| x.into()).collect(),
            outline_color: Some(outline_color.into()),
            outline_colors: outline_colors.into_iter().map(|x| x.into()).collect(),
            fill_color: Some(fill_color.into()),
            thickness,
        })
    }
}

impl From<PointsAnnotation> for foxglove::schemas::PointsAnnotation {
    fn from(value: PointsAnnotation) -> Self {
        value.0
    }
}

/// A position and orientation for an object or reference frame in 3D space
#[pyclass]
#[derive(Clone)]
pub(crate) struct Pose(pub(crate) foxglove::schemas::Pose);
#[pymethods]
impl Pose {
    #[new]
    fn new(position: Vector3, orientation: Quaternion) -> Self {
        Self(foxglove::schemas::Pose {
            position: Some(position.into()),
            orientation: Some(orientation.into()),
        })
    }
}

impl From<Pose> for foxglove::schemas::Pose {
    fn from(value: Pose) -> Self {
        value.0
    }
}

/// A timestamped pose for an object or reference frame in 3D space
#[pyclass]
#[derive(Clone)]
pub(crate) struct PoseInFrame(pub(crate) foxglove::schemas::PoseInFrame);
#[pymethods]
impl PoseInFrame {
    #[new]
    fn new(timestamp: Timestamp, frame_id: String, pose: Pose) -> Self {
        Self(foxglove::schemas::PoseInFrame {
            timestamp: Some(timestamp.into()),
            frame_id,
            pose: Some(pose.into()),
        })
    }
}

impl From<PoseInFrame> for foxglove::schemas::PoseInFrame {
    fn from(value: PoseInFrame) -> Self {
        value.0
    }
}

/// An array of timestamped poses for an object or reference frame in 3D space
#[pyclass]
#[derive(Clone)]
pub(crate) struct PosesInFrame(pub(crate) foxglove::schemas::PosesInFrame);
#[pymethods]
impl PosesInFrame {
    #[new]
    fn new(timestamp: Timestamp, frame_id: String, poses: Vec<Pose>) -> Self {
        Self(foxglove::schemas::PosesInFrame {
            timestamp: Some(timestamp.into()),
            frame_id,
            poses: poses.into_iter().map(|x| x.into()).collect(),
        })
    }
}

impl From<PosesInFrame> for foxglove::schemas::PosesInFrame {
    fn from(value: PosesInFrame) -> Self {
        value.0
    }
}

/// A [quaternion](https://eater.net/quaternions) representing a rotation in 3D space
#[pyclass]
#[derive(Clone)]
pub(crate) struct Quaternion(pub(crate) foxglove::schemas::Quaternion);
#[pymethods]
impl Quaternion {
    #[new]
    fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self(foxglove::schemas::Quaternion { x, y, z, w })
    }
}

impl From<Quaternion> for foxglove::schemas::Quaternion {
    fn from(value: Quaternion) -> Self {
        value.0
    }
}

/// A raw image
#[pyclass]
#[derive(Clone)]
pub(crate) struct RawImage(pub(crate) foxglove::schemas::RawImage);
#[pymethods]
impl RawImage {
    #[new]
    fn new(
        timestamp: Timestamp,
        frame_id: String,
        width: u32,
        height: u32,
        encoding: String,
        step: u32,
        data: Vec<u8>,
    ) -> Self {
        Self(foxglove::schemas::RawImage {
            timestamp: Some(timestamp.into()),
            frame_id,
            width,
            height,
            encoding,
            step,
            data,
        })
    }
}

impl From<RawImage> for foxglove::schemas::RawImage {
    fn from(value: RawImage) -> Self {
        value.0
    }
}

/// A primitive representing a sphere or ellipsoid
#[pyclass]
#[derive(Clone)]
pub(crate) struct SpherePrimitive(pub(crate) foxglove::schemas::SpherePrimitive);
#[pymethods]
impl SpherePrimitive {
    #[new]
    fn new(pose: Pose, size: Vector3, color: Color) -> Self {
        Self(foxglove::schemas::SpherePrimitive {
            pose: Some(pose.into()),
            size: Some(size.into()),
            color: Some(color.into()),
        })
    }
}

impl From<SpherePrimitive> for foxglove::schemas::SpherePrimitive {
    fn from(value: SpherePrimitive) -> Self {
        value.0
    }
}

/// A text label on a 2D image
#[pyclass]
#[derive(Clone)]
pub(crate) struct TextAnnotation(pub(crate) foxglove::schemas::TextAnnotation);
#[pymethods]
impl TextAnnotation {
    #[new]
    fn new(
        timestamp: Timestamp,
        position: Point2,
        text: String,
        font_size: f64,
        text_color: Color,
        background_color: Color,
    ) -> Self {
        Self(foxglove::schemas::TextAnnotation {
            timestamp: Some(timestamp.into()),
            position: Some(position.into()),
            text,
            font_size,
            text_color: Some(text_color.into()),
            background_color: Some(background_color.into()),
        })
    }
}

impl From<TextAnnotation> for foxglove::schemas::TextAnnotation {
    fn from(value: TextAnnotation) -> Self {
        value.0
    }
}

/// A primitive representing a text label
#[pyclass]
#[derive(Clone)]
pub(crate) struct TextPrimitive(pub(crate) foxglove::schemas::TextPrimitive);
#[pymethods]
impl TextPrimitive {
    #[new]
    fn new(
        pose: Pose,
        billboard: bool,
        font_size: f64,
        scale_invariant: bool,
        color: Color,
        text: String,
    ) -> Self {
        Self(foxglove::schemas::TextPrimitive {
            pose: Some(pose.into()),
            billboard,
            font_size,
            scale_invariant,
            color: Some(color.into()),
            text,
        })
    }
}

impl From<TextPrimitive> for foxglove::schemas::TextPrimitive {
    fn from(value: TextPrimitive) -> Self {
        value.0
    }
}

/// A primitive representing a set of triangles or a surface tiled by triangles
#[pyclass]
#[derive(Clone)]
pub(crate) struct TriangleListPrimitive(pub(crate) foxglove::schemas::TriangleListPrimitive);
#[pymethods]
impl TriangleListPrimitive {
    #[new]
    fn new(
        pose: Pose,
        points: Vec<Point3>,
        color: Color,
        colors: Vec<Color>,
        indices: Vec<u32>,
    ) -> Self {
        Self(foxglove::schemas::TriangleListPrimitive {
            pose: Some(pose.into()),
            points: points.into_iter().map(|x| x.into()).collect(),
            color: Some(color.into()),
            colors: colors.into_iter().map(|x| x.into()).collect(),
            indices,
        })
    }
}

impl From<TriangleListPrimitive> for foxglove::schemas::TriangleListPrimitive {
    fn from(value: TriangleListPrimitive) -> Self {
        value.0
    }
}

/// A vector in 2D space that represents a direction only
#[pyclass]
#[derive(Clone)]
pub(crate) struct Vector2(pub(crate) foxglove::schemas::Vector2);
#[pymethods]
impl Vector2 {
    #[new]
    fn new(x: f64, y: f64) -> Self {
        Self(foxglove::schemas::Vector2 { x, y })
    }
}

impl From<Vector2> for foxglove::schemas::Vector2 {
    fn from(value: Vector2) -> Self {
        value.0
    }
}

/// A vector in 3D space that represents a direction only
#[pyclass]
#[derive(Clone)]
pub(crate) struct Vector3(pub(crate) foxglove::schemas::Vector3);
#[pymethods]
impl Vector3 {
    #[new]
    fn new(x: f64, y: f64, z: f64) -> Self {
        Self(foxglove::schemas::Vector3 { x, y, z })
    }
}

impl From<Vector3> for foxglove::schemas::Vector3 {
    fn from(value: Vector3) -> Self {
        value.0
    }
}

pub fn register_submodule(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let module = PyModule::new(parent_module.py(), "schemas")?;

    module.add_class::<Duration>()?;
    module.add_class::<Timestamp>()?;
    module.add_class::<LinePrimitiveLineType>()?;
    module.add_class::<LogLevel>()?;
    module.add_class::<SceneEntityDeletionType>()?;
    module.add_class::<PackedElementFieldNumericType>()?;
    module.add_class::<PointsAnnotationType>()?;
    module.add_class::<LocationFixPositionCovarianceType>()?;
    module.add_class::<ArrowPrimitive>()?;
    module.add_class::<CameraCalibration>()?;
    module.add_class::<CircleAnnotation>()?;
    module.add_class::<Color>()?;
    module.add_class::<CompressedImage>()?;
    module.add_class::<CompressedVideo>()?;
    module.add_class::<CylinderPrimitive>()?;
    module.add_class::<CubePrimitive>()?;
    module.add_class::<FrameTransform>()?;
    module.add_class::<FrameTransforms>()?;
    module.add_class::<GeoJson>()?;
    module.add_class::<Grid>()?;
    module.add_class::<ImageAnnotations>()?;
    module.add_class::<KeyValuePair>()?;
    module.add_class::<LaserScan>()?;
    module.add_class::<LinePrimitive>()?;
    module.add_class::<LocationFix>()?;
    module.add_class::<Log>()?;
    module.add_class::<SceneEntityDeletion>()?;
    module.add_class::<SceneEntity>()?;
    module.add_class::<SceneUpdate>()?;
    module.add_class::<ModelPrimitive>()?;
    module.add_class::<PackedElementField>()?;
    module.add_class::<Point2>()?;
    module.add_class::<Point3>()?;
    module.add_class::<PointCloud>()?;
    module.add_class::<PointsAnnotation>()?;
    module.add_class::<Pose>()?;
    module.add_class::<PoseInFrame>()?;
    module.add_class::<PosesInFrame>()?;
    module.add_class::<Quaternion>()?;
    module.add_class::<RawImage>()?;
    module.add_class::<SpherePrimitive>()?;
    module.add_class::<TextAnnotation>()?;
    module.add_class::<TextPrimitive>()?;
    module.add_class::<TriangleListPrimitive>()?;
    module.add_class::<Vector2>()?;
    module.add_class::<Vector3>()?;

    // Define as a package
    // https://github.com/PyO3/pyo3/issues/759
    let py = parent_module.py();
    py.import("sys")?
        .getattr("modules")?
        .set_item("foxglove._foxglove_py.schemas", &module)?;

    parent_module.add_submodule(&module)
}

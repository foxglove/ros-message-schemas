use super::schemas;
use crate::{errors::PyFoxgloveError, PartialMetadata};
use foxglove::TypedChannel;
use pyo3::prelude::*;

pub fn register_submodule(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let module = PyModule::new(parent_module.py(), "channels")?;

    module.add_class::<CameraCalibrationChannel>()?;
    module.add_class::<CircleAnnotationChannel>()?;
    module.add_class::<ColorChannel>()?;
    module.add_class::<CompressedImageChannel>()?;
    module.add_class::<CompressedVideoChannel>()?;
    module.add_class::<FrameTransformChannel>()?;
    module.add_class::<FrameTransformsChannel>()?;
    module.add_class::<GeoJsonChannel>()?;
    module.add_class::<GridChannel>()?;
    module.add_class::<ImageAnnotationsChannel>()?;
    module.add_class::<KeyValuePairChannel>()?;
    module.add_class::<LaserScanChannel>()?;
    module.add_class::<LocationFixChannel>()?;
    module.add_class::<LogChannel>()?;
    module.add_class::<SceneEntityDeletionChannel>()?;
    module.add_class::<SceneEntityChannel>()?;
    module.add_class::<SceneUpdateChannel>()?;
    module.add_class::<PackedElementFieldChannel>()?;
    module.add_class::<Point2Channel>()?;
    module.add_class::<Point3Channel>()?;
    module.add_class::<PointCloudChannel>()?;
    module.add_class::<PointsAnnotationChannel>()?;
    module.add_class::<PoseChannel>()?;
    module.add_class::<PoseInFrameChannel>()?;
    module.add_class::<PosesInFrameChannel>()?;
    module.add_class::<QuaternionChannel>()?;
    module.add_class::<RawImageChannel>()?;
    module.add_class::<TextAnnotationChannel>()?;
    module.add_class::<Vector2Channel>()?;
    module.add_class::<Vector3Channel>()?;

    // Define as a package
    // https://github.com/PyO3/pyo3/issues/759
    let py = parent_module.py();
    py.import("sys")?
        .getattr("modules")?
        .set_item("foxglove._foxglove_py.channels", &module)?;

    parent_module.add_submodule(&module)
}

#[pyclass(module = "foxglove.channels")]
struct CameraCalibrationChannel(TypedChannel<foxglove::schemas::CameraCalibration>);

#[pymethods]
impl CameraCalibrationChannel {
    #[new]
    fn new(topic: &str) -> PyResult<Self> {
        let base = TypedChannel::new(topic).map_err(PyFoxgloveError::from)?;
        Ok(Self(base))
    }

    fn log(&self, msg: &schemas::CameraCalibration) {
        self.0.log(&msg.0);
    }

    fn log_with_meta(
        &self,
        msg: &schemas::CameraCalibration,
        metadata: Bound<'_, PartialMetadata>,
    ) {
        let metadata = metadata
            .extract::<PartialMetadata>()
            .ok()
            .unwrap_or_default();
        self.0.log_with_meta(&msg.0, metadata.into());
    }

    fn __repr__(&self) -> String {
        format!("CameraCalibrationChannel(topic='{}')", self.0.topic()).to_string()
    }
}

#[pyclass(module = "foxglove.channels")]
struct CircleAnnotationChannel(TypedChannel<foxglove::schemas::CircleAnnotation>);

#[pymethods]
impl CircleAnnotationChannel {
    #[new]
    fn new(topic: &str) -> PyResult<Self> {
        let base = TypedChannel::new(topic).map_err(PyFoxgloveError::from)?;
        Ok(Self(base))
    }

    fn log(&self, msg: &schemas::CircleAnnotation) {
        self.0.log(&msg.0);
    }

    fn log_with_meta(&self, msg: &schemas::CircleAnnotation, metadata: Bound<'_, PartialMetadata>) {
        let metadata = metadata
            .extract::<PartialMetadata>()
            .ok()
            .unwrap_or_default();
        self.0.log_with_meta(&msg.0, metadata.into());
    }

    fn __repr__(&self) -> String {
        format!("CircleAnnotationChannel(topic='{}')", self.0.topic()).to_string()
    }
}

#[pyclass(module = "foxglove.channels")]
struct ColorChannel(TypedChannel<foxglove::schemas::Color>);

#[pymethods]
impl ColorChannel {
    #[new]
    fn new(topic: &str) -> PyResult<Self> {
        let base = TypedChannel::new(topic).map_err(PyFoxgloveError::from)?;
        Ok(Self(base))
    }

    fn log(&self, msg: &schemas::Color) {
        self.0.log(&msg.0);
    }

    fn log_with_meta(&self, msg: &schemas::Color, metadata: Bound<'_, PartialMetadata>) {
        let metadata = metadata
            .extract::<PartialMetadata>()
            .ok()
            .unwrap_or_default();
        self.0.log_with_meta(&msg.0, metadata.into());
    }

    fn __repr__(&self) -> String {
        format!("ColorChannel(topic='{}')", self.0.topic()).to_string()
    }
}

#[pyclass(module = "foxglove.channels")]
struct CompressedImageChannel(TypedChannel<foxglove::schemas::CompressedImage>);

#[pymethods]
impl CompressedImageChannel {
    #[new]
    fn new(topic: &str) -> PyResult<Self> {
        let base = TypedChannel::new(topic).map_err(PyFoxgloveError::from)?;
        Ok(Self(base))
    }

    fn log(&self, msg: &schemas::CompressedImage) {
        self.0.log(&msg.0);
    }

    fn log_with_meta(&self, msg: &schemas::CompressedImage, metadata: Bound<'_, PartialMetadata>) {
        let metadata = metadata
            .extract::<PartialMetadata>()
            .ok()
            .unwrap_or_default();
        self.0.log_with_meta(&msg.0, metadata.into());
    }

    fn __repr__(&self) -> String {
        format!("CompressedImageChannel(topic='{}')", self.0.topic()).to_string()
    }
}

#[pyclass(module = "foxglove.channels")]
struct CompressedVideoChannel(TypedChannel<foxglove::schemas::CompressedVideo>);

#[pymethods]
impl CompressedVideoChannel {
    #[new]
    fn new(topic: &str) -> PyResult<Self> {
        let base = TypedChannel::new(topic).map_err(PyFoxgloveError::from)?;
        Ok(Self(base))
    }

    fn log(&self, msg: &schemas::CompressedVideo) {
        self.0.log(&msg.0);
    }

    fn log_with_meta(&self, msg: &schemas::CompressedVideo, metadata: Bound<'_, PartialMetadata>) {
        let metadata = metadata
            .extract::<PartialMetadata>()
            .ok()
            .unwrap_or_default();
        self.0.log_with_meta(&msg.0, metadata.into());
    }

    fn __repr__(&self) -> String {
        format!("CompressedVideoChannel(topic='{}')", self.0.topic()).to_string()
    }
}

#[pyclass(module = "foxglove.channels")]
struct FrameTransformChannel(TypedChannel<foxglove::schemas::FrameTransform>);

#[pymethods]
impl FrameTransformChannel {
    #[new]
    fn new(topic: &str) -> PyResult<Self> {
        let base = TypedChannel::new(topic).map_err(PyFoxgloveError::from)?;
        Ok(Self(base))
    }

    fn log(&self, msg: &schemas::FrameTransform) {
        self.0.log(&msg.0);
    }

    fn log_with_meta(&self, msg: &schemas::FrameTransform, metadata: Bound<'_, PartialMetadata>) {
        let metadata = metadata
            .extract::<PartialMetadata>()
            .ok()
            .unwrap_or_default();
        self.0.log_with_meta(&msg.0, metadata.into());
    }

    fn __repr__(&self) -> String {
        format!("FrameTransformChannel(topic='{}')", self.0.topic()).to_string()
    }
}

#[pyclass(module = "foxglove.channels")]
struct FrameTransformsChannel(TypedChannel<foxglove::schemas::FrameTransforms>);

#[pymethods]
impl FrameTransformsChannel {
    #[new]
    fn new(topic: &str) -> PyResult<Self> {
        let base = TypedChannel::new(topic).map_err(PyFoxgloveError::from)?;
        Ok(Self(base))
    }

    fn log(&self, msg: &schemas::FrameTransforms) {
        self.0.log(&msg.0);
    }

    fn log_with_meta(&self, msg: &schemas::FrameTransforms, metadata: Bound<'_, PartialMetadata>) {
        let metadata = metadata
            .extract::<PartialMetadata>()
            .ok()
            .unwrap_or_default();
        self.0.log_with_meta(&msg.0, metadata.into());
    }

    fn __repr__(&self) -> String {
        format!("FrameTransformsChannel(topic='{}')", self.0.topic()).to_string()
    }
}

#[pyclass(module = "foxglove.channels")]
struct GeoJsonChannel(TypedChannel<foxglove::schemas::GeoJson>);

#[pymethods]
impl GeoJsonChannel {
    #[new]
    fn new(topic: &str) -> PyResult<Self> {
        let base = TypedChannel::new(topic).map_err(PyFoxgloveError::from)?;
        Ok(Self(base))
    }

    fn log(&self, msg: &schemas::GeoJson) {
        self.0.log(&msg.0);
    }

    fn log_with_meta(&self, msg: &schemas::GeoJson, metadata: Bound<'_, PartialMetadata>) {
        let metadata = metadata
            .extract::<PartialMetadata>()
            .ok()
            .unwrap_or_default();
        self.0.log_with_meta(&msg.0, metadata.into());
    }

    fn __repr__(&self) -> String {
        format!("GeoJsonChannel(topic='{}')", self.0.topic()).to_string()
    }
}

#[pyclass(module = "foxglove.channels")]
struct GridChannel(TypedChannel<foxglove::schemas::Grid>);

#[pymethods]
impl GridChannel {
    #[new]
    fn new(topic: &str) -> PyResult<Self> {
        let base = TypedChannel::new(topic).map_err(PyFoxgloveError::from)?;
        Ok(Self(base))
    }

    fn log(&self, msg: &schemas::Grid) {
        self.0.log(&msg.0);
    }

    fn log_with_meta(&self, msg: &schemas::Grid, metadata: Bound<'_, PartialMetadata>) {
        let metadata = metadata
            .extract::<PartialMetadata>()
            .ok()
            .unwrap_or_default();
        self.0.log_with_meta(&msg.0, metadata.into());
    }

    fn __repr__(&self) -> String {
        format!("GridChannel(topic='{}')", self.0.topic()).to_string()
    }
}

#[pyclass(module = "foxglove.channels")]
struct ImageAnnotationsChannel(TypedChannel<foxglove::schemas::ImageAnnotations>);

#[pymethods]
impl ImageAnnotationsChannel {
    #[new]
    fn new(topic: &str) -> PyResult<Self> {
        let base = TypedChannel::new(topic).map_err(PyFoxgloveError::from)?;
        Ok(Self(base))
    }

    fn log(&self, msg: &schemas::ImageAnnotations) {
        self.0.log(&msg.0);
    }

    fn log_with_meta(&self, msg: &schemas::ImageAnnotations, metadata: Bound<'_, PartialMetadata>) {
        let metadata = metadata
            .extract::<PartialMetadata>()
            .ok()
            .unwrap_or_default();
        self.0.log_with_meta(&msg.0, metadata.into());
    }

    fn __repr__(&self) -> String {
        format!("ImageAnnotationsChannel(topic='{}')", self.0.topic()).to_string()
    }
}

#[pyclass(module = "foxglove.channels")]
struct KeyValuePairChannel(TypedChannel<foxglove::schemas::KeyValuePair>);

#[pymethods]
impl KeyValuePairChannel {
    #[new]
    fn new(topic: &str) -> PyResult<Self> {
        let base = TypedChannel::new(topic).map_err(PyFoxgloveError::from)?;
        Ok(Self(base))
    }

    fn log(&self, msg: &schemas::KeyValuePair) {
        self.0.log(&msg.0);
    }

    fn log_with_meta(&self, msg: &schemas::KeyValuePair, metadata: Bound<'_, PartialMetadata>) {
        let metadata = metadata
            .extract::<PartialMetadata>()
            .ok()
            .unwrap_or_default();
        self.0.log_with_meta(&msg.0, metadata.into());
    }

    fn __repr__(&self) -> String {
        format!("KeyValuePairChannel(topic='{}')", self.0.topic()).to_string()
    }
}

#[pyclass(module = "foxglove.channels")]
struct LaserScanChannel(TypedChannel<foxglove::schemas::LaserScan>);

#[pymethods]
impl LaserScanChannel {
    #[new]
    fn new(topic: &str) -> PyResult<Self> {
        let base = TypedChannel::new(topic).map_err(PyFoxgloveError::from)?;
        Ok(Self(base))
    }

    fn log(&self, msg: &schemas::LaserScan) {
        self.0.log(&msg.0);
    }

    fn log_with_meta(&self, msg: &schemas::LaserScan, metadata: Bound<'_, PartialMetadata>) {
        let metadata = metadata
            .extract::<PartialMetadata>()
            .ok()
            .unwrap_or_default();
        self.0.log_with_meta(&msg.0, metadata.into());
    }

    fn __repr__(&self) -> String {
        format!("LaserScanChannel(topic='{}')", self.0.topic()).to_string()
    }
}

#[pyclass(module = "foxglove.channels")]
struct LocationFixChannel(TypedChannel<foxglove::schemas::LocationFix>);

#[pymethods]
impl LocationFixChannel {
    #[new]
    fn new(topic: &str) -> PyResult<Self> {
        let base = TypedChannel::new(topic).map_err(PyFoxgloveError::from)?;
        Ok(Self(base))
    }

    fn log(&self, msg: &schemas::LocationFix) {
        self.0.log(&msg.0);
    }

    fn log_with_meta(&self, msg: &schemas::LocationFix, metadata: Bound<'_, PartialMetadata>) {
        let metadata = metadata
            .extract::<PartialMetadata>()
            .ok()
            .unwrap_or_default();
        self.0.log_with_meta(&msg.0, metadata.into());
    }

    fn __repr__(&self) -> String {
        format!("LocationFixChannel(topic='{}')", self.0.topic()).to_string()
    }
}

#[pyclass(module = "foxglove.channels")]
struct LogChannel(TypedChannel<foxglove::schemas::Log>);

#[pymethods]
impl LogChannel {
    #[new]
    fn new(topic: &str) -> PyResult<Self> {
        let base = TypedChannel::new(topic).map_err(PyFoxgloveError::from)?;
        Ok(Self(base))
    }

    fn log(&self, msg: &schemas::Log) {
        self.0.log(&msg.0);
    }

    fn log_with_meta(&self, msg: &schemas::Log, metadata: Bound<'_, PartialMetadata>) {
        let metadata = metadata
            .extract::<PartialMetadata>()
            .ok()
            .unwrap_or_default();
        self.0.log_with_meta(&msg.0, metadata.into());
    }

    fn __repr__(&self) -> String {
        format!("LogChannel(topic='{}')", self.0.topic()).to_string()
    }
}

#[pyclass(module = "foxglove.channels")]
struct SceneEntityDeletionChannel(TypedChannel<foxglove::schemas::SceneEntityDeletion>);

#[pymethods]
impl SceneEntityDeletionChannel {
    #[new]
    fn new(topic: &str) -> PyResult<Self> {
        let base = TypedChannel::new(topic).map_err(PyFoxgloveError::from)?;
        Ok(Self(base))
    }

    fn log(&self, msg: &schemas::SceneEntityDeletion) {
        self.0.log(&msg.0);
    }

    fn log_with_meta(
        &self,
        msg: &schemas::SceneEntityDeletion,
        metadata: Bound<'_, PartialMetadata>,
    ) {
        let metadata = metadata
            .extract::<PartialMetadata>()
            .ok()
            .unwrap_or_default();
        self.0.log_with_meta(&msg.0, metadata.into());
    }

    fn __repr__(&self) -> String {
        format!("SceneEntityDeletionChannel(topic='{}')", self.0.topic()).to_string()
    }
}

#[pyclass(module = "foxglove.channels")]
struct SceneEntityChannel(TypedChannel<foxglove::schemas::SceneEntity>);

#[pymethods]
impl SceneEntityChannel {
    #[new]
    fn new(topic: &str) -> PyResult<Self> {
        let base = TypedChannel::new(topic).map_err(PyFoxgloveError::from)?;
        Ok(Self(base))
    }

    fn log(&self, msg: &schemas::SceneEntity) {
        self.0.log(&msg.0);
    }

    fn log_with_meta(&self, msg: &schemas::SceneEntity, metadata: Bound<'_, PartialMetadata>) {
        let metadata = metadata
            .extract::<PartialMetadata>()
            .ok()
            .unwrap_or_default();
        self.0.log_with_meta(&msg.0, metadata.into());
    }

    fn __repr__(&self) -> String {
        format!("SceneEntityChannel(topic='{}')", self.0.topic()).to_string()
    }
}

#[pyclass(module = "foxglove.channels")]
struct SceneUpdateChannel(TypedChannel<foxglove::schemas::SceneUpdate>);

#[pymethods]
impl SceneUpdateChannel {
    #[new]
    fn new(topic: &str) -> PyResult<Self> {
        let base = TypedChannel::new(topic).map_err(PyFoxgloveError::from)?;
        Ok(Self(base))
    }

    fn log(&self, msg: &schemas::SceneUpdate) {
        self.0.log(&msg.0);
    }

    fn log_with_meta(&self, msg: &schemas::SceneUpdate, metadata: Bound<'_, PartialMetadata>) {
        let metadata = metadata
            .extract::<PartialMetadata>()
            .ok()
            .unwrap_or_default();
        self.0.log_with_meta(&msg.0, metadata.into());
    }

    fn __repr__(&self) -> String {
        format!("SceneUpdateChannel(topic='{}')", self.0.topic()).to_string()
    }
}

#[pyclass(module = "foxglove.channels")]
struct PackedElementFieldChannel(TypedChannel<foxglove::schemas::PackedElementField>);

#[pymethods]
impl PackedElementFieldChannel {
    #[new]
    fn new(topic: &str) -> PyResult<Self> {
        let base = TypedChannel::new(topic).map_err(PyFoxgloveError::from)?;
        Ok(Self(base))
    }

    fn log(&self, msg: &schemas::PackedElementField) {
        self.0.log(&msg.0);
    }

    fn log_with_meta(
        &self,
        msg: &schemas::PackedElementField,
        metadata: Bound<'_, PartialMetadata>,
    ) {
        let metadata = metadata
            .extract::<PartialMetadata>()
            .ok()
            .unwrap_or_default();
        self.0.log_with_meta(&msg.0, metadata.into());
    }

    fn __repr__(&self) -> String {
        format!("PackedElementFieldChannel(topic='{}')", self.0.topic()).to_string()
    }
}

#[pyclass(module = "foxglove.channels")]
struct Point2Channel(TypedChannel<foxglove::schemas::Point2>);

#[pymethods]
impl Point2Channel {
    #[new]
    fn new(topic: &str) -> PyResult<Self> {
        let base = TypedChannel::new(topic).map_err(PyFoxgloveError::from)?;
        Ok(Self(base))
    }

    fn log(&self, msg: &schemas::Point2) {
        self.0.log(&msg.0);
    }

    fn log_with_meta(&self, msg: &schemas::Point2, metadata: Bound<'_, PartialMetadata>) {
        let metadata = metadata
            .extract::<PartialMetadata>()
            .ok()
            .unwrap_or_default();
        self.0.log_with_meta(&msg.0, metadata.into());
    }

    fn __repr__(&self) -> String {
        format!("Point2Channel(topic='{}')", self.0.topic()).to_string()
    }
}

#[pyclass(module = "foxglove.channels")]
struct Point3Channel(TypedChannel<foxglove::schemas::Point3>);

#[pymethods]
impl Point3Channel {
    #[new]
    fn new(topic: &str) -> PyResult<Self> {
        let base = TypedChannel::new(topic).map_err(PyFoxgloveError::from)?;
        Ok(Self(base))
    }

    fn log(&self, msg: &schemas::Point3) {
        self.0.log(&msg.0);
    }

    fn log_with_meta(&self, msg: &schemas::Point3, metadata: Bound<'_, PartialMetadata>) {
        let metadata = metadata
            .extract::<PartialMetadata>()
            .ok()
            .unwrap_or_default();
        self.0.log_with_meta(&msg.0, metadata.into());
    }

    fn __repr__(&self) -> String {
        format!("Point3Channel(topic='{}')", self.0.topic()).to_string()
    }
}

#[pyclass(module = "foxglove.channels")]
struct PointCloudChannel(TypedChannel<foxglove::schemas::PointCloud>);

#[pymethods]
impl PointCloudChannel {
    #[new]
    fn new(topic: &str) -> PyResult<Self> {
        let base = TypedChannel::new(topic).map_err(PyFoxgloveError::from)?;
        Ok(Self(base))
    }

    fn log(&self, msg: &schemas::PointCloud) {
        self.0.log(&msg.0);
    }

    fn log_with_meta(&self, msg: &schemas::PointCloud, metadata: Bound<'_, PartialMetadata>) {
        let metadata = metadata
            .extract::<PartialMetadata>()
            .ok()
            .unwrap_or_default();
        self.0.log_with_meta(&msg.0, metadata.into());
    }

    fn __repr__(&self) -> String {
        format!("PointCloudChannel(topic='{}')", self.0.topic()).to_string()
    }
}

#[pyclass(module = "foxglove.channels")]
struct PointsAnnotationChannel(TypedChannel<foxglove::schemas::PointsAnnotation>);

#[pymethods]
impl PointsAnnotationChannel {
    #[new]
    fn new(topic: &str) -> PyResult<Self> {
        let base = TypedChannel::new(topic).map_err(PyFoxgloveError::from)?;
        Ok(Self(base))
    }

    fn log(&self, msg: &schemas::PointsAnnotation) {
        self.0.log(&msg.0);
    }

    fn log_with_meta(&self, msg: &schemas::PointsAnnotation, metadata: Bound<'_, PartialMetadata>) {
        let metadata = metadata
            .extract::<PartialMetadata>()
            .ok()
            .unwrap_or_default();
        self.0.log_with_meta(&msg.0, metadata.into());
    }

    fn __repr__(&self) -> String {
        format!("PointsAnnotationChannel(topic='{}')", self.0.topic()).to_string()
    }
}

#[pyclass(module = "foxglove.channels")]
struct PoseChannel(TypedChannel<foxglove::schemas::Pose>);

#[pymethods]
impl PoseChannel {
    #[new]
    fn new(topic: &str) -> PyResult<Self> {
        let base = TypedChannel::new(topic).map_err(PyFoxgloveError::from)?;
        Ok(Self(base))
    }

    fn log(&self, msg: &schemas::Pose) {
        self.0.log(&msg.0);
    }

    fn log_with_meta(&self, msg: &schemas::Pose, metadata: Bound<'_, PartialMetadata>) {
        let metadata = metadata
            .extract::<PartialMetadata>()
            .ok()
            .unwrap_or_default();
        self.0.log_with_meta(&msg.0, metadata.into());
    }

    fn __repr__(&self) -> String {
        format!("PoseChannel(topic='{}')", self.0.topic()).to_string()
    }
}

#[pyclass(module = "foxglove.channels")]
struct PoseInFrameChannel(TypedChannel<foxglove::schemas::PoseInFrame>);

#[pymethods]
impl PoseInFrameChannel {
    #[new]
    fn new(topic: &str) -> PyResult<Self> {
        let base = TypedChannel::new(topic).map_err(PyFoxgloveError::from)?;
        Ok(Self(base))
    }

    fn log(&self, msg: &schemas::PoseInFrame) {
        self.0.log(&msg.0);
    }

    fn log_with_meta(&self, msg: &schemas::PoseInFrame, metadata: Bound<'_, PartialMetadata>) {
        let metadata = metadata
            .extract::<PartialMetadata>()
            .ok()
            .unwrap_or_default();
        self.0.log_with_meta(&msg.0, metadata.into());
    }

    fn __repr__(&self) -> String {
        format!("PoseInFrameChannel(topic='{}')", self.0.topic()).to_string()
    }
}

#[pyclass(module = "foxglove.channels")]
struct PosesInFrameChannel(TypedChannel<foxglove::schemas::PosesInFrame>);

#[pymethods]
impl PosesInFrameChannel {
    #[new]
    fn new(topic: &str) -> PyResult<Self> {
        let base = TypedChannel::new(topic).map_err(PyFoxgloveError::from)?;
        Ok(Self(base))
    }

    fn log(&self, msg: &schemas::PosesInFrame) {
        self.0.log(&msg.0);
    }

    fn log_with_meta(&self, msg: &schemas::PosesInFrame, metadata: Bound<'_, PartialMetadata>) {
        let metadata = metadata
            .extract::<PartialMetadata>()
            .ok()
            .unwrap_or_default();
        self.0.log_with_meta(&msg.0, metadata.into());
    }

    fn __repr__(&self) -> String {
        format!("PosesInFrameChannel(topic='{}')", self.0.topic()).to_string()
    }
}

#[pyclass(module = "foxglove.channels")]
struct QuaternionChannel(TypedChannel<foxglove::schemas::Quaternion>);

#[pymethods]
impl QuaternionChannel {
    #[new]
    fn new(topic: &str) -> PyResult<Self> {
        let base = TypedChannel::new(topic).map_err(PyFoxgloveError::from)?;
        Ok(Self(base))
    }

    fn log(&self, msg: &schemas::Quaternion) {
        self.0.log(&msg.0);
    }

    fn log_with_meta(&self, msg: &schemas::Quaternion, metadata: Bound<'_, PartialMetadata>) {
        let metadata = metadata
            .extract::<PartialMetadata>()
            .ok()
            .unwrap_or_default();
        self.0.log_with_meta(&msg.0, metadata.into());
    }

    fn __repr__(&self) -> String {
        format!("QuaternionChannel(topic='{}')", self.0.topic()).to_string()
    }
}

#[pyclass(module = "foxglove.channels")]
struct RawImageChannel(TypedChannel<foxglove::schemas::RawImage>);

#[pymethods]
impl RawImageChannel {
    #[new]
    fn new(topic: &str) -> PyResult<Self> {
        let base = TypedChannel::new(topic).map_err(PyFoxgloveError::from)?;
        Ok(Self(base))
    }

    fn log(&self, msg: &schemas::RawImage) {
        self.0.log(&msg.0);
    }

    fn log_with_meta(&self, msg: &schemas::RawImage, metadata: Bound<'_, PartialMetadata>) {
        let metadata = metadata
            .extract::<PartialMetadata>()
            .ok()
            .unwrap_or_default();
        self.0.log_with_meta(&msg.0, metadata.into());
    }

    fn __repr__(&self) -> String {
        format!("RawImageChannel(topic='{}')", self.0.topic()).to_string()
    }
}

#[pyclass(module = "foxglove.channels")]
struct TextAnnotationChannel(TypedChannel<foxglove::schemas::TextAnnotation>);

#[pymethods]
impl TextAnnotationChannel {
    #[new]
    fn new(topic: &str) -> PyResult<Self> {
        let base = TypedChannel::new(topic).map_err(PyFoxgloveError::from)?;
        Ok(Self(base))
    }

    fn log(&self, msg: &schemas::TextAnnotation) {
        self.0.log(&msg.0);
    }

    fn log_with_meta(&self, msg: &schemas::TextAnnotation, metadata: Bound<'_, PartialMetadata>) {
        let metadata = metadata
            .extract::<PartialMetadata>()
            .ok()
            .unwrap_or_default();
        self.0.log_with_meta(&msg.0, metadata.into());
    }

    fn __repr__(&self) -> String {
        format!("TextAnnotationChannel(topic='{}')", self.0.topic()).to_string()
    }
}

#[pyclass(module = "foxglove.channels")]
struct Vector2Channel(TypedChannel<foxglove::schemas::Vector2>);

#[pymethods]
impl Vector2Channel {
    #[new]
    fn new(topic: &str) -> PyResult<Self> {
        let base = TypedChannel::new(topic).map_err(PyFoxgloveError::from)?;
        Ok(Self(base))
    }

    fn log(&self, msg: &schemas::Vector2) {
        self.0.log(&msg.0);
    }

    fn log_with_meta(&self, msg: &schemas::Vector2, metadata: Bound<'_, PartialMetadata>) {
        let metadata = metadata
            .extract::<PartialMetadata>()
            .ok()
            .unwrap_or_default();
        self.0.log_with_meta(&msg.0, metadata.into());
    }

    fn __repr__(&self) -> String {
        format!("Vector2Channel(topic='{}')", self.0.topic()).to_string()
    }
}

#[pyclass(module = "foxglove.channels")]
struct Vector3Channel(TypedChannel<foxglove::schemas::Vector3>);

#[pymethods]
impl Vector3Channel {
    #[new]
    fn new(topic: &str) -> PyResult<Self> {
        let base = TypedChannel::new(topic).map_err(PyFoxgloveError::from)?;
        Ok(Self(base))
    }

    fn log(&self, msg: &schemas::Vector3) {
        self.0.log(&msg.0);
    }

    fn log_with_meta(&self, msg: &schemas::Vector3, metadata: Bound<'_, PartialMetadata>) {
        let metadata = metadata
            .extract::<PartialMetadata>()
            .ok()
            .unwrap_or_default();
        self.0.log_with_meta(&msg.0, metadata.into());
    }

    fn __repr__(&self) -> String {
        format!("Vector3Channel(topic='{}')", self.0.topic()).to_string()
    }
}

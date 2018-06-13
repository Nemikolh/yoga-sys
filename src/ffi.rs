/* automatically generated by rust-bindgen */

pub type va_list = __builtin_va_list;
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum YGAlign {
    YGAlignAuto = 0,
    YGAlignFlexStart = 1,
    YGAlignCenter = 2,
    YGAlignFlexEnd = 3,
    YGAlignStretch = 4,
    YGAlignBaseline = 5,
    YGAlignSpaceBetween = 6,
    YGAlignSpaceAround = 7,
}
extern "C" {
    #[link_name = "\u{1}_YGAlignToString"]
    pub fn YGAlignToString(value: YGAlign) -> *const ::std::os::raw::c_char;
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum YGDimension {
    YGDimensionWidth = 0,
    YGDimensionHeight = 1,
}
extern "C" {
    #[link_name = "\u{1}_YGDimensionToString"]
    pub fn YGDimensionToString(value: YGDimension) -> *const ::std::os::raw::c_char;
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum YGDirection {
    YGDirectionInherit = 0,
    YGDirectionLTR = 1,
    YGDirectionRTL = 2,
}
extern "C" {
    #[link_name = "\u{1}_YGDirectionToString"]
    pub fn YGDirectionToString(value: YGDirection) -> *const ::std::os::raw::c_char;
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum YGDisplay {
    YGDisplayFlex = 0,
    YGDisplayNone = 1,
}
extern "C" {
    #[link_name = "\u{1}_YGDisplayToString"]
    pub fn YGDisplayToString(value: YGDisplay) -> *const ::std::os::raw::c_char;
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum YGEdge {
    YGEdgeLeft = 0,
    YGEdgeTop = 1,
    YGEdgeRight = 2,
    YGEdgeBottom = 3,
    YGEdgeStart = 4,
    YGEdgeEnd = 5,
    YGEdgeHorizontal = 6,
    YGEdgeVertical = 7,
    YGEdgeAll = 8,
}
extern "C" {
    #[link_name = "\u{1}_YGEdgeToString"]
    pub fn YGEdgeToString(value: YGEdge) -> *const ::std::os::raw::c_char;
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum YGExperimentalFeature {
    YGExperimentalFeatureWebFlexBasis = 0,
}
extern "C" {
    #[link_name = "\u{1}_YGExperimentalFeatureToString"]
    pub fn YGExperimentalFeatureToString(
        value: YGExperimentalFeature,
    ) -> *const ::std::os::raw::c_char;
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum YGFlexDirection {
    YGFlexDirectionColumn = 0,
    YGFlexDirectionColumnReverse = 1,
    YGFlexDirectionRow = 2,
    YGFlexDirectionRowReverse = 3,
}
extern "C" {
    #[link_name = "\u{1}_YGFlexDirectionToString"]
    pub fn YGFlexDirectionToString(value: YGFlexDirection) -> *const ::std::os::raw::c_char;
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum YGJustify {
    YGJustifyFlexStart = 0,
    YGJustifyCenter = 1,
    YGJustifyFlexEnd = 2,
    YGJustifySpaceBetween = 3,
    YGJustifySpaceAround = 4,
    YGJustifySpaceEvenly = 5,
}
extern "C" {
    #[link_name = "\u{1}_YGJustifyToString"]
    pub fn YGJustifyToString(value: YGJustify) -> *const ::std::os::raw::c_char;
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum YGLogLevel {
    YGLogLevelError = 0,
    YGLogLevelWarn = 1,
    YGLogLevelInfo = 2,
    YGLogLevelDebug = 3,
    YGLogLevelVerbose = 4,
    YGLogLevelFatal = 5,
}
extern "C" {
    #[link_name = "\u{1}_YGLogLevelToString"]
    pub fn YGLogLevelToString(value: YGLogLevel) -> *const ::std::os::raw::c_char;
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum YGMeasureMode {
    YGMeasureModeUndefined = 0,
    YGMeasureModeExactly = 1,
    YGMeasureModeAtMost = 2,
}
extern "C" {
    #[link_name = "\u{1}_YGMeasureModeToString"]
    pub fn YGMeasureModeToString(value: YGMeasureMode) -> *const ::std::os::raw::c_char;
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum YGNodeType {
    YGNodeTypeDefault = 0,
    YGNodeTypeText = 1,
}
extern "C" {
    #[link_name = "\u{1}_YGNodeTypeToString"]
    pub fn YGNodeTypeToString(value: YGNodeType) -> *const ::std::os::raw::c_char;
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum YGOverflow {
    YGOverflowVisible = 0,
    YGOverflowHidden = 1,
    YGOverflowScroll = 2,
}
extern "C" {
    #[link_name = "\u{1}_YGOverflowToString"]
    pub fn YGOverflowToString(value: YGOverflow) -> *const ::std::os::raw::c_char;
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum YGPositionType {
    YGPositionTypeRelative = 0,
    YGPositionTypeAbsolute = 1,
}
extern "C" {
    #[link_name = "\u{1}_YGPositionTypeToString"]
    pub fn YGPositionTypeToString(value: YGPositionType) -> *const ::std::os::raw::c_char;
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum YGPrintOptions {
    YGPrintOptionsLayout = 1,
    YGPrintOptionsStyle = 2,
    YGPrintOptionsChildren = 4,
}
extern "C" {
    #[link_name = "\u{1}_YGPrintOptionsToString"]
    pub fn YGPrintOptionsToString(value: YGPrintOptions) -> *const ::std::os::raw::c_char;
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum YGUnit {
    YGUnitUndefined = 0,
    YGUnitPoint = 1,
    YGUnitPercent = 2,
    YGUnitAuto = 3,
}
extern "C" {
    #[link_name = "\u{1}_YGUnitToString"]
    pub fn YGUnitToString(value: YGUnit) -> *const ::std::os::raw::c_char;
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum YGWrap {
    YGWrapNoWrap = 0,
    YGWrapWrap = 1,
    YGWrapWrapReverse = 2,
}
extern "C" {
    #[link_name = "\u{1}_YGWrapToString"]
    pub fn YGWrapToString(value: YGWrap) -> *const ::std::os::raw::c_char;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct YGSize {
    pub width: f32,
    pub height: f32,
}
#[test]
fn bindgen_test_layout_YGSize() {
    assert_eq!(
        ::std::mem::size_of::<YGSize>(),
        8usize,
        concat!("Size of: ", stringify!(YGSize))
    );
    assert_eq!(
        ::std::mem::align_of::<YGSize>(),
        4usize,
        concat!("Alignment of ", stringify!(YGSize))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<YGSize>())).width as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(YGSize),
            "::",
            stringify!(width)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<YGSize>())).height as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(YGSize),
            "::",
            stringify!(height)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct YGValue {
    pub value: f32,
    pub unit: YGUnit,
}
#[test]
fn bindgen_test_layout_YGValue() {
    assert_eq!(
        ::std::mem::size_of::<YGValue>(),
        8usize,
        concat!("Size of: ", stringify!(YGValue))
    );
    assert_eq!(
        ::std::mem::align_of::<YGValue>(),
        4usize,
        concat!("Alignment of ", stringify!(YGValue))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<YGValue>())).value as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(YGValue),
            "::",
            stringify!(value)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<YGValue>())).unit as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(YGValue),
            "::",
            stringify!(unit)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct YGConfig {
    _unused: [u8; 0],
}
pub type YGConfigRef = *mut YGConfig;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct YGNode {
    _unused: [u8; 0],
}
pub type YGNodeRef = *mut YGNode;
pub type YGMeasureFunc = ::std::option::Option<
    unsafe extern "C" fn(
        node: YGNodeRef,
        width: f32,
        widthMode: YGMeasureMode,
        height: f32,
        heightMode: YGMeasureMode,
    ) -> YGSize,
>;
pub type YGBaselineFunc =
    ::std::option::Option<unsafe extern "C" fn(node: YGNodeRef, width: f32, height: f32) -> f32>;
pub type YGDirtiedFunc = ::std::option::Option<unsafe extern "C" fn(node: YGNodeRef)>;
pub type YGPrintFunc = ::std::option::Option<unsafe extern "C" fn(node: YGNodeRef)>;
pub type YGLogger = ::std::option::Option<
    unsafe extern "C" fn(
        config: YGConfigRef,
        node: YGNodeRef,
        level: YGLogLevel,
        format: *const ::std::os::raw::c_char,
        args: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int,
>;
pub type YGCloneNodeFunc = ::std::option::Option<
    unsafe extern "C" fn(oldNode: YGNodeRef, owner: YGNodeRef, childIndex: ::std::os::raw::c_int)
        -> YGNodeRef,
>;
extern "C" {
    #[link_name = "\u{1}_YGNodeNew"]
    pub fn YGNodeNew() -> YGNodeRef;
}
extern "C" {
    #[link_name = "\u{1}_YGNodeNewWithConfig"]
    pub fn YGNodeNewWithConfig(config: YGConfigRef) -> YGNodeRef;
}
extern "C" {
    #[link_name = "\u{1}_YGNodeClone"]
    pub fn YGNodeClone(node: YGNodeRef) -> YGNodeRef;
}
extern "C" {
    #[link_name = "\u{1}_YGNodeFree"]
    pub fn YGNodeFree(node: YGNodeRef);
}
extern "C" {
    #[link_name = "\u{1}_YGNodeFreeRecursive"]
    pub fn YGNodeFreeRecursive(node: YGNodeRef);
}
extern "C" {
    #[link_name = "\u{1}_YGNodeReset"]
    pub fn YGNodeReset(node: YGNodeRef);
}
extern "C" {
    #[link_name = "\u{1}_YGNodeGetInstanceCount"]
    pub fn YGNodeGetInstanceCount() -> i32;
}
extern "C" {
    #[link_name = "\u{1}_YGNodeInsertChild"]
    pub fn YGNodeInsertChild(node: YGNodeRef, child: YGNodeRef, index: u32);
}
extern "C" {
    #[link_name = "\u{1}_YGNodeInsertSharedChild"]
    pub fn YGNodeInsertSharedChild(node: YGNodeRef, child: YGNodeRef, index: u32);
}
extern "C" {
    #[link_name = "\u{1}_YGNodeRemoveChild"]
    pub fn YGNodeRemoveChild(node: YGNodeRef, child: YGNodeRef);
}
extern "C" {
    #[link_name = "\u{1}_YGNodeRemoveAllChildren"]
    pub fn YGNodeRemoveAllChildren(node: YGNodeRef);
}
extern "C" {
    #[link_name = "\u{1}_YGNodeGetChild"]
    pub fn YGNodeGetChild(node: YGNodeRef, index: u32) -> YGNodeRef;
}
extern "C" {
    #[link_name = "\u{1}_YGNodeGetOwner"]
    pub fn YGNodeGetOwner(node: YGNodeRef) -> YGNodeRef;
}
extern "C" {
    #[link_name = "\u{1}_YGNodeGetParent"]
    pub fn YGNodeGetParent(node: YGNodeRef) -> YGNodeRef;
}
extern "C" {
    #[link_name = "\u{1}_YGNodeGetChildCount"]
    pub fn YGNodeGetChildCount(node: YGNodeRef) -> u32;
}
extern "C" {
    #[link_name = "\u{1}_YGNodeSetChildren"]
    pub fn YGNodeSetChildren(owner: YGNodeRef, children: *const YGNodeRef, count: u32);
}
extern "C" {
    #[link_name = "\u{1}_YGNodeCalculateLayout"]
    pub fn YGNodeCalculateLayout(
        node: YGNodeRef,
        availableWidth: f32,
        availableHeight: f32,
        ownerDirection: YGDirection,
    );
}
extern "C" {
    #[link_name = "\u{1}_YGNodeMarkDirty"]
    pub fn YGNodeMarkDirty(node: YGNodeRef);
}
extern "C" {
    #[link_name = "\u{1}_YGNodeMarkDirtyAndPropogateToDescendants"]
    pub fn YGNodeMarkDirtyAndPropogateToDescendants(node: YGNodeRef);
}
extern "C" {
    #[link_name = "\u{1}_YGNodePrint"]
    pub fn YGNodePrint(node: YGNodeRef, options: YGPrintOptions);
}
extern "C" {
    #[link_name = "\u{1}_YGFloatIsUndefined"]
    pub fn YGFloatIsUndefined(value: f32) -> bool;
}
extern "C" {
    #[link_name = "\u{1}_YGNodeCanUseCachedMeasurement"]
    pub fn YGNodeCanUseCachedMeasurement(
        widthMode: YGMeasureMode,
        width: f32,
        heightMode: YGMeasureMode,
        height: f32,
        lastWidthMode: YGMeasureMode,
        lastWidth: f32,
        lastHeightMode: YGMeasureMode,
        lastHeight: f32,
        lastComputedWidth: f32,
        lastComputedHeight: f32,
        marginRow: f32,
        marginColumn: f32,
        config: YGConfigRef,
    ) -> bool;
}
extern "C" {
    #[link_name = "\u{1}_YGNodeCopyStyle"]
    pub fn YGNodeCopyStyle(dstNode: YGNodeRef, srcNode: YGNodeRef);
}
extern "C" {
    #[link_name = "\u{1}_YGNodeGetContext"]
    pub fn YGNodeGetContext(node: YGNodeRef) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    #[link_name = "\u{1}_YGNodeSetContext"]
    pub fn YGNodeSetContext(node: YGNodeRef, context: *mut ::std::os::raw::c_void);
}
extern "C" {
    #[link_name = "\u{1}_YGNodeGetMeasureFunc"]
    pub fn YGNodeGetMeasureFunc(node: YGNodeRef) -> YGMeasureFunc;
}
extern "C" {
    #[link_name = "\u{1}_YGNodeSetMeasureFunc"]
    pub fn YGNodeSetMeasureFunc(node: YGNodeRef, measureFunc: YGMeasureFunc);
}
extern "C" {
    #[link_name = "\u{1}_YGNodeGetBaselineFunc"]
    pub fn YGNodeGetBaselineFunc(node: YGNodeRef) -> YGBaselineFunc;
}
extern "C" {
    #[link_name = "\u{1}_YGNodeSetBaselineFunc"]
    pub fn YGNodeSetBaselineFunc(node: YGNodeRef, baselineFunc: YGBaselineFunc);
}
extern "C" {
    #[link_name = "\u{1}_YGNodeGetDirtiedFunc"]
    pub fn YGNodeGetDirtiedFunc(node: YGNodeRef) -> YGDirtiedFunc;
}
extern "C" {
    #[link_name = "\u{1}_YGNodeSetDirtiedFunc"]
    pub fn YGNodeSetDirtiedFunc(node: YGNodeRef, dirtiedFunc: YGDirtiedFunc);
}
extern "C" {
    #[link_name = "\u{1}_YGNodeGetPrintFunc"]
    pub fn YGNodeGetPrintFunc(node: YGNodeRef) -> YGPrintFunc;
}
extern "C" {
    #[link_name = "\u{1}_YGNodeSetPrintFunc"]
    pub fn YGNodeSetPrintFunc(node: YGNodeRef, printFunc: YGPrintFunc);
}
extern "C" {
    #[link_name = "\u{1}_YGNodeGetHasNewLayout"]
    pub fn YGNodeGetHasNewLayout(node: YGNodeRef) -> bool;
}
extern "C" {
    #[link_name = "\u{1}_YGNodeSetHasNewLayout"]
    pub fn YGNodeSetHasNewLayout(node: YGNodeRef, hasNewLayout: bool);
}
extern "C" {
    #[link_name = "\u{1}_YGNodeGetNodeType"]
    pub fn YGNodeGetNodeType(node: YGNodeRef) -> YGNodeType;
}
extern "C" {
    #[link_name = "\u{1}_YGNodeSetNodeType"]
    pub fn YGNodeSetNodeType(node: YGNodeRef, nodeType: YGNodeType);
}
extern "C" {
    #[link_name = "\u{1}_YGNodeIsDirty"]
    pub fn YGNodeIsDirty(node: YGNodeRef) -> bool;
}
extern "C" {
    #[link_name = "\u{1}_YGNodeLayoutGetDidUseLegacyFlag"]
    pub fn YGNodeLayoutGetDidUseLegacyFlag(node: YGNodeRef) -> bool;
}
extern "C" {
    #[link_name = "\u{1}_YGNodeStyleSetDirection"]
    pub fn YGNodeStyleSetDirection(node: YGNodeRef, direction: YGDirection);
}
extern "C" {
    #[link_name = "\u{1}_YGNodeStyleGetDirection"]
    pub fn YGNodeStyleGetDirection(node: YGNodeRef) -> YGDirection;
}
extern "C" {
    #[link_name = "\u{1}_YGNodeStyleSetFlexDirection"]
    pub fn YGNodeStyleSetFlexDirection(node: YGNodeRef, flexDirection: YGFlexDirection);
}
extern "C" {
    #[link_name = "\u{1}_YGNodeStyleGetFlexDirection"]
    pub fn YGNodeStyleGetFlexDirection(node: YGNodeRef) -> YGFlexDirection;
}
extern "C" {
    #[link_name = "\u{1}_YGNodeStyleSetJustifyContent"]
    pub fn YGNodeStyleSetJustifyContent(node: YGNodeRef, justifyContent: YGJustify);
}
extern "C" {
    #[link_name = "\u{1}_YGNodeStyleGetJustifyContent"]
    pub fn YGNodeStyleGetJustifyContent(node: YGNodeRef) -> YGJustify;
}
extern "C" {
    #[link_name = "\u{1}_YGNodeStyleSetAlignContent"]
    pub fn YGNodeStyleSetAlignContent(node: YGNodeRef, alignContent: YGAlign);
}
extern "C" {
    #[link_name = "\u{1}_YGNodeStyleGetAlignContent"]
    pub fn YGNodeStyleGetAlignContent(node: YGNodeRef) -> YGAlign;
}
extern "C" {
    #[link_name = "\u{1}_YGNodeStyleSetAlignItems"]
    pub fn YGNodeStyleSetAlignItems(node: YGNodeRef, alignItems: YGAlign);
}
extern "C" {
    #[link_name = "\u{1}_YGNodeStyleGetAlignItems"]
    pub fn YGNodeStyleGetAlignItems(node: YGNodeRef) -> YGAlign;
}
extern "C" {
    #[link_name = "\u{1}_YGNodeStyleSetAlignSelf"]
    pub fn YGNodeStyleSetAlignSelf(node: YGNodeRef, alignSelf: YGAlign);
}
extern "C" {
    #[link_name = "\u{1}_YGNodeStyleGetAlignSelf"]
    pub fn YGNodeStyleGetAlignSelf(node: YGNodeRef) -> YGAlign;
}
extern "C" {
    #[link_name = "\u{1}_YGNodeStyleSetPositionType"]
    pub fn YGNodeStyleSetPositionType(node: YGNodeRef, positionType: YGPositionType);
}
extern "C" {
    #[link_name = "\u{1}_YGNodeStyleGetPositionType"]
    pub fn YGNodeStyleGetPositionType(node: YGNodeRef) -> YGPositionType;
}
extern "C" {
    #[link_name = "\u{1}_YGNodeStyleSetFlexWrap"]
    pub fn YGNodeStyleSetFlexWrap(node: YGNodeRef, flexWrap: YGWrap);
}
extern "C" {
    #[link_name = "\u{1}_YGNodeStyleGetFlexWrap"]
    pub fn YGNodeStyleGetFlexWrap(node: YGNodeRef) -> YGWrap;
}
extern "C" {
    #[link_name = "\u{1}_YGNodeStyleSetOverflow"]
    pub fn YGNodeStyleSetOverflow(node: YGNodeRef, overflow: YGOverflow);
}
extern "C" {
    #[link_name = "\u{1}_YGNodeStyleGetOverflow"]
    pub fn YGNodeStyleGetOverflow(node: YGNodeRef) -> YGOverflow;
}
extern "C" {
    #[link_name = "\u{1}_YGNodeStyleSetDisplay"]
    pub fn YGNodeStyleSetDisplay(node: YGNodeRef, display: YGDisplay);
}
extern "C" {
    #[link_name = "\u{1}_YGNodeStyleGetDisplay"]
    pub fn YGNodeStyleGetDisplay(node: YGNodeRef) -> YGDisplay;
}
extern "C" {
    #[link_name = "\u{1}_YGNodeStyleSetFlex"]
    pub fn YGNodeStyleSetFlex(node: YGNodeRef, flex: f32);
}
extern "C" {
    #[link_name = "\u{1}_YGNodeStyleGetFlex"]
    pub fn YGNodeStyleGetFlex(node: YGNodeRef) -> f32;
}
extern "C" {
    #[link_name = "\u{1}_YGNodeStyleSetFlexGrow"]
    pub fn YGNodeStyleSetFlexGrow(node: YGNodeRef, flexGrow: f32);
}
extern "C" {
    #[link_name = "\u{1}_YGNodeStyleGetFlexGrow"]
    pub fn YGNodeStyleGetFlexGrow(node: YGNodeRef) -> f32;
}
extern "C" {
    #[link_name = "\u{1}_YGNodeStyleSetFlexShrink"]
    pub fn YGNodeStyleSetFlexShrink(node: YGNodeRef, flexShrink: f32);
}
extern "C" {
    #[link_name = "\u{1}_YGNodeStyleGetFlexShrink"]
    pub fn YGNodeStyleGetFlexShrink(node: YGNodeRef) -> f32;
}
extern "C" {
    #[link_name = "\u{1}_YGNodeStyleSetFlexBasis"]
    pub fn YGNodeStyleSetFlexBasis(node: YGNodeRef, flexBasis: f32);
}
extern "C" {
    #[link_name = "\u{1}_YGNodeStyleSetFlexBasisPercent"]
    pub fn YGNodeStyleSetFlexBasisPercent(node: YGNodeRef, flexBasis: f32);
}
extern "C" {
    #[link_name = "\u{1}_YGNodeStyleGetFlexBasis"]
    pub fn YGNodeStyleGetFlexBasis(node: YGNodeRef) -> YGValue;
}
extern "C" {
    #[link_name = "\u{1}_YGNodeStyleSetFlexBasisAuto"]
    pub fn YGNodeStyleSetFlexBasisAuto(node: YGNodeRef);
}
extern "C" {
    #[link_name = "\u{1}_YGNodeStyleSetPosition"]
    pub fn YGNodeStyleSetPosition(node: YGNodeRef, edge: YGEdge, position: f32);
}
extern "C" {
    #[link_name = "\u{1}_YGNodeStyleSetPositionPercent"]
    pub fn YGNodeStyleSetPositionPercent(node: YGNodeRef, edge: YGEdge, position: f32);
}
extern "C" {
    #[link_name = "\u{1}_YGNodeStyleGetPosition"]
    pub fn YGNodeStyleGetPosition(node: YGNodeRef, edge: YGEdge) -> YGValue;
}
extern "C" {
    #[link_name = "\u{1}_YGNodeStyleSetMargin"]
    pub fn YGNodeStyleSetMargin(node: YGNodeRef, edge: YGEdge, margin: f32);
}
extern "C" {
    #[link_name = "\u{1}_YGNodeStyleSetMarginPercent"]
    pub fn YGNodeStyleSetMarginPercent(node: YGNodeRef, edge: YGEdge, margin: f32);
}
extern "C" {
    #[link_name = "\u{1}_YGNodeStyleGetMargin"]
    pub fn YGNodeStyleGetMargin(node: YGNodeRef, edge: YGEdge) -> YGValue;
}
extern "C" {
    #[link_name = "\u{1}_YGNodeStyleSetMarginAuto"]
    pub fn YGNodeStyleSetMarginAuto(node: YGNodeRef, edge: YGEdge);
}
extern "C" {
    #[link_name = "\u{1}_YGNodeStyleSetPadding"]
    pub fn YGNodeStyleSetPadding(node: YGNodeRef, edge: YGEdge, padding: f32);
}
extern "C" {
    #[link_name = "\u{1}_YGNodeStyleSetPaddingPercent"]
    pub fn YGNodeStyleSetPaddingPercent(node: YGNodeRef, edge: YGEdge, padding: f32);
}
extern "C" {
    #[link_name = "\u{1}_YGNodeStyleGetPadding"]
    pub fn YGNodeStyleGetPadding(node: YGNodeRef, edge: YGEdge) -> YGValue;
}
extern "C" {
    #[link_name = "\u{1}_YGNodeStyleSetBorder"]
    pub fn YGNodeStyleSetBorder(node: YGNodeRef, edge: YGEdge, border: f32);
}
extern "C" {
    #[link_name = "\u{1}_YGNodeStyleGetBorder"]
    pub fn YGNodeStyleGetBorder(node: YGNodeRef, edge: YGEdge) -> f32;
}
extern "C" {
    #[link_name = "\u{1}_YGNodeStyleSetWidth"]
    pub fn YGNodeStyleSetWidth(node: YGNodeRef, width: f32);
}
extern "C" {
    #[link_name = "\u{1}_YGNodeStyleSetWidthPercent"]
    pub fn YGNodeStyleSetWidthPercent(node: YGNodeRef, width: f32);
}
extern "C" {
    #[link_name = "\u{1}_YGNodeStyleGetWidth"]
    pub fn YGNodeStyleGetWidth(node: YGNodeRef) -> YGValue;
}
extern "C" {
    #[link_name = "\u{1}_YGNodeStyleSetWidthAuto"]
    pub fn YGNodeStyleSetWidthAuto(node: YGNodeRef);
}
extern "C" {
    #[link_name = "\u{1}_YGNodeStyleSetHeight"]
    pub fn YGNodeStyleSetHeight(node: YGNodeRef, height: f32);
}
extern "C" {
    #[link_name = "\u{1}_YGNodeStyleSetHeightPercent"]
    pub fn YGNodeStyleSetHeightPercent(node: YGNodeRef, height: f32);
}
extern "C" {
    #[link_name = "\u{1}_YGNodeStyleGetHeight"]
    pub fn YGNodeStyleGetHeight(node: YGNodeRef) -> YGValue;
}
extern "C" {
    #[link_name = "\u{1}_YGNodeStyleSetHeightAuto"]
    pub fn YGNodeStyleSetHeightAuto(node: YGNodeRef);
}
extern "C" {
    #[link_name = "\u{1}_YGNodeStyleSetMinWidth"]
    pub fn YGNodeStyleSetMinWidth(node: YGNodeRef, minWidth: f32);
}
extern "C" {
    #[link_name = "\u{1}_YGNodeStyleSetMinWidthPercent"]
    pub fn YGNodeStyleSetMinWidthPercent(node: YGNodeRef, minWidth: f32);
}
extern "C" {
    #[link_name = "\u{1}_YGNodeStyleGetMinWidth"]
    pub fn YGNodeStyleGetMinWidth(node: YGNodeRef) -> YGValue;
}
extern "C" {
    #[link_name = "\u{1}_YGNodeStyleSetMinHeight"]
    pub fn YGNodeStyleSetMinHeight(node: YGNodeRef, minHeight: f32);
}
extern "C" {
    #[link_name = "\u{1}_YGNodeStyleSetMinHeightPercent"]
    pub fn YGNodeStyleSetMinHeightPercent(node: YGNodeRef, minHeight: f32);
}
extern "C" {
    #[link_name = "\u{1}_YGNodeStyleGetMinHeight"]
    pub fn YGNodeStyleGetMinHeight(node: YGNodeRef) -> YGValue;
}
extern "C" {
    #[link_name = "\u{1}_YGNodeStyleSetMaxWidth"]
    pub fn YGNodeStyleSetMaxWidth(node: YGNodeRef, maxWidth: f32);
}
extern "C" {
    #[link_name = "\u{1}_YGNodeStyleSetMaxWidthPercent"]
    pub fn YGNodeStyleSetMaxWidthPercent(node: YGNodeRef, maxWidth: f32);
}
extern "C" {
    #[link_name = "\u{1}_YGNodeStyleGetMaxWidth"]
    pub fn YGNodeStyleGetMaxWidth(node: YGNodeRef) -> YGValue;
}
extern "C" {
    #[link_name = "\u{1}_YGNodeStyleSetMaxHeight"]
    pub fn YGNodeStyleSetMaxHeight(node: YGNodeRef, maxHeight: f32);
}
extern "C" {
    #[link_name = "\u{1}_YGNodeStyleSetMaxHeightPercent"]
    pub fn YGNodeStyleSetMaxHeightPercent(node: YGNodeRef, maxHeight: f32);
}
extern "C" {
    #[link_name = "\u{1}_YGNodeStyleGetMaxHeight"]
    pub fn YGNodeStyleGetMaxHeight(node: YGNodeRef) -> YGValue;
}
extern "C" {
    #[link_name = "\u{1}_YGNodeStyleSetAspectRatio"]
    pub fn YGNodeStyleSetAspectRatio(node: YGNodeRef, aspectRatio: f32);
}
extern "C" {
    #[link_name = "\u{1}_YGNodeStyleGetAspectRatio"]
    pub fn YGNodeStyleGetAspectRatio(node: YGNodeRef) -> f32;
}
extern "C" {
    #[link_name = "\u{1}_YGNodeLayoutGetLeft"]
    pub fn YGNodeLayoutGetLeft(node: YGNodeRef) -> f32;
}
extern "C" {
    #[link_name = "\u{1}_YGNodeLayoutGetTop"]
    pub fn YGNodeLayoutGetTop(node: YGNodeRef) -> f32;
}
extern "C" {
    #[link_name = "\u{1}_YGNodeLayoutGetRight"]
    pub fn YGNodeLayoutGetRight(node: YGNodeRef) -> f32;
}
extern "C" {
    #[link_name = "\u{1}_YGNodeLayoutGetBottom"]
    pub fn YGNodeLayoutGetBottom(node: YGNodeRef) -> f32;
}
extern "C" {
    #[link_name = "\u{1}_YGNodeLayoutGetWidth"]
    pub fn YGNodeLayoutGetWidth(node: YGNodeRef) -> f32;
}
extern "C" {
    #[link_name = "\u{1}_YGNodeLayoutGetHeight"]
    pub fn YGNodeLayoutGetHeight(node: YGNodeRef) -> f32;
}
extern "C" {
    #[link_name = "\u{1}_YGNodeLayoutGetDirection"]
    pub fn YGNodeLayoutGetDirection(node: YGNodeRef) -> YGDirection;
}
extern "C" {
    #[link_name = "\u{1}_YGNodeLayoutGetHadOverflow"]
    pub fn YGNodeLayoutGetHadOverflow(node: YGNodeRef) -> bool;
}
extern "C" {
    #[link_name = "\u{1}_YGNodeLayoutGetDidLegacyStretchFlagAffectLayout"]
    pub fn YGNodeLayoutGetDidLegacyStretchFlagAffectLayout(node: YGNodeRef) -> bool;
}
extern "C" {
    #[link_name = "\u{1}_YGNodeLayoutGetMargin"]
    pub fn YGNodeLayoutGetMargin(node: YGNodeRef, edge: YGEdge) -> f32;
}
extern "C" {
    #[link_name = "\u{1}_YGNodeLayoutGetBorder"]
    pub fn YGNodeLayoutGetBorder(node: YGNodeRef, edge: YGEdge) -> f32;
}
extern "C" {
    #[link_name = "\u{1}_YGNodeLayoutGetPadding"]
    pub fn YGNodeLayoutGetPadding(node: YGNodeRef, edge: YGEdge) -> f32;
}
extern "C" {
    #[link_name = "\u{1}_YGConfigSetLogger"]
    pub fn YGConfigSetLogger(config: YGConfigRef, logger: YGLogger);
}
extern "C" {
    #[link_name = "\u{1}_YGLog"]
    pub fn YGLog(node: YGNodeRef, level: YGLogLevel, message: *const ::std::os::raw::c_char, ...);
}
extern "C" {
    #[link_name = "\u{1}_YGLogWithConfig"]
    pub fn YGLogWithConfig(
        config: YGConfigRef,
        level: YGLogLevel,
        format: *const ::std::os::raw::c_char,
        ...
    );
}
extern "C" {
    #[link_name = "\u{1}_YGAssert"]
    pub fn YGAssert(condition: bool, message: *const ::std::os::raw::c_char);
}
extern "C" {
    #[link_name = "\u{1}_YGAssertWithNode"]
    pub fn YGAssertWithNode(
        node: YGNodeRef,
        condition: bool,
        message: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    #[link_name = "\u{1}_YGAssertWithConfig"]
    pub fn YGAssertWithConfig(
        config: YGConfigRef,
        condition: bool,
        message: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    #[link_name = "\u{1}_YGConfigSetPointScaleFactor"]
    pub fn YGConfigSetPointScaleFactor(config: YGConfigRef, pixelsInPoint: f32);
}
extern "C" {
    #[link_name = "\u{1}_YGConfigSetShouldDiffLayoutWithoutLegacyStretchBehaviour"]
    pub fn YGConfigSetShouldDiffLayoutWithoutLegacyStretchBehaviour(
        config: YGConfigRef,
        shouldDiffLayout: bool,
    );
}
extern "C" {
    #[link_name = "\u{1}_YGConfigSetUseLegacyStretchBehaviour"]
    pub fn YGConfigSetUseLegacyStretchBehaviour(
        config: YGConfigRef,
        useLegacyStretchBehaviour: bool,
    );
}
extern "C" {
    #[link_name = "\u{1}_YGConfigNew"]
    pub fn YGConfigNew() -> YGConfigRef;
}
extern "C" {
    #[link_name = "\u{1}_YGConfigFree"]
    pub fn YGConfigFree(config: YGConfigRef);
}
extern "C" {
    #[link_name = "\u{1}_YGConfigCopy"]
    pub fn YGConfigCopy(dest: YGConfigRef, src: YGConfigRef);
}
extern "C" {
    #[link_name = "\u{1}_YGConfigGetInstanceCount"]
    pub fn YGConfigGetInstanceCount() -> i32;
}
extern "C" {
    #[link_name = "\u{1}_YGConfigSetExperimentalFeatureEnabled"]
    pub fn YGConfigSetExperimentalFeatureEnabled(
        config: YGConfigRef,
        feature: YGExperimentalFeature,
        enabled: bool,
    );
}
extern "C" {
    #[link_name = "\u{1}_YGConfigIsExperimentalFeatureEnabled"]
    pub fn YGConfigIsExperimentalFeatureEnabled(
        config: YGConfigRef,
        feature: YGExperimentalFeature,
    ) -> bool;
}
extern "C" {
    #[link_name = "\u{1}_YGConfigSetUseWebDefaults"]
    pub fn YGConfigSetUseWebDefaults(config: YGConfigRef, enabled: bool);
}
extern "C" {
    #[link_name = "\u{1}_YGConfigGetUseWebDefaults"]
    pub fn YGConfigGetUseWebDefaults(config: YGConfigRef) -> bool;
}
extern "C" {
    #[link_name = "\u{1}_YGConfigSetCloneNodeFunc"]
    pub fn YGConfigSetCloneNodeFunc(config: YGConfigRef, callback: YGCloneNodeFunc);
}
extern "C" {
    #[link_name = "\u{1}_YGConfigGetDefault"]
    pub fn YGConfigGetDefault() -> YGConfigRef;
}
extern "C" {
    #[link_name = "\u{1}_YGConfigSetContext"]
    pub fn YGConfigSetContext(config: YGConfigRef, context: *mut ::std::os::raw::c_void);
}
extern "C" {
    #[link_name = "\u{1}_YGConfigGetContext"]
    pub fn YGConfigGetContext(config: YGConfigRef) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    #[link_name = "\u{1}_YGRoundValueToPixelGrid"]
    pub fn YGRoundValueToPixelGrid(
        value: f32,
        pointScaleFactor: f32,
        forceCeil: bool,
        forceFloor: bool,
    ) -> f32;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct YGNodeList {
    _unused: [u8; 0],
}
pub type YGNodeListRef = *mut YGNodeList;
extern "C" {
    #[link_name = "\u{1}_YGNodeListNew"]
    pub fn YGNodeListNew(initialCapacity: u32) -> YGNodeListRef;
}
extern "C" {
    #[link_name = "\u{1}_YGNodeListFree"]
    pub fn YGNodeListFree(list: YGNodeListRef);
}
extern "C" {
    #[link_name = "\u{1}_YGNodeListCount"]
    pub fn YGNodeListCount(list: YGNodeListRef) -> u32;
}
extern "C" {
    #[link_name = "\u{1}_YGNodeListAdd"]
    pub fn YGNodeListAdd(listp: *mut YGNodeListRef, node: YGNodeRef);
}
extern "C" {
    #[link_name = "\u{1}_YGNodeListInsert"]
    pub fn YGNodeListInsert(listp: *mut YGNodeListRef, node: YGNodeRef, index: u32);
}
extern "C" {
    #[link_name = "\u{1}_YGNodeListReplace"]
    pub fn YGNodeListReplace(list: YGNodeListRef, index: u32, newNode: YGNodeRef);
}
extern "C" {
    #[link_name = "\u{1}_YGNodeListRemoveAll"]
    pub fn YGNodeListRemoveAll(list: YGNodeListRef);
}
extern "C" {
    #[link_name = "\u{1}_YGNodeListRemove"]
    pub fn YGNodeListRemove(list: YGNodeListRef, index: u32) -> YGNodeRef;
}
extern "C" {
    #[link_name = "\u{1}_YGNodeListDelete"]
    pub fn YGNodeListDelete(list: YGNodeListRef, node: YGNodeRef) -> YGNodeRef;
}
extern "C" {
    #[link_name = "\u{1}_YGNodeListGet"]
    pub fn YGNodeListGet(list: YGNodeListRef, index: u32) -> YGNodeRef;
}
extern "C" {
    #[link_name = "\u{1}_YGNodeListClone"]
    pub fn YGNodeListClone(list: YGNodeListRef) -> YGNodeListRef;
}
pub type __builtin_va_list = [__va_list_tag; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __va_list_tag {
    pub gp_offset: ::std::os::raw::c_uint,
    pub fp_offset: ::std::os::raw::c_uint,
    pub overflow_arg_area: *mut ::std::os::raw::c_void,
    pub reg_save_area: *mut ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout___va_list_tag() {
    assert_eq!(
        ::std::mem::size_of::<__va_list_tag>(),
        24usize,
        concat!("Size of: ", stringify!(__va_list_tag))
    );
    assert_eq!(
        ::std::mem::align_of::<__va_list_tag>(),
        8usize,
        concat!("Alignment of ", stringify!(__va_list_tag))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__va_list_tag>())).gp_offset as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__va_list_tag),
            "::",
            stringify!(gp_offset)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__va_list_tag>())).fp_offset as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(__va_list_tag),
            "::",
            stringify!(fp_offset)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__va_list_tag>())).overflow_arg_area as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__va_list_tag),
            "::",
            stringify!(overflow_arg_area)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__va_list_tag>())).reg_save_area as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(__va_list_tag),
            "::",
            stringify!(reg_save_area)
        )
    );
}

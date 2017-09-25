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
    pub fn YGAlignToString(value: YGAlign) -> *const ::std::os::raw::c_char;
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum YGDimension { YGDimensionWidth = 0, YGDimensionHeight = 1, }
extern "C" {
    pub fn YGDimensionToString(value: YGDimension)
     -> *const ::std::os::raw::c_char;
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum YGDirection {
    YGDirectionInherit = 0,
    YGDirectionLTR = 1,
    YGDirectionRTL = 2,
}
extern "C" {
    pub fn YGDirectionToString(value: YGDirection)
     -> *const ::std::os::raw::c_char;
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum YGDisplay { YGDisplayFlex = 0, YGDisplayNone = 1, }
extern "C" {
    pub fn YGDisplayToString(value: YGDisplay)
     -> *const ::std::os::raw::c_char;
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
    pub fn YGEdgeToString(value: YGEdge) -> *const ::std::os::raw::c_char;
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum YGExperimentalFeature { YGExperimentalFeatureWebFlexBasis = 0, }
extern "C" {
    pub fn YGExperimentalFeatureToString(value: YGExperimentalFeature)
     -> *const ::std::os::raw::c_char;
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
    pub fn YGFlexDirectionToString(value: YGFlexDirection)
     -> *const ::std::os::raw::c_char;
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum YGJustify {
    YGJustifyFlexStart = 0,
    YGJustifyCenter = 1,
    YGJustifyFlexEnd = 2,
    YGJustifySpaceBetween = 3,
    YGJustifySpaceAround = 4,
}
extern "C" {
    pub fn YGJustifyToString(value: YGJustify)
     -> *const ::std::os::raw::c_char;
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
    pub fn YGLogLevelToString(value: YGLogLevel)
     -> *const ::std::os::raw::c_char;
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum YGMeasureMode {
    YGMeasureModeUndefined = 0,
    YGMeasureModeExactly = 1,
    YGMeasureModeAtMost = 2,
}
extern "C" {
    pub fn YGMeasureModeToString(value: YGMeasureMode)
     -> *const ::std::os::raw::c_char;
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum YGNodeType { YGNodeTypeDefault = 0, YGNodeTypeText = 1, }
extern "C" {
    pub fn YGNodeTypeToString(value: YGNodeType)
     -> *const ::std::os::raw::c_char;
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum YGOverflow {
    YGOverflowVisible = 0,
    YGOverflowHidden = 1,
    YGOverflowScroll = 2,
}
extern "C" {
    pub fn YGOverflowToString(value: YGOverflow)
     -> *const ::std::os::raw::c_char;
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum YGPositionType {
    YGPositionTypeRelative = 0,
    YGPositionTypeAbsolute = 1,
}
extern "C" {
    pub fn YGPositionTypeToString(value: YGPositionType)
     -> *const ::std::os::raw::c_char;
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum YGPrintOptions {
    YGPrintOptionsLayout = 1,
    YGPrintOptionsStyle = 2,
    YGPrintOptionsChildren = 4,
}
extern "C" {
    pub fn YGPrintOptionsToString(value: YGPrintOptions)
     -> *const ::std::os::raw::c_char;
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
    pub fn YGUnitToString(value: YGUnit) -> *const ::std::os::raw::c_char;
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum YGWrap { YGWrapNoWrap = 0, YGWrapWrap = 1, YGWrapWrapReverse = 2, }
extern "C" {
    pub fn YGWrapToString(value: YGWrap) -> *const ::std::os::raw::c_char;
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct YGSize {
    pub width: f32,
    pub height: f32,
}
#[test]
fn bindgen_test_layout_YGSize() {
    assert_eq!(::std::mem::size_of::<YGSize>() , 8usize , concat ! (
               "Size of: " , stringify ! ( YGSize ) ));
    assert_eq! (::std::mem::align_of::<YGSize>() , 4usize , concat ! (
                "Alignment of " , stringify ! ( YGSize ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const YGSize ) ) . width as * const _ as usize
                } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( YGSize ) , "::" ,
                stringify ! ( width ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const YGSize ) ) . height as * const _ as usize
                } , 4usize , concat ! (
                "Alignment of field: " , stringify ! ( YGSize ) , "::" ,
                stringify ! ( height ) ));
}
impl Clone for YGSize {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct YGValue {
    pub value: f32,
    pub unit: YGUnit,
}
#[test]
fn bindgen_test_layout_YGValue() {
    assert_eq!(::std::mem::size_of::<YGValue>() , 8usize , concat ! (
               "Size of: " , stringify ! ( YGValue ) ));
    assert_eq! (::std::mem::align_of::<YGValue>() , 4usize , concat ! (
                "Alignment of " , stringify ! ( YGValue ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const YGValue ) ) . value as * const _ as usize
                } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( YGValue ) , "::" ,
                stringify ! ( value ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const YGValue ) ) . unit as * const _ as usize
                } , 4usize , concat ! (
                "Alignment of field: " , stringify ! ( YGValue ) , "::" ,
                stringify ! ( unit ) ));
}
impl Clone for YGValue {
    fn clone(&self) -> Self { *self }
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
pub type YGMeasureFunc =
    ::std::option::Option<unsafe extern "C" fn(node: YGNodeRef, width: f32,
                                               widthMode: YGMeasureMode,
                                               height: f32,
                                               heightMode: YGMeasureMode)
                              -> YGSize>;
pub type YGBaselineFunc =
    ::std::option::Option<unsafe extern "C" fn(node: YGNodeRef, width: f32,
                                               height: f32) -> f32>;
pub type YGPrintFunc =
    ::std::option::Option<unsafe extern "C" fn(node: YGNodeRef)>;
pub type YGLogger =
    ::std::option::Option<unsafe extern "C" fn(config: YGConfigRef,
                                               node: YGNodeRef,
                                               level: YGLogLevel,
                                               format:
                                                   *const ::std::os::raw::c_char,
                                               args: *mut __va_list_tag)
                              -> ::std::os::raw::c_int>;
pub type YGMalloc =
    ::std::option::Option<unsafe extern "C" fn(size: usize)
                              -> *mut ::std::os::raw::c_void>;
pub type YGCalloc =
    ::std::option::Option<unsafe extern "C" fn(count: usize, size: usize)
                              -> *mut ::std::os::raw::c_void>;
pub type YGRealloc =
    ::std::option::Option<unsafe extern "C" fn(ptr:
                                                   *mut ::std::os::raw::c_void,
                                               size: usize)
                              -> *mut ::std::os::raw::c_void>;
pub type YGFree =
    ::std::option::Option<unsafe extern "C" fn(ptr:
                                                   *mut ::std::os::raw::c_void)>;
extern "C" {
    pub fn YGNodeNew() -> YGNodeRef;
}
extern "C" {
    pub fn YGNodeNewWithConfig(config: YGConfigRef) -> YGNodeRef;
}
extern "C" {
    pub fn YGNodeFree(node: YGNodeRef);
}
extern "C" {
    pub fn YGNodeFreeRecursive(node: YGNodeRef);
}
extern "C" {
    pub fn YGNodeReset(node: YGNodeRef);
}
extern "C" {
    pub fn YGNodeGetInstanceCount() -> i32;
}
extern "C" {
    pub fn YGNodeInsertChild(node: YGNodeRef, child: YGNodeRef, index: u32);
}
extern "C" {
    pub fn YGNodeRemoveChild(node: YGNodeRef, child: YGNodeRef);
}
extern "C" {
    pub fn YGNodeGetChild(node: YGNodeRef, index: u32) -> YGNodeRef;
}
extern "C" {
    pub fn YGNodeGetParent(node: YGNodeRef) -> YGNodeRef;
}
extern "C" {
    pub fn YGNodeGetChildCount(node: YGNodeRef) -> u32;
}
extern "C" {
    pub fn YGNodeCalculateLayout(node: YGNodeRef, availableWidth: f32,
                                 availableHeight: f32,
                                 parentDirection: YGDirection);
}
extern "C" {
    pub fn YGNodeMarkDirty(node: YGNodeRef);
}
extern "C" {
    pub fn YGNodeIsDirty(node: YGNodeRef) -> bool;
}
extern "C" {
    pub fn YGNodePrint(node: YGNodeRef, options: YGPrintOptions);
}
extern "C" {
    pub fn YGFloatIsUndefined(value: f32) -> bool;
}
extern "C" {
    pub fn YGNodeCanUseCachedMeasurement(widthMode: YGMeasureMode, width: f32,
                                         heightMode: YGMeasureMode,
                                         height: f32,
                                         lastWidthMode: YGMeasureMode,
                                         lastWidth: f32,
                                         lastHeightMode: YGMeasureMode,
                                         lastHeight: f32,
                                         lastComputedWidth: f32,
                                         lastComputedHeight: f32,
                                         marginRow: f32, marginColumn: f32,
                                         config: YGConfigRef) -> bool;
}
extern "C" {
    pub fn YGNodeCopyStyle(dstNode: YGNodeRef, srcNode: YGNodeRef);
}
extern "C" {
    pub fn YGNodeSetContext(node: YGNodeRef,
                            context: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn YGNodeGetContext(node: YGNodeRef) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn YGNodeSetMeasureFunc(node: YGNodeRef, measureFunc: YGMeasureFunc);
}
extern "C" {
    pub fn YGNodeGetMeasureFunc(node: YGNodeRef) -> YGMeasureFunc;
}
extern "C" {
    pub fn YGNodeSetBaselineFunc(node: YGNodeRef,
                                 baselineFunc: YGBaselineFunc);
}
extern "C" {
    pub fn YGNodeGetBaselineFunc(node: YGNodeRef) -> YGBaselineFunc;
}
extern "C" {
    pub fn YGNodeSetPrintFunc(node: YGNodeRef, printFunc: YGPrintFunc);
}
extern "C" {
    pub fn YGNodeGetPrintFunc(node: YGNodeRef) -> YGPrintFunc;
}
extern "C" {
    pub fn YGNodeSetHasNewLayout(node: YGNodeRef, hasNewLayout: bool);
}
extern "C" {
    pub fn YGNodeGetHasNewLayout(node: YGNodeRef) -> bool;
}
extern "C" {
    pub fn YGNodeSetNodeType(node: YGNodeRef, nodeType: YGNodeType);
}
extern "C" {
    pub fn YGNodeGetNodeType(node: YGNodeRef) -> YGNodeType;
}
extern "C" {
    pub fn YGNodeStyleSetDirection(node: YGNodeRef, direction: YGDirection);
}
extern "C" {
    pub fn YGNodeStyleGetDirection(node: YGNodeRef) -> YGDirection;
}
extern "C" {
    pub fn YGNodeStyleSetFlexDirection(node: YGNodeRef,
                                       flexDirection: YGFlexDirection);
}
extern "C" {
    pub fn YGNodeStyleGetFlexDirection(node: YGNodeRef) -> YGFlexDirection;
}
extern "C" {
    pub fn YGNodeStyleSetJustifyContent(node: YGNodeRef,
                                        justifyContent: YGJustify);
}
extern "C" {
    pub fn YGNodeStyleGetJustifyContent(node: YGNodeRef) -> YGJustify;
}
extern "C" {
    pub fn YGNodeStyleSetAlignContent(node: YGNodeRef, alignContent: YGAlign);
}
extern "C" {
    pub fn YGNodeStyleGetAlignContent(node: YGNodeRef) -> YGAlign;
}
extern "C" {
    pub fn YGNodeStyleSetAlignItems(node: YGNodeRef, alignItems: YGAlign);
}
extern "C" {
    pub fn YGNodeStyleGetAlignItems(node: YGNodeRef) -> YGAlign;
}
extern "C" {
    pub fn YGNodeStyleSetAlignSelf(node: YGNodeRef, alignSelf: YGAlign);
}
extern "C" {
    pub fn YGNodeStyleGetAlignSelf(node: YGNodeRef) -> YGAlign;
}
extern "C" {
    pub fn YGNodeStyleSetPositionType(node: YGNodeRef,
                                      positionType: YGPositionType);
}
extern "C" {
    pub fn YGNodeStyleGetPositionType(node: YGNodeRef) -> YGPositionType;
}
extern "C" {
    pub fn YGNodeStyleSetFlexWrap(node: YGNodeRef, flexWrap: YGWrap);
}
extern "C" {
    pub fn YGNodeStyleGetFlexWrap(node: YGNodeRef) -> YGWrap;
}
extern "C" {
    pub fn YGNodeStyleSetOverflow(node: YGNodeRef, overflow: YGOverflow);
}
extern "C" {
    pub fn YGNodeStyleGetOverflow(node: YGNodeRef) -> YGOverflow;
}
extern "C" {
    pub fn YGNodeStyleSetDisplay(node: YGNodeRef, display: YGDisplay);
}
extern "C" {
    pub fn YGNodeStyleGetDisplay(node: YGNodeRef) -> YGDisplay;
}
extern "C" {
    pub fn YGNodeStyleSetFlex(node: YGNodeRef, flex: f32);
}
extern "C" {
    pub fn YGNodeStyleGetFlex(node: YGNodeRef) -> f32;
}
extern "C" {
    pub fn YGNodeStyleSetFlexGrow(node: YGNodeRef, flexGrow: f32);
}
extern "C" {
    pub fn YGNodeStyleGetFlexGrow(node: YGNodeRef) -> f32;
}
extern "C" {
    pub fn YGNodeStyleSetFlexShrink(node: YGNodeRef, flexShrink: f32);
}
extern "C" {
    pub fn YGNodeStyleGetFlexShrink(node: YGNodeRef) -> f32;
}
extern "C" {
    pub fn YGNodeStyleSetFlexBasis(node: YGNodeRef, flexBasis: f32);
}
extern "C" {
    pub fn YGNodeStyleSetFlexBasisPercent(node: YGNodeRef, flexBasis: f32);
}
extern "C" {
    pub fn YGNodeStyleGetFlexBasis(node: YGNodeRef) -> YGValue;
}
extern "C" {
    pub fn YGNodeStyleSetFlexBasisAuto(node: YGNodeRef);
}
extern "C" {
    pub fn YGNodeStyleSetPosition(node: YGNodeRef, edge: YGEdge,
                                  position: f32);
}
extern "C" {
    pub fn YGNodeStyleSetPositionPercent(node: YGNodeRef, edge: YGEdge,
                                         position: f32);
}
extern "C" {
    pub fn YGNodeStyleGetPosition(node: YGNodeRef, edge: YGEdge) -> YGValue;
}
extern "C" {
    pub fn YGNodeStyleSetMargin(node: YGNodeRef, edge: YGEdge, margin: f32);
}
extern "C" {
    pub fn YGNodeStyleSetMarginPercent(node: YGNodeRef, edge: YGEdge,
                                       margin: f32);
}
extern "C" {
    pub fn YGNodeStyleGetMargin(node: YGNodeRef, edge: YGEdge) -> YGValue;
}
extern "C" {
    pub fn YGNodeStyleSetMarginAuto(node: YGNodeRef, edge: YGEdge);
}
extern "C" {
    pub fn YGNodeStyleSetPadding(node: YGNodeRef, edge: YGEdge, padding: f32);
}
extern "C" {
    pub fn YGNodeStyleSetPaddingPercent(node: YGNodeRef, edge: YGEdge,
                                        padding: f32);
}
extern "C" {
    pub fn YGNodeStyleGetPadding(node: YGNodeRef, edge: YGEdge) -> YGValue;
}
extern "C" {
    pub fn YGNodeStyleSetBorder(node: YGNodeRef, edge: YGEdge, border: f32);
}
extern "C" {
    pub fn YGNodeStyleGetBorder(node: YGNodeRef, edge: YGEdge) -> f32;
}
extern "C" {
    pub fn YGNodeStyleSetWidth(node: YGNodeRef, width: f32);
}
extern "C" {
    pub fn YGNodeStyleSetWidthPercent(node: YGNodeRef, width: f32);
}
extern "C" {
    pub fn YGNodeStyleGetWidth(node: YGNodeRef) -> YGValue;
}
extern "C" {
    pub fn YGNodeStyleSetWidthAuto(node: YGNodeRef);
}
extern "C" {
    pub fn YGNodeStyleSetHeight(node: YGNodeRef, height: f32);
}
extern "C" {
    pub fn YGNodeStyleSetHeightPercent(node: YGNodeRef, height: f32);
}
extern "C" {
    pub fn YGNodeStyleGetHeight(node: YGNodeRef) -> YGValue;
}
extern "C" {
    pub fn YGNodeStyleSetHeightAuto(node: YGNodeRef);
}
extern "C" {
    pub fn YGNodeStyleSetMinWidth(node: YGNodeRef, minWidth: f32);
}
extern "C" {
    pub fn YGNodeStyleSetMinWidthPercent(node: YGNodeRef, minWidth: f32);
}
extern "C" {
    pub fn YGNodeStyleGetMinWidth(node: YGNodeRef) -> YGValue;
}
extern "C" {
    pub fn YGNodeStyleSetMinHeight(node: YGNodeRef, minHeight: f32);
}
extern "C" {
    pub fn YGNodeStyleSetMinHeightPercent(node: YGNodeRef, minHeight: f32);
}
extern "C" {
    pub fn YGNodeStyleGetMinHeight(node: YGNodeRef) -> YGValue;
}
extern "C" {
    pub fn YGNodeStyleSetMaxWidth(node: YGNodeRef, maxWidth: f32);
}
extern "C" {
    pub fn YGNodeStyleSetMaxWidthPercent(node: YGNodeRef, maxWidth: f32);
}
extern "C" {
    pub fn YGNodeStyleGetMaxWidth(node: YGNodeRef) -> YGValue;
}
extern "C" {
    pub fn YGNodeStyleSetMaxHeight(node: YGNodeRef, maxHeight: f32);
}
extern "C" {
    pub fn YGNodeStyleSetMaxHeightPercent(node: YGNodeRef, maxHeight: f32);
}
extern "C" {
    pub fn YGNodeStyleGetMaxHeight(node: YGNodeRef) -> YGValue;
}
extern "C" {
    pub fn YGNodeStyleSetAspectRatio(node: YGNodeRef, aspectRatio: f32);
}
extern "C" {
    pub fn YGNodeStyleGetAspectRatio(node: YGNodeRef) -> f32;
}
extern "C" {
    pub fn YGNodeLayoutGetLeft(node: YGNodeRef) -> f32;
}
extern "C" {
    pub fn YGNodeLayoutGetTop(node: YGNodeRef) -> f32;
}
extern "C" {
    pub fn YGNodeLayoutGetRight(node: YGNodeRef) -> f32;
}
extern "C" {
    pub fn YGNodeLayoutGetBottom(node: YGNodeRef) -> f32;
}
extern "C" {
    pub fn YGNodeLayoutGetWidth(node: YGNodeRef) -> f32;
}
extern "C" {
    pub fn YGNodeLayoutGetHeight(node: YGNodeRef) -> f32;
}
extern "C" {
    pub fn YGNodeLayoutGetDirection(node: YGNodeRef) -> YGDirection;
}
extern "C" {
    pub fn YGNodeLayoutGetHadOverflow(node: YGNodeRef) -> bool;
}
extern "C" {
    pub fn YGNodeLayoutGetMargin(node: YGNodeRef, edge: YGEdge) -> f32;
}
extern "C" {
    pub fn YGNodeLayoutGetBorder(node: YGNodeRef, edge: YGEdge) -> f32;
}
extern "C" {
    pub fn YGNodeLayoutGetPadding(node: YGNodeRef, edge: YGEdge) -> f32;
}
extern "C" {
    pub fn YGConfigSetLogger(config: YGConfigRef, logger: YGLogger);
}
extern "C" {
    pub fn YGLog(node: YGNodeRef, level: YGLogLevel,
                 message: *const ::std::os::raw::c_char, ...);
}
extern "C" {
    pub fn YGLogWithConfig(config: YGConfigRef, level: YGLogLevel,
                           format: *const ::std::os::raw::c_char, ...);
}
extern "C" {
    pub fn YGAssert(condition: bool, message: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn YGAssertWithNode(node: YGNodeRef, condition: bool,
                            message: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn YGAssertWithConfig(config: YGConfigRef, condition: bool,
                              message: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn YGConfigSetPointScaleFactor(config: YGConfigRef,
                                       pixelsInPoint: f32);
}
extern "C" {
    pub fn YGConfigSetUseLegacyStretchBehaviour(config: YGConfigRef,
                                                useLegacyStretchBehaviour:
                                                    bool);
}
extern "C" {
    pub fn YGConfigNew() -> YGConfigRef;
}
extern "C" {
    pub fn YGConfigFree(config: YGConfigRef);
}
extern "C" {
    pub fn YGConfigCopy(dest: YGConfigRef, src: YGConfigRef);
}
extern "C" {
    pub fn YGConfigGetInstanceCount() -> i32;
}
extern "C" {
    pub fn YGConfigSetExperimentalFeatureEnabled(config: YGConfigRef,
                                                 feature:
                                                     YGExperimentalFeature,
                                                 enabled: bool);
}
extern "C" {
    pub fn YGConfigIsExperimentalFeatureEnabled(config: YGConfigRef,
                                                feature:
                                                    YGExperimentalFeature)
     -> bool;
}
extern "C" {
    pub fn YGConfigSetUseWebDefaults(config: YGConfigRef, enabled: bool);
}
extern "C" {
    pub fn YGConfigGetUseWebDefaults(config: YGConfigRef) -> bool;
}
extern "C" {
    pub fn YGConfigGetDefault() -> YGConfigRef;
}
extern "C" {
    pub fn YGConfigSetContext(config: YGConfigRef,
                              context: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn YGConfigGetContext(config: YGConfigRef)
     -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn YGSetMemoryFuncs(ygmalloc: YGMalloc, yccalloc: YGCalloc,
                            ygrealloc: YGRealloc, ygfree: YGFree);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct YGNodeList {
    _unused: [u8; 0],
}
pub type YGNodeListRef = *mut YGNodeList;
extern "C" {
    pub fn YGNodeListNew(initialCapacity: u32) -> YGNodeListRef;
}
extern "C" {
    pub fn YGNodeListFree(list: YGNodeListRef);
}
extern "C" {
    pub fn YGNodeListCount(list: YGNodeListRef) -> u32;
}
extern "C" {
    pub fn YGNodeListAdd(listp: *mut YGNodeListRef, node: YGNodeRef);
}
extern "C" {
    pub fn YGNodeListInsert(listp: *mut YGNodeListRef, node: YGNodeRef,
                            index: u32);
}
extern "C" {
    pub fn YGNodeListRemove(list: YGNodeListRef, index: u32) -> YGNodeRef;
}
extern "C" {
    pub fn YGNodeListDelete(list: YGNodeListRef, node: YGNodeRef)
     -> YGNodeRef;
}
extern "C" {
    pub fn YGNodeListGet(list: YGNodeListRef, index: u32) -> YGNodeRef;
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct __va_list_tag {
    pub gp_offset: ::std::os::raw::c_uint,
    pub fp_offset: ::std::os::raw::c_uint,
    pub overflow_arg_area: *mut ::std::os::raw::c_void,
    pub reg_save_area: *mut ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout___va_list_tag() {
    assert_eq!(::std::mem::size_of::<__va_list_tag>() , 24usize , concat ! (
               "Size of: " , stringify ! ( __va_list_tag ) ));
    assert_eq! (::std::mem::align_of::<__va_list_tag>() , 8usize , concat ! (
                "Alignment of " , stringify ! ( __va_list_tag ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const __va_list_tag ) ) . gp_offset as * const
                _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( __va_list_tag ) , "::"
                , stringify ! ( gp_offset ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const __va_list_tag ) ) . fp_offset as * const
                _ as usize } , 4usize , concat ! (
                "Alignment of field: " , stringify ! ( __va_list_tag ) , "::"
                , stringify ! ( fp_offset ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const __va_list_tag ) ) . overflow_arg_area as
                * const _ as usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( __va_list_tag ) , "::"
                , stringify ! ( overflow_arg_area ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const __va_list_tag ) ) . reg_save_area as *
                const _ as usize } , 16usize , concat ! (
                "Alignment of field: " , stringify ! ( __va_list_tag ) , "::"
                , stringify ! ( reg_save_area ) ));
}
impl Clone for __va_list_tag {
    fn clone(&self) -> Self { *self }
}
pub type __builtin_va_list = [__va_list_tag; 1usize];

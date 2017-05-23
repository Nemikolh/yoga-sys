use super::*;

#[test]
fn basic_ygnode_test() {
    unsafe {
        let root = YGNodeNew();
        YGNodeStyleSetWidth(root, 500.);
        YGNodeStyleSetHeight(root, 120.);
        YGNodeStyleSetFlexDirection(root, YGFlexDirection::YGFlexDirectionRow);
        YGNodeStyleSetPadding(root, YGEdge::YGEdgeAll, 20.);

        let image = YGNodeNew();
        YGNodeStyleSetWidth(image, 80.);
        YGNodeStyleSetMargin(image, YGEdge::YGEdgeEnd, 20.);

        let text = YGNodeNew();
        YGNodeStyleSetHeight(text, 25.);
        YGNodeStyleSetAlignSelf(text, YGAlign::YGAlignCenter);
        YGNodeStyleSetFlexGrow(text, 1.);

        YGNodeInsertChild(root, image, 0);
        YGNodeInsertChild(root, text, 1);
        YGNodeCalculateLayout(root, 500., 120., YGDirection::YGDirectionRTL);

        let left = YGNodeLayoutGetLeft(root);
        let right = YGNodeLayoutGetRight(root);
        let top = YGNodeLayoutGetTop(root);
        let bottom = YGNodeLayoutGetBottom(root);
        let width = YGNodeLayoutGetWidth(root);
        let height = YGNodeLayoutGetHeight(root);
        assert_eq!(left, 0.);
        assert_eq!(right, 0.);
        assert_eq!(top, 0.);
        assert_eq!(bottom, 0.);
        assert_eq!(width, 500.);
        assert_eq!(height, 120.);


        let left = YGNodeLayoutGetLeft(image);
        let right = YGNodeLayoutGetRight(image);
        let top = YGNodeLayoutGetTop(image);
        let bottom = YGNodeLayoutGetBottom(image);
        let width = YGNodeLayoutGetWidth(image);
        let height = YGNodeLayoutGetHeight(image);
        assert_eq!(left, 400.);
        assert_eq!(right, 20.);
        assert_eq!(top, 20.);
        assert_eq!(bottom, 0.);
        assert_eq!(width, 80.);
        assert_eq!(height, 80.);


        let left = YGNodeLayoutGetLeft(text);
        let right = YGNodeLayoutGetRight(text);
        let top = YGNodeLayoutGetTop(text);
        let bottom = YGNodeLayoutGetBottom(text);
        let width = YGNodeLayoutGetWidth(text);
        let height = YGNodeLayoutGetHeight(text);
        assert_eq!(left, 20.);
        assert_eq!(right, 120.);
        assert_eq!(top, 48.);
        assert_eq!(bottom, 0.);
        assert_eq!(width, 360.);
        assert_eq!(height, 25.);

    }
}
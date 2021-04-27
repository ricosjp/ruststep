#![allow(dead_code)]
pub mod explicit_draughting {
    use crate::primitive::*;
    #[derive(Debug, Clone, PartialEq)]
    pub enum ApprovedItem {
        DrawingRevision(Box<DrawingRevision>),
        DrawingSheetRevision(Box<DrawingSheetRevision>),
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum AreaOrView {
        PresentationArea(Box<PresentationArea>),
        PresentationView(Box<PresentationView>),
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum Axis2Placement {
        Axis2Placement2D(Box<Axis2Placement2D>),
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum BSplineCurveForm {
        EllipticArc,
        PolylineForm,
        ParabolicArc,
        CircularArc,
        Unspecified,
        HyperbolicArc,
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum BoxCharacteristicSelect {
        BoxHeight(Box<BoxHeight>),
        BoxWidth(Box<BoxWidth>),
        BoxSlantAngle(Box<BoxSlantAngle>),
        BoxRotateAngle(Box<BoxRotateAngle>),
    }
    pub type BoxHeight = PositiveRatioMeasure;
    pub type BoxRotateAngle = PlaneAngleMeasure;
    pub type BoxSlantAngle = PlaneAngleMeasure;
    pub type BoxWidth = PositiveRatioMeasure;
    #[derive(Debug, Clone, PartialEq)]
    pub enum CharacterSpacingSelect {
        LengthMeasure(Box<LengthMeasure>),
        RatioMeasure(Box<RatioMeasure>),
        MeasureWithUnit(Box<MeasureWithUnit>),
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum CharacterStyleSelect {
        TextStyleForDefinedFont(Box<TextStyleForDefinedFont>),
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum CharacterizedDefinition {
        CharacterizedProductDefinition(Box<CharacterizedProductDefinition>),
        ShapeDefinition(Box<ShapeDefinition>),
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum CharacterizedProductDefinition {
        ProductDefinition(Box<ProductDefinition>),
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum ClassifiedItem {
        DrawingRevision(Box<DrawingRevision>),
        DrawingSheetRevision(Box<DrawingSheetRevision>),
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum ContractedItem {
        DrawingRevision(Box<DrawingRevision>),
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum CurveFontOrScaledCurveFontSelect {
        CurveStyleFontSelect(Box<CurveStyleFontSelect>),
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum CurveOrAnnotationCurveOccurrence {
        Curve(Box<Curve>),
        AnnotationCurveOccurrence(Box<AnnotationCurveOccurrence>),
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum CurveOrRender {
        CurveStyle(Box<CurveStyle>),
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum CurveStyleFontSelect {
        CurveStyleFont(Box<CurveStyleFont>),
        PreDefinedCurveFont(Box<PreDefinedCurveFont>),
        ExternallyDefinedCurveFont(Box<ExternallyDefinedCurveFont>),
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum DateTimeSelect {
        Date(Box<Date>),
    }
    pub type DayInMonthNumber = i64;
    #[derive(Debug, Clone, PartialEq)]
    pub enum DefinedSymbolSelect {
        PreDefinedSymbol(Box<PreDefinedSymbol>),
        ExternallyDefinedSymbol(Box<ExternallyDefinedSymbol>),
    }
    pub type DimensionCount = i64;
    #[derive(Debug, Clone, PartialEq)]
    pub enum DimensionExtentUsage {
        Origin,
        Target,
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum DraughtingCalloutElement {
        AnnotationTextOccurrence(Box<AnnotationTextOccurrence>),
        AnnotationSymbolOccurrence(Box<AnnotationSymbolOccurrence>),
        AnnotationCurveOccurrence(Box<AnnotationCurveOccurrence>),
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum DraughtingGroupedItem {
        AnnotationOccurrence(Box<AnnotationOccurrence>),
        GeometricSetSelect(Box<GeometricSetSelect>),
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum DraughtingOrganizationItem {
        ProductDefinitionFormation(Box<ProductDefinitionFormation>),
        DrawingRevision(Box<DrawingRevision>),
        DrawingSheetRevision(Box<DrawingSheetRevision>),
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum DraughtingPresentedItemSelect {
        ProductDefinitionFormation(Box<ProductDefinitionFormation>),
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum DraughtingTitledItem {
        DrawingRevision(Box<DrawingRevision>),
        DrawingSheetRevision(Box<DrawingSheetRevision>),
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum FillAreaStyleTileShapeSelect {
        FillAreaStyleTileSymbolWithStyle(Box<FillAreaStyleTileSymbolWithStyle>),
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum FillStyleSelect {
        FillAreaStyleColour(Box<FillAreaStyleColour>),
        ExternallyDefinedTileStyle(Box<ExternallyDefinedTileStyle>),
        FillAreaStyleTiles(Box<FillAreaStyleTiles>),
        ExternallyDefinedHatchStyle(Box<ExternallyDefinedHatchStyle>),
        FillAreaStyleHatching(Box<FillAreaStyleHatching>),
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum FontSelect {
        PreDefinedTextFont(Box<PreDefinedTextFont>),
        ExternallyDefinedTextFont(Box<ExternallyDefinedTextFont>),
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum GeometricSetSelect {
        Point(Box<Point>),
        Curve(Box<Curve>),
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum HidingOrBlankingSelect {
        PresentationArea(Box<PresentationArea>),
        PresentationView(Box<PresentationView>),
        AnnotationFillArea(Box<AnnotationFillArea>),
    }
    pub type Identifier = String;
    #[derive(Debug, Clone, PartialEq)]
    pub enum InvisibilityContext {
        PresentationLayerUsage(Box<PresentationLayerUsage>),
        PresentationRepresentation(Box<PresentationRepresentation>),
        PresentationSet(Box<PresentationSet>),
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum InvisibleItem {
        StyledItem(Box<StyledItem>),
        PresentationLayerAssignment(Box<PresentationLayerAssignment>),
        PresentationRepresentation(Box<PresentationRepresentation>),
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum KnotType {
        UniformKnots,
        QuasiUniformKnots,
        PiecewiseBezierKnots,
        Unspecified,
    }
    pub type Label = String;
    #[derive(Debug, Clone, PartialEq)]
    pub enum LayeredItem {
        PresentationRepresentation(Box<PresentationRepresentation>),
        RepresentationItem(Box<RepresentationItem>),
    }
    pub type LengthMeasure = f64;
    #[derive(Debug, Clone, PartialEq)]
    pub enum MeasureValue {
        LengthMeasure(Box<LengthMeasure>),
        PlaneAngleMeasure(Box<PlaneAngleMeasure>),
        RatioMeasure(Box<RatioMeasure>),
        ParameterValue(Box<ParameterValue>),
        PositiveLengthMeasure(Box<PositiveLengthMeasure>),
        PositiveRatioMeasure(Box<PositiveRatioMeasure>),
    }
    pub type MonthInYearNumber = i64;
    #[derive(Debug, Clone, PartialEq)]
    pub enum NullStyle {
        Null,
    }
    pub type ParameterValue = f64;
    #[derive(Debug, Clone, PartialEq)]
    pub enum PersonOrganizationSelect {
        Person(Box<Person>),
        Organization(Box<Organization>),
        PersonAndOrganization(Box<PersonAndOrganization>),
    }
    pub type PlaneAngleMeasure = f64;
    pub type PositiveLengthMeasure = LengthMeasure;
    pub type PositiveRatioMeasure = RatioMeasure;
    pub type PresentableText = String;
    #[derive(Debug, Clone, PartialEq)]
    pub enum PresentationRepresentationSelect {
        PresentationRepresentation(Box<PresentationRepresentation>),
        PresentationSet(Box<PresentationSet>),
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum PresentationSizeAssignmentSelect {
        PresentationView(Box<PresentationView>),
        PresentationArea(Box<PresentationArea>),
        AreaInSet(Box<AreaInSet>),
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum PresentationStyleSelect {
        CurveStyle(Box<CurveStyle>),
        SymbolStyle(Box<SymbolStyle>),
        FillAreaStyle(Box<FillAreaStyle>),
        TextStyle(Box<TextStyle>),
        NullStyle(Box<NullStyle>),
    }
    pub type RatioMeasure = f64;
    #[derive(Debug, Clone, PartialEq)]
    pub enum ShapeDefinition {
        ProductDefinitionShape(Box<ProductDefinitionShape>),
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum SiPrefix {
        Exa,
        Pico,
        Mega,
        Femto,
        Atto,
        Centi,
        Nano,
        Hecto,
        Micro,
        Tera,
        Giga,
        Milli,
        Peta,
        Deci,
        Kilo,
        Deca,
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum SiUnitName {
        Hertz,
        DegreeCelsius,
        Siemens,
        Sievert,
        Lux,
        Watt,
        Ohm,
        Second,
        Becquerel,
        Pascal,
        Henry,
        Tesla,
        Volt,
        Joule,
        Kelvin,
        Ampere,
        Gram,
        Steradian,
        Mole,
        Lumen,
        Gray,
        Candela,
        Farad,
        Radian,
        Newton,
        Metre,
        Weber,
        Coulomb,
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum SizeSelect {
        PositiveLengthMeasure(Box<PositiveLengthMeasure>),
        MeasureWithUnit(Box<MeasureWithUnit>),
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum SourceItem {
        Identifier(Box<Identifier>),
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum SpecifiedItem {
        DrawingRevision(Box<DrawingRevision>),
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum StyleContextSelect {
        Representation(Box<Representation>),
        RepresentationItem(Box<RepresentationItem>),
        PresentationSet(Box<PresentationSet>),
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum SymbolStyleSelect {
        SymbolColour(Box<SymbolColour>),
    }
    pub type Text = String;
    pub type TextAlignment = Label;
    pub type TextDelineation = Label;
    #[derive(Debug, Clone, PartialEq)]
    pub enum TextOrCharacter {
        AnnotationText(Box<AnnotationText>),
        CompositeText(Box<CompositeText>),
        TextLiteral(Box<TextLiteral>),
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum TextPath {
        Up,
        Right,
        Down,
        Left,
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum TransitionCode {
        Discontinuous,
        ContSameGradientSameCurvature,
        ContSameGradient,
        Continuous,
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum TrimmingPreference {
        Parameter,
        Unspecified,
        Cartesian,
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum TrimmingSelect {
        CartesianPoint(Box<CartesianPoint>),
        ParameterValue(Box<ParameterValue>),
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum Unit {
        NamedUnit(Box<NamedUnit>),
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum VectorOrDirection {
        Vector(Box<Vector>),
        Direction(Box<Direction>),
    }
    pub type YearNumber = i64;
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct Address {
        pub internal_location: Option<Label>,
        pub street_number: Option<Label>,
        pub street: Option<Label>,
        pub postal_box: Option<Label>,
        pub town: Option<Label>,
        pub region: Option<Label>,
        pub postal_code: Option<Label>,
        pub country: Option<Label>,
        pub facsimile_number: Option<Label>,
        pub telephone_number: Option<Label>,
        pub electronic_mail_address: Option<Label>,
        pub telex_number: Option<Label>,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct AngularDimension {
        pub dimension_curve_directed_callout: DimensionCurveDirectedCallout,
    }
    impl ::std::ops::Deref for AngularDimension {
        type Target = DimensionCurveDirectedCallout;
        fn deref(&self) -> &Self::Target {
            &self.dimension_curve_directed_callout
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct AnnotationCurveOccurrence {
        pub annotation_occurrence: AnnotationOccurrence,
    }
    impl ::std::ops::Deref for AnnotationCurveOccurrence {
        type Target = AnnotationOccurrence;
        fn deref(&self) -> &Self::Target {
            &self.annotation_occurrence
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct AnnotationFillArea {
        pub geometric_representation_item: GeometricRepresentationItem,
        pub boundaries: Curve,
    }
    impl ::std::ops::Deref for AnnotationFillArea {
        type Target = GeometricRepresentationItem;
        fn deref(&self) -> &Self::Target {
            &self.geometric_representation_item
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct AnnotationFillAreaOccurrence {
        pub annotation_occurrence: AnnotationOccurrence,
        pub fill_style_target: Point,
    }
    impl ::std::ops::Deref for AnnotationFillAreaOccurrence {
        type Target = AnnotationOccurrence;
        fn deref(&self) -> &Self::Target {
            &self.annotation_occurrence
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct AnnotationOccurrence {
        pub styled_item: StyledItem,
    }
    impl ::std::ops::Deref for AnnotationOccurrence {
        type Target = StyledItem;
        fn deref(&self) -> &Self::Target {
            &self.styled_item
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct AnnotationSubfigureOccurrence {
        pub annotation_symbol_occurrence: AnnotationSymbolOccurrence,
    }
    impl ::std::ops::Deref for AnnotationSubfigureOccurrence {
        type Target = AnnotationSymbolOccurrence;
        fn deref(&self) -> &Self::Target {
            &self.annotation_symbol_occurrence
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct AnnotationSymbol {
        pub mapped_item: MappedItem,
    }
    impl ::std::ops::Deref for AnnotationSymbol {
        type Target = MappedItem;
        fn deref(&self) -> &Self::Target {
            &self.mapped_item
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct AnnotationSymbolOccurrence {
        pub annotation_occurrence: AnnotationOccurrence,
    }
    impl ::std::ops::Deref for AnnotationSymbolOccurrence {
        type Target = AnnotationOccurrence;
        fn deref(&self) -> &Self::Target {
            &self.annotation_occurrence
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct AnnotationText {
        pub mapped_item: MappedItem,
    }
    impl ::std::ops::Deref for AnnotationText {
        type Target = MappedItem;
        fn deref(&self) -> &Self::Target {
            &self.mapped_item
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct AnnotationTextOccurrence {
        pub annotation_occurrence: AnnotationOccurrence,
    }
    impl ::std::ops::Deref for AnnotationTextOccurrence {
        type Target = AnnotationOccurrence;
        fn deref(&self) -> &Self::Target {
            &self.annotation_occurrence
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ApplicationContext {
        pub application: Text,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ApplicationContextElement {
        pub name: Label,
        pub frame_of_reference: ApplicationContext,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ApplicationProtocolDefinition {
        pub status: Label,
        pub application_interpreted_model_schema_name: Label,
        pub application_protocol_year: YearNumber,
        pub application: ApplicationContext,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct Approval {
        pub status: ApprovalStatus,
        pub level: Label,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ApprovalAssignment {
        pub assigned_approval: Approval,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ApprovalDateTime {
        pub date_time: DateTimeSelect,
        pub dated_approval: Approval,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ApprovalPersonOrganization {
        pub person_organization: PersonOrganizationSelect,
        pub authorized_approval: Approval,
        pub role: ApprovalRole,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ApprovalRole {
        pub role: Label,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ApprovalStatus {
        pub name: Label,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct AreaInSet {
        pub area: PresentationArea,
        pub in_set: PresentationSet,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct Axis2Placement2D {
        pub placement: Placement,
        pub ref_direction: Option<Direction>,
    }
    impl ::std::ops::Deref for Axis2Placement2D {
        type Target = Placement;
        fn deref(&self) -> &Self::Target {
            &self.placement
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct BSplineCurve {
        pub bounded_curve: BoundedCurve,
        pub degree: i64,
        pub control_points_list: CartesianPoint,
        pub curve_form: BSplineCurveForm,
        pub closed_curve: Logical,
        pub self_intersect: Logical,
    }
    impl ::std::ops::Deref for BSplineCurve {
        type Target = BoundedCurve;
        fn deref(&self) -> &Self::Target {
            &self.bounded_curve
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct BSplineCurveWithKnots {
        pub b_spline_curve: BSplineCurve,
        pub knot_multiplicities: i64,
        pub knots: ParameterValue,
        pub knot_spec: KnotType,
    }
    impl ::std::ops::Deref for BSplineCurveWithKnots {
        type Target = BSplineCurve;
        fn deref(&self) -> &Self::Target {
            &self.b_spline_curve
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct BezierCurve {
        pub b_spline_curve: BSplineCurve,
    }
    impl ::std::ops::Deref for BezierCurve {
        type Target = BSplineCurve;
        fn deref(&self) -> &Self::Target {
            &self.b_spline_curve
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct BoundedCurve {
        pub curve: Curve,
    }
    impl ::std::ops::Deref for BoundedCurve {
        type Target = Curve;
        fn deref(&self) -> &Self::Target {
            &self.curve
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct CalendarDate {
        pub date: Date,
        pub day_component: DayInMonthNumber,
        pub month_component: MonthInYearNumber,
    }
    impl ::std::ops::Deref for CalendarDate {
        type Target = Date;
        fn deref(&self) -> &Self::Target {
            &self.date
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct CameraImage {
        pub mapped_item: MappedItem,
    }
    impl ::std::ops::Deref for CameraImage {
        type Target = MappedItem;
        fn deref(&self) -> &Self::Target {
            &self.mapped_item
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct CameraImage2DWithScale {
        pub camera_image: CameraImage,
    }
    impl ::std::ops::Deref for CameraImage2DWithScale {
        type Target = CameraImage;
        fn deref(&self) -> &Self::Target {
            &self.camera_image
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct CameraModel {
        pub geometric_representation_item: GeometricRepresentationItem,
    }
    impl ::std::ops::Deref for CameraModel {
        type Target = GeometricRepresentationItem;
        fn deref(&self) -> &Self::Target {
            &self.geometric_representation_item
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct CameraModelD2 {
        pub camera_model: CameraModel,
        pub view_window: PlanarBox,
        pub view_window_clipping: bool,
    }
    impl ::std::ops::Deref for CameraModelD2 {
        type Target = CameraModel;
        fn deref(&self) -> &Self::Target {
            &self.camera_model
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct CameraUsage {
        pub representation_map: RepresentationMap,
    }
    impl ::std::ops::Deref for CameraUsage {
        type Target = RepresentationMap;
        fn deref(&self) -> &Self::Target {
            &self.representation_map
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct CartesianPoint {
        pub point: Point,
        pub coordinates: LengthMeasure,
    }
    impl ::std::ops::Deref for CartesianPoint {
        type Target = Point;
        fn deref(&self) -> &Self::Target {
            &self.point
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct Circle {
        pub conic: Conic,
        pub radius: PositiveLengthMeasure,
    }
    impl ::std::ops::Deref for Circle {
        type Target = Conic;
        fn deref(&self) -> &Self::Target {
            &self.conic
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct Colour {}
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ColourRgb {
        pub colour_specification: ColourSpecification,
        pub red: f64,
        pub green: f64,
        pub blue: f64,
    }
    impl ::std::ops::Deref for ColourRgb {
        type Target = ColourSpecification;
        fn deref(&self) -> &Self::Target {
            &self.colour_specification
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ColourSpecification {
        pub colour: Colour,
        pub name: Colour,
    }
    impl ::std::ops::Deref for ColourSpecification {
        type Target = Colour;
        fn deref(&self) -> &Self::Target {
            &self.colour
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct CompositeCurve {
        pub bounded_curve: BoundedCurve,
        pub segments: CompositeCurveSegment,
        pub self_intersect: Logical,
    }
    impl ::std::ops::Deref for CompositeCurve {
        type Target = BoundedCurve;
        fn deref(&self) -> &Self::Target {
            &self.bounded_curve
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct CompositeCurveSegment {
        pub transition: TransitionCode,
        pub same_sense: bool,
        pub parent_curve: Curve,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct CompositeText {
        pub geometric_representation_item: GeometricRepresentationItem,
        pub collected_text: TextOrCharacter,
    }
    impl ::std::ops::Deref for CompositeText {
        type Target = GeometricRepresentationItem;
        fn deref(&self) -> &Self::Target {
            &self.geometric_representation_item
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct CompositeTextWithAssociatedCurves {
        pub composite_text: CompositeText,
        pub associated_curves: Curve,
    }
    impl ::std::ops::Deref for CompositeTextWithAssociatedCurves {
        type Target = CompositeText;
        fn deref(&self) -> &Self::Target {
            &self.composite_text
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct CompositeTextWithBlankingBox {
        pub composite_text: CompositeText,
        pub blanking: PlanarBox,
    }
    impl ::std::ops::Deref for CompositeTextWithBlankingBox {
        type Target = CompositeText;
        fn deref(&self) -> &Self::Target {
            &self.composite_text
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct CompositeTextWithExtent {
        pub composite_text: CompositeText,
        pub extent: PlanarExtent,
    }
    impl ::std::ops::Deref for CompositeTextWithExtent {
        type Target = CompositeText;
        fn deref(&self) -> &Self::Target {
            &self.composite_text
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct Conic {
        pub curve: Curve,
        pub position: Axis2Placement,
    }
    impl ::std::ops::Deref for Conic {
        type Target = Curve;
        fn deref(&self) -> &Self::Target {
            &self.curve
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ContextDependentInvisibility {
        pub invisibility: Invisibility,
        pub presentation_context: InvisibilityContext,
    }
    impl ::std::ops::Deref for ContextDependentInvisibility {
        type Target = Invisibility;
        fn deref(&self) -> &Self::Target {
            &self.invisibility
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct Contract {
        pub name: Label,
        pub purpose: Text,
        pub kind: ContractType,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ContractAssignment {
        pub assigned_contract: Contract,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ContractType {
        pub description: Label,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ConversionBasedUnit {
        pub named_unit: NamedUnit,
        pub name: Label,
        pub conversion_factor: MeasureWithUnit,
    }
    impl ::std::ops::Deref for ConversionBasedUnit {
        type Target = NamedUnit;
        fn deref(&self) -> &Self::Target {
            &self.named_unit
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct Curve {
        pub geometric_representation_item: GeometricRepresentationItem,
    }
    impl ::std::ops::Deref for Curve {
        type Target = GeometricRepresentationItem;
        fn deref(&self) -> &Self::Target {
            &self.geometric_representation_item
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct CurveDimension {
        pub dimension_curve_directed_callout: DimensionCurveDirectedCallout,
    }
    impl ::std::ops::Deref for CurveDimension {
        type Target = DimensionCurveDirectedCallout;
        fn deref(&self) -> &Self::Target {
            &self.dimension_curve_directed_callout
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct CurveStyle {
        pub name: Label,
        pub curve_font: CurveFontOrScaledCurveFontSelect,
        pub curve_width: SizeSelect,
        pub curve_colour: Colour,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct CurveStyleFont {
        pub name: Label,
        pub pattern_list: CurveStyleFontPattern,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct CurveStyleFontPattern {
        pub visible_segment_length: PositiveLengthMeasure,
        pub invisible_segment_length: PositiveLengthMeasure,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct Date {
        pub year_component: YearNumber,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DatumFeatureCallout {
        pub draughting_callout: DraughtingCallout,
    }
    impl ::std::ops::Deref for DatumFeatureCallout {
        type Target = DraughtingCallout;
        fn deref(&self) -> &Self::Target {
            &self.draughting_callout
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DatumTargetCallout {
        pub draughting_callout: DraughtingCallout,
    }
    impl ::std::ops::Deref for DatumTargetCallout {
        type Target = DraughtingCallout;
        fn deref(&self) -> &Self::Target {
            &self.draughting_callout
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DefinedSymbol {
        pub geometric_representation_item: GeometricRepresentationItem,
        pub definition: DefinedSymbolSelect,
        pub target: SymbolTarget,
    }
    impl ::std::ops::Deref for DefinedSymbol {
        type Target = GeometricRepresentationItem;
        fn deref(&self) -> &Self::Target {
            &self.geometric_representation_item
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DiameterDimension {
        pub dimension_curve_directed_callout: DimensionCurveDirectedCallout,
    }
    impl ::std::ops::Deref for DiameterDimension {
        type Target = DimensionCurveDirectedCallout;
        fn deref(&self) -> &Self::Target {
            &self.dimension_curve_directed_callout
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DimensionCalloutComponentRelationship {
        pub draughting_callout_relationship: DraughtingCalloutRelationship,
    }
    impl ::std::ops::Deref for DimensionCalloutComponentRelationship {
        type Target = DraughtingCalloutRelationship;
        fn deref(&self) -> &Self::Target {
            &self.draughting_callout_relationship
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DimensionCalloutRelationship {
        pub draughting_callout_relationship: DraughtingCalloutRelationship,
    }
    impl ::std::ops::Deref for DimensionCalloutRelationship {
        type Target = DraughtingCalloutRelationship;
        fn deref(&self) -> &Self::Target {
            &self.draughting_callout_relationship
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DimensionCurve {
        pub annotation_curve_occurrence: AnnotationCurveOccurrence,
    }
    impl ::std::ops::Deref for DimensionCurve {
        type Target = AnnotationCurveOccurrence;
        fn deref(&self) -> &Self::Target {
            &self.annotation_curve_occurrence
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DimensionCurveDirectedCallout {
        pub draughting_callout: DraughtingCallout,
    }
    impl ::std::ops::Deref for DimensionCurveDirectedCallout {
        type Target = DraughtingCallout;
        fn deref(&self) -> &Self::Target {
            &self.draughting_callout
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DimensionCurveTerminator {
        pub terminator_symbol: TerminatorSymbol,
        pub role: DimensionExtentUsage,
    }
    impl ::std::ops::Deref for DimensionCurveTerminator {
        type Target = TerminatorSymbol;
        fn deref(&self) -> &Self::Target {
            &self.terminator_symbol
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DimensionPair {
        pub draughting_callout_relationship: DraughtingCalloutRelationship,
    }
    impl ::std::ops::Deref for DimensionPair {
        type Target = DraughtingCalloutRelationship;
        fn deref(&self) -> &Self::Target {
            &self.draughting_callout_relationship
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DimensionalExponents {
        pub length_exponent: f64,
        pub mass_exponent: f64,
        pub time_exponent: f64,
        pub electric_current_exponent: f64,
        pub thermodynamic_temperature_exponent: f64,
        pub amount_of_substance_exponent: f64,
        pub luminous_intensity_exponent: f64,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct Direction {
        pub geometric_representation_item: GeometricRepresentationItem,
        pub direction_ratios: f64,
    }
    impl ::std::ops::Deref for Direction {
        type Target = GeometricRepresentationItem;
        fn deref(&self) -> &Self::Target {
            &self.geometric_representation_item
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct Document {
        pub id: Identifier,
        pub name: Label,
        pub description: Text,
        pub kind: DocumentType,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DocumentReference {
        pub assigned_document: Document,
        pub source: Label,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DocumentType {
        pub product_data_type: Label,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DraughtingAnnotationOccurrence {
        pub annotation_occurrence: AnnotationOccurrence,
    }
    impl ::std::ops::Deref for DraughtingAnnotationOccurrence {
        type Target = AnnotationOccurrence;
        fn deref(&self) -> &Self::Target {
            &self.annotation_occurrence
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DraughtingApprovalAssignment {
        pub approval_assignment: ApprovalAssignment,
        pub approved_items: ApprovedItem,
    }
    impl ::std::ops::Deref for DraughtingApprovalAssignment {
        type Target = ApprovalAssignment;
        fn deref(&self) -> &Self::Target {
            &self.approval_assignment
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DraughtingCallout {
        pub geometric_representation_item: GeometricRepresentationItem,
        pub contents: DraughtingCalloutElement,
    }
    impl ::std::ops::Deref for DraughtingCallout {
        type Target = GeometricRepresentationItem;
        fn deref(&self) -> &Self::Target {
            &self.geometric_representation_item
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DraughtingCalloutRelationship {
        pub name: Label,
        pub description: Text,
        pub relating_draughting_callout: DraughtingCallout,
        pub related_draughting_callout: DraughtingCallout,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DraughtingContractAssignment {
        pub contract_assignment: ContractAssignment,
        pub items: ContractedItem,
    }
    impl ::std::ops::Deref for DraughtingContractAssignment {
        type Target = ContractAssignment;
        fn deref(&self) -> &Self::Target {
            &self.contract_assignment
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DraughtingDrawingRevision {
        pub drawing_revision: DrawingRevision,
    }
    impl ::std::ops::Deref for DraughtingDrawingRevision {
        type Target = DrawingRevision;
        fn deref(&self) -> &Self::Target {
            &self.drawing_revision
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DraughtingElements {
        pub draughting_callout: DraughtingCallout,
    }
    impl ::std::ops::Deref for DraughtingElements {
        type Target = DraughtingCallout;
        fn deref(&self) -> &Self::Target {
            &self.draughting_callout
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DraughtingGroupAssignment {
        pub group_assignment: GroupAssignment,
        pub items: DraughtingGroupedItem,
    }
    impl ::std::ops::Deref for DraughtingGroupAssignment {
        type Target = GroupAssignment;
        fn deref(&self) -> &Self::Target {
            &self.group_assignment
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DraughtingModel {
        pub representation: Representation,
    }
    impl ::std::ops::Deref for DraughtingModel {
        type Target = Representation;
        fn deref(&self) -> &Self::Target {
            &self.representation
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DraughtingOrganizationAssignment {
        pub organization_assignment: OrganizationAssignment,
        pub assigned_items: DraughtingOrganizationItem,
    }
    impl ::std::ops::Deref for DraughtingOrganizationAssignment {
        type Target = OrganizationAssignment;
        fn deref(&self) -> &Self::Target {
            &self.organization_assignment
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DraughtingPersonAndOrganizationAssignment {
        pub person_and_organization_assignment: PersonAndOrganizationAssignment,
        pub assigned_items: DraughtingOrganizationItem,
    }
    impl ::std::ops::Deref for DraughtingPersonAndOrganizationAssignment {
        type Target = PersonAndOrganizationAssignment;
        fn deref(&self) -> &Self::Target {
            &self.person_and_organization_assignment
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DraughtingPersonAssignment {
        pub person_assignment: PersonAssignment,
        pub assigned_items: DraughtingOrganizationItem,
    }
    impl ::std::ops::Deref for DraughtingPersonAssignment {
        type Target = PersonAssignment;
        fn deref(&self) -> &Self::Target {
            &self.person_assignment
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DraughtingPreDefinedColour {
        pub pre_defined_colour: PreDefinedColour,
    }
    impl ::std::ops::Deref for DraughtingPreDefinedColour {
        type Target = PreDefinedColour;
        fn deref(&self) -> &Self::Target {
            &self.pre_defined_colour
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DraughtingPreDefinedCurveFont {
        pub pre_defined_curve_font: PreDefinedCurveFont,
    }
    impl ::std::ops::Deref for DraughtingPreDefinedCurveFont {
        type Target = PreDefinedCurveFont;
        fn deref(&self) -> &Self::Target {
            &self.pre_defined_curve_font
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DraughtingPreDefinedTextFont {
        pub pre_defined_text_font: PreDefinedTextFont,
    }
    impl ::std::ops::Deref for DraughtingPreDefinedTextFont {
        type Target = PreDefinedTextFont;
        fn deref(&self) -> &Self::Target {
            &self.pre_defined_text_font
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DraughtingPresentedItem {
        pub presented_item: PresentedItem,
        pub items: DraughtingPresentedItemSelect,
    }
    impl ::std::ops::Deref for DraughtingPresentedItem {
        type Target = PresentedItem;
        fn deref(&self) -> &Self::Target {
            &self.presented_item
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DraughtingSecurityClassificationAssignment {
        pub security_classification_assignment: SecurityClassificationAssignment,
        pub assigned_items: ClassifiedItem,
    }
    impl ::std::ops::Deref for DraughtingSecurityClassificationAssignment {
        type Target = SecurityClassificationAssignment;
        fn deref(&self) -> &Self::Target {
            &self.security_classification_assignment
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DraughtingSpecificationReference {
        pub document_reference: DocumentReference,
        pub specified_items: SpecifiedItem,
    }
    impl ::std::ops::Deref for DraughtingSpecificationReference {
        type Target = DocumentReference;
        fn deref(&self) -> &Self::Target {
            &self.document_reference
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DraughtingSubfigureRepresentation {
        pub symbol_representation: SymbolRepresentation,
    }
    impl ::std::ops::Deref for DraughtingSubfigureRepresentation {
        type Target = SymbolRepresentation;
        fn deref(&self) -> &Self::Target {
            &self.symbol_representation
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DraughtingSymbolRepresentation {
        pub symbol_representation: SymbolRepresentation,
    }
    impl ::std::ops::Deref for DraughtingSymbolRepresentation {
        type Target = SymbolRepresentation;
        fn deref(&self) -> &Self::Target {
            &self.symbol_representation
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DraughtingTextLiteralWithDelineation {
        pub text_literal_with_delineation: TextLiteralWithDelineation,
    }
    impl ::std::ops::Deref for DraughtingTextLiteralWithDelineation {
        type Target = TextLiteralWithDelineation;
        fn deref(&self) -> &Self::Target {
            &self.text_literal_with_delineation
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DraughtingTitle {
        pub items: DraughtingTitledItem,
        pub language: Label,
        pub contents: Text,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DrawingDefinition {
        pub drawing_number: Identifier,
        pub drawing_type: Option<Label>,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DrawingRevision {
        pub presentation_set: PresentationSet,
        pub revision_identifier: Identifier,
        pub drawing_identifier: DrawingDefinition,
        pub intended_scale: Option<Text>,
    }
    impl ::std::ops::Deref for DrawingRevision {
        type Target = PresentationSet;
        fn deref(&self) -> &Self::Target {
            &self.presentation_set
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DrawingSheetLayout {
        pub draughting_symbol_representation: DraughtingSymbolRepresentation,
    }
    impl ::std::ops::Deref for DrawingSheetLayout {
        type Target = DraughtingSymbolRepresentation;
        fn deref(&self) -> &Self::Target {
            &self.draughting_symbol_representation
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DrawingSheetRevision {
        pub presentation_area: PresentationArea,
        pub revision_identifier: Identifier,
    }
    impl ::std::ops::Deref for DrawingSheetRevision {
        type Target = PresentationArea;
        fn deref(&self) -> &Self::Target {
            &self.presentation_area
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DrawingSheetRevisionUsage {
        pub area_in_set: AreaInSet,
        pub sheet_number: Identifier,
    }
    impl ::std::ops::Deref for DrawingSheetRevisionUsage {
        type Target = AreaInSet;
        fn deref(&self) -> &Self::Target {
            &self.area_in_set
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct Ellipse {
        pub conic: Conic,
        pub semi_axis_1: PositiveLengthMeasure,
        pub semi_axis_2: PositiveLengthMeasure,
    }
    impl ::std::ops::Deref for Ellipse {
        type Target = Conic;
        fn deref(&self) -> &Self::Target {
            &self.conic
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ExternalSource {
        pub source_id: SourceItem,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ExternallyDefinedCurveFont {
        pub externally_defined_item: ExternallyDefinedItem,
    }
    impl ::std::ops::Deref for ExternallyDefinedCurveFont {
        type Target = ExternallyDefinedItem;
        fn deref(&self) -> &Self::Target {
            &self.externally_defined_item
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ExternallyDefinedHatchStyle {
        pub externally_defined_item: ExternallyDefinedItem,
        pub geometric_representation_item: GeometricRepresentationItem,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ExternallyDefinedItem {
        pub item_id: SourceItem,
        pub source: ExternalSource,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ExternallyDefinedSymbol {
        pub externally_defined_item: ExternallyDefinedItem,
    }
    impl ::std::ops::Deref for ExternallyDefinedSymbol {
        type Target = ExternallyDefinedItem;
        fn deref(&self) -> &Self::Target {
            &self.externally_defined_item
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ExternallyDefinedTextFont {
        pub externally_defined_item: ExternallyDefinedItem,
    }
    impl ::std::ops::Deref for ExternallyDefinedTextFont {
        type Target = ExternallyDefinedItem;
        fn deref(&self) -> &Self::Target {
            &self.externally_defined_item
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ExternallyDefinedTileStyle {
        pub externally_defined_item: ExternallyDefinedItem,
        pub geometric_representation_item: GeometricRepresentationItem,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct FillAreaStyle {
        pub name: Label,
        pub fill_styles: FillStyleSelect,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct FillAreaStyleColour {
        pub name: Label,
        pub fill_colour: Colour,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct FillAreaStyleHatching {
        pub geometric_representation_item: GeometricRepresentationItem,
        pub hatch_line_appearance: CurveStyle,
        pub start_of_next_hatch_line: OneDirectionRepeatFactor,
        pub point_of_reference_hatch_line: CartesianPoint,
        pub pattern_start: CartesianPoint,
        pub hatch_line_angle: PlaneAngleMeasure,
    }
    impl ::std::ops::Deref for FillAreaStyleHatching {
        type Target = GeometricRepresentationItem;
        fn deref(&self) -> &Self::Target {
            &self.geometric_representation_item
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct FillAreaStyleTileSymbolWithStyle {
        pub geometric_representation_item: GeometricRepresentationItem,
        pub symbol: AnnotationSymbolOccurrence,
    }
    impl ::std::ops::Deref for FillAreaStyleTileSymbolWithStyle {
        type Target = GeometricRepresentationItem;
        fn deref(&self) -> &Self::Target {
            &self.geometric_representation_item
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct FillAreaStyleTiles {
        pub geometric_representation_item: GeometricRepresentationItem,
        pub tiling_pattern: TwoDirectionRepeatFactor,
        pub tiles: FillAreaStyleTileShapeSelect,
        pub tiling_scale: PositiveRatioMeasure,
    }
    impl ::std::ops::Deref for FillAreaStyleTiles {
        type Target = GeometricRepresentationItem;
        fn deref(&self) -> &Self::Target {
            &self.geometric_representation_item
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct GeometricCurveSet {
        pub geometric_set: GeometricSet,
    }
    impl ::std::ops::Deref for GeometricCurveSet {
        type Target = GeometricSet;
        fn deref(&self) -> &Self::Target {
            &self.geometric_set
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct GeometricRepresentationContext {
        pub representation_context: RepresentationContext,
        pub coordinate_space_dimension: DimensionCount,
    }
    impl ::std::ops::Deref for GeometricRepresentationContext {
        type Target = RepresentationContext;
        fn deref(&self) -> &Self::Target {
            &self.representation_context
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct GeometricRepresentationItem {
        pub representation_item: RepresentationItem,
    }
    impl ::std::ops::Deref for GeometricRepresentationItem {
        type Target = RepresentationItem;
        fn deref(&self) -> &Self::Target {
            &self.representation_item
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct GeometricSet {
        pub geometric_representation_item: GeometricRepresentationItem,
        pub elements: GeometricSetSelect,
    }
    impl ::std::ops::Deref for GeometricSet {
        type Target = GeometricRepresentationItem;
        fn deref(&self) -> &Self::Target {
            &self.geometric_representation_item
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct GeometricalToleranceCallout {
        pub draughting_callout: DraughtingCallout,
    }
    impl ::std::ops::Deref for GeometricalToleranceCallout {
        type Target = DraughtingCallout;
        fn deref(&self) -> &Self::Target {
            &self.draughting_callout
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct GeometricallyBounded2DWireframeRepresentation {
        pub shape_representation: ShapeRepresentation,
    }
    impl ::std::ops::Deref for GeometricallyBounded2DWireframeRepresentation {
        type Target = ShapeRepresentation;
        fn deref(&self) -> &Self::Target {
            &self.shape_representation
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct GlobalUnitAssignedContext {
        pub representation_context: RepresentationContext,
        pub units: Unit,
    }
    impl ::std::ops::Deref for GlobalUnitAssignedContext {
        type Target = RepresentationContext;
        fn deref(&self) -> &Self::Target {
            &self.representation_context
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct Group {
        pub name: Label,
        pub description: Text,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct GroupAssignment {
        pub assigned_group: Group,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct GroupRelationship {
        pub name: Label,
        pub description: Text,
        pub relating_group: Group,
        pub related_group: Group,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct Hyperbola {
        pub conic: Conic,
        pub semi_axis: PositiveLengthMeasure,
        pub semi_imag_axis: PositiveLengthMeasure,
    }
    impl ::std::ops::Deref for Hyperbola {
        type Target = Conic;
        fn deref(&self) -> &Self::Target {
            &self.conic
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct Invisibility {
        pub invisible_items: InvisibleItem,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct LeaderCurve {
        pub annotation_curve_occurrence: AnnotationCurveOccurrence,
    }
    impl ::std::ops::Deref for LeaderCurve {
        type Target = AnnotationCurveOccurrence;
        fn deref(&self) -> &Self::Target {
            &self.annotation_curve_occurrence
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct LeaderDirectedCallout {
        pub draughting_callout: DraughtingCallout,
    }
    impl ::std::ops::Deref for LeaderDirectedCallout {
        type Target = DraughtingCallout;
        fn deref(&self) -> &Self::Target {
            &self.draughting_callout
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct LeaderDirectedDimension {
        pub leader_directed_callout: LeaderDirectedCallout,
    }
    impl ::std::ops::Deref for LeaderDirectedDimension {
        type Target = LeaderDirectedCallout;
        fn deref(&self) -> &Self::Target {
            &self.leader_directed_callout
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct LeaderTerminator {
        pub terminator_symbol: TerminatorSymbol,
    }
    impl ::std::ops::Deref for LeaderTerminator {
        type Target = TerminatorSymbol;
        fn deref(&self) -> &Self::Target {
            &self.terminator_symbol
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct LengthMeasureWithUnit {
        pub measure_with_unit: MeasureWithUnit,
    }
    impl ::std::ops::Deref for LengthMeasureWithUnit {
        type Target = MeasureWithUnit;
        fn deref(&self) -> &Self::Target {
            &self.measure_with_unit
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct LengthUnit {
        pub named_unit: NamedUnit,
    }
    impl ::std::ops::Deref for LengthUnit {
        type Target = NamedUnit;
        fn deref(&self) -> &Self::Target {
            &self.named_unit
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct Line {
        pub curve: Curve,
        pub pnt: CartesianPoint,
        pub dir: Vector,
    }
    impl ::std::ops::Deref for Line {
        type Target = Curve;
        fn deref(&self) -> &Self::Target {
            &self.curve
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct LinearDimension {
        pub dimension_curve_directed_callout: DimensionCurveDirectedCallout,
    }
    impl ::std::ops::Deref for LinearDimension {
        type Target = DimensionCurveDirectedCallout;
        fn deref(&self) -> &Self::Target {
            &self.dimension_curve_directed_callout
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct MappedItem {
        pub representation_item: RepresentationItem,
        pub mapping_source: RepresentationMap,
        pub mapping_target: RepresentationItem,
    }
    impl ::std::ops::Deref for MappedItem {
        type Target = RepresentationItem;
        fn deref(&self) -> &Self::Target {
            &self.representation_item
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct MeasureWithUnit {
        pub value_component: MeasureValue,
        pub unit_component: Unit,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct NamedUnit {
        pub dimensions: DimensionalExponents,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct OffsetCurve2D {
        pub curve: Curve,
        pub basis_curve: Curve,
        pub distance: LengthMeasure,
        pub self_intersect: Logical,
    }
    impl ::std::ops::Deref for OffsetCurve2D {
        type Target = Curve;
        fn deref(&self) -> &Self::Target {
            &self.curve
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct OneDirectionRepeatFactor {
        pub geometric_representation_item: GeometricRepresentationItem,
        pub repeat_factor: Vector,
    }
    impl ::std::ops::Deref for OneDirectionRepeatFactor {
        type Target = GeometricRepresentationItem;
        fn deref(&self) -> &Self::Target {
            &self.geometric_representation_item
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct OrdinateDimension {
        pub projection_directed_callout: ProjectionDirectedCallout,
    }
    impl ::std::ops::Deref for OrdinateDimension {
        type Target = ProjectionDirectedCallout;
        fn deref(&self) -> &Self::Target {
            &self.projection_directed_callout
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct Organization {
        pub id: Option<Identifier>,
        pub name: Label,
        pub description: Text,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct OrganizationAssignment {
        pub assigned_organization: Organization,
        pub role: OrganizationRole,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct OrganizationRole {
        pub name: Label,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct OrganizationalAddress {
        pub address: Address,
        pub organizations: Organization,
        pub description: Text,
    }
    impl ::std::ops::Deref for OrganizationalAddress {
        type Target = Address;
        fn deref(&self) -> &Self::Target {
            &self.address
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct Parabola {
        pub conic: Conic,
        pub focal_dist: LengthMeasure,
    }
    impl ::std::ops::Deref for Parabola {
        type Target = Conic;
        fn deref(&self) -> &Self::Target {
            &self.conic
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct Person {
        pub id: Identifier,
        pub last_name: Option<Label>,
        pub first_name: Option<Label>,
        pub middle_names: Option<Label>,
        pub prefix_titles: Option<Label>,
        pub suffix_titles: Option<Label>,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PersonAndOrganization {
        pub the_person: Person,
        pub the_organization: Organization,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PersonAndOrganizationAssignment {
        pub assigned_person_and_organization: PersonAndOrganization,
        pub role: PersonAndOrganizationRole,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PersonAndOrganizationRole {
        pub name: Label,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PersonAssignment {
        pub assigned_person: Person,
        pub role: PersonRole,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PersonRole {
        pub name: Label,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PersonalAddress {
        pub address: Address,
        pub people: Person,
        pub description: Text,
    }
    impl ::std::ops::Deref for PersonalAddress {
        type Target = Address;
        fn deref(&self) -> &Self::Target {
            &self.address
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct Placement {
        pub geometric_representation_item: GeometricRepresentationItem,
        pub location: CartesianPoint,
    }
    impl ::std::ops::Deref for Placement {
        type Target = GeometricRepresentationItem;
        fn deref(&self) -> &Self::Target {
            &self.geometric_representation_item
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PlanarBox {
        pub planar_extent: PlanarExtent,
        pub placement: Axis2Placement,
    }
    impl ::std::ops::Deref for PlanarBox {
        type Target = PlanarExtent;
        fn deref(&self) -> &Self::Target {
            &self.planar_extent
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PlanarExtent {
        pub geometric_representation_item: GeometricRepresentationItem,
        pub size_in_x: LengthMeasure,
        pub size_in_y: LengthMeasure,
    }
    impl ::std::ops::Deref for PlanarExtent {
        type Target = GeometricRepresentationItem;
        fn deref(&self) -> &Self::Target {
            &self.geometric_representation_item
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PlaneAngleMeasureWithUnit {
        pub measure_with_unit: MeasureWithUnit,
    }
    impl ::std::ops::Deref for PlaneAngleMeasureWithUnit {
        type Target = MeasureWithUnit;
        fn deref(&self) -> &Self::Target {
            &self.measure_with_unit
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PlaneAngleUnit {
        pub named_unit: NamedUnit,
    }
    impl ::std::ops::Deref for PlaneAngleUnit {
        type Target = NamedUnit;
        fn deref(&self) -> &Self::Target {
            &self.named_unit
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct Point {
        pub geometric_representation_item: GeometricRepresentationItem,
    }
    impl ::std::ops::Deref for Point {
        type Target = GeometricRepresentationItem;
        fn deref(&self) -> &Self::Target {
            &self.geometric_representation_item
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PointOnCurve {
        pub point: Point,
        pub basis_curve: Curve,
        pub point_parameter: ParameterValue,
    }
    impl ::std::ops::Deref for PointOnCurve {
        type Target = Point;
        fn deref(&self) -> &Self::Target {
            &self.point
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct Polyline {
        pub bounded_curve: BoundedCurve,
        pub points: CartesianPoint,
    }
    impl ::std::ops::Deref for Polyline {
        type Target = BoundedCurve;
        fn deref(&self) -> &Self::Target {
            &self.bounded_curve
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PreDefinedColour {
        pub pre_defined_item: PreDefinedItem,
        pub colour: Colour,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PreDefinedCurveFont {
        pub pre_defined_item: PreDefinedItem,
    }
    impl ::std::ops::Deref for PreDefinedCurveFont {
        type Target = PreDefinedItem;
        fn deref(&self) -> &Self::Target {
            &self.pre_defined_item
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PreDefinedDimensionSymbol {
        pub pre_defined_symbol: PreDefinedSymbol,
    }
    impl ::std::ops::Deref for PreDefinedDimensionSymbol {
        type Target = PreDefinedSymbol;
        fn deref(&self) -> &Self::Target {
            &self.pre_defined_symbol
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PreDefinedGeometricalToleranceSymbol {
        pub pre_defined_symbol: PreDefinedSymbol,
    }
    impl ::std::ops::Deref for PreDefinedGeometricalToleranceSymbol {
        type Target = PreDefinedSymbol;
        fn deref(&self) -> &Self::Target {
            &self.pre_defined_symbol
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PreDefinedItem {
        pub name: Label,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PreDefinedPointMarkerSymbol {
        pub pre_defined_symbol: PreDefinedSymbol,
    }
    impl ::std::ops::Deref for PreDefinedPointMarkerSymbol {
        type Target = PreDefinedSymbol;
        fn deref(&self) -> &Self::Target {
            &self.pre_defined_symbol
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PreDefinedSymbol {
        pub pre_defined_item: PreDefinedItem,
    }
    impl ::std::ops::Deref for PreDefinedSymbol {
        type Target = PreDefinedItem;
        fn deref(&self) -> &Self::Target {
            &self.pre_defined_item
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PreDefinedTerminatorSymbol {
        pub pre_defined_symbol: PreDefinedSymbol,
    }
    impl ::std::ops::Deref for PreDefinedTerminatorSymbol {
        type Target = PreDefinedSymbol;
        fn deref(&self) -> &Self::Target {
            &self.pre_defined_symbol
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PreDefinedTextFont {
        pub pre_defined_item: PreDefinedItem,
    }
    impl ::std::ops::Deref for PreDefinedTextFont {
        type Target = PreDefinedItem;
        fn deref(&self) -> &Self::Target {
            &self.pre_defined_item
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PresentationArea {
        pub presentation_representation: PresentationRepresentation,
    }
    impl ::std::ops::Deref for PresentationArea {
        type Target = PresentationRepresentation;
        fn deref(&self) -> &Self::Target {
            &self.presentation_representation
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PresentationLayerAssignment {
        pub name: Label,
        pub description: Text,
        pub assigned_items: LayeredItem,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PresentationLayerUsage {
        pub assignment: PresentationLayerAssignment,
        pub presentation: PresentationRepresentation,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PresentationRepresentation {
        pub representation: Representation,
    }
    impl ::std::ops::Deref for PresentationRepresentation {
        type Target = Representation;
        fn deref(&self) -> &Self::Target {
            &self.representation
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PresentationSet {}
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PresentationSize {
        pub unit: PresentationSizeAssignmentSelect,
        pub size: PlanarBox,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PresentationStyleAssignment {
        pub styles: PresentationStyleSelect,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PresentationStyleByContext {
        pub presentation_style_assignment: PresentationStyleAssignment,
        pub style_context: StyleContextSelect,
    }
    impl ::std::ops::Deref for PresentationStyleByContext {
        type Target = PresentationStyleAssignment;
        fn deref(&self) -> &Self::Target {
            &self.presentation_style_assignment
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PresentationView {
        pub presentation_representation: PresentationRepresentation,
    }
    impl ::std::ops::Deref for PresentationView {
        type Target = PresentationRepresentation;
        fn deref(&self) -> &Self::Target {
            &self.presentation_representation
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PresentedItem {}
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PresentedItemRepresentation {
        pub presentation: PresentationRepresentationSelect,
        pub item: PresentedItem,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct Product {
        pub id: Identifier,
        pub name: Label,
        pub description: Text,
        pub frame_of_reference: ProductContext,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ProductContext {
        pub application_context_element: ApplicationContextElement,
        pub discipline_type: Label,
    }
    impl ::std::ops::Deref for ProductContext {
        type Target = ApplicationContextElement;
        fn deref(&self) -> &Self::Target {
            &self.application_context_element
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ProductDefinition {
        pub id: Identifier,
        pub description: Text,
        pub formation: ProductDefinitionFormation,
        pub frame_of_reference: ProductDefinitionContext,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ProductDefinitionContext {
        pub application_context_element: ApplicationContextElement,
        pub life_cycle_stage: Label,
    }
    impl ::std::ops::Deref for ProductDefinitionContext {
        type Target = ApplicationContextElement;
        fn deref(&self) -> &Self::Target {
            &self.application_context_element
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ProductDefinitionFormation {
        pub id: Identifier,
        pub description: Text,
        pub of_product: Product,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ProductDefinitionShape {
        pub property_definition: PropertyDefinition,
    }
    impl ::std::ops::Deref for ProductDefinitionShape {
        type Target = PropertyDefinition;
        fn deref(&self) -> &Self::Target {
            &self.property_definition
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ProjectionCurve {
        pub annotation_curve_occurrence: AnnotationCurveOccurrence,
    }
    impl ::std::ops::Deref for ProjectionCurve {
        type Target = AnnotationCurveOccurrence;
        fn deref(&self) -> &Self::Target {
            &self.annotation_curve_occurrence
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ProjectionDirectedCallout {
        pub draughting_callout: DraughtingCallout,
    }
    impl ::std::ops::Deref for ProjectionDirectedCallout {
        type Target = DraughtingCallout;
        fn deref(&self) -> &Self::Target {
            &self.draughting_callout
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PropertyDefinition {
        pub name: Label,
        pub description: Text,
        pub definition: CharacterizedDefinition,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PropertyDefinitionRepresentation {
        pub definition: PropertyDefinition,
        pub used_representation: Representation,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct QuasiUniformCurve {
        pub b_spline_curve: BSplineCurve,
    }
    impl ::std::ops::Deref for QuasiUniformCurve {
        type Target = BSplineCurve;
        fn deref(&self) -> &Self::Target {
            &self.b_spline_curve
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct RadiusDimension {
        pub dimension_curve_directed_callout: DimensionCurveDirectedCallout,
    }
    impl ::std::ops::Deref for RadiusDimension {
        type Target = DimensionCurveDirectedCallout;
        fn deref(&self) -> &Self::Target {
            &self.dimension_curve_directed_callout
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct RationalBSplineCurve {
        pub b_spline_curve: BSplineCurve,
        pub weights_data: f64,
    }
    impl ::std::ops::Deref for RationalBSplineCurve {
        type Target = BSplineCurve;
        fn deref(&self) -> &Self::Target {
            &self.b_spline_curve
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct Representation {
        pub name: Label,
        pub items: RepresentationItem,
        pub context_of_items: RepresentationContext,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct RepresentationContext {
        pub context_identifier: Identifier,
        pub context_type: Text,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct RepresentationItem {
        pub name: Label,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct RepresentationMap {
        pub mapping_origin: RepresentationItem,
        pub mapped_representation: Representation,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct SecurityClassification {
        pub name: Label,
        pub purpose: Text,
        pub security_level: SecurityClassificationLevel,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct SecurityClassificationAssignment {
        pub assigned_security_classification: SecurityClassification,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct SecurityClassificationLevel {
        pub name: Label,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ShapeDefinitionRepresentation {
        pub property_definition_representation: PropertyDefinitionRepresentation,
    }
    impl ::std::ops::Deref for ShapeDefinitionRepresentation {
        type Target = PropertyDefinitionRepresentation;
        fn deref(&self) -> &Self::Target {
            &self.property_definition_representation
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ShapeRepresentation {
        pub representation: Representation,
    }
    impl ::std::ops::Deref for ShapeRepresentation {
        type Target = Representation;
        fn deref(&self) -> &Self::Target {
            &self.representation
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct SiUnit {
        pub named_unit: NamedUnit,
        pub prefix: Option<SiPrefix>,
        pub name: SiUnitName,
    }
    impl ::std::ops::Deref for SiUnit {
        type Target = NamedUnit;
        fn deref(&self) -> &Self::Target {
            &self.named_unit
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct StructuredDimensionCallout {
        pub draughting_callout: DraughtingCallout,
    }
    impl ::std::ops::Deref for StructuredDimensionCallout {
        type Target = DraughtingCallout;
        fn deref(&self) -> &Self::Target {
            &self.draughting_callout
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct StyledItem {
        pub representation_item: RepresentationItem,
        pub styles: PresentationStyleAssignment,
        pub item: RepresentationItem,
    }
    impl ::std::ops::Deref for StyledItem {
        type Target = RepresentationItem;
        fn deref(&self) -> &Self::Target {
            &self.representation_item
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct SymbolColour {
        pub colour_of_symbol: Colour,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct SymbolRepresentation {
        pub representation: Representation,
    }
    impl ::std::ops::Deref for SymbolRepresentation {
        type Target = Representation;
        fn deref(&self) -> &Self::Target {
            &self.representation
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct SymbolRepresentationMap {
        pub representation_map: RepresentationMap,
    }
    impl ::std::ops::Deref for SymbolRepresentationMap {
        type Target = RepresentationMap;
        fn deref(&self) -> &Self::Target {
            &self.representation_map
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct SymbolStyle {
        pub name: Label,
        pub style_of_symbol: SymbolStyleSelect,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct SymbolTarget {
        pub geometric_representation_item: GeometricRepresentationItem,
        pub placement: Axis2Placement,
        pub x_scale: PositiveRatioMeasure,
        pub y_scale: PositiveRatioMeasure,
    }
    impl ::std::ops::Deref for SymbolTarget {
        type Target = GeometricRepresentationItem;
        fn deref(&self) -> &Self::Target {
            &self.geometric_representation_item
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct TerminatorSymbol {
        pub annotation_symbol_occurrence: AnnotationSymbolOccurrence,
        pub annotated_curve: AnnotationCurveOccurrence,
    }
    impl ::std::ops::Deref for TerminatorSymbol {
        type Target = AnnotationSymbolOccurrence;
        fn deref(&self) -> &Self::Target {
            &self.annotation_symbol_occurrence
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct TextLiteral {
        pub geometric_representation_item: GeometricRepresentationItem,
        pub literal: PresentableText,
        pub placement: Axis2Placement,
        pub alignment: TextAlignment,
        pub path: TextPath,
        pub font: FontSelect,
    }
    impl ::std::ops::Deref for TextLiteral {
        type Target = GeometricRepresentationItem;
        fn deref(&self) -> &Self::Target {
            &self.geometric_representation_item
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct TextLiteralWithAssociatedCurves {
        pub text_literal: TextLiteral,
        pub associated_curves: Curve,
    }
    impl ::std::ops::Deref for TextLiteralWithAssociatedCurves {
        type Target = TextLiteral;
        fn deref(&self) -> &Self::Target {
            &self.text_literal
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct TextLiteralWithBlankingBox {
        pub text_literal: TextLiteral,
        pub blanking: PlanarBox,
    }
    impl ::std::ops::Deref for TextLiteralWithBlankingBox {
        type Target = TextLiteral;
        fn deref(&self) -> &Self::Target {
            &self.text_literal
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct TextLiteralWithDelineation {
        pub text_literal: TextLiteral,
        pub delineation: TextDelineation,
    }
    impl ::std::ops::Deref for TextLiteralWithDelineation {
        type Target = TextLiteral;
        fn deref(&self) -> &Self::Target {
            &self.text_literal
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct TextLiteralWithExtent {
        pub text_literal: TextLiteral,
        pub extent: PlanarExtent,
    }
    impl ::std::ops::Deref for TextLiteralWithExtent {
        type Target = TextLiteral;
        fn deref(&self) -> &Self::Target {
            &self.text_literal
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct TextStyle {
        pub name: Label,
        pub character_appearance: CharacterStyleSelect,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct TextStyleForDefinedFont {
        pub text_colour: Colour,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct TextStyleWithBoxCharacteristics {
        pub text_style: TextStyle,
        pub characteristics: BoxCharacteristicSelect,
    }
    impl ::std::ops::Deref for TextStyleWithBoxCharacteristics {
        type Target = TextStyle;
        fn deref(&self) -> &Self::Target {
            &self.text_style
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct TextStyleWithMirror {
        pub text_style: TextStyle,
        pub mirror_placement: Axis2Placement,
    }
    impl ::std::ops::Deref for TextStyleWithMirror {
        type Target = TextStyle;
        fn deref(&self) -> &Self::Target {
            &self.text_style
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct TrimmedCurve {
        pub bounded_curve: BoundedCurve,
        pub basis_curve: Curve,
        pub trim_1: TrimmingSelect,
        pub trim_2: TrimmingSelect,
        pub sense_agreement: bool,
        pub master_representation: TrimmingPreference,
    }
    impl ::std::ops::Deref for TrimmedCurve {
        type Target = BoundedCurve;
        fn deref(&self) -> &Self::Target {
            &self.bounded_curve
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct TwoDirectionRepeatFactor {
        pub one_direction_repeat_factor: OneDirectionRepeatFactor,
        pub second_repeat_factor: Vector,
    }
    impl ::std::ops::Deref for TwoDirectionRepeatFactor {
        type Target = OneDirectionRepeatFactor;
        fn deref(&self) -> &Self::Target {
            &self.one_direction_repeat_factor
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct UniformCurve {
        pub b_spline_curve: BSplineCurve,
    }
    impl ::std::ops::Deref for UniformCurve {
        type Target = BSplineCurve;
        fn deref(&self) -> &Self::Target {
            &self.b_spline_curve
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct Vector {
        pub geometric_representation_item: GeometricRepresentationItem,
        pub orientation: Direction,
        pub magnitude: LengthMeasure,
    }
    impl ::std::ops::Deref for Vector {
        type Target = GeometricRepresentationItem;
        fn deref(&self) -> &Self::Target {
            &self.geometric_representation_item
        }
    }
}

#![allow(dead_code)]
pub mod explicit_draughting {
    use crate::primitive::*;
    #[derive(Debug)]
    pub enum ApprovedItem {
        DrawingRevision(Box<DrawingRevision>),
        DrawingSheetRevision(Box<DrawingSheetRevision>),
    }
    #[derive(Debug)]
    pub enum AreaOrView {
        PresentationArea(Box<PresentationArea>),
        PresentationView(Box<PresentationView>),
    }
    #[derive(Debug)]
    pub enum Axis2Placement {
        Axis2Placement2D(Box<Axis2Placement2D>),
    }
    #[derive(Debug)]
    pub enum BSplineCurveForm {
        EllipticArc,
        PolylineForm,
        ParabolicArc,
        CircularArc,
        Unspecified,
        HyperbolicArc,
    }
    #[derive(Debug)]
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
    #[derive(Debug)]
    pub enum CharacterSpacingSelect {
        LengthMeasure(Box<LengthMeasure>),
        RatioMeasure(Box<RatioMeasure>),
        MeasureWithUnit(Box<dyn MeasureWithUnitAny>),
    }
    #[derive(Debug)]
    pub enum CharacterStyleSelect {
        TextStyleForDefinedFont(Box<TextStyleForDefinedFont>),
    }
    #[derive(Debug)]
    pub enum CharacterizedDefinition {
        CharacterizedProductDefinition(Box<CharacterizedProductDefinition>),
        ShapeDefinition(Box<ShapeDefinition>),
    }
    #[derive(Debug)]
    pub enum CharacterizedProductDefinition {
        ProductDefinition(Box<ProductDefinition>),
    }
    #[derive(Debug)]
    pub enum ClassifiedItem {
        DrawingRevision(Box<DrawingRevision>),
        DrawingSheetRevision(Box<DrawingSheetRevision>),
    }
    #[derive(Debug)]
    pub enum ContractedItem {
        DrawingRevision(Box<DrawingRevision>),
    }
    #[derive(Debug)]
    pub enum CurveFontOrScaledCurveFontSelect {
        CurveStyleFontSelect(Box<CurveStyleFontSelect>),
    }
    #[derive(Debug)]
    pub enum CurveOrAnnotationCurveOccurrence {
        Curve(Box<dyn CurveAny>),
        AnnotationCurveOccurrence(Box<AnnotationCurveOccurrence>),
    }
    #[derive(Debug)]
    pub enum CurveOrRender {
        CurveStyle(Box<CurveStyle>),
    }
    #[derive(Debug)]
    pub enum CurveStyleFontSelect {
        CurveStyleFont(Box<CurveStyleFont>),
        PreDefinedCurveFont(Box<PreDefinedCurveFont>),
        ExternallyDefinedCurveFont(Box<ExternallyDefinedCurveFont>),
    }
    #[derive(Debug)]
    pub enum DateTimeSelect {
        Date(Box<dyn DateAny>),
    }
    pub type DayInMonthNumber = i64;
    #[derive(Debug)]
    pub enum DefinedSymbolSelect {
        PreDefinedSymbol(Box<PreDefinedSymbol>),
        ExternallyDefinedSymbol(Box<ExternallyDefinedSymbol>),
    }
    pub type DimensionCount = i64;
    #[derive(Debug)]
    pub enum DimensionExtentUsage {
        Origin,
        Target,
    }
    #[derive(Debug)]
    pub enum DraughtingCalloutElement {
        AnnotationTextOccurrence(Box<AnnotationTextOccurrence>),
        AnnotationSymbolOccurrence(Box<AnnotationSymbolOccurrence>),
        AnnotationCurveOccurrence(Box<AnnotationCurveOccurrence>),
    }
    #[derive(Debug)]
    pub enum DraughtingGroupedItem {
        AnnotationOccurrence(Box<dyn AnnotationOccurrenceAny>),
        GeometricSetSelect(Box<GeometricSetSelect>),
    }
    #[derive(Debug)]
    pub enum DraughtingOrganizationItem {
        ProductDefinitionFormation(Box<ProductDefinitionFormation>),
        DrawingRevision(Box<DrawingRevision>),
        DrawingSheetRevision(Box<DrawingSheetRevision>),
    }
    #[derive(Debug)]
    pub enum DraughtingPresentedItemSelect {
        ProductDefinitionFormation(Box<ProductDefinitionFormation>),
    }
    #[derive(Debug)]
    pub enum DraughtingTitledItem {
        DrawingRevision(Box<DrawingRevision>),
        DrawingSheetRevision(Box<DrawingSheetRevision>),
    }
    #[derive(Debug)]
    pub enum FillAreaStyleTileShapeSelect {
        FillAreaStyleTileSymbolWithStyle(Box<FillAreaStyleTileSymbolWithStyle>),
    }
    #[derive(Debug)]
    pub enum FillStyleSelect {
        FillAreaStyleColour(Box<FillAreaStyleColour>),
        ExternallyDefinedTileStyle(Box<ExternallyDefinedTileStyle>),
        FillAreaStyleTiles(Box<FillAreaStyleTiles>),
        ExternallyDefinedHatchStyle(Box<ExternallyDefinedHatchStyle>),
        FillAreaStyleHatching(Box<FillAreaStyleHatching>),
    }
    #[derive(Debug)]
    pub enum FontSelect {
        PreDefinedTextFont(Box<PreDefinedTextFont>),
        ExternallyDefinedTextFont(Box<ExternallyDefinedTextFont>),
    }
    #[derive(Debug)]
    pub enum GeometricSetSelect {
        Point(Box<dyn PointAny>),
        Curve(Box<dyn CurveAny>),
    }
    #[derive(Debug)]
    pub enum HidingOrBlankingSelect {
        PresentationArea(Box<PresentationArea>),
        PresentationView(Box<PresentationView>),
        AnnotationFillArea(Box<AnnotationFillArea>),
    }
    pub type Identifier = String;
    #[derive(Debug)]
    pub enum InvisibilityContext {
        PresentationLayerUsage(Box<PresentationLayerUsage>),
        PresentationRepresentation(Box<PresentationRepresentation>),
        PresentationSet(Box<PresentationSet>),
    }
    #[derive(Debug)]
    pub enum InvisibleItem {
        StyledItem(Box<StyledItem>),
        PresentationLayerAssignment(Box<PresentationLayerAssignment>),
        PresentationRepresentation(Box<PresentationRepresentation>),
    }
    #[derive(Debug)]
    pub enum KnotType {
        UniformKnots,
        QuasiUniformKnots,
        PiecewiseBezierKnots,
        Unspecified,
    }
    pub type Label = String;
    #[derive(Debug)]
    pub enum LayeredItem {
        PresentationRepresentation(Box<PresentationRepresentation>),
        RepresentationItem(Box<RepresentationItem>),
    }
    pub type LengthMeasure = f64;
    #[derive(Debug)]
    pub enum MeasureValue {
        LengthMeasure(Box<LengthMeasure>),
        PlaneAngleMeasure(Box<PlaneAngleMeasure>),
        RatioMeasure(Box<RatioMeasure>),
        ParameterValue(Box<ParameterValue>),
        PositiveLengthMeasure(Box<PositiveLengthMeasure>),
        PositiveRatioMeasure(Box<PositiveRatioMeasure>),
    }
    pub type MonthInYearNumber = i64;
    #[derive(Debug)]
    pub enum NullStyle {
        Null,
    }
    pub type ParameterValue = f64;
    #[derive(Debug)]
    pub enum PersonOrganizationSelect {
        Person(Box<Person>),
        Organization(Box<Organization>),
        PersonAndOrganization(Box<PersonAndOrganization>),
    }
    pub type PlaneAngleMeasure = f64;
    pub type PositiveLengthMeasure = LengthMeasure;
    pub type PositiveRatioMeasure = RatioMeasure;
    pub type PresentableText = String;
    #[derive(Debug)]
    pub enum PresentationRepresentationSelect {
        PresentationRepresentation(Box<PresentationRepresentation>),
        PresentationSet(Box<PresentationSet>),
    }
    #[derive(Debug)]
    pub enum PresentationSizeAssignmentSelect {
        PresentationView(Box<PresentationView>),
        PresentationArea(Box<PresentationArea>),
        AreaInSet(Box<AreaInSet>),
    }
    #[derive(Debug)]
    pub enum PresentationStyleSelect {
        CurveStyle(Box<CurveStyle>),
        SymbolStyle(Box<SymbolStyle>),
        FillAreaStyle(Box<FillAreaStyle>),
        TextStyle(Box<TextStyle>),
        NullStyle(Box<NullStyle>),
    }
    pub type RatioMeasure = f64;
    #[derive(Debug)]
    pub enum ShapeDefinition {
        ProductDefinitionShape(Box<ProductDefinitionShape>),
    }
    #[derive(Debug)]
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
    #[derive(Debug)]
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
    #[derive(Debug)]
    pub enum SizeSelect {
        PositiveLengthMeasure(Box<PositiveLengthMeasure>),
        MeasureWithUnit(Box<dyn MeasureWithUnitAny>),
    }
    #[derive(Debug)]
    pub enum SourceItem {
        Identifier(Box<Identifier>),
    }
    #[derive(Debug)]
    pub enum SpecifiedItem {
        DrawingRevision(Box<DrawingRevision>),
    }
    #[derive(Debug)]
    pub enum StyleContextSelect {
        Representation(Box<Representation>),
        RepresentationItem(Box<RepresentationItem>),
        PresentationSet(Box<PresentationSet>),
    }
    #[derive(Debug)]
    pub enum SymbolStyleSelect {
        SymbolColour(Box<SymbolColour>),
    }
    pub type Text = String;
    pub type TextAlignment = Label;
    pub type TextDelineation = Label;
    #[derive(Debug)]
    pub enum TextOrCharacter {
        AnnotationText(Box<AnnotationText>),
        CompositeText(Box<CompositeText>),
        TextLiteral(Box<TextLiteral>),
    }
    #[derive(Debug)]
    pub enum TextPath {
        Up,
        Right,
        Down,
        Left,
    }
    #[derive(Debug)]
    pub enum TransitionCode {
        Discontinuous,
        ContSameGradientSameCurvature,
        ContSameGradient,
        Continuous,
    }
    #[derive(Debug)]
    pub enum TrimmingPreference {
        Parameter,
        Unspecified,
        Cartesian,
    }
    #[derive(Debug)]
    pub enum TrimmingSelect {
        CartesianPoint(Box<CartesianPoint>),
        ParameterValue(Box<ParameterValue>),
    }
    #[derive(Debug)]
    pub enum Unit {
        NamedUnit(Box<dyn NamedUnitAny>),
    }
    #[derive(Debug)]
    pub enum VectorOrDirection {
        Vector(Box<Vector>),
        Direction(Box<Direction>),
    }
    pub type YearNumber = i64;
    #[derive(Debug, derive_new :: new)]
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
    #[derive(Debug, derive_new :: new)]
    pub struct AngularDimension {
        pub dimension_curve_directed_callout: DimensionCurveDirectedCallout,
    }
    impl AnnotationOccurrenceAny for AnnotationCurveOccurrence {}
    #[derive(Debug, derive_new :: new)]
    pub struct AnnotationCurveOccurrence {
        pub annotation_occurrence: Box<dyn AnnotationOccurrenceAny>,
    }
    impl GeometricRepresentationItemAny for AnnotationFillArea {}
    #[derive(Debug, derive_new :: new)]
    pub struct AnnotationFillArea {
        pub boundaries: Vec<Box<dyn CurveAny>>,
        pub geometric_representation_item: Box<dyn GeometricRepresentationItemAny>,
    }
    impl AnnotationOccurrenceAny for AnnotationFillAreaOccurrence {}
    #[derive(Debug, derive_new :: new)]
    pub struct AnnotationFillAreaOccurrence {
        pub fill_style_target: Box<dyn PointAny>,
        pub annotation_occurrence: Box<dyn AnnotationOccurrenceAny>,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct AnnotationOccurrence {
        pub styled_item: StyledItem,
    }
    pub trait AnnotationOccurrenceAny:
        ::std::any::Any + ::std::fmt::Debug + dyn_clone::DynClone
    {
    }
    dyn_clone::clone_trait_object!(AnnotationOccurrenceAny);
    impl AnnotationOccurrenceAny for AnnotationOccurrence {}
    #[derive(Debug, derive_new :: new)]
    pub struct AnnotationSubfigureOccurrence {
        pub annotation_symbol_occurrence: AnnotationSymbolOccurrence,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct AnnotationSymbol {
        pub mapped_item: MappedItem,
    }
    impl AnnotationOccurrenceAny for AnnotationSymbolOccurrence {}
    #[derive(Debug, derive_new :: new)]
    pub struct AnnotationSymbolOccurrence {
        pub annotation_occurrence: Box<dyn AnnotationOccurrenceAny>,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct AnnotationText {
        pub mapped_item: MappedItem,
    }
    impl AnnotationOccurrenceAny for AnnotationTextOccurrence {}
    #[derive(Debug, derive_new :: new)]
    pub struct AnnotationTextOccurrence {
        pub annotation_occurrence: Box<dyn AnnotationOccurrenceAny>,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct ApplicationContext {
        pub application: Text,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct ApplicationContextElement {
        pub name: Label,
        pub frame_of_reference: ApplicationContext,
    }
    pub trait ApplicationContextElementAny:
        ::std::any::Any + ::std::fmt::Debug + dyn_clone::DynClone
    {
    }
    dyn_clone::clone_trait_object!(ApplicationContextElementAny);
    impl ApplicationContextElementAny for ApplicationContextElement {}
    #[derive(Debug, derive_new :: new)]
    pub struct ApplicationProtocolDefinition {
        pub status: Label,
        pub application_interpreted_model_schema_name: Label,
        pub application_protocol_year: YearNumber,
        pub application: ApplicationContext,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct Approval {
        pub status: ApprovalStatus,
        pub level: Label,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct ApprovalAssignment {
        pub assigned_approval: Approval,
    }
    pub trait ApprovalAssignmentAny:
        ::std::any::Any + ::std::fmt::Debug + dyn_clone::DynClone
    {
    }
    dyn_clone::clone_trait_object!(ApprovalAssignmentAny);
    impl ApprovalAssignmentAny for ApprovalAssignment {}
    #[derive(Debug, derive_new :: new)]
    pub struct ApprovalDateTime {
        pub date_time: DateTimeSelect,
        pub dated_approval: Approval,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct ApprovalPersonOrganization {
        pub person_organization: PersonOrganizationSelect,
        pub authorized_approval: Approval,
        pub role: ApprovalRole,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct ApprovalRole {
        pub role: Label,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct ApprovalStatus {
        pub name: Label,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct AreaInSet {
        pub area: PresentationArea,
        pub in_set: PresentationSet,
    }
    impl PlacementAny for Axis2Placement2D {}
    #[derive(Debug, derive_new :: new)]
    pub struct Axis2Placement2D {
        pub ref_direction: Option<Direction>,
        pub placement: Box<dyn PlacementAny>,
    }
    impl BoundedCurveAny for BSplineCurve {}
    #[derive(Debug, derive_new :: new)]
    pub struct BSplineCurve {
        pub degree: i64,
        pub control_points_list: Vec<CartesianPoint>,
        pub curve_form: BSplineCurveForm,
        pub closed_curve: Logical,
        pub self_intersect: Logical,
        pub bounded_curve: Box<dyn BoundedCurveAny>,
    }
    pub trait BSplineCurveAny: ::std::any::Any + ::std::fmt::Debug + dyn_clone::DynClone {}
    dyn_clone::clone_trait_object!(BSplineCurveAny);
    impl BSplineCurveAny for BSplineCurve {}
    impl BSplineCurveAny for BSplineCurveWithKnots {}
    #[derive(Debug, derive_new :: new)]
    pub struct BSplineCurveWithKnots {
        pub knot_multiplicities: Vec<i64>,
        pub knots: Vec<ParameterValue>,
        pub knot_spec: KnotType,
        pub b_spline_curve: Box<dyn BSplineCurveAny>,
    }
    impl BSplineCurveAny for BezierCurve {}
    #[derive(Debug, derive_new :: new)]
    pub struct BezierCurve {
        pub b_spline_curve: Box<dyn BSplineCurveAny>,
    }
    impl CurveAny for BoundedCurve {}
    #[derive(Debug, derive_new :: new)]
    pub struct BoundedCurve {
        pub curve: Box<dyn CurveAny>,
    }
    pub trait BoundedCurveAny: ::std::any::Any + ::std::fmt::Debug + dyn_clone::DynClone {}
    dyn_clone::clone_trait_object!(BoundedCurveAny);
    impl BoundedCurveAny for BoundedCurve {}
    impl DateAny for CalendarDate {}
    #[derive(Debug, derive_new :: new)]
    pub struct CalendarDate {
        pub day_component: DayInMonthNumber,
        pub month_component: MonthInYearNumber,
        pub date: Box<dyn DateAny>,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct CameraImage {
        pub mapped_item: MappedItem,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct CameraImage2DWithScale {
        pub camera_image: CameraImage,
    }
    impl GeometricRepresentationItemAny for CameraModel {}
    #[derive(Debug, derive_new :: new)]
    pub struct CameraModel {
        pub geometric_representation_item: Box<dyn GeometricRepresentationItemAny>,
    }
    pub trait CameraModelAny: ::std::any::Any + ::std::fmt::Debug + dyn_clone::DynClone {}
    dyn_clone::clone_trait_object!(CameraModelAny);
    impl CameraModelAny for CameraModel {}
    impl CameraModelAny for CameraModelD2 {}
    #[derive(Debug, derive_new :: new)]
    pub struct CameraModelD2 {
        pub view_window: PlanarBox,
        pub view_window_clipping: bool,
        pub camera_model: Box<dyn CameraModelAny>,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct CameraUsage {
        pub representation_map: RepresentationMap,
    }
    impl PointAny for CartesianPoint {}
    #[derive(Debug, derive_new :: new)]
    pub struct CartesianPoint {
        pub coordinates: Vec<LengthMeasure>,
        pub point: Box<dyn PointAny>,
    }
    impl ConicAny for Circle {}
    #[derive(Debug, derive_new :: new)]
    pub struct Circle {
        pub radius: PositiveLengthMeasure,
        pub conic: Box<dyn ConicAny>,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct Colour {}
    #[derive(Debug, derive_new :: new)]
    pub struct ColourRgb {
        pub red: f64,
        pub green: f64,
        pub blue: f64,
        pub colour_specification: ColourSpecification,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct ColourSpecification {
        pub name: Colour,
        pub colour: Colour,
    }
    impl BoundedCurveAny for CompositeCurve {}
    #[derive(Debug, derive_new :: new)]
    pub struct CompositeCurve {
        pub segments: Vec<CompositeCurveSegment>,
        pub self_intersect: Logical,
        pub bounded_curve: Box<dyn BoundedCurveAny>,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct CompositeCurveSegment {
        pub transition: TransitionCode,
        pub same_sense: bool,
        pub parent_curve: Box<dyn CurveAny>,
    }
    impl GeometricRepresentationItemAny for CompositeText {}
    #[derive(Debug, derive_new :: new)]
    pub struct CompositeText {
        pub collected_text: Vec<TextOrCharacter>,
        pub geometric_representation_item: Box<dyn GeometricRepresentationItemAny>,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct CompositeTextWithAssociatedCurves {
        pub associated_curves: Vec<Box<dyn CurveAny>>,
        pub composite_text: CompositeText,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct CompositeTextWithBlankingBox {
        pub blanking: PlanarBox,
        pub composite_text: CompositeText,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct CompositeTextWithExtent {
        pub extent: PlanarExtent,
        pub composite_text: CompositeText,
    }
    impl CurveAny for Conic {}
    #[derive(Debug, derive_new :: new)]
    pub struct Conic {
        pub position: Axis2Placement,
        pub curve: Box<dyn CurveAny>,
    }
    pub trait ConicAny: ::std::any::Any + ::std::fmt::Debug + dyn_clone::DynClone {}
    dyn_clone::clone_trait_object!(ConicAny);
    impl ConicAny for Conic {}
    #[derive(Debug, derive_new :: new)]
    pub struct ContextDependentInvisibility {
        pub presentation_context: InvisibilityContext,
        pub invisibility: Invisibility,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct Contract {
        pub name: Label,
        pub purpose: Text,
        pub kind: ContractType,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct ContractAssignment {
        pub assigned_contract: Contract,
    }
    pub trait ContractAssignmentAny:
        ::std::any::Any + ::std::fmt::Debug + dyn_clone::DynClone
    {
    }
    dyn_clone::clone_trait_object!(ContractAssignmentAny);
    impl ContractAssignmentAny for ContractAssignment {}
    #[derive(Debug, derive_new :: new)]
    pub struct ContractType {
        pub description: Label,
    }
    impl NamedUnitAny for ConversionBasedUnit {}
    #[derive(Debug, derive_new :: new)]
    pub struct ConversionBasedUnit {
        pub name: Label,
        pub conversion_factor: Box<dyn MeasureWithUnitAny>,
        pub named_unit: Box<dyn NamedUnitAny>,
    }
    impl GeometricRepresentationItemAny for Curve {}
    #[derive(Debug, derive_new :: new)]
    pub struct Curve {
        pub geometric_representation_item: Box<dyn GeometricRepresentationItemAny>,
    }
    pub trait CurveAny: ::std::any::Any + ::std::fmt::Debug + dyn_clone::DynClone {}
    dyn_clone::clone_trait_object!(CurveAny);
    impl CurveAny for Curve {}
    #[derive(Debug, derive_new :: new)]
    pub struct CurveDimension {
        pub dimension_curve_directed_callout: DimensionCurveDirectedCallout,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct CurveStyle {
        pub name: Label,
        pub curve_font: CurveFontOrScaledCurveFontSelect,
        pub curve_width: SizeSelect,
        pub curve_colour: Colour,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct CurveStyleFont {
        pub name: Label,
        pub pattern_list: Vec<CurveStyleFontPattern>,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct CurveStyleFontPattern {
        pub visible_segment_length: PositiveLengthMeasure,
        pub invisible_segment_length: PositiveLengthMeasure,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct Date {
        pub year_component: YearNumber,
    }
    pub trait DateAny: ::std::any::Any + ::std::fmt::Debug + dyn_clone::DynClone {}
    dyn_clone::clone_trait_object!(DateAny);
    impl DateAny for Date {}
    #[derive(Debug, derive_new :: new)]
    pub struct DatumFeatureCallout {
        pub draughting_callout: DraughtingCallout,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct DatumTargetCallout {
        pub draughting_callout: DraughtingCallout,
    }
    impl GeometricRepresentationItemAny for DefinedSymbol {}
    #[derive(Debug, derive_new :: new)]
    pub struct DefinedSymbol {
        pub definition: DefinedSymbolSelect,
        pub target: SymbolTarget,
        pub geometric_representation_item: Box<dyn GeometricRepresentationItemAny>,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct DiameterDimension {
        pub dimension_curve_directed_callout: DimensionCurveDirectedCallout,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct DimensionCalloutComponentRelationship {
        pub draughting_callout_relationship: DraughtingCalloutRelationship,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct DimensionCalloutRelationship {
        pub draughting_callout_relationship: DraughtingCalloutRelationship,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct DimensionCurve {
        pub annotation_curve_occurrence: AnnotationCurveOccurrence,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct DimensionCurveDirectedCallout {
        pub draughting_callout: DraughtingCallout,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct DimensionCurveTerminator {
        pub role: DimensionExtentUsage,
        pub terminator_symbol: TerminatorSymbol,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct DimensionPair {
        pub draughting_callout_relationship: DraughtingCalloutRelationship,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct DimensionalExponents {
        pub length_exponent: f64,
        pub mass_exponent: f64,
        pub time_exponent: f64,
        pub electric_current_exponent: f64,
        pub thermodynamic_temperature_exponent: f64,
        pub amount_of_substance_exponent: f64,
        pub luminous_intensity_exponent: f64,
    }
    impl GeometricRepresentationItemAny for Direction {}
    #[derive(Debug, derive_new :: new)]
    pub struct Direction {
        pub direction_ratios: Vec<f64>,
        pub geometric_representation_item: Box<dyn GeometricRepresentationItemAny>,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct Document {
        pub id: Identifier,
        pub name: Label,
        pub description: Text,
        pub kind: DocumentType,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct DocumentReference {
        pub assigned_document: Document,
        pub source: Label,
    }
    pub trait DocumentReferenceAny:
        ::std::any::Any + ::std::fmt::Debug + dyn_clone::DynClone
    {
    }
    dyn_clone::clone_trait_object!(DocumentReferenceAny);
    impl DocumentReferenceAny for DocumentReference {}
    #[derive(Debug, derive_new :: new)]
    pub struct DocumentType {
        pub product_data_type: Label,
    }
    impl AnnotationOccurrenceAny for DraughtingAnnotationOccurrence {}
    #[derive(Debug, derive_new :: new)]
    pub struct DraughtingAnnotationOccurrence {
        pub annotation_occurrence: Box<dyn AnnotationOccurrenceAny>,
    }
    impl ApprovalAssignmentAny for DraughtingApprovalAssignment {}
    #[derive(Debug, derive_new :: new)]
    pub struct DraughtingApprovalAssignment {
        pub approved_items: Vec<ApprovedItem>,
        pub approval_assignment: Box<dyn ApprovalAssignmentAny>,
    }
    impl GeometricRepresentationItemAny for DraughtingCallout {}
    #[derive(Debug, derive_new :: new)]
    pub struct DraughtingCallout {
        pub contents: Vec<DraughtingCalloutElement>,
        pub geometric_representation_item: Box<dyn GeometricRepresentationItemAny>,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct DraughtingCalloutRelationship {
        pub name: Label,
        pub description: Text,
        pub relating_draughting_callout: DraughtingCallout,
        pub related_draughting_callout: DraughtingCallout,
    }
    impl ContractAssignmentAny for DraughtingContractAssignment {}
    #[derive(Debug, derive_new :: new)]
    pub struct DraughtingContractAssignment {
        pub items: Vec<ContractedItem>,
        pub contract_assignment: Box<dyn ContractAssignmentAny>,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct DraughtingDrawingRevision {
        pub drawing_revision: DrawingRevision,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct DraughtingElements {
        pub draughting_callout: DraughtingCallout,
    }
    impl GroupAssignmentAny for DraughtingGroupAssignment {}
    #[derive(Debug, derive_new :: new)]
    pub struct DraughtingGroupAssignment {
        pub items: Vec<DraughtingGroupedItem>,
        pub group_assignment: Box<dyn GroupAssignmentAny>,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct DraughtingModel {
        pub representation: Representation,
    }
    impl OrganizationAssignmentAny for DraughtingOrganizationAssignment {}
    #[derive(Debug, derive_new :: new)]
    pub struct DraughtingOrganizationAssignment {
        pub assigned_items: Vec<DraughtingOrganizationItem>,
        pub organization_assignment: Box<dyn OrganizationAssignmentAny>,
    }
    impl PersonAndOrganizationAssignmentAny for DraughtingPersonAndOrganizationAssignment {}
    #[derive(Debug, derive_new :: new)]
    pub struct DraughtingPersonAndOrganizationAssignment {
        pub assigned_items: Vec<DraughtingOrganizationItem>,
        pub person_and_organization_assignment: Box<dyn PersonAndOrganizationAssignmentAny>,
    }
    impl PersonAssignmentAny for DraughtingPersonAssignment {}
    #[derive(Debug, derive_new :: new)]
    pub struct DraughtingPersonAssignment {
        pub assigned_items: Vec<DraughtingOrganizationItem>,
        pub person_assignment: Box<dyn PersonAssignmentAny>,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct DraughtingPreDefinedColour {
        pub pre_defined_colour: PreDefinedColour,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct DraughtingPreDefinedCurveFont {
        pub pre_defined_curve_font: PreDefinedCurveFont,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct DraughtingPreDefinedTextFont {
        pub pre_defined_text_font: PreDefinedTextFont,
    }
    impl PresentedItemAny for DraughtingPresentedItem {}
    #[derive(Debug, derive_new :: new)]
    pub struct DraughtingPresentedItem {
        pub items: Vec<DraughtingPresentedItemSelect>,
        pub presented_item: Box<dyn PresentedItemAny>,
    }
    impl SecurityClassificationAssignmentAny for DraughtingSecurityClassificationAssignment {}
    #[derive(Debug, derive_new :: new)]
    pub struct DraughtingSecurityClassificationAssignment {
        pub assigned_items: Vec<ClassifiedItem>,
        pub security_classification_assignment: Box<dyn SecurityClassificationAssignmentAny>,
    }
    impl DocumentReferenceAny for DraughtingSpecificationReference {}
    #[derive(Debug, derive_new :: new)]
    pub struct DraughtingSpecificationReference {
        pub specified_items: Vec<SpecifiedItem>,
        pub document_reference: Box<dyn DocumentReferenceAny>,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct DraughtingSubfigureRepresentation {
        pub symbol_representation: SymbolRepresentation,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct DraughtingSymbolRepresentation {
        pub symbol_representation: SymbolRepresentation,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct DraughtingTextLiteralWithDelineation {
        pub text_literal_with_delineation: TextLiteralWithDelineation,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct DraughtingTitle {
        pub items: Vec<DraughtingTitledItem>,
        pub language: Label,
        pub contents: Text,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct DrawingDefinition {
        pub drawing_number: Identifier,
        pub drawing_type: Option<Label>,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct DrawingRevision {
        pub revision_identifier: Identifier,
        pub drawing_identifier: DrawingDefinition,
        pub intended_scale: Option<Text>,
        pub presentation_set: PresentationSet,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct DrawingSheetLayout {
        pub draughting_symbol_representation: DraughtingSymbolRepresentation,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct DrawingSheetRevision {
        pub revision_identifier: Identifier,
        pub presentation_area: PresentationArea,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct DrawingSheetRevisionUsage {
        pub sheet_number: Identifier,
        pub area_in_set: AreaInSet,
    }
    impl ConicAny for Ellipse {}
    #[derive(Debug, derive_new :: new)]
    pub struct Ellipse {
        pub semi_axis_1: PositiveLengthMeasure,
        pub semi_axis_2: PositiveLengthMeasure,
        pub conic: Box<dyn ConicAny>,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct ExternalSource {
        pub source_id: SourceItem,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct ExternallyDefinedCurveFont {
        pub externally_defined_item: ExternallyDefinedItem,
    }
    impl GeometricRepresentationItemAny for ExternallyDefinedHatchStyle {}
    #[derive(Debug, derive_new :: new)]
    pub struct ExternallyDefinedHatchStyle {
        pub externally_defined_item: ExternallyDefinedItem,
        pub geometric_representation_item: Box<dyn GeometricRepresentationItemAny>,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct ExternallyDefinedItem {
        pub item_id: SourceItem,
        pub source: ExternalSource,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct ExternallyDefinedSymbol {
        pub externally_defined_item: ExternallyDefinedItem,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct ExternallyDefinedTextFont {
        pub externally_defined_item: ExternallyDefinedItem,
    }
    impl GeometricRepresentationItemAny for ExternallyDefinedTileStyle {}
    #[derive(Debug, derive_new :: new)]
    pub struct ExternallyDefinedTileStyle {
        pub externally_defined_item: ExternallyDefinedItem,
        pub geometric_representation_item: Box<dyn GeometricRepresentationItemAny>,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct FillAreaStyle {
        pub name: Label,
        pub fill_styles: Vec<FillStyleSelect>,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct FillAreaStyleColour {
        pub name: Label,
        pub fill_colour: Colour,
    }
    impl GeometricRepresentationItemAny for FillAreaStyleHatching {}
    #[derive(Debug, derive_new :: new)]
    pub struct FillAreaStyleHatching {
        pub hatch_line_appearance: CurveStyle,
        pub start_of_next_hatch_line: OneDirectionRepeatFactor,
        pub point_of_reference_hatch_line: CartesianPoint,
        pub pattern_start: CartesianPoint,
        pub hatch_line_angle: PlaneAngleMeasure,
        pub geometric_representation_item: Box<dyn GeometricRepresentationItemAny>,
    }
    impl GeometricRepresentationItemAny for FillAreaStyleTileSymbolWithStyle {}
    #[derive(Debug, derive_new :: new)]
    pub struct FillAreaStyleTileSymbolWithStyle {
        pub symbol: AnnotationSymbolOccurrence,
        pub geometric_representation_item: Box<dyn GeometricRepresentationItemAny>,
    }
    impl GeometricRepresentationItemAny for FillAreaStyleTiles {}
    #[derive(Debug, derive_new :: new)]
    pub struct FillAreaStyleTiles {
        pub tiling_pattern: TwoDirectionRepeatFactor,
        pub tiles: Vec<FillAreaStyleTileShapeSelect>,
        pub tiling_scale: PositiveRatioMeasure,
        pub geometric_representation_item: Box<dyn GeometricRepresentationItemAny>,
    }
    impl GeometricSetAny for GeometricCurveSet {}
    #[derive(Debug, derive_new :: new)]
    pub struct GeometricCurveSet {
        pub geometric_set: Box<dyn GeometricSetAny>,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct GeometricRepresentationContext {
        pub coordinate_space_dimension: DimensionCount,
        pub representation_context: RepresentationContext,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct GeometricRepresentationItem {
        pub representation_item: RepresentationItem,
    }
    pub trait GeometricRepresentationItemAny:
        ::std::any::Any + ::std::fmt::Debug + dyn_clone::DynClone
    {
    }
    dyn_clone::clone_trait_object!(GeometricRepresentationItemAny);
    impl GeometricRepresentationItemAny for GeometricRepresentationItem {}
    impl GeometricRepresentationItemAny for GeometricSet {}
    #[derive(Debug, derive_new :: new)]
    pub struct GeometricSet {
        pub elements: Vec<GeometricSetSelect>,
        pub geometric_representation_item: Box<dyn GeometricRepresentationItemAny>,
    }
    pub trait GeometricSetAny: ::std::any::Any + ::std::fmt::Debug + dyn_clone::DynClone {}
    dyn_clone::clone_trait_object!(GeometricSetAny);
    impl GeometricSetAny for GeometricSet {}
    #[derive(Debug, derive_new :: new)]
    pub struct GeometricalToleranceCallout {
        pub draughting_callout: DraughtingCallout,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct GeometricallyBounded2DWireframeRepresentation {
        pub shape_representation: ShapeRepresentation,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct GlobalUnitAssignedContext {
        pub units: Vec<Unit>,
        pub representation_context: RepresentationContext,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct Group {
        pub name: Label,
        pub description: Text,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct GroupAssignment {
        pub assigned_group: Group,
    }
    pub trait GroupAssignmentAny:
        ::std::any::Any + ::std::fmt::Debug + dyn_clone::DynClone
    {
    }
    dyn_clone::clone_trait_object!(GroupAssignmentAny);
    impl GroupAssignmentAny for GroupAssignment {}
    #[derive(Debug, derive_new :: new)]
    pub struct GroupRelationship {
        pub name: Label,
        pub description: Text,
        pub relating_group: Group,
        pub related_group: Group,
    }
    impl ConicAny for Hyperbola {}
    #[derive(Debug, derive_new :: new)]
    pub struct Hyperbola {
        pub semi_axis: PositiveLengthMeasure,
        pub semi_imag_axis: PositiveLengthMeasure,
        pub conic: Box<dyn ConicAny>,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct Invisibility {
        pub invisible_items: Vec<InvisibleItem>,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct LeaderCurve {
        pub annotation_curve_occurrence: AnnotationCurveOccurrence,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct LeaderDirectedCallout {
        pub draughting_callout: DraughtingCallout,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct LeaderDirectedDimension {
        pub leader_directed_callout: LeaderDirectedCallout,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct LeaderTerminator {
        pub terminator_symbol: TerminatorSymbol,
    }
    impl MeasureWithUnitAny for LengthMeasureWithUnit {}
    #[derive(Debug, derive_new :: new)]
    pub struct LengthMeasureWithUnit {
        pub measure_with_unit: Box<dyn MeasureWithUnitAny>,
    }
    impl NamedUnitAny for LengthUnit {}
    #[derive(Debug, derive_new :: new)]
    pub struct LengthUnit {
        pub named_unit: Box<dyn NamedUnitAny>,
    }
    impl CurveAny for Line {}
    #[derive(Debug, derive_new :: new)]
    pub struct Line {
        pub pnt: CartesianPoint,
        pub dir: Vector,
        pub curve: Box<dyn CurveAny>,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct LinearDimension {
        pub dimension_curve_directed_callout: DimensionCurveDirectedCallout,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct MappedItem {
        pub mapping_source: RepresentationMap,
        pub mapping_target: RepresentationItem,
        pub representation_item: RepresentationItem,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct MeasureWithUnit {
        pub value_component: MeasureValue,
        pub unit_component: Unit,
    }
    pub trait MeasureWithUnitAny:
        ::std::any::Any + ::std::fmt::Debug + dyn_clone::DynClone
    {
    }
    dyn_clone::clone_trait_object!(MeasureWithUnitAny);
    impl MeasureWithUnitAny for MeasureWithUnit {}
    #[derive(Debug, derive_new :: new)]
    pub struct NamedUnit {
        pub dimensions: DimensionalExponents,
    }
    pub trait NamedUnitAny: ::std::any::Any + ::std::fmt::Debug + dyn_clone::DynClone {}
    dyn_clone::clone_trait_object!(NamedUnitAny);
    impl NamedUnitAny for NamedUnit {}
    impl CurveAny for OffsetCurve2D {}
    #[derive(Debug, derive_new :: new)]
    pub struct OffsetCurve2D {
        pub basis_curve: Box<dyn CurveAny>,
        pub distance: LengthMeasure,
        pub self_intersect: Logical,
        pub curve: Box<dyn CurveAny>,
    }
    impl GeometricRepresentationItemAny for OneDirectionRepeatFactor {}
    #[derive(Debug, derive_new :: new)]
    pub struct OneDirectionRepeatFactor {
        pub repeat_factor: Vector,
        pub geometric_representation_item: Box<dyn GeometricRepresentationItemAny>,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct OrdinateDimension {
        pub projection_directed_callout: ProjectionDirectedCallout,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct Organization {
        pub id: Option<Identifier>,
        pub name: Label,
        pub description: Text,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct OrganizationAssignment {
        pub assigned_organization: Organization,
        pub role: OrganizationRole,
    }
    pub trait OrganizationAssignmentAny:
        ::std::any::Any + ::std::fmt::Debug + dyn_clone::DynClone
    {
    }
    dyn_clone::clone_trait_object!(OrganizationAssignmentAny);
    impl OrganizationAssignmentAny for OrganizationAssignment {}
    #[derive(Debug, derive_new :: new)]
    pub struct OrganizationRole {
        pub name: Label,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct OrganizationalAddress {
        pub organizations: Vec<Organization>,
        pub description: Text,
        pub address: Address,
    }
    impl ConicAny for Parabola {}
    #[derive(Debug, derive_new :: new)]
    pub struct Parabola {
        pub focal_dist: LengthMeasure,
        pub conic: Box<dyn ConicAny>,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct Person {
        pub id: Identifier,
        pub last_name: Option<Label>,
        pub first_name: Option<Label>,
        pub middle_names: Option<Vec<Label>>,
        pub prefix_titles: Option<Vec<Label>>,
        pub suffix_titles: Option<Vec<Label>>,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct PersonAndOrganization {
        pub the_person: Person,
        pub the_organization: Organization,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct PersonAndOrganizationAssignment {
        pub assigned_person_and_organization: PersonAndOrganization,
        pub role: PersonAndOrganizationRole,
    }
    pub trait PersonAndOrganizationAssignmentAny:
        ::std::any::Any + ::std::fmt::Debug + dyn_clone::DynClone
    {
    }
    dyn_clone::clone_trait_object!(PersonAndOrganizationAssignmentAny);
    impl PersonAndOrganizationAssignmentAny for PersonAndOrganizationAssignment {}
    #[derive(Debug, derive_new :: new)]
    pub struct PersonAndOrganizationRole {
        pub name: Label,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct PersonAssignment {
        pub assigned_person: Person,
        pub role: PersonRole,
    }
    pub trait PersonAssignmentAny:
        ::std::any::Any + ::std::fmt::Debug + dyn_clone::DynClone
    {
    }
    dyn_clone::clone_trait_object!(PersonAssignmentAny);
    impl PersonAssignmentAny for PersonAssignment {}
    #[derive(Debug, derive_new :: new)]
    pub struct PersonRole {
        pub name: Label,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct PersonalAddress {
        pub people: Vec<Person>,
        pub description: Text,
        pub address: Address,
    }
    impl GeometricRepresentationItemAny for Placement {}
    #[derive(Debug, derive_new :: new)]
    pub struct Placement {
        pub location: CartesianPoint,
        pub geometric_representation_item: Box<dyn GeometricRepresentationItemAny>,
    }
    pub trait PlacementAny: ::std::any::Any + ::std::fmt::Debug + dyn_clone::DynClone {}
    dyn_clone::clone_trait_object!(PlacementAny);
    impl PlacementAny for Placement {}
    #[derive(Debug, derive_new :: new)]
    pub struct PlanarBox {
        pub placement: Axis2Placement,
        pub planar_extent: PlanarExtent,
    }
    impl GeometricRepresentationItemAny for PlanarExtent {}
    #[derive(Debug, derive_new :: new)]
    pub struct PlanarExtent {
        pub size_in_x: LengthMeasure,
        pub size_in_y: LengthMeasure,
        pub geometric_representation_item: Box<dyn GeometricRepresentationItemAny>,
    }
    impl MeasureWithUnitAny for PlaneAngleMeasureWithUnit {}
    #[derive(Debug, derive_new :: new)]
    pub struct PlaneAngleMeasureWithUnit {
        pub measure_with_unit: Box<dyn MeasureWithUnitAny>,
    }
    impl NamedUnitAny for PlaneAngleUnit {}
    #[derive(Debug, derive_new :: new)]
    pub struct PlaneAngleUnit {
        pub named_unit: Box<dyn NamedUnitAny>,
    }
    impl GeometricRepresentationItemAny for Point {}
    #[derive(Debug, derive_new :: new)]
    pub struct Point {
        pub geometric_representation_item: Box<dyn GeometricRepresentationItemAny>,
    }
    pub trait PointAny: ::std::any::Any + ::std::fmt::Debug + dyn_clone::DynClone {}
    dyn_clone::clone_trait_object!(PointAny);
    impl PointAny for Point {}
    impl PointAny for PointOnCurve {}
    #[derive(Debug, derive_new :: new)]
    pub struct PointOnCurve {
        pub basis_curve: Box<dyn CurveAny>,
        pub point_parameter: ParameterValue,
        pub point: Box<dyn PointAny>,
    }
    impl BoundedCurveAny for Polyline {}
    #[derive(Debug, derive_new :: new)]
    pub struct Polyline {
        pub points: Vec<CartesianPoint>,
        pub bounded_curve: Box<dyn BoundedCurveAny>,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct PreDefinedColour {
        pub pre_defined_item: PreDefinedItem,
        pub colour: Colour,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct PreDefinedCurveFont {
        pub pre_defined_item: PreDefinedItem,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct PreDefinedDimensionSymbol {
        pub pre_defined_symbol: PreDefinedSymbol,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct PreDefinedGeometricalToleranceSymbol {
        pub pre_defined_symbol: PreDefinedSymbol,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct PreDefinedItem {
        pub name: Label,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct PreDefinedPointMarkerSymbol {
        pub pre_defined_symbol: PreDefinedSymbol,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct PreDefinedSymbol {
        pub pre_defined_item: PreDefinedItem,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct PreDefinedTerminatorSymbol {
        pub pre_defined_symbol: PreDefinedSymbol,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct PreDefinedTextFont {
        pub pre_defined_item: PreDefinedItem,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct PresentationArea {
        pub presentation_representation: PresentationRepresentation,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct PresentationLayerAssignment {
        pub name: Label,
        pub description: Text,
        pub assigned_items: Vec<LayeredItem>,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct PresentationLayerUsage {
        pub assignment: PresentationLayerAssignment,
        pub presentation: PresentationRepresentation,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct PresentationRepresentation {
        pub representation: Representation,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct PresentationSet {}
    #[derive(Debug, derive_new :: new)]
    pub struct PresentationSize {
        pub unit: PresentationSizeAssignmentSelect,
        pub size: PlanarBox,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct PresentationStyleAssignment {
        pub styles: Vec<PresentationStyleSelect>,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct PresentationStyleByContext {
        pub style_context: StyleContextSelect,
        pub presentation_style_assignment: PresentationStyleAssignment,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct PresentationView {
        pub presentation_representation: PresentationRepresentation,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct PresentedItem {}
    pub trait PresentedItemAny: ::std::any::Any + ::std::fmt::Debug + dyn_clone::DynClone {}
    dyn_clone::clone_trait_object!(PresentedItemAny);
    impl PresentedItemAny for PresentedItem {}
    #[derive(Debug, derive_new :: new)]
    pub struct PresentedItemRepresentation {
        pub presentation: PresentationRepresentationSelect,
        pub item: Box<dyn PresentedItemAny>,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct Product {
        pub id: Identifier,
        pub name: Label,
        pub description: Text,
        pub frame_of_reference: Vec<ProductContext>,
    }
    impl ApplicationContextElementAny for ProductContext {}
    #[derive(Debug, derive_new :: new)]
    pub struct ProductContext {
        pub discipline_type: Label,
        pub application_context_element: Box<dyn ApplicationContextElementAny>,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct ProductDefinition {
        pub id: Identifier,
        pub description: Text,
        pub formation: ProductDefinitionFormation,
        pub frame_of_reference: ProductDefinitionContext,
    }
    impl ApplicationContextElementAny for ProductDefinitionContext {}
    #[derive(Debug, derive_new :: new)]
    pub struct ProductDefinitionContext {
        pub life_cycle_stage: Label,
        pub application_context_element: Box<dyn ApplicationContextElementAny>,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct ProductDefinitionFormation {
        pub id: Identifier,
        pub description: Text,
        pub of_product: Product,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct ProductDefinitionShape {
        pub property_definition: PropertyDefinition,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct ProjectionCurve {
        pub annotation_curve_occurrence: AnnotationCurveOccurrence,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct ProjectionDirectedCallout {
        pub draughting_callout: DraughtingCallout,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct PropertyDefinition {
        pub name: Label,
        pub description: Text,
        pub definition: CharacterizedDefinition,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct PropertyDefinitionRepresentation {
        pub definition: PropertyDefinition,
        pub used_representation: Representation,
    }
    impl BSplineCurveAny for QuasiUniformCurve {}
    #[derive(Debug, derive_new :: new)]
    pub struct QuasiUniformCurve {
        pub b_spline_curve: Box<dyn BSplineCurveAny>,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct RadiusDimension {
        pub dimension_curve_directed_callout: DimensionCurveDirectedCallout,
    }
    impl BSplineCurveAny for RationalBSplineCurve {}
    #[derive(Debug, derive_new :: new)]
    pub struct RationalBSplineCurve {
        pub weights_data: Vec<f64>,
        pub b_spline_curve: Box<dyn BSplineCurveAny>,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct Representation {
        pub name: Label,
        pub items: Vec<RepresentationItem>,
        pub context_of_items: RepresentationContext,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct RepresentationContext {
        pub context_identifier: Identifier,
        pub context_type: Text,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct RepresentationItem {
        pub name: Label,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct RepresentationMap {
        pub mapping_origin: RepresentationItem,
        pub mapped_representation: Representation,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct SecurityClassification {
        pub name: Label,
        pub purpose: Text,
        pub security_level: SecurityClassificationLevel,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct SecurityClassificationAssignment {
        pub assigned_security_classification: SecurityClassification,
    }
    pub trait SecurityClassificationAssignmentAny:
        ::std::any::Any + ::std::fmt::Debug + dyn_clone::DynClone
    {
    }
    dyn_clone::clone_trait_object!(SecurityClassificationAssignmentAny);
    impl SecurityClassificationAssignmentAny for SecurityClassificationAssignment {}
    #[derive(Debug, derive_new :: new)]
    pub struct SecurityClassificationLevel {
        pub name: Label,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct ShapeDefinitionRepresentation {
        pub property_definition_representation: PropertyDefinitionRepresentation,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct ShapeRepresentation {
        pub representation: Representation,
    }
    impl NamedUnitAny for SiUnit {}
    #[derive(Debug, derive_new :: new)]
    pub struct SiUnit {
        pub prefix: Option<SiPrefix>,
        pub name: SiUnitName,
        pub named_unit: Box<dyn NamedUnitAny>,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct StructuredDimensionCallout {
        pub draughting_callout: DraughtingCallout,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct StyledItem {
        pub styles: Vec<PresentationStyleAssignment>,
        pub item: RepresentationItem,
        pub representation_item: RepresentationItem,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct SymbolColour {
        pub colour_of_symbol: Colour,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct SymbolRepresentation {
        pub representation: Representation,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct SymbolRepresentationMap {
        pub representation_map: RepresentationMap,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct SymbolStyle {
        pub name: Label,
        pub style_of_symbol: SymbolStyleSelect,
    }
    impl GeometricRepresentationItemAny for SymbolTarget {}
    #[derive(Debug, derive_new :: new)]
    pub struct SymbolTarget {
        pub placement: Axis2Placement,
        pub x_scale: PositiveRatioMeasure,
        pub y_scale: PositiveRatioMeasure,
        pub geometric_representation_item: Box<dyn GeometricRepresentationItemAny>,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct TerminatorSymbol {
        pub annotated_curve: AnnotationCurveOccurrence,
        pub annotation_symbol_occurrence: AnnotationSymbolOccurrence,
    }
    impl GeometricRepresentationItemAny for TextLiteral {}
    #[derive(Debug, derive_new :: new)]
    pub struct TextLiteral {
        pub literal: PresentableText,
        pub placement: Axis2Placement,
        pub alignment: TextAlignment,
        pub path: TextPath,
        pub font: FontSelect,
        pub geometric_representation_item: Box<dyn GeometricRepresentationItemAny>,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct TextLiteralWithAssociatedCurves {
        pub associated_curves: Vec<Box<dyn CurveAny>>,
        pub text_literal: TextLiteral,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct TextLiteralWithBlankingBox {
        pub blanking: PlanarBox,
        pub text_literal: TextLiteral,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct TextLiteralWithDelineation {
        pub delineation: TextDelineation,
        pub text_literal: TextLiteral,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct TextLiteralWithExtent {
        pub extent: PlanarExtent,
        pub text_literal: TextLiteral,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct TextStyle {
        pub name: Label,
        pub character_appearance: CharacterStyleSelect,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct TextStyleForDefinedFont {
        pub text_colour: Colour,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct TextStyleWithBoxCharacteristics {
        pub characteristics: Vec<BoxCharacteristicSelect>,
        pub text_style: TextStyle,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct TextStyleWithMirror {
        pub mirror_placement: Axis2Placement,
        pub text_style: TextStyle,
    }
    impl BoundedCurveAny for TrimmedCurve {}
    #[derive(Debug, derive_new :: new)]
    pub struct TrimmedCurve {
        pub basis_curve: Box<dyn CurveAny>,
        pub trim_1: Vec<TrimmingSelect>,
        pub trim_2: Vec<TrimmingSelect>,
        pub sense_agreement: bool,
        pub master_representation: TrimmingPreference,
        pub bounded_curve: Box<dyn BoundedCurveAny>,
    }
    #[derive(Debug, derive_new :: new)]
    pub struct TwoDirectionRepeatFactor {
        pub second_repeat_factor: Vector,
        pub one_direction_repeat_factor: OneDirectionRepeatFactor,
    }
    impl BSplineCurveAny for UniformCurve {}
    #[derive(Debug, derive_new :: new)]
    pub struct UniformCurve {
        pub b_spline_curve: Box<dyn BSplineCurveAny>,
    }
    impl GeometricRepresentationItemAny for Vector {}
    #[derive(Debug, derive_new :: new)]
    pub struct Vector {
        pub orientation: Direction,
        pub magnitude: LengthMeasure,
        pub geometric_representation_item: Box<dyn GeometricRepresentationItemAny>,
    }
}

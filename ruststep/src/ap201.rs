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
    #[derive(Debug, Clone, PartialEq)]
    pub struct BoxHeight(pub PositiveRatioMeasure);
    #[derive(Debug, Clone, PartialEq)]
    pub struct BoxRotateAngle(pub PlaneAngleMeasure);
    #[derive(Debug, Clone, PartialEq)]
    pub struct BoxSlantAngle(pub PlaneAngleMeasure);
    #[derive(Debug, Clone, PartialEq)]
    pub struct BoxWidth(pub PositiveRatioMeasure);
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
    #[derive(Debug, Clone, PartialEq)]
    pub struct DayInMonthNumber(pub i64);
    #[derive(Debug, Clone, PartialEq)]
    pub enum DefinedSymbolSelect {
        PreDefinedSymbol(Box<PreDefinedSymbol>),
        ExternallyDefinedSymbol(Box<ExternallyDefinedSymbol>),
    }
    #[derive(Debug, Clone, PartialEq)]
    pub struct DimensionCount(pub i64);
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
    #[derive(Debug, Clone, PartialEq)]
    pub struct Identifier(pub String);
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
    #[derive(Debug, Clone, PartialEq)]
    pub struct Label(pub String);
    #[derive(Debug, Clone, PartialEq)]
    pub enum LayeredItem {
        PresentationRepresentation(Box<PresentationRepresentation>),
        RepresentationItem(Box<RepresentationItem>),
    }
    #[derive(Debug, Clone, PartialEq)]
    pub struct LengthMeasure(pub f64);
    #[derive(Debug, Clone, PartialEq)]
    pub enum MeasureValue {
        LengthMeasure(Box<LengthMeasure>),
        PlaneAngleMeasure(Box<PlaneAngleMeasure>),
        RatioMeasure(Box<RatioMeasure>),
        ParameterValue(Box<ParameterValue>),
        PositiveLengthMeasure(Box<PositiveLengthMeasure>),
        PositiveRatioMeasure(Box<PositiveRatioMeasure>),
    }
    #[derive(Debug, Clone, PartialEq)]
    pub struct MonthInYearNumber(pub i64);
    #[derive(Debug, Clone, PartialEq)]
    pub enum NullStyle {
        Null,
    }
    #[derive(Debug, Clone, PartialEq)]
    pub struct ParameterValue(pub f64);
    #[derive(Debug, Clone, PartialEq)]
    pub enum PersonOrganizationSelect {
        Person(Box<Person>),
        Organization(Box<Organization>),
        PersonAndOrganization(Box<PersonAndOrganization>),
    }
    #[derive(Debug, Clone, PartialEq)]
    pub struct PlaneAngleMeasure(pub f64);
    #[derive(Debug, Clone, PartialEq)]
    pub struct PositiveLengthMeasure(pub LengthMeasure);
    #[derive(Debug, Clone, PartialEq)]
    pub struct PositiveRatioMeasure(pub RatioMeasure);
    #[derive(Debug, Clone, PartialEq)]
    pub struct PresentableText(pub String);
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
    #[derive(Debug, Clone, PartialEq)]
    pub struct RatioMeasure(pub f64);
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
    #[derive(Debug, Clone, PartialEq)]
    pub struct Text(pub String);
    #[derive(Debug, Clone, PartialEq)]
    pub struct TextAlignment(pub Label);
    #[derive(Debug, Clone, PartialEq)]
    pub struct TextDelineation(pub Label);
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
    #[derive(Debug, Clone, PartialEq)]
    pub struct YearNumber(pub i64);
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
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct AnnotationCurveOccurrence {
        pub annotation_occurrence: AnnotationOccurrence,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct AnnotationFillArea {
        pub geometric_representation_item: GeometricRepresentationItem,
        pub boundaries: Curve,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct AnnotationFillAreaOccurrence {
        pub annotation_occurrence: AnnotationOccurrence,
        pub fill_style_target: Point,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct AnnotationOccurrence {
        pub styled_item: StyledItem,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct AnnotationSubfigureOccurrence {
        pub annotation_symbol_occurrence: AnnotationSymbolOccurrence,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct AnnotationSymbol {
        pub mapped_item: MappedItem,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct AnnotationSymbolOccurrence {
        pub annotation_occurrence: AnnotationOccurrence,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct AnnotationText {
        pub mapped_item: MappedItem,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct AnnotationTextOccurrence {
        pub annotation_occurrence: AnnotationOccurrence,
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
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct BSplineCurve {
        pub bounded_curve: BoundedCurve,
        pub degree: i64,
        pub control_points_list: CartesianPoint,
        pub curve_form: BSplineCurveForm,
        pub closed_curve: Logical,
        pub self_intersect: Logical,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct BSplineCurveWithKnots {
        pub b_spline_curve: BSplineCurve,
        pub knot_multiplicities: i64,
        pub knots: ParameterValue,
        pub knot_spec: KnotType,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct BezierCurve {
        pub b_spline_curve: BSplineCurve,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct BoundedCurve {
        pub curve: Curve,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct CalendarDate {
        pub date: Date,
        pub day_component: DayInMonthNumber,
        pub month_component: MonthInYearNumber,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct CameraImage {
        pub mapped_item: MappedItem,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct CameraImage2DWithScale {
        pub camera_image: CameraImage,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct CameraModel {
        pub geometric_representation_item: GeometricRepresentationItem,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct CameraModelD2 {
        pub camera_model: CameraModel,
        pub view_window: PlanarBox,
        pub view_window_clipping: bool,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct CameraUsage {
        pub representation_map: RepresentationMap,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct CartesianPoint {
        pub point: Point,
        pub coordinates: LengthMeasure,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct Circle {
        pub conic: Conic,
        pub radius: PositiveLengthMeasure,
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
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ColourSpecification {
        pub colour: Colour,
        pub name: Colour,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct CompositeCurve {
        pub bounded_curve: BoundedCurve,
        pub segments: CompositeCurveSegment,
        pub self_intersect: Logical,
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
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct CompositeTextWithAssociatedCurves {
        pub composite_text: CompositeText,
        pub associated_curves: Curve,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct CompositeTextWithBlankingBox {
        pub composite_text: CompositeText,
        pub blanking: PlanarBox,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct CompositeTextWithExtent {
        pub composite_text: CompositeText,
        pub extent: PlanarExtent,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct Conic {
        pub curve: Curve,
        pub position: Axis2Placement,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ContextDependentInvisibility {
        pub invisibility: Invisibility,
        pub presentation_context: InvisibilityContext,
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
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct Curve {
        pub geometric_representation_item: GeometricRepresentationItem,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct CurveDimension {
        pub dimension_curve_directed_callout: DimensionCurveDirectedCallout,
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
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DatumTargetCallout {
        pub draughting_callout: DraughtingCallout,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DefinedSymbol {
        pub geometric_representation_item: GeometricRepresentationItem,
        pub definition: DefinedSymbolSelect,
        pub target: SymbolTarget,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DiameterDimension {
        pub dimension_curve_directed_callout: DimensionCurveDirectedCallout,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DimensionCalloutComponentRelationship {
        pub draughting_callout_relationship: DraughtingCalloutRelationship,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DimensionCalloutRelationship {
        pub draughting_callout_relationship: DraughtingCalloutRelationship,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DimensionCurve {
        pub annotation_curve_occurrence: AnnotationCurveOccurrence,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DimensionCurveDirectedCallout {
        pub draughting_callout: DraughtingCallout,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DimensionCurveTerminator {
        pub terminator_symbol: TerminatorSymbol,
        pub role: DimensionExtentUsage,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DimensionPair {
        pub draughting_callout_relationship: DraughtingCalloutRelationship,
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
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DraughtingApprovalAssignment {
        pub approval_assignment: ApprovalAssignment,
        pub approved_items: ApprovedItem,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DraughtingCallout {
        pub geometric_representation_item: GeometricRepresentationItem,
        pub contents: DraughtingCalloutElement,
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
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DraughtingDrawingRevision {
        pub drawing_revision: DrawingRevision,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DraughtingElements {
        pub draughting_callout: DraughtingCallout,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DraughtingGroupAssignment {
        pub group_assignment: GroupAssignment,
        pub items: DraughtingGroupedItem,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DraughtingModel {
        pub representation: Representation,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DraughtingOrganizationAssignment {
        pub organization_assignment: OrganizationAssignment,
        pub assigned_items: DraughtingOrganizationItem,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DraughtingPersonAndOrganizationAssignment {
        pub person_and_organization_assignment: PersonAndOrganizationAssignment,
        pub assigned_items: DraughtingOrganizationItem,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DraughtingPersonAssignment {
        pub person_assignment: PersonAssignment,
        pub assigned_items: DraughtingOrganizationItem,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DraughtingPreDefinedColour {
        pub pre_defined_colour: PreDefinedColour,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DraughtingPreDefinedCurveFont {
        pub pre_defined_curve_font: PreDefinedCurveFont,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DraughtingPreDefinedTextFont {
        pub pre_defined_text_font: PreDefinedTextFont,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DraughtingPresentedItem {
        pub presented_item: PresentedItem,
        pub items: DraughtingPresentedItemSelect,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DraughtingSecurityClassificationAssignment {
        pub security_classification_assignment: SecurityClassificationAssignment,
        pub assigned_items: ClassifiedItem,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DraughtingSpecificationReference {
        pub document_reference: DocumentReference,
        pub specified_items: SpecifiedItem,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DraughtingSubfigureRepresentation {
        pub symbol_representation: SymbolRepresentation,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DraughtingSymbolRepresentation {
        pub symbol_representation: SymbolRepresentation,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DraughtingTextLiteralWithDelineation {
        pub text_literal_with_delineation: TextLiteralWithDelineation,
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
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DrawingSheetLayout {
        pub draughting_symbol_representation: DraughtingSymbolRepresentation,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DrawingSheetRevision {
        pub presentation_area: PresentationArea,
        pub revision_identifier: Identifier,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DrawingSheetRevisionUsage {
        pub area_in_set: AreaInSet,
        pub sheet_number: Identifier,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct Ellipse {
        pub conic: Conic,
        pub semi_axis_1: PositiveLengthMeasure,
        pub semi_axis_2: PositiveLengthMeasure,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ExternalSource {
        pub source_id: SourceItem,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ExternallyDefinedCurveFont {
        pub externally_defined_item: ExternallyDefinedItem,
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
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ExternallyDefinedTextFont {
        pub externally_defined_item: ExternallyDefinedItem,
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
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct FillAreaStyleTileSymbolWithStyle {
        pub geometric_representation_item: GeometricRepresentationItem,
        pub symbol: AnnotationSymbolOccurrence,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct FillAreaStyleTiles {
        pub geometric_representation_item: GeometricRepresentationItem,
        pub tiling_pattern: TwoDirectionRepeatFactor,
        pub tiles: FillAreaStyleTileShapeSelect,
        pub tiling_scale: PositiveRatioMeasure,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct GeometricCurveSet {
        pub geometric_set: GeometricSet,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct GeometricRepresentationContext {
        pub representation_context: RepresentationContext,
        pub coordinate_space_dimension: DimensionCount,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct GeometricRepresentationItem {
        pub representation_item: RepresentationItem,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct GeometricSet {
        pub geometric_representation_item: GeometricRepresentationItem,
        pub elements: GeometricSetSelect,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct GeometricalToleranceCallout {
        pub draughting_callout: DraughtingCallout,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct GeometricallyBounded2DWireframeRepresentation {
        pub shape_representation: ShapeRepresentation,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct GlobalUnitAssignedContext {
        pub representation_context: RepresentationContext,
        pub units: Unit,
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
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct Invisibility {
        pub invisible_items: InvisibleItem,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct LeaderCurve {
        pub annotation_curve_occurrence: AnnotationCurveOccurrence,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct LeaderDirectedCallout {
        pub draughting_callout: DraughtingCallout,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct LeaderDirectedDimension {
        pub leader_directed_callout: LeaderDirectedCallout,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct LeaderTerminator {
        pub terminator_symbol: TerminatorSymbol,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct LengthMeasureWithUnit {
        pub measure_with_unit: MeasureWithUnit,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct LengthUnit {
        pub named_unit: NamedUnit,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct Line {
        pub curve: Curve,
        pub pnt: CartesianPoint,
        pub dir: Vector,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct LinearDimension {
        pub dimension_curve_directed_callout: DimensionCurveDirectedCallout,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct MappedItem {
        pub representation_item: RepresentationItem,
        pub mapping_source: RepresentationMap,
        pub mapping_target: RepresentationItem,
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
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct OneDirectionRepeatFactor {
        pub geometric_representation_item: GeometricRepresentationItem,
        pub repeat_factor: Vector,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct OrdinateDimension {
        pub projection_directed_callout: ProjectionDirectedCallout,
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
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct Parabola {
        pub conic: Conic,
        pub focal_dist: LengthMeasure,
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
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct Placement {
        pub geometric_representation_item: GeometricRepresentationItem,
        pub location: CartesianPoint,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PlanarBox {
        pub planar_extent: PlanarExtent,
        pub placement: Axis2Placement,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PlanarExtent {
        pub geometric_representation_item: GeometricRepresentationItem,
        pub size_in_x: LengthMeasure,
        pub size_in_y: LengthMeasure,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PlaneAngleMeasureWithUnit {
        pub measure_with_unit: MeasureWithUnit,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PlaneAngleUnit {
        pub named_unit: NamedUnit,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct Point {
        pub geometric_representation_item: GeometricRepresentationItem,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PointOnCurve {
        pub point: Point,
        pub basis_curve: Curve,
        pub point_parameter: ParameterValue,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct Polyline {
        pub bounded_curve: BoundedCurve,
        pub points: CartesianPoint,
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
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PreDefinedDimensionSymbol {
        pub pre_defined_symbol: PreDefinedSymbol,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PreDefinedGeometricalToleranceSymbol {
        pub pre_defined_symbol: PreDefinedSymbol,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PreDefinedItem {
        pub name: Label,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PreDefinedPointMarkerSymbol {
        pub pre_defined_symbol: PreDefinedSymbol,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PreDefinedSymbol {
        pub pre_defined_item: PreDefinedItem,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PreDefinedTerminatorSymbol {
        pub pre_defined_symbol: PreDefinedSymbol,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PreDefinedTextFont {
        pub pre_defined_item: PreDefinedItem,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PresentationArea {
        pub presentation_representation: PresentationRepresentation,
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
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PresentationView {
        pub presentation_representation: PresentationRepresentation,
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
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ProjectionCurve {
        pub annotation_curve_occurrence: AnnotationCurveOccurrence,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ProjectionDirectedCallout {
        pub draughting_callout: DraughtingCallout,
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
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct RadiusDimension {
        pub dimension_curve_directed_callout: DimensionCurveDirectedCallout,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct RationalBSplineCurve {
        pub b_spline_curve: BSplineCurve,
        pub weights_data: f64,
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
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ShapeRepresentation {
        pub representation: Representation,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct SiUnit {
        pub named_unit: NamedUnit,
        pub prefix: Option<SiPrefix>,
        pub name: SiUnitName,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct StructuredDimensionCallout {
        pub draughting_callout: DraughtingCallout,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct StyledItem {
        pub representation_item: RepresentationItem,
        pub styles: PresentationStyleAssignment,
        pub item: RepresentationItem,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct SymbolColour {
        pub colour_of_symbol: Colour,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct SymbolRepresentation {
        pub representation: Representation,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct SymbolRepresentationMap {
        pub representation_map: RepresentationMap,
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
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct TerminatorSymbol {
        pub annotation_symbol_occurrence: AnnotationSymbolOccurrence,
        pub annotated_curve: AnnotationCurveOccurrence,
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
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct TextLiteralWithAssociatedCurves {
        pub text_literal: TextLiteral,
        pub associated_curves: Curve,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct TextLiteralWithBlankingBox {
        pub text_literal: TextLiteral,
        pub blanking: PlanarBox,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct TextLiteralWithDelineation {
        pub text_literal: TextLiteral,
        pub delineation: TextDelineation,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct TextLiteralWithExtent {
        pub text_literal: TextLiteral,
        pub extent: PlanarExtent,
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
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct TextStyleWithMirror {
        pub text_style: TextStyle,
        pub mirror_placement: Axis2Placement,
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
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct TwoDirectionRepeatFactor {
        pub one_direction_repeat_factor: OneDirectionRepeatFactor,
        pub second_repeat_factor: Vector,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct UniformCurve {
        pub b_spline_curve: BSplineCurve,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct Vector {
        pub geometric_representation_item: GeometricRepresentationItem,
        pub orientation: Direction,
        pub magnitude: LengthMeasure,
    }
}

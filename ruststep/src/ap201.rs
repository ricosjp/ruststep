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
        internal_location: Option<Label>,
        street_number: Option<Label>,
        street: Option<Label>,
        postal_box: Option<Label>,
        town: Option<Label>,
        region: Option<Label>,
        postal_code: Option<Label>,
        country: Option<Label>,
        facsimile_number: Option<Label>,
        telephone_number: Option<Label>,
        electronic_mail_address: Option<Label>,
        telex_number: Option<Label>,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct AngularDimension {
        dimension_curve_directed_callout: DimensionCurveDirectedCallout,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct AnnotationCurveOccurrence {
        annotation_occurrence: AnnotationOccurrence,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct AnnotationFillArea {
        geometric_representation_item: GeometricRepresentationItem,
        boundaries: Curve,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct AnnotationFillAreaOccurrence {
        annotation_occurrence: AnnotationOccurrence,
        fill_style_target: Point,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct AnnotationOccurrence {
        styled_item: StyledItem,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct AnnotationSubfigureOccurrence {
        annotation_symbol_occurrence: AnnotationSymbolOccurrence,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct AnnotationSymbol {
        mapped_item: MappedItem,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct AnnotationSymbolOccurrence {
        annotation_occurrence: AnnotationOccurrence,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct AnnotationText {
        mapped_item: MappedItem,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct AnnotationTextOccurrence {
        annotation_occurrence: AnnotationOccurrence,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ApplicationContext {
        application: Text,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ApplicationContextElement {
        name: Label,
        frame_of_reference: ApplicationContext,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ApplicationProtocolDefinition {
        status: Label,
        application_interpreted_model_schema_name: Label,
        application_protocol_year: YearNumber,
        application: ApplicationContext,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct Approval {
        status: ApprovalStatus,
        level: Label,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ApprovalAssignment {
        assigned_approval: Approval,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ApprovalDateTime {
        date_time: DateTimeSelect,
        dated_approval: Approval,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ApprovalPersonOrganization {
        person_organization: PersonOrganizationSelect,
        authorized_approval: Approval,
        role: ApprovalRole,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ApprovalRole {
        role: Label,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ApprovalStatus {
        name: Label,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct AreaInSet {
        area: PresentationArea,
        in_set: PresentationSet,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct Axis2Placement2D {
        placement: Placement,
        ref_direction: Option<Direction>,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct BSplineCurve {
        bounded_curve: BoundedCurve,
        degree: i64,
        control_points_list: CartesianPoint,
        curve_form: BSplineCurveForm,
        closed_curve: Logical,
        self_intersect: Logical,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct BSplineCurveWithKnots {
        b_spline_curve: BSplineCurve,
        knot_multiplicities: i64,
        knots: ParameterValue,
        knot_spec: KnotType,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct BezierCurve {
        b_spline_curve: BSplineCurve,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct BoundedCurve {
        curve: Curve,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct CalendarDate {
        date: Date,
        day_component: DayInMonthNumber,
        month_component: MonthInYearNumber,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct CameraImage {
        mapped_item: MappedItem,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct CameraImage2DWithScale {
        camera_image: CameraImage,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct CameraModel {
        geometric_representation_item: GeometricRepresentationItem,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct CameraModelD2 {
        camera_model: CameraModel,
        view_window: PlanarBox,
        view_window_clipping: bool,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct CameraUsage {
        representation_map: RepresentationMap,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct CartesianPoint {
        point: Point,
        coordinates: LengthMeasure,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct Circle {
        conic: Conic,
        radius: PositiveLengthMeasure,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct Colour {}
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ColourRgb {
        colour_specification: ColourSpecification,
        red: f64,
        green: f64,
        blue: f64,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ColourSpecification {
        colour: Colour,
        name: Colour,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct CompositeCurve {
        bounded_curve: BoundedCurve,
        segments: CompositeCurveSegment,
        self_intersect: Logical,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct CompositeCurveSegment {
        transition: TransitionCode,
        same_sense: bool,
        parent_curve: Curve,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct CompositeText {
        geometric_representation_item: GeometricRepresentationItem,
        collected_text: TextOrCharacter,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct CompositeTextWithAssociatedCurves {
        composite_text: CompositeText,
        associated_curves: Curve,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct CompositeTextWithBlankingBox {
        composite_text: CompositeText,
        blanking: PlanarBox,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct CompositeTextWithExtent {
        composite_text: CompositeText,
        extent: PlanarExtent,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct Conic {
        curve: Curve,
        position: Axis2Placement,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ContextDependentInvisibility {
        invisibility: Invisibility,
        presentation_context: InvisibilityContext,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct Contract {
        name: Label,
        purpose: Text,
        kind: ContractType,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ContractAssignment {
        assigned_contract: Contract,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ContractType {
        description: Label,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ConversionBasedUnit {
        named_unit: NamedUnit,
        name: Label,
        conversion_factor: MeasureWithUnit,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct Curve {
        geometric_representation_item: GeometricRepresentationItem,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct CurveDimension {
        dimension_curve_directed_callout: DimensionCurveDirectedCallout,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct CurveStyle {
        name: Label,
        curve_font: CurveFontOrScaledCurveFontSelect,
        curve_width: SizeSelect,
        curve_colour: Colour,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct CurveStyleFont {
        name: Label,
        pattern_list: CurveStyleFontPattern,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct CurveStyleFontPattern {
        visible_segment_length: PositiveLengthMeasure,
        invisible_segment_length: PositiveLengthMeasure,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct Date {
        year_component: YearNumber,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DatumFeatureCallout {
        draughting_callout: DraughtingCallout,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DatumTargetCallout {
        draughting_callout: DraughtingCallout,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DefinedSymbol {
        geometric_representation_item: GeometricRepresentationItem,
        definition: DefinedSymbolSelect,
        target: SymbolTarget,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DiameterDimension {
        dimension_curve_directed_callout: DimensionCurveDirectedCallout,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DimensionCalloutComponentRelationship {
        draughting_callout_relationship: DraughtingCalloutRelationship,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DimensionCalloutRelationship {
        draughting_callout_relationship: DraughtingCalloutRelationship,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DimensionCurve {
        annotation_curve_occurrence: AnnotationCurveOccurrence,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DimensionCurveDirectedCallout {
        draughting_callout: DraughtingCallout,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DimensionCurveTerminator {
        terminator_symbol: TerminatorSymbol,
        role: DimensionExtentUsage,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DimensionPair {
        draughting_callout_relationship: DraughtingCalloutRelationship,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DimensionalExponents {
        length_exponent: f64,
        mass_exponent: f64,
        time_exponent: f64,
        electric_current_exponent: f64,
        thermodynamic_temperature_exponent: f64,
        amount_of_substance_exponent: f64,
        luminous_intensity_exponent: f64,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct Direction {
        geometric_representation_item: GeometricRepresentationItem,
        direction_ratios: f64,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct Document {
        id: Identifier,
        name: Label,
        description: Text,
        kind: DocumentType,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DocumentReference {
        assigned_document: Document,
        source: Label,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DocumentType {
        product_data_type: Label,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DraughtingAnnotationOccurrence {
        annotation_occurrence: AnnotationOccurrence,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DraughtingApprovalAssignment {
        approval_assignment: ApprovalAssignment,
        approved_items: ApprovedItem,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DraughtingCallout {
        geometric_representation_item: GeometricRepresentationItem,
        contents: DraughtingCalloutElement,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DraughtingCalloutRelationship {
        name: Label,
        description: Text,
        relating_draughting_callout: DraughtingCallout,
        related_draughting_callout: DraughtingCallout,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DraughtingContractAssignment {
        contract_assignment: ContractAssignment,
        items: ContractedItem,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DraughtingDrawingRevision {
        drawing_revision: DrawingRevision,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DraughtingElements {
        draughting_callout: DraughtingCallout,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DraughtingGroupAssignment {
        group_assignment: GroupAssignment,
        items: DraughtingGroupedItem,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DraughtingModel {
        representation: Representation,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DraughtingOrganizationAssignment {
        organization_assignment: OrganizationAssignment,
        assigned_items: DraughtingOrganizationItem,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DraughtingPersonAndOrganizationAssignment {
        person_and_organization_assignment: PersonAndOrganizationAssignment,
        assigned_items: DraughtingOrganizationItem,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DraughtingPersonAssignment {
        person_assignment: PersonAssignment,
        assigned_items: DraughtingOrganizationItem,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DraughtingPreDefinedColour {
        pre_defined_colour: PreDefinedColour,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DraughtingPreDefinedCurveFont {
        pre_defined_curve_font: PreDefinedCurveFont,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DraughtingPreDefinedTextFont {
        pre_defined_text_font: PreDefinedTextFont,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DraughtingPresentedItem {
        presented_item: PresentedItem,
        items: DraughtingPresentedItemSelect,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DraughtingSecurityClassificationAssignment {
        security_classification_assignment: SecurityClassificationAssignment,
        assigned_items: ClassifiedItem,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DraughtingSpecificationReference {
        document_reference: DocumentReference,
        specified_items: SpecifiedItem,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DraughtingSubfigureRepresentation {
        symbol_representation: SymbolRepresentation,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DraughtingSymbolRepresentation {
        symbol_representation: SymbolRepresentation,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DraughtingTextLiteralWithDelineation {
        text_literal_with_delineation: TextLiteralWithDelineation,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DraughtingTitle {
        items: DraughtingTitledItem,
        language: Label,
        contents: Text,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DrawingDefinition {
        drawing_number: Identifier,
        drawing_type: Option<Label>,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DrawingRevision {
        presentation_set: PresentationSet,
        revision_identifier: Identifier,
        drawing_identifier: DrawingDefinition,
        intended_scale: Option<Text>,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DrawingSheetLayout {
        draughting_symbol_representation: DraughtingSymbolRepresentation,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DrawingSheetRevision {
        presentation_area: PresentationArea,
        revision_identifier: Identifier,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DrawingSheetRevisionUsage {
        area_in_set: AreaInSet,
        sheet_number: Identifier,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct Ellipse {
        conic: Conic,
        semi_axis_1: PositiveLengthMeasure,
        semi_axis_2: PositiveLengthMeasure,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ExternalSource {
        source_id: SourceItem,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ExternallyDefinedCurveFont {
        externally_defined_item: ExternallyDefinedItem,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ExternallyDefinedHatchStyle {
        externally_defined_item: ExternallyDefinedItem,
        geometric_representation_item: GeometricRepresentationItem,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ExternallyDefinedItem {
        item_id: SourceItem,
        source: ExternalSource,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ExternallyDefinedSymbol {
        externally_defined_item: ExternallyDefinedItem,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ExternallyDefinedTextFont {
        externally_defined_item: ExternallyDefinedItem,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ExternallyDefinedTileStyle {
        externally_defined_item: ExternallyDefinedItem,
        geometric_representation_item: GeometricRepresentationItem,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct FillAreaStyle {
        name: Label,
        fill_styles: FillStyleSelect,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct FillAreaStyleColour {
        name: Label,
        fill_colour: Colour,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct FillAreaStyleHatching {
        geometric_representation_item: GeometricRepresentationItem,
        hatch_line_appearance: CurveStyle,
        start_of_next_hatch_line: OneDirectionRepeatFactor,
        point_of_reference_hatch_line: CartesianPoint,
        pattern_start: CartesianPoint,
        hatch_line_angle: PlaneAngleMeasure,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct FillAreaStyleTileSymbolWithStyle {
        geometric_representation_item: GeometricRepresentationItem,
        symbol: AnnotationSymbolOccurrence,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct FillAreaStyleTiles {
        geometric_representation_item: GeometricRepresentationItem,
        tiling_pattern: TwoDirectionRepeatFactor,
        tiles: FillAreaStyleTileShapeSelect,
        tiling_scale: PositiveRatioMeasure,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct GeometricCurveSet {
        geometric_set: GeometricSet,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct GeometricRepresentationContext {
        representation_context: RepresentationContext,
        coordinate_space_dimension: DimensionCount,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct GeometricRepresentationItem {
        representation_item: RepresentationItem,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct GeometricSet {
        geometric_representation_item: GeometricRepresentationItem,
        elements: GeometricSetSelect,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct GeometricalToleranceCallout {
        draughting_callout: DraughtingCallout,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct GeometricallyBounded2DWireframeRepresentation {
        shape_representation: ShapeRepresentation,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct GlobalUnitAssignedContext {
        representation_context: RepresentationContext,
        units: Unit,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct Group {
        name: Label,
        description: Text,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct GroupAssignment {
        assigned_group: Group,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct GroupRelationship {
        name: Label,
        description: Text,
        relating_group: Group,
        related_group: Group,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct Hyperbola {
        conic: Conic,
        semi_axis: PositiveLengthMeasure,
        semi_imag_axis: PositiveLengthMeasure,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct Invisibility {
        invisible_items: InvisibleItem,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct LeaderCurve {
        annotation_curve_occurrence: AnnotationCurveOccurrence,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct LeaderDirectedCallout {
        draughting_callout: DraughtingCallout,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct LeaderDirectedDimension {
        leader_directed_callout: LeaderDirectedCallout,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct LeaderTerminator {
        terminator_symbol: TerminatorSymbol,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct LengthMeasureWithUnit {
        measure_with_unit: MeasureWithUnit,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct LengthUnit {
        named_unit: NamedUnit,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct Line {
        curve: Curve,
        pnt: CartesianPoint,
        dir: Vector,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct LinearDimension {
        dimension_curve_directed_callout: DimensionCurveDirectedCallout,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct MappedItem {
        representation_item: RepresentationItem,
        mapping_source: RepresentationMap,
        mapping_target: RepresentationItem,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct MeasureWithUnit {
        value_component: MeasureValue,
        unit_component: Unit,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct NamedUnit {
        dimensions: DimensionalExponents,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct OffsetCurve2D {
        curve: Curve,
        basis_curve: Curve,
        distance: LengthMeasure,
        self_intersect: Logical,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct OneDirectionRepeatFactor {
        geometric_representation_item: GeometricRepresentationItem,
        repeat_factor: Vector,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct OrdinateDimension {
        projection_directed_callout: ProjectionDirectedCallout,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct Organization {
        id: Option<Identifier>,
        name: Label,
        description: Text,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct OrganizationAssignment {
        assigned_organization: Organization,
        role: OrganizationRole,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct OrganizationRole {
        name: Label,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct OrganizationalAddress {
        address: Address,
        organizations: Organization,
        description: Text,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct Parabola {
        conic: Conic,
        focal_dist: LengthMeasure,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct Person {
        id: Identifier,
        last_name: Option<Label>,
        first_name: Option<Label>,
        middle_names: Option<Label>,
        prefix_titles: Option<Label>,
        suffix_titles: Option<Label>,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PersonAndOrganization {
        the_person: Person,
        the_organization: Organization,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PersonAndOrganizationAssignment {
        assigned_person_and_organization: PersonAndOrganization,
        role: PersonAndOrganizationRole,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PersonAndOrganizationRole {
        name: Label,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PersonAssignment {
        assigned_person: Person,
        role: PersonRole,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PersonRole {
        name: Label,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PersonalAddress {
        address: Address,
        people: Person,
        description: Text,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct Placement {
        geometric_representation_item: GeometricRepresentationItem,
        location: CartesianPoint,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PlanarBox {
        planar_extent: PlanarExtent,
        placement: Axis2Placement,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PlanarExtent {
        geometric_representation_item: GeometricRepresentationItem,
        size_in_x: LengthMeasure,
        size_in_y: LengthMeasure,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PlaneAngleMeasureWithUnit {
        measure_with_unit: MeasureWithUnit,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PlaneAngleUnit {
        named_unit: NamedUnit,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct Point {
        geometric_representation_item: GeometricRepresentationItem,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PointOnCurve {
        point: Point,
        basis_curve: Curve,
        point_parameter: ParameterValue,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct Polyline {
        bounded_curve: BoundedCurve,
        points: CartesianPoint,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PreDefinedColour {
        pre_defined_item: PreDefinedItem,
        colour: Colour,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PreDefinedCurveFont {
        pre_defined_item: PreDefinedItem,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PreDefinedDimensionSymbol {
        pre_defined_symbol: PreDefinedSymbol,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PreDefinedGeometricalToleranceSymbol {
        pre_defined_symbol: PreDefinedSymbol,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PreDefinedItem {
        name: Label,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PreDefinedPointMarkerSymbol {
        pre_defined_symbol: PreDefinedSymbol,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PreDefinedSymbol {
        pre_defined_item: PreDefinedItem,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PreDefinedTerminatorSymbol {
        pre_defined_symbol: PreDefinedSymbol,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PreDefinedTextFont {
        pre_defined_item: PreDefinedItem,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PresentationArea {
        presentation_representation: PresentationRepresentation,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PresentationLayerAssignment {
        name: Label,
        description: Text,
        assigned_items: LayeredItem,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PresentationLayerUsage {
        assignment: PresentationLayerAssignment,
        presentation: PresentationRepresentation,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PresentationRepresentation {
        representation: Representation,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PresentationSet {}
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PresentationSize {
        unit: PresentationSizeAssignmentSelect,
        size: PlanarBox,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PresentationStyleAssignment {
        styles: PresentationStyleSelect,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PresentationStyleByContext {
        presentation_style_assignment: PresentationStyleAssignment,
        style_context: StyleContextSelect,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PresentationView {
        presentation_representation: PresentationRepresentation,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PresentedItem {}
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PresentedItemRepresentation {
        presentation: PresentationRepresentationSelect,
        item: PresentedItem,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct Product {
        id: Identifier,
        name: Label,
        description: Text,
        frame_of_reference: ProductContext,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ProductContext {
        application_context_element: ApplicationContextElement,
        discipline_type: Label,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ProductDefinition {
        id: Identifier,
        description: Text,
        formation: ProductDefinitionFormation,
        frame_of_reference: ProductDefinitionContext,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ProductDefinitionContext {
        application_context_element: ApplicationContextElement,
        life_cycle_stage: Label,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ProductDefinitionFormation {
        id: Identifier,
        description: Text,
        of_product: Product,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ProductDefinitionShape {
        property_definition: PropertyDefinition,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ProjectionCurve {
        annotation_curve_occurrence: AnnotationCurveOccurrence,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ProjectionDirectedCallout {
        draughting_callout: DraughtingCallout,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PropertyDefinition {
        name: Label,
        description: Text,
        definition: CharacterizedDefinition,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PropertyDefinitionRepresentation {
        definition: PropertyDefinition,
        used_representation: Representation,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct QuasiUniformCurve {
        b_spline_curve: BSplineCurve,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct RadiusDimension {
        dimension_curve_directed_callout: DimensionCurveDirectedCallout,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct RationalBSplineCurve {
        b_spline_curve: BSplineCurve,
        weights_data: f64,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct Representation {
        name: Label,
        items: RepresentationItem,
        context_of_items: RepresentationContext,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct RepresentationContext {
        context_identifier: Identifier,
        context_type: Text,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct RepresentationItem {
        name: Label,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct RepresentationMap {
        mapping_origin: RepresentationItem,
        mapped_representation: Representation,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct SecurityClassification {
        name: Label,
        purpose: Text,
        security_level: SecurityClassificationLevel,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct SecurityClassificationAssignment {
        assigned_security_classification: SecurityClassification,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct SecurityClassificationLevel {
        name: Label,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ShapeDefinitionRepresentation {
        property_definition_representation: PropertyDefinitionRepresentation,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ShapeRepresentation {
        representation: Representation,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct SiUnit {
        named_unit: NamedUnit,
        prefix: Option<SiPrefix>,
        name: SiUnitName,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct StructuredDimensionCallout {
        draughting_callout: DraughtingCallout,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct StyledItem {
        representation_item: RepresentationItem,
        styles: PresentationStyleAssignment,
        item: RepresentationItem,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct SymbolColour {
        colour_of_symbol: Colour,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct SymbolRepresentation {
        representation: Representation,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct SymbolRepresentationMap {
        representation_map: RepresentationMap,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct SymbolStyle {
        name: Label,
        style_of_symbol: SymbolStyleSelect,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct SymbolTarget {
        geometric_representation_item: GeometricRepresentationItem,
        placement: Axis2Placement,
        x_scale: PositiveRatioMeasure,
        y_scale: PositiveRatioMeasure,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct TerminatorSymbol {
        annotation_symbol_occurrence: AnnotationSymbolOccurrence,
        annotated_curve: AnnotationCurveOccurrence,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct TextLiteral {
        geometric_representation_item: GeometricRepresentationItem,
        literal: PresentableText,
        placement: Axis2Placement,
        alignment: TextAlignment,
        path: TextPath,
        font: FontSelect,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct TextLiteralWithAssociatedCurves {
        text_literal: TextLiteral,
        associated_curves: Curve,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct TextLiteralWithBlankingBox {
        text_literal: TextLiteral,
        blanking: PlanarBox,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct TextLiteralWithDelineation {
        text_literal: TextLiteral,
        delineation: TextDelineation,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct TextLiteralWithExtent {
        text_literal: TextLiteral,
        extent: PlanarExtent,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct TextStyle {
        name: Label,
        character_appearance: CharacterStyleSelect,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct TextStyleForDefinedFont {
        text_colour: Colour,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct TextStyleWithBoxCharacteristics {
        text_style: TextStyle,
        characteristics: BoxCharacteristicSelect,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct TextStyleWithMirror {
        text_style: TextStyle,
        mirror_placement: Axis2Placement,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct TrimmedCurve {
        bounded_curve: BoundedCurve,
        basis_curve: Curve,
        trim_1: TrimmingSelect,
        trim_2: TrimmingSelect,
        sense_agreement: bool,
        master_representation: TrimmingPreference,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct TwoDirectionRepeatFactor {
        one_direction_repeat_factor: OneDirectionRepeatFactor,
        second_repeat_factor: Vector,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct UniformCurve {
        b_spline_curve: BSplineCurve,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct Vector {
        geometric_representation_item: GeometricRepresentationItem,
        orientation: Direction,
        magnitude: LengthMeasure,
    }
}

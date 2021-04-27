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
    #[derive(Clone, Debug, PartialEq)]
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
    impl Address {
        pub fn new(
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
        ) -> Self {
            Self {
                internal_location,
                street_number,
                street,
                postal_box,
                town,
                region,
                postal_code,
                country,
                facsimile_number,
                telephone_number,
                electronic_mail_address,
                telex_number,
            }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct AngularDimension {}
    impl AngularDimension {
        pub fn new() -> Self {
            Self {}
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct AnnotationCurveOccurrence {}
    impl AnnotationCurveOccurrence {
        pub fn new() -> Self {
            Self {}
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct AnnotationFillArea {
        boundaries: Curve,
    }
    impl AnnotationFillArea {
        pub fn new(boundaries: Curve) -> Self {
            Self { boundaries }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct AnnotationFillAreaOccurrence {
        fill_style_target: Point,
    }
    impl AnnotationFillAreaOccurrence {
        pub fn new(fill_style_target: Point) -> Self {
            Self { fill_style_target }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct AnnotationOccurrence {}
    impl AnnotationOccurrence {
        pub fn new() -> Self {
            Self {}
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct AnnotationSubfigureOccurrence {}
    impl AnnotationSubfigureOccurrence {
        pub fn new() -> Self {
            Self {}
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct AnnotationSymbol {}
    impl AnnotationSymbol {
        pub fn new() -> Self {
            Self {}
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct AnnotationSymbolOccurrence {}
    impl AnnotationSymbolOccurrence {
        pub fn new() -> Self {
            Self {}
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct AnnotationText {}
    impl AnnotationText {
        pub fn new() -> Self {
            Self {}
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct AnnotationTextOccurrence {}
    impl AnnotationTextOccurrence {
        pub fn new() -> Self {
            Self {}
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct ApplicationContext {
        application: Text,
    }
    impl ApplicationContext {
        pub fn new(application: Text) -> Self {
            Self { application }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct ApplicationContextElement {
        name: Label,
        frame_of_reference: ApplicationContext,
    }
    impl ApplicationContextElement {
        pub fn new(name: Label, frame_of_reference: ApplicationContext) -> Self {
            Self {
                name,
                frame_of_reference,
            }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct ApplicationProtocolDefinition {
        status: Label,
        application_interpreted_model_schema_name: Label,
        application_protocol_year: YearNumber,
        application: ApplicationContext,
    }
    impl ApplicationProtocolDefinition {
        pub fn new(
            status: Label,
            application_interpreted_model_schema_name: Label,
            application_protocol_year: YearNumber,
            application: ApplicationContext,
        ) -> Self {
            Self {
                status,
                application_interpreted_model_schema_name,
                application_protocol_year,
                application,
            }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct Approval {
        status: ApprovalStatus,
        level: Label,
    }
    impl Approval {
        pub fn new(status: ApprovalStatus, level: Label) -> Self {
            Self { status, level }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct ApprovalAssignment {
        assigned_approval: Approval,
    }
    impl ApprovalAssignment {
        pub fn new(assigned_approval: Approval) -> Self {
            Self { assigned_approval }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct ApprovalDateTime {
        date_time: DateTimeSelect,
        dated_approval: Approval,
    }
    impl ApprovalDateTime {
        pub fn new(date_time: DateTimeSelect, dated_approval: Approval) -> Self {
            Self {
                date_time,
                dated_approval,
            }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct ApprovalPersonOrganization {
        person_organization: PersonOrganizationSelect,
        authorized_approval: Approval,
        role: ApprovalRole,
    }
    impl ApprovalPersonOrganization {
        pub fn new(
            person_organization: PersonOrganizationSelect,
            authorized_approval: Approval,
            role: ApprovalRole,
        ) -> Self {
            Self {
                person_organization,
                authorized_approval,
                role,
            }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct ApprovalRole {
        role: Label,
    }
    impl ApprovalRole {
        pub fn new(role: Label) -> Self {
            Self { role }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct ApprovalStatus {
        name: Label,
    }
    impl ApprovalStatus {
        pub fn new(name: Label) -> Self {
            Self { name }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct AreaInSet {
        area: PresentationArea,
        in_set: PresentationSet,
    }
    impl AreaInSet {
        pub fn new(area: PresentationArea, in_set: PresentationSet) -> Self {
            Self { area, in_set }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct Axis2Placement2D {
        ref_direction: Option<Direction>,
    }
    impl Axis2Placement2D {
        pub fn new(ref_direction: Option<Direction>) -> Self {
            Self { ref_direction }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct BSplineCurve {
        degree: i64,
        control_points_list: CartesianPoint,
        curve_form: BSplineCurveForm,
        closed_curve: Logical,
        self_intersect: Logical,
    }
    impl BSplineCurve {
        pub fn new(
            degree: i64,
            control_points_list: CartesianPoint,
            curve_form: BSplineCurveForm,
            closed_curve: Logical,
            self_intersect: Logical,
        ) -> Self {
            Self {
                degree,
                control_points_list,
                curve_form,
                closed_curve,
                self_intersect,
            }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct BSplineCurveWithKnots {
        knot_multiplicities: i64,
        knots: ParameterValue,
        knot_spec: KnotType,
    }
    impl BSplineCurveWithKnots {
        pub fn new(knot_multiplicities: i64, knots: ParameterValue, knot_spec: KnotType) -> Self {
            Self {
                knot_multiplicities,
                knots,
                knot_spec,
            }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct BezierCurve {}
    impl BezierCurve {
        pub fn new() -> Self {
            Self {}
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct BoundedCurve {}
    impl BoundedCurve {
        pub fn new() -> Self {
            Self {}
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct CalendarDate {
        day_component: DayInMonthNumber,
        month_component: MonthInYearNumber,
    }
    impl CalendarDate {
        pub fn new(day_component: DayInMonthNumber, month_component: MonthInYearNumber) -> Self {
            Self {
                day_component,
                month_component,
            }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct CameraImage {}
    impl CameraImage {
        pub fn new() -> Self {
            Self {}
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct CameraImage2DWithScale {}
    impl CameraImage2DWithScale {
        pub fn new() -> Self {
            Self {}
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct CameraModel {}
    impl CameraModel {
        pub fn new() -> Self {
            Self {}
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct CameraModelD2 {
        view_window: PlanarBox,
        view_window_clipping: bool,
    }
    impl CameraModelD2 {
        pub fn new(view_window: PlanarBox, view_window_clipping: bool) -> Self {
            Self {
                view_window,
                view_window_clipping,
            }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct CameraUsage {}
    impl CameraUsage {
        pub fn new() -> Self {
            Self {}
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct CartesianPoint {
        coordinates: LengthMeasure,
    }
    impl CartesianPoint {
        pub fn new(coordinates: LengthMeasure) -> Self {
            Self { coordinates }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct Circle {
        radius: PositiveLengthMeasure,
    }
    impl Circle {
        pub fn new(radius: PositiveLengthMeasure) -> Self {
            Self { radius }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct Colour {}
    impl Colour {
        pub fn new() -> Self {
            Self {}
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct ColourRgb {
        red: f64,
        green: f64,
        blue: f64,
    }
    impl ColourRgb {
        pub fn new(red: f64, green: f64, blue: f64) -> Self {
            Self { red, green, blue }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct ColourSpecification {
        name: Colour,
    }
    impl ColourSpecification {
        pub fn new(name: Colour) -> Self {
            Self { name }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct CompositeCurve {
        segments: CompositeCurveSegment,
        self_intersect: Logical,
    }
    impl CompositeCurve {
        pub fn new(segments: CompositeCurveSegment, self_intersect: Logical) -> Self {
            Self {
                segments,
                self_intersect,
            }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct CompositeCurveSegment {
        transition: TransitionCode,
        same_sense: bool,
        parent_curve: Curve,
    }
    impl CompositeCurveSegment {
        pub fn new(transition: TransitionCode, same_sense: bool, parent_curve: Curve) -> Self {
            Self {
                transition,
                same_sense,
                parent_curve,
            }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct CompositeText {
        collected_text: TextOrCharacter,
    }
    impl CompositeText {
        pub fn new(collected_text: TextOrCharacter) -> Self {
            Self { collected_text }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct CompositeTextWithAssociatedCurves {
        associated_curves: Curve,
    }
    impl CompositeTextWithAssociatedCurves {
        pub fn new(associated_curves: Curve) -> Self {
            Self { associated_curves }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct CompositeTextWithBlankingBox {
        blanking: PlanarBox,
    }
    impl CompositeTextWithBlankingBox {
        pub fn new(blanking: PlanarBox) -> Self {
            Self { blanking }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct CompositeTextWithExtent {
        extent: PlanarExtent,
    }
    impl CompositeTextWithExtent {
        pub fn new(extent: PlanarExtent) -> Self {
            Self { extent }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct Conic {
        position: Axis2Placement,
    }
    impl Conic {
        pub fn new(position: Axis2Placement) -> Self {
            Self { position }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct ContextDependentInvisibility {
        presentation_context: InvisibilityContext,
    }
    impl ContextDependentInvisibility {
        pub fn new(presentation_context: InvisibilityContext) -> Self {
            Self {
                presentation_context,
            }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct Contract {
        name: Label,
        purpose: Text,
        kind: ContractType,
    }
    impl Contract {
        pub fn new(name: Label, purpose: Text, kind: ContractType) -> Self {
            Self {
                name,
                purpose,
                kind,
            }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct ContractAssignment {
        assigned_contract: Contract,
    }
    impl ContractAssignment {
        pub fn new(assigned_contract: Contract) -> Self {
            Self { assigned_contract }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct ContractType {
        description: Label,
    }
    impl ContractType {
        pub fn new(description: Label) -> Self {
            Self { description }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct ConversionBasedUnit {
        name: Label,
        conversion_factor: MeasureWithUnit,
    }
    impl ConversionBasedUnit {
        pub fn new(name: Label, conversion_factor: MeasureWithUnit) -> Self {
            Self {
                name,
                conversion_factor,
            }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct Curve {}
    impl Curve {
        pub fn new() -> Self {
            Self {}
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct CurveDimension {}
    impl CurveDimension {
        pub fn new() -> Self {
            Self {}
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct CurveStyle {
        name: Label,
        curve_font: CurveFontOrScaledCurveFontSelect,
        curve_width: SizeSelect,
        curve_colour: Colour,
    }
    impl CurveStyle {
        pub fn new(
            name: Label,
            curve_font: CurveFontOrScaledCurveFontSelect,
            curve_width: SizeSelect,
            curve_colour: Colour,
        ) -> Self {
            Self {
                name,
                curve_font,
                curve_width,
                curve_colour,
            }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct CurveStyleFont {
        name: Label,
        pattern_list: CurveStyleFontPattern,
    }
    impl CurveStyleFont {
        pub fn new(name: Label, pattern_list: CurveStyleFontPattern) -> Self {
            Self { name, pattern_list }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct CurveStyleFontPattern {
        visible_segment_length: PositiveLengthMeasure,
        invisible_segment_length: PositiveLengthMeasure,
    }
    impl CurveStyleFontPattern {
        pub fn new(
            visible_segment_length: PositiveLengthMeasure,
            invisible_segment_length: PositiveLengthMeasure,
        ) -> Self {
            Self {
                visible_segment_length,
                invisible_segment_length,
            }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct Date {
        year_component: YearNumber,
    }
    impl Date {
        pub fn new(year_component: YearNumber) -> Self {
            Self { year_component }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct DatumFeatureCallout {}
    impl DatumFeatureCallout {
        pub fn new() -> Self {
            Self {}
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct DatumTargetCallout {}
    impl DatumTargetCallout {
        pub fn new() -> Self {
            Self {}
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct DefinedSymbol {
        definition: DefinedSymbolSelect,
        target: SymbolTarget,
    }
    impl DefinedSymbol {
        pub fn new(definition: DefinedSymbolSelect, target: SymbolTarget) -> Self {
            Self { definition, target }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct DiameterDimension {}
    impl DiameterDimension {
        pub fn new() -> Self {
            Self {}
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct DimensionCalloutComponentRelationship {}
    impl DimensionCalloutComponentRelationship {
        pub fn new() -> Self {
            Self {}
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct DimensionCalloutRelationship {}
    impl DimensionCalloutRelationship {
        pub fn new() -> Self {
            Self {}
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct DimensionCurve {}
    impl DimensionCurve {
        pub fn new() -> Self {
            Self {}
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct DimensionCurveDirectedCallout {}
    impl DimensionCurveDirectedCallout {
        pub fn new() -> Self {
            Self {}
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct DimensionCurveTerminator {
        role: DimensionExtentUsage,
    }
    impl DimensionCurveTerminator {
        pub fn new(role: DimensionExtentUsage) -> Self {
            Self { role }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct DimensionPair {}
    impl DimensionPair {
        pub fn new() -> Self {
            Self {}
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct DimensionalExponents {
        length_exponent: f64,
        mass_exponent: f64,
        time_exponent: f64,
        electric_current_exponent: f64,
        thermodynamic_temperature_exponent: f64,
        amount_of_substance_exponent: f64,
        luminous_intensity_exponent: f64,
    }
    impl DimensionalExponents {
        pub fn new(
            length_exponent: f64,
            mass_exponent: f64,
            time_exponent: f64,
            electric_current_exponent: f64,
            thermodynamic_temperature_exponent: f64,
            amount_of_substance_exponent: f64,
            luminous_intensity_exponent: f64,
        ) -> Self {
            Self {
                length_exponent,
                mass_exponent,
                time_exponent,
                electric_current_exponent,
                thermodynamic_temperature_exponent,
                amount_of_substance_exponent,
                luminous_intensity_exponent,
            }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct Direction {
        direction_ratios: f64,
    }
    impl Direction {
        pub fn new(direction_ratios: f64) -> Self {
            Self { direction_ratios }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct Document {
        id: Identifier,
        name: Label,
        description: Text,
        kind: DocumentType,
    }
    impl Document {
        pub fn new(id: Identifier, name: Label, description: Text, kind: DocumentType) -> Self {
            Self {
                id,
                name,
                description,
                kind,
            }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct DocumentReference {
        assigned_document: Document,
        source: Label,
    }
    impl DocumentReference {
        pub fn new(assigned_document: Document, source: Label) -> Self {
            Self {
                assigned_document,
                source,
            }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct DocumentType {
        product_data_type: Label,
    }
    impl DocumentType {
        pub fn new(product_data_type: Label) -> Self {
            Self { product_data_type }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct DraughtingAnnotationOccurrence {}
    impl DraughtingAnnotationOccurrence {
        pub fn new() -> Self {
            Self {}
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct DraughtingApprovalAssignment {
        approved_items: ApprovedItem,
    }
    impl DraughtingApprovalAssignment {
        pub fn new(approved_items: ApprovedItem) -> Self {
            Self { approved_items }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct DraughtingCallout {
        contents: DraughtingCalloutElement,
    }
    impl DraughtingCallout {
        pub fn new(contents: DraughtingCalloutElement) -> Self {
            Self { contents }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct DraughtingCalloutRelationship {
        name: Label,
        description: Text,
        relating_draughting_callout: DraughtingCallout,
        related_draughting_callout: DraughtingCallout,
    }
    impl DraughtingCalloutRelationship {
        pub fn new(
            name: Label,
            description: Text,
            relating_draughting_callout: DraughtingCallout,
            related_draughting_callout: DraughtingCallout,
        ) -> Self {
            Self {
                name,
                description,
                relating_draughting_callout,
                related_draughting_callout,
            }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct DraughtingContractAssignment {
        items: ContractedItem,
    }
    impl DraughtingContractAssignment {
        pub fn new(items: ContractedItem) -> Self {
            Self { items }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct DraughtingDrawingRevision {}
    impl DraughtingDrawingRevision {
        pub fn new() -> Self {
            Self {}
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct DraughtingElements {}
    impl DraughtingElements {
        pub fn new() -> Self {
            Self {}
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct DraughtingGroupAssignment {
        items: DraughtingGroupedItem,
    }
    impl DraughtingGroupAssignment {
        pub fn new(items: DraughtingGroupedItem) -> Self {
            Self { items }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct DraughtingModel {}
    impl DraughtingModel {
        pub fn new() -> Self {
            Self {}
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct DraughtingOrganizationAssignment {
        assigned_items: DraughtingOrganizationItem,
    }
    impl DraughtingOrganizationAssignment {
        pub fn new(assigned_items: DraughtingOrganizationItem) -> Self {
            Self { assigned_items }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct DraughtingPersonAndOrganizationAssignment {
        assigned_items: DraughtingOrganizationItem,
    }
    impl DraughtingPersonAndOrganizationAssignment {
        pub fn new(assigned_items: DraughtingOrganizationItem) -> Self {
            Self { assigned_items }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct DraughtingPersonAssignment {
        assigned_items: DraughtingOrganizationItem,
    }
    impl DraughtingPersonAssignment {
        pub fn new(assigned_items: DraughtingOrganizationItem) -> Self {
            Self { assigned_items }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct DraughtingPreDefinedColour {}
    impl DraughtingPreDefinedColour {
        pub fn new() -> Self {
            Self {}
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct DraughtingPreDefinedCurveFont {}
    impl DraughtingPreDefinedCurveFont {
        pub fn new() -> Self {
            Self {}
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct DraughtingPreDefinedTextFont {}
    impl DraughtingPreDefinedTextFont {
        pub fn new() -> Self {
            Self {}
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct DraughtingPresentedItem {
        items: DraughtingPresentedItemSelect,
    }
    impl DraughtingPresentedItem {
        pub fn new(items: DraughtingPresentedItemSelect) -> Self {
            Self { items }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct DraughtingSecurityClassificationAssignment {
        assigned_items: ClassifiedItem,
    }
    impl DraughtingSecurityClassificationAssignment {
        pub fn new(assigned_items: ClassifiedItem) -> Self {
            Self { assigned_items }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct DraughtingSpecificationReference {
        specified_items: SpecifiedItem,
    }
    impl DraughtingSpecificationReference {
        pub fn new(specified_items: SpecifiedItem) -> Self {
            Self { specified_items }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct DraughtingSubfigureRepresentation {}
    impl DraughtingSubfigureRepresentation {
        pub fn new() -> Self {
            Self {}
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct DraughtingSymbolRepresentation {}
    impl DraughtingSymbolRepresentation {
        pub fn new() -> Self {
            Self {}
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct DraughtingTextLiteralWithDelineation {}
    impl DraughtingTextLiteralWithDelineation {
        pub fn new() -> Self {
            Self {}
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct DraughtingTitle {
        items: DraughtingTitledItem,
        language: Label,
        contents: Text,
    }
    impl DraughtingTitle {
        pub fn new(items: DraughtingTitledItem, language: Label, contents: Text) -> Self {
            Self {
                items,
                language,
                contents,
            }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct DrawingDefinition {
        drawing_number: Identifier,
        drawing_type: Option<Label>,
    }
    impl DrawingDefinition {
        pub fn new(drawing_number: Identifier, drawing_type: Option<Label>) -> Self {
            Self {
                drawing_number,
                drawing_type,
            }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct DrawingRevision {
        revision_identifier: Identifier,
        drawing_identifier: DrawingDefinition,
        intended_scale: Option<Text>,
    }
    impl DrawingRevision {
        pub fn new(
            revision_identifier: Identifier,
            drawing_identifier: DrawingDefinition,
            intended_scale: Option<Text>,
        ) -> Self {
            Self {
                revision_identifier,
                drawing_identifier,
                intended_scale,
            }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct DrawingSheetLayout {}
    impl DrawingSheetLayout {
        pub fn new() -> Self {
            Self {}
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct DrawingSheetRevision {
        revision_identifier: Identifier,
    }
    impl DrawingSheetRevision {
        pub fn new(revision_identifier: Identifier) -> Self {
            Self {
                revision_identifier,
            }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct DrawingSheetRevisionUsage {
        sheet_number: Identifier,
    }
    impl DrawingSheetRevisionUsage {
        pub fn new(sheet_number: Identifier) -> Self {
            Self { sheet_number }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct Ellipse {
        semi_axis_1: PositiveLengthMeasure,
        semi_axis_2: PositiveLengthMeasure,
    }
    impl Ellipse {
        pub fn new(semi_axis_1: PositiveLengthMeasure, semi_axis_2: PositiveLengthMeasure) -> Self {
            Self {
                semi_axis_1,
                semi_axis_2,
            }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct ExternalSource {
        source_id: SourceItem,
    }
    impl ExternalSource {
        pub fn new(source_id: SourceItem) -> Self {
            Self { source_id }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct ExternallyDefinedCurveFont {}
    impl ExternallyDefinedCurveFont {
        pub fn new() -> Self {
            Self {}
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct ExternallyDefinedHatchStyle {}
    impl ExternallyDefinedHatchStyle {
        pub fn new() -> Self {
            Self {}
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct ExternallyDefinedItem {
        item_id: SourceItem,
        source: ExternalSource,
    }
    impl ExternallyDefinedItem {
        pub fn new(item_id: SourceItem, source: ExternalSource) -> Self {
            Self { item_id, source }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct ExternallyDefinedSymbol {}
    impl ExternallyDefinedSymbol {
        pub fn new() -> Self {
            Self {}
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct ExternallyDefinedTextFont {}
    impl ExternallyDefinedTextFont {
        pub fn new() -> Self {
            Self {}
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct ExternallyDefinedTileStyle {}
    impl ExternallyDefinedTileStyle {
        pub fn new() -> Self {
            Self {}
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct FillAreaStyle {
        name: Label,
        fill_styles: FillStyleSelect,
    }
    impl FillAreaStyle {
        pub fn new(name: Label, fill_styles: FillStyleSelect) -> Self {
            Self { name, fill_styles }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct FillAreaStyleColour {
        name: Label,
        fill_colour: Colour,
    }
    impl FillAreaStyleColour {
        pub fn new(name: Label, fill_colour: Colour) -> Self {
            Self { name, fill_colour }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct FillAreaStyleHatching {
        hatch_line_appearance: CurveStyle,
        start_of_next_hatch_line: OneDirectionRepeatFactor,
        point_of_reference_hatch_line: CartesianPoint,
        pattern_start: CartesianPoint,
        hatch_line_angle: PlaneAngleMeasure,
    }
    impl FillAreaStyleHatching {
        pub fn new(
            hatch_line_appearance: CurveStyle,
            start_of_next_hatch_line: OneDirectionRepeatFactor,
            point_of_reference_hatch_line: CartesianPoint,
            pattern_start: CartesianPoint,
            hatch_line_angle: PlaneAngleMeasure,
        ) -> Self {
            Self {
                hatch_line_appearance,
                start_of_next_hatch_line,
                point_of_reference_hatch_line,
                pattern_start,
                hatch_line_angle,
            }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct FillAreaStyleTileSymbolWithStyle {
        symbol: AnnotationSymbolOccurrence,
    }
    impl FillAreaStyleTileSymbolWithStyle {
        pub fn new(symbol: AnnotationSymbolOccurrence) -> Self {
            Self { symbol }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct FillAreaStyleTiles {
        tiling_pattern: TwoDirectionRepeatFactor,
        tiles: FillAreaStyleTileShapeSelect,
        tiling_scale: PositiveRatioMeasure,
    }
    impl FillAreaStyleTiles {
        pub fn new(
            tiling_pattern: TwoDirectionRepeatFactor,
            tiles: FillAreaStyleTileShapeSelect,
            tiling_scale: PositiveRatioMeasure,
        ) -> Self {
            Self {
                tiling_pattern,
                tiles,
                tiling_scale,
            }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct GeometricCurveSet {}
    impl GeometricCurveSet {
        pub fn new() -> Self {
            Self {}
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct GeometricRepresentationContext {
        coordinate_space_dimension: DimensionCount,
    }
    impl GeometricRepresentationContext {
        pub fn new(coordinate_space_dimension: DimensionCount) -> Self {
            Self {
                coordinate_space_dimension,
            }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct GeometricRepresentationItem {}
    impl GeometricRepresentationItem {
        pub fn new() -> Self {
            Self {}
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct GeometricSet {
        elements: GeometricSetSelect,
    }
    impl GeometricSet {
        pub fn new(elements: GeometricSetSelect) -> Self {
            Self { elements }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct GeometricalToleranceCallout {}
    impl GeometricalToleranceCallout {
        pub fn new() -> Self {
            Self {}
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct GeometricallyBounded2DWireframeRepresentation {}
    impl GeometricallyBounded2DWireframeRepresentation {
        pub fn new() -> Self {
            Self {}
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct GlobalUnitAssignedContext {
        units: Unit,
    }
    impl GlobalUnitAssignedContext {
        pub fn new(units: Unit) -> Self {
            Self { units }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct Group {
        name: Label,
        description: Text,
    }
    impl Group {
        pub fn new(name: Label, description: Text) -> Self {
            Self { name, description }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct GroupAssignment {
        assigned_group: Group,
    }
    impl GroupAssignment {
        pub fn new(assigned_group: Group) -> Self {
            Self { assigned_group }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct GroupRelationship {
        name: Label,
        description: Text,
        relating_group: Group,
        related_group: Group,
    }
    impl GroupRelationship {
        pub fn new(
            name: Label,
            description: Text,
            relating_group: Group,
            related_group: Group,
        ) -> Self {
            Self {
                name,
                description,
                relating_group,
                related_group,
            }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct Hyperbola {
        semi_axis: PositiveLengthMeasure,
        semi_imag_axis: PositiveLengthMeasure,
    }
    impl Hyperbola {
        pub fn new(
            semi_axis: PositiveLengthMeasure,
            semi_imag_axis: PositiveLengthMeasure,
        ) -> Self {
            Self {
                semi_axis,
                semi_imag_axis,
            }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct Invisibility {
        invisible_items: InvisibleItem,
    }
    impl Invisibility {
        pub fn new(invisible_items: InvisibleItem) -> Self {
            Self { invisible_items }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct LeaderCurve {}
    impl LeaderCurve {
        pub fn new() -> Self {
            Self {}
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct LeaderDirectedCallout {}
    impl LeaderDirectedCallout {
        pub fn new() -> Self {
            Self {}
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct LeaderDirectedDimension {}
    impl LeaderDirectedDimension {
        pub fn new() -> Self {
            Self {}
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct LeaderTerminator {}
    impl LeaderTerminator {
        pub fn new() -> Self {
            Self {}
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct LengthMeasureWithUnit {}
    impl LengthMeasureWithUnit {
        pub fn new() -> Self {
            Self {}
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct LengthUnit {}
    impl LengthUnit {
        pub fn new() -> Self {
            Self {}
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct Line {
        pnt: CartesianPoint,
        dir: Vector,
    }
    impl Line {
        pub fn new(pnt: CartesianPoint, dir: Vector) -> Self {
            Self { pnt, dir }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct LinearDimension {}
    impl LinearDimension {
        pub fn new() -> Self {
            Self {}
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct MappedItem {
        mapping_source: RepresentationMap,
        mapping_target: RepresentationItem,
    }
    impl MappedItem {
        pub fn new(mapping_source: RepresentationMap, mapping_target: RepresentationItem) -> Self {
            Self {
                mapping_source,
                mapping_target,
            }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct MeasureWithUnit {
        value_component: MeasureValue,
        unit_component: Unit,
    }
    impl MeasureWithUnit {
        pub fn new(value_component: MeasureValue, unit_component: Unit) -> Self {
            Self {
                value_component,
                unit_component,
            }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct NamedUnit {
        dimensions: DimensionalExponents,
    }
    impl NamedUnit {
        pub fn new(dimensions: DimensionalExponents) -> Self {
            Self { dimensions }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct OffsetCurve2D {
        basis_curve: Curve,
        distance: LengthMeasure,
        self_intersect: Logical,
    }
    impl OffsetCurve2D {
        pub fn new(basis_curve: Curve, distance: LengthMeasure, self_intersect: Logical) -> Self {
            Self {
                basis_curve,
                distance,
                self_intersect,
            }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct OneDirectionRepeatFactor {
        repeat_factor: Vector,
    }
    impl OneDirectionRepeatFactor {
        pub fn new(repeat_factor: Vector) -> Self {
            Self { repeat_factor }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct OrdinateDimension {}
    impl OrdinateDimension {
        pub fn new() -> Self {
            Self {}
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct Organization {
        id: Option<Identifier>,
        name: Label,
        description: Text,
    }
    impl Organization {
        pub fn new(id: Option<Identifier>, name: Label, description: Text) -> Self {
            Self {
                id,
                name,
                description,
            }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct OrganizationAssignment {
        assigned_organization: Organization,
        role: OrganizationRole,
    }
    impl OrganizationAssignment {
        pub fn new(assigned_organization: Organization, role: OrganizationRole) -> Self {
            Self {
                assigned_organization,
                role,
            }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct OrganizationRole {
        name: Label,
    }
    impl OrganizationRole {
        pub fn new(name: Label) -> Self {
            Self { name }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct OrganizationalAddress {
        organizations: Organization,
        description: Text,
    }
    impl OrganizationalAddress {
        pub fn new(organizations: Organization, description: Text) -> Self {
            Self {
                organizations,
                description,
            }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct Parabola {
        focal_dist: LengthMeasure,
    }
    impl Parabola {
        pub fn new(focal_dist: LengthMeasure) -> Self {
            Self { focal_dist }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct Person {
        id: Identifier,
        last_name: Option<Label>,
        first_name: Option<Label>,
        middle_names: Option<Label>,
        prefix_titles: Option<Label>,
        suffix_titles: Option<Label>,
    }
    impl Person {
        pub fn new(
            id: Identifier,
            last_name: Option<Label>,
            first_name: Option<Label>,
            middle_names: Option<Label>,
            prefix_titles: Option<Label>,
            suffix_titles: Option<Label>,
        ) -> Self {
            Self {
                id,
                last_name,
                first_name,
                middle_names,
                prefix_titles,
                suffix_titles,
            }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct PersonAndOrganization {
        the_person: Person,
        the_organization: Organization,
    }
    impl PersonAndOrganization {
        pub fn new(the_person: Person, the_organization: Organization) -> Self {
            Self {
                the_person,
                the_organization,
            }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct PersonAndOrganizationAssignment {
        assigned_person_and_organization: PersonAndOrganization,
        role: PersonAndOrganizationRole,
    }
    impl PersonAndOrganizationAssignment {
        pub fn new(
            assigned_person_and_organization: PersonAndOrganization,
            role: PersonAndOrganizationRole,
        ) -> Self {
            Self {
                assigned_person_and_organization,
                role,
            }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct PersonAndOrganizationRole {
        name: Label,
    }
    impl PersonAndOrganizationRole {
        pub fn new(name: Label) -> Self {
            Self { name }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct PersonAssignment {
        assigned_person: Person,
        role: PersonRole,
    }
    impl PersonAssignment {
        pub fn new(assigned_person: Person, role: PersonRole) -> Self {
            Self {
                assigned_person,
                role,
            }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct PersonRole {
        name: Label,
    }
    impl PersonRole {
        pub fn new(name: Label) -> Self {
            Self { name }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct PersonalAddress {
        people: Person,
        description: Text,
    }
    impl PersonalAddress {
        pub fn new(people: Person, description: Text) -> Self {
            Self {
                people,
                description,
            }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct Placement {
        location: CartesianPoint,
    }
    impl Placement {
        pub fn new(location: CartesianPoint) -> Self {
            Self { location }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct PlanarBox {
        placement: Axis2Placement,
    }
    impl PlanarBox {
        pub fn new(placement: Axis2Placement) -> Self {
            Self { placement }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct PlanarExtent {
        size_in_x: LengthMeasure,
        size_in_y: LengthMeasure,
    }
    impl PlanarExtent {
        pub fn new(size_in_x: LengthMeasure, size_in_y: LengthMeasure) -> Self {
            Self {
                size_in_x,
                size_in_y,
            }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct PlaneAngleMeasureWithUnit {}
    impl PlaneAngleMeasureWithUnit {
        pub fn new() -> Self {
            Self {}
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct PlaneAngleUnit {}
    impl PlaneAngleUnit {
        pub fn new() -> Self {
            Self {}
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct Point {}
    impl Point {
        pub fn new() -> Self {
            Self {}
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct PointOnCurve {
        basis_curve: Curve,
        point_parameter: ParameterValue,
    }
    impl PointOnCurve {
        pub fn new(basis_curve: Curve, point_parameter: ParameterValue) -> Self {
            Self {
                basis_curve,
                point_parameter,
            }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct Polyline {
        points: CartesianPoint,
    }
    impl Polyline {
        pub fn new(points: CartesianPoint) -> Self {
            Self { points }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct PreDefinedColour {}
    impl PreDefinedColour {
        pub fn new() -> Self {
            Self {}
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct PreDefinedCurveFont {}
    impl PreDefinedCurveFont {
        pub fn new() -> Self {
            Self {}
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct PreDefinedDimensionSymbol {}
    impl PreDefinedDimensionSymbol {
        pub fn new() -> Self {
            Self {}
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct PreDefinedGeometricalToleranceSymbol {}
    impl PreDefinedGeometricalToleranceSymbol {
        pub fn new() -> Self {
            Self {}
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct PreDefinedItem {
        name: Label,
    }
    impl PreDefinedItem {
        pub fn new(name: Label) -> Self {
            Self { name }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct PreDefinedPointMarkerSymbol {}
    impl PreDefinedPointMarkerSymbol {
        pub fn new() -> Self {
            Self {}
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct PreDefinedSymbol {}
    impl PreDefinedSymbol {
        pub fn new() -> Self {
            Self {}
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct PreDefinedTerminatorSymbol {}
    impl PreDefinedTerminatorSymbol {
        pub fn new() -> Self {
            Self {}
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct PreDefinedTextFont {}
    impl PreDefinedTextFont {
        pub fn new() -> Self {
            Self {}
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct PresentationArea {}
    impl PresentationArea {
        pub fn new() -> Self {
            Self {}
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct PresentationLayerAssignment {
        name: Label,
        description: Text,
        assigned_items: LayeredItem,
    }
    impl PresentationLayerAssignment {
        pub fn new(name: Label, description: Text, assigned_items: LayeredItem) -> Self {
            Self {
                name,
                description,
                assigned_items,
            }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct PresentationLayerUsage {
        assignment: PresentationLayerAssignment,
        presentation: PresentationRepresentation,
    }
    impl PresentationLayerUsage {
        pub fn new(
            assignment: PresentationLayerAssignment,
            presentation: PresentationRepresentation,
        ) -> Self {
            Self {
                assignment,
                presentation,
            }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct PresentationRepresentation {}
    impl PresentationRepresentation {
        pub fn new() -> Self {
            Self {}
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct PresentationSet {}
    impl PresentationSet {
        pub fn new() -> Self {
            Self {}
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct PresentationSize {
        unit: PresentationSizeAssignmentSelect,
        size: PlanarBox,
    }
    impl PresentationSize {
        pub fn new(unit: PresentationSizeAssignmentSelect, size: PlanarBox) -> Self {
            Self { unit, size }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct PresentationStyleAssignment {
        styles: PresentationStyleSelect,
    }
    impl PresentationStyleAssignment {
        pub fn new(styles: PresentationStyleSelect) -> Self {
            Self { styles }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct PresentationStyleByContext {
        style_context: StyleContextSelect,
    }
    impl PresentationStyleByContext {
        pub fn new(style_context: StyleContextSelect) -> Self {
            Self { style_context }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct PresentationView {}
    impl PresentationView {
        pub fn new() -> Self {
            Self {}
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct PresentedItem {}
    impl PresentedItem {
        pub fn new() -> Self {
            Self {}
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct PresentedItemRepresentation {
        presentation: PresentationRepresentationSelect,
        item: PresentedItem,
    }
    impl PresentedItemRepresentation {
        pub fn new(presentation: PresentationRepresentationSelect, item: PresentedItem) -> Self {
            Self { presentation, item }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct Product {
        id: Identifier,
        name: Label,
        description: Text,
        frame_of_reference: ProductContext,
    }
    impl Product {
        pub fn new(
            id: Identifier,
            name: Label,
            description: Text,
            frame_of_reference: ProductContext,
        ) -> Self {
            Self {
                id,
                name,
                description,
                frame_of_reference,
            }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct ProductContext {
        discipline_type: Label,
    }
    impl ProductContext {
        pub fn new(discipline_type: Label) -> Self {
            Self { discipline_type }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct ProductDefinition {
        id: Identifier,
        description: Text,
        formation: ProductDefinitionFormation,
        frame_of_reference: ProductDefinitionContext,
    }
    impl ProductDefinition {
        pub fn new(
            id: Identifier,
            description: Text,
            formation: ProductDefinitionFormation,
            frame_of_reference: ProductDefinitionContext,
        ) -> Self {
            Self {
                id,
                description,
                formation,
                frame_of_reference,
            }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct ProductDefinitionContext {
        life_cycle_stage: Label,
    }
    impl ProductDefinitionContext {
        pub fn new(life_cycle_stage: Label) -> Self {
            Self { life_cycle_stage }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct ProductDefinitionFormation {
        id: Identifier,
        description: Text,
        of_product: Product,
    }
    impl ProductDefinitionFormation {
        pub fn new(id: Identifier, description: Text, of_product: Product) -> Self {
            Self {
                id,
                description,
                of_product,
            }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct ProductDefinitionShape {}
    impl ProductDefinitionShape {
        pub fn new() -> Self {
            Self {}
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct ProjectionCurve {}
    impl ProjectionCurve {
        pub fn new() -> Self {
            Self {}
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct ProjectionDirectedCallout {}
    impl ProjectionDirectedCallout {
        pub fn new() -> Self {
            Self {}
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct PropertyDefinition {
        name: Label,
        description: Text,
        definition: CharacterizedDefinition,
    }
    impl PropertyDefinition {
        pub fn new(name: Label, description: Text, definition: CharacterizedDefinition) -> Self {
            Self {
                name,
                description,
                definition,
            }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct PropertyDefinitionRepresentation {
        definition: PropertyDefinition,
        used_representation: Representation,
    }
    impl PropertyDefinitionRepresentation {
        pub fn new(definition: PropertyDefinition, used_representation: Representation) -> Self {
            Self {
                definition,
                used_representation,
            }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct QuasiUniformCurve {}
    impl QuasiUniformCurve {
        pub fn new() -> Self {
            Self {}
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct RadiusDimension {}
    impl RadiusDimension {
        pub fn new() -> Self {
            Self {}
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct RationalBSplineCurve {
        weights_data: f64,
    }
    impl RationalBSplineCurve {
        pub fn new(weights_data: f64) -> Self {
            Self { weights_data }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct Representation {
        name: Label,
        items: RepresentationItem,
        context_of_items: RepresentationContext,
    }
    impl Representation {
        pub fn new(
            name: Label,
            items: RepresentationItem,
            context_of_items: RepresentationContext,
        ) -> Self {
            Self {
                name,
                items,
                context_of_items,
            }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct RepresentationContext {
        context_identifier: Identifier,
        context_type: Text,
    }
    impl RepresentationContext {
        pub fn new(context_identifier: Identifier, context_type: Text) -> Self {
            Self {
                context_identifier,
                context_type,
            }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct RepresentationItem {
        name: Label,
    }
    impl RepresentationItem {
        pub fn new(name: Label) -> Self {
            Self { name }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct RepresentationMap {
        mapping_origin: RepresentationItem,
        mapped_representation: Representation,
    }
    impl RepresentationMap {
        pub fn new(
            mapping_origin: RepresentationItem,
            mapped_representation: Representation,
        ) -> Self {
            Self {
                mapping_origin,
                mapped_representation,
            }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct SecurityClassification {
        name: Label,
        purpose: Text,
        security_level: SecurityClassificationLevel,
    }
    impl SecurityClassification {
        pub fn new(
            name: Label,
            purpose: Text,
            security_level: SecurityClassificationLevel,
        ) -> Self {
            Self {
                name,
                purpose,
                security_level,
            }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct SecurityClassificationAssignment {
        assigned_security_classification: SecurityClassification,
    }
    impl SecurityClassificationAssignment {
        pub fn new(assigned_security_classification: SecurityClassification) -> Self {
            Self {
                assigned_security_classification,
            }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct SecurityClassificationLevel {
        name: Label,
    }
    impl SecurityClassificationLevel {
        pub fn new(name: Label) -> Self {
            Self { name }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct ShapeDefinitionRepresentation {}
    impl ShapeDefinitionRepresentation {
        pub fn new() -> Self {
            Self {}
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct ShapeRepresentation {}
    impl ShapeRepresentation {
        pub fn new() -> Self {
            Self {}
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct SiUnit {
        prefix: Option<SiPrefix>,
        name: SiUnitName,
    }
    impl SiUnit {
        pub fn new(prefix: Option<SiPrefix>, name: SiUnitName) -> Self {
            Self { prefix, name }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct StructuredDimensionCallout {}
    impl StructuredDimensionCallout {
        pub fn new() -> Self {
            Self {}
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct StyledItem {
        styles: PresentationStyleAssignment,
        item: RepresentationItem,
    }
    impl StyledItem {
        pub fn new(styles: PresentationStyleAssignment, item: RepresentationItem) -> Self {
            Self { styles, item }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct SymbolColour {
        colour_of_symbol: Colour,
    }
    impl SymbolColour {
        pub fn new(colour_of_symbol: Colour) -> Self {
            Self { colour_of_symbol }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct SymbolRepresentation {}
    impl SymbolRepresentation {
        pub fn new() -> Self {
            Self {}
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct SymbolRepresentationMap {}
    impl SymbolRepresentationMap {
        pub fn new() -> Self {
            Self {}
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct SymbolStyle {
        name: Label,
        style_of_symbol: SymbolStyleSelect,
    }
    impl SymbolStyle {
        pub fn new(name: Label, style_of_symbol: SymbolStyleSelect) -> Self {
            Self {
                name,
                style_of_symbol,
            }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct SymbolTarget {
        placement: Axis2Placement,
        x_scale: PositiveRatioMeasure,
        y_scale: PositiveRatioMeasure,
    }
    impl SymbolTarget {
        pub fn new(
            placement: Axis2Placement,
            x_scale: PositiveRatioMeasure,
            y_scale: PositiveRatioMeasure,
        ) -> Self {
            Self {
                placement,
                x_scale,
                y_scale,
            }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct TerminatorSymbol {
        annotated_curve: AnnotationCurveOccurrence,
    }
    impl TerminatorSymbol {
        pub fn new(annotated_curve: AnnotationCurveOccurrence) -> Self {
            Self { annotated_curve }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct TextLiteral {
        literal: PresentableText,
        placement: Axis2Placement,
        alignment: TextAlignment,
        path: TextPath,
        font: FontSelect,
    }
    impl TextLiteral {
        pub fn new(
            literal: PresentableText,
            placement: Axis2Placement,
            alignment: TextAlignment,
            path: TextPath,
            font: FontSelect,
        ) -> Self {
            Self {
                literal,
                placement,
                alignment,
                path,
                font,
            }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct TextLiteralWithAssociatedCurves {
        associated_curves: Curve,
    }
    impl TextLiteralWithAssociatedCurves {
        pub fn new(associated_curves: Curve) -> Self {
            Self { associated_curves }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct TextLiteralWithBlankingBox {
        blanking: PlanarBox,
    }
    impl TextLiteralWithBlankingBox {
        pub fn new(blanking: PlanarBox) -> Self {
            Self { blanking }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct TextLiteralWithDelineation {
        delineation: TextDelineation,
    }
    impl TextLiteralWithDelineation {
        pub fn new(delineation: TextDelineation) -> Self {
            Self { delineation }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct TextLiteralWithExtent {
        extent: PlanarExtent,
    }
    impl TextLiteralWithExtent {
        pub fn new(extent: PlanarExtent) -> Self {
            Self { extent }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct TextStyle {
        name: Label,
        character_appearance: CharacterStyleSelect,
    }
    impl TextStyle {
        pub fn new(name: Label, character_appearance: CharacterStyleSelect) -> Self {
            Self {
                name,
                character_appearance,
            }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct TextStyleForDefinedFont {
        text_colour: Colour,
    }
    impl TextStyleForDefinedFont {
        pub fn new(text_colour: Colour) -> Self {
            Self { text_colour }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct TextStyleWithBoxCharacteristics {
        characteristics: BoxCharacteristicSelect,
    }
    impl TextStyleWithBoxCharacteristics {
        pub fn new(characteristics: BoxCharacteristicSelect) -> Self {
            Self { characteristics }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct TextStyleWithMirror {
        mirror_placement: Axis2Placement,
    }
    impl TextStyleWithMirror {
        pub fn new(mirror_placement: Axis2Placement) -> Self {
            Self { mirror_placement }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct TrimmedCurve {
        basis_curve: Curve,
        trim_1: TrimmingSelect,
        trim_2: TrimmingSelect,
        sense_agreement: bool,
        master_representation: TrimmingPreference,
    }
    impl TrimmedCurve {
        pub fn new(
            basis_curve: Curve,
            trim_1: TrimmingSelect,
            trim_2: TrimmingSelect,
            sense_agreement: bool,
            master_representation: TrimmingPreference,
        ) -> Self {
            Self {
                basis_curve,
                trim_1,
                trim_2,
                sense_agreement,
                master_representation,
            }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct TwoDirectionRepeatFactor {
        second_repeat_factor: Vector,
    }
    impl TwoDirectionRepeatFactor {
        pub fn new(second_repeat_factor: Vector) -> Self {
            Self {
                second_repeat_factor,
            }
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct UniformCurve {}
    impl UniformCurve {
        pub fn new() -> Self {
            Self {}
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct Vector {
        orientation: Direction,
        magnitude: LengthMeasure,
    }
    impl Vector {
        pub fn new(orientation: Direction, magnitude: LengthMeasure) -> Self {
            Self {
                orientation,
                magnitude,
            }
        }
    }
}

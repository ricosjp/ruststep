#![allow(dead_code)]
pub mod explicit_draughting {
    use crate::{primitive::*, tables::*};
    use std::collections::HashMap;
    #[derive(Debug, Clone, PartialEq, Default)]
    pub struct Table {
        address: HashMap<u64, AddressHolder>,
        angular_dimension: HashMap<u64, AngularDimensionHolder>,
        annotation_curve_occurrence: HashMap<u64, AnnotationCurveOccurrenceHolder>,
        annotation_fill_area: HashMap<u64, AnnotationFillAreaHolder>,
        annotation_fill_area_occurrence: HashMap<u64, AnnotationFillAreaOccurrenceHolder>,
        annotation_occurrence: HashMap<u64, AnnotationOccurrenceHolder>,
        annotation_subfigure_occurrence: HashMap<u64, AnnotationSubfigureOccurrenceHolder>,
        annotation_symbol: HashMap<u64, AnnotationSymbolHolder>,
        annotation_symbol_occurrence: HashMap<u64, AnnotationSymbolOccurrenceHolder>,
        annotation_text: HashMap<u64, AnnotationTextHolder>,
        annotation_text_occurrence: HashMap<u64, AnnotationTextOccurrenceHolder>,
        application_context: HashMap<u64, ApplicationContextHolder>,
        application_context_element: HashMap<u64, ApplicationContextElementHolder>,
        application_protocol_definition: HashMap<u64, ApplicationProtocolDefinitionHolder>,
        approval: HashMap<u64, ApprovalHolder>,
        approval_assignment: HashMap<u64, ApprovalAssignmentHolder>,
        approval_date_time: HashMap<u64, ApprovalDateTimeHolder>,
        approval_person_organization: HashMap<u64, ApprovalPersonOrganizationHolder>,
        approval_role: HashMap<u64, ApprovalRoleHolder>,
        approval_status: HashMap<u64, ApprovalStatusHolder>,
        area_in_set: HashMap<u64, AreaInSetHolder>,
        axis_2_placement_2d: HashMap<u64, Axis2Placement2DHolder>,
        b_spline_curve: HashMap<u64, BSplineCurveHolder>,
        b_spline_curve_with_knots: HashMap<u64, BSplineCurveWithKnotsHolder>,
        bezier_curve: HashMap<u64, BezierCurveHolder>,
        bounded_curve: HashMap<u64, BoundedCurveHolder>,
        calendar_date: HashMap<u64, CalendarDateHolder>,
        camera_image: HashMap<u64, CameraImageHolder>,
        camera_image_2d_with_scale: HashMap<u64, CameraImage2DWithScaleHolder>,
        camera_model: HashMap<u64, CameraModelHolder>,
        camera_model_d2: HashMap<u64, CameraModelD2Holder>,
        camera_usage: HashMap<u64, CameraUsageHolder>,
        cartesian_point: HashMap<u64, CartesianPointHolder>,
        circle: HashMap<u64, CircleHolder>,
        colour: HashMap<u64, ColourHolder>,
        colour_rgb: HashMap<u64, ColourRgbHolder>,
        colour_specification: HashMap<u64, ColourSpecificationHolder>,
        composite_curve: HashMap<u64, CompositeCurveHolder>,
        composite_curve_segment: HashMap<u64, CompositeCurveSegmentHolder>,
        composite_text: HashMap<u64, CompositeTextHolder>,
        composite_text_with_associated_curves:
            HashMap<u64, CompositeTextWithAssociatedCurvesHolder>,
        composite_text_with_blanking_box: HashMap<u64, CompositeTextWithBlankingBoxHolder>,
        composite_text_with_extent: HashMap<u64, CompositeTextWithExtentHolder>,
        conic: HashMap<u64, ConicHolder>,
        context_dependent_invisibility: HashMap<u64, ContextDependentInvisibilityHolder>,
        contract: HashMap<u64, ContractHolder>,
        contract_assignment: HashMap<u64, ContractAssignmentHolder>,
        contract_type: HashMap<u64, ContractTypeHolder>,
        conversion_based_unit: HashMap<u64, ConversionBasedUnitHolder>,
        curve: HashMap<u64, CurveHolder>,
        curve_dimension: HashMap<u64, CurveDimensionHolder>,
        curve_style: HashMap<u64, CurveStyleHolder>,
        curve_style_font: HashMap<u64, CurveStyleFontHolder>,
        curve_style_font_pattern: HashMap<u64, CurveStyleFontPatternHolder>,
        date: HashMap<u64, DateHolder>,
        datum_feature_callout: HashMap<u64, DatumFeatureCalloutHolder>,
        datum_target_callout: HashMap<u64, DatumTargetCalloutHolder>,
        defined_symbol: HashMap<u64, DefinedSymbolHolder>,
        diameter_dimension: HashMap<u64, DiameterDimensionHolder>,
        dimension_callout_component_relationship:
            HashMap<u64, DimensionCalloutComponentRelationshipHolder>,
        dimension_callout_relationship: HashMap<u64, DimensionCalloutRelationshipHolder>,
        dimension_curve: HashMap<u64, DimensionCurveHolder>,
        dimension_curve_directed_callout: HashMap<u64, DimensionCurveDirectedCalloutHolder>,
        dimension_curve_terminator: HashMap<u64, DimensionCurveTerminatorHolder>,
        dimension_pair: HashMap<u64, DimensionPairHolder>,
        dimensional_exponents: HashMap<u64, DimensionalExponentsHolder>,
        direction: HashMap<u64, DirectionHolder>,
        document: HashMap<u64, DocumentHolder>,
        document_reference: HashMap<u64, DocumentReferenceHolder>,
        document_type: HashMap<u64, DocumentTypeHolder>,
        draughting_annotation_occurrence: HashMap<u64, DraughtingAnnotationOccurrenceHolder>,
        draughting_approval_assignment: HashMap<u64, DraughtingApprovalAssignmentHolder>,
        draughting_callout: HashMap<u64, DraughtingCalloutHolder>,
        draughting_callout_relationship: HashMap<u64, DraughtingCalloutRelationshipHolder>,
        draughting_contract_assignment: HashMap<u64, DraughtingContractAssignmentHolder>,
        draughting_drawing_revision: HashMap<u64, DraughtingDrawingRevisionHolder>,
        draughting_elements: HashMap<u64, DraughtingElementsHolder>,
        draughting_group_assignment: HashMap<u64, DraughtingGroupAssignmentHolder>,
        draughting_model: HashMap<u64, DraughtingModelHolder>,
        draughting_organization_assignment: HashMap<u64, DraughtingOrganizationAssignmentHolder>,
        draughting_person_and_organization_assignment:
            HashMap<u64, DraughtingPersonAndOrganizationAssignmentHolder>,
        draughting_person_assignment: HashMap<u64, DraughtingPersonAssignmentHolder>,
        draughting_pre_defined_colour: HashMap<u64, DraughtingPreDefinedColourHolder>,
        draughting_pre_defined_curve_font: HashMap<u64, DraughtingPreDefinedCurveFontHolder>,
        draughting_pre_defined_text_font: HashMap<u64, DraughtingPreDefinedTextFontHolder>,
        draughting_presented_item: HashMap<u64, DraughtingPresentedItemHolder>,
        draughting_security_classification_assignment:
            HashMap<u64, DraughtingSecurityClassificationAssignmentHolder>,
        draughting_specification_reference: HashMap<u64, DraughtingSpecificationReferenceHolder>,
        draughting_subfigure_representation: HashMap<u64, DraughtingSubfigureRepresentationHolder>,
        draughting_symbol_representation: HashMap<u64, DraughtingSymbolRepresentationHolder>,
        draughting_text_literal_with_delineation:
            HashMap<u64, DraughtingTextLiteralWithDelineationHolder>,
        draughting_title: HashMap<u64, DraughtingTitleHolder>,
        drawing_definition: HashMap<u64, DrawingDefinitionHolder>,
        drawing_revision: HashMap<u64, DrawingRevisionHolder>,
        drawing_sheet_layout: HashMap<u64, DrawingSheetLayoutHolder>,
        drawing_sheet_revision: HashMap<u64, DrawingSheetRevisionHolder>,
        drawing_sheet_revision_usage: HashMap<u64, DrawingSheetRevisionUsageHolder>,
        ellipse: HashMap<u64, EllipseHolder>,
        external_source: HashMap<u64, ExternalSourceHolder>,
        externally_defined_curve_font: HashMap<u64, ExternallyDefinedCurveFontHolder>,
        externally_defined_hatch_style: HashMap<u64, ExternallyDefinedHatchStyleHolder>,
        externally_defined_item: HashMap<u64, ExternallyDefinedItemHolder>,
        externally_defined_symbol: HashMap<u64, ExternallyDefinedSymbolHolder>,
        externally_defined_text_font: HashMap<u64, ExternallyDefinedTextFontHolder>,
        externally_defined_tile_style: HashMap<u64, ExternallyDefinedTileStyleHolder>,
        fill_area_style: HashMap<u64, FillAreaStyleHolder>,
        fill_area_style_colour: HashMap<u64, FillAreaStyleColourHolder>,
        fill_area_style_hatching: HashMap<u64, FillAreaStyleHatchingHolder>,
        fill_area_style_tile_symbol_with_style:
            HashMap<u64, FillAreaStyleTileSymbolWithStyleHolder>,
        fill_area_style_tiles: HashMap<u64, FillAreaStyleTilesHolder>,
        geometric_curve_set: HashMap<u64, GeometricCurveSetHolder>,
        geometric_representation_context: HashMap<u64, GeometricRepresentationContextHolder>,
        geometric_representation_item: HashMap<u64, GeometricRepresentationItemHolder>,
        geometric_set: HashMap<u64, GeometricSetHolder>,
        geometrical_tolerance_callout: HashMap<u64, GeometricalToleranceCalloutHolder>,
        geometrically_bounded_2d_wireframe_representation:
            HashMap<u64, GeometricallyBounded2DWireframeRepresentationHolder>,
        global_unit_assigned_context: HashMap<u64, GlobalUnitAssignedContextHolder>,
        group: HashMap<u64, GroupHolder>,
        group_assignment: HashMap<u64, GroupAssignmentHolder>,
        group_relationship: HashMap<u64, GroupRelationshipHolder>,
        hyperbola: HashMap<u64, HyperbolaHolder>,
        invisibility: HashMap<u64, InvisibilityHolder>,
        leader_curve: HashMap<u64, LeaderCurveHolder>,
        leader_directed_callout: HashMap<u64, LeaderDirectedCalloutHolder>,
        leader_directed_dimension: HashMap<u64, LeaderDirectedDimensionHolder>,
        leader_terminator: HashMap<u64, LeaderTerminatorHolder>,
        length_measure_with_unit: HashMap<u64, LengthMeasureWithUnitHolder>,
        length_unit: HashMap<u64, LengthUnitHolder>,
        line: HashMap<u64, LineHolder>,
        linear_dimension: HashMap<u64, LinearDimensionHolder>,
        mapped_item: HashMap<u64, MappedItemHolder>,
        measure_with_unit: HashMap<u64, MeasureWithUnitHolder>,
        named_unit: HashMap<u64, NamedUnitHolder>,
        offset_curve_2d: HashMap<u64, OffsetCurve2DHolder>,
        one_direction_repeat_factor: HashMap<u64, OneDirectionRepeatFactorHolder>,
        ordinate_dimension: HashMap<u64, OrdinateDimensionHolder>,
        organization: HashMap<u64, OrganizationHolder>,
        organization_assignment: HashMap<u64, OrganizationAssignmentHolder>,
        organization_role: HashMap<u64, OrganizationRoleHolder>,
        organizational_address: HashMap<u64, OrganizationalAddressHolder>,
        parabola: HashMap<u64, ParabolaHolder>,
        person: HashMap<u64, PersonHolder>,
        person_and_organization: HashMap<u64, PersonAndOrganizationHolder>,
        person_and_organization_assignment: HashMap<u64, PersonAndOrganizationAssignmentHolder>,
        person_and_organization_role: HashMap<u64, PersonAndOrganizationRoleHolder>,
        person_assignment: HashMap<u64, PersonAssignmentHolder>,
        person_role: HashMap<u64, PersonRoleHolder>,
        personal_address: HashMap<u64, PersonalAddressHolder>,
        placement: HashMap<u64, PlacementHolder>,
        planar_box: HashMap<u64, PlanarBoxHolder>,
        planar_extent: HashMap<u64, PlanarExtentHolder>,
        plane_angle_measure_with_unit: HashMap<u64, PlaneAngleMeasureWithUnitHolder>,
        plane_angle_unit: HashMap<u64, PlaneAngleUnitHolder>,
        point: HashMap<u64, PointHolder>,
        point_on_curve: HashMap<u64, PointOnCurveHolder>,
        polyline: HashMap<u64, PolylineHolder>,
        pre_defined_colour: HashMap<u64, PreDefinedColourHolder>,
        pre_defined_curve_font: HashMap<u64, PreDefinedCurveFontHolder>,
        pre_defined_dimension_symbol: HashMap<u64, PreDefinedDimensionSymbolHolder>,
        pre_defined_geometrical_tolerance_symbol:
            HashMap<u64, PreDefinedGeometricalToleranceSymbolHolder>,
        pre_defined_item: HashMap<u64, PreDefinedItemHolder>,
        pre_defined_point_marker_symbol: HashMap<u64, PreDefinedPointMarkerSymbolHolder>,
        pre_defined_symbol: HashMap<u64, PreDefinedSymbolHolder>,
        pre_defined_terminator_symbol: HashMap<u64, PreDefinedTerminatorSymbolHolder>,
        pre_defined_text_font: HashMap<u64, PreDefinedTextFontHolder>,
        presentation_area: HashMap<u64, PresentationAreaHolder>,
        presentation_layer_assignment: HashMap<u64, PresentationLayerAssignmentHolder>,
        presentation_layer_usage: HashMap<u64, PresentationLayerUsageHolder>,
        presentation_representation: HashMap<u64, PresentationRepresentationHolder>,
        presentation_set: HashMap<u64, PresentationSetHolder>,
        presentation_size: HashMap<u64, PresentationSizeHolder>,
        presentation_style_assignment: HashMap<u64, PresentationStyleAssignmentHolder>,
        presentation_style_by_context: HashMap<u64, PresentationStyleByContextHolder>,
        presentation_view: HashMap<u64, PresentationViewHolder>,
        presented_item: HashMap<u64, PresentedItemHolder>,
        presented_item_representation: HashMap<u64, PresentedItemRepresentationHolder>,
        product: HashMap<u64, ProductHolder>,
        product_context: HashMap<u64, ProductContextHolder>,
        product_definition: HashMap<u64, ProductDefinitionHolder>,
        product_definition_context: HashMap<u64, ProductDefinitionContextHolder>,
        product_definition_formation: HashMap<u64, ProductDefinitionFormationHolder>,
        product_definition_shape: HashMap<u64, ProductDefinitionShapeHolder>,
        projection_curve: HashMap<u64, ProjectionCurveHolder>,
        projection_directed_callout: HashMap<u64, ProjectionDirectedCalloutHolder>,
        property_definition: HashMap<u64, PropertyDefinitionHolder>,
        property_definition_representation: HashMap<u64, PropertyDefinitionRepresentationHolder>,
        quasi_uniform_curve: HashMap<u64, QuasiUniformCurveHolder>,
        radius_dimension: HashMap<u64, RadiusDimensionHolder>,
        rational_b_spline_curve: HashMap<u64, RationalBSplineCurveHolder>,
        representation: HashMap<u64, RepresentationHolder>,
        representation_context: HashMap<u64, RepresentationContextHolder>,
        representation_item: HashMap<u64, RepresentationItemHolder>,
        representation_map: HashMap<u64, RepresentationMapHolder>,
        security_classification: HashMap<u64, SecurityClassificationHolder>,
        security_classification_assignment: HashMap<u64, SecurityClassificationAssignmentHolder>,
        security_classification_level: HashMap<u64, SecurityClassificationLevelHolder>,
        shape_definition_representation: HashMap<u64, ShapeDefinitionRepresentationHolder>,
        shape_representation: HashMap<u64, ShapeRepresentationHolder>,
        si_unit: HashMap<u64, SiUnitHolder>,
        structured_dimension_callout: HashMap<u64, StructuredDimensionCalloutHolder>,
        styled_item: HashMap<u64, StyledItemHolder>,
        symbol_colour: HashMap<u64, SymbolColourHolder>,
        symbol_representation: HashMap<u64, SymbolRepresentationHolder>,
        symbol_representation_map: HashMap<u64, SymbolRepresentationMapHolder>,
        symbol_style: HashMap<u64, SymbolStyleHolder>,
        symbol_target: HashMap<u64, SymbolTargetHolder>,
        terminator_symbol: HashMap<u64, TerminatorSymbolHolder>,
        text_literal: HashMap<u64, TextLiteralHolder>,
        text_literal_with_associated_curves: HashMap<u64, TextLiteralWithAssociatedCurvesHolder>,
        text_literal_with_blanking_box: HashMap<u64, TextLiteralWithBlankingBoxHolder>,
        text_literal_with_delineation: HashMap<u64, TextLiteralWithDelineationHolder>,
        text_literal_with_extent: HashMap<u64, TextLiteralWithExtentHolder>,
        text_style: HashMap<u64, TextStyleHolder>,
        text_style_for_defined_font: HashMap<u64, TextStyleForDefinedFontHolder>,
        text_style_with_box_characteristics: HashMap<u64, TextStyleWithBoxCharacteristicsHolder>,
        text_style_with_mirror: HashMap<u64, TextStyleWithMirrorHolder>,
        trimmed_curve: HashMap<u64, TrimmedCurveHolder>,
        two_direction_repeat_factor: HashMap<u64, TwoDirectionRepeatFactorHolder>,
        uniform_curve: HashMap<u64, UniformCurveHolder>,
        vector: HashMap<u64, VectorHolder>,
    }
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
    #[derive(Clone, Debug, PartialEq)]
    pub struct AddressHolder {
        pub internal_location: Option<PlaceHolder<Label>>,
        pub street_number: Option<PlaceHolder<Label>>,
        pub street: Option<PlaceHolder<Label>>,
        pub postal_box: Option<PlaceHolder<Label>>,
        pub town: Option<PlaceHolder<Label>>,
        pub region: Option<PlaceHolder<Label>>,
        pub postal_code: Option<PlaceHolder<Label>>,
        pub country: Option<PlaceHolder<Label>>,
        pub facsimile_number: Option<PlaceHolder<Label>>,
        pub telephone_number: Option<PlaceHolder<Label>>,
        pub electronic_mail_address: Option<PlaceHolder<Label>>,
        pub telex_number: Option<PlaceHolder<Label>>,
    }
    impl ::std::ops::Deref for AngularDimension {
        type Target = DimensionCurveDirectedCallout;
        fn deref(&self) -> &Self::Target {
            &self.dimension_curve_directed_callout
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct AngularDimension {
        pub dimension_curve_directed_callout: DimensionCurveDirectedCallout,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct AngularDimensionHolder {
        pub dimension_curve_directed_callout: PlaceHolder<DimensionCurveDirectedCallout>,
    }
    impl ::std::ops::Deref for AnnotationCurveOccurrence {
        type Target = AnnotationOccurrence;
        fn deref(&self) -> &Self::Target {
            &self.annotation_occurrence
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct AnnotationCurveOccurrence {
        pub annotation_occurrence: AnnotationOccurrence,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct AnnotationCurveOccurrenceHolder {
        pub annotation_occurrence: PlaceHolder<AnnotationOccurrence>,
    }
    impl ::std::ops::Deref for AnnotationFillArea {
        type Target = GeometricRepresentationItem;
        fn deref(&self) -> &Self::Target {
            &self.geometric_representation_item
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct AnnotationFillArea {
        pub boundaries: Vec<Curve>,
        pub geometric_representation_item: GeometricRepresentationItem,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct AnnotationFillAreaHolder {
        pub boundaries: PlaceHolder<Vec<Curve>>,
        pub geometric_representation_item: PlaceHolder<GeometricRepresentationItem>,
    }
    impl ::std::ops::Deref for AnnotationFillAreaOccurrence {
        type Target = AnnotationOccurrence;
        fn deref(&self) -> &Self::Target {
            &self.annotation_occurrence
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct AnnotationFillAreaOccurrence {
        pub fill_style_target: Point,
        pub annotation_occurrence: AnnotationOccurrence,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct AnnotationFillAreaOccurrenceHolder {
        pub fill_style_target: PlaceHolder<Point>,
        pub annotation_occurrence: PlaceHolder<AnnotationOccurrence>,
    }
    impl ::std::ops::Deref for AnnotationOccurrence {
        type Target = StyledItem;
        fn deref(&self) -> &Self::Target {
            &self.styled_item
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct AnnotationOccurrence {
        pub styled_item: StyledItem,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct AnnotationOccurrenceHolder {
        pub styled_item: PlaceHolder<StyledItem>,
    }
    impl ::std::ops::Deref for AnnotationSubfigureOccurrence {
        type Target = AnnotationSymbolOccurrence;
        fn deref(&self) -> &Self::Target {
            &self.annotation_symbol_occurrence
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct AnnotationSubfigureOccurrence {
        pub annotation_symbol_occurrence: AnnotationSymbolOccurrence,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct AnnotationSubfigureOccurrenceHolder {
        pub annotation_symbol_occurrence: PlaceHolder<AnnotationSymbolOccurrence>,
    }
    impl ::std::ops::Deref for AnnotationSymbol {
        type Target = MappedItem;
        fn deref(&self) -> &Self::Target {
            &self.mapped_item
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct AnnotationSymbol {
        pub mapped_item: MappedItem,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct AnnotationSymbolHolder {
        pub mapped_item: PlaceHolder<MappedItem>,
    }
    impl ::std::ops::Deref for AnnotationSymbolOccurrence {
        type Target = AnnotationOccurrence;
        fn deref(&self) -> &Self::Target {
            &self.annotation_occurrence
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct AnnotationSymbolOccurrence {
        pub annotation_occurrence: AnnotationOccurrence,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct AnnotationSymbolOccurrenceHolder {
        pub annotation_occurrence: PlaceHolder<AnnotationOccurrence>,
    }
    impl ::std::ops::Deref for AnnotationText {
        type Target = MappedItem;
        fn deref(&self) -> &Self::Target {
            &self.mapped_item
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct AnnotationText {
        pub mapped_item: MappedItem,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct AnnotationTextHolder {
        pub mapped_item: PlaceHolder<MappedItem>,
    }
    impl ::std::ops::Deref for AnnotationTextOccurrence {
        type Target = AnnotationOccurrence;
        fn deref(&self) -> &Self::Target {
            &self.annotation_occurrence
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct AnnotationTextOccurrence {
        pub annotation_occurrence: AnnotationOccurrence,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct AnnotationTextOccurrenceHolder {
        pub annotation_occurrence: PlaceHolder<AnnotationOccurrence>,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ApplicationContext {
        pub application: Text,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct ApplicationContextHolder {
        pub application: PlaceHolder<Text>,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ApplicationContextElement {
        pub name: Label,
        pub frame_of_reference: ApplicationContext,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct ApplicationContextElementHolder {
        pub name: PlaceHolder<Label>,
        pub frame_of_reference: PlaceHolder<ApplicationContext>,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ApplicationProtocolDefinition {
        pub status: Label,
        pub application_interpreted_model_schema_name: Label,
        pub application_protocol_year: YearNumber,
        pub application: ApplicationContext,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct ApplicationProtocolDefinitionHolder {
        pub status: PlaceHolder<Label>,
        pub application_interpreted_model_schema_name: PlaceHolder<Label>,
        pub application_protocol_year: PlaceHolder<YearNumber>,
        pub application: PlaceHolder<ApplicationContext>,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct Approval {
        pub status: ApprovalStatus,
        pub level: Label,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct ApprovalHolder {
        pub status: PlaceHolder<ApprovalStatus>,
        pub level: PlaceHolder<Label>,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ApprovalAssignment {
        pub assigned_approval: Approval,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct ApprovalAssignmentHolder {
        pub assigned_approval: PlaceHolder<Approval>,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ApprovalDateTime {
        pub date_time: DateTimeSelect,
        pub dated_approval: Approval,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct ApprovalDateTimeHolder {
        pub date_time: PlaceHolder<DateTimeSelect>,
        pub dated_approval: PlaceHolder<Approval>,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ApprovalPersonOrganization {
        pub person_organization: PersonOrganizationSelect,
        pub authorized_approval: Approval,
        pub role: ApprovalRole,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct ApprovalPersonOrganizationHolder {
        pub person_organization: PlaceHolder<PersonOrganizationSelect>,
        pub authorized_approval: PlaceHolder<Approval>,
        pub role: PlaceHolder<ApprovalRole>,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ApprovalRole {
        pub role: Label,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct ApprovalRoleHolder {
        pub role: PlaceHolder<Label>,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ApprovalStatus {
        pub name: Label,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct ApprovalStatusHolder {
        pub name: PlaceHolder<Label>,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct AreaInSet {
        pub area: PresentationArea,
        pub in_set: PresentationSet,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct AreaInSetHolder {
        pub area: PlaceHolder<PresentationArea>,
        pub in_set: PlaceHolder<PresentationSet>,
    }
    impl ::std::ops::Deref for Axis2Placement2D {
        type Target = Placement;
        fn deref(&self) -> &Self::Target {
            &self.placement
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct Axis2Placement2D {
        pub ref_direction: Option<Direction>,
        pub placement: Placement,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct Axis2Placement2DHolder {
        pub ref_direction: Option<PlaceHolder<Direction>>,
        pub placement: PlaceHolder<Placement>,
    }
    impl ::std::ops::Deref for BSplineCurve {
        type Target = BoundedCurve;
        fn deref(&self) -> &Self::Target {
            &self.bounded_curve
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct BSplineCurve {
        pub degree: i64,
        pub control_points_list: Vec<CartesianPoint>,
        pub curve_form: BSplineCurveForm,
        pub closed_curve: Logical,
        pub self_intersect: Logical,
        pub bounded_curve: BoundedCurve,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct BSplineCurveHolder {
        pub degree: i64,
        pub control_points_list: PlaceHolder<Vec<CartesianPoint>>,
        pub curve_form: PlaceHolder<BSplineCurveForm>,
        pub closed_curve: Logical,
        pub self_intersect: Logical,
        pub bounded_curve: PlaceHolder<BoundedCurve>,
    }
    impl ::std::ops::Deref for BSplineCurveWithKnots {
        type Target = BSplineCurve;
        fn deref(&self) -> &Self::Target {
            &self.b_spline_curve
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct BSplineCurveWithKnots {
        pub knot_multiplicities: Vec<i64>,
        pub knots: Vec<ParameterValue>,
        pub knot_spec: KnotType,
        pub b_spline_curve: BSplineCurve,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct BSplineCurveWithKnotsHolder {
        pub knot_multiplicities: PlaceHolder<Vec<i64>>,
        pub knots: PlaceHolder<Vec<ParameterValue>>,
        pub knot_spec: PlaceHolder<KnotType>,
        pub b_spline_curve: PlaceHolder<BSplineCurve>,
    }
    impl ::std::ops::Deref for BezierCurve {
        type Target = BSplineCurve;
        fn deref(&self) -> &Self::Target {
            &self.b_spline_curve
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct BezierCurve {
        pub b_spline_curve: BSplineCurve,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct BezierCurveHolder {
        pub b_spline_curve: PlaceHolder<BSplineCurve>,
    }
    impl ::std::ops::Deref for BoundedCurve {
        type Target = Curve;
        fn deref(&self) -> &Self::Target {
            &self.curve
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct BoundedCurve {
        pub curve: Curve,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct BoundedCurveHolder {
        pub curve: PlaceHolder<Curve>,
    }
    impl ::std::ops::Deref for CalendarDate {
        type Target = Date;
        fn deref(&self) -> &Self::Target {
            &self.date
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct CalendarDate {
        pub day_component: DayInMonthNumber,
        pub month_component: MonthInYearNumber,
        pub date: Date,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct CalendarDateHolder {
        pub day_component: PlaceHolder<DayInMonthNumber>,
        pub month_component: PlaceHolder<MonthInYearNumber>,
        pub date: PlaceHolder<Date>,
    }
    impl ::std::ops::Deref for CameraImage {
        type Target = MappedItem;
        fn deref(&self) -> &Self::Target {
            &self.mapped_item
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct CameraImage {
        pub mapped_item: MappedItem,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct CameraImageHolder {
        pub mapped_item: PlaceHolder<MappedItem>,
    }
    impl ::std::ops::Deref for CameraImage2DWithScale {
        type Target = CameraImage;
        fn deref(&self) -> &Self::Target {
            &self.camera_image
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct CameraImage2DWithScale {
        pub camera_image: CameraImage,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct CameraImage2DWithScaleHolder {
        pub camera_image: PlaceHolder<CameraImage>,
    }
    impl ::std::ops::Deref for CameraModel {
        type Target = GeometricRepresentationItem;
        fn deref(&self) -> &Self::Target {
            &self.geometric_representation_item
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct CameraModel {
        pub geometric_representation_item: GeometricRepresentationItem,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct CameraModelHolder {
        pub geometric_representation_item: PlaceHolder<GeometricRepresentationItem>,
    }
    impl ::std::ops::Deref for CameraModelD2 {
        type Target = CameraModel;
        fn deref(&self) -> &Self::Target {
            &self.camera_model
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct CameraModelD2 {
        pub view_window: PlanarBox,
        pub view_window_clipping: bool,
        pub camera_model: CameraModel,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct CameraModelD2Holder {
        pub view_window: PlaceHolder<PlanarBox>,
        pub view_window_clipping: bool,
        pub camera_model: PlaceHolder<CameraModel>,
    }
    impl ::std::ops::Deref for CameraUsage {
        type Target = RepresentationMap;
        fn deref(&self) -> &Self::Target {
            &self.representation_map
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct CameraUsage {
        pub representation_map: RepresentationMap,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct CameraUsageHolder {
        pub representation_map: PlaceHolder<RepresentationMap>,
    }
    impl ::std::ops::Deref for CartesianPoint {
        type Target = Point;
        fn deref(&self) -> &Self::Target {
            &self.point
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct CartesianPoint {
        pub coordinates: Vec<LengthMeasure>,
        pub point: Point,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct CartesianPointHolder {
        pub coordinates: PlaceHolder<Vec<LengthMeasure>>,
        pub point: PlaceHolder<Point>,
    }
    impl ::std::ops::Deref for Circle {
        type Target = Conic;
        fn deref(&self) -> &Self::Target {
            &self.conic
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct Circle {
        pub radius: PositiveLengthMeasure,
        pub conic: Conic,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct CircleHolder {
        pub radius: PlaceHolder<PositiveLengthMeasure>,
        pub conic: PlaceHolder<Conic>,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct Colour {}
    #[derive(Clone, Debug, PartialEq)]
    pub struct ColourHolder {}
    impl ::std::ops::Deref for ColourRgb {
        type Target = ColourSpecification;
        fn deref(&self) -> &Self::Target {
            &self.colour_specification
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ColourRgb {
        pub red: f64,
        pub green: f64,
        pub blue: f64,
        pub colour_specification: ColourSpecification,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct ColourRgbHolder {
        pub red: f64,
        pub green: f64,
        pub blue: f64,
        pub colour_specification: PlaceHolder<ColourSpecification>,
    }
    impl ::std::ops::Deref for ColourSpecification {
        type Target = Colour;
        fn deref(&self) -> &Self::Target {
            &self.colour
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ColourSpecification {
        pub name: Colour,
        pub colour: Colour,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct ColourSpecificationHolder {
        pub name: PlaceHolder<Colour>,
        pub colour: PlaceHolder<Colour>,
    }
    impl ::std::ops::Deref for CompositeCurve {
        type Target = BoundedCurve;
        fn deref(&self) -> &Self::Target {
            &self.bounded_curve
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct CompositeCurve {
        pub segments: Vec<CompositeCurveSegment>,
        pub self_intersect: Logical,
        pub bounded_curve: BoundedCurve,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct CompositeCurveHolder {
        pub segments: PlaceHolder<Vec<CompositeCurveSegment>>,
        pub self_intersect: Logical,
        pub bounded_curve: PlaceHolder<BoundedCurve>,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct CompositeCurveSegment {
        pub transition: TransitionCode,
        pub same_sense: bool,
        pub parent_curve: Curve,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct CompositeCurveSegmentHolder {
        pub transition: PlaceHolder<TransitionCode>,
        pub same_sense: bool,
        pub parent_curve: PlaceHolder<Curve>,
    }
    impl ::std::ops::Deref for CompositeText {
        type Target = GeometricRepresentationItem;
        fn deref(&self) -> &Self::Target {
            &self.geometric_representation_item
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct CompositeText {
        pub collected_text: Vec<TextOrCharacter>,
        pub geometric_representation_item: GeometricRepresentationItem,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct CompositeTextHolder {
        pub collected_text: PlaceHolder<Vec<TextOrCharacter>>,
        pub geometric_representation_item: PlaceHolder<GeometricRepresentationItem>,
    }
    impl ::std::ops::Deref for CompositeTextWithAssociatedCurves {
        type Target = CompositeText;
        fn deref(&self) -> &Self::Target {
            &self.composite_text
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct CompositeTextWithAssociatedCurves {
        pub associated_curves: Vec<Curve>,
        pub composite_text: CompositeText,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct CompositeTextWithAssociatedCurvesHolder {
        pub associated_curves: PlaceHolder<Vec<Curve>>,
        pub composite_text: PlaceHolder<CompositeText>,
    }
    impl ::std::ops::Deref for CompositeTextWithBlankingBox {
        type Target = CompositeText;
        fn deref(&self) -> &Self::Target {
            &self.composite_text
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct CompositeTextWithBlankingBox {
        pub blanking: PlanarBox,
        pub composite_text: CompositeText,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct CompositeTextWithBlankingBoxHolder {
        pub blanking: PlaceHolder<PlanarBox>,
        pub composite_text: PlaceHolder<CompositeText>,
    }
    impl ::std::ops::Deref for CompositeTextWithExtent {
        type Target = CompositeText;
        fn deref(&self) -> &Self::Target {
            &self.composite_text
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct CompositeTextWithExtent {
        pub extent: PlanarExtent,
        pub composite_text: CompositeText,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct CompositeTextWithExtentHolder {
        pub extent: PlaceHolder<PlanarExtent>,
        pub composite_text: PlaceHolder<CompositeText>,
    }
    impl ::std::ops::Deref for Conic {
        type Target = Curve;
        fn deref(&self) -> &Self::Target {
            &self.curve
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct Conic {
        pub position: Axis2Placement,
        pub curve: Curve,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct ConicHolder {
        pub position: PlaceHolder<Axis2Placement>,
        pub curve: PlaceHolder<Curve>,
    }
    impl ::std::ops::Deref for ContextDependentInvisibility {
        type Target = Invisibility;
        fn deref(&self) -> &Self::Target {
            &self.invisibility
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ContextDependentInvisibility {
        pub presentation_context: InvisibilityContext,
        pub invisibility: Invisibility,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct ContextDependentInvisibilityHolder {
        pub presentation_context: PlaceHolder<InvisibilityContext>,
        pub invisibility: PlaceHolder<Invisibility>,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct Contract {
        pub name: Label,
        pub purpose: Text,
        pub kind: ContractType,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct ContractHolder {
        pub name: PlaceHolder<Label>,
        pub purpose: PlaceHolder<Text>,
        pub kind: PlaceHolder<ContractType>,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ContractAssignment {
        pub assigned_contract: Contract,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct ContractAssignmentHolder {
        pub assigned_contract: PlaceHolder<Contract>,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ContractType {
        pub description: Label,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct ContractTypeHolder {
        pub description: PlaceHolder<Label>,
    }
    impl ::std::ops::Deref for ConversionBasedUnit {
        type Target = NamedUnit;
        fn deref(&self) -> &Self::Target {
            &self.named_unit
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ConversionBasedUnit {
        pub name: Label,
        pub conversion_factor: MeasureWithUnit,
        pub named_unit: NamedUnit,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct ConversionBasedUnitHolder {
        pub name: PlaceHolder<Label>,
        pub conversion_factor: PlaceHolder<MeasureWithUnit>,
        pub named_unit: PlaceHolder<NamedUnit>,
    }
    impl ::std::ops::Deref for Curve {
        type Target = GeometricRepresentationItem;
        fn deref(&self) -> &Self::Target {
            &self.geometric_representation_item
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct Curve {
        pub geometric_representation_item: GeometricRepresentationItem,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct CurveHolder {
        pub geometric_representation_item: PlaceHolder<GeometricRepresentationItem>,
    }
    impl ::std::ops::Deref for CurveDimension {
        type Target = DimensionCurveDirectedCallout;
        fn deref(&self) -> &Self::Target {
            &self.dimension_curve_directed_callout
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct CurveDimension {
        pub dimension_curve_directed_callout: DimensionCurveDirectedCallout,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct CurveDimensionHolder {
        pub dimension_curve_directed_callout: PlaceHolder<DimensionCurveDirectedCallout>,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct CurveStyle {
        pub name: Label,
        pub curve_font: CurveFontOrScaledCurveFontSelect,
        pub curve_width: SizeSelect,
        pub curve_colour: Colour,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct CurveStyleHolder {
        pub name: PlaceHolder<Label>,
        pub curve_font: PlaceHolder<CurveFontOrScaledCurveFontSelect>,
        pub curve_width: PlaceHolder<SizeSelect>,
        pub curve_colour: PlaceHolder<Colour>,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct CurveStyleFont {
        pub name: Label,
        pub pattern_list: Vec<CurveStyleFontPattern>,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct CurveStyleFontHolder {
        pub name: PlaceHolder<Label>,
        pub pattern_list: PlaceHolder<Vec<CurveStyleFontPattern>>,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct CurveStyleFontPattern {
        pub visible_segment_length: PositiveLengthMeasure,
        pub invisible_segment_length: PositiveLengthMeasure,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct CurveStyleFontPatternHolder {
        pub visible_segment_length: PlaceHolder<PositiveLengthMeasure>,
        pub invisible_segment_length: PlaceHolder<PositiveLengthMeasure>,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct Date {
        pub year_component: YearNumber,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct DateHolder {
        pub year_component: PlaceHolder<YearNumber>,
    }
    impl ::std::ops::Deref for DatumFeatureCallout {
        type Target = DraughtingCallout;
        fn deref(&self) -> &Self::Target {
            &self.draughting_callout
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DatumFeatureCallout {
        pub draughting_callout: DraughtingCallout,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct DatumFeatureCalloutHolder {
        pub draughting_callout: PlaceHolder<DraughtingCallout>,
    }
    impl ::std::ops::Deref for DatumTargetCallout {
        type Target = DraughtingCallout;
        fn deref(&self) -> &Self::Target {
            &self.draughting_callout
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DatumTargetCallout {
        pub draughting_callout: DraughtingCallout,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct DatumTargetCalloutHolder {
        pub draughting_callout: PlaceHolder<DraughtingCallout>,
    }
    impl ::std::ops::Deref for DefinedSymbol {
        type Target = GeometricRepresentationItem;
        fn deref(&self) -> &Self::Target {
            &self.geometric_representation_item
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DefinedSymbol {
        pub definition: DefinedSymbolSelect,
        pub target: SymbolTarget,
        pub geometric_representation_item: GeometricRepresentationItem,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct DefinedSymbolHolder {
        pub definition: PlaceHolder<DefinedSymbolSelect>,
        pub target: PlaceHolder<SymbolTarget>,
        pub geometric_representation_item: PlaceHolder<GeometricRepresentationItem>,
    }
    impl ::std::ops::Deref for DiameterDimension {
        type Target = DimensionCurveDirectedCallout;
        fn deref(&self) -> &Self::Target {
            &self.dimension_curve_directed_callout
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DiameterDimension {
        pub dimension_curve_directed_callout: DimensionCurveDirectedCallout,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct DiameterDimensionHolder {
        pub dimension_curve_directed_callout: PlaceHolder<DimensionCurveDirectedCallout>,
    }
    impl ::std::ops::Deref for DimensionCalloutComponentRelationship {
        type Target = DraughtingCalloutRelationship;
        fn deref(&self) -> &Self::Target {
            &self.draughting_callout_relationship
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DimensionCalloutComponentRelationship {
        pub draughting_callout_relationship: DraughtingCalloutRelationship,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct DimensionCalloutComponentRelationshipHolder {
        pub draughting_callout_relationship: PlaceHolder<DraughtingCalloutRelationship>,
    }
    impl ::std::ops::Deref for DimensionCalloutRelationship {
        type Target = DraughtingCalloutRelationship;
        fn deref(&self) -> &Self::Target {
            &self.draughting_callout_relationship
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DimensionCalloutRelationship {
        pub draughting_callout_relationship: DraughtingCalloutRelationship,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct DimensionCalloutRelationshipHolder {
        pub draughting_callout_relationship: PlaceHolder<DraughtingCalloutRelationship>,
    }
    impl ::std::ops::Deref for DimensionCurve {
        type Target = AnnotationCurveOccurrence;
        fn deref(&self) -> &Self::Target {
            &self.annotation_curve_occurrence
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DimensionCurve {
        pub annotation_curve_occurrence: AnnotationCurveOccurrence,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct DimensionCurveHolder {
        pub annotation_curve_occurrence: PlaceHolder<AnnotationCurveOccurrence>,
    }
    impl ::std::ops::Deref for DimensionCurveDirectedCallout {
        type Target = DraughtingCallout;
        fn deref(&self) -> &Self::Target {
            &self.draughting_callout
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DimensionCurveDirectedCallout {
        pub draughting_callout: DraughtingCallout,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct DimensionCurveDirectedCalloutHolder {
        pub draughting_callout: PlaceHolder<DraughtingCallout>,
    }
    impl ::std::ops::Deref for DimensionCurveTerminator {
        type Target = TerminatorSymbol;
        fn deref(&self) -> &Self::Target {
            &self.terminator_symbol
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DimensionCurveTerminator {
        pub role: DimensionExtentUsage,
        pub terminator_symbol: TerminatorSymbol,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct DimensionCurveTerminatorHolder {
        pub role: PlaceHolder<DimensionExtentUsage>,
        pub terminator_symbol: PlaceHolder<TerminatorSymbol>,
    }
    impl ::std::ops::Deref for DimensionPair {
        type Target = DraughtingCalloutRelationship;
        fn deref(&self) -> &Self::Target {
            &self.draughting_callout_relationship
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DimensionPair {
        pub draughting_callout_relationship: DraughtingCalloutRelationship,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct DimensionPairHolder {
        pub draughting_callout_relationship: PlaceHolder<DraughtingCalloutRelationship>,
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
    #[derive(Clone, Debug, PartialEq)]
    pub struct DimensionalExponentsHolder {
        pub length_exponent: f64,
        pub mass_exponent: f64,
        pub time_exponent: f64,
        pub electric_current_exponent: f64,
        pub thermodynamic_temperature_exponent: f64,
        pub amount_of_substance_exponent: f64,
        pub luminous_intensity_exponent: f64,
    }
    impl ::std::ops::Deref for Direction {
        type Target = GeometricRepresentationItem;
        fn deref(&self) -> &Self::Target {
            &self.geometric_representation_item
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct Direction {
        pub direction_ratios: Vec<f64>,
        pub geometric_representation_item: GeometricRepresentationItem,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct DirectionHolder {
        pub direction_ratios: PlaceHolder<Vec<f64>>,
        pub geometric_representation_item: PlaceHolder<GeometricRepresentationItem>,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct Document {
        pub id: Identifier,
        pub name: Label,
        pub description: Text,
        pub kind: DocumentType,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct DocumentHolder {
        pub id: PlaceHolder<Identifier>,
        pub name: PlaceHolder<Label>,
        pub description: PlaceHolder<Text>,
        pub kind: PlaceHolder<DocumentType>,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DocumentReference {
        pub assigned_document: Document,
        pub source: Label,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct DocumentReferenceHolder {
        pub assigned_document: PlaceHolder<Document>,
        pub source: PlaceHolder<Label>,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DocumentType {
        pub product_data_type: Label,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct DocumentTypeHolder {
        pub product_data_type: PlaceHolder<Label>,
    }
    impl ::std::ops::Deref for DraughtingAnnotationOccurrence {
        type Target = AnnotationOccurrence;
        fn deref(&self) -> &Self::Target {
            &self.annotation_occurrence
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DraughtingAnnotationOccurrence {
        pub annotation_occurrence: AnnotationOccurrence,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct DraughtingAnnotationOccurrenceHolder {
        pub annotation_occurrence: PlaceHolder<AnnotationOccurrence>,
    }
    impl ::std::ops::Deref for DraughtingApprovalAssignment {
        type Target = ApprovalAssignment;
        fn deref(&self) -> &Self::Target {
            &self.approval_assignment
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DraughtingApprovalAssignment {
        pub approved_items: Vec<ApprovedItem>,
        pub approval_assignment: ApprovalAssignment,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct DraughtingApprovalAssignmentHolder {
        pub approved_items: PlaceHolder<Vec<ApprovedItem>>,
        pub approval_assignment: PlaceHolder<ApprovalAssignment>,
    }
    impl ::std::ops::Deref for DraughtingCallout {
        type Target = GeometricRepresentationItem;
        fn deref(&self) -> &Self::Target {
            &self.geometric_representation_item
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DraughtingCallout {
        pub contents: Vec<DraughtingCalloutElement>,
        pub geometric_representation_item: GeometricRepresentationItem,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct DraughtingCalloutHolder {
        pub contents: PlaceHolder<Vec<DraughtingCalloutElement>>,
        pub geometric_representation_item: PlaceHolder<GeometricRepresentationItem>,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DraughtingCalloutRelationship {
        pub name: Label,
        pub description: Text,
        pub relating_draughting_callout: DraughtingCallout,
        pub related_draughting_callout: DraughtingCallout,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct DraughtingCalloutRelationshipHolder {
        pub name: PlaceHolder<Label>,
        pub description: PlaceHolder<Text>,
        pub relating_draughting_callout: PlaceHolder<DraughtingCallout>,
        pub related_draughting_callout: PlaceHolder<DraughtingCallout>,
    }
    impl ::std::ops::Deref for DraughtingContractAssignment {
        type Target = ContractAssignment;
        fn deref(&self) -> &Self::Target {
            &self.contract_assignment
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DraughtingContractAssignment {
        pub items: Vec<ContractedItem>,
        pub contract_assignment: ContractAssignment,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct DraughtingContractAssignmentHolder {
        pub items: PlaceHolder<Vec<ContractedItem>>,
        pub contract_assignment: PlaceHolder<ContractAssignment>,
    }
    impl ::std::ops::Deref for DraughtingDrawingRevision {
        type Target = DrawingRevision;
        fn deref(&self) -> &Self::Target {
            &self.drawing_revision
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DraughtingDrawingRevision {
        pub drawing_revision: DrawingRevision,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct DraughtingDrawingRevisionHolder {
        pub drawing_revision: PlaceHolder<DrawingRevision>,
    }
    impl ::std::ops::Deref for DraughtingElements {
        type Target = DraughtingCallout;
        fn deref(&self) -> &Self::Target {
            &self.draughting_callout
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DraughtingElements {
        pub draughting_callout: DraughtingCallout,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct DraughtingElementsHolder {
        pub draughting_callout: PlaceHolder<DraughtingCallout>,
    }
    impl ::std::ops::Deref for DraughtingGroupAssignment {
        type Target = GroupAssignment;
        fn deref(&self) -> &Self::Target {
            &self.group_assignment
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DraughtingGroupAssignment {
        pub items: Vec<DraughtingGroupedItem>,
        pub group_assignment: GroupAssignment,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct DraughtingGroupAssignmentHolder {
        pub items: PlaceHolder<Vec<DraughtingGroupedItem>>,
        pub group_assignment: PlaceHolder<GroupAssignment>,
    }
    impl ::std::ops::Deref for DraughtingModel {
        type Target = Representation;
        fn deref(&self) -> &Self::Target {
            &self.representation
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DraughtingModel {
        pub representation: Representation,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct DraughtingModelHolder {
        pub representation: PlaceHolder<Representation>,
    }
    impl ::std::ops::Deref for DraughtingOrganizationAssignment {
        type Target = OrganizationAssignment;
        fn deref(&self) -> &Self::Target {
            &self.organization_assignment
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DraughtingOrganizationAssignment {
        pub assigned_items: Vec<DraughtingOrganizationItem>,
        pub organization_assignment: OrganizationAssignment,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct DraughtingOrganizationAssignmentHolder {
        pub assigned_items: PlaceHolder<Vec<DraughtingOrganizationItem>>,
        pub organization_assignment: PlaceHolder<OrganizationAssignment>,
    }
    impl ::std::ops::Deref for DraughtingPersonAndOrganizationAssignment {
        type Target = PersonAndOrganizationAssignment;
        fn deref(&self) -> &Self::Target {
            &self.person_and_organization_assignment
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DraughtingPersonAndOrganizationAssignment {
        pub assigned_items: Vec<DraughtingOrganizationItem>,
        pub person_and_organization_assignment: PersonAndOrganizationAssignment,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct DraughtingPersonAndOrganizationAssignmentHolder {
        pub assigned_items: PlaceHolder<Vec<DraughtingOrganizationItem>>,
        pub person_and_organization_assignment: PlaceHolder<PersonAndOrganizationAssignment>,
    }
    impl ::std::ops::Deref for DraughtingPersonAssignment {
        type Target = PersonAssignment;
        fn deref(&self) -> &Self::Target {
            &self.person_assignment
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DraughtingPersonAssignment {
        pub assigned_items: Vec<DraughtingOrganizationItem>,
        pub person_assignment: PersonAssignment,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct DraughtingPersonAssignmentHolder {
        pub assigned_items: PlaceHolder<Vec<DraughtingOrganizationItem>>,
        pub person_assignment: PlaceHolder<PersonAssignment>,
    }
    impl ::std::ops::Deref for DraughtingPreDefinedColour {
        type Target = PreDefinedColour;
        fn deref(&self) -> &Self::Target {
            &self.pre_defined_colour
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DraughtingPreDefinedColour {
        pub pre_defined_colour: PreDefinedColour,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct DraughtingPreDefinedColourHolder {
        pub pre_defined_colour: PlaceHolder<PreDefinedColour>,
    }
    impl ::std::ops::Deref for DraughtingPreDefinedCurveFont {
        type Target = PreDefinedCurveFont;
        fn deref(&self) -> &Self::Target {
            &self.pre_defined_curve_font
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DraughtingPreDefinedCurveFont {
        pub pre_defined_curve_font: PreDefinedCurveFont,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct DraughtingPreDefinedCurveFontHolder {
        pub pre_defined_curve_font: PlaceHolder<PreDefinedCurveFont>,
    }
    impl ::std::ops::Deref for DraughtingPreDefinedTextFont {
        type Target = PreDefinedTextFont;
        fn deref(&self) -> &Self::Target {
            &self.pre_defined_text_font
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DraughtingPreDefinedTextFont {
        pub pre_defined_text_font: PreDefinedTextFont,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct DraughtingPreDefinedTextFontHolder {
        pub pre_defined_text_font: PlaceHolder<PreDefinedTextFont>,
    }
    impl ::std::ops::Deref for DraughtingPresentedItem {
        type Target = PresentedItem;
        fn deref(&self) -> &Self::Target {
            &self.presented_item
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DraughtingPresentedItem {
        pub items: Vec<DraughtingPresentedItemSelect>,
        pub presented_item: PresentedItem,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct DraughtingPresentedItemHolder {
        pub items: PlaceHolder<Vec<DraughtingPresentedItemSelect>>,
        pub presented_item: PlaceHolder<PresentedItem>,
    }
    impl ::std::ops::Deref for DraughtingSecurityClassificationAssignment {
        type Target = SecurityClassificationAssignment;
        fn deref(&self) -> &Self::Target {
            &self.security_classification_assignment
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DraughtingSecurityClassificationAssignment {
        pub assigned_items: Vec<ClassifiedItem>,
        pub security_classification_assignment: SecurityClassificationAssignment,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct DraughtingSecurityClassificationAssignmentHolder {
        pub assigned_items: PlaceHolder<Vec<ClassifiedItem>>,
        pub security_classification_assignment: PlaceHolder<SecurityClassificationAssignment>,
    }
    impl ::std::ops::Deref for DraughtingSpecificationReference {
        type Target = DocumentReference;
        fn deref(&self) -> &Self::Target {
            &self.document_reference
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DraughtingSpecificationReference {
        pub specified_items: Vec<SpecifiedItem>,
        pub document_reference: DocumentReference,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct DraughtingSpecificationReferenceHolder {
        pub specified_items: PlaceHolder<Vec<SpecifiedItem>>,
        pub document_reference: PlaceHolder<DocumentReference>,
    }
    impl ::std::ops::Deref for DraughtingSubfigureRepresentation {
        type Target = SymbolRepresentation;
        fn deref(&self) -> &Self::Target {
            &self.symbol_representation
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DraughtingSubfigureRepresentation {
        pub symbol_representation: SymbolRepresentation,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct DraughtingSubfigureRepresentationHolder {
        pub symbol_representation: PlaceHolder<SymbolRepresentation>,
    }
    impl ::std::ops::Deref for DraughtingSymbolRepresentation {
        type Target = SymbolRepresentation;
        fn deref(&self) -> &Self::Target {
            &self.symbol_representation
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DraughtingSymbolRepresentation {
        pub symbol_representation: SymbolRepresentation,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct DraughtingSymbolRepresentationHolder {
        pub symbol_representation: PlaceHolder<SymbolRepresentation>,
    }
    impl ::std::ops::Deref for DraughtingTextLiteralWithDelineation {
        type Target = TextLiteralWithDelineation;
        fn deref(&self) -> &Self::Target {
            &self.text_literal_with_delineation
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DraughtingTextLiteralWithDelineation {
        pub text_literal_with_delineation: TextLiteralWithDelineation,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct DraughtingTextLiteralWithDelineationHolder {
        pub text_literal_with_delineation: PlaceHolder<TextLiteralWithDelineation>,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DraughtingTitle {
        pub items: Vec<DraughtingTitledItem>,
        pub language: Label,
        pub contents: Text,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct DraughtingTitleHolder {
        pub items: PlaceHolder<Vec<DraughtingTitledItem>>,
        pub language: PlaceHolder<Label>,
        pub contents: PlaceHolder<Text>,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DrawingDefinition {
        pub drawing_number: Identifier,
        pub drawing_type: Option<Label>,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct DrawingDefinitionHolder {
        pub drawing_number: PlaceHolder<Identifier>,
        pub drawing_type: Option<PlaceHolder<Label>>,
    }
    impl ::std::ops::Deref for DrawingRevision {
        type Target = PresentationSet;
        fn deref(&self) -> &Self::Target {
            &self.presentation_set
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DrawingRevision {
        pub revision_identifier: Identifier,
        pub drawing_identifier: DrawingDefinition,
        pub intended_scale: Option<Text>,
        pub presentation_set: PresentationSet,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct DrawingRevisionHolder {
        pub revision_identifier: PlaceHolder<Identifier>,
        pub drawing_identifier: PlaceHolder<DrawingDefinition>,
        pub intended_scale: Option<PlaceHolder<Text>>,
        pub presentation_set: PlaceHolder<PresentationSet>,
    }
    impl ::std::ops::Deref for DrawingSheetLayout {
        type Target = DraughtingSymbolRepresentation;
        fn deref(&self) -> &Self::Target {
            &self.draughting_symbol_representation
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DrawingSheetLayout {
        pub draughting_symbol_representation: DraughtingSymbolRepresentation,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct DrawingSheetLayoutHolder {
        pub draughting_symbol_representation: PlaceHolder<DraughtingSymbolRepresentation>,
    }
    impl ::std::ops::Deref for DrawingSheetRevision {
        type Target = PresentationArea;
        fn deref(&self) -> &Self::Target {
            &self.presentation_area
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DrawingSheetRevision {
        pub revision_identifier: Identifier,
        pub presentation_area: PresentationArea,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct DrawingSheetRevisionHolder {
        pub revision_identifier: PlaceHolder<Identifier>,
        pub presentation_area: PlaceHolder<PresentationArea>,
    }
    impl ::std::ops::Deref for DrawingSheetRevisionUsage {
        type Target = AreaInSet;
        fn deref(&self) -> &Self::Target {
            &self.area_in_set
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DrawingSheetRevisionUsage {
        pub sheet_number: Identifier,
        pub area_in_set: AreaInSet,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct DrawingSheetRevisionUsageHolder {
        pub sheet_number: PlaceHolder<Identifier>,
        pub area_in_set: PlaceHolder<AreaInSet>,
    }
    impl ::std::ops::Deref for Ellipse {
        type Target = Conic;
        fn deref(&self) -> &Self::Target {
            &self.conic
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct Ellipse {
        pub semi_axis_1: PositiveLengthMeasure,
        pub semi_axis_2: PositiveLengthMeasure,
        pub conic: Conic,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct EllipseHolder {
        pub semi_axis_1: PlaceHolder<PositiveLengthMeasure>,
        pub semi_axis_2: PlaceHolder<PositiveLengthMeasure>,
        pub conic: PlaceHolder<Conic>,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ExternalSource {
        pub source_id: SourceItem,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct ExternalSourceHolder {
        pub source_id: PlaceHolder<SourceItem>,
    }
    impl ::std::ops::Deref for ExternallyDefinedCurveFont {
        type Target = ExternallyDefinedItem;
        fn deref(&self) -> &Self::Target {
            &self.externally_defined_item
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ExternallyDefinedCurveFont {
        pub externally_defined_item: ExternallyDefinedItem,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct ExternallyDefinedCurveFontHolder {
        pub externally_defined_item: PlaceHolder<ExternallyDefinedItem>,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ExternallyDefinedHatchStyle {
        pub externally_defined_item: ExternallyDefinedItem,
        pub geometric_representation_item: GeometricRepresentationItem,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct ExternallyDefinedHatchStyleHolder {
        pub externally_defined_item: PlaceHolder<ExternallyDefinedItem>,
        pub geometric_representation_item: PlaceHolder<GeometricRepresentationItem>,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ExternallyDefinedItem {
        pub item_id: SourceItem,
        pub source: ExternalSource,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct ExternallyDefinedItemHolder {
        pub item_id: PlaceHolder<SourceItem>,
        pub source: PlaceHolder<ExternalSource>,
    }
    impl ::std::ops::Deref for ExternallyDefinedSymbol {
        type Target = ExternallyDefinedItem;
        fn deref(&self) -> &Self::Target {
            &self.externally_defined_item
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ExternallyDefinedSymbol {
        pub externally_defined_item: ExternallyDefinedItem,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct ExternallyDefinedSymbolHolder {
        pub externally_defined_item: PlaceHolder<ExternallyDefinedItem>,
    }
    impl ::std::ops::Deref for ExternallyDefinedTextFont {
        type Target = ExternallyDefinedItem;
        fn deref(&self) -> &Self::Target {
            &self.externally_defined_item
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ExternallyDefinedTextFont {
        pub externally_defined_item: ExternallyDefinedItem,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct ExternallyDefinedTextFontHolder {
        pub externally_defined_item: PlaceHolder<ExternallyDefinedItem>,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ExternallyDefinedTileStyle {
        pub externally_defined_item: ExternallyDefinedItem,
        pub geometric_representation_item: GeometricRepresentationItem,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct ExternallyDefinedTileStyleHolder {
        pub externally_defined_item: PlaceHolder<ExternallyDefinedItem>,
        pub geometric_representation_item: PlaceHolder<GeometricRepresentationItem>,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct FillAreaStyle {
        pub name: Label,
        pub fill_styles: Vec<FillStyleSelect>,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct FillAreaStyleHolder {
        pub name: PlaceHolder<Label>,
        pub fill_styles: PlaceHolder<Vec<FillStyleSelect>>,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct FillAreaStyleColour {
        pub name: Label,
        pub fill_colour: Colour,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct FillAreaStyleColourHolder {
        pub name: PlaceHolder<Label>,
        pub fill_colour: PlaceHolder<Colour>,
    }
    impl ::std::ops::Deref for FillAreaStyleHatching {
        type Target = GeometricRepresentationItem;
        fn deref(&self) -> &Self::Target {
            &self.geometric_representation_item
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct FillAreaStyleHatching {
        pub hatch_line_appearance: CurveStyle,
        pub start_of_next_hatch_line: OneDirectionRepeatFactor,
        pub point_of_reference_hatch_line: CartesianPoint,
        pub pattern_start: CartesianPoint,
        pub hatch_line_angle: PlaneAngleMeasure,
        pub geometric_representation_item: GeometricRepresentationItem,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct FillAreaStyleHatchingHolder {
        pub hatch_line_appearance: PlaceHolder<CurveStyle>,
        pub start_of_next_hatch_line: PlaceHolder<OneDirectionRepeatFactor>,
        pub point_of_reference_hatch_line: PlaceHolder<CartesianPoint>,
        pub pattern_start: PlaceHolder<CartesianPoint>,
        pub hatch_line_angle: PlaceHolder<PlaneAngleMeasure>,
        pub geometric_representation_item: PlaceHolder<GeometricRepresentationItem>,
    }
    impl ::std::ops::Deref for FillAreaStyleTileSymbolWithStyle {
        type Target = GeometricRepresentationItem;
        fn deref(&self) -> &Self::Target {
            &self.geometric_representation_item
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct FillAreaStyleTileSymbolWithStyle {
        pub symbol: AnnotationSymbolOccurrence,
        pub geometric_representation_item: GeometricRepresentationItem,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct FillAreaStyleTileSymbolWithStyleHolder {
        pub symbol: PlaceHolder<AnnotationSymbolOccurrence>,
        pub geometric_representation_item: PlaceHolder<GeometricRepresentationItem>,
    }
    impl ::std::ops::Deref for FillAreaStyleTiles {
        type Target = GeometricRepresentationItem;
        fn deref(&self) -> &Self::Target {
            &self.geometric_representation_item
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct FillAreaStyleTiles {
        pub tiling_pattern: TwoDirectionRepeatFactor,
        pub tiles: Vec<FillAreaStyleTileShapeSelect>,
        pub tiling_scale: PositiveRatioMeasure,
        pub geometric_representation_item: GeometricRepresentationItem,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct FillAreaStyleTilesHolder {
        pub tiling_pattern: PlaceHolder<TwoDirectionRepeatFactor>,
        pub tiles: PlaceHolder<Vec<FillAreaStyleTileShapeSelect>>,
        pub tiling_scale: PlaceHolder<PositiveRatioMeasure>,
        pub geometric_representation_item: PlaceHolder<GeometricRepresentationItem>,
    }
    impl ::std::ops::Deref for GeometricCurveSet {
        type Target = GeometricSet;
        fn deref(&self) -> &Self::Target {
            &self.geometric_set
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct GeometricCurveSet {
        pub geometric_set: GeometricSet,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct GeometricCurveSetHolder {
        pub geometric_set: PlaceHolder<GeometricSet>,
    }
    impl ::std::ops::Deref for GeometricRepresentationContext {
        type Target = RepresentationContext;
        fn deref(&self) -> &Self::Target {
            &self.representation_context
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct GeometricRepresentationContext {
        pub coordinate_space_dimension: DimensionCount,
        pub representation_context: RepresentationContext,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct GeometricRepresentationContextHolder {
        pub coordinate_space_dimension: PlaceHolder<DimensionCount>,
        pub representation_context: PlaceHolder<RepresentationContext>,
    }
    impl ::std::ops::Deref for GeometricRepresentationItem {
        type Target = RepresentationItem;
        fn deref(&self) -> &Self::Target {
            &self.representation_item
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct GeometricRepresentationItem {
        pub representation_item: RepresentationItem,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct GeometricRepresentationItemHolder {
        pub representation_item: PlaceHolder<RepresentationItem>,
    }
    impl ::std::ops::Deref for GeometricSet {
        type Target = GeometricRepresentationItem;
        fn deref(&self) -> &Self::Target {
            &self.geometric_representation_item
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct GeometricSet {
        pub elements: Vec<GeometricSetSelect>,
        pub geometric_representation_item: GeometricRepresentationItem,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct GeometricSetHolder {
        pub elements: PlaceHolder<Vec<GeometricSetSelect>>,
        pub geometric_representation_item: PlaceHolder<GeometricRepresentationItem>,
    }
    impl ::std::ops::Deref for GeometricalToleranceCallout {
        type Target = DraughtingCallout;
        fn deref(&self) -> &Self::Target {
            &self.draughting_callout
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct GeometricalToleranceCallout {
        pub draughting_callout: DraughtingCallout,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct GeometricalToleranceCalloutHolder {
        pub draughting_callout: PlaceHolder<DraughtingCallout>,
    }
    impl ::std::ops::Deref for GeometricallyBounded2DWireframeRepresentation {
        type Target = ShapeRepresentation;
        fn deref(&self) -> &Self::Target {
            &self.shape_representation
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct GeometricallyBounded2DWireframeRepresentation {
        pub shape_representation: ShapeRepresentation,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct GeometricallyBounded2DWireframeRepresentationHolder {
        pub shape_representation: PlaceHolder<ShapeRepresentation>,
    }
    impl ::std::ops::Deref for GlobalUnitAssignedContext {
        type Target = RepresentationContext;
        fn deref(&self) -> &Self::Target {
            &self.representation_context
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct GlobalUnitAssignedContext {
        pub units: Vec<Unit>,
        pub representation_context: RepresentationContext,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct GlobalUnitAssignedContextHolder {
        pub units: PlaceHolder<Vec<Unit>>,
        pub representation_context: PlaceHolder<RepresentationContext>,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct Group {
        pub name: Label,
        pub description: Text,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct GroupHolder {
        pub name: PlaceHolder<Label>,
        pub description: PlaceHolder<Text>,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct GroupAssignment {
        pub assigned_group: Group,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct GroupAssignmentHolder {
        pub assigned_group: PlaceHolder<Group>,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct GroupRelationship {
        pub name: Label,
        pub description: Text,
        pub relating_group: Group,
        pub related_group: Group,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct GroupRelationshipHolder {
        pub name: PlaceHolder<Label>,
        pub description: PlaceHolder<Text>,
        pub relating_group: PlaceHolder<Group>,
        pub related_group: PlaceHolder<Group>,
    }
    impl ::std::ops::Deref for Hyperbola {
        type Target = Conic;
        fn deref(&self) -> &Self::Target {
            &self.conic
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct Hyperbola {
        pub semi_axis: PositiveLengthMeasure,
        pub semi_imag_axis: PositiveLengthMeasure,
        pub conic: Conic,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct HyperbolaHolder {
        pub semi_axis: PlaceHolder<PositiveLengthMeasure>,
        pub semi_imag_axis: PlaceHolder<PositiveLengthMeasure>,
        pub conic: PlaceHolder<Conic>,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct Invisibility {
        pub invisible_items: Vec<InvisibleItem>,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct InvisibilityHolder {
        pub invisible_items: PlaceHolder<Vec<InvisibleItem>>,
    }
    impl ::std::ops::Deref for LeaderCurve {
        type Target = AnnotationCurveOccurrence;
        fn deref(&self) -> &Self::Target {
            &self.annotation_curve_occurrence
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct LeaderCurve {
        pub annotation_curve_occurrence: AnnotationCurveOccurrence,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct LeaderCurveHolder {
        pub annotation_curve_occurrence: PlaceHolder<AnnotationCurveOccurrence>,
    }
    impl ::std::ops::Deref for LeaderDirectedCallout {
        type Target = DraughtingCallout;
        fn deref(&self) -> &Self::Target {
            &self.draughting_callout
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct LeaderDirectedCallout {
        pub draughting_callout: DraughtingCallout,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct LeaderDirectedCalloutHolder {
        pub draughting_callout: PlaceHolder<DraughtingCallout>,
    }
    impl ::std::ops::Deref for LeaderDirectedDimension {
        type Target = LeaderDirectedCallout;
        fn deref(&self) -> &Self::Target {
            &self.leader_directed_callout
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct LeaderDirectedDimension {
        pub leader_directed_callout: LeaderDirectedCallout,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct LeaderDirectedDimensionHolder {
        pub leader_directed_callout: PlaceHolder<LeaderDirectedCallout>,
    }
    impl ::std::ops::Deref for LeaderTerminator {
        type Target = TerminatorSymbol;
        fn deref(&self) -> &Self::Target {
            &self.terminator_symbol
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct LeaderTerminator {
        pub terminator_symbol: TerminatorSymbol,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct LeaderTerminatorHolder {
        pub terminator_symbol: PlaceHolder<TerminatorSymbol>,
    }
    impl ::std::ops::Deref for LengthMeasureWithUnit {
        type Target = MeasureWithUnit;
        fn deref(&self) -> &Self::Target {
            &self.measure_with_unit
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct LengthMeasureWithUnit {
        pub measure_with_unit: MeasureWithUnit,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct LengthMeasureWithUnitHolder {
        pub measure_with_unit: PlaceHolder<MeasureWithUnit>,
    }
    impl ::std::ops::Deref for LengthUnit {
        type Target = NamedUnit;
        fn deref(&self) -> &Self::Target {
            &self.named_unit
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct LengthUnit {
        pub named_unit: NamedUnit,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct LengthUnitHolder {
        pub named_unit: PlaceHolder<NamedUnit>,
    }
    impl ::std::ops::Deref for Line {
        type Target = Curve;
        fn deref(&self) -> &Self::Target {
            &self.curve
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct Line {
        pub pnt: CartesianPoint,
        pub dir: Vector,
        pub curve: Curve,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct LineHolder {
        pub pnt: PlaceHolder<CartesianPoint>,
        pub dir: PlaceHolder<Vector>,
        pub curve: PlaceHolder<Curve>,
    }
    impl ::std::ops::Deref for LinearDimension {
        type Target = DimensionCurveDirectedCallout;
        fn deref(&self) -> &Self::Target {
            &self.dimension_curve_directed_callout
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct LinearDimension {
        pub dimension_curve_directed_callout: DimensionCurveDirectedCallout,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct LinearDimensionHolder {
        pub dimension_curve_directed_callout: PlaceHolder<DimensionCurveDirectedCallout>,
    }
    impl ::std::ops::Deref for MappedItem {
        type Target = RepresentationItem;
        fn deref(&self) -> &Self::Target {
            &self.representation_item
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct MappedItem {
        pub mapping_source: RepresentationMap,
        pub mapping_target: RepresentationItem,
        pub representation_item: RepresentationItem,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct MappedItemHolder {
        pub mapping_source: PlaceHolder<RepresentationMap>,
        pub mapping_target: PlaceHolder<RepresentationItem>,
        pub representation_item: PlaceHolder<RepresentationItem>,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct MeasureWithUnit {
        pub value_component: MeasureValue,
        pub unit_component: Unit,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct MeasureWithUnitHolder {
        pub value_component: PlaceHolder<MeasureValue>,
        pub unit_component: PlaceHolder<Unit>,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct NamedUnit {
        pub dimensions: DimensionalExponents,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct NamedUnitHolder {
        pub dimensions: PlaceHolder<DimensionalExponents>,
    }
    impl ::std::ops::Deref for OffsetCurve2D {
        type Target = Curve;
        fn deref(&self) -> &Self::Target {
            &self.curve
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct OffsetCurve2D {
        pub basis_curve: Curve,
        pub distance: LengthMeasure,
        pub self_intersect: Logical,
        pub curve: Curve,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct OffsetCurve2DHolder {
        pub basis_curve: PlaceHolder<Curve>,
        pub distance: PlaceHolder<LengthMeasure>,
        pub self_intersect: Logical,
        pub curve: PlaceHolder<Curve>,
    }
    impl ::std::ops::Deref for OneDirectionRepeatFactor {
        type Target = GeometricRepresentationItem;
        fn deref(&self) -> &Self::Target {
            &self.geometric_representation_item
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct OneDirectionRepeatFactor {
        pub repeat_factor: Vector,
        pub geometric_representation_item: GeometricRepresentationItem,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct OneDirectionRepeatFactorHolder {
        pub repeat_factor: PlaceHolder<Vector>,
        pub geometric_representation_item: PlaceHolder<GeometricRepresentationItem>,
    }
    impl ::std::ops::Deref for OrdinateDimension {
        type Target = ProjectionDirectedCallout;
        fn deref(&self) -> &Self::Target {
            &self.projection_directed_callout
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct OrdinateDimension {
        pub projection_directed_callout: ProjectionDirectedCallout,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct OrdinateDimensionHolder {
        pub projection_directed_callout: PlaceHolder<ProjectionDirectedCallout>,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct Organization {
        pub id: Option<Identifier>,
        pub name: Label,
        pub description: Text,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct OrganizationHolder {
        pub id: Option<PlaceHolder<Identifier>>,
        pub name: PlaceHolder<Label>,
        pub description: PlaceHolder<Text>,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct OrganizationAssignment {
        pub assigned_organization: Organization,
        pub role: OrganizationRole,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct OrganizationAssignmentHolder {
        pub assigned_organization: PlaceHolder<Organization>,
        pub role: PlaceHolder<OrganizationRole>,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct OrganizationRole {
        pub name: Label,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct OrganizationRoleHolder {
        pub name: PlaceHolder<Label>,
    }
    impl ::std::ops::Deref for OrganizationalAddress {
        type Target = Address;
        fn deref(&self) -> &Self::Target {
            &self.address
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct OrganizationalAddress {
        pub organizations: Vec<Organization>,
        pub description: Text,
        pub address: Address,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct OrganizationalAddressHolder {
        pub organizations: PlaceHolder<Vec<Organization>>,
        pub description: PlaceHolder<Text>,
        pub address: PlaceHolder<Address>,
    }
    impl ::std::ops::Deref for Parabola {
        type Target = Conic;
        fn deref(&self) -> &Self::Target {
            &self.conic
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct Parabola {
        pub focal_dist: LengthMeasure,
        pub conic: Conic,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct ParabolaHolder {
        pub focal_dist: PlaceHolder<LengthMeasure>,
        pub conic: PlaceHolder<Conic>,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct Person {
        pub id: Identifier,
        pub last_name: Option<Label>,
        pub first_name: Option<Label>,
        pub middle_names: Option<Vec<Label>>,
        pub prefix_titles: Option<Vec<Label>>,
        pub suffix_titles: Option<Vec<Label>>,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct PersonHolder {
        pub id: PlaceHolder<Identifier>,
        pub last_name: Option<PlaceHolder<Label>>,
        pub first_name: Option<PlaceHolder<Label>>,
        pub middle_names: Option<PlaceHolder<Vec<Label>>>,
        pub prefix_titles: Option<PlaceHolder<Vec<Label>>>,
        pub suffix_titles: Option<PlaceHolder<Vec<Label>>>,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PersonAndOrganization {
        pub the_person: Person,
        pub the_organization: Organization,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct PersonAndOrganizationHolder {
        pub the_person: PlaceHolder<Person>,
        pub the_organization: PlaceHolder<Organization>,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PersonAndOrganizationAssignment {
        pub assigned_person_and_organization: PersonAndOrganization,
        pub role: PersonAndOrganizationRole,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct PersonAndOrganizationAssignmentHolder {
        pub assigned_person_and_organization: PlaceHolder<PersonAndOrganization>,
        pub role: PlaceHolder<PersonAndOrganizationRole>,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PersonAndOrganizationRole {
        pub name: Label,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct PersonAndOrganizationRoleHolder {
        pub name: PlaceHolder<Label>,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PersonAssignment {
        pub assigned_person: Person,
        pub role: PersonRole,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct PersonAssignmentHolder {
        pub assigned_person: PlaceHolder<Person>,
        pub role: PlaceHolder<PersonRole>,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PersonRole {
        pub name: Label,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct PersonRoleHolder {
        pub name: PlaceHolder<Label>,
    }
    impl ::std::ops::Deref for PersonalAddress {
        type Target = Address;
        fn deref(&self) -> &Self::Target {
            &self.address
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PersonalAddress {
        pub people: Vec<Person>,
        pub description: Text,
        pub address: Address,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct PersonalAddressHolder {
        pub people: PlaceHolder<Vec<Person>>,
        pub description: PlaceHolder<Text>,
        pub address: PlaceHolder<Address>,
    }
    impl ::std::ops::Deref for Placement {
        type Target = GeometricRepresentationItem;
        fn deref(&self) -> &Self::Target {
            &self.geometric_representation_item
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct Placement {
        pub location: CartesianPoint,
        pub geometric_representation_item: GeometricRepresentationItem,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct PlacementHolder {
        pub location: PlaceHolder<CartesianPoint>,
        pub geometric_representation_item: PlaceHolder<GeometricRepresentationItem>,
    }
    impl ::std::ops::Deref for PlanarBox {
        type Target = PlanarExtent;
        fn deref(&self) -> &Self::Target {
            &self.planar_extent
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PlanarBox {
        pub placement: Axis2Placement,
        pub planar_extent: PlanarExtent,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct PlanarBoxHolder {
        pub placement: PlaceHolder<Axis2Placement>,
        pub planar_extent: PlaceHolder<PlanarExtent>,
    }
    impl ::std::ops::Deref for PlanarExtent {
        type Target = GeometricRepresentationItem;
        fn deref(&self) -> &Self::Target {
            &self.geometric_representation_item
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PlanarExtent {
        pub size_in_x: LengthMeasure,
        pub size_in_y: LengthMeasure,
        pub geometric_representation_item: GeometricRepresentationItem,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct PlanarExtentHolder {
        pub size_in_x: PlaceHolder<LengthMeasure>,
        pub size_in_y: PlaceHolder<LengthMeasure>,
        pub geometric_representation_item: PlaceHolder<GeometricRepresentationItem>,
    }
    impl ::std::ops::Deref for PlaneAngleMeasureWithUnit {
        type Target = MeasureWithUnit;
        fn deref(&self) -> &Self::Target {
            &self.measure_with_unit
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PlaneAngleMeasureWithUnit {
        pub measure_with_unit: MeasureWithUnit,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct PlaneAngleMeasureWithUnitHolder {
        pub measure_with_unit: PlaceHolder<MeasureWithUnit>,
    }
    impl ::std::ops::Deref for PlaneAngleUnit {
        type Target = NamedUnit;
        fn deref(&self) -> &Self::Target {
            &self.named_unit
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PlaneAngleUnit {
        pub named_unit: NamedUnit,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct PlaneAngleUnitHolder {
        pub named_unit: PlaceHolder<NamedUnit>,
    }
    impl ::std::ops::Deref for Point {
        type Target = GeometricRepresentationItem;
        fn deref(&self) -> &Self::Target {
            &self.geometric_representation_item
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct Point {
        pub geometric_representation_item: GeometricRepresentationItem,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct PointHolder {
        pub geometric_representation_item: PlaceHolder<GeometricRepresentationItem>,
    }
    impl ::std::ops::Deref for PointOnCurve {
        type Target = Point;
        fn deref(&self) -> &Self::Target {
            &self.point
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PointOnCurve {
        pub basis_curve: Curve,
        pub point_parameter: ParameterValue,
        pub point: Point,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct PointOnCurveHolder {
        pub basis_curve: PlaceHolder<Curve>,
        pub point_parameter: PlaceHolder<ParameterValue>,
        pub point: PlaceHolder<Point>,
    }
    impl ::std::ops::Deref for Polyline {
        type Target = BoundedCurve;
        fn deref(&self) -> &Self::Target {
            &self.bounded_curve
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct Polyline {
        pub points: Vec<CartesianPoint>,
        pub bounded_curve: BoundedCurve,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct PolylineHolder {
        pub points: PlaceHolder<Vec<CartesianPoint>>,
        pub bounded_curve: PlaceHolder<BoundedCurve>,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PreDefinedColour {
        pub pre_defined_item: PreDefinedItem,
        pub colour: Colour,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct PreDefinedColourHolder {
        pub pre_defined_item: PlaceHolder<PreDefinedItem>,
        pub colour: PlaceHolder<Colour>,
    }
    impl ::std::ops::Deref for PreDefinedCurveFont {
        type Target = PreDefinedItem;
        fn deref(&self) -> &Self::Target {
            &self.pre_defined_item
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PreDefinedCurveFont {
        pub pre_defined_item: PreDefinedItem,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct PreDefinedCurveFontHolder {
        pub pre_defined_item: PlaceHolder<PreDefinedItem>,
    }
    impl ::std::ops::Deref for PreDefinedDimensionSymbol {
        type Target = PreDefinedSymbol;
        fn deref(&self) -> &Self::Target {
            &self.pre_defined_symbol
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PreDefinedDimensionSymbol {
        pub pre_defined_symbol: PreDefinedSymbol,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct PreDefinedDimensionSymbolHolder {
        pub pre_defined_symbol: PlaceHolder<PreDefinedSymbol>,
    }
    impl ::std::ops::Deref for PreDefinedGeometricalToleranceSymbol {
        type Target = PreDefinedSymbol;
        fn deref(&self) -> &Self::Target {
            &self.pre_defined_symbol
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PreDefinedGeometricalToleranceSymbol {
        pub pre_defined_symbol: PreDefinedSymbol,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct PreDefinedGeometricalToleranceSymbolHolder {
        pub pre_defined_symbol: PlaceHolder<PreDefinedSymbol>,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PreDefinedItem {
        pub name: Label,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct PreDefinedItemHolder {
        pub name: PlaceHolder<Label>,
    }
    impl ::std::ops::Deref for PreDefinedPointMarkerSymbol {
        type Target = PreDefinedSymbol;
        fn deref(&self) -> &Self::Target {
            &self.pre_defined_symbol
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PreDefinedPointMarkerSymbol {
        pub pre_defined_symbol: PreDefinedSymbol,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct PreDefinedPointMarkerSymbolHolder {
        pub pre_defined_symbol: PlaceHolder<PreDefinedSymbol>,
    }
    impl ::std::ops::Deref for PreDefinedSymbol {
        type Target = PreDefinedItem;
        fn deref(&self) -> &Self::Target {
            &self.pre_defined_item
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PreDefinedSymbol {
        pub pre_defined_item: PreDefinedItem,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct PreDefinedSymbolHolder {
        pub pre_defined_item: PlaceHolder<PreDefinedItem>,
    }
    impl ::std::ops::Deref for PreDefinedTerminatorSymbol {
        type Target = PreDefinedSymbol;
        fn deref(&self) -> &Self::Target {
            &self.pre_defined_symbol
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PreDefinedTerminatorSymbol {
        pub pre_defined_symbol: PreDefinedSymbol,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct PreDefinedTerminatorSymbolHolder {
        pub pre_defined_symbol: PlaceHolder<PreDefinedSymbol>,
    }
    impl ::std::ops::Deref for PreDefinedTextFont {
        type Target = PreDefinedItem;
        fn deref(&self) -> &Self::Target {
            &self.pre_defined_item
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PreDefinedTextFont {
        pub pre_defined_item: PreDefinedItem,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct PreDefinedTextFontHolder {
        pub pre_defined_item: PlaceHolder<PreDefinedItem>,
    }
    impl ::std::ops::Deref for PresentationArea {
        type Target = PresentationRepresentation;
        fn deref(&self) -> &Self::Target {
            &self.presentation_representation
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PresentationArea {
        pub presentation_representation: PresentationRepresentation,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct PresentationAreaHolder {
        pub presentation_representation: PlaceHolder<PresentationRepresentation>,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PresentationLayerAssignment {
        pub name: Label,
        pub description: Text,
        pub assigned_items: Vec<LayeredItem>,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct PresentationLayerAssignmentHolder {
        pub name: PlaceHolder<Label>,
        pub description: PlaceHolder<Text>,
        pub assigned_items: PlaceHolder<Vec<LayeredItem>>,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PresentationLayerUsage {
        pub assignment: PresentationLayerAssignment,
        pub presentation: PresentationRepresentation,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct PresentationLayerUsageHolder {
        pub assignment: PlaceHolder<PresentationLayerAssignment>,
        pub presentation: PlaceHolder<PresentationRepresentation>,
    }
    impl ::std::ops::Deref for PresentationRepresentation {
        type Target = Representation;
        fn deref(&self) -> &Self::Target {
            &self.representation
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PresentationRepresentation {
        pub representation: Representation,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct PresentationRepresentationHolder {
        pub representation: PlaceHolder<Representation>,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PresentationSet {}
    #[derive(Clone, Debug, PartialEq)]
    pub struct PresentationSetHolder {}
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PresentationSize {
        pub unit: PresentationSizeAssignmentSelect,
        pub size: PlanarBox,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct PresentationSizeHolder {
        pub unit: PlaceHolder<PresentationSizeAssignmentSelect>,
        pub size: PlaceHolder<PlanarBox>,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PresentationStyleAssignment {
        pub styles: Vec<PresentationStyleSelect>,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct PresentationStyleAssignmentHolder {
        pub styles: PlaceHolder<Vec<PresentationStyleSelect>>,
    }
    impl ::std::ops::Deref for PresentationStyleByContext {
        type Target = PresentationStyleAssignment;
        fn deref(&self) -> &Self::Target {
            &self.presentation_style_assignment
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PresentationStyleByContext {
        pub style_context: StyleContextSelect,
        pub presentation_style_assignment: PresentationStyleAssignment,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct PresentationStyleByContextHolder {
        pub style_context: PlaceHolder<StyleContextSelect>,
        pub presentation_style_assignment: PlaceHolder<PresentationStyleAssignment>,
    }
    impl ::std::ops::Deref for PresentationView {
        type Target = PresentationRepresentation;
        fn deref(&self) -> &Self::Target {
            &self.presentation_representation
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PresentationView {
        pub presentation_representation: PresentationRepresentation,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct PresentationViewHolder {
        pub presentation_representation: PlaceHolder<PresentationRepresentation>,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PresentedItem {}
    #[derive(Clone, Debug, PartialEq)]
    pub struct PresentedItemHolder {}
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PresentedItemRepresentation {
        pub presentation: PresentationRepresentationSelect,
        pub item: PresentedItem,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct PresentedItemRepresentationHolder {
        pub presentation: PlaceHolder<PresentationRepresentationSelect>,
        pub item: PlaceHolder<PresentedItem>,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct Product {
        pub id: Identifier,
        pub name: Label,
        pub description: Text,
        pub frame_of_reference: Vec<ProductContext>,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct ProductHolder {
        pub id: PlaceHolder<Identifier>,
        pub name: PlaceHolder<Label>,
        pub description: PlaceHolder<Text>,
        pub frame_of_reference: PlaceHolder<Vec<ProductContext>>,
    }
    impl ::std::ops::Deref for ProductContext {
        type Target = ApplicationContextElement;
        fn deref(&self) -> &Self::Target {
            &self.application_context_element
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ProductContext {
        pub discipline_type: Label,
        pub application_context_element: ApplicationContextElement,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct ProductContextHolder {
        pub discipline_type: PlaceHolder<Label>,
        pub application_context_element: PlaceHolder<ApplicationContextElement>,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ProductDefinition {
        pub id: Identifier,
        pub description: Text,
        pub formation: ProductDefinitionFormation,
        pub frame_of_reference: ProductDefinitionContext,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct ProductDefinitionHolder {
        pub id: PlaceHolder<Identifier>,
        pub description: PlaceHolder<Text>,
        pub formation: PlaceHolder<ProductDefinitionFormation>,
        pub frame_of_reference: PlaceHolder<ProductDefinitionContext>,
    }
    impl ::std::ops::Deref for ProductDefinitionContext {
        type Target = ApplicationContextElement;
        fn deref(&self) -> &Self::Target {
            &self.application_context_element
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ProductDefinitionContext {
        pub life_cycle_stage: Label,
        pub application_context_element: ApplicationContextElement,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct ProductDefinitionContextHolder {
        pub life_cycle_stage: PlaceHolder<Label>,
        pub application_context_element: PlaceHolder<ApplicationContextElement>,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ProductDefinitionFormation {
        pub id: Identifier,
        pub description: Text,
        pub of_product: Product,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct ProductDefinitionFormationHolder {
        pub id: PlaceHolder<Identifier>,
        pub description: PlaceHolder<Text>,
        pub of_product: PlaceHolder<Product>,
    }
    impl ::std::ops::Deref for ProductDefinitionShape {
        type Target = PropertyDefinition;
        fn deref(&self) -> &Self::Target {
            &self.property_definition
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ProductDefinitionShape {
        pub property_definition: PropertyDefinition,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct ProductDefinitionShapeHolder {
        pub property_definition: PlaceHolder<PropertyDefinition>,
    }
    impl ::std::ops::Deref for ProjectionCurve {
        type Target = AnnotationCurveOccurrence;
        fn deref(&self) -> &Self::Target {
            &self.annotation_curve_occurrence
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ProjectionCurve {
        pub annotation_curve_occurrence: AnnotationCurveOccurrence,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct ProjectionCurveHolder {
        pub annotation_curve_occurrence: PlaceHolder<AnnotationCurveOccurrence>,
    }
    impl ::std::ops::Deref for ProjectionDirectedCallout {
        type Target = DraughtingCallout;
        fn deref(&self) -> &Self::Target {
            &self.draughting_callout
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ProjectionDirectedCallout {
        pub draughting_callout: DraughtingCallout,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct ProjectionDirectedCalloutHolder {
        pub draughting_callout: PlaceHolder<DraughtingCallout>,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PropertyDefinition {
        pub name: Label,
        pub description: Text,
        pub definition: CharacterizedDefinition,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct PropertyDefinitionHolder {
        pub name: PlaceHolder<Label>,
        pub description: PlaceHolder<Text>,
        pub definition: PlaceHolder<CharacterizedDefinition>,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PropertyDefinitionRepresentation {
        pub definition: PropertyDefinition,
        pub used_representation: Representation,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct PropertyDefinitionRepresentationHolder {
        pub definition: PlaceHolder<PropertyDefinition>,
        pub used_representation: PlaceHolder<Representation>,
    }
    impl ::std::ops::Deref for QuasiUniformCurve {
        type Target = BSplineCurve;
        fn deref(&self) -> &Self::Target {
            &self.b_spline_curve
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct QuasiUniformCurve {
        pub b_spline_curve: BSplineCurve,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct QuasiUniformCurveHolder {
        pub b_spline_curve: PlaceHolder<BSplineCurve>,
    }
    impl ::std::ops::Deref for RadiusDimension {
        type Target = DimensionCurveDirectedCallout;
        fn deref(&self) -> &Self::Target {
            &self.dimension_curve_directed_callout
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct RadiusDimension {
        pub dimension_curve_directed_callout: DimensionCurveDirectedCallout,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct RadiusDimensionHolder {
        pub dimension_curve_directed_callout: PlaceHolder<DimensionCurveDirectedCallout>,
    }
    impl ::std::ops::Deref for RationalBSplineCurve {
        type Target = BSplineCurve;
        fn deref(&self) -> &Self::Target {
            &self.b_spline_curve
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct RationalBSplineCurve {
        pub weights_data: Vec<f64>,
        pub b_spline_curve: BSplineCurve,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct RationalBSplineCurveHolder {
        pub weights_data: PlaceHolder<Vec<f64>>,
        pub b_spline_curve: PlaceHolder<BSplineCurve>,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct Representation {
        pub name: Label,
        pub items: Vec<RepresentationItem>,
        pub context_of_items: RepresentationContext,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct RepresentationHolder {
        pub name: PlaceHolder<Label>,
        pub items: PlaceHolder<Vec<RepresentationItem>>,
        pub context_of_items: PlaceHolder<RepresentationContext>,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct RepresentationContext {
        pub context_identifier: Identifier,
        pub context_type: Text,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct RepresentationContextHolder {
        pub context_identifier: PlaceHolder<Identifier>,
        pub context_type: PlaceHolder<Text>,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct RepresentationItem {
        pub name: Label,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct RepresentationItemHolder {
        pub name: PlaceHolder<Label>,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct RepresentationMap {
        pub mapping_origin: RepresentationItem,
        pub mapped_representation: Representation,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct RepresentationMapHolder {
        pub mapping_origin: PlaceHolder<RepresentationItem>,
        pub mapped_representation: PlaceHolder<Representation>,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct SecurityClassification {
        pub name: Label,
        pub purpose: Text,
        pub security_level: SecurityClassificationLevel,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct SecurityClassificationHolder {
        pub name: PlaceHolder<Label>,
        pub purpose: PlaceHolder<Text>,
        pub security_level: PlaceHolder<SecurityClassificationLevel>,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct SecurityClassificationAssignment {
        pub assigned_security_classification: SecurityClassification,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct SecurityClassificationAssignmentHolder {
        pub assigned_security_classification: PlaceHolder<SecurityClassification>,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct SecurityClassificationLevel {
        pub name: Label,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct SecurityClassificationLevelHolder {
        pub name: PlaceHolder<Label>,
    }
    impl ::std::ops::Deref for ShapeDefinitionRepresentation {
        type Target = PropertyDefinitionRepresentation;
        fn deref(&self) -> &Self::Target {
            &self.property_definition_representation
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ShapeDefinitionRepresentation {
        pub property_definition_representation: PropertyDefinitionRepresentation,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct ShapeDefinitionRepresentationHolder {
        pub property_definition_representation: PlaceHolder<PropertyDefinitionRepresentation>,
    }
    impl ::std::ops::Deref for ShapeRepresentation {
        type Target = Representation;
        fn deref(&self) -> &Self::Target {
            &self.representation
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ShapeRepresentation {
        pub representation: Representation,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct ShapeRepresentationHolder {
        pub representation: PlaceHolder<Representation>,
    }
    impl ::std::ops::Deref for SiUnit {
        type Target = NamedUnit;
        fn deref(&self) -> &Self::Target {
            &self.named_unit
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct SiUnit {
        pub prefix: Option<SiPrefix>,
        pub name: SiUnitName,
        pub named_unit: NamedUnit,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct SiUnitHolder {
        pub prefix: Option<PlaceHolder<SiPrefix>>,
        pub name: PlaceHolder<SiUnitName>,
        pub named_unit: PlaceHolder<NamedUnit>,
    }
    impl ::std::ops::Deref for StructuredDimensionCallout {
        type Target = DraughtingCallout;
        fn deref(&self) -> &Self::Target {
            &self.draughting_callout
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct StructuredDimensionCallout {
        pub draughting_callout: DraughtingCallout,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct StructuredDimensionCalloutHolder {
        pub draughting_callout: PlaceHolder<DraughtingCallout>,
    }
    impl ::std::ops::Deref for StyledItem {
        type Target = RepresentationItem;
        fn deref(&self) -> &Self::Target {
            &self.representation_item
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct StyledItem {
        pub styles: Vec<PresentationStyleAssignment>,
        pub item: RepresentationItem,
        pub representation_item: RepresentationItem,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct StyledItemHolder {
        pub styles: PlaceHolder<Vec<PresentationStyleAssignment>>,
        pub item: PlaceHolder<RepresentationItem>,
        pub representation_item: PlaceHolder<RepresentationItem>,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct SymbolColour {
        pub colour_of_symbol: Colour,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct SymbolColourHolder {
        pub colour_of_symbol: PlaceHolder<Colour>,
    }
    impl ::std::ops::Deref for SymbolRepresentation {
        type Target = Representation;
        fn deref(&self) -> &Self::Target {
            &self.representation
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct SymbolRepresentation {
        pub representation: Representation,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct SymbolRepresentationHolder {
        pub representation: PlaceHolder<Representation>,
    }
    impl ::std::ops::Deref for SymbolRepresentationMap {
        type Target = RepresentationMap;
        fn deref(&self) -> &Self::Target {
            &self.representation_map
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct SymbolRepresentationMap {
        pub representation_map: RepresentationMap,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct SymbolRepresentationMapHolder {
        pub representation_map: PlaceHolder<RepresentationMap>,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct SymbolStyle {
        pub name: Label,
        pub style_of_symbol: SymbolStyleSelect,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct SymbolStyleHolder {
        pub name: PlaceHolder<Label>,
        pub style_of_symbol: PlaceHolder<SymbolStyleSelect>,
    }
    impl ::std::ops::Deref for SymbolTarget {
        type Target = GeometricRepresentationItem;
        fn deref(&self) -> &Self::Target {
            &self.geometric_representation_item
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct SymbolTarget {
        pub placement: Axis2Placement,
        pub x_scale: PositiveRatioMeasure,
        pub y_scale: PositiveRatioMeasure,
        pub geometric_representation_item: GeometricRepresentationItem,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct SymbolTargetHolder {
        pub placement: PlaceHolder<Axis2Placement>,
        pub x_scale: PlaceHolder<PositiveRatioMeasure>,
        pub y_scale: PlaceHolder<PositiveRatioMeasure>,
        pub geometric_representation_item: PlaceHolder<GeometricRepresentationItem>,
    }
    impl ::std::ops::Deref for TerminatorSymbol {
        type Target = AnnotationSymbolOccurrence;
        fn deref(&self) -> &Self::Target {
            &self.annotation_symbol_occurrence
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct TerminatorSymbol {
        pub annotated_curve: AnnotationCurveOccurrence,
        pub annotation_symbol_occurrence: AnnotationSymbolOccurrence,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct TerminatorSymbolHolder {
        pub annotated_curve: PlaceHolder<AnnotationCurveOccurrence>,
        pub annotation_symbol_occurrence: PlaceHolder<AnnotationSymbolOccurrence>,
    }
    impl ::std::ops::Deref for TextLiteral {
        type Target = GeometricRepresentationItem;
        fn deref(&self) -> &Self::Target {
            &self.geometric_representation_item
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct TextLiteral {
        pub literal: PresentableText,
        pub placement: Axis2Placement,
        pub alignment: TextAlignment,
        pub path: TextPath,
        pub font: FontSelect,
        pub geometric_representation_item: GeometricRepresentationItem,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct TextLiteralHolder {
        pub literal: PlaceHolder<PresentableText>,
        pub placement: PlaceHolder<Axis2Placement>,
        pub alignment: PlaceHolder<TextAlignment>,
        pub path: PlaceHolder<TextPath>,
        pub font: PlaceHolder<FontSelect>,
        pub geometric_representation_item: PlaceHolder<GeometricRepresentationItem>,
    }
    impl ::std::ops::Deref for TextLiteralWithAssociatedCurves {
        type Target = TextLiteral;
        fn deref(&self) -> &Self::Target {
            &self.text_literal
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct TextLiteralWithAssociatedCurves {
        pub associated_curves: Vec<Curve>,
        pub text_literal: TextLiteral,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct TextLiteralWithAssociatedCurvesHolder {
        pub associated_curves: PlaceHolder<Vec<Curve>>,
        pub text_literal: PlaceHolder<TextLiteral>,
    }
    impl ::std::ops::Deref for TextLiteralWithBlankingBox {
        type Target = TextLiteral;
        fn deref(&self) -> &Self::Target {
            &self.text_literal
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct TextLiteralWithBlankingBox {
        pub blanking: PlanarBox,
        pub text_literal: TextLiteral,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct TextLiteralWithBlankingBoxHolder {
        pub blanking: PlaceHolder<PlanarBox>,
        pub text_literal: PlaceHolder<TextLiteral>,
    }
    impl ::std::ops::Deref for TextLiteralWithDelineation {
        type Target = TextLiteral;
        fn deref(&self) -> &Self::Target {
            &self.text_literal
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct TextLiteralWithDelineation {
        pub delineation: TextDelineation,
        pub text_literal: TextLiteral,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct TextLiteralWithDelineationHolder {
        pub delineation: PlaceHolder<TextDelineation>,
        pub text_literal: PlaceHolder<TextLiteral>,
    }
    impl ::std::ops::Deref for TextLiteralWithExtent {
        type Target = TextLiteral;
        fn deref(&self) -> &Self::Target {
            &self.text_literal
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct TextLiteralWithExtent {
        pub extent: PlanarExtent,
        pub text_literal: TextLiteral,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct TextLiteralWithExtentHolder {
        pub extent: PlaceHolder<PlanarExtent>,
        pub text_literal: PlaceHolder<TextLiteral>,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct TextStyle {
        pub name: Label,
        pub character_appearance: CharacterStyleSelect,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct TextStyleHolder {
        pub name: PlaceHolder<Label>,
        pub character_appearance: PlaceHolder<CharacterStyleSelect>,
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct TextStyleForDefinedFont {
        pub text_colour: Colour,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct TextStyleForDefinedFontHolder {
        pub text_colour: PlaceHolder<Colour>,
    }
    impl ::std::ops::Deref for TextStyleWithBoxCharacteristics {
        type Target = TextStyle;
        fn deref(&self) -> &Self::Target {
            &self.text_style
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct TextStyleWithBoxCharacteristics {
        pub characteristics: Vec<BoxCharacteristicSelect>,
        pub text_style: TextStyle,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct TextStyleWithBoxCharacteristicsHolder {
        pub characteristics: PlaceHolder<Vec<BoxCharacteristicSelect>>,
        pub text_style: PlaceHolder<TextStyle>,
    }
    impl ::std::ops::Deref for TextStyleWithMirror {
        type Target = TextStyle;
        fn deref(&self) -> &Self::Target {
            &self.text_style
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct TextStyleWithMirror {
        pub mirror_placement: Axis2Placement,
        pub text_style: TextStyle,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct TextStyleWithMirrorHolder {
        pub mirror_placement: PlaceHolder<Axis2Placement>,
        pub text_style: PlaceHolder<TextStyle>,
    }
    impl ::std::ops::Deref for TrimmedCurve {
        type Target = BoundedCurve;
        fn deref(&self) -> &Self::Target {
            &self.bounded_curve
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct TrimmedCurve {
        pub basis_curve: Curve,
        pub trim_1: Vec<TrimmingSelect>,
        pub trim_2: Vec<TrimmingSelect>,
        pub sense_agreement: bool,
        pub master_representation: TrimmingPreference,
        pub bounded_curve: BoundedCurve,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct TrimmedCurveHolder {
        pub basis_curve: PlaceHolder<Curve>,
        pub trim_1: PlaceHolder<Vec<TrimmingSelect>>,
        pub trim_2: PlaceHolder<Vec<TrimmingSelect>>,
        pub sense_agreement: bool,
        pub master_representation: PlaceHolder<TrimmingPreference>,
        pub bounded_curve: PlaceHolder<BoundedCurve>,
    }
    impl ::std::ops::Deref for TwoDirectionRepeatFactor {
        type Target = OneDirectionRepeatFactor;
        fn deref(&self) -> &Self::Target {
            &self.one_direction_repeat_factor
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct TwoDirectionRepeatFactor {
        pub second_repeat_factor: Vector,
        pub one_direction_repeat_factor: OneDirectionRepeatFactor,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct TwoDirectionRepeatFactorHolder {
        pub second_repeat_factor: PlaceHolder<Vector>,
        pub one_direction_repeat_factor: PlaceHolder<OneDirectionRepeatFactor>,
    }
    impl ::std::ops::Deref for UniformCurve {
        type Target = BSplineCurve;
        fn deref(&self) -> &Self::Target {
            &self.b_spline_curve
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct UniformCurve {
        pub b_spline_curve: BSplineCurve,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct UniformCurveHolder {
        pub b_spline_curve: PlaceHolder<BSplineCurve>,
    }
    impl ::std::ops::Deref for Vector {
        type Target = GeometricRepresentationItem;
        fn deref(&self) -> &Self::Target {
            &self.geometric_representation_item
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct Vector {
        pub orientation: Direction,
        pub magnitude: LengthMeasure,
        pub geometric_representation_item: GeometricRepresentationItem,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct VectorHolder {
        pub orientation: PlaceHolder<Direction>,
        pub magnitude: PlaceHolder<LengthMeasure>,
        pub geometric_representation_item: PlaceHolder<GeometricRepresentationItem>,
    }
}

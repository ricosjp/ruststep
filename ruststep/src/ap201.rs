#![allow(dead_code)]
pub mod explicit_draughting {
    use crate::{error::Result, place_holder::*, primitive::*, tables::*};
    use std::collections::HashMap;
    #[derive(Debug, Clone, Default)]
    pub struct Tables {
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
        axis2_placement_2d: HashMap<u64, Axis2Placement2DHolder>,
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
    impl Tables {
        pub fn address_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<Address>> + 'table {
            self.address
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn angular_dimension_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<AngularDimension>> + 'table {
            self.angular_dimension
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn annotation_curve_occurrence_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<AnnotationCurveOccurrence>> + 'table {
            self.annotation_curve_occurrence
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn annotation_fill_area_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<AnnotationFillArea>> + 'table {
            self.annotation_fill_area
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn annotation_fill_area_occurrence_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<AnnotationFillAreaOccurrence>> + 'table {
            self.annotation_fill_area_occurrence
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn annotation_occurrence_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<AnnotationOccurrence>> + 'table {
            self.annotation_occurrence
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn annotation_subfigure_occurrence_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<AnnotationSubfigureOccurrence>> + 'table {
            self.annotation_subfigure_occurrence
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn annotation_symbol_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<AnnotationSymbol>> + 'table {
            self.annotation_symbol
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn annotation_symbol_occurrence_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<AnnotationSymbolOccurrence>> + 'table {
            self.annotation_symbol_occurrence
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn annotation_text_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<AnnotationText>> + 'table {
            self.annotation_text
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn annotation_text_occurrence_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<AnnotationTextOccurrence>> + 'table {
            self.annotation_text_occurrence
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn application_context_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ApplicationContext>> + 'table {
            self.application_context
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn application_context_element_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ApplicationContextElement>> + 'table {
            self.application_context_element
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn application_protocol_definition_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ApplicationProtocolDefinition>> + 'table {
            self.application_protocol_definition
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn approval_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<Approval>> + 'table {
            self.approval
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn approval_assignment_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ApprovalAssignment>> + 'table {
            self.approval_assignment
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn approval_date_time_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ApprovalDateTime>> + 'table {
            self.approval_date_time
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn approval_person_organization_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ApprovalPersonOrganization>> + 'table {
            self.approval_person_organization
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn approval_role_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ApprovalRole>> + 'table {
            self.approval_role
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn approval_status_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ApprovalStatus>> + 'table {
            self.approval_status
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn area_in_set_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<AreaInSet>> + 'table {
            self.area_in_set
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn axis2_placement_2d_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<Axis2Placement2D>> + 'table {
            self.axis2_placement_2d
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn b_spline_curve_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<BSplineCurve>> + 'table {
            self.b_spline_curve
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn b_spline_curve_with_knots_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<BSplineCurveWithKnots>> + 'table {
            self.b_spline_curve_with_knots
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn bezier_curve_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<BezierCurve>> + 'table {
            self.bezier_curve
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn bounded_curve_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<BoundedCurve>> + 'table {
            self.bounded_curve
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn calendar_date_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<CalendarDate>> + 'table {
            self.calendar_date
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn camera_image_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<CameraImage>> + 'table {
            self.camera_image
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn camera_image_2d_with_scale_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<CameraImage2DWithScale>> + 'table {
            self.camera_image_2d_with_scale
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn camera_model_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<CameraModel>> + 'table {
            self.camera_model
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn camera_model_d2_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<CameraModelD2>> + 'table {
            self.camera_model_d2
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn camera_usage_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<CameraUsage>> + 'table {
            self.camera_usage
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn cartesian_point_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<CartesianPoint>> + 'table {
            self.cartesian_point
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn circle_iter<'table>(&'table self) -> impl Iterator<Item = Result<Circle>> + 'table {
            self.circle
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn colour_iter<'table>(&'table self) -> impl Iterator<Item = Result<Colour>> + 'table {
            self.colour
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn colour_rgb_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ColourRgb>> + 'table {
            self.colour_rgb
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn colour_specification_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ColourSpecification>> + 'table {
            self.colour_specification
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn composite_curve_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<CompositeCurve>> + 'table {
            self.composite_curve
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn composite_curve_segment_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<CompositeCurveSegment>> + 'table {
            self.composite_curve_segment
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn composite_text_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<CompositeText>> + 'table {
            self.composite_text
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn composite_text_with_associated_curves_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<CompositeTextWithAssociatedCurves>> + 'table {
            self.composite_text_with_associated_curves
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn composite_text_with_blanking_box_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<CompositeTextWithBlankingBox>> + 'table {
            self.composite_text_with_blanking_box
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn composite_text_with_extent_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<CompositeTextWithExtent>> + 'table {
            self.composite_text_with_extent
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn conic_iter<'table>(&'table self) -> impl Iterator<Item = Result<Conic>> + 'table {
            self.conic
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn context_dependent_invisibility_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ContextDependentInvisibility>> + 'table {
            self.context_dependent_invisibility
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn contract_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<Contract>> + 'table {
            self.contract
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn contract_assignment_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ContractAssignment>> + 'table {
            self.contract_assignment
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn contract_type_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ContractType>> + 'table {
            self.contract_type
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn conversion_based_unit_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ConversionBasedUnit>> + 'table {
            self.conversion_based_unit
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn curve_iter<'table>(&'table self) -> impl Iterator<Item = Result<Curve>> + 'table {
            self.curve
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn curve_dimension_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<CurveDimension>> + 'table {
            self.curve_dimension
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn curve_style_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<CurveStyle>> + 'table {
            self.curve_style
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn curve_style_font_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<CurveStyleFont>> + 'table {
            self.curve_style_font
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn curve_style_font_pattern_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<CurveStyleFontPattern>> + 'table {
            self.curve_style_font_pattern
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn date_iter<'table>(&'table self) -> impl Iterator<Item = Result<Date>> + 'table {
            self.date
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn datum_feature_callout_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<DatumFeatureCallout>> + 'table {
            self.datum_feature_callout
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn datum_target_callout_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<DatumTargetCallout>> + 'table {
            self.datum_target_callout
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn defined_symbol_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<DefinedSymbol>> + 'table {
            self.defined_symbol
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn diameter_dimension_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<DiameterDimension>> + 'table {
            self.diameter_dimension
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn dimension_callout_component_relationship_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<DimensionCalloutComponentRelationship>> + 'table {
            self.dimension_callout_component_relationship
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn dimension_callout_relationship_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<DimensionCalloutRelationship>> + 'table {
            self.dimension_callout_relationship
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn dimension_curve_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<DimensionCurve>> + 'table {
            self.dimension_curve
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn dimension_curve_directed_callout_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<DimensionCurveDirectedCallout>> + 'table {
            self.dimension_curve_directed_callout
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn dimension_curve_terminator_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<DimensionCurveTerminator>> + 'table {
            self.dimension_curve_terminator
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn dimension_pair_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<DimensionPair>> + 'table {
            self.dimension_pair
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn dimensional_exponents_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<DimensionalExponents>> + 'table {
            self.dimensional_exponents
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn direction_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<Direction>> + 'table {
            self.direction
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn document_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<Document>> + 'table {
            self.document
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn document_reference_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<DocumentReference>> + 'table {
            self.document_reference
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn document_type_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<DocumentType>> + 'table {
            self.document_type
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn draughting_annotation_occurrence_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<DraughtingAnnotationOccurrence>> + 'table {
            self.draughting_annotation_occurrence
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn draughting_approval_assignment_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<DraughtingApprovalAssignment>> + 'table {
            self.draughting_approval_assignment
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn draughting_callout_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<DraughtingCallout>> + 'table {
            self.draughting_callout
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn draughting_callout_relationship_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<DraughtingCalloutRelationship>> + 'table {
            self.draughting_callout_relationship
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn draughting_contract_assignment_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<DraughtingContractAssignment>> + 'table {
            self.draughting_contract_assignment
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn draughting_drawing_revision_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<DraughtingDrawingRevision>> + 'table {
            self.draughting_drawing_revision
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn draughting_elements_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<DraughtingElements>> + 'table {
            self.draughting_elements
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn draughting_group_assignment_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<DraughtingGroupAssignment>> + 'table {
            self.draughting_group_assignment
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn draughting_model_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<DraughtingModel>> + 'table {
            self.draughting_model
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn draughting_organization_assignment_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<DraughtingOrganizationAssignment>> + 'table {
            self.draughting_organization_assignment
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn draughting_person_and_organization_assignment_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<DraughtingPersonAndOrganizationAssignment>> + 'table
        {
            self.draughting_person_and_organization_assignment
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn draughting_person_assignment_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<DraughtingPersonAssignment>> + 'table {
            self.draughting_person_assignment
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn draughting_pre_defined_colour_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<DraughtingPreDefinedColour>> + 'table {
            self.draughting_pre_defined_colour
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn draughting_pre_defined_curve_font_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<DraughtingPreDefinedCurveFont>> + 'table {
            self.draughting_pre_defined_curve_font
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn draughting_pre_defined_text_font_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<DraughtingPreDefinedTextFont>> + 'table {
            self.draughting_pre_defined_text_font
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn draughting_presented_item_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<DraughtingPresentedItem>> + 'table {
            self.draughting_presented_item
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn draughting_security_classification_assignment_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<DraughtingSecurityClassificationAssignment>> + 'table
        {
            self.draughting_security_classification_assignment
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn draughting_specification_reference_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<DraughtingSpecificationReference>> + 'table {
            self.draughting_specification_reference
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn draughting_subfigure_representation_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<DraughtingSubfigureRepresentation>> + 'table {
            self.draughting_subfigure_representation
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn draughting_symbol_representation_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<DraughtingSymbolRepresentation>> + 'table {
            self.draughting_symbol_representation
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn draughting_text_literal_with_delineation_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<DraughtingTextLiteralWithDelineation>> + 'table {
            self.draughting_text_literal_with_delineation
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn draughting_title_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<DraughtingTitle>> + 'table {
            self.draughting_title
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn drawing_definition_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<DrawingDefinition>> + 'table {
            self.drawing_definition
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn drawing_revision_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<DrawingRevision>> + 'table {
            self.drawing_revision
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn drawing_sheet_layout_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<DrawingSheetLayout>> + 'table {
            self.drawing_sheet_layout
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn drawing_sheet_revision_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<DrawingSheetRevision>> + 'table {
            self.drawing_sheet_revision
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn drawing_sheet_revision_usage_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<DrawingSheetRevisionUsage>> + 'table {
            self.drawing_sheet_revision_usage
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn ellipse_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<Ellipse>> + 'table {
            self.ellipse
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn external_source_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ExternalSource>> + 'table {
            self.external_source
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn externally_defined_curve_font_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ExternallyDefinedCurveFont>> + 'table {
            self.externally_defined_curve_font
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn externally_defined_hatch_style_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ExternallyDefinedHatchStyle>> + 'table {
            self.externally_defined_hatch_style
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn externally_defined_item_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ExternallyDefinedItem>> + 'table {
            self.externally_defined_item
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn externally_defined_symbol_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ExternallyDefinedSymbol>> + 'table {
            self.externally_defined_symbol
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn externally_defined_text_font_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ExternallyDefinedTextFont>> + 'table {
            self.externally_defined_text_font
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn externally_defined_tile_style_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ExternallyDefinedTileStyle>> + 'table {
            self.externally_defined_tile_style
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn fill_area_style_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<FillAreaStyle>> + 'table {
            self.fill_area_style
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn fill_area_style_colour_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<FillAreaStyleColour>> + 'table {
            self.fill_area_style_colour
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn fill_area_style_hatching_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<FillAreaStyleHatching>> + 'table {
            self.fill_area_style_hatching
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn fill_area_style_tile_symbol_with_style_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<FillAreaStyleTileSymbolWithStyle>> + 'table {
            self.fill_area_style_tile_symbol_with_style
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn fill_area_style_tiles_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<FillAreaStyleTiles>> + 'table {
            self.fill_area_style_tiles
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn geometric_curve_set_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<GeometricCurveSet>> + 'table {
            self.geometric_curve_set
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn geometric_representation_context_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<GeometricRepresentationContext>> + 'table {
            self.geometric_representation_context
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn geometric_representation_item_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<GeometricRepresentationItem>> + 'table {
            self.geometric_representation_item
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn geometric_set_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<GeometricSet>> + 'table {
            self.geometric_set
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn geometrical_tolerance_callout_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<GeometricalToleranceCallout>> + 'table {
            self.geometrical_tolerance_callout
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn geometrically_bounded_2d_wireframe_representation_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<GeometricallyBounded2DWireframeRepresentation>> + 'table
        {
            self.geometrically_bounded_2d_wireframe_representation
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn global_unit_assigned_context_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<GlobalUnitAssignedContext>> + 'table {
            self.global_unit_assigned_context
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn group_iter<'table>(&'table self) -> impl Iterator<Item = Result<Group>> + 'table {
            self.group
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn group_assignment_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<GroupAssignment>> + 'table {
            self.group_assignment
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn group_relationship_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<GroupRelationship>> + 'table {
            self.group_relationship
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn hyperbola_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<Hyperbola>> + 'table {
            self.hyperbola
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn invisibility_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<Invisibility>> + 'table {
            self.invisibility
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn leader_curve_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<LeaderCurve>> + 'table {
            self.leader_curve
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn leader_directed_callout_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<LeaderDirectedCallout>> + 'table {
            self.leader_directed_callout
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn leader_directed_dimension_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<LeaderDirectedDimension>> + 'table {
            self.leader_directed_dimension
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn leader_terminator_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<LeaderTerminator>> + 'table {
            self.leader_terminator
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn length_measure_with_unit_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<LengthMeasureWithUnit>> + 'table {
            self.length_measure_with_unit
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn length_unit_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<LengthUnit>> + 'table {
            self.length_unit
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn line_iter<'table>(&'table self) -> impl Iterator<Item = Result<Line>> + 'table {
            self.line
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn linear_dimension_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<LinearDimension>> + 'table {
            self.linear_dimension
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn mapped_item_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<MappedItem>> + 'table {
            self.mapped_item
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn measure_with_unit_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<MeasureWithUnit>> + 'table {
            self.measure_with_unit
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn named_unit_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<NamedUnit>> + 'table {
            self.named_unit
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn offset_curve_2d_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<OffsetCurve2D>> + 'table {
            self.offset_curve_2d
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn one_direction_repeat_factor_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<OneDirectionRepeatFactor>> + 'table {
            self.one_direction_repeat_factor
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn ordinate_dimension_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<OrdinateDimension>> + 'table {
            self.ordinate_dimension
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn organization_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<Organization>> + 'table {
            self.organization
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn organization_assignment_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<OrganizationAssignment>> + 'table {
            self.organization_assignment
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn organization_role_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<OrganizationRole>> + 'table {
            self.organization_role
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn organizational_address_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<OrganizationalAddress>> + 'table {
            self.organizational_address
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn parabola_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<Parabola>> + 'table {
            self.parabola
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn person_iter<'table>(&'table self) -> impl Iterator<Item = Result<Person>> + 'table {
            self.person
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn person_and_organization_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<PersonAndOrganization>> + 'table {
            self.person_and_organization
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn person_and_organization_assignment_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<PersonAndOrganizationAssignment>> + 'table {
            self.person_and_organization_assignment
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn person_and_organization_role_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<PersonAndOrganizationRole>> + 'table {
            self.person_and_organization_role
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn person_assignment_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<PersonAssignment>> + 'table {
            self.person_assignment
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn person_role_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<PersonRole>> + 'table {
            self.person_role
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn personal_address_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<PersonalAddress>> + 'table {
            self.personal_address
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn placement_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<Placement>> + 'table {
            self.placement
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn planar_box_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<PlanarBox>> + 'table {
            self.planar_box
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn planar_extent_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<PlanarExtent>> + 'table {
            self.planar_extent
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn plane_angle_measure_with_unit_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<PlaneAngleMeasureWithUnit>> + 'table {
            self.plane_angle_measure_with_unit
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn plane_angle_unit_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<PlaneAngleUnit>> + 'table {
            self.plane_angle_unit
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn point_iter<'table>(&'table self) -> impl Iterator<Item = Result<Point>> + 'table {
            self.point
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn point_on_curve_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<PointOnCurve>> + 'table {
            self.point_on_curve
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn polyline_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<Polyline>> + 'table {
            self.polyline
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn pre_defined_colour_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<PreDefinedColour>> + 'table {
            self.pre_defined_colour
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn pre_defined_curve_font_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<PreDefinedCurveFont>> + 'table {
            self.pre_defined_curve_font
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn pre_defined_dimension_symbol_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<PreDefinedDimensionSymbol>> + 'table {
            self.pre_defined_dimension_symbol
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn pre_defined_geometrical_tolerance_symbol_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<PreDefinedGeometricalToleranceSymbol>> + 'table {
            self.pre_defined_geometrical_tolerance_symbol
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn pre_defined_item_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<PreDefinedItem>> + 'table {
            self.pre_defined_item
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn pre_defined_point_marker_symbol_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<PreDefinedPointMarkerSymbol>> + 'table {
            self.pre_defined_point_marker_symbol
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn pre_defined_symbol_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<PreDefinedSymbol>> + 'table {
            self.pre_defined_symbol
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn pre_defined_terminator_symbol_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<PreDefinedTerminatorSymbol>> + 'table {
            self.pre_defined_terminator_symbol
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn pre_defined_text_font_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<PreDefinedTextFont>> + 'table {
            self.pre_defined_text_font
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn presentation_area_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<PresentationArea>> + 'table {
            self.presentation_area
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn presentation_layer_assignment_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<PresentationLayerAssignment>> + 'table {
            self.presentation_layer_assignment
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn presentation_layer_usage_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<PresentationLayerUsage>> + 'table {
            self.presentation_layer_usage
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn presentation_representation_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<PresentationRepresentation>> + 'table {
            self.presentation_representation
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn presentation_set_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<PresentationSet>> + 'table {
            self.presentation_set
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn presentation_size_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<PresentationSize>> + 'table {
            self.presentation_size
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn presentation_style_assignment_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<PresentationStyleAssignment>> + 'table {
            self.presentation_style_assignment
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn presentation_style_by_context_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<PresentationStyleByContext>> + 'table {
            self.presentation_style_by_context
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn presentation_view_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<PresentationView>> + 'table {
            self.presentation_view
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn presented_item_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<PresentedItem>> + 'table {
            self.presented_item
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn presented_item_representation_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<PresentedItemRepresentation>> + 'table {
            self.presented_item_representation
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn product_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<Product>> + 'table {
            self.product
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn product_context_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ProductContext>> + 'table {
            self.product_context
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn product_definition_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ProductDefinition>> + 'table {
            self.product_definition
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn product_definition_context_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ProductDefinitionContext>> + 'table {
            self.product_definition_context
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn product_definition_formation_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ProductDefinitionFormation>> + 'table {
            self.product_definition_formation
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn product_definition_shape_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ProductDefinitionShape>> + 'table {
            self.product_definition_shape
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn projection_curve_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ProjectionCurve>> + 'table {
            self.projection_curve
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn projection_directed_callout_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ProjectionDirectedCallout>> + 'table {
            self.projection_directed_callout
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn property_definition_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<PropertyDefinition>> + 'table {
            self.property_definition
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn property_definition_representation_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<PropertyDefinitionRepresentation>> + 'table {
            self.property_definition_representation
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn quasi_uniform_curve_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<QuasiUniformCurve>> + 'table {
            self.quasi_uniform_curve
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn radius_dimension_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<RadiusDimension>> + 'table {
            self.radius_dimension
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn rational_b_spline_curve_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<RationalBSplineCurve>> + 'table {
            self.rational_b_spline_curve
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn representation_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<Representation>> + 'table {
            self.representation
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn representation_context_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<RepresentationContext>> + 'table {
            self.representation_context
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn representation_item_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<RepresentationItem>> + 'table {
            self.representation_item
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn representation_map_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<RepresentationMap>> + 'table {
            self.representation_map
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn security_classification_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<SecurityClassification>> + 'table {
            self.security_classification
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn security_classification_assignment_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<SecurityClassificationAssignment>> + 'table {
            self.security_classification_assignment
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn security_classification_level_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<SecurityClassificationLevel>> + 'table {
            self.security_classification_level
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn shape_definition_representation_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ShapeDefinitionRepresentation>> + 'table {
            self.shape_definition_representation
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn shape_representation_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ShapeRepresentation>> + 'table {
            self.shape_representation
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn si_unit_iter<'table>(&'table self) -> impl Iterator<Item = Result<SiUnit>> + 'table {
            self.si_unit
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn structured_dimension_callout_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<StructuredDimensionCallout>> + 'table {
            self.structured_dimension_callout
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn styled_item_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<StyledItem>> + 'table {
            self.styled_item
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn symbol_colour_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<SymbolColour>> + 'table {
            self.symbol_colour
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn symbol_representation_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<SymbolRepresentation>> + 'table {
            self.symbol_representation
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn symbol_representation_map_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<SymbolRepresentationMap>> + 'table {
            self.symbol_representation_map
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn symbol_style_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<SymbolStyle>> + 'table {
            self.symbol_style
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn symbol_target_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<SymbolTarget>> + 'table {
            self.symbol_target
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn terminator_symbol_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<TerminatorSymbol>> + 'table {
            self.terminator_symbol
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn text_literal_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<TextLiteral>> + 'table {
            self.text_literal
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn text_literal_with_associated_curves_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<TextLiteralWithAssociatedCurves>> + 'table {
            self.text_literal_with_associated_curves
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn text_literal_with_blanking_box_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<TextLiteralWithBlankingBox>> + 'table {
            self.text_literal_with_blanking_box
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn text_literal_with_delineation_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<TextLiteralWithDelineation>> + 'table {
            self.text_literal_with_delineation
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn text_literal_with_extent_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<TextLiteralWithExtent>> + 'table {
            self.text_literal_with_extent
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn text_style_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<TextStyle>> + 'table {
            self.text_style
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn text_style_for_defined_font_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<TextStyleForDefinedFont>> + 'table {
            self.text_style_for_defined_font
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn text_style_with_box_characteristics_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<TextStyleWithBoxCharacteristics>> + 'table {
            self.text_style_with_box_characteristics
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn text_style_with_mirror_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<TextStyleWithMirror>> + 'table {
            self.text_style_with_mirror
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn trimmed_curve_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<TrimmedCurve>> + 'table {
            self.trimmed_curve
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn two_direction_repeat_factor_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<TwoDirectionRepeatFactor>> + 'table {
            self.two_direction_repeat_factor
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn uniform_curve_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<UniformCurve>> + 'table {
            self.uniform_curve
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn vector_iter<'table>(&'table self) -> impl Iterator<Item = Result<Vector>> + 'table {
            self.vector
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
    }
    impl EntityTable<AddressHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<Address> {
            crate::tables::get_owned(self, &self.address, entity_id)
        }
        fn owned_iter<'table>(&'table self) -> Box<dyn Iterator<Item = Result<Address>> + 'table> {
            crate::tables::owned_iter(self, &self.address)
        }
    }
    impl EntityTable<AngularDimensionHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<AngularDimension> {
            crate::tables::get_owned(self, &self.angular_dimension, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<AngularDimension>> + 'table> {
            crate::tables::owned_iter(self, &self.angular_dimension)
        }
    }
    impl EntityTable<AnnotationCurveOccurrenceHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<AnnotationCurveOccurrence> {
            crate::tables::get_owned(self, &self.annotation_curve_occurrence, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<AnnotationCurveOccurrence>> + 'table> {
            crate::tables::owned_iter(self, &self.annotation_curve_occurrence)
        }
    }
    impl EntityTable<AnnotationFillAreaHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<AnnotationFillArea> {
            crate::tables::get_owned(self, &self.annotation_fill_area, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<AnnotationFillArea>> + 'table> {
            crate::tables::owned_iter(self, &self.annotation_fill_area)
        }
    }
    impl EntityTable<AnnotationFillAreaOccurrenceHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<AnnotationFillAreaOccurrence> {
            crate::tables::get_owned(self, &self.annotation_fill_area_occurrence, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<AnnotationFillAreaOccurrence>> + 'table> {
            crate::tables::owned_iter(self, &self.annotation_fill_area_occurrence)
        }
    }
    impl EntityTable<AnnotationOccurrenceHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<AnnotationOccurrence> {
            crate::tables::get_owned(self, &self.annotation_occurrence, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<AnnotationOccurrence>> + 'table> {
            crate::tables::owned_iter(self, &self.annotation_occurrence)
        }
    }
    impl EntityTable<AnnotationSubfigureOccurrenceHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<AnnotationSubfigureOccurrence> {
            crate::tables::get_owned(self, &self.annotation_subfigure_occurrence, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<AnnotationSubfigureOccurrence>> + 'table> {
            crate::tables::owned_iter(self, &self.annotation_subfigure_occurrence)
        }
    }
    impl EntityTable<AnnotationSymbolHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<AnnotationSymbol> {
            crate::tables::get_owned(self, &self.annotation_symbol, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<AnnotationSymbol>> + 'table> {
            crate::tables::owned_iter(self, &self.annotation_symbol)
        }
    }
    impl EntityTable<AnnotationSymbolOccurrenceHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<AnnotationSymbolOccurrence> {
            crate::tables::get_owned(self, &self.annotation_symbol_occurrence, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<AnnotationSymbolOccurrence>> + 'table> {
            crate::tables::owned_iter(self, &self.annotation_symbol_occurrence)
        }
    }
    impl EntityTable<AnnotationTextHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<AnnotationText> {
            crate::tables::get_owned(self, &self.annotation_text, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<AnnotationText>> + 'table> {
            crate::tables::owned_iter(self, &self.annotation_text)
        }
    }
    impl EntityTable<AnnotationTextOccurrenceHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<AnnotationTextOccurrence> {
            crate::tables::get_owned(self, &self.annotation_text_occurrence, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<AnnotationTextOccurrence>> + 'table> {
            crate::tables::owned_iter(self, &self.annotation_text_occurrence)
        }
    }
    impl EntityTable<ApplicationContextHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<ApplicationContext> {
            crate::tables::get_owned(self, &self.application_context, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<ApplicationContext>> + 'table> {
            crate::tables::owned_iter(self, &self.application_context)
        }
    }
    impl EntityTable<ApplicationContextElementHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<ApplicationContextElement> {
            crate::tables::get_owned(self, &self.application_context_element, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<ApplicationContextElement>> + 'table> {
            crate::tables::owned_iter(self, &self.application_context_element)
        }
    }
    impl EntityTable<ApplicationProtocolDefinitionHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<ApplicationProtocolDefinition> {
            crate::tables::get_owned(self, &self.application_protocol_definition, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<ApplicationProtocolDefinition>> + 'table> {
            crate::tables::owned_iter(self, &self.application_protocol_definition)
        }
    }
    impl EntityTable<ApprovalHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<Approval> {
            crate::tables::get_owned(self, &self.approval, entity_id)
        }
        fn owned_iter<'table>(&'table self) -> Box<dyn Iterator<Item = Result<Approval>> + 'table> {
            crate::tables::owned_iter(self, &self.approval)
        }
    }
    impl EntityTable<ApprovalAssignmentHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<ApprovalAssignment> {
            crate::tables::get_owned(self, &self.approval_assignment, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<ApprovalAssignment>> + 'table> {
            crate::tables::owned_iter(self, &self.approval_assignment)
        }
    }
    impl EntityTable<ApprovalDateTimeHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<ApprovalDateTime> {
            crate::tables::get_owned(self, &self.approval_date_time, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<ApprovalDateTime>> + 'table> {
            crate::tables::owned_iter(self, &self.approval_date_time)
        }
    }
    impl EntityTable<ApprovalPersonOrganizationHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<ApprovalPersonOrganization> {
            crate::tables::get_owned(self, &self.approval_person_organization, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<ApprovalPersonOrganization>> + 'table> {
            crate::tables::owned_iter(self, &self.approval_person_organization)
        }
    }
    impl EntityTable<ApprovalRoleHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<ApprovalRole> {
            crate::tables::get_owned(self, &self.approval_role, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<ApprovalRole>> + 'table> {
            crate::tables::owned_iter(self, &self.approval_role)
        }
    }
    impl EntityTable<ApprovalStatusHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<ApprovalStatus> {
            crate::tables::get_owned(self, &self.approval_status, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<ApprovalStatus>> + 'table> {
            crate::tables::owned_iter(self, &self.approval_status)
        }
    }
    impl EntityTable<AreaInSetHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<AreaInSet> {
            crate::tables::get_owned(self, &self.area_in_set, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<AreaInSet>> + 'table> {
            crate::tables::owned_iter(self, &self.area_in_set)
        }
    }
    impl EntityTable<Axis2Placement2DHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<Axis2Placement2D> {
            crate::tables::get_owned(self, &self.axis2_placement_2d, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<Axis2Placement2D>> + 'table> {
            crate::tables::owned_iter(self, &self.axis2_placement_2d)
        }
    }
    impl EntityTable<BSplineCurveHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<BSplineCurve> {
            crate::tables::get_owned(self, &self.b_spline_curve, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<BSplineCurve>> + 'table> {
            crate::tables::owned_iter(self, &self.b_spline_curve)
        }
    }
    impl EntityTable<BSplineCurveWithKnotsHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<BSplineCurveWithKnots> {
            crate::tables::get_owned(self, &self.b_spline_curve_with_knots, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<BSplineCurveWithKnots>> + 'table> {
            crate::tables::owned_iter(self, &self.b_spline_curve_with_knots)
        }
    }
    impl EntityTable<BezierCurveHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<BezierCurve> {
            crate::tables::get_owned(self, &self.bezier_curve, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<BezierCurve>> + 'table> {
            crate::tables::owned_iter(self, &self.bezier_curve)
        }
    }
    impl EntityTable<BoundedCurveHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<BoundedCurve> {
            crate::tables::get_owned(self, &self.bounded_curve, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<BoundedCurve>> + 'table> {
            crate::tables::owned_iter(self, &self.bounded_curve)
        }
    }
    impl EntityTable<CalendarDateHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<CalendarDate> {
            crate::tables::get_owned(self, &self.calendar_date, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<CalendarDate>> + 'table> {
            crate::tables::owned_iter(self, &self.calendar_date)
        }
    }
    impl EntityTable<CameraImageHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<CameraImage> {
            crate::tables::get_owned(self, &self.camera_image, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<CameraImage>> + 'table> {
            crate::tables::owned_iter(self, &self.camera_image)
        }
    }
    impl EntityTable<CameraImage2DWithScaleHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<CameraImage2DWithScale> {
            crate::tables::get_owned(self, &self.camera_image_2d_with_scale, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<CameraImage2DWithScale>> + 'table> {
            crate::tables::owned_iter(self, &self.camera_image_2d_with_scale)
        }
    }
    impl EntityTable<CameraModelHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<CameraModel> {
            crate::tables::get_owned(self, &self.camera_model, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<CameraModel>> + 'table> {
            crate::tables::owned_iter(self, &self.camera_model)
        }
    }
    impl EntityTable<CameraModelD2Holder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<CameraModelD2> {
            crate::tables::get_owned(self, &self.camera_model_d2, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<CameraModelD2>> + 'table> {
            crate::tables::owned_iter(self, &self.camera_model_d2)
        }
    }
    impl EntityTable<CameraUsageHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<CameraUsage> {
            crate::tables::get_owned(self, &self.camera_usage, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<CameraUsage>> + 'table> {
            crate::tables::owned_iter(self, &self.camera_usage)
        }
    }
    impl EntityTable<CartesianPointHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<CartesianPoint> {
            crate::tables::get_owned(self, &self.cartesian_point, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<CartesianPoint>> + 'table> {
            crate::tables::owned_iter(self, &self.cartesian_point)
        }
    }
    impl EntityTable<CircleHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<Circle> {
            crate::tables::get_owned(self, &self.circle, entity_id)
        }
        fn owned_iter<'table>(&'table self) -> Box<dyn Iterator<Item = Result<Circle>> + 'table> {
            crate::tables::owned_iter(self, &self.circle)
        }
    }
    impl EntityTable<ColourHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<Colour> {
            crate::tables::get_owned(self, &self.colour, entity_id)
        }
        fn owned_iter<'table>(&'table self) -> Box<dyn Iterator<Item = Result<Colour>> + 'table> {
            crate::tables::owned_iter(self, &self.colour)
        }
    }
    impl EntityTable<ColourRgbHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<ColourRgb> {
            crate::tables::get_owned(self, &self.colour_rgb, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<ColourRgb>> + 'table> {
            crate::tables::owned_iter(self, &self.colour_rgb)
        }
    }
    impl EntityTable<ColourSpecificationHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<ColourSpecification> {
            crate::tables::get_owned(self, &self.colour_specification, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<ColourSpecification>> + 'table> {
            crate::tables::owned_iter(self, &self.colour_specification)
        }
    }
    impl EntityTable<CompositeCurveHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<CompositeCurve> {
            crate::tables::get_owned(self, &self.composite_curve, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<CompositeCurve>> + 'table> {
            crate::tables::owned_iter(self, &self.composite_curve)
        }
    }
    impl EntityTable<CompositeCurveSegmentHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<CompositeCurveSegment> {
            crate::tables::get_owned(self, &self.composite_curve_segment, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<CompositeCurveSegment>> + 'table> {
            crate::tables::owned_iter(self, &self.composite_curve_segment)
        }
    }
    impl EntityTable<CompositeTextHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<CompositeText> {
            crate::tables::get_owned(self, &self.composite_text, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<CompositeText>> + 'table> {
            crate::tables::owned_iter(self, &self.composite_text)
        }
    }
    impl EntityTable<CompositeTextWithAssociatedCurvesHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<CompositeTextWithAssociatedCurves> {
            crate::tables::get_owned(self, &self.composite_text_with_associated_curves, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<CompositeTextWithAssociatedCurves>> + 'table> {
            crate::tables::owned_iter(self, &self.composite_text_with_associated_curves)
        }
    }
    impl EntityTable<CompositeTextWithBlankingBoxHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<CompositeTextWithBlankingBox> {
            crate::tables::get_owned(self, &self.composite_text_with_blanking_box, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<CompositeTextWithBlankingBox>> + 'table> {
            crate::tables::owned_iter(self, &self.composite_text_with_blanking_box)
        }
    }
    impl EntityTable<CompositeTextWithExtentHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<CompositeTextWithExtent> {
            crate::tables::get_owned(self, &self.composite_text_with_extent, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<CompositeTextWithExtent>> + 'table> {
            crate::tables::owned_iter(self, &self.composite_text_with_extent)
        }
    }
    impl EntityTable<ConicHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<Conic> {
            crate::tables::get_owned(self, &self.conic, entity_id)
        }
        fn owned_iter<'table>(&'table self) -> Box<dyn Iterator<Item = Result<Conic>> + 'table> {
            crate::tables::owned_iter(self, &self.conic)
        }
    }
    impl EntityTable<ContextDependentInvisibilityHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<ContextDependentInvisibility> {
            crate::tables::get_owned(self, &self.context_dependent_invisibility, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<ContextDependentInvisibility>> + 'table> {
            crate::tables::owned_iter(self, &self.context_dependent_invisibility)
        }
    }
    impl EntityTable<ContractHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<Contract> {
            crate::tables::get_owned(self, &self.contract, entity_id)
        }
        fn owned_iter<'table>(&'table self) -> Box<dyn Iterator<Item = Result<Contract>> + 'table> {
            crate::tables::owned_iter(self, &self.contract)
        }
    }
    impl EntityTable<ContractAssignmentHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<ContractAssignment> {
            crate::tables::get_owned(self, &self.contract_assignment, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<ContractAssignment>> + 'table> {
            crate::tables::owned_iter(self, &self.contract_assignment)
        }
    }
    impl EntityTable<ContractTypeHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<ContractType> {
            crate::tables::get_owned(self, &self.contract_type, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<ContractType>> + 'table> {
            crate::tables::owned_iter(self, &self.contract_type)
        }
    }
    impl EntityTable<ConversionBasedUnitHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<ConversionBasedUnit> {
            crate::tables::get_owned(self, &self.conversion_based_unit, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<ConversionBasedUnit>> + 'table> {
            crate::tables::owned_iter(self, &self.conversion_based_unit)
        }
    }
    impl EntityTable<CurveHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<Curve> {
            crate::tables::get_owned(self, &self.curve, entity_id)
        }
        fn owned_iter<'table>(&'table self) -> Box<dyn Iterator<Item = Result<Curve>> + 'table> {
            crate::tables::owned_iter(self, &self.curve)
        }
    }
    impl EntityTable<CurveDimensionHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<CurveDimension> {
            crate::tables::get_owned(self, &self.curve_dimension, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<CurveDimension>> + 'table> {
            crate::tables::owned_iter(self, &self.curve_dimension)
        }
    }
    impl EntityTable<CurveStyleHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<CurveStyle> {
            crate::tables::get_owned(self, &self.curve_style, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<CurveStyle>> + 'table> {
            crate::tables::owned_iter(self, &self.curve_style)
        }
    }
    impl EntityTable<CurveStyleFontHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<CurveStyleFont> {
            crate::tables::get_owned(self, &self.curve_style_font, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<CurveStyleFont>> + 'table> {
            crate::tables::owned_iter(self, &self.curve_style_font)
        }
    }
    impl EntityTable<CurveStyleFontPatternHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<CurveStyleFontPattern> {
            crate::tables::get_owned(self, &self.curve_style_font_pattern, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<CurveStyleFontPattern>> + 'table> {
            crate::tables::owned_iter(self, &self.curve_style_font_pattern)
        }
    }
    impl EntityTable<DateHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<Date> {
            crate::tables::get_owned(self, &self.date, entity_id)
        }
        fn owned_iter<'table>(&'table self) -> Box<dyn Iterator<Item = Result<Date>> + 'table> {
            crate::tables::owned_iter(self, &self.date)
        }
    }
    impl EntityTable<DatumFeatureCalloutHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<DatumFeatureCallout> {
            crate::tables::get_owned(self, &self.datum_feature_callout, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<DatumFeatureCallout>> + 'table> {
            crate::tables::owned_iter(self, &self.datum_feature_callout)
        }
    }
    impl EntityTable<DatumTargetCalloutHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<DatumTargetCallout> {
            crate::tables::get_owned(self, &self.datum_target_callout, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<DatumTargetCallout>> + 'table> {
            crate::tables::owned_iter(self, &self.datum_target_callout)
        }
    }
    impl EntityTable<DefinedSymbolHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<DefinedSymbol> {
            crate::tables::get_owned(self, &self.defined_symbol, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<DefinedSymbol>> + 'table> {
            crate::tables::owned_iter(self, &self.defined_symbol)
        }
    }
    impl EntityTable<DiameterDimensionHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<DiameterDimension> {
            crate::tables::get_owned(self, &self.diameter_dimension, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<DiameterDimension>> + 'table> {
            crate::tables::owned_iter(self, &self.diameter_dimension)
        }
    }
    impl EntityTable<DimensionCalloutComponentRelationshipHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<DimensionCalloutComponentRelationship> {
            crate::tables::get_owned(
                self,
                &self.dimension_callout_component_relationship,
                entity_id,
            )
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<DimensionCalloutComponentRelationship>> + 'table>
        {
            crate::tables::owned_iter(self, &self.dimension_callout_component_relationship)
        }
    }
    impl EntityTable<DimensionCalloutRelationshipHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<DimensionCalloutRelationship> {
            crate::tables::get_owned(self, &self.dimension_callout_relationship, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<DimensionCalloutRelationship>> + 'table> {
            crate::tables::owned_iter(self, &self.dimension_callout_relationship)
        }
    }
    impl EntityTable<DimensionCurveHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<DimensionCurve> {
            crate::tables::get_owned(self, &self.dimension_curve, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<DimensionCurve>> + 'table> {
            crate::tables::owned_iter(self, &self.dimension_curve)
        }
    }
    impl EntityTable<DimensionCurveDirectedCalloutHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<DimensionCurveDirectedCallout> {
            crate::tables::get_owned(self, &self.dimension_curve_directed_callout, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<DimensionCurveDirectedCallout>> + 'table> {
            crate::tables::owned_iter(self, &self.dimension_curve_directed_callout)
        }
    }
    impl EntityTable<DimensionCurveTerminatorHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<DimensionCurveTerminator> {
            crate::tables::get_owned(self, &self.dimension_curve_terminator, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<DimensionCurveTerminator>> + 'table> {
            crate::tables::owned_iter(self, &self.dimension_curve_terminator)
        }
    }
    impl EntityTable<DimensionPairHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<DimensionPair> {
            crate::tables::get_owned(self, &self.dimension_pair, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<DimensionPair>> + 'table> {
            crate::tables::owned_iter(self, &self.dimension_pair)
        }
    }
    impl EntityTable<DimensionalExponentsHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<DimensionalExponents> {
            crate::tables::get_owned(self, &self.dimensional_exponents, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<DimensionalExponents>> + 'table> {
            crate::tables::owned_iter(self, &self.dimensional_exponents)
        }
    }
    impl EntityTable<DirectionHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<Direction> {
            crate::tables::get_owned(self, &self.direction, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<Direction>> + 'table> {
            crate::tables::owned_iter(self, &self.direction)
        }
    }
    impl EntityTable<DocumentHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<Document> {
            crate::tables::get_owned(self, &self.document, entity_id)
        }
        fn owned_iter<'table>(&'table self) -> Box<dyn Iterator<Item = Result<Document>> + 'table> {
            crate::tables::owned_iter(self, &self.document)
        }
    }
    impl EntityTable<DocumentReferenceHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<DocumentReference> {
            crate::tables::get_owned(self, &self.document_reference, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<DocumentReference>> + 'table> {
            crate::tables::owned_iter(self, &self.document_reference)
        }
    }
    impl EntityTable<DocumentTypeHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<DocumentType> {
            crate::tables::get_owned(self, &self.document_type, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<DocumentType>> + 'table> {
            crate::tables::owned_iter(self, &self.document_type)
        }
    }
    impl EntityTable<DraughtingAnnotationOccurrenceHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<DraughtingAnnotationOccurrence> {
            crate::tables::get_owned(self, &self.draughting_annotation_occurrence, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<DraughtingAnnotationOccurrence>> + 'table> {
            crate::tables::owned_iter(self, &self.draughting_annotation_occurrence)
        }
    }
    impl EntityTable<DraughtingApprovalAssignmentHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<DraughtingApprovalAssignment> {
            crate::tables::get_owned(self, &self.draughting_approval_assignment, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<DraughtingApprovalAssignment>> + 'table> {
            crate::tables::owned_iter(self, &self.draughting_approval_assignment)
        }
    }
    impl EntityTable<DraughtingCalloutHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<DraughtingCallout> {
            crate::tables::get_owned(self, &self.draughting_callout, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<DraughtingCallout>> + 'table> {
            crate::tables::owned_iter(self, &self.draughting_callout)
        }
    }
    impl EntityTable<DraughtingCalloutRelationshipHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<DraughtingCalloutRelationship> {
            crate::tables::get_owned(self, &self.draughting_callout_relationship, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<DraughtingCalloutRelationship>> + 'table> {
            crate::tables::owned_iter(self, &self.draughting_callout_relationship)
        }
    }
    impl EntityTable<DraughtingContractAssignmentHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<DraughtingContractAssignment> {
            crate::tables::get_owned(self, &self.draughting_contract_assignment, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<DraughtingContractAssignment>> + 'table> {
            crate::tables::owned_iter(self, &self.draughting_contract_assignment)
        }
    }
    impl EntityTable<DraughtingDrawingRevisionHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<DraughtingDrawingRevision> {
            crate::tables::get_owned(self, &self.draughting_drawing_revision, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<DraughtingDrawingRevision>> + 'table> {
            crate::tables::owned_iter(self, &self.draughting_drawing_revision)
        }
    }
    impl EntityTable<DraughtingElementsHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<DraughtingElements> {
            crate::tables::get_owned(self, &self.draughting_elements, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<DraughtingElements>> + 'table> {
            crate::tables::owned_iter(self, &self.draughting_elements)
        }
    }
    impl EntityTable<DraughtingGroupAssignmentHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<DraughtingGroupAssignment> {
            crate::tables::get_owned(self, &self.draughting_group_assignment, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<DraughtingGroupAssignment>> + 'table> {
            crate::tables::owned_iter(self, &self.draughting_group_assignment)
        }
    }
    impl EntityTable<DraughtingModelHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<DraughtingModel> {
            crate::tables::get_owned(self, &self.draughting_model, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<DraughtingModel>> + 'table> {
            crate::tables::owned_iter(self, &self.draughting_model)
        }
    }
    impl EntityTable<DraughtingOrganizationAssignmentHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<DraughtingOrganizationAssignment> {
            crate::tables::get_owned(self, &self.draughting_organization_assignment, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<DraughtingOrganizationAssignment>> + 'table> {
            crate::tables::owned_iter(self, &self.draughting_organization_assignment)
        }
    }
    impl EntityTable<DraughtingPersonAndOrganizationAssignmentHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<DraughtingPersonAndOrganizationAssignment> {
            crate::tables::get_owned(
                self,
                &self.draughting_person_and_organization_assignment,
                entity_id,
            )
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<DraughtingPersonAndOrganizationAssignment>> + 'table>
        {
            crate::tables::owned_iter(self, &self.draughting_person_and_organization_assignment)
        }
    }
    impl EntityTable<DraughtingPersonAssignmentHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<DraughtingPersonAssignment> {
            crate::tables::get_owned(self, &self.draughting_person_assignment, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<DraughtingPersonAssignment>> + 'table> {
            crate::tables::owned_iter(self, &self.draughting_person_assignment)
        }
    }
    impl EntityTable<DraughtingPreDefinedColourHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<DraughtingPreDefinedColour> {
            crate::tables::get_owned(self, &self.draughting_pre_defined_colour, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<DraughtingPreDefinedColour>> + 'table> {
            crate::tables::owned_iter(self, &self.draughting_pre_defined_colour)
        }
    }
    impl EntityTable<DraughtingPreDefinedCurveFontHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<DraughtingPreDefinedCurveFont> {
            crate::tables::get_owned(self, &self.draughting_pre_defined_curve_font, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<DraughtingPreDefinedCurveFont>> + 'table> {
            crate::tables::owned_iter(self, &self.draughting_pre_defined_curve_font)
        }
    }
    impl EntityTable<DraughtingPreDefinedTextFontHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<DraughtingPreDefinedTextFont> {
            crate::tables::get_owned(self, &self.draughting_pre_defined_text_font, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<DraughtingPreDefinedTextFont>> + 'table> {
            crate::tables::owned_iter(self, &self.draughting_pre_defined_text_font)
        }
    }
    impl EntityTable<DraughtingPresentedItemHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<DraughtingPresentedItem> {
            crate::tables::get_owned(self, &self.draughting_presented_item, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<DraughtingPresentedItem>> + 'table> {
            crate::tables::owned_iter(self, &self.draughting_presented_item)
        }
    }
    impl EntityTable<DraughtingSecurityClassificationAssignmentHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<DraughtingSecurityClassificationAssignment> {
            crate::tables::get_owned(
                self,
                &self.draughting_security_classification_assignment,
                entity_id,
            )
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<DraughtingSecurityClassificationAssignment>> + 'table>
        {
            crate::tables::owned_iter(self, &self.draughting_security_classification_assignment)
        }
    }
    impl EntityTable<DraughtingSpecificationReferenceHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<DraughtingSpecificationReference> {
            crate::tables::get_owned(self, &self.draughting_specification_reference, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<DraughtingSpecificationReference>> + 'table> {
            crate::tables::owned_iter(self, &self.draughting_specification_reference)
        }
    }
    impl EntityTable<DraughtingSubfigureRepresentationHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<DraughtingSubfigureRepresentation> {
            crate::tables::get_owned(self, &self.draughting_subfigure_representation, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<DraughtingSubfigureRepresentation>> + 'table> {
            crate::tables::owned_iter(self, &self.draughting_subfigure_representation)
        }
    }
    impl EntityTable<DraughtingSymbolRepresentationHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<DraughtingSymbolRepresentation> {
            crate::tables::get_owned(self, &self.draughting_symbol_representation, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<DraughtingSymbolRepresentation>> + 'table> {
            crate::tables::owned_iter(self, &self.draughting_symbol_representation)
        }
    }
    impl EntityTable<DraughtingTextLiteralWithDelineationHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<DraughtingTextLiteralWithDelineation> {
            crate::tables::get_owned(
                self,
                &self.draughting_text_literal_with_delineation,
                entity_id,
            )
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<DraughtingTextLiteralWithDelineation>> + 'table>
        {
            crate::tables::owned_iter(self, &self.draughting_text_literal_with_delineation)
        }
    }
    impl EntityTable<DraughtingTitleHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<DraughtingTitle> {
            crate::tables::get_owned(self, &self.draughting_title, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<DraughtingTitle>> + 'table> {
            crate::tables::owned_iter(self, &self.draughting_title)
        }
    }
    impl EntityTable<DrawingDefinitionHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<DrawingDefinition> {
            crate::tables::get_owned(self, &self.drawing_definition, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<DrawingDefinition>> + 'table> {
            crate::tables::owned_iter(self, &self.drawing_definition)
        }
    }
    impl EntityTable<DrawingRevisionHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<DrawingRevision> {
            crate::tables::get_owned(self, &self.drawing_revision, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<DrawingRevision>> + 'table> {
            crate::tables::owned_iter(self, &self.drawing_revision)
        }
    }
    impl EntityTable<DrawingSheetLayoutHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<DrawingSheetLayout> {
            crate::tables::get_owned(self, &self.drawing_sheet_layout, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<DrawingSheetLayout>> + 'table> {
            crate::tables::owned_iter(self, &self.drawing_sheet_layout)
        }
    }
    impl EntityTable<DrawingSheetRevisionHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<DrawingSheetRevision> {
            crate::tables::get_owned(self, &self.drawing_sheet_revision, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<DrawingSheetRevision>> + 'table> {
            crate::tables::owned_iter(self, &self.drawing_sheet_revision)
        }
    }
    impl EntityTable<DrawingSheetRevisionUsageHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<DrawingSheetRevisionUsage> {
            crate::tables::get_owned(self, &self.drawing_sheet_revision_usage, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<DrawingSheetRevisionUsage>> + 'table> {
            crate::tables::owned_iter(self, &self.drawing_sheet_revision_usage)
        }
    }
    impl EntityTable<EllipseHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<Ellipse> {
            crate::tables::get_owned(self, &self.ellipse, entity_id)
        }
        fn owned_iter<'table>(&'table self) -> Box<dyn Iterator<Item = Result<Ellipse>> + 'table> {
            crate::tables::owned_iter(self, &self.ellipse)
        }
    }
    impl EntityTable<ExternalSourceHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<ExternalSource> {
            crate::tables::get_owned(self, &self.external_source, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<ExternalSource>> + 'table> {
            crate::tables::owned_iter(self, &self.external_source)
        }
    }
    impl EntityTable<ExternallyDefinedCurveFontHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<ExternallyDefinedCurveFont> {
            crate::tables::get_owned(self, &self.externally_defined_curve_font, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<ExternallyDefinedCurveFont>> + 'table> {
            crate::tables::owned_iter(self, &self.externally_defined_curve_font)
        }
    }
    impl EntityTable<ExternallyDefinedHatchStyleHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<ExternallyDefinedHatchStyle> {
            crate::tables::get_owned(self, &self.externally_defined_hatch_style, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<ExternallyDefinedHatchStyle>> + 'table> {
            crate::tables::owned_iter(self, &self.externally_defined_hatch_style)
        }
    }
    impl EntityTable<ExternallyDefinedItemHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<ExternallyDefinedItem> {
            crate::tables::get_owned(self, &self.externally_defined_item, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<ExternallyDefinedItem>> + 'table> {
            crate::tables::owned_iter(self, &self.externally_defined_item)
        }
    }
    impl EntityTable<ExternallyDefinedSymbolHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<ExternallyDefinedSymbol> {
            crate::tables::get_owned(self, &self.externally_defined_symbol, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<ExternallyDefinedSymbol>> + 'table> {
            crate::tables::owned_iter(self, &self.externally_defined_symbol)
        }
    }
    impl EntityTable<ExternallyDefinedTextFontHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<ExternallyDefinedTextFont> {
            crate::tables::get_owned(self, &self.externally_defined_text_font, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<ExternallyDefinedTextFont>> + 'table> {
            crate::tables::owned_iter(self, &self.externally_defined_text_font)
        }
    }
    impl EntityTable<ExternallyDefinedTileStyleHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<ExternallyDefinedTileStyle> {
            crate::tables::get_owned(self, &self.externally_defined_tile_style, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<ExternallyDefinedTileStyle>> + 'table> {
            crate::tables::owned_iter(self, &self.externally_defined_tile_style)
        }
    }
    impl EntityTable<FillAreaStyleHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<FillAreaStyle> {
            crate::tables::get_owned(self, &self.fill_area_style, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<FillAreaStyle>> + 'table> {
            crate::tables::owned_iter(self, &self.fill_area_style)
        }
    }
    impl EntityTable<FillAreaStyleColourHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<FillAreaStyleColour> {
            crate::tables::get_owned(self, &self.fill_area_style_colour, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<FillAreaStyleColour>> + 'table> {
            crate::tables::owned_iter(self, &self.fill_area_style_colour)
        }
    }
    impl EntityTable<FillAreaStyleHatchingHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<FillAreaStyleHatching> {
            crate::tables::get_owned(self, &self.fill_area_style_hatching, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<FillAreaStyleHatching>> + 'table> {
            crate::tables::owned_iter(self, &self.fill_area_style_hatching)
        }
    }
    impl EntityTable<FillAreaStyleTileSymbolWithStyleHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<FillAreaStyleTileSymbolWithStyle> {
            crate::tables::get_owned(
                self,
                &self.fill_area_style_tile_symbol_with_style,
                entity_id,
            )
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<FillAreaStyleTileSymbolWithStyle>> + 'table> {
            crate::tables::owned_iter(self, &self.fill_area_style_tile_symbol_with_style)
        }
    }
    impl EntityTable<FillAreaStyleTilesHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<FillAreaStyleTiles> {
            crate::tables::get_owned(self, &self.fill_area_style_tiles, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<FillAreaStyleTiles>> + 'table> {
            crate::tables::owned_iter(self, &self.fill_area_style_tiles)
        }
    }
    impl EntityTable<GeometricCurveSetHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<GeometricCurveSet> {
            crate::tables::get_owned(self, &self.geometric_curve_set, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<GeometricCurveSet>> + 'table> {
            crate::tables::owned_iter(self, &self.geometric_curve_set)
        }
    }
    impl EntityTable<GeometricRepresentationContextHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<GeometricRepresentationContext> {
            crate::tables::get_owned(self, &self.geometric_representation_context, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<GeometricRepresentationContext>> + 'table> {
            crate::tables::owned_iter(self, &self.geometric_representation_context)
        }
    }
    impl EntityTable<GeometricRepresentationItemHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<GeometricRepresentationItem> {
            crate::tables::get_owned(self, &self.geometric_representation_item, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<GeometricRepresentationItem>> + 'table> {
            crate::tables::owned_iter(self, &self.geometric_representation_item)
        }
    }
    impl EntityTable<GeometricSetHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<GeometricSet> {
            crate::tables::get_owned(self, &self.geometric_set, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<GeometricSet>> + 'table> {
            crate::tables::owned_iter(self, &self.geometric_set)
        }
    }
    impl EntityTable<GeometricalToleranceCalloutHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<GeometricalToleranceCallout> {
            crate::tables::get_owned(self, &self.geometrical_tolerance_callout, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<GeometricalToleranceCallout>> + 'table> {
            crate::tables::owned_iter(self, &self.geometrical_tolerance_callout)
        }
    }
    impl EntityTable<GeometricallyBounded2DWireframeRepresentationHolder> for Tables {
        fn get_owned(
            &self,
            entity_id: u64,
        ) -> Result<GeometricallyBounded2DWireframeRepresentation> {
            crate::tables::get_owned(
                self,
                &self.geometrically_bounded_2d_wireframe_representation,
                entity_id,
            )
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<GeometricallyBounded2DWireframeRepresentation>> + 'table>
        {
            crate::tables::owned_iter(
                self,
                &self.geometrically_bounded_2d_wireframe_representation,
            )
        }
    }
    impl EntityTable<GlobalUnitAssignedContextHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<GlobalUnitAssignedContext> {
            crate::tables::get_owned(self, &self.global_unit_assigned_context, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<GlobalUnitAssignedContext>> + 'table> {
            crate::tables::owned_iter(self, &self.global_unit_assigned_context)
        }
    }
    impl EntityTable<GroupHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<Group> {
            crate::tables::get_owned(self, &self.group, entity_id)
        }
        fn owned_iter<'table>(&'table self) -> Box<dyn Iterator<Item = Result<Group>> + 'table> {
            crate::tables::owned_iter(self, &self.group)
        }
    }
    impl EntityTable<GroupAssignmentHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<GroupAssignment> {
            crate::tables::get_owned(self, &self.group_assignment, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<GroupAssignment>> + 'table> {
            crate::tables::owned_iter(self, &self.group_assignment)
        }
    }
    impl EntityTable<GroupRelationshipHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<GroupRelationship> {
            crate::tables::get_owned(self, &self.group_relationship, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<GroupRelationship>> + 'table> {
            crate::tables::owned_iter(self, &self.group_relationship)
        }
    }
    impl EntityTable<HyperbolaHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<Hyperbola> {
            crate::tables::get_owned(self, &self.hyperbola, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<Hyperbola>> + 'table> {
            crate::tables::owned_iter(self, &self.hyperbola)
        }
    }
    impl EntityTable<InvisibilityHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<Invisibility> {
            crate::tables::get_owned(self, &self.invisibility, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<Invisibility>> + 'table> {
            crate::tables::owned_iter(self, &self.invisibility)
        }
    }
    impl EntityTable<LeaderCurveHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<LeaderCurve> {
            crate::tables::get_owned(self, &self.leader_curve, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<LeaderCurve>> + 'table> {
            crate::tables::owned_iter(self, &self.leader_curve)
        }
    }
    impl EntityTable<LeaderDirectedCalloutHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<LeaderDirectedCallout> {
            crate::tables::get_owned(self, &self.leader_directed_callout, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<LeaderDirectedCallout>> + 'table> {
            crate::tables::owned_iter(self, &self.leader_directed_callout)
        }
    }
    impl EntityTable<LeaderDirectedDimensionHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<LeaderDirectedDimension> {
            crate::tables::get_owned(self, &self.leader_directed_dimension, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<LeaderDirectedDimension>> + 'table> {
            crate::tables::owned_iter(self, &self.leader_directed_dimension)
        }
    }
    impl EntityTable<LeaderTerminatorHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<LeaderTerminator> {
            crate::tables::get_owned(self, &self.leader_terminator, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<LeaderTerminator>> + 'table> {
            crate::tables::owned_iter(self, &self.leader_terminator)
        }
    }
    impl EntityTable<LengthMeasureWithUnitHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<LengthMeasureWithUnit> {
            crate::tables::get_owned(self, &self.length_measure_with_unit, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<LengthMeasureWithUnit>> + 'table> {
            crate::tables::owned_iter(self, &self.length_measure_with_unit)
        }
    }
    impl EntityTable<LengthUnitHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<LengthUnit> {
            crate::tables::get_owned(self, &self.length_unit, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<LengthUnit>> + 'table> {
            crate::tables::owned_iter(self, &self.length_unit)
        }
    }
    impl EntityTable<LineHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<Line> {
            crate::tables::get_owned(self, &self.line, entity_id)
        }
        fn owned_iter<'table>(&'table self) -> Box<dyn Iterator<Item = Result<Line>> + 'table> {
            crate::tables::owned_iter(self, &self.line)
        }
    }
    impl EntityTable<LinearDimensionHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<LinearDimension> {
            crate::tables::get_owned(self, &self.linear_dimension, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<LinearDimension>> + 'table> {
            crate::tables::owned_iter(self, &self.linear_dimension)
        }
    }
    impl EntityTable<MappedItemHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<MappedItem> {
            crate::tables::get_owned(self, &self.mapped_item, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<MappedItem>> + 'table> {
            crate::tables::owned_iter(self, &self.mapped_item)
        }
    }
    impl EntityTable<MeasureWithUnitHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<MeasureWithUnit> {
            crate::tables::get_owned(self, &self.measure_with_unit, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<MeasureWithUnit>> + 'table> {
            crate::tables::owned_iter(self, &self.measure_with_unit)
        }
    }
    impl EntityTable<NamedUnitHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<NamedUnit> {
            crate::tables::get_owned(self, &self.named_unit, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<NamedUnit>> + 'table> {
            crate::tables::owned_iter(self, &self.named_unit)
        }
    }
    impl EntityTable<OffsetCurve2DHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<OffsetCurve2D> {
            crate::tables::get_owned(self, &self.offset_curve_2d, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<OffsetCurve2D>> + 'table> {
            crate::tables::owned_iter(self, &self.offset_curve_2d)
        }
    }
    impl EntityTable<OneDirectionRepeatFactorHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<OneDirectionRepeatFactor> {
            crate::tables::get_owned(self, &self.one_direction_repeat_factor, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<OneDirectionRepeatFactor>> + 'table> {
            crate::tables::owned_iter(self, &self.one_direction_repeat_factor)
        }
    }
    impl EntityTable<OrdinateDimensionHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<OrdinateDimension> {
            crate::tables::get_owned(self, &self.ordinate_dimension, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<OrdinateDimension>> + 'table> {
            crate::tables::owned_iter(self, &self.ordinate_dimension)
        }
    }
    impl EntityTable<OrganizationHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<Organization> {
            crate::tables::get_owned(self, &self.organization, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<Organization>> + 'table> {
            crate::tables::owned_iter(self, &self.organization)
        }
    }
    impl EntityTable<OrganizationAssignmentHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<OrganizationAssignment> {
            crate::tables::get_owned(self, &self.organization_assignment, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<OrganizationAssignment>> + 'table> {
            crate::tables::owned_iter(self, &self.organization_assignment)
        }
    }
    impl EntityTable<OrganizationRoleHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<OrganizationRole> {
            crate::tables::get_owned(self, &self.organization_role, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<OrganizationRole>> + 'table> {
            crate::tables::owned_iter(self, &self.organization_role)
        }
    }
    impl EntityTable<OrganizationalAddressHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<OrganizationalAddress> {
            crate::tables::get_owned(self, &self.organizational_address, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<OrganizationalAddress>> + 'table> {
            crate::tables::owned_iter(self, &self.organizational_address)
        }
    }
    impl EntityTable<ParabolaHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<Parabola> {
            crate::tables::get_owned(self, &self.parabola, entity_id)
        }
        fn owned_iter<'table>(&'table self) -> Box<dyn Iterator<Item = Result<Parabola>> + 'table> {
            crate::tables::owned_iter(self, &self.parabola)
        }
    }
    impl EntityTable<PersonHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<Person> {
            crate::tables::get_owned(self, &self.person, entity_id)
        }
        fn owned_iter<'table>(&'table self) -> Box<dyn Iterator<Item = Result<Person>> + 'table> {
            crate::tables::owned_iter(self, &self.person)
        }
    }
    impl EntityTable<PersonAndOrganizationHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<PersonAndOrganization> {
            crate::tables::get_owned(self, &self.person_and_organization, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<PersonAndOrganization>> + 'table> {
            crate::tables::owned_iter(self, &self.person_and_organization)
        }
    }
    impl EntityTable<PersonAndOrganizationAssignmentHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<PersonAndOrganizationAssignment> {
            crate::tables::get_owned(self, &self.person_and_organization_assignment, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<PersonAndOrganizationAssignment>> + 'table> {
            crate::tables::owned_iter(self, &self.person_and_organization_assignment)
        }
    }
    impl EntityTable<PersonAndOrganizationRoleHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<PersonAndOrganizationRole> {
            crate::tables::get_owned(self, &self.person_and_organization_role, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<PersonAndOrganizationRole>> + 'table> {
            crate::tables::owned_iter(self, &self.person_and_organization_role)
        }
    }
    impl EntityTable<PersonAssignmentHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<PersonAssignment> {
            crate::tables::get_owned(self, &self.person_assignment, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<PersonAssignment>> + 'table> {
            crate::tables::owned_iter(self, &self.person_assignment)
        }
    }
    impl EntityTable<PersonRoleHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<PersonRole> {
            crate::tables::get_owned(self, &self.person_role, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<PersonRole>> + 'table> {
            crate::tables::owned_iter(self, &self.person_role)
        }
    }
    impl EntityTable<PersonalAddressHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<PersonalAddress> {
            crate::tables::get_owned(self, &self.personal_address, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<PersonalAddress>> + 'table> {
            crate::tables::owned_iter(self, &self.personal_address)
        }
    }
    impl EntityTable<PlacementHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<Placement> {
            crate::tables::get_owned(self, &self.placement, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<Placement>> + 'table> {
            crate::tables::owned_iter(self, &self.placement)
        }
    }
    impl EntityTable<PlanarBoxHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<PlanarBox> {
            crate::tables::get_owned(self, &self.planar_box, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<PlanarBox>> + 'table> {
            crate::tables::owned_iter(self, &self.planar_box)
        }
    }
    impl EntityTable<PlanarExtentHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<PlanarExtent> {
            crate::tables::get_owned(self, &self.planar_extent, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<PlanarExtent>> + 'table> {
            crate::tables::owned_iter(self, &self.planar_extent)
        }
    }
    impl EntityTable<PlaneAngleMeasureWithUnitHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<PlaneAngleMeasureWithUnit> {
            crate::tables::get_owned(self, &self.plane_angle_measure_with_unit, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<PlaneAngleMeasureWithUnit>> + 'table> {
            crate::tables::owned_iter(self, &self.plane_angle_measure_with_unit)
        }
    }
    impl EntityTable<PlaneAngleUnitHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<PlaneAngleUnit> {
            crate::tables::get_owned(self, &self.plane_angle_unit, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<PlaneAngleUnit>> + 'table> {
            crate::tables::owned_iter(self, &self.plane_angle_unit)
        }
    }
    impl EntityTable<PointHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<Point> {
            crate::tables::get_owned(self, &self.point, entity_id)
        }
        fn owned_iter<'table>(&'table self) -> Box<dyn Iterator<Item = Result<Point>> + 'table> {
            crate::tables::owned_iter(self, &self.point)
        }
    }
    impl EntityTable<PointOnCurveHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<PointOnCurve> {
            crate::tables::get_owned(self, &self.point_on_curve, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<PointOnCurve>> + 'table> {
            crate::tables::owned_iter(self, &self.point_on_curve)
        }
    }
    impl EntityTable<PolylineHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<Polyline> {
            crate::tables::get_owned(self, &self.polyline, entity_id)
        }
        fn owned_iter<'table>(&'table self) -> Box<dyn Iterator<Item = Result<Polyline>> + 'table> {
            crate::tables::owned_iter(self, &self.polyline)
        }
    }
    impl EntityTable<PreDefinedColourHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<PreDefinedColour> {
            crate::tables::get_owned(self, &self.pre_defined_colour, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<PreDefinedColour>> + 'table> {
            crate::tables::owned_iter(self, &self.pre_defined_colour)
        }
    }
    impl EntityTable<PreDefinedCurveFontHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<PreDefinedCurveFont> {
            crate::tables::get_owned(self, &self.pre_defined_curve_font, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<PreDefinedCurveFont>> + 'table> {
            crate::tables::owned_iter(self, &self.pre_defined_curve_font)
        }
    }
    impl EntityTable<PreDefinedDimensionSymbolHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<PreDefinedDimensionSymbol> {
            crate::tables::get_owned(self, &self.pre_defined_dimension_symbol, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<PreDefinedDimensionSymbol>> + 'table> {
            crate::tables::owned_iter(self, &self.pre_defined_dimension_symbol)
        }
    }
    impl EntityTable<PreDefinedGeometricalToleranceSymbolHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<PreDefinedGeometricalToleranceSymbol> {
            crate::tables::get_owned(
                self,
                &self.pre_defined_geometrical_tolerance_symbol,
                entity_id,
            )
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<PreDefinedGeometricalToleranceSymbol>> + 'table>
        {
            crate::tables::owned_iter(self, &self.pre_defined_geometrical_tolerance_symbol)
        }
    }
    impl EntityTable<PreDefinedItemHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<PreDefinedItem> {
            crate::tables::get_owned(self, &self.pre_defined_item, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<PreDefinedItem>> + 'table> {
            crate::tables::owned_iter(self, &self.pre_defined_item)
        }
    }
    impl EntityTable<PreDefinedPointMarkerSymbolHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<PreDefinedPointMarkerSymbol> {
            crate::tables::get_owned(self, &self.pre_defined_point_marker_symbol, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<PreDefinedPointMarkerSymbol>> + 'table> {
            crate::tables::owned_iter(self, &self.pre_defined_point_marker_symbol)
        }
    }
    impl EntityTable<PreDefinedSymbolHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<PreDefinedSymbol> {
            crate::tables::get_owned(self, &self.pre_defined_symbol, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<PreDefinedSymbol>> + 'table> {
            crate::tables::owned_iter(self, &self.pre_defined_symbol)
        }
    }
    impl EntityTable<PreDefinedTerminatorSymbolHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<PreDefinedTerminatorSymbol> {
            crate::tables::get_owned(self, &self.pre_defined_terminator_symbol, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<PreDefinedTerminatorSymbol>> + 'table> {
            crate::tables::owned_iter(self, &self.pre_defined_terminator_symbol)
        }
    }
    impl EntityTable<PreDefinedTextFontHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<PreDefinedTextFont> {
            crate::tables::get_owned(self, &self.pre_defined_text_font, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<PreDefinedTextFont>> + 'table> {
            crate::tables::owned_iter(self, &self.pre_defined_text_font)
        }
    }
    impl EntityTable<PresentationAreaHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<PresentationArea> {
            crate::tables::get_owned(self, &self.presentation_area, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<PresentationArea>> + 'table> {
            crate::tables::owned_iter(self, &self.presentation_area)
        }
    }
    impl EntityTable<PresentationLayerAssignmentHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<PresentationLayerAssignment> {
            crate::tables::get_owned(self, &self.presentation_layer_assignment, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<PresentationLayerAssignment>> + 'table> {
            crate::tables::owned_iter(self, &self.presentation_layer_assignment)
        }
    }
    impl EntityTable<PresentationLayerUsageHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<PresentationLayerUsage> {
            crate::tables::get_owned(self, &self.presentation_layer_usage, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<PresentationLayerUsage>> + 'table> {
            crate::tables::owned_iter(self, &self.presentation_layer_usage)
        }
    }
    impl EntityTable<PresentationRepresentationHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<PresentationRepresentation> {
            crate::tables::get_owned(self, &self.presentation_representation, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<PresentationRepresentation>> + 'table> {
            crate::tables::owned_iter(self, &self.presentation_representation)
        }
    }
    impl EntityTable<PresentationSetHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<PresentationSet> {
            crate::tables::get_owned(self, &self.presentation_set, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<PresentationSet>> + 'table> {
            crate::tables::owned_iter(self, &self.presentation_set)
        }
    }
    impl EntityTable<PresentationSizeHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<PresentationSize> {
            crate::tables::get_owned(self, &self.presentation_size, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<PresentationSize>> + 'table> {
            crate::tables::owned_iter(self, &self.presentation_size)
        }
    }
    impl EntityTable<PresentationStyleAssignmentHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<PresentationStyleAssignment> {
            crate::tables::get_owned(self, &self.presentation_style_assignment, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<PresentationStyleAssignment>> + 'table> {
            crate::tables::owned_iter(self, &self.presentation_style_assignment)
        }
    }
    impl EntityTable<PresentationStyleByContextHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<PresentationStyleByContext> {
            crate::tables::get_owned(self, &self.presentation_style_by_context, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<PresentationStyleByContext>> + 'table> {
            crate::tables::owned_iter(self, &self.presentation_style_by_context)
        }
    }
    impl EntityTable<PresentationViewHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<PresentationView> {
            crate::tables::get_owned(self, &self.presentation_view, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<PresentationView>> + 'table> {
            crate::tables::owned_iter(self, &self.presentation_view)
        }
    }
    impl EntityTable<PresentedItemHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<PresentedItem> {
            crate::tables::get_owned(self, &self.presented_item, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<PresentedItem>> + 'table> {
            crate::tables::owned_iter(self, &self.presented_item)
        }
    }
    impl EntityTable<PresentedItemRepresentationHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<PresentedItemRepresentation> {
            crate::tables::get_owned(self, &self.presented_item_representation, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<PresentedItemRepresentation>> + 'table> {
            crate::tables::owned_iter(self, &self.presented_item_representation)
        }
    }
    impl EntityTable<ProductHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<Product> {
            crate::tables::get_owned(self, &self.product, entity_id)
        }
        fn owned_iter<'table>(&'table self) -> Box<dyn Iterator<Item = Result<Product>> + 'table> {
            crate::tables::owned_iter(self, &self.product)
        }
    }
    impl EntityTable<ProductContextHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<ProductContext> {
            crate::tables::get_owned(self, &self.product_context, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<ProductContext>> + 'table> {
            crate::tables::owned_iter(self, &self.product_context)
        }
    }
    impl EntityTable<ProductDefinitionHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<ProductDefinition> {
            crate::tables::get_owned(self, &self.product_definition, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<ProductDefinition>> + 'table> {
            crate::tables::owned_iter(self, &self.product_definition)
        }
    }
    impl EntityTable<ProductDefinitionContextHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<ProductDefinitionContext> {
            crate::tables::get_owned(self, &self.product_definition_context, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<ProductDefinitionContext>> + 'table> {
            crate::tables::owned_iter(self, &self.product_definition_context)
        }
    }
    impl EntityTable<ProductDefinitionFormationHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<ProductDefinitionFormation> {
            crate::tables::get_owned(self, &self.product_definition_formation, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<ProductDefinitionFormation>> + 'table> {
            crate::tables::owned_iter(self, &self.product_definition_formation)
        }
    }
    impl EntityTable<ProductDefinitionShapeHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<ProductDefinitionShape> {
            crate::tables::get_owned(self, &self.product_definition_shape, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<ProductDefinitionShape>> + 'table> {
            crate::tables::owned_iter(self, &self.product_definition_shape)
        }
    }
    impl EntityTable<ProjectionCurveHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<ProjectionCurve> {
            crate::tables::get_owned(self, &self.projection_curve, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<ProjectionCurve>> + 'table> {
            crate::tables::owned_iter(self, &self.projection_curve)
        }
    }
    impl EntityTable<ProjectionDirectedCalloutHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<ProjectionDirectedCallout> {
            crate::tables::get_owned(self, &self.projection_directed_callout, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<ProjectionDirectedCallout>> + 'table> {
            crate::tables::owned_iter(self, &self.projection_directed_callout)
        }
    }
    impl EntityTable<PropertyDefinitionHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<PropertyDefinition> {
            crate::tables::get_owned(self, &self.property_definition, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<PropertyDefinition>> + 'table> {
            crate::tables::owned_iter(self, &self.property_definition)
        }
    }
    impl EntityTable<PropertyDefinitionRepresentationHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<PropertyDefinitionRepresentation> {
            crate::tables::get_owned(self, &self.property_definition_representation, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<PropertyDefinitionRepresentation>> + 'table> {
            crate::tables::owned_iter(self, &self.property_definition_representation)
        }
    }
    impl EntityTable<QuasiUniformCurveHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<QuasiUniformCurve> {
            crate::tables::get_owned(self, &self.quasi_uniform_curve, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<QuasiUniformCurve>> + 'table> {
            crate::tables::owned_iter(self, &self.quasi_uniform_curve)
        }
    }
    impl EntityTable<RadiusDimensionHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<RadiusDimension> {
            crate::tables::get_owned(self, &self.radius_dimension, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<RadiusDimension>> + 'table> {
            crate::tables::owned_iter(self, &self.radius_dimension)
        }
    }
    impl EntityTable<RationalBSplineCurveHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<RationalBSplineCurve> {
            crate::tables::get_owned(self, &self.rational_b_spline_curve, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<RationalBSplineCurve>> + 'table> {
            crate::tables::owned_iter(self, &self.rational_b_spline_curve)
        }
    }
    impl EntityTable<RepresentationHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<Representation> {
            crate::tables::get_owned(self, &self.representation, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<Representation>> + 'table> {
            crate::tables::owned_iter(self, &self.representation)
        }
    }
    impl EntityTable<RepresentationContextHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<RepresentationContext> {
            crate::tables::get_owned(self, &self.representation_context, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<RepresentationContext>> + 'table> {
            crate::tables::owned_iter(self, &self.representation_context)
        }
    }
    impl EntityTable<RepresentationItemHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<RepresentationItem> {
            crate::tables::get_owned(self, &self.representation_item, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<RepresentationItem>> + 'table> {
            crate::tables::owned_iter(self, &self.representation_item)
        }
    }
    impl EntityTable<RepresentationMapHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<RepresentationMap> {
            crate::tables::get_owned(self, &self.representation_map, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<RepresentationMap>> + 'table> {
            crate::tables::owned_iter(self, &self.representation_map)
        }
    }
    impl EntityTable<SecurityClassificationHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<SecurityClassification> {
            crate::tables::get_owned(self, &self.security_classification, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<SecurityClassification>> + 'table> {
            crate::tables::owned_iter(self, &self.security_classification)
        }
    }
    impl EntityTable<SecurityClassificationAssignmentHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<SecurityClassificationAssignment> {
            crate::tables::get_owned(self, &self.security_classification_assignment, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<SecurityClassificationAssignment>> + 'table> {
            crate::tables::owned_iter(self, &self.security_classification_assignment)
        }
    }
    impl EntityTable<SecurityClassificationLevelHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<SecurityClassificationLevel> {
            crate::tables::get_owned(self, &self.security_classification_level, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<SecurityClassificationLevel>> + 'table> {
            crate::tables::owned_iter(self, &self.security_classification_level)
        }
    }
    impl EntityTable<ShapeDefinitionRepresentationHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<ShapeDefinitionRepresentation> {
            crate::tables::get_owned(self, &self.shape_definition_representation, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<ShapeDefinitionRepresentation>> + 'table> {
            crate::tables::owned_iter(self, &self.shape_definition_representation)
        }
    }
    impl EntityTable<ShapeRepresentationHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<ShapeRepresentation> {
            crate::tables::get_owned(self, &self.shape_representation, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<ShapeRepresentation>> + 'table> {
            crate::tables::owned_iter(self, &self.shape_representation)
        }
    }
    impl EntityTable<SiUnitHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<SiUnit> {
            crate::tables::get_owned(self, &self.si_unit, entity_id)
        }
        fn owned_iter<'table>(&'table self) -> Box<dyn Iterator<Item = Result<SiUnit>> + 'table> {
            crate::tables::owned_iter(self, &self.si_unit)
        }
    }
    impl EntityTable<StructuredDimensionCalloutHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<StructuredDimensionCallout> {
            crate::tables::get_owned(self, &self.structured_dimension_callout, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<StructuredDimensionCallout>> + 'table> {
            crate::tables::owned_iter(self, &self.structured_dimension_callout)
        }
    }
    impl EntityTable<StyledItemHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<StyledItem> {
            crate::tables::get_owned(self, &self.styled_item, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<StyledItem>> + 'table> {
            crate::tables::owned_iter(self, &self.styled_item)
        }
    }
    impl EntityTable<SymbolColourHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<SymbolColour> {
            crate::tables::get_owned(self, &self.symbol_colour, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<SymbolColour>> + 'table> {
            crate::tables::owned_iter(self, &self.symbol_colour)
        }
    }
    impl EntityTable<SymbolRepresentationHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<SymbolRepresentation> {
            crate::tables::get_owned(self, &self.symbol_representation, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<SymbolRepresentation>> + 'table> {
            crate::tables::owned_iter(self, &self.symbol_representation)
        }
    }
    impl EntityTable<SymbolRepresentationMapHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<SymbolRepresentationMap> {
            crate::tables::get_owned(self, &self.symbol_representation_map, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<SymbolRepresentationMap>> + 'table> {
            crate::tables::owned_iter(self, &self.symbol_representation_map)
        }
    }
    impl EntityTable<SymbolStyleHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<SymbolStyle> {
            crate::tables::get_owned(self, &self.symbol_style, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<SymbolStyle>> + 'table> {
            crate::tables::owned_iter(self, &self.symbol_style)
        }
    }
    impl EntityTable<SymbolTargetHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<SymbolTarget> {
            crate::tables::get_owned(self, &self.symbol_target, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<SymbolTarget>> + 'table> {
            crate::tables::owned_iter(self, &self.symbol_target)
        }
    }
    impl EntityTable<TerminatorSymbolHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<TerminatorSymbol> {
            crate::tables::get_owned(self, &self.terminator_symbol, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<TerminatorSymbol>> + 'table> {
            crate::tables::owned_iter(self, &self.terminator_symbol)
        }
    }
    impl EntityTable<TextLiteralHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<TextLiteral> {
            crate::tables::get_owned(self, &self.text_literal, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<TextLiteral>> + 'table> {
            crate::tables::owned_iter(self, &self.text_literal)
        }
    }
    impl EntityTable<TextLiteralWithAssociatedCurvesHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<TextLiteralWithAssociatedCurves> {
            crate::tables::get_owned(self, &self.text_literal_with_associated_curves, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<TextLiteralWithAssociatedCurves>> + 'table> {
            crate::tables::owned_iter(self, &self.text_literal_with_associated_curves)
        }
    }
    impl EntityTable<TextLiteralWithBlankingBoxHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<TextLiteralWithBlankingBox> {
            crate::tables::get_owned(self, &self.text_literal_with_blanking_box, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<TextLiteralWithBlankingBox>> + 'table> {
            crate::tables::owned_iter(self, &self.text_literal_with_blanking_box)
        }
    }
    impl EntityTable<TextLiteralWithDelineationHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<TextLiteralWithDelineation> {
            crate::tables::get_owned(self, &self.text_literal_with_delineation, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<TextLiteralWithDelineation>> + 'table> {
            crate::tables::owned_iter(self, &self.text_literal_with_delineation)
        }
    }
    impl EntityTable<TextLiteralWithExtentHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<TextLiteralWithExtent> {
            crate::tables::get_owned(self, &self.text_literal_with_extent, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<TextLiteralWithExtent>> + 'table> {
            crate::tables::owned_iter(self, &self.text_literal_with_extent)
        }
    }
    impl EntityTable<TextStyleHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<TextStyle> {
            crate::tables::get_owned(self, &self.text_style, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<TextStyle>> + 'table> {
            crate::tables::owned_iter(self, &self.text_style)
        }
    }
    impl EntityTable<TextStyleForDefinedFontHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<TextStyleForDefinedFont> {
            crate::tables::get_owned(self, &self.text_style_for_defined_font, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<TextStyleForDefinedFont>> + 'table> {
            crate::tables::owned_iter(self, &self.text_style_for_defined_font)
        }
    }
    impl EntityTable<TextStyleWithBoxCharacteristicsHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<TextStyleWithBoxCharacteristics> {
            crate::tables::get_owned(self, &self.text_style_with_box_characteristics, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<TextStyleWithBoxCharacteristics>> + 'table> {
            crate::tables::owned_iter(self, &self.text_style_with_box_characteristics)
        }
    }
    impl EntityTable<TextStyleWithMirrorHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<TextStyleWithMirror> {
            crate::tables::get_owned(self, &self.text_style_with_mirror, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<TextStyleWithMirror>> + 'table> {
            crate::tables::owned_iter(self, &self.text_style_with_mirror)
        }
    }
    impl EntityTable<TrimmedCurveHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<TrimmedCurve> {
            crate::tables::get_owned(self, &self.trimmed_curve, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<TrimmedCurve>> + 'table> {
            crate::tables::owned_iter(self, &self.trimmed_curve)
        }
    }
    impl EntityTable<TwoDirectionRepeatFactorHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<TwoDirectionRepeatFactor> {
            crate::tables::get_owned(self, &self.two_direction_repeat_factor, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<TwoDirectionRepeatFactor>> + 'table> {
            crate::tables::owned_iter(self, &self.two_direction_repeat_factor)
        }
    }
    impl EntityTable<UniformCurveHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<UniformCurve> {
            crate::tables::get_owned(self, &self.uniform_curve, entity_id)
        }
        fn owned_iter<'table>(
            &'table self,
        ) -> Box<dyn Iterator<Item = Result<UniformCurve>> + 'table> {
            crate::tables::owned_iter(self, &self.uniform_curve)
        }
    }
    impl EntityTable<VectorHolder> for Tables {
        fn get_owned(&self, entity_id: u64) -> Result<Vector> {
            crate::tables::get_owned(self, &self.vector, entity_id)
        }
        fn owned_iter<'table>(&'table self) -> Box<dyn Iterator<Item = Result<Vector>> + 'table> {
            crate::tables::owned_iter(self, &self.vector)
        }
    }
    #[derive(Debug, Clone)]
    pub enum ApprovedItem {
        DrawingRevision(Box<DrawingRevision>),
        DrawingSheetRevision(Box<DrawingSheetRevision>),
    }
    #[derive(Debug, Clone)]
    pub enum AreaOrView {
        PresentationArea(Box<PresentationArea>),
        PresentationView(Box<PresentationView>),
    }
    #[derive(Debug, Clone)]
    pub enum Axis2Placement {
        Axis2Placement2D(Box<Axis2Placement2D>),
    }
    #[derive(Debug, Clone)]
    pub enum BSplineCurveForm {
        EllipticArc,
        PolylineForm,
        ParabolicArc,
        CircularArc,
        Unspecified,
        HyperbolicArc,
    }
    #[derive(Debug, Clone)]
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
    #[derive(Debug, Clone)]
    pub enum CharacterSpacingSelect {
        LengthMeasure(Box<LengthMeasure>),
        RatioMeasure(Box<RatioMeasure>),
        MeasureWithUnit(Box<dyn MeasureWithUnitAny>),
    }
    #[derive(Debug, Clone)]
    pub enum CharacterStyleSelect {
        TextStyleForDefinedFont(Box<TextStyleForDefinedFont>),
    }
    #[derive(Debug, Clone)]
    pub enum CharacterizedDefinition {
        CharacterizedProductDefinition(Box<CharacterizedProductDefinition>),
        ShapeDefinition(Box<ShapeDefinition>),
    }
    #[derive(Debug, Clone)]
    pub enum CharacterizedProductDefinition {
        ProductDefinition(Box<ProductDefinition>),
    }
    #[derive(Debug, Clone)]
    pub enum ClassifiedItem {
        DrawingRevision(Box<DrawingRevision>),
        DrawingSheetRevision(Box<DrawingSheetRevision>),
    }
    #[derive(Debug, Clone)]
    pub enum ContractedItem {
        DrawingRevision(Box<DrawingRevision>),
    }
    #[derive(Debug, Clone)]
    pub enum CurveFontOrScaledCurveFontSelect {
        CurveStyleFontSelect(Box<CurveStyleFontSelect>),
    }
    #[derive(Debug, Clone)]
    pub enum CurveOrAnnotationCurveOccurrence {
        Curve(Box<dyn CurveAny>),
        AnnotationCurveOccurrence(Box<AnnotationCurveOccurrence>),
    }
    #[derive(Debug, Clone)]
    pub enum CurveOrRender {
        CurveStyle(Box<CurveStyle>),
    }
    #[derive(Debug, Clone)]
    pub enum CurveStyleFontSelect {
        CurveStyleFont(Box<CurveStyleFont>),
        PreDefinedCurveFont(Box<PreDefinedCurveFont>),
        ExternallyDefinedCurveFont(Box<ExternallyDefinedCurveFont>),
    }
    #[derive(Debug, Clone)]
    pub enum DateTimeSelect {
        Date(Box<dyn DateAny>),
    }
    pub type DayInMonthNumber = i64;
    #[derive(Debug, Clone)]
    pub enum DefinedSymbolSelect {
        PreDefinedSymbol(Box<PreDefinedSymbol>),
        ExternallyDefinedSymbol(Box<ExternallyDefinedSymbol>),
    }
    pub type DimensionCount = i64;
    #[derive(Debug, Clone)]
    pub enum DimensionExtentUsage {
        Origin,
        Target,
    }
    #[derive(Debug, Clone)]
    pub enum DraughtingCalloutElement {
        AnnotationTextOccurrence(Box<AnnotationTextOccurrence>),
        AnnotationSymbolOccurrence(Box<AnnotationSymbolOccurrence>),
        AnnotationCurveOccurrence(Box<AnnotationCurveOccurrence>),
    }
    #[derive(Debug, Clone)]
    pub enum DraughtingGroupedItem {
        AnnotationOccurrence(Box<dyn AnnotationOccurrenceAny>),
        GeometricSetSelect(Box<GeometricSetSelect>),
    }
    #[derive(Debug, Clone)]
    pub enum DraughtingOrganizationItem {
        ProductDefinitionFormation(Box<ProductDefinitionFormation>),
        DrawingRevision(Box<DrawingRevision>),
        DrawingSheetRevision(Box<DrawingSheetRevision>),
    }
    #[derive(Debug, Clone)]
    pub enum DraughtingPresentedItemSelect {
        ProductDefinitionFormation(Box<ProductDefinitionFormation>),
    }
    #[derive(Debug, Clone)]
    pub enum DraughtingTitledItem {
        DrawingRevision(Box<DrawingRevision>),
        DrawingSheetRevision(Box<DrawingSheetRevision>),
    }
    #[derive(Debug, Clone)]
    pub enum FillAreaStyleTileShapeSelect {
        FillAreaStyleTileSymbolWithStyle(Box<FillAreaStyleTileSymbolWithStyle>),
    }
    #[derive(Debug, Clone)]
    pub enum FillStyleSelect {
        FillAreaStyleColour(Box<FillAreaStyleColour>),
        ExternallyDefinedTileStyle(Box<ExternallyDefinedTileStyle>),
        FillAreaStyleTiles(Box<FillAreaStyleTiles>),
        ExternallyDefinedHatchStyle(Box<ExternallyDefinedHatchStyle>),
        FillAreaStyleHatching(Box<FillAreaStyleHatching>),
    }
    #[derive(Debug, Clone)]
    pub enum FontSelect {
        PreDefinedTextFont(Box<PreDefinedTextFont>),
        ExternallyDefinedTextFont(Box<ExternallyDefinedTextFont>),
    }
    #[derive(Debug, Clone)]
    pub enum GeometricSetSelect {
        Point(Box<dyn PointAny>),
        Curve(Box<dyn CurveAny>),
    }
    #[derive(Debug, Clone)]
    pub enum HidingOrBlankingSelect {
        PresentationArea(Box<PresentationArea>),
        PresentationView(Box<PresentationView>),
        AnnotationFillArea(Box<AnnotationFillArea>),
    }
    pub type Identifier = String;
    #[derive(Debug, Clone)]
    pub enum InvisibilityContext {
        PresentationLayerUsage(Box<PresentationLayerUsage>),
        PresentationRepresentation(Box<PresentationRepresentation>),
        PresentationSet(Box<PresentationSet>),
    }
    #[derive(Debug, Clone)]
    pub enum InvisibleItem {
        StyledItem(Box<StyledItem>),
        PresentationLayerAssignment(Box<PresentationLayerAssignment>),
        PresentationRepresentation(Box<PresentationRepresentation>),
    }
    #[derive(Debug, Clone)]
    pub enum KnotType {
        UniformKnots,
        QuasiUniformKnots,
        PiecewiseBezierKnots,
        Unspecified,
    }
    pub type Label = String;
    #[derive(Debug, Clone)]
    pub enum LayeredItem {
        PresentationRepresentation(Box<PresentationRepresentation>),
        RepresentationItem(Box<RepresentationItem>),
    }
    pub type LengthMeasure = f64;
    #[derive(Debug, Clone)]
    pub enum MeasureValue {
        LengthMeasure(Box<LengthMeasure>),
        PlaneAngleMeasure(Box<PlaneAngleMeasure>),
        RatioMeasure(Box<RatioMeasure>),
        ParameterValue(Box<ParameterValue>),
        PositiveLengthMeasure(Box<PositiveLengthMeasure>),
        PositiveRatioMeasure(Box<PositiveRatioMeasure>),
    }
    pub type MonthInYearNumber = i64;
    #[derive(Debug, Clone)]
    pub enum NullStyle {
        Null,
    }
    pub type ParameterValue = f64;
    #[derive(Debug, Clone)]
    pub enum PersonOrganizationSelect {
        Person(Box<Person>),
        Organization(Box<Organization>),
        PersonAndOrganization(Box<PersonAndOrganization>),
    }
    pub type PlaneAngleMeasure = f64;
    pub type PositiveLengthMeasure = LengthMeasure;
    pub type PositiveRatioMeasure = RatioMeasure;
    pub type PresentableText = String;
    #[derive(Debug, Clone)]
    pub enum PresentationRepresentationSelect {
        PresentationRepresentation(Box<PresentationRepresentation>),
        PresentationSet(Box<PresentationSet>),
    }
    #[derive(Debug, Clone)]
    pub enum PresentationSizeAssignmentSelect {
        PresentationView(Box<PresentationView>),
        PresentationArea(Box<PresentationArea>),
        AreaInSet(Box<AreaInSet>),
    }
    #[derive(Debug, Clone)]
    pub enum PresentationStyleSelect {
        CurveStyle(Box<CurveStyle>),
        SymbolStyle(Box<SymbolStyle>),
        FillAreaStyle(Box<FillAreaStyle>),
        TextStyle(Box<TextStyle>),
        NullStyle(Box<NullStyle>),
    }
    pub type RatioMeasure = f64;
    #[derive(Debug, Clone)]
    pub enum ShapeDefinition {
        ProductDefinitionShape(Box<ProductDefinitionShape>),
    }
    #[derive(Debug, Clone)]
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
    #[derive(Debug, Clone)]
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
    #[derive(Debug, Clone)]
    pub enum SizeSelect {
        PositiveLengthMeasure(Box<PositiveLengthMeasure>),
        MeasureWithUnit(Box<dyn MeasureWithUnitAny>),
    }
    #[derive(Debug, Clone)]
    pub enum SourceItem {
        Identifier(Box<Identifier>),
    }
    #[derive(Debug, Clone)]
    pub enum SpecifiedItem {
        DrawingRevision(Box<DrawingRevision>),
    }
    #[derive(Debug, Clone)]
    pub enum StyleContextSelect {
        Representation(Box<Representation>),
        RepresentationItem(Box<RepresentationItem>),
        PresentationSet(Box<PresentationSet>),
    }
    #[derive(Debug, Clone)]
    pub enum SymbolStyleSelect {
        SymbolColour(Box<SymbolColour>),
    }
    pub type Text = String;
    pub type TextAlignment = Label;
    pub type TextDelineation = Label;
    #[derive(Debug, Clone)]
    pub enum TextOrCharacter {
        AnnotationText(Box<AnnotationText>),
        CompositeText(Box<CompositeText>),
        TextLiteral(Box<TextLiteral>),
    }
    #[derive(Debug, Clone)]
    pub enum TextPath {
        Up,
        Right,
        Down,
        Left,
    }
    #[derive(Debug, Clone)]
    pub enum TransitionCode {
        Discontinuous,
        ContSameGradientSameCurvature,
        ContSameGradient,
        Continuous,
    }
    #[derive(Debug, Clone)]
    pub enum TrimmingPreference {
        Parameter,
        Unspecified,
        Cartesian,
    }
    #[derive(Debug, Clone)]
    pub enum TrimmingSelect {
        CartesianPoint(Box<CartesianPoint>),
        ParameterValue(Box<ParameterValue>),
    }
    #[derive(Debug, Clone)]
    pub enum Unit {
        NamedUnit(Box<dyn NamedUnitAny>),
    }
    #[derive(Debug, Clone)]
    pub enum VectorOrDirection {
        Vector(Box<Vector>),
        Direction(Box<Direction>),
    }
    pub type YearNumber = i64;
    #[derive(Debug, Clone, derive_new :: new)]
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
    #[derive(Clone, Debug)]
    struct AddressHolder {
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
    impl Holder for AddressHolder {
        type Table = Tables;
        type Owned = Address;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "ADDRESS"
        }
        fn attr_len() -> usize {
            12usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct AngularDimension {
        pub dimension_curve_directed_callout: DimensionCurveDirectedCallout,
    }
    #[derive(Clone, Debug)]
    struct AngularDimensionHolder {
        dimension_curve_directed_callout: PlaceHolder<DimensionCurveDirectedCallout>,
    }
    impl Holder for AngularDimensionHolder {
        type Table = Tables;
        type Owned = AngularDimension;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "ANGULAR_DIMENSION"
        }
        fn attr_len() -> usize {
            1usize
        }
    }
    impl AnnotationOccurrenceAny for AnnotationCurveOccurrence {}
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct AnnotationCurveOccurrence {
        pub annotation_occurrence: Box<dyn AnnotationOccurrenceAny>,
    }
    #[derive(Clone, Debug)]
    struct AnnotationCurveOccurrenceHolder {
        annotation_occurrence: PlaceHolder<Box<dyn AnnotationOccurrenceAny>>,
    }
    impl Holder for AnnotationCurveOccurrenceHolder {
        type Table = Tables;
        type Owned = AnnotationCurveOccurrence;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "ANNOTATION_CURVE_OCCURRENCE"
        }
        fn attr_len() -> usize {
            1usize
        }
    }
    impl GeometricRepresentationItemAny for AnnotationFillArea {}
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct AnnotationFillArea {
        pub boundaries: Vec<Box<dyn CurveAny>>,
        pub geometric_representation_item: Box<dyn GeometricRepresentationItemAny>,
    }
    #[derive(Clone, Debug)]
    struct AnnotationFillAreaHolder {
        boundaries: PlaceHolder<Vec<Box<dyn CurveAny>>>,
        geometric_representation_item: PlaceHolder<Box<dyn GeometricRepresentationItemAny>>,
    }
    impl Holder for AnnotationFillAreaHolder {
        type Table = Tables;
        type Owned = AnnotationFillArea;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "ANNOTATION_FILL_AREA"
        }
        fn attr_len() -> usize {
            2usize
        }
    }
    impl AnnotationOccurrenceAny for AnnotationFillAreaOccurrence {}
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct AnnotationFillAreaOccurrence {
        pub fill_style_target: Box<dyn PointAny>,
        pub annotation_occurrence: Box<dyn AnnotationOccurrenceAny>,
    }
    #[derive(Clone, Debug)]
    struct AnnotationFillAreaOccurrenceHolder {
        fill_style_target: PlaceHolder<Box<dyn PointAny>>,
        annotation_occurrence: PlaceHolder<Box<dyn AnnotationOccurrenceAny>>,
    }
    impl Holder for AnnotationFillAreaOccurrenceHolder {
        type Table = Tables;
        type Owned = AnnotationFillAreaOccurrence;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "ANNOTATION_FILL_AREA_OCCURRENCE"
        }
        fn attr_len() -> usize {
            2usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct AnnotationOccurrence {
        pub styled_item: StyledItem,
    }
    #[derive(Clone, Debug)]
    struct AnnotationOccurrenceHolder {
        styled_item: PlaceHolder<StyledItem>,
    }
    impl Holder for AnnotationOccurrenceHolder {
        type Table = Tables;
        type Owned = AnnotationOccurrence;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "ANNOTATION_OCCURRENCE"
        }
        fn attr_len() -> usize {
            1usize
        }
    }
    pub trait AnnotationOccurrenceAny:
        ::std::any::Any + ::std::fmt::Debug + dyn_clone::DynClone
    {
    }
    dyn_clone::clone_trait_object!(AnnotationOccurrenceAny);
    impl AnnotationOccurrenceAny for AnnotationOccurrence {}
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct AnnotationSubfigureOccurrence {
        pub annotation_symbol_occurrence: AnnotationSymbolOccurrence,
    }
    #[derive(Clone, Debug)]
    struct AnnotationSubfigureOccurrenceHolder {
        annotation_symbol_occurrence: PlaceHolder<AnnotationSymbolOccurrence>,
    }
    impl Holder for AnnotationSubfigureOccurrenceHolder {
        type Table = Tables;
        type Owned = AnnotationSubfigureOccurrence;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "ANNOTATION_SUBFIGURE_OCCURRENCE"
        }
        fn attr_len() -> usize {
            1usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct AnnotationSymbol {
        pub mapped_item: MappedItem,
    }
    #[derive(Clone, Debug)]
    struct AnnotationSymbolHolder {
        mapped_item: PlaceHolder<MappedItem>,
    }
    impl Holder for AnnotationSymbolHolder {
        type Table = Tables;
        type Owned = AnnotationSymbol;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "ANNOTATION_SYMBOL"
        }
        fn attr_len() -> usize {
            1usize
        }
    }
    impl AnnotationOccurrenceAny for AnnotationSymbolOccurrence {}
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct AnnotationSymbolOccurrence {
        pub annotation_occurrence: Box<dyn AnnotationOccurrenceAny>,
    }
    #[derive(Clone, Debug)]
    struct AnnotationSymbolOccurrenceHolder {
        annotation_occurrence: PlaceHolder<Box<dyn AnnotationOccurrenceAny>>,
    }
    impl Holder for AnnotationSymbolOccurrenceHolder {
        type Table = Tables;
        type Owned = AnnotationSymbolOccurrence;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "ANNOTATION_SYMBOL_OCCURRENCE"
        }
        fn attr_len() -> usize {
            1usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct AnnotationText {
        pub mapped_item: MappedItem,
    }
    #[derive(Clone, Debug)]
    struct AnnotationTextHolder {
        mapped_item: PlaceHolder<MappedItem>,
    }
    impl Holder for AnnotationTextHolder {
        type Table = Tables;
        type Owned = AnnotationText;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "ANNOTATION_TEXT"
        }
        fn attr_len() -> usize {
            1usize
        }
    }
    impl AnnotationOccurrenceAny for AnnotationTextOccurrence {}
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct AnnotationTextOccurrence {
        pub annotation_occurrence: Box<dyn AnnotationOccurrenceAny>,
    }
    #[derive(Clone, Debug)]
    struct AnnotationTextOccurrenceHolder {
        annotation_occurrence: PlaceHolder<Box<dyn AnnotationOccurrenceAny>>,
    }
    impl Holder for AnnotationTextOccurrenceHolder {
        type Table = Tables;
        type Owned = AnnotationTextOccurrence;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "ANNOTATION_TEXT_OCCURRENCE"
        }
        fn attr_len() -> usize {
            1usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct ApplicationContext {
        pub application: Text,
    }
    #[derive(Clone, Debug)]
    struct ApplicationContextHolder {
        application: Text,
    }
    impl Holder for ApplicationContextHolder {
        type Table = Tables;
        type Owned = ApplicationContext;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "APPLICATION_CONTEXT"
        }
        fn attr_len() -> usize {
            1usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct ApplicationContextElement {
        pub name: Label,
        pub frame_of_reference: ApplicationContext,
    }
    #[derive(Clone, Debug)]
    struct ApplicationContextElementHolder {
        name: Label,
        frame_of_reference: PlaceHolder<ApplicationContext>,
    }
    impl Holder for ApplicationContextElementHolder {
        type Table = Tables;
        type Owned = ApplicationContextElement;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "APPLICATION_CONTEXT_ELEMENT"
        }
        fn attr_len() -> usize {
            2usize
        }
    }
    pub trait ApplicationContextElementAny:
        ::std::any::Any + ::std::fmt::Debug + dyn_clone::DynClone
    {
    }
    dyn_clone::clone_trait_object!(ApplicationContextElementAny);
    impl ApplicationContextElementAny for ApplicationContextElement {}
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct ApplicationProtocolDefinition {
        pub status: Label,
        pub application_interpreted_model_schema_name: Label,
        pub application_protocol_year: YearNumber,
        pub application: ApplicationContext,
    }
    #[derive(Clone, Debug)]
    struct ApplicationProtocolDefinitionHolder {
        status: Label,
        application_interpreted_model_schema_name: Label,
        application_protocol_year: YearNumber,
        application: PlaceHolder<ApplicationContext>,
    }
    impl Holder for ApplicationProtocolDefinitionHolder {
        type Table = Tables;
        type Owned = ApplicationProtocolDefinition;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "APPLICATION_PROTOCOL_DEFINITION"
        }
        fn attr_len() -> usize {
            4usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct Approval {
        pub status: ApprovalStatus,
        pub level: Label,
    }
    #[derive(Clone, Debug)]
    struct ApprovalHolder {
        status: PlaceHolder<ApprovalStatus>,
        level: Label,
    }
    impl Holder for ApprovalHolder {
        type Table = Tables;
        type Owned = Approval;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "APPROVAL"
        }
        fn attr_len() -> usize {
            2usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct ApprovalAssignment {
        pub assigned_approval: Approval,
    }
    #[derive(Clone, Debug)]
    struct ApprovalAssignmentHolder {
        assigned_approval: PlaceHolder<Approval>,
    }
    impl Holder for ApprovalAssignmentHolder {
        type Table = Tables;
        type Owned = ApprovalAssignment;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "APPROVAL_ASSIGNMENT"
        }
        fn attr_len() -> usize {
            1usize
        }
    }
    pub trait ApprovalAssignmentAny:
        ::std::any::Any + ::std::fmt::Debug + dyn_clone::DynClone
    {
    }
    dyn_clone::clone_trait_object!(ApprovalAssignmentAny);
    impl ApprovalAssignmentAny for ApprovalAssignment {}
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct ApprovalDateTime {
        pub date_time: DateTimeSelect,
        pub dated_approval: Approval,
    }
    #[derive(Clone, Debug)]
    struct ApprovalDateTimeHolder {
        date_time: PlaceHolder<DateTimeSelect>,
        dated_approval: PlaceHolder<Approval>,
    }
    impl Holder for ApprovalDateTimeHolder {
        type Table = Tables;
        type Owned = ApprovalDateTime;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "APPROVAL_DATE_TIME"
        }
        fn attr_len() -> usize {
            2usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct ApprovalPersonOrganization {
        pub person_organization: PersonOrganizationSelect,
        pub authorized_approval: Approval,
        pub role: ApprovalRole,
    }
    #[derive(Clone, Debug)]
    struct ApprovalPersonOrganizationHolder {
        person_organization: PlaceHolder<PersonOrganizationSelect>,
        authorized_approval: PlaceHolder<Approval>,
        role: PlaceHolder<ApprovalRole>,
    }
    impl Holder for ApprovalPersonOrganizationHolder {
        type Table = Tables;
        type Owned = ApprovalPersonOrganization;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "APPROVAL_PERSON_ORGANIZATION"
        }
        fn attr_len() -> usize {
            3usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct ApprovalRole {
        pub role: Label,
    }
    #[derive(Clone, Debug)]
    struct ApprovalRoleHolder {
        role: Label,
    }
    impl Holder for ApprovalRoleHolder {
        type Table = Tables;
        type Owned = ApprovalRole;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "APPROVAL_ROLE"
        }
        fn attr_len() -> usize {
            1usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct ApprovalStatus {
        pub name: Label,
    }
    #[derive(Clone, Debug)]
    struct ApprovalStatusHolder {
        name: Label,
    }
    impl Holder for ApprovalStatusHolder {
        type Table = Tables;
        type Owned = ApprovalStatus;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "APPROVAL_STATUS"
        }
        fn attr_len() -> usize {
            1usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct AreaInSet {
        pub area: PresentationArea,
        pub in_set: PresentationSet,
    }
    #[derive(Clone, Debug)]
    struct AreaInSetHolder {
        area: PlaceHolder<PresentationArea>,
        in_set: PlaceHolder<PresentationSet>,
    }
    impl Holder for AreaInSetHolder {
        type Table = Tables;
        type Owned = AreaInSet;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "AREA_IN_SET"
        }
        fn attr_len() -> usize {
            2usize
        }
    }
    impl PlacementAny for Axis2Placement2D {}
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct Axis2Placement2D {
        pub ref_direction: Option<Direction>,
        pub placement: Box<dyn PlacementAny>,
    }
    #[derive(Clone, Debug)]
    struct Axis2Placement2DHolder {
        ref_direction: Option<PlaceHolder<Direction>>,
        placement: PlaceHolder<Box<dyn PlacementAny>>,
    }
    impl Holder for Axis2Placement2DHolder {
        type Table = Tables;
        type Owned = Axis2Placement2D;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "AXIS2_PLACEMENT_2D"
        }
        fn attr_len() -> usize {
            2usize
        }
    }
    impl BoundedCurveAny for BSplineCurve {}
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct BSplineCurve {
        pub degree: i64,
        pub control_points_list: Vec<CartesianPoint>,
        pub curve_form: BSplineCurveForm,
        pub closed_curve: Logical,
        pub self_intersect: Logical,
        pub bounded_curve: Box<dyn BoundedCurveAny>,
    }
    #[derive(Clone, Debug)]
    struct BSplineCurveHolder {
        degree: i64,
        control_points_list: PlaceHolder<Vec<CartesianPoint>>,
        curve_form: PlaceHolder<BSplineCurveForm>,
        closed_curve: Logical,
        self_intersect: Logical,
        bounded_curve: PlaceHolder<Box<dyn BoundedCurveAny>>,
    }
    impl Holder for BSplineCurveHolder {
        type Table = Tables;
        type Owned = BSplineCurve;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "B_SPLINE_CURVE"
        }
        fn attr_len() -> usize {
            6usize
        }
    }
    pub trait BSplineCurveAny: ::std::any::Any + ::std::fmt::Debug + dyn_clone::DynClone {}
    dyn_clone::clone_trait_object!(BSplineCurveAny);
    impl BSplineCurveAny for BSplineCurve {}
    impl BSplineCurveAny for BSplineCurveWithKnots {}
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct BSplineCurveWithKnots {
        pub knot_multiplicities: Vec<i64>,
        pub knots: Vec<ParameterValue>,
        pub knot_spec: KnotType,
        pub b_spline_curve: Box<dyn BSplineCurveAny>,
    }
    #[derive(Clone, Debug)]
    struct BSplineCurveWithKnotsHolder {
        knot_multiplicities: PlaceHolder<Vec<i64>>,
        knots: PlaceHolder<Vec<ParameterValue>>,
        knot_spec: PlaceHolder<KnotType>,
        b_spline_curve: PlaceHolder<Box<dyn BSplineCurveAny>>,
    }
    impl Holder for BSplineCurveWithKnotsHolder {
        type Table = Tables;
        type Owned = BSplineCurveWithKnots;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "B_SPLINE_CURVE_WITH_KNOTS"
        }
        fn attr_len() -> usize {
            4usize
        }
    }
    impl BSplineCurveAny for BezierCurve {}
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct BezierCurve {
        pub b_spline_curve: Box<dyn BSplineCurveAny>,
    }
    #[derive(Clone, Debug)]
    struct BezierCurveHolder {
        b_spline_curve: PlaceHolder<Box<dyn BSplineCurveAny>>,
    }
    impl Holder for BezierCurveHolder {
        type Table = Tables;
        type Owned = BezierCurve;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "BEZIER_CURVE"
        }
        fn attr_len() -> usize {
            1usize
        }
    }
    impl CurveAny for BoundedCurve {}
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct BoundedCurve {
        pub curve: Box<dyn CurveAny>,
    }
    #[derive(Clone, Debug)]
    struct BoundedCurveHolder {
        curve: PlaceHolder<Box<dyn CurveAny>>,
    }
    impl Holder for BoundedCurveHolder {
        type Table = Tables;
        type Owned = BoundedCurve;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "BOUNDED_CURVE"
        }
        fn attr_len() -> usize {
            1usize
        }
    }
    pub trait BoundedCurveAny: ::std::any::Any + ::std::fmt::Debug + dyn_clone::DynClone {}
    dyn_clone::clone_trait_object!(BoundedCurveAny);
    impl BoundedCurveAny for BoundedCurve {}
    impl DateAny for CalendarDate {}
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct CalendarDate {
        pub day_component: DayInMonthNumber,
        pub month_component: MonthInYearNumber,
        pub date: Box<dyn DateAny>,
    }
    #[derive(Clone, Debug)]
    struct CalendarDateHolder {
        day_component: DayInMonthNumber,
        month_component: MonthInYearNumber,
        date: PlaceHolder<Box<dyn DateAny>>,
    }
    impl Holder for CalendarDateHolder {
        type Table = Tables;
        type Owned = CalendarDate;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "CALENDAR_DATE"
        }
        fn attr_len() -> usize {
            3usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct CameraImage {
        pub mapped_item: MappedItem,
    }
    #[derive(Clone, Debug)]
    struct CameraImageHolder {
        mapped_item: PlaceHolder<MappedItem>,
    }
    impl Holder for CameraImageHolder {
        type Table = Tables;
        type Owned = CameraImage;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "CAMERA_IMAGE"
        }
        fn attr_len() -> usize {
            1usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct CameraImage2DWithScale {
        pub camera_image: CameraImage,
    }
    #[derive(Clone, Debug)]
    struct CameraImage2DWithScaleHolder {
        camera_image: PlaceHolder<CameraImage>,
    }
    impl Holder for CameraImage2DWithScaleHolder {
        type Table = Tables;
        type Owned = CameraImage2DWithScale;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "CAMERA_IMAGE_2D_WITH_SCALE"
        }
        fn attr_len() -> usize {
            1usize
        }
    }
    impl GeometricRepresentationItemAny for CameraModel {}
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct CameraModel {
        pub geometric_representation_item: Box<dyn GeometricRepresentationItemAny>,
    }
    #[derive(Clone, Debug)]
    struct CameraModelHolder {
        geometric_representation_item: PlaceHolder<Box<dyn GeometricRepresentationItemAny>>,
    }
    impl Holder for CameraModelHolder {
        type Table = Tables;
        type Owned = CameraModel;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "CAMERA_MODEL"
        }
        fn attr_len() -> usize {
            1usize
        }
    }
    pub trait CameraModelAny: ::std::any::Any + ::std::fmt::Debug + dyn_clone::DynClone {}
    dyn_clone::clone_trait_object!(CameraModelAny);
    impl CameraModelAny for CameraModel {}
    impl CameraModelAny for CameraModelD2 {}
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct CameraModelD2 {
        pub view_window: PlanarBox,
        pub view_window_clipping: bool,
        pub camera_model: Box<dyn CameraModelAny>,
    }
    #[derive(Clone, Debug)]
    struct CameraModelD2Holder {
        view_window: PlaceHolder<PlanarBox>,
        view_window_clipping: bool,
        camera_model: PlaceHolder<Box<dyn CameraModelAny>>,
    }
    impl Holder for CameraModelD2Holder {
        type Table = Tables;
        type Owned = CameraModelD2;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "CAMERA_MODEL_D2"
        }
        fn attr_len() -> usize {
            3usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct CameraUsage {
        pub representation_map: RepresentationMap,
    }
    #[derive(Clone, Debug)]
    struct CameraUsageHolder {
        representation_map: PlaceHolder<RepresentationMap>,
    }
    impl Holder for CameraUsageHolder {
        type Table = Tables;
        type Owned = CameraUsage;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "CAMERA_USAGE"
        }
        fn attr_len() -> usize {
            1usize
        }
    }
    impl PointAny for CartesianPoint {}
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct CartesianPoint {
        pub coordinates: Vec<LengthMeasure>,
        pub point: Box<dyn PointAny>,
    }
    #[derive(Clone, Debug)]
    struct CartesianPointHolder {
        coordinates: PlaceHolder<Vec<LengthMeasure>>,
        point: PlaceHolder<Box<dyn PointAny>>,
    }
    impl Holder for CartesianPointHolder {
        type Table = Tables;
        type Owned = CartesianPoint;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "CARTESIAN_POINT"
        }
        fn attr_len() -> usize {
            2usize
        }
    }
    impl ConicAny for Circle {}
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct Circle {
        pub radius: PositiveLengthMeasure,
        pub conic: Box<dyn ConicAny>,
    }
    #[derive(Clone, Debug)]
    struct CircleHolder {
        radius: PositiveLengthMeasure,
        conic: PlaceHolder<Box<dyn ConicAny>>,
    }
    impl Holder for CircleHolder {
        type Table = Tables;
        type Owned = Circle;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "CIRCLE"
        }
        fn attr_len() -> usize {
            2usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct Colour {}
    #[derive(Clone, Debug)]
    struct ColourHolder {}
    impl Holder for ColourHolder {
        type Table = Tables;
        type Owned = Colour;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "COLOUR"
        }
        fn attr_len() -> usize {
            0usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct ColourRgb {
        pub red: f64,
        pub green: f64,
        pub blue: f64,
        pub colour_specification: ColourSpecification,
    }
    #[derive(Clone, Debug)]
    struct ColourRgbHolder {
        red: f64,
        green: f64,
        blue: f64,
        colour_specification: PlaceHolder<ColourSpecification>,
    }
    impl Holder for ColourRgbHolder {
        type Table = Tables;
        type Owned = ColourRgb;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "COLOUR_RGB"
        }
        fn attr_len() -> usize {
            4usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct ColourSpecification {
        pub name: Colour,
        pub colour: Colour,
    }
    #[derive(Clone, Debug)]
    struct ColourSpecificationHolder {
        name: PlaceHolder<Colour>,
        colour: PlaceHolder<Colour>,
    }
    impl Holder for ColourSpecificationHolder {
        type Table = Tables;
        type Owned = ColourSpecification;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "COLOUR_SPECIFICATION"
        }
        fn attr_len() -> usize {
            2usize
        }
    }
    impl BoundedCurveAny for CompositeCurve {}
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct CompositeCurve {
        pub segments: Vec<CompositeCurveSegment>,
        pub self_intersect: Logical,
        pub bounded_curve: Box<dyn BoundedCurveAny>,
    }
    #[derive(Clone, Debug)]
    struct CompositeCurveHolder {
        segments: PlaceHolder<Vec<CompositeCurveSegment>>,
        self_intersect: Logical,
        bounded_curve: PlaceHolder<Box<dyn BoundedCurveAny>>,
    }
    impl Holder for CompositeCurveHolder {
        type Table = Tables;
        type Owned = CompositeCurve;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "COMPOSITE_CURVE"
        }
        fn attr_len() -> usize {
            3usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct CompositeCurveSegment {
        pub transition: TransitionCode,
        pub same_sense: bool,
        pub parent_curve: Box<dyn CurveAny>,
    }
    #[derive(Clone, Debug)]
    struct CompositeCurveSegmentHolder {
        transition: PlaceHolder<TransitionCode>,
        same_sense: bool,
        parent_curve: PlaceHolder<Box<dyn CurveAny>>,
    }
    impl Holder for CompositeCurveSegmentHolder {
        type Table = Tables;
        type Owned = CompositeCurveSegment;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "COMPOSITE_CURVE_SEGMENT"
        }
        fn attr_len() -> usize {
            3usize
        }
    }
    impl GeometricRepresentationItemAny for CompositeText {}
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct CompositeText {
        pub collected_text: Vec<TextOrCharacter>,
        pub geometric_representation_item: Box<dyn GeometricRepresentationItemAny>,
    }
    #[derive(Clone, Debug)]
    struct CompositeTextHolder {
        collected_text: PlaceHolder<Vec<TextOrCharacter>>,
        geometric_representation_item: PlaceHolder<Box<dyn GeometricRepresentationItemAny>>,
    }
    impl Holder for CompositeTextHolder {
        type Table = Tables;
        type Owned = CompositeText;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "COMPOSITE_TEXT"
        }
        fn attr_len() -> usize {
            2usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct CompositeTextWithAssociatedCurves {
        pub associated_curves: Vec<Box<dyn CurveAny>>,
        pub composite_text: CompositeText,
    }
    #[derive(Clone, Debug)]
    struct CompositeTextWithAssociatedCurvesHolder {
        associated_curves: PlaceHolder<Vec<Box<dyn CurveAny>>>,
        composite_text: PlaceHolder<CompositeText>,
    }
    impl Holder for CompositeTextWithAssociatedCurvesHolder {
        type Table = Tables;
        type Owned = CompositeTextWithAssociatedCurves;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "COMPOSITE_TEXT_WITH_ASSOCIATED_CURVES"
        }
        fn attr_len() -> usize {
            2usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct CompositeTextWithBlankingBox {
        pub blanking: PlanarBox,
        pub composite_text: CompositeText,
    }
    #[derive(Clone, Debug)]
    struct CompositeTextWithBlankingBoxHolder {
        blanking: PlaceHolder<PlanarBox>,
        composite_text: PlaceHolder<CompositeText>,
    }
    impl Holder for CompositeTextWithBlankingBoxHolder {
        type Table = Tables;
        type Owned = CompositeTextWithBlankingBox;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "COMPOSITE_TEXT_WITH_BLANKING_BOX"
        }
        fn attr_len() -> usize {
            2usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct CompositeTextWithExtent {
        pub extent: PlanarExtent,
        pub composite_text: CompositeText,
    }
    #[derive(Clone, Debug)]
    struct CompositeTextWithExtentHolder {
        extent: PlaceHolder<PlanarExtent>,
        composite_text: PlaceHolder<CompositeText>,
    }
    impl Holder for CompositeTextWithExtentHolder {
        type Table = Tables;
        type Owned = CompositeTextWithExtent;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "COMPOSITE_TEXT_WITH_EXTENT"
        }
        fn attr_len() -> usize {
            2usize
        }
    }
    impl CurveAny for Conic {}
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct Conic {
        pub position: Axis2Placement,
        pub curve: Box<dyn CurveAny>,
    }
    #[derive(Clone, Debug)]
    struct ConicHolder {
        position: PlaceHolder<Axis2Placement>,
        curve: PlaceHolder<Box<dyn CurveAny>>,
    }
    impl Holder for ConicHolder {
        type Table = Tables;
        type Owned = Conic;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "CONIC"
        }
        fn attr_len() -> usize {
            2usize
        }
    }
    pub trait ConicAny: ::std::any::Any + ::std::fmt::Debug + dyn_clone::DynClone {}
    dyn_clone::clone_trait_object!(ConicAny);
    impl ConicAny for Conic {}
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct ContextDependentInvisibility {
        pub presentation_context: InvisibilityContext,
        pub invisibility: Invisibility,
    }
    #[derive(Clone, Debug)]
    struct ContextDependentInvisibilityHolder {
        presentation_context: PlaceHolder<InvisibilityContext>,
        invisibility: PlaceHolder<Invisibility>,
    }
    impl Holder for ContextDependentInvisibilityHolder {
        type Table = Tables;
        type Owned = ContextDependentInvisibility;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "CONTEXT_DEPENDENT_INVISIBILITY"
        }
        fn attr_len() -> usize {
            2usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct Contract {
        pub name: Label,
        pub purpose: Text,
        pub kind: ContractType,
    }
    #[derive(Clone, Debug)]
    struct ContractHolder {
        name: Label,
        purpose: Text,
        kind: PlaceHolder<ContractType>,
    }
    impl Holder for ContractHolder {
        type Table = Tables;
        type Owned = Contract;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "CONTRACT"
        }
        fn attr_len() -> usize {
            3usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct ContractAssignment {
        pub assigned_contract: Contract,
    }
    #[derive(Clone, Debug)]
    struct ContractAssignmentHolder {
        assigned_contract: PlaceHolder<Contract>,
    }
    impl Holder for ContractAssignmentHolder {
        type Table = Tables;
        type Owned = ContractAssignment;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "CONTRACT_ASSIGNMENT"
        }
        fn attr_len() -> usize {
            1usize
        }
    }
    pub trait ContractAssignmentAny:
        ::std::any::Any + ::std::fmt::Debug + dyn_clone::DynClone
    {
    }
    dyn_clone::clone_trait_object!(ContractAssignmentAny);
    impl ContractAssignmentAny for ContractAssignment {}
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct ContractType {
        pub description: Label,
    }
    #[derive(Clone, Debug)]
    struct ContractTypeHolder {
        description: Label,
    }
    impl Holder for ContractTypeHolder {
        type Table = Tables;
        type Owned = ContractType;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "CONTRACT_TYPE"
        }
        fn attr_len() -> usize {
            1usize
        }
    }
    impl NamedUnitAny for ConversionBasedUnit {}
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct ConversionBasedUnit {
        pub name: Label,
        pub conversion_factor: Box<dyn MeasureWithUnitAny>,
        pub named_unit: Box<dyn NamedUnitAny>,
    }
    #[derive(Clone, Debug)]
    struct ConversionBasedUnitHolder {
        name: Label,
        conversion_factor: PlaceHolder<Box<dyn MeasureWithUnitAny>>,
        named_unit: PlaceHolder<Box<dyn NamedUnitAny>>,
    }
    impl Holder for ConversionBasedUnitHolder {
        type Table = Tables;
        type Owned = ConversionBasedUnit;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "CONVERSION_BASED_UNIT"
        }
        fn attr_len() -> usize {
            3usize
        }
    }
    impl GeometricRepresentationItemAny for Curve {}
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct Curve {
        pub geometric_representation_item: Box<dyn GeometricRepresentationItemAny>,
    }
    #[derive(Clone, Debug)]
    struct CurveHolder {
        geometric_representation_item: PlaceHolder<Box<dyn GeometricRepresentationItemAny>>,
    }
    impl Holder for CurveHolder {
        type Table = Tables;
        type Owned = Curve;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "CURVE"
        }
        fn attr_len() -> usize {
            1usize
        }
    }
    pub trait CurveAny: ::std::any::Any + ::std::fmt::Debug + dyn_clone::DynClone {}
    dyn_clone::clone_trait_object!(CurveAny);
    impl CurveAny for Curve {}
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct CurveDimension {
        pub dimension_curve_directed_callout: DimensionCurveDirectedCallout,
    }
    #[derive(Clone, Debug)]
    struct CurveDimensionHolder {
        dimension_curve_directed_callout: PlaceHolder<DimensionCurveDirectedCallout>,
    }
    impl Holder for CurveDimensionHolder {
        type Table = Tables;
        type Owned = CurveDimension;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "CURVE_DIMENSION"
        }
        fn attr_len() -> usize {
            1usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct CurveStyle {
        pub name: Label,
        pub curve_font: CurveFontOrScaledCurveFontSelect,
        pub curve_width: SizeSelect,
        pub curve_colour: Colour,
    }
    #[derive(Clone, Debug)]
    struct CurveStyleHolder {
        name: Label,
        curve_font: PlaceHolder<CurveFontOrScaledCurveFontSelect>,
        curve_width: PlaceHolder<SizeSelect>,
        curve_colour: PlaceHolder<Colour>,
    }
    impl Holder for CurveStyleHolder {
        type Table = Tables;
        type Owned = CurveStyle;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "CURVE_STYLE"
        }
        fn attr_len() -> usize {
            4usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct CurveStyleFont {
        pub name: Label,
        pub pattern_list: Vec<CurveStyleFontPattern>,
    }
    #[derive(Clone, Debug)]
    struct CurveStyleFontHolder {
        name: Label,
        pattern_list: PlaceHolder<Vec<CurveStyleFontPattern>>,
    }
    impl Holder for CurveStyleFontHolder {
        type Table = Tables;
        type Owned = CurveStyleFont;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "CURVE_STYLE_FONT"
        }
        fn attr_len() -> usize {
            2usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct CurveStyleFontPattern {
        pub visible_segment_length: PositiveLengthMeasure,
        pub invisible_segment_length: PositiveLengthMeasure,
    }
    #[derive(Clone, Debug)]
    struct CurveStyleFontPatternHolder {
        visible_segment_length: PositiveLengthMeasure,
        invisible_segment_length: PositiveLengthMeasure,
    }
    impl Holder for CurveStyleFontPatternHolder {
        type Table = Tables;
        type Owned = CurveStyleFontPattern;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "CURVE_STYLE_FONT_PATTERN"
        }
        fn attr_len() -> usize {
            2usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct Date {
        pub year_component: YearNumber,
    }
    #[derive(Clone, Debug)]
    struct DateHolder {
        year_component: YearNumber,
    }
    impl Holder for DateHolder {
        type Table = Tables;
        type Owned = Date;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "DATE"
        }
        fn attr_len() -> usize {
            1usize
        }
    }
    pub trait DateAny: ::std::any::Any + ::std::fmt::Debug + dyn_clone::DynClone {}
    dyn_clone::clone_trait_object!(DateAny);
    impl DateAny for Date {}
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct DatumFeatureCallout {
        pub draughting_callout: DraughtingCallout,
    }
    #[derive(Clone, Debug)]
    struct DatumFeatureCalloutHolder {
        draughting_callout: PlaceHolder<DraughtingCallout>,
    }
    impl Holder for DatumFeatureCalloutHolder {
        type Table = Tables;
        type Owned = DatumFeatureCallout;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "DATUM_FEATURE_CALLOUT"
        }
        fn attr_len() -> usize {
            1usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct DatumTargetCallout {
        pub draughting_callout: DraughtingCallout,
    }
    #[derive(Clone, Debug)]
    struct DatumTargetCalloutHolder {
        draughting_callout: PlaceHolder<DraughtingCallout>,
    }
    impl Holder for DatumTargetCalloutHolder {
        type Table = Tables;
        type Owned = DatumTargetCallout;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "DATUM_TARGET_CALLOUT"
        }
        fn attr_len() -> usize {
            1usize
        }
    }
    impl GeometricRepresentationItemAny for DefinedSymbol {}
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct DefinedSymbol {
        pub definition: DefinedSymbolSelect,
        pub target: SymbolTarget,
        pub geometric_representation_item: Box<dyn GeometricRepresentationItemAny>,
    }
    #[derive(Clone, Debug)]
    struct DefinedSymbolHolder {
        definition: PlaceHolder<DefinedSymbolSelect>,
        target: PlaceHolder<SymbolTarget>,
        geometric_representation_item: PlaceHolder<Box<dyn GeometricRepresentationItemAny>>,
    }
    impl Holder for DefinedSymbolHolder {
        type Table = Tables;
        type Owned = DefinedSymbol;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "DEFINED_SYMBOL"
        }
        fn attr_len() -> usize {
            3usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct DiameterDimension {
        pub dimension_curve_directed_callout: DimensionCurveDirectedCallout,
    }
    #[derive(Clone, Debug)]
    struct DiameterDimensionHolder {
        dimension_curve_directed_callout: PlaceHolder<DimensionCurveDirectedCallout>,
    }
    impl Holder for DiameterDimensionHolder {
        type Table = Tables;
        type Owned = DiameterDimension;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "DIAMETER_DIMENSION"
        }
        fn attr_len() -> usize {
            1usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct DimensionCalloutComponentRelationship {
        pub draughting_callout_relationship: DraughtingCalloutRelationship,
    }
    #[derive(Clone, Debug)]
    struct DimensionCalloutComponentRelationshipHolder {
        draughting_callout_relationship: PlaceHolder<DraughtingCalloutRelationship>,
    }
    impl Holder for DimensionCalloutComponentRelationshipHolder {
        type Table = Tables;
        type Owned = DimensionCalloutComponentRelationship;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "DIMENSION_CALLOUT_COMPONENT_RELATIONSHIP"
        }
        fn attr_len() -> usize {
            1usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct DimensionCalloutRelationship {
        pub draughting_callout_relationship: DraughtingCalloutRelationship,
    }
    #[derive(Clone, Debug)]
    struct DimensionCalloutRelationshipHolder {
        draughting_callout_relationship: PlaceHolder<DraughtingCalloutRelationship>,
    }
    impl Holder for DimensionCalloutRelationshipHolder {
        type Table = Tables;
        type Owned = DimensionCalloutRelationship;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "DIMENSION_CALLOUT_RELATIONSHIP"
        }
        fn attr_len() -> usize {
            1usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct DimensionCurve {
        pub annotation_curve_occurrence: AnnotationCurveOccurrence,
    }
    #[derive(Clone, Debug)]
    struct DimensionCurveHolder {
        annotation_curve_occurrence: PlaceHolder<AnnotationCurveOccurrence>,
    }
    impl Holder for DimensionCurveHolder {
        type Table = Tables;
        type Owned = DimensionCurve;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "DIMENSION_CURVE"
        }
        fn attr_len() -> usize {
            1usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct DimensionCurveDirectedCallout {
        pub draughting_callout: DraughtingCallout,
    }
    #[derive(Clone, Debug)]
    struct DimensionCurveDirectedCalloutHolder {
        draughting_callout: PlaceHolder<DraughtingCallout>,
    }
    impl Holder for DimensionCurveDirectedCalloutHolder {
        type Table = Tables;
        type Owned = DimensionCurveDirectedCallout;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "DIMENSION_CURVE_DIRECTED_CALLOUT"
        }
        fn attr_len() -> usize {
            1usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct DimensionCurveTerminator {
        pub role: DimensionExtentUsage,
        pub terminator_symbol: TerminatorSymbol,
    }
    #[derive(Clone, Debug)]
    struct DimensionCurveTerminatorHolder {
        role: PlaceHolder<DimensionExtentUsage>,
        terminator_symbol: PlaceHolder<TerminatorSymbol>,
    }
    impl Holder for DimensionCurveTerminatorHolder {
        type Table = Tables;
        type Owned = DimensionCurveTerminator;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "DIMENSION_CURVE_TERMINATOR"
        }
        fn attr_len() -> usize {
            2usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct DimensionPair {
        pub draughting_callout_relationship: DraughtingCalloutRelationship,
    }
    #[derive(Clone, Debug)]
    struct DimensionPairHolder {
        draughting_callout_relationship: PlaceHolder<DraughtingCalloutRelationship>,
    }
    impl Holder for DimensionPairHolder {
        type Table = Tables;
        type Owned = DimensionPair;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "DIMENSION_PAIR"
        }
        fn attr_len() -> usize {
            1usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct DimensionalExponents {
        pub length_exponent: f64,
        pub mass_exponent: f64,
        pub time_exponent: f64,
        pub electric_current_exponent: f64,
        pub thermodynamic_temperature_exponent: f64,
        pub amount_of_substance_exponent: f64,
        pub luminous_intensity_exponent: f64,
    }
    #[derive(Clone, Debug)]
    struct DimensionalExponentsHolder {
        length_exponent: f64,
        mass_exponent: f64,
        time_exponent: f64,
        electric_current_exponent: f64,
        thermodynamic_temperature_exponent: f64,
        amount_of_substance_exponent: f64,
        luminous_intensity_exponent: f64,
    }
    impl Holder for DimensionalExponentsHolder {
        type Table = Tables;
        type Owned = DimensionalExponents;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "DIMENSIONAL_EXPONENTS"
        }
        fn attr_len() -> usize {
            7usize
        }
    }
    impl GeometricRepresentationItemAny for Direction {}
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct Direction {
        pub direction_ratios: Vec<f64>,
        pub geometric_representation_item: Box<dyn GeometricRepresentationItemAny>,
    }
    #[derive(Clone, Debug)]
    struct DirectionHolder {
        direction_ratios: PlaceHolder<Vec<f64>>,
        geometric_representation_item: PlaceHolder<Box<dyn GeometricRepresentationItemAny>>,
    }
    impl Holder for DirectionHolder {
        type Table = Tables;
        type Owned = Direction;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "DIRECTION"
        }
        fn attr_len() -> usize {
            2usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct Document {
        pub id: Identifier,
        pub name: Label,
        pub description: Text,
        pub kind: DocumentType,
    }
    #[derive(Clone, Debug)]
    struct DocumentHolder {
        id: Identifier,
        name: Label,
        description: Text,
        kind: PlaceHolder<DocumentType>,
    }
    impl Holder for DocumentHolder {
        type Table = Tables;
        type Owned = Document;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "DOCUMENT"
        }
        fn attr_len() -> usize {
            4usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct DocumentReference {
        pub assigned_document: Document,
        pub source: Label,
    }
    #[derive(Clone, Debug)]
    struct DocumentReferenceHolder {
        assigned_document: PlaceHolder<Document>,
        source: Label,
    }
    impl Holder for DocumentReferenceHolder {
        type Table = Tables;
        type Owned = DocumentReference;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "DOCUMENT_REFERENCE"
        }
        fn attr_len() -> usize {
            2usize
        }
    }
    pub trait DocumentReferenceAny:
        ::std::any::Any + ::std::fmt::Debug + dyn_clone::DynClone
    {
    }
    dyn_clone::clone_trait_object!(DocumentReferenceAny);
    impl DocumentReferenceAny for DocumentReference {}
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct DocumentType {
        pub product_data_type: Label,
    }
    #[derive(Clone, Debug)]
    struct DocumentTypeHolder {
        product_data_type: Label,
    }
    impl Holder for DocumentTypeHolder {
        type Table = Tables;
        type Owned = DocumentType;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "DOCUMENT_TYPE"
        }
        fn attr_len() -> usize {
            1usize
        }
    }
    impl AnnotationOccurrenceAny for DraughtingAnnotationOccurrence {}
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct DraughtingAnnotationOccurrence {
        pub annotation_occurrence: Box<dyn AnnotationOccurrenceAny>,
    }
    #[derive(Clone, Debug)]
    struct DraughtingAnnotationOccurrenceHolder {
        annotation_occurrence: PlaceHolder<Box<dyn AnnotationOccurrenceAny>>,
    }
    impl Holder for DraughtingAnnotationOccurrenceHolder {
        type Table = Tables;
        type Owned = DraughtingAnnotationOccurrence;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "DRAUGHTING_ANNOTATION_OCCURRENCE"
        }
        fn attr_len() -> usize {
            1usize
        }
    }
    impl ApprovalAssignmentAny for DraughtingApprovalAssignment {}
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct DraughtingApprovalAssignment {
        pub approved_items: Vec<ApprovedItem>,
        pub approval_assignment: Box<dyn ApprovalAssignmentAny>,
    }
    #[derive(Clone, Debug)]
    struct DraughtingApprovalAssignmentHolder {
        approved_items: PlaceHolder<Vec<ApprovedItem>>,
        approval_assignment: PlaceHolder<Box<dyn ApprovalAssignmentAny>>,
    }
    impl Holder for DraughtingApprovalAssignmentHolder {
        type Table = Tables;
        type Owned = DraughtingApprovalAssignment;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "DRAUGHTING_APPROVAL_ASSIGNMENT"
        }
        fn attr_len() -> usize {
            2usize
        }
    }
    impl GeometricRepresentationItemAny for DraughtingCallout {}
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct DraughtingCallout {
        pub contents: Vec<DraughtingCalloutElement>,
        pub geometric_representation_item: Box<dyn GeometricRepresentationItemAny>,
    }
    #[derive(Clone, Debug)]
    struct DraughtingCalloutHolder {
        contents: PlaceHolder<Vec<DraughtingCalloutElement>>,
        geometric_representation_item: PlaceHolder<Box<dyn GeometricRepresentationItemAny>>,
    }
    impl Holder for DraughtingCalloutHolder {
        type Table = Tables;
        type Owned = DraughtingCallout;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "DRAUGHTING_CALLOUT"
        }
        fn attr_len() -> usize {
            2usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct DraughtingCalloutRelationship {
        pub name: Label,
        pub description: Text,
        pub relating_draughting_callout: DraughtingCallout,
        pub related_draughting_callout: DraughtingCallout,
    }
    #[derive(Clone, Debug)]
    struct DraughtingCalloutRelationshipHolder {
        name: Label,
        description: Text,
        relating_draughting_callout: PlaceHolder<DraughtingCallout>,
        related_draughting_callout: PlaceHolder<DraughtingCallout>,
    }
    impl Holder for DraughtingCalloutRelationshipHolder {
        type Table = Tables;
        type Owned = DraughtingCalloutRelationship;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "DRAUGHTING_CALLOUT_RELATIONSHIP"
        }
        fn attr_len() -> usize {
            4usize
        }
    }
    impl ContractAssignmentAny for DraughtingContractAssignment {}
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct DraughtingContractAssignment {
        pub items: Vec<ContractedItem>,
        pub contract_assignment: Box<dyn ContractAssignmentAny>,
    }
    #[derive(Clone, Debug)]
    struct DraughtingContractAssignmentHolder {
        items: PlaceHolder<Vec<ContractedItem>>,
        contract_assignment: PlaceHolder<Box<dyn ContractAssignmentAny>>,
    }
    impl Holder for DraughtingContractAssignmentHolder {
        type Table = Tables;
        type Owned = DraughtingContractAssignment;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "DRAUGHTING_CONTRACT_ASSIGNMENT"
        }
        fn attr_len() -> usize {
            2usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct DraughtingDrawingRevision {
        pub drawing_revision: DrawingRevision,
    }
    #[derive(Clone, Debug)]
    struct DraughtingDrawingRevisionHolder {
        drawing_revision: PlaceHolder<DrawingRevision>,
    }
    impl Holder for DraughtingDrawingRevisionHolder {
        type Table = Tables;
        type Owned = DraughtingDrawingRevision;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "DRAUGHTING_DRAWING_REVISION"
        }
        fn attr_len() -> usize {
            1usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct DraughtingElements {
        pub draughting_callout: DraughtingCallout,
    }
    #[derive(Clone, Debug)]
    struct DraughtingElementsHolder {
        draughting_callout: PlaceHolder<DraughtingCallout>,
    }
    impl Holder for DraughtingElementsHolder {
        type Table = Tables;
        type Owned = DraughtingElements;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "DRAUGHTING_ELEMENTS"
        }
        fn attr_len() -> usize {
            1usize
        }
    }
    impl GroupAssignmentAny for DraughtingGroupAssignment {}
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct DraughtingGroupAssignment {
        pub items: Vec<DraughtingGroupedItem>,
        pub group_assignment: Box<dyn GroupAssignmentAny>,
    }
    #[derive(Clone, Debug)]
    struct DraughtingGroupAssignmentHolder {
        items: PlaceHolder<Vec<DraughtingGroupedItem>>,
        group_assignment: PlaceHolder<Box<dyn GroupAssignmentAny>>,
    }
    impl Holder for DraughtingGroupAssignmentHolder {
        type Table = Tables;
        type Owned = DraughtingGroupAssignment;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "DRAUGHTING_GROUP_ASSIGNMENT"
        }
        fn attr_len() -> usize {
            2usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct DraughtingModel {
        pub representation: Representation,
    }
    #[derive(Clone, Debug)]
    struct DraughtingModelHolder {
        representation: PlaceHolder<Representation>,
    }
    impl Holder for DraughtingModelHolder {
        type Table = Tables;
        type Owned = DraughtingModel;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "DRAUGHTING_MODEL"
        }
        fn attr_len() -> usize {
            1usize
        }
    }
    impl OrganizationAssignmentAny for DraughtingOrganizationAssignment {}
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct DraughtingOrganizationAssignment {
        pub assigned_items: Vec<DraughtingOrganizationItem>,
        pub organization_assignment: Box<dyn OrganizationAssignmentAny>,
    }
    #[derive(Clone, Debug)]
    struct DraughtingOrganizationAssignmentHolder {
        assigned_items: PlaceHolder<Vec<DraughtingOrganizationItem>>,
        organization_assignment: PlaceHolder<Box<dyn OrganizationAssignmentAny>>,
    }
    impl Holder for DraughtingOrganizationAssignmentHolder {
        type Table = Tables;
        type Owned = DraughtingOrganizationAssignment;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "DRAUGHTING_ORGANIZATION_ASSIGNMENT"
        }
        fn attr_len() -> usize {
            2usize
        }
    }
    impl PersonAndOrganizationAssignmentAny for DraughtingPersonAndOrganizationAssignment {}
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct DraughtingPersonAndOrganizationAssignment {
        pub assigned_items: Vec<DraughtingOrganizationItem>,
        pub person_and_organization_assignment: Box<dyn PersonAndOrganizationAssignmentAny>,
    }
    #[derive(Clone, Debug)]
    struct DraughtingPersonAndOrganizationAssignmentHolder {
        assigned_items: PlaceHolder<Vec<DraughtingOrganizationItem>>,
        person_and_organization_assignment:
            PlaceHolder<Box<dyn PersonAndOrganizationAssignmentAny>>,
    }
    impl Holder for DraughtingPersonAndOrganizationAssignmentHolder {
        type Table = Tables;
        type Owned = DraughtingPersonAndOrganizationAssignment;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "DRAUGHTING_PERSON_AND_ORGANIZATION_ASSIGNMENT"
        }
        fn attr_len() -> usize {
            2usize
        }
    }
    impl PersonAssignmentAny for DraughtingPersonAssignment {}
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct DraughtingPersonAssignment {
        pub assigned_items: Vec<DraughtingOrganizationItem>,
        pub person_assignment: Box<dyn PersonAssignmentAny>,
    }
    #[derive(Clone, Debug)]
    struct DraughtingPersonAssignmentHolder {
        assigned_items: PlaceHolder<Vec<DraughtingOrganizationItem>>,
        person_assignment: PlaceHolder<Box<dyn PersonAssignmentAny>>,
    }
    impl Holder for DraughtingPersonAssignmentHolder {
        type Table = Tables;
        type Owned = DraughtingPersonAssignment;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "DRAUGHTING_PERSON_ASSIGNMENT"
        }
        fn attr_len() -> usize {
            2usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct DraughtingPreDefinedColour {
        pub pre_defined_colour: PreDefinedColour,
    }
    #[derive(Clone, Debug)]
    struct DraughtingPreDefinedColourHolder {
        pre_defined_colour: PlaceHolder<PreDefinedColour>,
    }
    impl Holder for DraughtingPreDefinedColourHolder {
        type Table = Tables;
        type Owned = DraughtingPreDefinedColour;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "DRAUGHTING_PRE_DEFINED_COLOUR"
        }
        fn attr_len() -> usize {
            1usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct DraughtingPreDefinedCurveFont {
        pub pre_defined_curve_font: PreDefinedCurveFont,
    }
    #[derive(Clone, Debug)]
    struct DraughtingPreDefinedCurveFontHolder {
        pre_defined_curve_font: PlaceHolder<PreDefinedCurveFont>,
    }
    impl Holder for DraughtingPreDefinedCurveFontHolder {
        type Table = Tables;
        type Owned = DraughtingPreDefinedCurveFont;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "DRAUGHTING_PRE_DEFINED_CURVE_FONT"
        }
        fn attr_len() -> usize {
            1usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct DraughtingPreDefinedTextFont {
        pub pre_defined_text_font: PreDefinedTextFont,
    }
    #[derive(Clone, Debug)]
    struct DraughtingPreDefinedTextFontHolder {
        pre_defined_text_font: PlaceHolder<PreDefinedTextFont>,
    }
    impl Holder for DraughtingPreDefinedTextFontHolder {
        type Table = Tables;
        type Owned = DraughtingPreDefinedTextFont;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "DRAUGHTING_PRE_DEFINED_TEXT_FONT"
        }
        fn attr_len() -> usize {
            1usize
        }
    }
    impl PresentedItemAny for DraughtingPresentedItem {}
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct DraughtingPresentedItem {
        pub items: Vec<DraughtingPresentedItemSelect>,
        pub presented_item: Box<dyn PresentedItemAny>,
    }
    #[derive(Clone, Debug)]
    struct DraughtingPresentedItemHolder {
        items: PlaceHolder<Vec<DraughtingPresentedItemSelect>>,
        presented_item: PlaceHolder<Box<dyn PresentedItemAny>>,
    }
    impl Holder for DraughtingPresentedItemHolder {
        type Table = Tables;
        type Owned = DraughtingPresentedItem;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "DRAUGHTING_PRESENTED_ITEM"
        }
        fn attr_len() -> usize {
            2usize
        }
    }
    impl SecurityClassificationAssignmentAny for DraughtingSecurityClassificationAssignment {}
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct DraughtingSecurityClassificationAssignment {
        pub assigned_items: Vec<ClassifiedItem>,
        pub security_classification_assignment: Box<dyn SecurityClassificationAssignmentAny>,
    }
    #[derive(Clone, Debug)]
    struct DraughtingSecurityClassificationAssignmentHolder {
        assigned_items: PlaceHolder<Vec<ClassifiedItem>>,
        security_classification_assignment:
            PlaceHolder<Box<dyn SecurityClassificationAssignmentAny>>,
    }
    impl Holder for DraughtingSecurityClassificationAssignmentHolder {
        type Table = Tables;
        type Owned = DraughtingSecurityClassificationAssignment;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "DRAUGHTING_SECURITY_CLASSIFICATION_ASSIGNMENT"
        }
        fn attr_len() -> usize {
            2usize
        }
    }
    impl DocumentReferenceAny for DraughtingSpecificationReference {}
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct DraughtingSpecificationReference {
        pub specified_items: Vec<SpecifiedItem>,
        pub document_reference: Box<dyn DocumentReferenceAny>,
    }
    #[derive(Clone, Debug)]
    struct DraughtingSpecificationReferenceHolder {
        specified_items: PlaceHolder<Vec<SpecifiedItem>>,
        document_reference: PlaceHolder<Box<dyn DocumentReferenceAny>>,
    }
    impl Holder for DraughtingSpecificationReferenceHolder {
        type Table = Tables;
        type Owned = DraughtingSpecificationReference;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "DRAUGHTING_SPECIFICATION_REFERENCE"
        }
        fn attr_len() -> usize {
            2usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct DraughtingSubfigureRepresentation {
        pub symbol_representation: SymbolRepresentation,
    }
    #[derive(Clone, Debug)]
    struct DraughtingSubfigureRepresentationHolder {
        symbol_representation: PlaceHolder<SymbolRepresentation>,
    }
    impl Holder for DraughtingSubfigureRepresentationHolder {
        type Table = Tables;
        type Owned = DraughtingSubfigureRepresentation;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "DRAUGHTING_SUBFIGURE_REPRESENTATION"
        }
        fn attr_len() -> usize {
            1usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct DraughtingSymbolRepresentation {
        pub symbol_representation: SymbolRepresentation,
    }
    #[derive(Clone, Debug)]
    struct DraughtingSymbolRepresentationHolder {
        symbol_representation: PlaceHolder<SymbolRepresentation>,
    }
    impl Holder for DraughtingSymbolRepresentationHolder {
        type Table = Tables;
        type Owned = DraughtingSymbolRepresentation;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "DRAUGHTING_SYMBOL_REPRESENTATION"
        }
        fn attr_len() -> usize {
            1usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct DraughtingTextLiteralWithDelineation {
        pub text_literal_with_delineation: TextLiteralWithDelineation,
    }
    #[derive(Clone, Debug)]
    struct DraughtingTextLiteralWithDelineationHolder {
        text_literal_with_delineation: PlaceHolder<TextLiteralWithDelineation>,
    }
    impl Holder for DraughtingTextLiteralWithDelineationHolder {
        type Table = Tables;
        type Owned = DraughtingTextLiteralWithDelineation;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "DRAUGHTING_TEXT_LITERAL_WITH_DELINEATION"
        }
        fn attr_len() -> usize {
            1usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct DraughtingTitle {
        pub items: Vec<DraughtingTitledItem>,
        pub language: Label,
        pub contents: Text,
    }
    #[derive(Clone, Debug)]
    struct DraughtingTitleHolder {
        items: PlaceHolder<Vec<DraughtingTitledItem>>,
        language: Label,
        contents: Text,
    }
    impl Holder for DraughtingTitleHolder {
        type Table = Tables;
        type Owned = DraughtingTitle;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "DRAUGHTING_TITLE"
        }
        fn attr_len() -> usize {
            3usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct DrawingDefinition {
        pub drawing_number: Identifier,
        pub drawing_type: Option<Label>,
    }
    #[derive(Clone, Debug)]
    struct DrawingDefinitionHolder {
        drawing_number: Identifier,
        drawing_type: Option<Label>,
    }
    impl Holder for DrawingDefinitionHolder {
        type Table = Tables;
        type Owned = DrawingDefinition;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "DRAWING_DEFINITION"
        }
        fn attr_len() -> usize {
            2usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct DrawingRevision {
        pub revision_identifier: Identifier,
        pub drawing_identifier: DrawingDefinition,
        pub intended_scale: Option<Text>,
        pub presentation_set: PresentationSet,
    }
    #[derive(Clone, Debug)]
    struct DrawingRevisionHolder {
        revision_identifier: Identifier,
        drawing_identifier: PlaceHolder<DrawingDefinition>,
        intended_scale: Option<Text>,
        presentation_set: PlaceHolder<PresentationSet>,
    }
    impl Holder for DrawingRevisionHolder {
        type Table = Tables;
        type Owned = DrawingRevision;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "DRAWING_REVISION"
        }
        fn attr_len() -> usize {
            4usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct DrawingSheetLayout {
        pub draughting_symbol_representation: DraughtingSymbolRepresentation,
    }
    #[derive(Clone, Debug)]
    struct DrawingSheetLayoutHolder {
        draughting_symbol_representation: PlaceHolder<DraughtingSymbolRepresentation>,
    }
    impl Holder for DrawingSheetLayoutHolder {
        type Table = Tables;
        type Owned = DrawingSheetLayout;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "DRAWING_SHEET_LAYOUT"
        }
        fn attr_len() -> usize {
            1usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct DrawingSheetRevision {
        pub revision_identifier: Identifier,
        pub presentation_area: PresentationArea,
    }
    #[derive(Clone, Debug)]
    struct DrawingSheetRevisionHolder {
        revision_identifier: Identifier,
        presentation_area: PlaceHolder<PresentationArea>,
    }
    impl Holder for DrawingSheetRevisionHolder {
        type Table = Tables;
        type Owned = DrawingSheetRevision;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "DRAWING_SHEET_REVISION"
        }
        fn attr_len() -> usize {
            2usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct DrawingSheetRevisionUsage {
        pub sheet_number: Identifier,
        pub area_in_set: AreaInSet,
    }
    #[derive(Clone, Debug)]
    struct DrawingSheetRevisionUsageHolder {
        sheet_number: Identifier,
        area_in_set: PlaceHolder<AreaInSet>,
    }
    impl Holder for DrawingSheetRevisionUsageHolder {
        type Table = Tables;
        type Owned = DrawingSheetRevisionUsage;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "DRAWING_SHEET_REVISION_USAGE"
        }
        fn attr_len() -> usize {
            2usize
        }
    }
    impl ConicAny for Ellipse {}
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct Ellipse {
        pub semi_axis_1: PositiveLengthMeasure,
        pub semi_axis_2: PositiveLengthMeasure,
        pub conic: Box<dyn ConicAny>,
    }
    #[derive(Clone, Debug)]
    struct EllipseHolder {
        semi_axis_1: PositiveLengthMeasure,
        semi_axis_2: PositiveLengthMeasure,
        conic: PlaceHolder<Box<dyn ConicAny>>,
    }
    impl Holder for EllipseHolder {
        type Table = Tables;
        type Owned = Ellipse;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "ELLIPSE"
        }
        fn attr_len() -> usize {
            3usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct ExternalSource {
        pub source_id: SourceItem,
    }
    #[derive(Clone, Debug)]
    struct ExternalSourceHolder {
        source_id: PlaceHolder<SourceItem>,
    }
    impl Holder for ExternalSourceHolder {
        type Table = Tables;
        type Owned = ExternalSource;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "EXTERNAL_SOURCE"
        }
        fn attr_len() -> usize {
            1usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct ExternallyDefinedCurveFont {
        pub externally_defined_item: ExternallyDefinedItem,
    }
    #[derive(Clone, Debug)]
    struct ExternallyDefinedCurveFontHolder {
        externally_defined_item: PlaceHolder<ExternallyDefinedItem>,
    }
    impl Holder for ExternallyDefinedCurveFontHolder {
        type Table = Tables;
        type Owned = ExternallyDefinedCurveFont;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "EXTERNALLY_DEFINED_CURVE_FONT"
        }
        fn attr_len() -> usize {
            1usize
        }
    }
    impl GeometricRepresentationItemAny for ExternallyDefinedHatchStyle {}
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct ExternallyDefinedHatchStyle {
        pub externally_defined_item: ExternallyDefinedItem,
        pub geometric_representation_item: Box<dyn GeometricRepresentationItemAny>,
    }
    #[derive(Clone, Debug)]
    struct ExternallyDefinedHatchStyleHolder {
        externally_defined_item: PlaceHolder<ExternallyDefinedItem>,
        geometric_representation_item: PlaceHolder<Box<dyn GeometricRepresentationItemAny>>,
    }
    impl Holder for ExternallyDefinedHatchStyleHolder {
        type Table = Tables;
        type Owned = ExternallyDefinedHatchStyle;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "EXTERNALLY_DEFINED_HATCH_STYLE"
        }
        fn attr_len() -> usize {
            2usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct ExternallyDefinedItem {
        pub item_id: SourceItem,
        pub source: ExternalSource,
    }
    #[derive(Clone, Debug)]
    struct ExternallyDefinedItemHolder {
        item_id: PlaceHolder<SourceItem>,
        source: PlaceHolder<ExternalSource>,
    }
    impl Holder for ExternallyDefinedItemHolder {
        type Table = Tables;
        type Owned = ExternallyDefinedItem;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "EXTERNALLY_DEFINED_ITEM"
        }
        fn attr_len() -> usize {
            2usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct ExternallyDefinedSymbol {
        pub externally_defined_item: ExternallyDefinedItem,
    }
    #[derive(Clone, Debug)]
    struct ExternallyDefinedSymbolHolder {
        externally_defined_item: PlaceHolder<ExternallyDefinedItem>,
    }
    impl Holder for ExternallyDefinedSymbolHolder {
        type Table = Tables;
        type Owned = ExternallyDefinedSymbol;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "EXTERNALLY_DEFINED_SYMBOL"
        }
        fn attr_len() -> usize {
            1usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct ExternallyDefinedTextFont {
        pub externally_defined_item: ExternallyDefinedItem,
    }
    #[derive(Clone, Debug)]
    struct ExternallyDefinedTextFontHolder {
        externally_defined_item: PlaceHolder<ExternallyDefinedItem>,
    }
    impl Holder for ExternallyDefinedTextFontHolder {
        type Table = Tables;
        type Owned = ExternallyDefinedTextFont;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "EXTERNALLY_DEFINED_TEXT_FONT"
        }
        fn attr_len() -> usize {
            1usize
        }
    }
    impl GeometricRepresentationItemAny for ExternallyDefinedTileStyle {}
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct ExternallyDefinedTileStyle {
        pub externally_defined_item: ExternallyDefinedItem,
        pub geometric_representation_item: Box<dyn GeometricRepresentationItemAny>,
    }
    #[derive(Clone, Debug)]
    struct ExternallyDefinedTileStyleHolder {
        externally_defined_item: PlaceHolder<ExternallyDefinedItem>,
        geometric_representation_item: PlaceHolder<Box<dyn GeometricRepresentationItemAny>>,
    }
    impl Holder for ExternallyDefinedTileStyleHolder {
        type Table = Tables;
        type Owned = ExternallyDefinedTileStyle;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "EXTERNALLY_DEFINED_TILE_STYLE"
        }
        fn attr_len() -> usize {
            2usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct FillAreaStyle {
        pub name: Label,
        pub fill_styles: Vec<FillStyleSelect>,
    }
    #[derive(Clone, Debug)]
    struct FillAreaStyleHolder {
        name: Label,
        fill_styles: PlaceHolder<Vec<FillStyleSelect>>,
    }
    impl Holder for FillAreaStyleHolder {
        type Table = Tables;
        type Owned = FillAreaStyle;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "FILL_AREA_STYLE"
        }
        fn attr_len() -> usize {
            2usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct FillAreaStyleColour {
        pub name: Label,
        pub fill_colour: Colour,
    }
    #[derive(Clone, Debug)]
    struct FillAreaStyleColourHolder {
        name: Label,
        fill_colour: PlaceHolder<Colour>,
    }
    impl Holder for FillAreaStyleColourHolder {
        type Table = Tables;
        type Owned = FillAreaStyleColour;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "FILL_AREA_STYLE_COLOUR"
        }
        fn attr_len() -> usize {
            2usize
        }
    }
    impl GeometricRepresentationItemAny for FillAreaStyleHatching {}
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct FillAreaStyleHatching {
        pub hatch_line_appearance: CurveStyle,
        pub start_of_next_hatch_line: OneDirectionRepeatFactor,
        pub point_of_reference_hatch_line: CartesianPoint,
        pub pattern_start: CartesianPoint,
        pub hatch_line_angle: PlaneAngleMeasure,
        pub geometric_representation_item: Box<dyn GeometricRepresentationItemAny>,
    }
    #[derive(Clone, Debug)]
    struct FillAreaStyleHatchingHolder {
        hatch_line_appearance: PlaceHolder<CurveStyle>,
        start_of_next_hatch_line: PlaceHolder<OneDirectionRepeatFactor>,
        point_of_reference_hatch_line: PlaceHolder<CartesianPoint>,
        pattern_start: PlaceHolder<CartesianPoint>,
        hatch_line_angle: PlaneAngleMeasure,
        geometric_representation_item: PlaceHolder<Box<dyn GeometricRepresentationItemAny>>,
    }
    impl Holder for FillAreaStyleHatchingHolder {
        type Table = Tables;
        type Owned = FillAreaStyleHatching;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "FILL_AREA_STYLE_HATCHING"
        }
        fn attr_len() -> usize {
            6usize
        }
    }
    impl GeometricRepresentationItemAny for FillAreaStyleTileSymbolWithStyle {}
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct FillAreaStyleTileSymbolWithStyle {
        pub symbol: AnnotationSymbolOccurrence,
        pub geometric_representation_item: Box<dyn GeometricRepresentationItemAny>,
    }
    #[derive(Clone, Debug)]
    struct FillAreaStyleTileSymbolWithStyleHolder {
        symbol: PlaceHolder<AnnotationSymbolOccurrence>,
        geometric_representation_item: PlaceHolder<Box<dyn GeometricRepresentationItemAny>>,
    }
    impl Holder for FillAreaStyleTileSymbolWithStyleHolder {
        type Table = Tables;
        type Owned = FillAreaStyleTileSymbolWithStyle;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "FILL_AREA_STYLE_TILE_SYMBOL_WITH_STYLE"
        }
        fn attr_len() -> usize {
            2usize
        }
    }
    impl GeometricRepresentationItemAny for FillAreaStyleTiles {}
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct FillAreaStyleTiles {
        pub tiling_pattern: TwoDirectionRepeatFactor,
        pub tiles: Vec<FillAreaStyleTileShapeSelect>,
        pub tiling_scale: PositiveRatioMeasure,
        pub geometric_representation_item: Box<dyn GeometricRepresentationItemAny>,
    }
    #[derive(Clone, Debug)]
    struct FillAreaStyleTilesHolder {
        tiling_pattern: PlaceHolder<TwoDirectionRepeatFactor>,
        tiles: PlaceHolder<Vec<FillAreaStyleTileShapeSelect>>,
        tiling_scale: PositiveRatioMeasure,
        geometric_representation_item: PlaceHolder<Box<dyn GeometricRepresentationItemAny>>,
    }
    impl Holder for FillAreaStyleTilesHolder {
        type Table = Tables;
        type Owned = FillAreaStyleTiles;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "FILL_AREA_STYLE_TILES"
        }
        fn attr_len() -> usize {
            4usize
        }
    }
    impl GeometricSetAny for GeometricCurveSet {}
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct GeometricCurveSet {
        pub geometric_set: Box<dyn GeometricSetAny>,
    }
    #[derive(Clone, Debug)]
    struct GeometricCurveSetHolder {
        geometric_set: PlaceHolder<Box<dyn GeometricSetAny>>,
    }
    impl Holder for GeometricCurveSetHolder {
        type Table = Tables;
        type Owned = GeometricCurveSet;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "GEOMETRIC_CURVE_SET"
        }
        fn attr_len() -> usize {
            1usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct GeometricRepresentationContext {
        pub coordinate_space_dimension: DimensionCount,
        pub representation_context: RepresentationContext,
    }
    #[derive(Clone, Debug)]
    struct GeometricRepresentationContextHolder {
        coordinate_space_dimension: DimensionCount,
        representation_context: PlaceHolder<RepresentationContext>,
    }
    impl Holder for GeometricRepresentationContextHolder {
        type Table = Tables;
        type Owned = GeometricRepresentationContext;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "GEOMETRIC_REPRESENTATION_CONTEXT"
        }
        fn attr_len() -> usize {
            2usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct GeometricRepresentationItem {
        pub representation_item: RepresentationItem,
    }
    #[derive(Clone, Debug)]
    struct GeometricRepresentationItemHolder {
        representation_item: PlaceHolder<RepresentationItem>,
    }
    impl Holder for GeometricRepresentationItemHolder {
        type Table = Tables;
        type Owned = GeometricRepresentationItem;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "GEOMETRIC_REPRESENTATION_ITEM"
        }
        fn attr_len() -> usize {
            1usize
        }
    }
    pub trait GeometricRepresentationItemAny:
        ::std::any::Any + ::std::fmt::Debug + dyn_clone::DynClone
    {
    }
    dyn_clone::clone_trait_object!(GeometricRepresentationItemAny);
    impl GeometricRepresentationItemAny for GeometricRepresentationItem {}
    impl GeometricRepresentationItemAny for GeometricSet {}
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct GeometricSet {
        pub elements: Vec<GeometricSetSelect>,
        pub geometric_representation_item: Box<dyn GeometricRepresentationItemAny>,
    }
    #[derive(Clone, Debug)]
    struct GeometricSetHolder {
        elements: PlaceHolder<Vec<GeometricSetSelect>>,
        geometric_representation_item: PlaceHolder<Box<dyn GeometricRepresentationItemAny>>,
    }
    impl Holder for GeometricSetHolder {
        type Table = Tables;
        type Owned = GeometricSet;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "GEOMETRIC_SET"
        }
        fn attr_len() -> usize {
            2usize
        }
    }
    pub trait GeometricSetAny: ::std::any::Any + ::std::fmt::Debug + dyn_clone::DynClone {}
    dyn_clone::clone_trait_object!(GeometricSetAny);
    impl GeometricSetAny for GeometricSet {}
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct GeometricalToleranceCallout {
        pub draughting_callout: DraughtingCallout,
    }
    #[derive(Clone, Debug)]
    struct GeometricalToleranceCalloutHolder {
        draughting_callout: PlaceHolder<DraughtingCallout>,
    }
    impl Holder for GeometricalToleranceCalloutHolder {
        type Table = Tables;
        type Owned = GeometricalToleranceCallout;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "GEOMETRICAL_TOLERANCE_CALLOUT"
        }
        fn attr_len() -> usize {
            1usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct GeometricallyBounded2DWireframeRepresentation {
        pub shape_representation: ShapeRepresentation,
    }
    #[derive(Clone, Debug)]
    struct GeometricallyBounded2DWireframeRepresentationHolder {
        shape_representation: PlaceHolder<ShapeRepresentation>,
    }
    impl Holder for GeometricallyBounded2DWireframeRepresentationHolder {
        type Table = Tables;
        type Owned = GeometricallyBounded2DWireframeRepresentation;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "GEOMETRICALLY_BOUNDED_2D_WIREFRAME_REPRESENTATION"
        }
        fn attr_len() -> usize {
            1usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct GlobalUnitAssignedContext {
        pub units: Vec<Unit>,
        pub representation_context: RepresentationContext,
    }
    #[derive(Clone, Debug)]
    struct GlobalUnitAssignedContextHolder {
        units: PlaceHolder<Vec<Unit>>,
        representation_context: PlaceHolder<RepresentationContext>,
    }
    impl Holder for GlobalUnitAssignedContextHolder {
        type Table = Tables;
        type Owned = GlobalUnitAssignedContext;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "GLOBAL_UNIT_ASSIGNED_CONTEXT"
        }
        fn attr_len() -> usize {
            2usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct Group {
        pub name: Label,
        pub description: Text,
    }
    #[derive(Clone, Debug)]
    struct GroupHolder {
        name: Label,
        description: Text,
    }
    impl Holder for GroupHolder {
        type Table = Tables;
        type Owned = Group;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "GROUP"
        }
        fn attr_len() -> usize {
            2usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct GroupAssignment {
        pub assigned_group: Group,
    }
    #[derive(Clone, Debug)]
    struct GroupAssignmentHolder {
        assigned_group: PlaceHolder<Group>,
    }
    impl Holder for GroupAssignmentHolder {
        type Table = Tables;
        type Owned = GroupAssignment;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "GROUP_ASSIGNMENT"
        }
        fn attr_len() -> usize {
            1usize
        }
    }
    pub trait GroupAssignmentAny:
        ::std::any::Any + ::std::fmt::Debug + dyn_clone::DynClone
    {
    }
    dyn_clone::clone_trait_object!(GroupAssignmentAny);
    impl GroupAssignmentAny for GroupAssignment {}
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct GroupRelationship {
        pub name: Label,
        pub description: Text,
        pub relating_group: Group,
        pub related_group: Group,
    }
    #[derive(Clone, Debug)]
    struct GroupRelationshipHolder {
        name: Label,
        description: Text,
        relating_group: PlaceHolder<Group>,
        related_group: PlaceHolder<Group>,
    }
    impl Holder for GroupRelationshipHolder {
        type Table = Tables;
        type Owned = GroupRelationship;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "GROUP_RELATIONSHIP"
        }
        fn attr_len() -> usize {
            4usize
        }
    }
    impl ConicAny for Hyperbola {}
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct Hyperbola {
        pub semi_axis: PositiveLengthMeasure,
        pub semi_imag_axis: PositiveLengthMeasure,
        pub conic: Box<dyn ConicAny>,
    }
    #[derive(Clone, Debug)]
    struct HyperbolaHolder {
        semi_axis: PositiveLengthMeasure,
        semi_imag_axis: PositiveLengthMeasure,
        conic: PlaceHolder<Box<dyn ConicAny>>,
    }
    impl Holder for HyperbolaHolder {
        type Table = Tables;
        type Owned = Hyperbola;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "HYPERBOLA"
        }
        fn attr_len() -> usize {
            3usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct Invisibility {
        pub invisible_items: Vec<InvisibleItem>,
    }
    #[derive(Clone, Debug)]
    struct InvisibilityHolder {
        invisible_items: PlaceHolder<Vec<InvisibleItem>>,
    }
    impl Holder for InvisibilityHolder {
        type Table = Tables;
        type Owned = Invisibility;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "INVISIBILITY"
        }
        fn attr_len() -> usize {
            1usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct LeaderCurve {
        pub annotation_curve_occurrence: AnnotationCurveOccurrence,
    }
    #[derive(Clone, Debug)]
    struct LeaderCurveHolder {
        annotation_curve_occurrence: PlaceHolder<AnnotationCurveOccurrence>,
    }
    impl Holder for LeaderCurveHolder {
        type Table = Tables;
        type Owned = LeaderCurve;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "LEADER_CURVE"
        }
        fn attr_len() -> usize {
            1usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct LeaderDirectedCallout {
        pub draughting_callout: DraughtingCallout,
    }
    #[derive(Clone, Debug)]
    struct LeaderDirectedCalloutHolder {
        draughting_callout: PlaceHolder<DraughtingCallout>,
    }
    impl Holder for LeaderDirectedCalloutHolder {
        type Table = Tables;
        type Owned = LeaderDirectedCallout;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "LEADER_DIRECTED_CALLOUT"
        }
        fn attr_len() -> usize {
            1usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct LeaderDirectedDimension {
        pub leader_directed_callout: LeaderDirectedCallout,
    }
    #[derive(Clone, Debug)]
    struct LeaderDirectedDimensionHolder {
        leader_directed_callout: PlaceHolder<LeaderDirectedCallout>,
    }
    impl Holder for LeaderDirectedDimensionHolder {
        type Table = Tables;
        type Owned = LeaderDirectedDimension;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "LEADER_DIRECTED_DIMENSION"
        }
        fn attr_len() -> usize {
            1usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct LeaderTerminator {
        pub terminator_symbol: TerminatorSymbol,
    }
    #[derive(Clone, Debug)]
    struct LeaderTerminatorHolder {
        terminator_symbol: PlaceHolder<TerminatorSymbol>,
    }
    impl Holder for LeaderTerminatorHolder {
        type Table = Tables;
        type Owned = LeaderTerminator;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "LEADER_TERMINATOR"
        }
        fn attr_len() -> usize {
            1usize
        }
    }
    impl MeasureWithUnitAny for LengthMeasureWithUnit {}
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct LengthMeasureWithUnit {
        pub measure_with_unit: Box<dyn MeasureWithUnitAny>,
    }
    #[derive(Clone, Debug)]
    struct LengthMeasureWithUnitHolder {
        measure_with_unit: PlaceHolder<Box<dyn MeasureWithUnitAny>>,
    }
    impl Holder for LengthMeasureWithUnitHolder {
        type Table = Tables;
        type Owned = LengthMeasureWithUnit;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "LENGTH_MEASURE_WITH_UNIT"
        }
        fn attr_len() -> usize {
            1usize
        }
    }
    impl NamedUnitAny for LengthUnit {}
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct LengthUnit {
        pub named_unit: Box<dyn NamedUnitAny>,
    }
    #[derive(Clone, Debug)]
    struct LengthUnitHolder {
        named_unit: PlaceHolder<Box<dyn NamedUnitAny>>,
    }
    impl Holder for LengthUnitHolder {
        type Table = Tables;
        type Owned = LengthUnit;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "LENGTH_UNIT"
        }
        fn attr_len() -> usize {
            1usize
        }
    }
    impl CurveAny for Line {}
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct Line {
        pub pnt: CartesianPoint,
        pub dir: Vector,
        pub curve: Box<dyn CurveAny>,
    }
    #[derive(Clone, Debug)]
    struct LineHolder {
        pnt: PlaceHolder<CartesianPoint>,
        dir: PlaceHolder<Vector>,
        curve: PlaceHolder<Box<dyn CurveAny>>,
    }
    impl Holder for LineHolder {
        type Table = Tables;
        type Owned = Line;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "LINE"
        }
        fn attr_len() -> usize {
            3usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct LinearDimension {
        pub dimension_curve_directed_callout: DimensionCurveDirectedCallout,
    }
    #[derive(Clone, Debug)]
    struct LinearDimensionHolder {
        dimension_curve_directed_callout: PlaceHolder<DimensionCurveDirectedCallout>,
    }
    impl Holder for LinearDimensionHolder {
        type Table = Tables;
        type Owned = LinearDimension;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "LINEAR_DIMENSION"
        }
        fn attr_len() -> usize {
            1usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct MappedItem {
        pub mapping_source: RepresentationMap,
        pub mapping_target: RepresentationItem,
        pub representation_item: RepresentationItem,
    }
    #[derive(Clone, Debug)]
    struct MappedItemHolder {
        mapping_source: PlaceHolder<RepresentationMap>,
        mapping_target: PlaceHolder<RepresentationItem>,
        representation_item: PlaceHolder<RepresentationItem>,
    }
    impl Holder for MappedItemHolder {
        type Table = Tables;
        type Owned = MappedItem;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "MAPPED_ITEM"
        }
        fn attr_len() -> usize {
            3usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct MeasureWithUnit {
        pub value_component: MeasureValue,
        pub unit_component: Unit,
    }
    #[derive(Clone, Debug)]
    struct MeasureWithUnitHolder {
        value_component: PlaceHolder<MeasureValue>,
        unit_component: PlaceHolder<Unit>,
    }
    impl Holder for MeasureWithUnitHolder {
        type Table = Tables;
        type Owned = MeasureWithUnit;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "MEASURE_WITH_UNIT"
        }
        fn attr_len() -> usize {
            2usize
        }
    }
    pub trait MeasureWithUnitAny:
        ::std::any::Any + ::std::fmt::Debug + dyn_clone::DynClone
    {
    }
    dyn_clone::clone_trait_object!(MeasureWithUnitAny);
    impl MeasureWithUnitAny for MeasureWithUnit {}
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct NamedUnit {
        pub dimensions: DimensionalExponents,
    }
    #[derive(Clone, Debug)]
    struct NamedUnitHolder {
        dimensions: PlaceHolder<DimensionalExponents>,
    }
    impl Holder for NamedUnitHolder {
        type Table = Tables;
        type Owned = NamedUnit;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "NAMED_UNIT"
        }
        fn attr_len() -> usize {
            1usize
        }
    }
    pub trait NamedUnitAny: ::std::any::Any + ::std::fmt::Debug + dyn_clone::DynClone {}
    dyn_clone::clone_trait_object!(NamedUnitAny);
    impl NamedUnitAny for NamedUnit {}
    impl CurveAny for OffsetCurve2D {}
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct OffsetCurve2D {
        pub basis_curve: Box<dyn CurveAny>,
        pub distance: LengthMeasure,
        pub self_intersect: Logical,
        pub curve: Box<dyn CurveAny>,
    }
    #[derive(Clone, Debug)]
    struct OffsetCurve2DHolder {
        basis_curve: PlaceHolder<Box<dyn CurveAny>>,
        distance: LengthMeasure,
        self_intersect: Logical,
        curve: PlaceHolder<Box<dyn CurveAny>>,
    }
    impl Holder for OffsetCurve2DHolder {
        type Table = Tables;
        type Owned = OffsetCurve2D;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "OFFSET_CURVE_2D"
        }
        fn attr_len() -> usize {
            4usize
        }
    }
    impl GeometricRepresentationItemAny for OneDirectionRepeatFactor {}
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct OneDirectionRepeatFactor {
        pub repeat_factor: Vector,
        pub geometric_representation_item: Box<dyn GeometricRepresentationItemAny>,
    }
    #[derive(Clone, Debug)]
    struct OneDirectionRepeatFactorHolder {
        repeat_factor: PlaceHolder<Vector>,
        geometric_representation_item: PlaceHolder<Box<dyn GeometricRepresentationItemAny>>,
    }
    impl Holder for OneDirectionRepeatFactorHolder {
        type Table = Tables;
        type Owned = OneDirectionRepeatFactor;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "ONE_DIRECTION_REPEAT_FACTOR"
        }
        fn attr_len() -> usize {
            2usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct OrdinateDimension {
        pub projection_directed_callout: ProjectionDirectedCallout,
    }
    #[derive(Clone, Debug)]
    struct OrdinateDimensionHolder {
        projection_directed_callout: PlaceHolder<ProjectionDirectedCallout>,
    }
    impl Holder for OrdinateDimensionHolder {
        type Table = Tables;
        type Owned = OrdinateDimension;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "ORDINATE_DIMENSION"
        }
        fn attr_len() -> usize {
            1usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct Organization {
        pub id: Option<Identifier>,
        pub name: Label,
        pub description: Text,
    }
    #[derive(Clone, Debug)]
    struct OrganizationHolder {
        id: Option<Identifier>,
        name: Label,
        description: Text,
    }
    impl Holder for OrganizationHolder {
        type Table = Tables;
        type Owned = Organization;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "ORGANIZATION"
        }
        fn attr_len() -> usize {
            3usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct OrganizationAssignment {
        pub assigned_organization: Organization,
        pub role: OrganizationRole,
    }
    #[derive(Clone, Debug)]
    struct OrganizationAssignmentHolder {
        assigned_organization: PlaceHolder<Organization>,
        role: PlaceHolder<OrganizationRole>,
    }
    impl Holder for OrganizationAssignmentHolder {
        type Table = Tables;
        type Owned = OrganizationAssignment;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "ORGANIZATION_ASSIGNMENT"
        }
        fn attr_len() -> usize {
            2usize
        }
    }
    pub trait OrganizationAssignmentAny:
        ::std::any::Any + ::std::fmt::Debug + dyn_clone::DynClone
    {
    }
    dyn_clone::clone_trait_object!(OrganizationAssignmentAny);
    impl OrganizationAssignmentAny for OrganizationAssignment {}
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct OrganizationRole {
        pub name: Label,
    }
    #[derive(Clone, Debug)]
    struct OrganizationRoleHolder {
        name: Label,
    }
    impl Holder for OrganizationRoleHolder {
        type Table = Tables;
        type Owned = OrganizationRole;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "ORGANIZATION_ROLE"
        }
        fn attr_len() -> usize {
            1usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct OrganizationalAddress {
        pub organizations: Vec<Organization>,
        pub description: Text,
        pub address: Address,
    }
    #[derive(Clone, Debug)]
    struct OrganizationalAddressHolder {
        organizations: PlaceHolder<Vec<Organization>>,
        description: Text,
        address: PlaceHolder<Address>,
    }
    impl Holder for OrganizationalAddressHolder {
        type Table = Tables;
        type Owned = OrganizationalAddress;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "ORGANIZATIONAL_ADDRESS"
        }
        fn attr_len() -> usize {
            3usize
        }
    }
    impl ConicAny for Parabola {}
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct Parabola {
        pub focal_dist: LengthMeasure,
        pub conic: Box<dyn ConicAny>,
    }
    #[derive(Clone, Debug)]
    struct ParabolaHolder {
        focal_dist: LengthMeasure,
        conic: PlaceHolder<Box<dyn ConicAny>>,
    }
    impl Holder for ParabolaHolder {
        type Table = Tables;
        type Owned = Parabola;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "PARABOLA"
        }
        fn attr_len() -> usize {
            2usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct Person {
        pub id: Identifier,
        pub last_name: Option<Label>,
        pub first_name: Option<Label>,
        pub middle_names: Option<Vec<Label>>,
        pub prefix_titles: Option<Vec<Label>>,
        pub suffix_titles: Option<Vec<Label>>,
    }
    #[derive(Clone, Debug)]
    struct PersonHolder {
        id: Identifier,
        last_name: Option<Label>,
        first_name: Option<Label>,
        middle_names: Option<PlaceHolder<Vec<Label>>>,
        prefix_titles: Option<PlaceHolder<Vec<Label>>>,
        suffix_titles: Option<PlaceHolder<Vec<Label>>>,
    }
    impl Holder for PersonHolder {
        type Table = Tables;
        type Owned = Person;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "PERSON"
        }
        fn attr_len() -> usize {
            6usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct PersonAndOrganization {
        pub the_person: Person,
        pub the_organization: Organization,
    }
    #[derive(Clone, Debug)]
    struct PersonAndOrganizationHolder {
        the_person: PlaceHolder<Person>,
        the_organization: PlaceHolder<Organization>,
    }
    impl Holder for PersonAndOrganizationHolder {
        type Table = Tables;
        type Owned = PersonAndOrganization;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "PERSON_AND_ORGANIZATION"
        }
        fn attr_len() -> usize {
            2usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct PersonAndOrganizationAssignment {
        pub assigned_person_and_organization: PersonAndOrganization,
        pub role: PersonAndOrganizationRole,
    }
    #[derive(Clone, Debug)]
    struct PersonAndOrganizationAssignmentHolder {
        assigned_person_and_organization: PlaceHolder<PersonAndOrganization>,
        role: PlaceHolder<PersonAndOrganizationRole>,
    }
    impl Holder for PersonAndOrganizationAssignmentHolder {
        type Table = Tables;
        type Owned = PersonAndOrganizationAssignment;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "PERSON_AND_ORGANIZATION_ASSIGNMENT"
        }
        fn attr_len() -> usize {
            2usize
        }
    }
    pub trait PersonAndOrganizationAssignmentAny:
        ::std::any::Any + ::std::fmt::Debug + dyn_clone::DynClone
    {
    }
    dyn_clone::clone_trait_object!(PersonAndOrganizationAssignmentAny);
    impl PersonAndOrganizationAssignmentAny for PersonAndOrganizationAssignment {}
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct PersonAndOrganizationRole {
        pub name: Label,
    }
    #[derive(Clone, Debug)]
    struct PersonAndOrganizationRoleHolder {
        name: Label,
    }
    impl Holder for PersonAndOrganizationRoleHolder {
        type Table = Tables;
        type Owned = PersonAndOrganizationRole;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "PERSON_AND_ORGANIZATION_ROLE"
        }
        fn attr_len() -> usize {
            1usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct PersonAssignment {
        pub assigned_person: Person,
        pub role: PersonRole,
    }
    #[derive(Clone, Debug)]
    struct PersonAssignmentHolder {
        assigned_person: PlaceHolder<Person>,
        role: PlaceHolder<PersonRole>,
    }
    impl Holder for PersonAssignmentHolder {
        type Table = Tables;
        type Owned = PersonAssignment;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "PERSON_ASSIGNMENT"
        }
        fn attr_len() -> usize {
            2usize
        }
    }
    pub trait PersonAssignmentAny:
        ::std::any::Any + ::std::fmt::Debug + dyn_clone::DynClone
    {
    }
    dyn_clone::clone_trait_object!(PersonAssignmentAny);
    impl PersonAssignmentAny for PersonAssignment {}
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct PersonRole {
        pub name: Label,
    }
    #[derive(Clone, Debug)]
    struct PersonRoleHolder {
        name: Label,
    }
    impl Holder for PersonRoleHolder {
        type Table = Tables;
        type Owned = PersonRole;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "PERSON_ROLE"
        }
        fn attr_len() -> usize {
            1usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct PersonalAddress {
        pub people: Vec<Person>,
        pub description: Text,
        pub address: Address,
    }
    #[derive(Clone, Debug)]
    struct PersonalAddressHolder {
        people: PlaceHolder<Vec<Person>>,
        description: Text,
        address: PlaceHolder<Address>,
    }
    impl Holder for PersonalAddressHolder {
        type Table = Tables;
        type Owned = PersonalAddress;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "PERSONAL_ADDRESS"
        }
        fn attr_len() -> usize {
            3usize
        }
    }
    impl GeometricRepresentationItemAny for Placement {}
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct Placement {
        pub location: CartesianPoint,
        pub geometric_representation_item: Box<dyn GeometricRepresentationItemAny>,
    }
    #[derive(Clone, Debug)]
    struct PlacementHolder {
        location: PlaceHolder<CartesianPoint>,
        geometric_representation_item: PlaceHolder<Box<dyn GeometricRepresentationItemAny>>,
    }
    impl Holder for PlacementHolder {
        type Table = Tables;
        type Owned = Placement;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "PLACEMENT"
        }
        fn attr_len() -> usize {
            2usize
        }
    }
    pub trait PlacementAny: ::std::any::Any + ::std::fmt::Debug + dyn_clone::DynClone {}
    dyn_clone::clone_trait_object!(PlacementAny);
    impl PlacementAny for Placement {}
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct PlanarBox {
        pub placement: Axis2Placement,
        pub planar_extent: PlanarExtent,
    }
    #[derive(Clone, Debug)]
    struct PlanarBoxHolder {
        placement: PlaceHolder<Axis2Placement>,
        planar_extent: PlaceHolder<PlanarExtent>,
    }
    impl Holder for PlanarBoxHolder {
        type Table = Tables;
        type Owned = PlanarBox;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "PLANAR_BOX"
        }
        fn attr_len() -> usize {
            2usize
        }
    }
    impl GeometricRepresentationItemAny for PlanarExtent {}
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct PlanarExtent {
        pub size_in_x: LengthMeasure,
        pub size_in_y: LengthMeasure,
        pub geometric_representation_item: Box<dyn GeometricRepresentationItemAny>,
    }
    #[derive(Clone, Debug)]
    struct PlanarExtentHolder {
        size_in_x: LengthMeasure,
        size_in_y: LengthMeasure,
        geometric_representation_item: PlaceHolder<Box<dyn GeometricRepresentationItemAny>>,
    }
    impl Holder for PlanarExtentHolder {
        type Table = Tables;
        type Owned = PlanarExtent;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "PLANAR_EXTENT"
        }
        fn attr_len() -> usize {
            3usize
        }
    }
    impl MeasureWithUnitAny for PlaneAngleMeasureWithUnit {}
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct PlaneAngleMeasureWithUnit {
        pub measure_with_unit: Box<dyn MeasureWithUnitAny>,
    }
    #[derive(Clone, Debug)]
    struct PlaneAngleMeasureWithUnitHolder {
        measure_with_unit: PlaceHolder<Box<dyn MeasureWithUnitAny>>,
    }
    impl Holder for PlaneAngleMeasureWithUnitHolder {
        type Table = Tables;
        type Owned = PlaneAngleMeasureWithUnit;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "PLANE_ANGLE_MEASURE_WITH_UNIT"
        }
        fn attr_len() -> usize {
            1usize
        }
    }
    impl NamedUnitAny for PlaneAngleUnit {}
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct PlaneAngleUnit {
        pub named_unit: Box<dyn NamedUnitAny>,
    }
    #[derive(Clone, Debug)]
    struct PlaneAngleUnitHolder {
        named_unit: PlaceHolder<Box<dyn NamedUnitAny>>,
    }
    impl Holder for PlaneAngleUnitHolder {
        type Table = Tables;
        type Owned = PlaneAngleUnit;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "PLANE_ANGLE_UNIT"
        }
        fn attr_len() -> usize {
            1usize
        }
    }
    impl GeometricRepresentationItemAny for Point {}
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct Point {
        pub geometric_representation_item: Box<dyn GeometricRepresentationItemAny>,
    }
    #[derive(Clone, Debug)]
    struct PointHolder {
        geometric_representation_item: PlaceHolder<Box<dyn GeometricRepresentationItemAny>>,
    }
    impl Holder for PointHolder {
        type Table = Tables;
        type Owned = Point;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "POINT"
        }
        fn attr_len() -> usize {
            1usize
        }
    }
    pub trait PointAny: ::std::any::Any + ::std::fmt::Debug + dyn_clone::DynClone {}
    dyn_clone::clone_trait_object!(PointAny);
    impl PointAny for Point {}
    impl PointAny for PointOnCurve {}
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct PointOnCurve {
        pub basis_curve: Box<dyn CurveAny>,
        pub point_parameter: ParameterValue,
        pub point: Box<dyn PointAny>,
    }
    #[derive(Clone, Debug)]
    struct PointOnCurveHolder {
        basis_curve: PlaceHolder<Box<dyn CurveAny>>,
        point_parameter: ParameterValue,
        point: PlaceHolder<Box<dyn PointAny>>,
    }
    impl Holder for PointOnCurveHolder {
        type Table = Tables;
        type Owned = PointOnCurve;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "POINT_ON_CURVE"
        }
        fn attr_len() -> usize {
            3usize
        }
    }
    impl BoundedCurveAny for Polyline {}
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct Polyline {
        pub points: Vec<CartesianPoint>,
        pub bounded_curve: Box<dyn BoundedCurveAny>,
    }
    #[derive(Clone, Debug)]
    struct PolylineHolder {
        points: PlaceHolder<Vec<CartesianPoint>>,
        bounded_curve: PlaceHolder<Box<dyn BoundedCurveAny>>,
    }
    impl Holder for PolylineHolder {
        type Table = Tables;
        type Owned = Polyline;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "POLYLINE"
        }
        fn attr_len() -> usize {
            2usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct PreDefinedColour {
        pub pre_defined_item: PreDefinedItem,
        pub colour: Colour,
    }
    #[derive(Clone, Debug)]
    struct PreDefinedColourHolder {
        pre_defined_item: PlaceHolder<PreDefinedItem>,
        colour: PlaceHolder<Colour>,
    }
    impl Holder for PreDefinedColourHolder {
        type Table = Tables;
        type Owned = PreDefinedColour;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "PRE_DEFINED_COLOUR"
        }
        fn attr_len() -> usize {
            2usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct PreDefinedCurveFont {
        pub pre_defined_item: PreDefinedItem,
    }
    #[derive(Clone, Debug)]
    struct PreDefinedCurveFontHolder {
        pre_defined_item: PlaceHolder<PreDefinedItem>,
    }
    impl Holder for PreDefinedCurveFontHolder {
        type Table = Tables;
        type Owned = PreDefinedCurveFont;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "PRE_DEFINED_CURVE_FONT"
        }
        fn attr_len() -> usize {
            1usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct PreDefinedDimensionSymbol {
        pub pre_defined_symbol: PreDefinedSymbol,
    }
    #[derive(Clone, Debug)]
    struct PreDefinedDimensionSymbolHolder {
        pre_defined_symbol: PlaceHolder<PreDefinedSymbol>,
    }
    impl Holder for PreDefinedDimensionSymbolHolder {
        type Table = Tables;
        type Owned = PreDefinedDimensionSymbol;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "PRE_DEFINED_DIMENSION_SYMBOL"
        }
        fn attr_len() -> usize {
            1usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct PreDefinedGeometricalToleranceSymbol {
        pub pre_defined_symbol: PreDefinedSymbol,
    }
    #[derive(Clone, Debug)]
    struct PreDefinedGeometricalToleranceSymbolHolder {
        pre_defined_symbol: PlaceHolder<PreDefinedSymbol>,
    }
    impl Holder for PreDefinedGeometricalToleranceSymbolHolder {
        type Table = Tables;
        type Owned = PreDefinedGeometricalToleranceSymbol;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "PRE_DEFINED_GEOMETRICAL_TOLERANCE_SYMBOL"
        }
        fn attr_len() -> usize {
            1usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct PreDefinedItem {
        pub name: Label,
    }
    #[derive(Clone, Debug)]
    struct PreDefinedItemHolder {
        name: Label,
    }
    impl Holder for PreDefinedItemHolder {
        type Table = Tables;
        type Owned = PreDefinedItem;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "PRE_DEFINED_ITEM"
        }
        fn attr_len() -> usize {
            1usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct PreDefinedPointMarkerSymbol {
        pub pre_defined_symbol: PreDefinedSymbol,
    }
    #[derive(Clone, Debug)]
    struct PreDefinedPointMarkerSymbolHolder {
        pre_defined_symbol: PlaceHolder<PreDefinedSymbol>,
    }
    impl Holder for PreDefinedPointMarkerSymbolHolder {
        type Table = Tables;
        type Owned = PreDefinedPointMarkerSymbol;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "PRE_DEFINED_POINT_MARKER_SYMBOL"
        }
        fn attr_len() -> usize {
            1usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct PreDefinedSymbol {
        pub pre_defined_item: PreDefinedItem,
    }
    #[derive(Clone, Debug)]
    struct PreDefinedSymbolHolder {
        pre_defined_item: PlaceHolder<PreDefinedItem>,
    }
    impl Holder for PreDefinedSymbolHolder {
        type Table = Tables;
        type Owned = PreDefinedSymbol;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "PRE_DEFINED_SYMBOL"
        }
        fn attr_len() -> usize {
            1usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct PreDefinedTerminatorSymbol {
        pub pre_defined_symbol: PreDefinedSymbol,
    }
    #[derive(Clone, Debug)]
    struct PreDefinedTerminatorSymbolHolder {
        pre_defined_symbol: PlaceHolder<PreDefinedSymbol>,
    }
    impl Holder for PreDefinedTerminatorSymbolHolder {
        type Table = Tables;
        type Owned = PreDefinedTerminatorSymbol;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "PRE_DEFINED_TERMINATOR_SYMBOL"
        }
        fn attr_len() -> usize {
            1usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct PreDefinedTextFont {
        pub pre_defined_item: PreDefinedItem,
    }
    #[derive(Clone, Debug)]
    struct PreDefinedTextFontHolder {
        pre_defined_item: PlaceHolder<PreDefinedItem>,
    }
    impl Holder for PreDefinedTextFontHolder {
        type Table = Tables;
        type Owned = PreDefinedTextFont;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "PRE_DEFINED_TEXT_FONT"
        }
        fn attr_len() -> usize {
            1usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct PresentationArea {
        pub presentation_representation: PresentationRepresentation,
    }
    #[derive(Clone, Debug)]
    struct PresentationAreaHolder {
        presentation_representation: PlaceHolder<PresentationRepresentation>,
    }
    impl Holder for PresentationAreaHolder {
        type Table = Tables;
        type Owned = PresentationArea;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "PRESENTATION_AREA"
        }
        fn attr_len() -> usize {
            1usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct PresentationLayerAssignment {
        pub name: Label,
        pub description: Text,
        pub assigned_items: Vec<LayeredItem>,
    }
    #[derive(Clone, Debug)]
    struct PresentationLayerAssignmentHolder {
        name: Label,
        description: Text,
        assigned_items: PlaceHolder<Vec<LayeredItem>>,
    }
    impl Holder for PresentationLayerAssignmentHolder {
        type Table = Tables;
        type Owned = PresentationLayerAssignment;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "PRESENTATION_LAYER_ASSIGNMENT"
        }
        fn attr_len() -> usize {
            3usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct PresentationLayerUsage {
        pub assignment: PresentationLayerAssignment,
        pub presentation: PresentationRepresentation,
    }
    #[derive(Clone, Debug)]
    struct PresentationLayerUsageHolder {
        assignment: PlaceHolder<PresentationLayerAssignment>,
        presentation: PlaceHolder<PresentationRepresentation>,
    }
    impl Holder for PresentationLayerUsageHolder {
        type Table = Tables;
        type Owned = PresentationLayerUsage;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "PRESENTATION_LAYER_USAGE"
        }
        fn attr_len() -> usize {
            2usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct PresentationRepresentation {
        pub representation: Representation,
    }
    #[derive(Clone, Debug)]
    struct PresentationRepresentationHolder {
        representation: PlaceHolder<Representation>,
    }
    impl Holder for PresentationRepresentationHolder {
        type Table = Tables;
        type Owned = PresentationRepresentation;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "PRESENTATION_REPRESENTATION"
        }
        fn attr_len() -> usize {
            1usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct PresentationSet {}
    #[derive(Clone, Debug)]
    struct PresentationSetHolder {}
    impl Holder for PresentationSetHolder {
        type Table = Tables;
        type Owned = PresentationSet;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "PRESENTATION_SET"
        }
        fn attr_len() -> usize {
            0usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct PresentationSize {
        pub unit: PresentationSizeAssignmentSelect,
        pub size: PlanarBox,
    }
    #[derive(Clone, Debug)]
    struct PresentationSizeHolder {
        unit: PlaceHolder<PresentationSizeAssignmentSelect>,
        size: PlaceHolder<PlanarBox>,
    }
    impl Holder for PresentationSizeHolder {
        type Table = Tables;
        type Owned = PresentationSize;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "PRESENTATION_SIZE"
        }
        fn attr_len() -> usize {
            2usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct PresentationStyleAssignment {
        pub styles: Vec<PresentationStyleSelect>,
    }
    #[derive(Clone, Debug)]
    struct PresentationStyleAssignmentHolder {
        styles: PlaceHolder<Vec<PresentationStyleSelect>>,
    }
    impl Holder for PresentationStyleAssignmentHolder {
        type Table = Tables;
        type Owned = PresentationStyleAssignment;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "PRESENTATION_STYLE_ASSIGNMENT"
        }
        fn attr_len() -> usize {
            1usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct PresentationStyleByContext {
        pub style_context: StyleContextSelect,
        pub presentation_style_assignment: PresentationStyleAssignment,
    }
    #[derive(Clone, Debug)]
    struct PresentationStyleByContextHolder {
        style_context: PlaceHolder<StyleContextSelect>,
        presentation_style_assignment: PlaceHolder<PresentationStyleAssignment>,
    }
    impl Holder for PresentationStyleByContextHolder {
        type Table = Tables;
        type Owned = PresentationStyleByContext;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "PRESENTATION_STYLE_BY_CONTEXT"
        }
        fn attr_len() -> usize {
            2usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct PresentationView {
        pub presentation_representation: PresentationRepresentation,
    }
    #[derive(Clone, Debug)]
    struct PresentationViewHolder {
        presentation_representation: PlaceHolder<PresentationRepresentation>,
    }
    impl Holder for PresentationViewHolder {
        type Table = Tables;
        type Owned = PresentationView;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "PRESENTATION_VIEW"
        }
        fn attr_len() -> usize {
            1usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct PresentedItem {}
    #[derive(Clone, Debug)]
    struct PresentedItemHolder {}
    impl Holder for PresentedItemHolder {
        type Table = Tables;
        type Owned = PresentedItem;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "PRESENTED_ITEM"
        }
        fn attr_len() -> usize {
            0usize
        }
    }
    pub trait PresentedItemAny: ::std::any::Any + ::std::fmt::Debug + dyn_clone::DynClone {}
    dyn_clone::clone_trait_object!(PresentedItemAny);
    impl PresentedItemAny for PresentedItem {}
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct PresentedItemRepresentation {
        pub presentation: PresentationRepresentationSelect,
        pub item: Box<dyn PresentedItemAny>,
    }
    #[derive(Clone, Debug)]
    struct PresentedItemRepresentationHolder {
        presentation: PlaceHolder<PresentationRepresentationSelect>,
        item: PlaceHolder<Box<dyn PresentedItemAny>>,
    }
    impl Holder for PresentedItemRepresentationHolder {
        type Table = Tables;
        type Owned = PresentedItemRepresentation;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "PRESENTED_ITEM_REPRESENTATION"
        }
        fn attr_len() -> usize {
            2usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct Product {
        pub id: Identifier,
        pub name: Label,
        pub description: Text,
        pub frame_of_reference: Vec<ProductContext>,
    }
    #[derive(Clone, Debug)]
    struct ProductHolder {
        id: Identifier,
        name: Label,
        description: Text,
        frame_of_reference: PlaceHolder<Vec<ProductContext>>,
    }
    impl Holder for ProductHolder {
        type Table = Tables;
        type Owned = Product;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "PRODUCT"
        }
        fn attr_len() -> usize {
            4usize
        }
    }
    impl ApplicationContextElementAny for ProductContext {}
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct ProductContext {
        pub discipline_type: Label,
        pub application_context_element: Box<dyn ApplicationContextElementAny>,
    }
    #[derive(Clone, Debug)]
    struct ProductContextHolder {
        discipline_type: Label,
        application_context_element: PlaceHolder<Box<dyn ApplicationContextElementAny>>,
    }
    impl Holder for ProductContextHolder {
        type Table = Tables;
        type Owned = ProductContext;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "PRODUCT_CONTEXT"
        }
        fn attr_len() -> usize {
            2usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct ProductDefinition {
        pub id: Identifier,
        pub description: Text,
        pub formation: ProductDefinitionFormation,
        pub frame_of_reference: ProductDefinitionContext,
    }
    #[derive(Clone, Debug)]
    struct ProductDefinitionHolder {
        id: Identifier,
        description: Text,
        formation: PlaceHolder<ProductDefinitionFormation>,
        frame_of_reference: PlaceHolder<ProductDefinitionContext>,
    }
    impl Holder for ProductDefinitionHolder {
        type Table = Tables;
        type Owned = ProductDefinition;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "PRODUCT_DEFINITION"
        }
        fn attr_len() -> usize {
            4usize
        }
    }
    impl ApplicationContextElementAny for ProductDefinitionContext {}
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct ProductDefinitionContext {
        pub life_cycle_stage: Label,
        pub application_context_element: Box<dyn ApplicationContextElementAny>,
    }
    #[derive(Clone, Debug)]
    struct ProductDefinitionContextHolder {
        life_cycle_stage: Label,
        application_context_element: PlaceHolder<Box<dyn ApplicationContextElementAny>>,
    }
    impl Holder for ProductDefinitionContextHolder {
        type Table = Tables;
        type Owned = ProductDefinitionContext;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "PRODUCT_DEFINITION_CONTEXT"
        }
        fn attr_len() -> usize {
            2usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct ProductDefinitionFormation {
        pub id: Identifier,
        pub description: Text,
        pub of_product: Product,
    }
    #[derive(Clone, Debug)]
    struct ProductDefinitionFormationHolder {
        id: Identifier,
        description: Text,
        of_product: PlaceHolder<Product>,
    }
    impl Holder for ProductDefinitionFormationHolder {
        type Table = Tables;
        type Owned = ProductDefinitionFormation;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "PRODUCT_DEFINITION_FORMATION"
        }
        fn attr_len() -> usize {
            3usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct ProductDefinitionShape {
        pub property_definition: PropertyDefinition,
    }
    #[derive(Clone, Debug)]
    struct ProductDefinitionShapeHolder {
        property_definition: PlaceHolder<PropertyDefinition>,
    }
    impl Holder for ProductDefinitionShapeHolder {
        type Table = Tables;
        type Owned = ProductDefinitionShape;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "PRODUCT_DEFINITION_SHAPE"
        }
        fn attr_len() -> usize {
            1usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct ProjectionCurve {
        pub annotation_curve_occurrence: AnnotationCurveOccurrence,
    }
    #[derive(Clone, Debug)]
    struct ProjectionCurveHolder {
        annotation_curve_occurrence: PlaceHolder<AnnotationCurveOccurrence>,
    }
    impl Holder for ProjectionCurveHolder {
        type Table = Tables;
        type Owned = ProjectionCurve;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "PROJECTION_CURVE"
        }
        fn attr_len() -> usize {
            1usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct ProjectionDirectedCallout {
        pub draughting_callout: DraughtingCallout,
    }
    #[derive(Clone, Debug)]
    struct ProjectionDirectedCalloutHolder {
        draughting_callout: PlaceHolder<DraughtingCallout>,
    }
    impl Holder for ProjectionDirectedCalloutHolder {
        type Table = Tables;
        type Owned = ProjectionDirectedCallout;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "PROJECTION_DIRECTED_CALLOUT"
        }
        fn attr_len() -> usize {
            1usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct PropertyDefinition {
        pub name: Label,
        pub description: Text,
        pub definition: CharacterizedDefinition,
    }
    #[derive(Clone, Debug)]
    struct PropertyDefinitionHolder {
        name: Label,
        description: Text,
        definition: PlaceHolder<CharacterizedDefinition>,
    }
    impl Holder for PropertyDefinitionHolder {
        type Table = Tables;
        type Owned = PropertyDefinition;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "PROPERTY_DEFINITION"
        }
        fn attr_len() -> usize {
            3usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct PropertyDefinitionRepresentation {
        pub definition: PropertyDefinition,
        pub used_representation: Representation,
    }
    #[derive(Clone, Debug)]
    struct PropertyDefinitionRepresentationHolder {
        definition: PlaceHolder<PropertyDefinition>,
        used_representation: PlaceHolder<Representation>,
    }
    impl Holder for PropertyDefinitionRepresentationHolder {
        type Table = Tables;
        type Owned = PropertyDefinitionRepresentation;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "PROPERTY_DEFINITION_REPRESENTATION"
        }
        fn attr_len() -> usize {
            2usize
        }
    }
    impl BSplineCurveAny for QuasiUniformCurve {}
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct QuasiUniformCurve {
        pub b_spline_curve: Box<dyn BSplineCurveAny>,
    }
    #[derive(Clone, Debug)]
    struct QuasiUniformCurveHolder {
        b_spline_curve: PlaceHolder<Box<dyn BSplineCurveAny>>,
    }
    impl Holder for QuasiUniformCurveHolder {
        type Table = Tables;
        type Owned = QuasiUniformCurve;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "QUASI_UNIFORM_CURVE"
        }
        fn attr_len() -> usize {
            1usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct RadiusDimension {
        pub dimension_curve_directed_callout: DimensionCurveDirectedCallout,
    }
    #[derive(Clone, Debug)]
    struct RadiusDimensionHolder {
        dimension_curve_directed_callout: PlaceHolder<DimensionCurveDirectedCallout>,
    }
    impl Holder for RadiusDimensionHolder {
        type Table = Tables;
        type Owned = RadiusDimension;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "RADIUS_DIMENSION"
        }
        fn attr_len() -> usize {
            1usize
        }
    }
    impl BSplineCurveAny for RationalBSplineCurve {}
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct RationalBSplineCurve {
        pub weights_data: Vec<f64>,
        pub b_spline_curve: Box<dyn BSplineCurveAny>,
    }
    #[derive(Clone, Debug)]
    struct RationalBSplineCurveHolder {
        weights_data: PlaceHolder<Vec<f64>>,
        b_spline_curve: PlaceHolder<Box<dyn BSplineCurveAny>>,
    }
    impl Holder for RationalBSplineCurveHolder {
        type Table = Tables;
        type Owned = RationalBSplineCurve;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "RATIONAL_B_SPLINE_CURVE"
        }
        fn attr_len() -> usize {
            2usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct Representation {
        pub name: Label,
        pub items: Vec<RepresentationItem>,
        pub context_of_items: RepresentationContext,
    }
    #[derive(Clone, Debug)]
    struct RepresentationHolder {
        name: Label,
        items: PlaceHolder<Vec<RepresentationItem>>,
        context_of_items: PlaceHolder<RepresentationContext>,
    }
    impl Holder for RepresentationHolder {
        type Table = Tables;
        type Owned = Representation;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "REPRESENTATION"
        }
        fn attr_len() -> usize {
            3usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct RepresentationContext {
        pub context_identifier: Identifier,
        pub context_type: Text,
    }
    #[derive(Clone, Debug)]
    struct RepresentationContextHolder {
        context_identifier: Identifier,
        context_type: Text,
    }
    impl Holder for RepresentationContextHolder {
        type Table = Tables;
        type Owned = RepresentationContext;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "REPRESENTATION_CONTEXT"
        }
        fn attr_len() -> usize {
            2usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct RepresentationItem {
        pub name: Label,
    }
    #[derive(Clone, Debug)]
    struct RepresentationItemHolder {
        name: Label,
    }
    impl Holder for RepresentationItemHolder {
        type Table = Tables;
        type Owned = RepresentationItem;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "REPRESENTATION_ITEM"
        }
        fn attr_len() -> usize {
            1usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct RepresentationMap {
        pub mapping_origin: RepresentationItem,
        pub mapped_representation: Representation,
    }
    #[derive(Clone, Debug)]
    struct RepresentationMapHolder {
        mapping_origin: PlaceHolder<RepresentationItem>,
        mapped_representation: PlaceHolder<Representation>,
    }
    impl Holder for RepresentationMapHolder {
        type Table = Tables;
        type Owned = RepresentationMap;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "REPRESENTATION_MAP"
        }
        fn attr_len() -> usize {
            2usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct SecurityClassification {
        pub name: Label,
        pub purpose: Text,
        pub security_level: SecurityClassificationLevel,
    }
    #[derive(Clone, Debug)]
    struct SecurityClassificationHolder {
        name: Label,
        purpose: Text,
        security_level: PlaceHolder<SecurityClassificationLevel>,
    }
    impl Holder for SecurityClassificationHolder {
        type Table = Tables;
        type Owned = SecurityClassification;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "SECURITY_CLASSIFICATION"
        }
        fn attr_len() -> usize {
            3usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct SecurityClassificationAssignment {
        pub assigned_security_classification: SecurityClassification,
    }
    #[derive(Clone, Debug)]
    struct SecurityClassificationAssignmentHolder {
        assigned_security_classification: PlaceHolder<SecurityClassification>,
    }
    impl Holder for SecurityClassificationAssignmentHolder {
        type Table = Tables;
        type Owned = SecurityClassificationAssignment;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "SECURITY_CLASSIFICATION_ASSIGNMENT"
        }
        fn attr_len() -> usize {
            1usize
        }
    }
    pub trait SecurityClassificationAssignmentAny:
        ::std::any::Any + ::std::fmt::Debug + dyn_clone::DynClone
    {
    }
    dyn_clone::clone_trait_object!(SecurityClassificationAssignmentAny);
    impl SecurityClassificationAssignmentAny for SecurityClassificationAssignment {}
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct SecurityClassificationLevel {
        pub name: Label,
    }
    #[derive(Clone, Debug)]
    struct SecurityClassificationLevelHolder {
        name: Label,
    }
    impl Holder for SecurityClassificationLevelHolder {
        type Table = Tables;
        type Owned = SecurityClassificationLevel;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "SECURITY_CLASSIFICATION_LEVEL"
        }
        fn attr_len() -> usize {
            1usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct ShapeDefinitionRepresentation {
        pub property_definition_representation: PropertyDefinitionRepresentation,
    }
    #[derive(Clone, Debug)]
    struct ShapeDefinitionRepresentationHolder {
        property_definition_representation: PlaceHolder<PropertyDefinitionRepresentation>,
    }
    impl Holder for ShapeDefinitionRepresentationHolder {
        type Table = Tables;
        type Owned = ShapeDefinitionRepresentation;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "SHAPE_DEFINITION_REPRESENTATION"
        }
        fn attr_len() -> usize {
            1usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct ShapeRepresentation {
        pub representation: Representation,
    }
    #[derive(Clone, Debug)]
    struct ShapeRepresentationHolder {
        representation: PlaceHolder<Representation>,
    }
    impl Holder for ShapeRepresentationHolder {
        type Table = Tables;
        type Owned = ShapeRepresentation;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "SHAPE_REPRESENTATION"
        }
        fn attr_len() -> usize {
            1usize
        }
    }
    impl NamedUnitAny for SiUnit {}
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct SiUnit {
        pub prefix: Option<SiPrefix>,
        pub name: SiUnitName,
        pub named_unit: Box<dyn NamedUnitAny>,
    }
    #[derive(Clone, Debug)]
    struct SiUnitHolder {
        prefix: Option<PlaceHolder<SiPrefix>>,
        name: PlaceHolder<SiUnitName>,
        named_unit: PlaceHolder<Box<dyn NamedUnitAny>>,
    }
    impl Holder for SiUnitHolder {
        type Table = Tables;
        type Owned = SiUnit;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "SI_UNIT"
        }
        fn attr_len() -> usize {
            3usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct StructuredDimensionCallout {
        pub draughting_callout: DraughtingCallout,
    }
    #[derive(Clone, Debug)]
    struct StructuredDimensionCalloutHolder {
        draughting_callout: PlaceHolder<DraughtingCallout>,
    }
    impl Holder for StructuredDimensionCalloutHolder {
        type Table = Tables;
        type Owned = StructuredDimensionCallout;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "STRUCTURED_DIMENSION_CALLOUT"
        }
        fn attr_len() -> usize {
            1usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct StyledItem {
        pub styles: Vec<PresentationStyleAssignment>,
        pub item: RepresentationItem,
        pub representation_item: RepresentationItem,
    }
    #[derive(Clone, Debug)]
    struct StyledItemHolder {
        styles: PlaceHolder<Vec<PresentationStyleAssignment>>,
        item: PlaceHolder<RepresentationItem>,
        representation_item: PlaceHolder<RepresentationItem>,
    }
    impl Holder for StyledItemHolder {
        type Table = Tables;
        type Owned = StyledItem;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "STYLED_ITEM"
        }
        fn attr_len() -> usize {
            3usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct SymbolColour {
        pub colour_of_symbol: Colour,
    }
    #[derive(Clone, Debug)]
    struct SymbolColourHolder {
        colour_of_symbol: PlaceHolder<Colour>,
    }
    impl Holder for SymbolColourHolder {
        type Table = Tables;
        type Owned = SymbolColour;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "SYMBOL_COLOUR"
        }
        fn attr_len() -> usize {
            1usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct SymbolRepresentation {
        pub representation: Representation,
    }
    #[derive(Clone, Debug)]
    struct SymbolRepresentationHolder {
        representation: PlaceHolder<Representation>,
    }
    impl Holder for SymbolRepresentationHolder {
        type Table = Tables;
        type Owned = SymbolRepresentation;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "SYMBOL_REPRESENTATION"
        }
        fn attr_len() -> usize {
            1usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct SymbolRepresentationMap {
        pub representation_map: RepresentationMap,
    }
    #[derive(Clone, Debug)]
    struct SymbolRepresentationMapHolder {
        representation_map: PlaceHolder<RepresentationMap>,
    }
    impl Holder for SymbolRepresentationMapHolder {
        type Table = Tables;
        type Owned = SymbolRepresentationMap;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "SYMBOL_REPRESENTATION_MAP"
        }
        fn attr_len() -> usize {
            1usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct SymbolStyle {
        pub name: Label,
        pub style_of_symbol: SymbolStyleSelect,
    }
    #[derive(Clone, Debug)]
    struct SymbolStyleHolder {
        name: Label,
        style_of_symbol: PlaceHolder<SymbolStyleSelect>,
    }
    impl Holder for SymbolStyleHolder {
        type Table = Tables;
        type Owned = SymbolStyle;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "SYMBOL_STYLE"
        }
        fn attr_len() -> usize {
            2usize
        }
    }
    impl GeometricRepresentationItemAny for SymbolTarget {}
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct SymbolTarget {
        pub placement: Axis2Placement,
        pub x_scale: PositiveRatioMeasure,
        pub y_scale: PositiveRatioMeasure,
        pub geometric_representation_item: Box<dyn GeometricRepresentationItemAny>,
    }
    #[derive(Clone, Debug)]
    struct SymbolTargetHolder {
        placement: PlaceHolder<Axis2Placement>,
        x_scale: PositiveRatioMeasure,
        y_scale: PositiveRatioMeasure,
        geometric_representation_item: PlaceHolder<Box<dyn GeometricRepresentationItemAny>>,
    }
    impl Holder for SymbolTargetHolder {
        type Table = Tables;
        type Owned = SymbolTarget;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "SYMBOL_TARGET"
        }
        fn attr_len() -> usize {
            4usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct TerminatorSymbol {
        pub annotated_curve: AnnotationCurveOccurrence,
        pub annotation_symbol_occurrence: AnnotationSymbolOccurrence,
    }
    #[derive(Clone, Debug)]
    struct TerminatorSymbolHolder {
        annotated_curve: PlaceHolder<AnnotationCurveOccurrence>,
        annotation_symbol_occurrence: PlaceHolder<AnnotationSymbolOccurrence>,
    }
    impl Holder for TerminatorSymbolHolder {
        type Table = Tables;
        type Owned = TerminatorSymbol;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "TERMINATOR_SYMBOL"
        }
        fn attr_len() -> usize {
            2usize
        }
    }
    impl GeometricRepresentationItemAny for TextLiteral {}
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct TextLiteral {
        pub literal: PresentableText,
        pub placement: Axis2Placement,
        pub alignment: TextAlignment,
        pub path: TextPath,
        pub font: FontSelect,
        pub geometric_representation_item: Box<dyn GeometricRepresentationItemAny>,
    }
    #[derive(Clone, Debug)]
    struct TextLiteralHolder {
        literal: PresentableText,
        placement: PlaceHolder<Axis2Placement>,
        alignment: TextAlignment,
        path: PlaceHolder<TextPath>,
        font: PlaceHolder<FontSelect>,
        geometric_representation_item: PlaceHolder<Box<dyn GeometricRepresentationItemAny>>,
    }
    impl Holder for TextLiteralHolder {
        type Table = Tables;
        type Owned = TextLiteral;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "TEXT_LITERAL"
        }
        fn attr_len() -> usize {
            6usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct TextLiteralWithAssociatedCurves {
        pub associated_curves: Vec<Box<dyn CurveAny>>,
        pub text_literal: TextLiteral,
    }
    #[derive(Clone, Debug)]
    struct TextLiteralWithAssociatedCurvesHolder {
        associated_curves: PlaceHolder<Vec<Box<dyn CurveAny>>>,
        text_literal: PlaceHolder<TextLiteral>,
    }
    impl Holder for TextLiteralWithAssociatedCurvesHolder {
        type Table = Tables;
        type Owned = TextLiteralWithAssociatedCurves;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "TEXT_LITERAL_WITH_ASSOCIATED_CURVES"
        }
        fn attr_len() -> usize {
            2usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct TextLiteralWithBlankingBox {
        pub blanking: PlanarBox,
        pub text_literal: TextLiteral,
    }
    #[derive(Clone, Debug)]
    struct TextLiteralWithBlankingBoxHolder {
        blanking: PlaceHolder<PlanarBox>,
        text_literal: PlaceHolder<TextLiteral>,
    }
    impl Holder for TextLiteralWithBlankingBoxHolder {
        type Table = Tables;
        type Owned = TextLiteralWithBlankingBox;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "TEXT_LITERAL_WITH_BLANKING_BOX"
        }
        fn attr_len() -> usize {
            2usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct TextLiteralWithDelineation {
        pub delineation: TextDelineation,
        pub text_literal: TextLiteral,
    }
    #[derive(Clone, Debug)]
    struct TextLiteralWithDelineationHolder {
        delineation: TextDelineation,
        text_literal: PlaceHolder<TextLiteral>,
    }
    impl Holder for TextLiteralWithDelineationHolder {
        type Table = Tables;
        type Owned = TextLiteralWithDelineation;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "TEXT_LITERAL_WITH_DELINEATION"
        }
        fn attr_len() -> usize {
            2usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct TextLiteralWithExtent {
        pub extent: PlanarExtent,
        pub text_literal: TextLiteral,
    }
    #[derive(Clone, Debug)]
    struct TextLiteralWithExtentHolder {
        extent: PlaceHolder<PlanarExtent>,
        text_literal: PlaceHolder<TextLiteral>,
    }
    impl Holder for TextLiteralWithExtentHolder {
        type Table = Tables;
        type Owned = TextLiteralWithExtent;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "TEXT_LITERAL_WITH_EXTENT"
        }
        fn attr_len() -> usize {
            2usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct TextStyle {
        pub name: Label,
        pub character_appearance: CharacterStyleSelect,
    }
    #[derive(Clone, Debug)]
    struct TextStyleHolder {
        name: Label,
        character_appearance: PlaceHolder<CharacterStyleSelect>,
    }
    impl Holder for TextStyleHolder {
        type Table = Tables;
        type Owned = TextStyle;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "TEXT_STYLE"
        }
        fn attr_len() -> usize {
            2usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct TextStyleForDefinedFont {
        pub text_colour: Colour,
    }
    #[derive(Clone, Debug)]
    struct TextStyleForDefinedFontHolder {
        text_colour: PlaceHolder<Colour>,
    }
    impl Holder for TextStyleForDefinedFontHolder {
        type Table = Tables;
        type Owned = TextStyleForDefinedFont;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "TEXT_STYLE_FOR_DEFINED_FONT"
        }
        fn attr_len() -> usize {
            1usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct TextStyleWithBoxCharacteristics {
        pub characteristics: Vec<BoxCharacteristicSelect>,
        pub text_style: TextStyle,
    }
    #[derive(Clone, Debug)]
    struct TextStyleWithBoxCharacteristicsHolder {
        characteristics: PlaceHolder<Vec<BoxCharacteristicSelect>>,
        text_style: PlaceHolder<TextStyle>,
    }
    impl Holder for TextStyleWithBoxCharacteristicsHolder {
        type Table = Tables;
        type Owned = TextStyleWithBoxCharacteristics;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "TEXT_STYLE_WITH_BOX_CHARACTERISTICS"
        }
        fn attr_len() -> usize {
            2usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct TextStyleWithMirror {
        pub mirror_placement: Axis2Placement,
        pub text_style: TextStyle,
    }
    #[derive(Clone, Debug)]
    struct TextStyleWithMirrorHolder {
        mirror_placement: PlaceHolder<Axis2Placement>,
        text_style: PlaceHolder<TextStyle>,
    }
    impl Holder for TextStyleWithMirrorHolder {
        type Table = Tables;
        type Owned = TextStyleWithMirror;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "TEXT_STYLE_WITH_MIRROR"
        }
        fn attr_len() -> usize {
            2usize
        }
    }
    impl BoundedCurveAny for TrimmedCurve {}
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct TrimmedCurve {
        pub basis_curve: Box<dyn CurveAny>,
        pub trim_1: Vec<TrimmingSelect>,
        pub trim_2: Vec<TrimmingSelect>,
        pub sense_agreement: bool,
        pub master_representation: TrimmingPreference,
        pub bounded_curve: Box<dyn BoundedCurveAny>,
    }
    #[derive(Clone, Debug)]
    struct TrimmedCurveHolder {
        basis_curve: PlaceHolder<Box<dyn CurveAny>>,
        trim_1: PlaceHolder<Vec<TrimmingSelect>>,
        trim_2: PlaceHolder<Vec<TrimmingSelect>>,
        sense_agreement: bool,
        master_representation: PlaceHolder<TrimmingPreference>,
        bounded_curve: PlaceHolder<Box<dyn BoundedCurveAny>>,
    }
    impl Holder for TrimmedCurveHolder {
        type Table = Tables;
        type Owned = TrimmedCurve;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "TRIMMED_CURVE"
        }
        fn attr_len() -> usize {
            6usize
        }
    }
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct TwoDirectionRepeatFactor {
        pub second_repeat_factor: Vector,
        pub one_direction_repeat_factor: OneDirectionRepeatFactor,
    }
    #[derive(Clone, Debug)]
    struct TwoDirectionRepeatFactorHolder {
        second_repeat_factor: PlaceHolder<Vector>,
        one_direction_repeat_factor: PlaceHolder<OneDirectionRepeatFactor>,
    }
    impl Holder for TwoDirectionRepeatFactorHolder {
        type Table = Tables;
        type Owned = TwoDirectionRepeatFactor;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "TWO_DIRECTION_REPEAT_FACTOR"
        }
        fn attr_len() -> usize {
            2usize
        }
    }
    impl BSplineCurveAny for UniformCurve {}
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct UniformCurve {
        pub b_spline_curve: Box<dyn BSplineCurveAny>,
    }
    #[derive(Clone, Debug)]
    struct UniformCurveHolder {
        b_spline_curve: PlaceHolder<Box<dyn BSplineCurveAny>>,
    }
    impl Holder for UniformCurveHolder {
        type Table = Tables;
        type Owned = UniformCurve;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "UNIFORM_CURVE"
        }
        fn attr_len() -> usize {
            1usize
        }
    }
    impl GeometricRepresentationItemAny for Vector {}
    #[derive(Debug, Clone, derive_new :: new)]
    pub struct Vector {
        pub orientation: Direction,
        pub magnitude: LengthMeasure,
        pub geometric_representation_item: Box<dyn GeometricRepresentationItemAny>,
    }
    #[derive(Clone, Debug)]
    struct VectorHolder {
        orientation: PlaceHolder<Direction>,
        magnitude: LengthMeasure,
        geometric_representation_item: PlaceHolder<Box<dyn GeometricRepresentationItemAny>>,
    }
    impl Holder for VectorHolder {
        type Table = Tables;
        type Owned = Vector;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
        fn name() -> &'static str {
            "VECTOR"
        }
        fn attr_len() -> usize {
            3usize
        }
    }
}

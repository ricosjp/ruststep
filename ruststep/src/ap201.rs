#![allow(dead_code)]
pub mod explicit_draughting {
    use crate::{error::Result, primitive::*, tables::*};
    use std::collections::HashMap;
    #[derive(Debug, Clone, PartialEq, Default)]
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
        pub fn axis_2_placement_2d_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<Axis2Placement2D>> + 'table {
            self.axis_2_placement_2d
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
    struct AddressHolder {
        internal_location: Option<PlaceHolder<Label>>,
        street_number: Option<PlaceHolder<Label>>,
        street: Option<PlaceHolder<Label>>,
        postal_box: Option<PlaceHolder<Label>>,
        town: Option<PlaceHolder<Label>>,
        region: Option<PlaceHolder<Label>>,
        postal_code: Option<PlaceHolder<Label>>,
        country: Option<PlaceHolder<Label>>,
        facsimile_number: Option<PlaceHolder<Label>>,
        telephone_number: Option<PlaceHolder<Label>>,
        electronic_mail_address: Option<PlaceHolder<Label>>,
        telex_number: Option<PlaceHolder<Label>>,
    }
    impl Holder for AddressHolder {
        type Table = Tables;
        type Owned = Address;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct AngularDimensionHolder {
        dimension_curve_directed_callout: PlaceHolder<DimensionCurveDirectedCallout>,
    }
    impl Holder for AngularDimensionHolder {
        type Table = Tables;
        type Owned = AngularDimension;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct AnnotationCurveOccurrenceHolder {
        annotation_occurrence: PlaceHolder<AnnotationOccurrence>,
    }
    impl Holder for AnnotationCurveOccurrenceHolder {
        type Table = Tables;
        type Owned = AnnotationCurveOccurrence;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct AnnotationFillAreaHolder {
        boundaries: PlaceHolder<Vec<Curve>>,
        geometric_representation_item: PlaceHolder<GeometricRepresentationItem>,
    }
    impl Holder for AnnotationFillAreaHolder {
        type Table = Tables;
        type Owned = AnnotationFillArea;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct AnnotationFillAreaOccurrenceHolder {
        fill_style_target: PlaceHolder<Point>,
        annotation_occurrence: PlaceHolder<AnnotationOccurrence>,
    }
    impl Holder for AnnotationFillAreaOccurrenceHolder {
        type Table = Tables;
        type Owned = AnnotationFillAreaOccurrence;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct AnnotationOccurrenceHolder {
        styled_item: PlaceHolder<StyledItem>,
    }
    impl Holder for AnnotationOccurrenceHolder {
        type Table = Tables;
        type Owned = AnnotationOccurrence;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct AnnotationSubfigureOccurrenceHolder {
        annotation_symbol_occurrence: PlaceHolder<AnnotationSymbolOccurrence>,
    }
    impl Holder for AnnotationSubfigureOccurrenceHolder {
        type Table = Tables;
        type Owned = AnnotationSubfigureOccurrence;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct AnnotationSymbolHolder {
        mapped_item: PlaceHolder<MappedItem>,
    }
    impl Holder for AnnotationSymbolHolder {
        type Table = Tables;
        type Owned = AnnotationSymbol;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct AnnotationSymbolOccurrenceHolder {
        annotation_occurrence: PlaceHolder<AnnotationOccurrence>,
    }
    impl Holder for AnnotationSymbolOccurrenceHolder {
        type Table = Tables;
        type Owned = AnnotationSymbolOccurrence;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct AnnotationTextHolder {
        mapped_item: PlaceHolder<MappedItem>,
    }
    impl Holder for AnnotationTextHolder {
        type Table = Tables;
        type Owned = AnnotationText;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct AnnotationTextOccurrenceHolder {
        annotation_occurrence: PlaceHolder<AnnotationOccurrence>,
    }
    impl Holder for AnnotationTextOccurrenceHolder {
        type Table = Tables;
        type Owned = AnnotationTextOccurrence;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ApplicationContext {
        pub application: Text,
    }
    #[derive(Clone, Debug, PartialEq)]
    struct ApplicationContextHolder {
        application: PlaceHolder<Text>,
    }
    impl Holder for ApplicationContextHolder {
        type Table = Tables;
        type Owned = ApplicationContext;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ApplicationContextElement {
        pub name: Label,
        pub frame_of_reference: ApplicationContext,
    }
    #[derive(Clone, Debug, PartialEq)]
    struct ApplicationContextElementHolder {
        name: PlaceHolder<Label>,
        frame_of_reference: PlaceHolder<ApplicationContext>,
    }
    impl Holder for ApplicationContextElementHolder {
        type Table = Tables;
        type Owned = ApplicationContextElement;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ApplicationProtocolDefinition {
        pub status: Label,
        pub application_interpreted_model_schema_name: Label,
        pub application_protocol_year: YearNumber,
        pub application: ApplicationContext,
    }
    #[derive(Clone, Debug, PartialEq)]
    struct ApplicationProtocolDefinitionHolder {
        status: PlaceHolder<Label>,
        application_interpreted_model_schema_name: PlaceHolder<Label>,
        application_protocol_year: PlaceHolder<YearNumber>,
        application: PlaceHolder<ApplicationContext>,
    }
    impl Holder for ApplicationProtocolDefinitionHolder {
        type Table = Tables;
        type Owned = ApplicationProtocolDefinition;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct Approval {
        pub status: ApprovalStatus,
        pub level: Label,
    }
    #[derive(Clone, Debug, PartialEq)]
    struct ApprovalHolder {
        status: PlaceHolder<ApprovalStatus>,
        level: PlaceHolder<Label>,
    }
    impl Holder for ApprovalHolder {
        type Table = Tables;
        type Owned = Approval;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ApprovalAssignment {
        pub assigned_approval: Approval,
    }
    #[derive(Clone, Debug, PartialEq)]
    struct ApprovalAssignmentHolder {
        assigned_approval: PlaceHolder<Approval>,
    }
    impl Holder for ApprovalAssignmentHolder {
        type Table = Tables;
        type Owned = ApprovalAssignment;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ApprovalDateTime {
        pub date_time: DateTimeSelect,
        pub dated_approval: Approval,
    }
    #[derive(Clone, Debug, PartialEq)]
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
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ApprovalPersonOrganization {
        pub person_organization: PersonOrganizationSelect,
        pub authorized_approval: Approval,
        pub role: ApprovalRole,
    }
    #[derive(Clone, Debug, PartialEq)]
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
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ApprovalRole {
        pub role: Label,
    }
    #[derive(Clone, Debug, PartialEq)]
    struct ApprovalRoleHolder {
        role: PlaceHolder<Label>,
    }
    impl Holder for ApprovalRoleHolder {
        type Table = Tables;
        type Owned = ApprovalRole;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ApprovalStatus {
        pub name: Label,
    }
    #[derive(Clone, Debug, PartialEq)]
    struct ApprovalStatusHolder {
        name: PlaceHolder<Label>,
    }
    impl Holder for ApprovalStatusHolder {
        type Table = Tables;
        type Owned = ApprovalStatus;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct AreaInSet {
        pub area: PresentationArea,
        pub in_set: PresentationSet,
    }
    #[derive(Clone, Debug, PartialEq)]
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
    struct Axis2Placement2DHolder {
        ref_direction: Option<PlaceHolder<Direction>>,
        placement: PlaceHolder<Placement>,
    }
    impl Holder for Axis2Placement2DHolder {
        type Table = Tables;
        type Owned = Axis2Placement2D;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct BSplineCurveHolder {
        degree: i64,
        control_points_list: PlaceHolder<Vec<CartesianPoint>>,
        curve_form: PlaceHolder<BSplineCurveForm>,
        closed_curve: Logical,
        self_intersect: Logical,
        bounded_curve: PlaceHolder<BoundedCurve>,
    }
    impl Holder for BSplineCurveHolder {
        type Table = Tables;
        type Owned = BSplineCurve;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct BSplineCurveWithKnotsHolder {
        knot_multiplicities: PlaceHolder<Vec<i64>>,
        knots: PlaceHolder<Vec<ParameterValue>>,
        knot_spec: PlaceHolder<KnotType>,
        b_spline_curve: PlaceHolder<BSplineCurve>,
    }
    impl Holder for BSplineCurveWithKnotsHolder {
        type Table = Tables;
        type Owned = BSplineCurveWithKnots;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct BezierCurveHolder {
        b_spline_curve: PlaceHolder<BSplineCurve>,
    }
    impl Holder for BezierCurveHolder {
        type Table = Tables;
        type Owned = BezierCurve;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct BoundedCurveHolder {
        curve: PlaceHolder<Curve>,
    }
    impl Holder for BoundedCurveHolder {
        type Table = Tables;
        type Owned = BoundedCurve;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct CalendarDateHolder {
        day_component: PlaceHolder<DayInMonthNumber>,
        month_component: PlaceHolder<MonthInYearNumber>,
        date: PlaceHolder<Date>,
    }
    impl Holder for CalendarDateHolder {
        type Table = Tables;
        type Owned = CalendarDate;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct CameraImageHolder {
        mapped_item: PlaceHolder<MappedItem>,
    }
    impl Holder for CameraImageHolder {
        type Table = Tables;
        type Owned = CameraImage;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct CameraImage2DWithScaleHolder {
        camera_image: PlaceHolder<CameraImage>,
    }
    impl Holder for CameraImage2DWithScaleHolder {
        type Table = Tables;
        type Owned = CameraImage2DWithScale;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct CameraModelHolder {
        geometric_representation_item: PlaceHolder<GeometricRepresentationItem>,
    }
    impl Holder for CameraModelHolder {
        type Table = Tables;
        type Owned = CameraModel;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct CameraModelD2Holder {
        view_window: PlaceHolder<PlanarBox>,
        view_window_clipping: bool,
        camera_model: PlaceHolder<CameraModel>,
    }
    impl Holder for CameraModelD2Holder {
        type Table = Tables;
        type Owned = CameraModelD2;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct CameraUsageHolder {
        representation_map: PlaceHolder<RepresentationMap>,
    }
    impl Holder for CameraUsageHolder {
        type Table = Tables;
        type Owned = CameraUsage;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct CartesianPointHolder {
        coordinates: PlaceHolder<Vec<LengthMeasure>>,
        point: PlaceHolder<Point>,
    }
    impl Holder for CartesianPointHolder {
        type Table = Tables;
        type Owned = CartesianPoint;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct CircleHolder {
        radius: PlaceHolder<PositiveLengthMeasure>,
        conic: PlaceHolder<Conic>,
    }
    impl Holder for CircleHolder {
        type Table = Tables;
        type Owned = Circle;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct Colour {}
    #[derive(Clone, Debug, PartialEq)]
    struct ColourHolder {}
    impl Holder for ColourHolder {
        type Table = Tables;
        type Owned = Colour;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
    }
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
    struct CompositeCurveHolder {
        segments: PlaceHolder<Vec<CompositeCurveSegment>>,
        self_intersect: Logical,
        bounded_curve: PlaceHolder<BoundedCurve>,
    }
    impl Holder for CompositeCurveHolder {
        type Table = Tables;
        type Owned = CompositeCurve;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct CompositeCurveSegment {
        pub transition: TransitionCode,
        pub same_sense: bool,
        pub parent_curve: Curve,
    }
    #[derive(Clone, Debug, PartialEq)]
    struct CompositeCurveSegmentHolder {
        transition: PlaceHolder<TransitionCode>,
        same_sense: bool,
        parent_curve: PlaceHolder<Curve>,
    }
    impl Holder for CompositeCurveSegmentHolder {
        type Table = Tables;
        type Owned = CompositeCurveSegment;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct CompositeTextHolder {
        collected_text: PlaceHolder<Vec<TextOrCharacter>>,
        geometric_representation_item: PlaceHolder<GeometricRepresentationItem>,
    }
    impl Holder for CompositeTextHolder {
        type Table = Tables;
        type Owned = CompositeText;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct CompositeTextWithAssociatedCurvesHolder {
        associated_curves: PlaceHolder<Vec<Curve>>,
        composite_text: PlaceHolder<CompositeText>,
    }
    impl Holder for CompositeTextWithAssociatedCurvesHolder {
        type Table = Tables;
        type Owned = CompositeTextWithAssociatedCurves;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct ConicHolder {
        position: PlaceHolder<Axis2Placement>,
        curve: PlaceHolder<Curve>,
    }
    impl Holder for ConicHolder {
        type Table = Tables;
        type Owned = Conic;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct Contract {
        pub name: Label,
        pub purpose: Text,
        pub kind: ContractType,
    }
    #[derive(Clone, Debug, PartialEq)]
    struct ContractHolder {
        name: PlaceHolder<Label>,
        purpose: PlaceHolder<Text>,
        kind: PlaceHolder<ContractType>,
    }
    impl Holder for ContractHolder {
        type Table = Tables;
        type Owned = Contract;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ContractAssignment {
        pub assigned_contract: Contract,
    }
    #[derive(Clone, Debug, PartialEq)]
    struct ContractAssignmentHolder {
        assigned_contract: PlaceHolder<Contract>,
    }
    impl Holder for ContractAssignmentHolder {
        type Table = Tables;
        type Owned = ContractAssignment;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ContractType {
        pub description: Label,
    }
    #[derive(Clone, Debug, PartialEq)]
    struct ContractTypeHolder {
        description: PlaceHolder<Label>,
    }
    impl Holder for ContractTypeHolder {
        type Table = Tables;
        type Owned = ContractType;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct ConversionBasedUnitHolder {
        name: PlaceHolder<Label>,
        conversion_factor: PlaceHolder<MeasureWithUnit>,
        named_unit: PlaceHolder<NamedUnit>,
    }
    impl Holder for ConversionBasedUnitHolder {
        type Table = Tables;
        type Owned = ConversionBasedUnit;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct CurveHolder {
        geometric_representation_item: PlaceHolder<GeometricRepresentationItem>,
    }
    impl Holder for CurveHolder {
        type Table = Tables;
        type Owned = Curve;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct CurveDimensionHolder {
        dimension_curve_directed_callout: PlaceHolder<DimensionCurveDirectedCallout>,
    }
    impl Holder for CurveDimensionHolder {
        type Table = Tables;
        type Owned = CurveDimension;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct CurveStyle {
        pub name: Label,
        pub curve_font: CurveFontOrScaledCurveFontSelect,
        pub curve_width: SizeSelect,
        pub curve_colour: Colour,
    }
    #[derive(Clone, Debug, PartialEq)]
    struct CurveStyleHolder {
        name: PlaceHolder<Label>,
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
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct CurveStyleFont {
        pub name: Label,
        pub pattern_list: Vec<CurveStyleFontPattern>,
    }
    #[derive(Clone, Debug, PartialEq)]
    struct CurveStyleFontHolder {
        name: PlaceHolder<Label>,
        pattern_list: PlaceHolder<Vec<CurveStyleFontPattern>>,
    }
    impl Holder for CurveStyleFontHolder {
        type Table = Tables;
        type Owned = CurveStyleFont;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct CurveStyleFontPattern {
        pub visible_segment_length: PositiveLengthMeasure,
        pub invisible_segment_length: PositiveLengthMeasure,
    }
    #[derive(Clone, Debug, PartialEq)]
    struct CurveStyleFontPatternHolder {
        visible_segment_length: PlaceHolder<PositiveLengthMeasure>,
        invisible_segment_length: PlaceHolder<PositiveLengthMeasure>,
    }
    impl Holder for CurveStyleFontPatternHolder {
        type Table = Tables;
        type Owned = CurveStyleFontPattern;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct Date {
        pub year_component: YearNumber,
    }
    #[derive(Clone, Debug, PartialEq)]
    struct DateHolder {
        year_component: PlaceHolder<YearNumber>,
    }
    impl Holder for DateHolder {
        type Table = Tables;
        type Owned = Date;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct DatumFeatureCalloutHolder {
        draughting_callout: PlaceHolder<DraughtingCallout>,
    }
    impl Holder for DatumFeatureCalloutHolder {
        type Table = Tables;
        type Owned = DatumFeatureCallout;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct DatumTargetCalloutHolder {
        draughting_callout: PlaceHolder<DraughtingCallout>,
    }
    impl Holder for DatumTargetCalloutHolder {
        type Table = Tables;
        type Owned = DatumTargetCallout;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct DefinedSymbolHolder {
        definition: PlaceHolder<DefinedSymbolSelect>,
        target: PlaceHolder<SymbolTarget>,
        geometric_representation_item: PlaceHolder<GeometricRepresentationItem>,
    }
    impl Holder for DefinedSymbolHolder {
        type Table = Tables;
        type Owned = DefinedSymbol;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct DiameterDimensionHolder {
        dimension_curve_directed_callout: PlaceHolder<DimensionCurveDirectedCallout>,
    }
    impl Holder for DiameterDimensionHolder {
        type Table = Tables;
        type Owned = DiameterDimension;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct DimensionCalloutComponentRelationshipHolder {
        draughting_callout_relationship: PlaceHolder<DraughtingCalloutRelationship>,
    }
    impl Holder for DimensionCalloutComponentRelationshipHolder {
        type Table = Tables;
        type Owned = DimensionCalloutComponentRelationship;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct DimensionCalloutRelationshipHolder {
        draughting_callout_relationship: PlaceHolder<DraughtingCalloutRelationship>,
    }
    impl Holder for DimensionCalloutRelationshipHolder {
        type Table = Tables;
        type Owned = DimensionCalloutRelationship;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct DimensionCurveHolder {
        annotation_curve_occurrence: PlaceHolder<AnnotationCurveOccurrence>,
    }
    impl Holder for DimensionCurveHolder {
        type Table = Tables;
        type Owned = DimensionCurve;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct DimensionCurveDirectedCalloutHolder {
        draughting_callout: PlaceHolder<DraughtingCallout>,
    }
    impl Holder for DimensionCurveDirectedCalloutHolder {
        type Table = Tables;
        type Owned = DimensionCurveDirectedCallout;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct DimensionPairHolder {
        draughting_callout_relationship: PlaceHolder<DraughtingCalloutRelationship>,
    }
    impl Holder for DimensionPairHolder {
        type Table = Tables;
        type Owned = DimensionPair;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
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
    #[derive(Clone, Debug, PartialEq)]
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
    struct DirectionHolder {
        direction_ratios: PlaceHolder<Vec<f64>>,
        geometric_representation_item: PlaceHolder<GeometricRepresentationItem>,
    }
    impl Holder for DirectionHolder {
        type Table = Tables;
        type Owned = Direction;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct Document {
        pub id: Identifier,
        pub name: Label,
        pub description: Text,
        pub kind: DocumentType,
    }
    #[derive(Clone, Debug, PartialEq)]
    struct DocumentHolder {
        id: PlaceHolder<Identifier>,
        name: PlaceHolder<Label>,
        description: PlaceHolder<Text>,
        kind: PlaceHolder<DocumentType>,
    }
    impl Holder for DocumentHolder {
        type Table = Tables;
        type Owned = Document;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DocumentReference {
        pub assigned_document: Document,
        pub source: Label,
    }
    #[derive(Clone, Debug, PartialEq)]
    struct DocumentReferenceHolder {
        assigned_document: PlaceHolder<Document>,
        source: PlaceHolder<Label>,
    }
    impl Holder for DocumentReferenceHolder {
        type Table = Tables;
        type Owned = DocumentReference;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DocumentType {
        pub product_data_type: Label,
    }
    #[derive(Clone, Debug, PartialEq)]
    struct DocumentTypeHolder {
        product_data_type: PlaceHolder<Label>,
    }
    impl Holder for DocumentTypeHolder {
        type Table = Tables;
        type Owned = DocumentType;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct DraughtingAnnotationOccurrenceHolder {
        annotation_occurrence: PlaceHolder<AnnotationOccurrence>,
    }
    impl Holder for DraughtingAnnotationOccurrenceHolder {
        type Table = Tables;
        type Owned = DraughtingAnnotationOccurrence;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct DraughtingApprovalAssignmentHolder {
        approved_items: PlaceHolder<Vec<ApprovedItem>>,
        approval_assignment: PlaceHolder<ApprovalAssignment>,
    }
    impl Holder for DraughtingApprovalAssignmentHolder {
        type Table = Tables;
        type Owned = DraughtingApprovalAssignment;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct DraughtingCalloutHolder {
        contents: PlaceHolder<Vec<DraughtingCalloutElement>>,
        geometric_representation_item: PlaceHolder<GeometricRepresentationItem>,
    }
    impl Holder for DraughtingCalloutHolder {
        type Table = Tables;
        type Owned = DraughtingCallout;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DraughtingCalloutRelationship {
        pub name: Label,
        pub description: Text,
        pub relating_draughting_callout: DraughtingCallout,
        pub related_draughting_callout: DraughtingCallout,
    }
    #[derive(Clone, Debug, PartialEq)]
    struct DraughtingCalloutRelationshipHolder {
        name: PlaceHolder<Label>,
        description: PlaceHolder<Text>,
        relating_draughting_callout: PlaceHolder<DraughtingCallout>,
        related_draughting_callout: PlaceHolder<DraughtingCallout>,
    }
    impl Holder for DraughtingCalloutRelationshipHolder {
        type Table = Tables;
        type Owned = DraughtingCalloutRelationship;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct DraughtingContractAssignmentHolder {
        items: PlaceHolder<Vec<ContractedItem>>,
        contract_assignment: PlaceHolder<ContractAssignment>,
    }
    impl Holder for DraughtingContractAssignmentHolder {
        type Table = Tables;
        type Owned = DraughtingContractAssignment;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct DraughtingDrawingRevisionHolder {
        drawing_revision: PlaceHolder<DrawingRevision>,
    }
    impl Holder for DraughtingDrawingRevisionHolder {
        type Table = Tables;
        type Owned = DraughtingDrawingRevision;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct DraughtingElementsHolder {
        draughting_callout: PlaceHolder<DraughtingCallout>,
    }
    impl Holder for DraughtingElementsHolder {
        type Table = Tables;
        type Owned = DraughtingElements;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct DraughtingGroupAssignmentHolder {
        items: PlaceHolder<Vec<DraughtingGroupedItem>>,
        group_assignment: PlaceHolder<GroupAssignment>,
    }
    impl Holder for DraughtingGroupAssignmentHolder {
        type Table = Tables;
        type Owned = DraughtingGroupAssignment;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct DraughtingModelHolder {
        representation: PlaceHolder<Representation>,
    }
    impl Holder for DraughtingModelHolder {
        type Table = Tables;
        type Owned = DraughtingModel;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct DraughtingOrganizationAssignmentHolder {
        assigned_items: PlaceHolder<Vec<DraughtingOrganizationItem>>,
        organization_assignment: PlaceHolder<OrganizationAssignment>,
    }
    impl Holder for DraughtingOrganizationAssignmentHolder {
        type Table = Tables;
        type Owned = DraughtingOrganizationAssignment;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct DraughtingPersonAndOrganizationAssignmentHolder {
        assigned_items: PlaceHolder<Vec<DraughtingOrganizationItem>>,
        person_and_organization_assignment: PlaceHolder<PersonAndOrganizationAssignment>,
    }
    impl Holder for DraughtingPersonAndOrganizationAssignmentHolder {
        type Table = Tables;
        type Owned = DraughtingPersonAndOrganizationAssignment;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct DraughtingPersonAssignmentHolder {
        assigned_items: PlaceHolder<Vec<DraughtingOrganizationItem>>,
        person_assignment: PlaceHolder<PersonAssignment>,
    }
    impl Holder for DraughtingPersonAssignmentHolder {
        type Table = Tables;
        type Owned = DraughtingPersonAssignment;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct DraughtingPreDefinedColourHolder {
        pre_defined_colour: PlaceHolder<PreDefinedColour>,
    }
    impl Holder for DraughtingPreDefinedColourHolder {
        type Table = Tables;
        type Owned = DraughtingPreDefinedColour;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct DraughtingPreDefinedCurveFontHolder {
        pre_defined_curve_font: PlaceHolder<PreDefinedCurveFont>,
    }
    impl Holder for DraughtingPreDefinedCurveFontHolder {
        type Table = Tables;
        type Owned = DraughtingPreDefinedCurveFont;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct DraughtingPreDefinedTextFontHolder {
        pre_defined_text_font: PlaceHolder<PreDefinedTextFont>,
    }
    impl Holder for DraughtingPreDefinedTextFontHolder {
        type Table = Tables;
        type Owned = DraughtingPreDefinedTextFont;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct DraughtingPresentedItemHolder {
        items: PlaceHolder<Vec<DraughtingPresentedItemSelect>>,
        presented_item: PlaceHolder<PresentedItem>,
    }
    impl Holder for DraughtingPresentedItemHolder {
        type Table = Tables;
        type Owned = DraughtingPresentedItem;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct DraughtingSecurityClassificationAssignmentHolder {
        assigned_items: PlaceHolder<Vec<ClassifiedItem>>,
        security_classification_assignment: PlaceHolder<SecurityClassificationAssignment>,
    }
    impl Holder for DraughtingSecurityClassificationAssignmentHolder {
        type Table = Tables;
        type Owned = DraughtingSecurityClassificationAssignment;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct DraughtingSpecificationReferenceHolder {
        specified_items: PlaceHolder<Vec<SpecifiedItem>>,
        document_reference: PlaceHolder<DocumentReference>,
    }
    impl Holder for DraughtingSpecificationReferenceHolder {
        type Table = Tables;
        type Owned = DraughtingSpecificationReference;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct DraughtingSubfigureRepresentationHolder {
        symbol_representation: PlaceHolder<SymbolRepresentation>,
    }
    impl Holder for DraughtingSubfigureRepresentationHolder {
        type Table = Tables;
        type Owned = DraughtingSubfigureRepresentation;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct DraughtingSymbolRepresentationHolder {
        symbol_representation: PlaceHolder<SymbolRepresentation>,
    }
    impl Holder for DraughtingSymbolRepresentationHolder {
        type Table = Tables;
        type Owned = DraughtingSymbolRepresentation;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct DraughtingTextLiteralWithDelineationHolder {
        text_literal_with_delineation: PlaceHolder<TextLiteralWithDelineation>,
    }
    impl Holder for DraughtingTextLiteralWithDelineationHolder {
        type Table = Tables;
        type Owned = DraughtingTextLiteralWithDelineation;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DraughtingTitle {
        pub items: Vec<DraughtingTitledItem>,
        pub language: Label,
        pub contents: Text,
    }
    #[derive(Clone, Debug, PartialEq)]
    struct DraughtingTitleHolder {
        items: PlaceHolder<Vec<DraughtingTitledItem>>,
        language: PlaceHolder<Label>,
        contents: PlaceHolder<Text>,
    }
    impl Holder for DraughtingTitleHolder {
        type Table = Tables;
        type Owned = DraughtingTitle;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct DrawingDefinition {
        pub drawing_number: Identifier,
        pub drawing_type: Option<Label>,
    }
    #[derive(Clone, Debug, PartialEq)]
    struct DrawingDefinitionHolder {
        drawing_number: PlaceHolder<Identifier>,
        drawing_type: Option<PlaceHolder<Label>>,
    }
    impl Holder for DrawingDefinitionHolder {
        type Table = Tables;
        type Owned = DrawingDefinition;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct DrawingRevisionHolder {
        revision_identifier: PlaceHolder<Identifier>,
        drawing_identifier: PlaceHolder<DrawingDefinition>,
        intended_scale: Option<PlaceHolder<Text>>,
        presentation_set: PlaceHolder<PresentationSet>,
    }
    impl Holder for DrawingRevisionHolder {
        type Table = Tables;
        type Owned = DrawingRevision;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct DrawingSheetLayoutHolder {
        draughting_symbol_representation: PlaceHolder<DraughtingSymbolRepresentation>,
    }
    impl Holder for DrawingSheetLayoutHolder {
        type Table = Tables;
        type Owned = DrawingSheetLayout;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct DrawingSheetRevisionHolder {
        revision_identifier: PlaceHolder<Identifier>,
        presentation_area: PlaceHolder<PresentationArea>,
    }
    impl Holder for DrawingSheetRevisionHolder {
        type Table = Tables;
        type Owned = DrawingSheetRevision;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct DrawingSheetRevisionUsageHolder {
        sheet_number: PlaceHolder<Identifier>,
        area_in_set: PlaceHolder<AreaInSet>,
    }
    impl Holder for DrawingSheetRevisionUsageHolder {
        type Table = Tables;
        type Owned = DrawingSheetRevisionUsage;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct EllipseHolder {
        semi_axis_1: PlaceHolder<PositiveLengthMeasure>,
        semi_axis_2: PlaceHolder<PositiveLengthMeasure>,
        conic: PlaceHolder<Conic>,
    }
    impl Holder for EllipseHolder {
        type Table = Tables;
        type Owned = Ellipse;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ExternalSource {
        pub source_id: SourceItem,
    }
    #[derive(Clone, Debug, PartialEq)]
    struct ExternalSourceHolder {
        source_id: PlaceHolder<SourceItem>,
    }
    impl Holder for ExternalSourceHolder {
        type Table = Tables;
        type Owned = ExternalSource;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct ExternallyDefinedCurveFontHolder {
        externally_defined_item: PlaceHolder<ExternallyDefinedItem>,
    }
    impl Holder for ExternallyDefinedCurveFontHolder {
        type Table = Tables;
        type Owned = ExternallyDefinedCurveFont;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ExternallyDefinedHatchStyle {
        pub externally_defined_item: ExternallyDefinedItem,
        pub geometric_representation_item: GeometricRepresentationItem,
    }
    #[derive(Clone, Debug, PartialEq)]
    struct ExternallyDefinedHatchStyleHolder {
        externally_defined_item: PlaceHolder<ExternallyDefinedItem>,
        geometric_representation_item: PlaceHolder<GeometricRepresentationItem>,
    }
    impl Holder for ExternallyDefinedHatchStyleHolder {
        type Table = Tables;
        type Owned = ExternallyDefinedHatchStyle;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ExternallyDefinedItem {
        pub item_id: SourceItem,
        pub source: ExternalSource,
    }
    #[derive(Clone, Debug, PartialEq)]
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
    struct ExternallyDefinedSymbolHolder {
        externally_defined_item: PlaceHolder<ExternallyDefinedItem>,
    }
    impl Holder for ExternallyDefinedSymbolHolder {
        type Table = Tables;
        type Owned = ExternallyDefinedSymbol;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct ExternallyDefinedTextFontHolder {
        externally_defined_item: PlaceHolder<ExternallyDefinedItem>,
    }
    impl Holder for ExternallyDefinedTextFontHolder {
        type Table = Tables;
        type Owned = ExternallyDefinedTextFont;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ExternallyDefinedTileStyle {
        pub externally_defined_item: ExternallyDefinedItem,
        pub geometric_representation_item: GeometricRepresentationItem,
    }
    #[derive(Clone, Debug, PartialEq)]
    struct ExternallyDefinedTileStyleHolder {
        externally_defined_item: PlaceHolder<ExternallyDefinedItem>,
        geometric_representation_item: PlaceHolder<GeometricRepresentationItem>,
    }
    impl Holder for ExternallyDefinedTileStyleHolder {
        type Table = Tables;
        type Owned = ExternallyDefinedTileStyle;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct FillAreaStyle {
        pub name: Label,
        pub fill_styles: Vec<FillStyleSelect>,
    }
    #[derive(Clone, Debug, PartialEq)]
    struct FillAreaStyleHolder {
        name: PlaceHolder<Label>,
        fill_styles: PlaceHolder<Vec<FillStyleSelect>>,
    }
    impl Holder for FillAreaStyleHolder {
        type Table = Tables;
        type Owned = FillAreaStyle;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct FillAreaStyleColour {
        pub name: Label,
        pub fill_colour: Colour,
    }
    #[derive(Clone, Debug, PartialEq)]
    struct FillAreaStyleColourHolder {
        name: PlaceHolder<Label>,
        fill_colour: PlaceHolder<Colour>,
    }
    impl Holder for FillAreaStyleColourHolder {
        type Table = Tables;
        type Owned = FillAreaStyleColour;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct FillAreaStyleHatchingHolder {
        hatch_line_appearance: PlaceHolder<CurveStyle>,
        start_of_next_hatch_line: PlaceHolder<OneDirectionRepeatFactor>,
        point_of_reference_hatch_line: PlaceHolder<CartesianPoint>,
        pattern_start: PlaceHolder<CartesianPoint>,
        hatch_line_angle: PlaceHolder<PlaneAngleMeasure>,
        geometric_representation_item: PlaceHolder<GeometricRepresentationItem>,
    }
    impl Holder for FillAreaStyleHatchingHolder {
        type Table = Tables;
        type Owned = FillAreaStyleHatching;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct FillAreaStyleTileSymbolWithStyleHolder {
        symbol: PlaceHolder<AnnotationSymbolOccurrence>,
        geometric_representation_item: PlaceHolder<GeometricRepresentationItem>,
    }
    impl Holder for FillAreaStyleTileSymbolWithStyleHolder {
        type Table = Tables;
        type Owned = FillAreaStyleTileSymbolWithStyle;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct FillAreaStyleTilesHolder {
        tiling_pattern: PlaceHolder<TwoDirectionRepeatFactor>,
        tiles: PlaceHolder<Vec<FillAreaStyleTileShapeSelect>>,
        tiling_scale: PlaceHolder<PositiveRatioMeasure>,
        geometric_representation_item: PlaceHolder<GeometricRepresentationItem>,
    }
    impl Holder for FillAreaStyleTilesHolder {
        type Table = Tables;
        type Owned = FillAreaStyleTiles;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct GeometricCurveSetHolder {
        geometric_set: PlaceHolder<GeometricSet>,
    }
    impl Holder for GeometricCurveSetHolder {
        type Table = Tables;
        type Owned = GeometricCurveSet;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct GeometricRepresentationContextHolder {
        coordinate_space_dimension: PlaceHolder<DimensionCount>,
        representation_context: PlaceHolder<RepresentationContext>,
    }
    impl Holder for GeometricRepresentationContextHolder {
        type Table = Tables;
        type Owned = GeometricRepresentationContext;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct GeometricRepresentationItemHolder {
        representation_item: PlaceHolder<RepresentationItem>,
    }
    impl Holder for GeometricRepresentationItemHolder {
        type Table = Tables;
        type Owned = GeometricRepresentationItem;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct GeometricSetHolder {
        elements: PlaceHolder<Vec<GeometricSetSelect>>,
        geometric_representation_item: PlaceHolder<GeometricRepresentationItem>,
    }
    impl Holder for GeometricSetHolder {
        type Table = Tables;
        type Owned = GeometricSet;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct GeometricalToleranceCalloutHolder {
        draughting_callout: PlaceHolder<DraughtingCallout>,
    }
    impl Holder for GeometricalToleranceCalloutHolder {
        type Table = Tables;
        type Owned = GeometricalToleranceCallout;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct GeometricallyBounded2DWireframeRepresentationHolder {
        shape_representation: PlaceHolder<ShapeRepresentation>,
    }
    impl Holder for GeometricallyBounded2DWireframeRepresentationHolder {
        type Table = Tables;
        type Owned = GeometricallyBounded2DWireframeRepresentation;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct Group {
        pub name: Label,
        pub description: Text,
    }
    #[derive(Clone, Debug, PartialEq)]
    struct GroupHolder {
        name: PlaceHolder<Label>,
        description: PlaceHolder<Text>,
    }
    impl Holder for GroupHolder {
        type Table = Tables;
        type Owned = Group;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct GroupAssignment {
        pub assigned_group: Group,
    }
    #[derive(Clone, Debug, PartialEq)]
    struct GroupAssignmentHolder {
        assigned_group: PlaceHolder<Group>,
    }
    impl Holder for GroupAssignmentHolder {
        type Table = Tables;
        type Owned = GroupAssignment;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct GroupRelationship {
        pub name: Label,
        pub description: Text,
        pub relating_group: Group,
        pub related_group: Group,
    }
    #[derive(Clone, Debug, PartialEq)]
    struct GroupRelationshipHolder {
        name: PlaceHolder<Label>,
        description: PlaceHolder<Text>,
        relating_group: PlaceHolder<Group>,
        related_group: PlaceHolder<Group>,
    }
    impl Holder for GroupRelationshipHolder {
        type Table = Tables;
        type Owned = GroupRelationship;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct HyperbolaHolder {
        semi_axis: PlaceHolder<PositiveLengthMeasure>,
        semi_imag_axis: PlaceHolder<PositiveLengthMeasure>,
        conic: PlaceHolder<Conic>,
    }
    impl Holder for HyperbolaHolder {
        type Table = Tables;
        type Owned = Hyperbola;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct Invisibility {
        pub invisible_items: Vec<InvisibleItem>,
    }
    #[derive(Clone, Debug, PartialEq)]
    struct InvisibilityHolder {
        invisible_items: PlaceHolder<Vec<InvisibleItem>>,
    }
    impl Holder for InvisibilityHolder {
        type Table = Tables;
        type Owned = Invisibility;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct LeaderCurveHolder {
        annotation_curve_occurrence: PlaceHolder<AnnotationCurveOccurrence>,
    }
    impl Holder for LeaderCurveHolder {
        type Table = Tables;
        type Owned = LeaderCurve;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct LeaderDirectedCalloutHolder {
        draughting_callout: PlaceHolder<DraughtingCallout>,
    }
    impl Holder for LeaderDirectedCalloutHolder {
        type Table = Tables;
        type Owned = LeaderDirectedCallout;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct LeaderDirectedDimensionHolder {
        leader_directed_callout: PlaceHolder<LeaderDirectedCallout>,
    }
    impl Holder for LeaderDirectedDimensionHolder {
        type Table = Tables;
        type Owned = LeaderDirectedDimension;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct LeaderTerminatorHolder {
        terminator_symbol: PlaceHolder<TerminatorSymbol>,
    }
    impl Holder for LeaderTerminatorHolder {
        type Table = Tables;
        type Owned = LeaderTerminator;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct LengthMeasureWithUnitHolder {
        measure_with_unit: PlaceHolder<MeasureWithUnit>,
    }
    impl Holder for LengthMeasureWithUnitHolder {
        type Table = Tables;
        type Owned = LengthMeasureWithUnit;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct LengthUnitHolder {
        named_unit: PlaceHolder<NamedUnit>,
    }
    impl Holder for LengthUnitHolder {
        type Table = Tables;
        type Owned = LengthUnit;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct LineHolder {
        pnt: PlaceHolder<CartesianPoint>,
        dir: PlaceHolder<Vector>,
        curve: PlaceHolder<Curve>,
    }
    impl Holder for LineHolder {
        type Table = Tables;
        type Owned = Line;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct LinearDimensionHolder {
        dimension_curve_directed_callout: PlaceHolder<DimensionCurveDirectedCallout>,
    }
    impl Holder for LinearDimensionHolder {
        type Table = Tables;
        type Owned = LinearDimension;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct MeasureWithUnit {
        pub value_component: MeasureValue,
        pub unit_component: Unit,
    }
    #[derive(Clone, Debug, PartialEq)]
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
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct NamedUnit {
        pub dimensions: DimensionalExponents,
    }
    #[derive(Clone, Debug, PartialEq)]
    struct NamedUnitHolder {
        dimensions: PlaceHolder<DimensionalExponents>,
    }
    impl Holder for NamedUnitHolder {
        type Table = Tables;
        type Owned = NamedUnit;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct OffsetCurve2DHolder {
        basis_curve: PlaceHolder<Curve>,
        distance: PlaceHolder<LengthMeasure>,
        self_intersect: Logical,
        curve: PlaceHolder<Curve>,
    }
    impl Holder for OffsetCurve2DHolder {
        type Table = Tables;
        type Owned = OffsetCurve2D;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct OneDirectionRepeatFactorHolder {
        repeat_factor: PlaceHolder<Vector>,
        geometric_representation_item: PlaceHolder<GeometricRepresentationItem>,
    }
    impl Holder for OneDirectionRepeatFactorHolder {
        type Table = Tables;
        type Owned = OneDirectionRepeatFactor;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct OrdinateDimensionHolder {
        projection_directed_callout: PlaceHolder<ProjectionDirectedCallout>,
    }
    impl Holder for OrdinateDimensionHolder {
        type Table = Tables;
        type Owned = OrdinateDimension;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct Organization {
        pub id: Option<Identifier>,
        pub name: Label,
        pub description: Text,
    }
    #[derive(Clone, Debug, PartialEq)]
    struct OrganizationHolder {
        id: Option<PlaceHolder<Identifier>>,
        name: PlaceHolder<Label>,
        description: PlaceHolder<Text>,
    }
    impl Holder for OrganizationHolder {
        type Table = Tables;
        type Owned = Organization;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct OrganizationAssignment {
        pub assigned_organization: Organization,
        pub role: OrganizationRole,
    }
    #[derive(Clone, Debug, PartialEq)]
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
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct OrganizationRole {
        pub name: Label,
    }
    #[derive(Clone, Debug, PartialEq)]
    struct OrganizationRoleHolder {
        name: PlaceHolder<Label>,
    }
    impl Holder for OrganizationRoleHolder {
        type Table = Tables;
        type Owned = OrganizationRole;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct OrganizationalAddressHolder {
        organizations: PlaceHolder<Vec<Organization>>,
        description: PlaceHolder<Text>,
        address: PlaceHolder<Address>,
    }
    impl Holder for OrganizationalAddressHolder {
        type Table = Tables;
        type Owned = OrganizationalAddress;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct ParabolaHolder {
        focal_dist: PlaceHolder<LengthMeasure>,
        conic: PlaceHolder<Conic>,
    }
    impl Holder for ParabolaHolder {
        type Table = Tables;
        type Owned = Parabola;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct PersonHolder {
        id: PlaceHolder<Identifier>,
        last_name: Option<PlaceHolder<Label>>,
        first_name: Option<PlaceHolder<Label>>,
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
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PersonAndOrganization {
        pub the_person: Person,
        pub the_organization: Organization,
    }
    #[derive(Clone, Debug, PartialEq)]
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
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PersonAndOrganizationAssignment {
        pub assigned_person_and_organization: PersonAndOrganization,
        pub role: PersonAndOrganizationRole,
    }
    #[derive(Clone, Debug, PartialEq)]
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
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PersonAndOrganizationRole {
        pub name: Label,
    }
    #[derive(Clone, Debug, PartialEq)]
    struct PersonAndOrganizationRoleHolder {
        name: PlaceHolder<Label>,
    }
    impl Holder for PersonAndOrganizationRoleHolder {
        type Table = Tables;
        type Owned = PersonAndOrganizationRole;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PersonAssignment {
        pub assigned_person: Person,
        pub role: PersonRole,
    }
    #[derive(Clone, Debug, PartialEq)]
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
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PersonRole {
        pub name: Label,
    }
    #[derive(Clone, Debug, PartialEq)]
    struct PersonRoleHolder {
        name: PlaceHolder<Label>,
    }
    impl Holder for PersonRoleHolder {
        type Table = Tables;
        type Owned = PersonRole;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct PersonalAddressHolder {
        people: PlaceHolder<Vec<Person>>,
        description: PlaceHolder<Text>,
        address: PlaceHolder<Address>,
    }
    impl Holder for PersonalAddressHolder {
        type Table = Tables;
        type Owned = PersonalAddress;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct PlacementHolder {
        location: PlaceHolder<CartesianPoint>,
        geometric_representation_item: PlaceHolder<GeometricRepresentationItem>,
    }
    impl Holder for PlacementHolder {
        type Table = Tables;
        type Owned = Placement;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct PlanarExtentHolder {
        size_in_x: PlaceHolder<LengthMeasure>,
        size_in_y: PlaceHolder<LengthMeasure>,
        geometric_representation_item: PlaceHolder<GeometricRepresentationItem>,
    }
    impl Holder for PlanarExtentHolder {
        type Table = Tables;
        type Owned = PlanarExtent;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct PlaneAngleMeasureWithUnitHolder {
        measure_with_unit: PlaceHolder<MeasureWithUnit>,
    }
    impl Holder for PlaneAngleMeasureWithUnitHolder {
        type Table = Tables;
        type Owned = PlaneAngleMeasureWithUnit;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct PlaneAngleUnitHolder {
        named_unit: PlaceHolder<NamedUnit>,
    }
    impl Holder for PlaneAngleUnitHolder {
        type Table = Tables;
        type Owned = PlaneAngleUnit;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct PointHolder {
        geometric_representation_item: PlaceHolder<GeometricRepresentationItem>,
    }
    impl Holder for PointHolder {
        type Table = Tables;
        type Owned = Point;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct PointOnCurveHolder {
        basis_curve: PlaceHolder<Curve>,
        point_parameter: PlaceHolder<ParameterValue>,
        point: PlaceHolder<Point>,
    }
    impl Holder for PointOnCurveHolder {
        type Table = Tables;
        type Owned = PointOnCurve;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct PolylineHolder {
        points: PlaceHolder<Vec<CartesianPoint>>,
        bounded_curve: PlaceHolder<BoundedCurve>,
    }
    impl Holder for PolylineHolder {
        type Table = Tables;
        type Owned = Polyline;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PreDefinedColour {
        pub pre_defined_item: PreDefinedItem,
        pub colour: Colour,
    }
    #[derive(Clone, Debug, PartialEq)]
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
    struct PreDefinedCurveFontHolder {
        pre_defined_item: PlaceHolder<PreDefinedItem>,
    }
    impl Holder for PreDefinedCurveFontHolder {
        type Table = Tables;
        type Owned = PreDefinedCurveFont;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct PreDefinedDimensionSymbolHolder {
        pre_defined_symbol: PlaceHolder<PreDefinedSymbol>,
    }
    impl Holder for PreDefinedDimensionSymbolHolder {
        type Table = Tables;
        type Owned = PreDefinedDimensionSymbol;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct PreDefinedGeometricalToleranceSymbolHolder {
        pre_defined_symbol: PlaceHolder<PreDefinedSymbol>,
    }
    impl Holder for PreDefinedGeometricalToleranceSymbolHolder {
        type Table = Tables;
        type Owned = PreDefinedGeometricalToleranceSymbol;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PreDefinedItem {
        pub name: Label,
    }
    #[derive(Clone, Debug, PartialEq)]
    struct PreDefinedItemHolder {
        name: PlaceHolder<Label>,
    }
    impl Holder for PreDefinedItemHolder {
        type Table = Tables;
        type Owned = PreDefinedItem;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct PreDefinedPointMarkerSymbolHolder {
        pre_defined_symbol: PlaceHolder<PreDefinedSymbol>,
    }
    impl Holder for PreDefinedPointMarkerSymbolHolder {
        type Table = Tables;
        type Owned = PreDefinedPointMarkerSymbol;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct PreDefinedSymbolHolder {
        pre_defined_item: PlaceHolder<PreDefinedItem>,
    }
    impl Holder for PreDefinedSymbolHolder {
        type Table = Tables;
        type Owned = PreDefinedSymbol;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct PreDefinedTerminatorSymbolHolder {
        pre_defined_symbol: PlaceHolder<PreDefinedSymbol>,
    }
    impl Holder for PreDefinedTerminatorSymbolHolder {
        type Table = Tables;
        type Owned = PreDefinedTerminatorSymbol;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct PreDefinedTextFontHolder {
        pre_defined_item: PlaceHolder<PreDefinedItem>,
    }
    impl Holder for PreDefinedTextFontHolder {
        type Table = Tables;
        type Owned = PreDefinedTextFont;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct PresentationAreaHolder {
        presentation_representation: PlaceHolder<PresentationRepresentation>,
    }
    impl Holder for PresentationAreaHolder {
        type Table = Tables;
        type Owned = PresentationArea;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PresentationLayerAssignment {
        pub name: Label,
        pub description: Text,
        pub assigned_items: Vec<LayeredItem>,
    }
    #[derive(Clone, Debug, PartialEq)]
    struct PresentationLayerAssignmentHolder {
        name: PlaceHolder<Label>,
        description: PlaceHolder<Text>,
        assigned_items: PlaceHolder<Vec<LayeredItem>>,
    }
    impl Holder for PresentationLayerAssignmentHolder {
        type Table = Tables;
        type Owned = PresentationLayerAssignment;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PresentationLayerUsage {
        pub assignment: PresentationLayerAssignment,
        pub presentation: PresentationRepresentation,
    }
    #[derive(Clone, Debug, PartialEq)]
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
    struct PresentationRepresentationHolder {
        representation: PlaceHolder<Representation>,
    }
    impl Holder for PresentationRepresentationHolder {
        type Table = Tables;
        type Owned = PresentationRepresentation;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PresentationSet {}
    #[derive(Clone, Debug, PartialEq)]
    struct PresentationSetHolder {}
    impl Holder for PresentationSetHolder {
        type Table = Tables;
        type Owned = PresentationSet;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PresentationSize {
        pub unit: PresentationSizeAssignmentSelect,
        pub size: PlanarBox,
    }
    #[derive(Clone, Debug, PartialEq)]
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
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PresentationStyleAssignment {
        pub styles: Vec<PresentationStyleSelect>,
    }
    #[derive(Clone, Debug, PartialEq)]
    struct PresentationStyleAssignmentHolder {
        styles: PlaceHolder<Vec<PresentationStyleSelect>>,
    }
    impl Holder for PresentationStyleAssignmentHolder {
        type Table = Tables;
        type Owned = PresentationStyleAssignment;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct PresentationViewHolder {
        presentation_representation: PlaceHolder<PresentationRepresentation>,
    }
    impl Holder for PresentationViewHolder {
        type Table = Tables;
        type Owned = PresentationView;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PresentedItem {}
    #[derive(Clone, Debug, PartialEq)]
    struct PresentedItemHolder {}
    impl Holder for PresentedItemHolder {
        type Table = Tables;
        type Owned = PresentedItem;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PresentedItemRepresentation {
        pub presentation: PresentationRepresentationSelect,
        pub item: PresentedItem,
    }
    #[derive(Clone, Debug, PartialEq)]
    struct PresentedItemRepresentationHolder {
        presentation: PlaceHolder<PresentationRepresentationSelect>,
        item: PlaceHolder<PresentedItem>,
    }
    impl Holder for PresentedItemRepresentationHolder {
        type Table = Tables;
        type Owned = PresentedItemRepresentation;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct Product {
        pub id: Identifier,
        pub name: Label,
        pub description: Text,
        pub frame_of_reference: Vec<ProductContext>,
    }
    #[derive(Clone, Debug, PartialEq)]
    struct ProductHolder {
        id: PlaceHolder<Identifier>,
        name: PlaceHolder<Label>,
        description: PlaceHolder<Text>,
        frame_of_reference: PlaceHolder<Vec<ProductContext>>,
    }
    impl Holder for ProductHolder {
        type Table = Tables;
        type Owned = Product;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct ProductContextHolder {
        discipline_type: PlaceHolder<Label>,
        application_context_element: PlaceHolder<ApplicationContextElement>,
    }
    impl Holder for ProductContextHolder {
        type Table = Tables;
        type Owned = ProductContext;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ProductDefinition {
        pub id: Identifier,
        pub description: Text,
        pub formation: ProductDefinitionFormation,
        pub frame_of_reference: ProductDefinitionContext,
    }
    #[derive(Clone, Debug, PartialEq)]
    struct ProductDefinitionHolder {
        id: PlaceHolder<Identifier>,
        description: PlaceHolder<Text>,
        formation: PlaceHolder<ProductDefinitionFormation>,
        frame_of_reference: PlaceHolder<ProductDefinitionContext>,
    }
    impl Holder for ProductDefinitionHolder {
        type Table = Tables;
        type Owned = ProductDefinition;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct ProductDefinitionContextHolder {
        life_cycle_stage: PlaceHolder<Label>,
        application_context_element: PlaceHolder<ApplicationContextElement>,
    }
    impl Holder for ProductDefinitionContextHolder {
        type Table = Tables;
        type Owned = ProductDefinitionContext;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct ProductDefinitionFormation {
        pub id: Identifier,
        pub description: Text,
        pub of_product: Product,
    }
    #[derive(Clone, Debug, PartialEq)]
    struct ProductDefinitionFormationHolder {
        id: PlaceHolder<Identifier>,
        description: PlaceHolder<Text>,
        of_product: PlaceHolder<Product>,
    }
    impl Holder for ProductDefinitionFormationHolder {
        type Table = Tables;
        type Owned = ProductDefinitionFormation;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct ProductDefinitionShapeHolder {
        property_definition: PlaceHolder<PropertyDefinition>,
    }
    impl Holder for ProductDefinitionShapeHolder {
        type Table = Tables;
        type Owned = ProductDefinitionShape;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct ProjectionCurveHolder {
        annotation_curve_occurrence: PlaceHolder<AnnotationCurveOccurrence>,
    }
    impl Holder for ProjectionCurveHolder {
        type Table = Tables;
        type Owned = ProjectionCurve;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct ProjectionDirectedCalloutHolder {
        draughting_callout: PlaceHolder<DraughtingCallout>,
    }
    impl Holder for ProjectionDirectedCalloutHolder {
        type Table = Tables;
        type Owned = ProjectionDirectedCallout;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PropertyDefinition {
        pub name: Label,
        pub description: Text,
        pub definition: CharacterizedDefinition,
    }
    #[derive(Clone, Debug, PartialEq)]
    struct PropertyDefinitionHolder {
        name: PlaceHolder<Label>,
        description: PlaceHolder<Text>,
        definition: PlaceHolder<CharacterizedDefinition>,
    }
    impl Holder for PropertyDefinitionHolder {
        type Table = Tables;
        type Owned = PropertyDefinition;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct PropertyDefinitionRepresentation {
        pub definition: PropertyDefinition,
        pub used_representation: Representation,
    }
    #[derive(Clone, Debug, PartialEq)]
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
    struct QuasiUniformCurveHolder {
        b_spline_curve: PlaceHolder<BSplineCurve>,
    }
    impl Holder for QuasiUniformCurveHolder {
        type Table = Tables;
        type Owned = QuasiUniformCurve;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct RadiusDimensionHolder {
        dimension_curve_directed_callout: PlaceHolder<DimensionCurveDirectedCallout>,
    }
    impl Holder for RadiusDimensionHolder {
        type Table = Tables;
        type Owned = RadiusDimension;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct RationalBSplineCurveHolder {
        weights_data: PlaceHolder<Vec<f64>>,
        b_spline_curve: PlaceHolder<BSplineCurve>,
    }
    impl Holder for RationalBSplineCurveHolder {
        type Table = Tables;
        type Owned = RationalBSplineCurve;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct Representation {
        pub name: Label,
        pub items: Vec<RepresentationItem>,
        pub context_of_items: RepresentationContext,
    }
    #[derive(Clone, Debug, PartialEq)]
    struct RepresentationHolder {
        name: PlaceHolder<Label>,
        items: PlaceHolder<Vec<RepresentationItem>>,
        context_of_items: PlaceHolder<RepresentationContext>,
    }
    impl Holder for RepresentationHolder {
        type Table = Tables;
        type Owned = Representation;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct RepresentationContext {
        pub context_identifier: Identifier,
        pub context_type: Text,
    }
    #[derive(Clone, Debug, PartialEq)]
    struct RepresentationContextHolder {
        context_identifier: PlaceHolder<Identifier>,
        context_type: PlaceHolder<Text>,
    }
    impl Holder for RepresentationContextHolder {
        type Table = Tables;
        type Owned = RepresentationContext;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct RepresentationItem {
        pub name: Label,
    }
    #[derive(Clone, Debug, PartialEq)]
    struct RepresentationItemHolder {
        name: PlaceHolder<Label>,
    }
    impl Holder for RepresentationItemHolder {
        type Table = Tables;
        type Owned = RepresentationItem;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct RepresentationMap {
        pub mapping_origin: RepresentationItem,
        pub mapped_representation: Representation,
    }
    #[derive(Clone, Debug, PartialEq)]
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
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct SecurityClassification {
        pub name: Label,
        pub purpose: Text,
        pub security_level: SecurityClassificationLevel,
    }
    #[derive(Clone, Debug, PartialEq)]
    struct SecurityClassificationHolder {
        name: PlaceHolder<Label>,
        purpose: PlaceHolder<Text>,
        security_level: PlaceHolder<SecurityClassificationLevel>,
    }
    impl Holder for SecurityClassificationHolder {
        type Table = Tables;
        type Owned = SecurityClassification;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct SecurityClassificationAssignment {
        pub assigned_security_classification: SecurityClassification,
    }
    #[derive(Clone, Debug, PartialEq)]
    struct SecurityClassificationAssignmentHolder {
        assigned_security_classification: PlaceHolder<SecurityClassification>,
    }
    impl Holder for SecurityClassificationAssignmentHolder {
        type Table = Tables;
        type Owned = SecurityClassificationAssignment;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct SecurityClassificationLevel {
        pub name: Label,
    }
    #[derive(Clone, Debug, PartialEq)]
    struct SecurityClassificationLevelHolder {
        name: PlaceHolder<Label>,
    }
    impl Holder for SecurityClassificationLevelHolder {
        type Table = Tables;
        type Owned = SecurityClassificationLevel;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct ShapeDefinitionRepresentationHolder {
        property_definition_representation: PlaceHolder<PropertyDefinitionRepresentation>,
    }
    impl Holder for ShapeDefinitionRepresentationHolder {
        type Table = Tables;
        type Owned = ShapeDefinitionRepresentation;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct ShapeRepresentationHolder {
        representation: PlaceHolder<Representation>,
    }
    impl Holder for ShapeRepresentationHolder {
        type Table = Tables;
        type Owned = ShapeRepresentation;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct SiUnitHolder {
        prefix: Option<PlaceHolder<SiPrefix>>,
        name: PlaceHolder<SiUnitName>,
        named_unit: PlaceHolder<NamedUnit>,
    }
    impl Holder for SiUnitHolder {
        type Table = Tables;
        type Owned = SiUnit;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct StructuredDimensionCalloutHolder {
        draughting_callout: PlaceHolder<DraughtingCallout>,
    }
    impl Holder for StructuredDimensionCalloutHolder {
        type Table = Tables;
        type Owned = StructuredDimensionCallout;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct SymbolColour {
        pub colour_of_symbol: Colour,
    }
    #[derive(Clone, Debug, PartialEq)]
    struct SymbolColourHolder {
        colour_of_symbol: PlaceHolder<Colour>,
    }
    impl Holder for SymbolColourHolder {
        type Table = Tables;
        type Owned = SymbolColour;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct SymbolRepresentationHolder {
        representation: PlaceHolder<Representation>,
    }
    impl Holder for SymbolRepresentationHolder {
        type Table = Tables;
        type Owned = SymbolRepresentation;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct SymbolRepresentationMapHolder {
        representation_map: PlaceHolder<RepresentationMap>,
    }
    impl Holder for SymbolRepresentationMapHolder {
        type Table = Tables;
        type Owned = SymbolRepresentationMap;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct SymbolStyle {
        pub name: Label,
        pub style_of_symbol: SymbolStyleSelect,
    }
    #[derive(Clone, Debug, PartialEq)]
    struct SymbolStyleHolder {
        name: PlaceHolder<Label>,
        style_of_symbol: PlaceHolder<SymbolStyleSelect>,
    }
    impl Holder for SymbolStyleHolder {
        type Table = Tables;
        type Owned = SymbolStyle;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct SymbolTargetHolder {
        placement: PlaceHolder<Axis2Placement>,
        x_scale: PlaceHolder<PositiveRatioMeasure>,
        y_scale: PlaceHolder<PositiveRatioMeasure>,
        geometric_representation_item: PlaceHolder<GeometricRepresentationItem>,
    }
    impl Holder for SymbolTargetHolder {
        type Table = Tables;
        type Owned = SymbolTarget;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct TextLiteralHolder {
        literal: PlaceHolder<PresentableText>,
        placement: PlaceHolder<Axis2Placement>,
        alignment: PlaceHolder<TextAlignment>,
        path: PlaceHolder<TextPath>,
        font: PlaceHolder<FontSelect>,
        geometric_representation_item: PlaceHolder<GeometricRepresentationItem>,
    }
    impl Holder for TextLiteralHolder {
        type Table = Tables;
        type Owned = TextLiteral;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct TextLiteralWithAssociatedCurvesHolder {
        associated_curves: PlaceHolder<Vec<Curve>>,
        text_literal: PlaceHolder<TextLiteral>,
    }
    impl Holder for TextLiteralWithAssociatedCurvesHolder {
        type Table = Tables;
        type Owned = TextLiteralWithAssociatedCurves;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct TextLiteralWithDelineationHolder {
        delineation: PlaceHolder<TextDelineation>,
        text_literal: PlaceHolder<TextLiteral>,
    }
    impl Holder for TextLiteralWithDelineationHolder {
        type Table = Tables;
        type Owned = TextLiteralWithDelineation;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct TextStyle {
        pub name: Label,
        pub character_appearance: CharacterStyleSelect,
    }
    #[derive(Clone, Debug, PartialEq)]
    struct TextStyleHolder {
        name: PlaceHolder<Label>,
        character_appearance: PlaceHolder<CharacterStyleSelect>,
    }
    impl Holder for TextStyleHolder {
        type Table = Tables;
        type Owned = TextStyle;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
    }
    #[derive(Clone, Debug, PartialEq, derive_new :: new)]
    pub struct TextStyleForDefinedFont {
        pub text_colour: Colour,
    }
    #[derive(Clone, Debug, PartialEq)]
    struct TextStyleForDefinedFontHolder {
        text_colour: PlaceHolder<Colour>,
    }
    impl Holder for TextStyleForDefinedFontHolder {
        type Table = Tables;
        type Owned = TextStyleForDefinedFont;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct TrimmedCurveHolder {
        basis_curve: PlaceHolder<Curve>,
        trim_1: PlaceHolder<Vec<TrimmingSelect>>,
        trim_2: PlaceHolder<Vec<TrimmingSelect>>,
        sense_agreement: bool,
        master_representation: PlaceHolder<TrimmingPreference>,
        bounded_curve: PlaceHolder<BoundedCurve>,
    }
    impl Holder for TrimmedCurveHolder {
        type Table = Tables;
        type Owned = TrimmedCurve;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct UniformCurveHolder {
        b_spline_curve: PlaceHolder<BSplineCurve>,
    }
    impl Holder for UniformCurveHolder {
        type Table = Tables;
        type Owned = UniformCurve;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
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
    struct VectorHolder {
        orientation: PlaceHolder<Direction>,
        magnitude: PlaceHolder<LengthMeasure>,
        geometric_representation_item: PlaceHolder<GeometricRepresentationItem>,
    }
    impl Holder for VectorHolder {
        type Table = Tables;
        type Owned = Vector;
        fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
            todo!()
        }
    }
}

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
    #[derive(Debug, Clone, PartialEq)]
    pub enum ApprovedItem {
        DrawingRevision(DrawingRevisionAny),
        DrawingSheetRevision(Box<DrawingSheetRevision>),
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum AreaOrView {
        PresentationArea(PresentationAreaAny),
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
        MeasureWithUnit(MeasureWithUnitAny),
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
        DrawingRevision(DrawingRevisionAny),
        DrawingSheetRevision(Box<DrawingSheetRevision>),
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum ContractedItem {
        DrawingRevision(DrawingRevisionAny),
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum CurveFontOrScaledCurveFontSelect {
        CurveStyleFontSelect(Box<CurveStyleFontSelect>),
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum CurveOrAnnotationCurveOccurrence {
        Curve(CurveAny),
        AnnotationCurveOccurrence(AnnotationCurveOccurrenceAny),
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum CurveOrRender {
        CurveStyle(Box<CurveStyle>),
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum CurveStyleFontSelect {
        CurveStyleFont(Box<CurveStyleFont>),
        PreDefinedCurveFont(PreDefinedCurveFontAny),
        ExternallyDefinedCurveFont(Box<ExternallyDefinedCurveFont>),
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum DateTimeSelect {
        Date(DateAny),
    }
    pub type DayInMonthNumber = i64;
    #[derive(Debug, Clone, PartialEq)]
    pub enum DefinedSymbolSelect {
        PreDefinedSymbol(PreDefinedSymbolAny),
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
        AnnotationSymbolOccurrence(AnnotationSymbolOccurrenceAny),
        AnnotationCurveOccurrence(AnnotationCurveOccurrenceAny),
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum DraughtingGroupedItem {
        AnnotationOccurrence(AnnotationOccurrenceAny),
        GeometricSetSelect(Box<GeometricSetSelect>),
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum DraughtingOrganizationItem {
        ProductDefinitionFormation(Box<ProductDefinitionFormation>),
        DrawingRevision(DrawingRevisionAny),
        DrawingSheetRevision(Box<DrawingSheetRevision>),
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum DraughtingPresentedItemSelect {
        ProductDefinitionFormation(Box<ProductDefinitionFormation>),
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum DraughtingTitledItem {
        DrawingRevision(DrawingRevisionAny),
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
        PreDefinedTextFont(PreDefinedTextFontAny),
        ExternallyDefinedTextFont(Box<ExternallyDefinedTextFont>),
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum GeometricSetSelect {
        Point(PointAny),
        Curve(CurveAny),
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum HidingOrBlankingSelect {
        PresentationArea(PresentationAreaAny),
        PresentationView(Box<PresentationView>),
        AnnotationFillArea(Box<AnnotationFillArea>),
    }
    pub type Identifier = String;
    #[derive(Debug, Clone, PartialEq)]
    pub enum InvisibilityContext {
        PresentationLayerUsage(Box<PresentationLayerUsage>),
        PresentationRepresentation(PresentationRepresentationAny),
        PresentationSet(PresentationSetAny),
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum InvisibleItem {
        StyledItem(StyledItemAny),
        PresentationLayerAssignment(Box<PresentationLayerAssignment>),
        PresentationRepresentation(PresentationRepresentationAny),
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
        PresentationRepresentation(PresentationRepresentationAny),
        RepresentationItem(RepresentationItemAny),
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
        PresentationRepresentation(PresentationRepresentationAny),
        PresentationSet(PresentationSetAny),
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum PresentationSizeAssignmentSelect {
        PresentationView(Box<PresentationView>),
        PresentationArea(PresentationAreaAny),
        AreaInSet(AreaInSetAny),
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum PresentationStyleSelect {
        CurveStyle(Box<CurveStyle>),
        SymbolStyle(Box<SymbolStyle>),
        FillAreaStyle(Box<FillAreaStyle>),
        TextStyle(TextStyleAny),
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
        MeasureWithUnit(MeasureWithUnitAny),
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum SourceItem {
        Identifier(Box<Identifier>),
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum SpecifiedItem {
        DrawingRevision(DrawingRevisionAny),
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum StyleContextSelect {
        Representation(RepresentationAny),
        RepresentationItem(RepresentationItemAny),
        PresentationSet(PresentationSetAny),
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
        CompositeText(CompositeTextAny),
        TextLiteral(TextLiteralAny),
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
        NamedUnit(NamedUnitAny),
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum VectorOrDirection {
        Vector(Box<Vector>),
        Direction(Box<Direction>),
    }
    pub type YearNumber = i64;
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = address)]
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
    #[derive(Debug, Clone, PartialEq)]
    pub enum AddressAny {
        OrganizationalAddress(Box<OrganizationalAddress>),
        PersonalAddress(Box<PersonalAddress>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = angular_dimension)]
    pub struct AngularDimension {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = annotation_curve_occurrence)]
    pub struct AnnotationCurveOccurrence {}
    #[derive(Debug, Clone, PartialEq)]
    pub enum AnnotationCurveOccurrenceAny {
        DimensionCurve(Box<DimensionCurve>),
        LeaderCurve(Box<LeaderCurve>),
        ProjectionCurve(Box<ProjectionCurve>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = annotation_fill_area)]
    pub struct AnnotationFillArea {
        pub boundaries: Vec<CurveAny>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = annotation_fill_area_occurrence)]
    pub struct AnnotationFillAreaOccurrence {
        pub fill_style_target: PointAny,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = annotation_occurrence)]
    pub struct AnnotationOccurrence {}
    #[derive(Debug, Clone, PartialEq)]
    pub enum AnnotationOccurrenceAny {
        AnnotationCurveOccurrence(Box<AnnotationCurveOccurrence>),
        AnnotationFillAreaOccurrence(Box<AnnotationFillAreaOccurrence>),
        AnnotationSymbolOccurrence(Box<AnnotationSymbolOccurrence>),
        AnnotationTextOccurrence(Box<AnnotationTextOccurrence>),
        DraughtingAnnotationOccurrence(Box<DraughtingAnnotationOccurrence>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = annotation_subfigure_occurrence)]
    pub struct AnnotationSubfigureOccurrence {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = annotation_symbol)]
    pub struct AnnotationSymbol {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = annotation_symbol_occurrence)]
    pub struct AnnotationSymbolOccurrence {}
    #[derive(Debug, Clone, PartialEq)]
    pub enum AnnotationSymbolOccurrenceAny {
        AnnotationSubfigureOccurrence(Box<AnnotationSubfigureOccurrence>),
        TerminatorSymbol(Box<TerminatorSymbol>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = annotation_text)]
    pub struct AnnotationText {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = annotation_text_occurrence)]
    pub struct AnnotationTextOccurrence {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = application_context)]
    pub struct ApplicationContext {
        pub application: Text,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = application_context_element)]
    pub struct ApplicationContextElement {
        pub name: Label,
        pub frame_of_reference: ApplicationContext,
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum ApplicationContextElementAny {
        ProductContext(Box<ProductContext>),
        ProductDefinitionContext(Box<ProductDefinitionContext>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = application_protocol_definition)]
    pub struct ApplicationProtocolDefinition {
        pub status: Label,
        pub application_interpreted_model_schema_name: Label,
        pub application_protocol_year: YearNumber,
        pub application: ApplicationContext,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = approval)]
    pub struct Approval {
        pub status: ApprovalStatus,
        pub level: Label,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = approval_assignment)]
    pub struct ApprovalAssignment {
        pub assigned_approval: Approval,
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum ApprovalAssignmentAny {
        DraughtingApprovalAssignment(Box<DraughtingApprovalAssignment>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = approval_date_time)]
    pub struct ApprovalDateTime {
        pub date_time: DateTimeSelect,
        pub dated_approval: Approval,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = approval_person_organization)]
    pub struct ApprovalPersonOrganization {
        pub person_organization: PersonOrganizationSelect,
        pub authorized_approval: Approval,
        pub role: ApprovalRole,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = approval_role)]
    pub struct ApprovalRole {
        pub role: Label,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = approval_status)]
    pub struct ApprovalStatus {
        pub name: Label,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = area_in_set)]
    pub struct AreaInSet {
        pub area: PresentationAreaAny,
        pub in_set: PresentationSetAny,
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum AreaInSetAny {
        DrawingSheetRevisionUsage(Box<DrawingSheetRevisionUsage>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = axis2_placement_2d)]
    pub struct Axis2Placement2D {
        pub ref_direction: Option<Direction>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = b_spline_curve)]
    pub struct BSplineCurve {
        pub degree: i64,
        pub control_points_list: Vec<CartesianPoint>,
        pub curve_form: BSplineCurveForm,
        pub closed_curve: Logical,
        pub self_intersect: Logical,
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum BSplineCurveAny {
        BSplineCurveWithKnots(Box<BSplineCurveWithKnots>),
        BezierCurve(Box<BezierCurve>),
        QuasiUniformCurve(Box<QuasiUniformCurve>),
        RationalBSplineCurve(Box<RationalBSplineCurve>),
        UniformCurve(Box<UniformCurve>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = b_spline_curve_with_knots)]
    pub struct BSplineCurveWithKnots {
        pub knot_multiplicities: Vec<i64>,
        pub knots: Vec<ParameterValue>,
        pub knot_spec: KnotType,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = bezier_curve)]
    pub struct BezierCurve {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = bounded_curve)]
    pub struct BoundedCurve {}
    #[derive(Debug, Clone, PartialEq)]
    pub enum BoundedCurveAny {
        BSplineCurve(Box<BSplineCurve>),
        CompositeCurve(Box<CompositeCurve>),
        Polyline(Box<Polyline>),
        TrimmedCurve(Box<TrimmedCurve>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = calendar_date)]
    pub struct CalendarDate {
        pub day_component: DayInMonthNumber,
        pub month_component: MonthInYearNumber,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = camera_image)]
    pub struct CameraImage {}
    #[derive(Debug, Clone, PartialEq)]
    pub enum CameraImageAny {
        CameraImage2DWithScale(Box<CameraImage2DWithScale>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = camera_image_2d_with_scale)]
    pub struct CameraImage2DWithScale {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = camera_model)]
    pub struct CameraModel {}
    #[derive(Debug, Clone, PartialEq)]
    pub enum CameraModelAny {
        CameraModelD2(Box<CameraModelD2>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = camera_model_d2)]
    pub struct CameraModelD2 {
        pub view_window: PlanarBox,
        pub view_window_clipping: bool,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = camera_usage)]
    pub struct CameraUsage {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = cartesian_point)]
    pub struct CartesianPoint {
        pub coordinates: Vec<LengthMeasure>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = circle)]
    pub struct Circle {
        pub radius: PositiveLengthMeasure,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = colour)]
    pub struct Colour {}
    #[derive(Debug, Clone, PartialEq)]
    pub enum ColourAny {
        ColourSpecification(Box<ColourSpecification>),
        PreDefinedColour(Box<PreDefinedColour>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = colour_rgb)]
    pub struct ColourRgb {
        pub red: f64,
        pub green: f64,
        pub blue: f64,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = colour_specification)]
    pub struct ColourSpecification {
        pub name: ColourAny,
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum ColourSpecificationAny {
        ColourRgb(Box<ColourRgb>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = composite_curve)]
    pub struct CompositeCurve {
        pub segments: Vec<CompositeCurveSegment>,
        pub self_intersect: Logical,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = composite_curve_segment)]
    pub struct CompositeCurveSegment {
        pub transition: TransitionCode,
        pub same_sense: bool,
        pub parent_curve: CurveAny,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = composite_text)]
    pub struct CompositeText {
        pub collected_text: Vec<TextOrCharacter>,
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum CompositeTextAny {
        CompositeTextWithAssociatedCurves(Box<CompositeTextWithAssociatedCurves>),
        CompositeTextWithBlankingBox(Box<CompositeTextWithBlankingBox>),
        CompositeTextWithExtent(Box<CompositeTextWithExtent>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = composite_text_with_associated_curves)]
    pub struct CompositeTextWithAssociatedCurves {
        pub associated_curves: Vec<CurveAny>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = composite_text_with_blanking_box)]
    pub struct CompositeTextWithBlankingBox {
        pub blanking: PlanarBox,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = composite_text_with_extent)]
    pub struct CompositeTextWithExtent {
        pub extent: PlanarExtentAny,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = conic)]
    pub struct Conic {
        pub position: Axis2Placement,
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum ConicAny {
        Circle(Box<Circle>),
        Ellipse(Box<Ellipse>),
        Hyperbola(Box<Hyperbola>),
        Parabola(Box<Parabola>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = context_dependent_invisibility)]
    pub struct ContextDependentInvisibility {
        pub presentation_context: InvisibilityContext,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = contract)]
    pub struct Contract {
        pub name: Label,
        pub purpose: Text,
        pub kind: ContractType,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = contract_assignment)]
    pub struct ContractAssignment {
        pub assigned_contract: Contract,
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum ContractAssignmentAny {
        DraughtingContractAssignment(Box<DraughtingContractAssignment>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = contract_type)]
    pub struct ContractType {
        pub description: Label,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = conversion_based_unit)]
    pub struct ConversionBasedUnit {
        pub name: Label,
        pub conversion_factor: MeasureWithUnitAny,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = curve)]
    pub struct Curve {}
    #[derive(Debug, Clone, PartialEq)]
    pub enum CurveAny {
        BoundedCurve(Box<BoundedCurve>),
        Conic(Box<Conic>),
        Line(Box<Line>),
        OffsetCurve2D(Box<OffsetCurve2D>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = curve_dimension)]
    pub struct CurveDimension {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = curve_style)]
    pub struct CurveStyle {
        pub name: Label,
        pub curve_font: CurveFontOrScaledCurveFontSelect,
        pub curve_width: SizeSelect,
        pub curve_colour: ColourAny,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = curve_style_font)]
    pub struct CurveStyleFont {
        pub name: Label,
        pub pattern_list: Vec<CurveStyleFontPattern>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = curve_style_font_pattern)]
    pub struct CurveStyleFontPattern {
        pub visible_segment_length: PositiveLengthMeasure,
        pub invisible_segment_length: PositiveLengthMeasure,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = date)]
    pub struct Date {
        pub year_component: YearNumber,
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum DateAny {
        CalendarDate(Box<CalendarDate>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = datum_feature_callout)]
    pub struct DatumFeatureCallout {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = datum_target_callout)]
    pub struct DatumTargetCallout {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = defined_symbol)]
    pub struct DefinedSymbol {
        pub definition: DefinedSymbolSelect,
        pub target: SymbolTarget,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = diameter_dimension)]
    pub struct DiameterDimension {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = dimension_callout_component_relationship)]
    pub struct DimensionCalloutComponentRelationship {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = dimension_callout_relationship)]
    pub struct DimensionCalloutRelationship {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = dimension_curve)]
    pub struct DimensionCurve {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = dimension_curve_directed_callout)]
    pub struct DimensionCurveDirectedCallout {}
    #[derive(Debug, Clone, PartialEq)]
    pub enum DimensionCurveDirectedCalloutAny {
        AngularDimension(Box<AngularDimension>),
        CurveDimension(Box<CurveDimension>),
        DiameterDimension(Box<DiameterDimension>),
        LinearDimension(Box<LinearDimension>),
        RadiusDimension(Box<RadiusDimension>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = dimension_curve_terminator)]
    pub struct DimensionCurveTerminator {
        pub role: DimensionExtentUsage,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = dimension_pair)]
    pub struct DimensionPair {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = dimensional_exponents)]
    pub struct DimensionalExponents {
        pub length_exponent: f64,
        pub mass_exponent: f64,
        pub time_exponent: f64,
        pub electric_current_exponent: f64,
        pub thermodynamic_temperature_exponent: f64,
        pub amount_of_substance_exponent: f64,
        pub luminous_intensity_exponent: f64,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = direction)]
    pub struct Direction {
        pub direction_ratios: Vec<f64>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = document)]
    pub struct Document {
        pub id: Identifier,
        pub name: Label,
        pub description: Text,
        pub kind: DocumentType,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = document_reference)]
    pub struct DocumentReference {
        pub assigned_document: Document,
        pub source: Label,
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum DocumentReferenceAny {
        DraughtingSpecificationReference(Box<DraughtingSpecificationReference>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = document_type)]
    pub struct DocumentType {
        pub product_data_type: Label,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = draughting_annotation_occurrence)]
    pub struct DraughtingAnnotationOccurrence {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = draughting_approval_assignment)]
    pub struct DraughtingApprovalAssignment {
        pub approved_items: Vec<ApprovedItem>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = draughting_callout)]
    pub struct DraughtingCallout {
        pub contents: Vec<DraughtingCalloutElement>,
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum DraughtingCalloutAny {
        DatumFeatureCallout(Box<DatumFeatureCallout>),
        DatumTargetCallout(Box<DatumTargetCallout>),
        DimensionCurveDirectedCallout(Box<DimensionCurveDirectedCallout>),
        DraughtingElements(Box<DraughtingElements>),
        GeometricalToleranceCallout(Box<GeometricalToleranceCallout>),
        LeaderDirectedCallout(Box<LeaderDirectedCallout>),
        ProjectionDirectedCallout(Box<ProjectionDirectedCallout>),
        StructuredDimensionCallout(Box<StructuredDimensionCallout>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = draughting_callout_relationship)]
    pub struct DraughtingCalloutRelationship {
        pub name: Label,
        pub description: Text,
        pub relating_draughting_callout: DraughtingCalloutAny,
        pub related_draughting_callout: DraughtingCalloutAny,
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum DraughtingCalloutRelationshipAny {
        DimensionCalloutComponentRelationship(Box<DimensionCalloutComponentRelationship>),
        DimensionCalloutRelationship(Box<DimensionCalloutRelationship>),
        DimensionPair(Box<DimensionPair>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = draughting_contract_assignment)]
    pub struct DraughtingContractAssignment {
        pub items: Vec<ContractedItem>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = draughting_drawing_revision)]
    pub struct DraughtingDrawingRevision {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = draughting_elements)]
    pub struct DraughtingElements {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = draughting_group_assignment)]
    pub struct DraughtingGroupAssignment {
        pub items: Vec<DraughtingGroupedItem>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = draughting_model)]
    pub struct DraughtingModel {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = draughting_organization_assignment)]
    pub struct DraughtingOrganizationAssignment {
        pub assigned_items: Vec<DraughtingOrganizationItem>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = draughting_person_and_organization_assignment)]
    pub struct DraughtingPersonAndOrganizationAssignment {
        pub assigned_items: Vec<DraughtingOrganizationItem>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = draughting_person_assignment)]
    pub struct DraughtingPersonAssignment {
        pub assigned_items: Vec<DraughtingOrganizationItem>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = draughting_pre_defined_colour)]
    pub struct DraughtingPreDefinedColour {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = draughting_pre_defined_curve_font)]
    pub struct DraughtingPreDefinedCurveFont {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = draughting_pre_defined_text_font)]
    pub struct DraughtingPreDefinedTextFont {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = draughting_presented_item)]
    pub struct DraughtingPresentedItem {
        pub items: Vec<DraughtingPresentedItemSelect>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = draughting_security_classification_assignment)]
    pub struct DraughtingSecurityClassificationAssignment {
        pub assigned_items: Vec<ClassifiedItem>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = draughting_specification_reference)]
    pub struct DraughtingSpecificationReference {
        pub specified_items: Vec<SpecifiedItem>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = draughting_subfigure_representation)]
    pub struct DraughtingSubfigureRepresentation {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = draughting_symbol_representation)]
    pub struct DraughtingSymbolRepresentation {}
    #[derive(Debug, Clone, PartialEq)]
    pub enum DraughtingSymbolRepresentationAny {
        DrawingSheetLayout(Box<DrawingSheetLayout>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = draughting_text_literal_with_delineation)]
    pub struct DraughtingTextLiteralWithDelineation {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = draughting_title)]
    pub struct DraughtingTitle {
        pub items: Vec<DraughtingTitledItem>,
        pub language: Label,
        pub contents: Text,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = drawing_definition)]
    pub struct DrawingDefinition {
        pub drawing_number: Identifier,
        pub drawing_type: Option<Label>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = drawing_revision)]
    pub struct DrawingRevision {
        pub revision_identifier: Identifier,
        pub drawing_identifier: DrawingDefinition,
        pub intended_scale: Option<Text>,
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum DrawingRevisionAny {
        DraughtingDrawingRevision(Box<DraughtingDrawingRevision>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = drawing_sheet_layout)]
    pub struct DrawingSheetLayout {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = drawing_sheet_revision)]
    pub struct DrawingSheetRevision {
        pub revision_identifier: Identifier,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = drawing_sheet_revision_usage)]
    pub struct DrawingSheetRevisionUsage {
        pub sheet_number: Identifier,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = ellipse)]
    pub struct Ellipse {
        pub semi_axis_1: PositiveLengthMeasure,
        pub semi_axis_2: PositiveLengthMeasure,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = external_source)]
    pub struct ExternalSource {
        pub source_id: SourceItem,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = externally_defined_curve_font)]
    pub struct ExternallyDefinedCurveFont {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = externally_defined_hatch_style)]
    pub struct ExternallyDefinedHatchStyle {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = externally_defined_item)]
    pub struct ExternallyDefinedItem {
        pub item_id: SourceItem,
        pub source: ExternalSource,
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum ExternallyDefinedItemAny {
        ExternallyDefinedCurveFont(Box<ExternallyDefinedCurveFont>),
        ExternallyDefinedHatchStyle(Box<ExternallyDefinedHatchStyle>),
        ExternallyDefinedSymbol(Box<ExternallyDefinedSymbol>),
        ExternallyDefinedTextFont(Box<ExternallyDefinedTextFont>),
        ExternallyDefinedTileStyle(Box<ExternallyDefinedTileStyle>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = externally_defined_symbol)]
    pub struct ExternallyDefinedSymbol {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = externally_defined_text_font)]
    pub struct ExternallyDefinedTextFont {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = externally_defined_tile_style)]
    pub struct ExternallyDefinedTileStyle {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = fill_area_style)]
    pub struct FillAreaStyle {
        pub name: Label,
        pub fill_styles: Vec<FillStyleSelect>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = fill_area_style_colour)]
    pub struct FillAreaStyleColour {
        pub name: Label,
        pub fill_colour: ColourAny,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = fill_area_style_hatching)]
    pub struct FillAreaStyleHatching {
        pub hatch_line_appearance: CurveStyle,
        pub start_of_next_hatch_line: OneDirectionRepeatFactorAny,
        pub point_of_reference_hatch_line: CartesianPoint,
        pub pattern_start: CartesianPoint,
        pub hatch_line_angle: PlaneAngleMeasure,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = fill_area_style_tile_symbol_with_style)]
    pub struct FillAreaStyleTileSymbolWithStyle {
        pub symbol: AnnotationSymbolOccurrenceAny,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = fill_area_style_tiles)]
    pub struct FillAreaStyleTiles {
        pub tiling_pattern: TwoDirectionRepeatFactor,
        pub tiles: Vec<FillAreaStyleTileShapeSelect>,
        pub tiling_scale: PositiveRatioMeasure,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = geometric_curve_set)]
    pub struct GeometricCurveSet {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = geometric_representation_context)]
    pub struct GeometricRepresentationContext {
        pub coordinate_space_dimension: DimensionCount,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = geometric_representation_item)]
    pub struct GeometricRepresentationItem {}
    #[derive(Debug, Clone, PartialEq)]
    pub enum GeometricRepresentationItemAny {
        AnnotationFillArea(Box<AnnotationFillArea>),
        CameraModel(Box<CameraModel>),
        CompositeText(Box<CompositeText>),
        Curve(Box<Curve>),
        DefinedSymbol(Box<DefinedSymbol>),
        Direction(Box<Direction>),
        DraughtingCallout(Box<DraughtingCallout>),
        ExternallyDefinedHatchStyle(Box<ExternallyDefinedHatchStyle>),
        ExternallyDefinedTileStyle(Box<ExternallyDefinedTileStyle>),
        FillAreaStyleHatching(Box<FillAreaStyleHatching>),
        FillAreaStyleTileSymbolWithStyle(Box<FillAreaStyleTileSymbolWithStyle>),
        FillAreaStyleTiles(Box<FillAreaStyleTiles>),
        GeometricSet(Box<GeometricSet>),
        OneDirectionRepeatFactor(Box<OneDirectionRepeatFactor>),
        Placement(Box<Placement>),
        PlanarExtent(Box<PlanarExtent>),
        Point(Box<Point>),
        SymbolTarget(Box<SymbolTarget>),
        TextLiteral(Box<TextLiteral>),
        Vector(Box<Vector>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = geometric_set)]
    pub struct GeometricSet {
        pub elements: Vec<GeometricSetSelect>,
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum GeometricSetAny {
        GeometricCurveSet(Box<GeometricCurveSet>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = geometrical_tolerance_callout)]
    pub struct GeometricalToleranceCallout {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = geometrically_bounded_2d_wireframe_representation)]
    pub struct GeometricallyBounded2DWireframeRepresentation {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = global_unit_assigned_context)]
    pub struct GlobalUnitAssignedContext {
        pub units: Vec<Unit>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = group)]
    pub struct Group {
        pub name: Label,
        pub description: Text,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = group_assignment)]
    pub struct GroupAssignment {
        pub assigned_group: Group,
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum GroupAssignmentAny {
        DraughtingGroupAssignment(Box<DraughtingGroupAssignment>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = group_relationship)]
    pub struct GroupRelationship {
        pub name: Label,
        pub description: Text,
        pub relating_group: Group,
        pub related_group: Group,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = hyperbola)]
    pub struct Hyperbola {
        pub semi_axis: PositiveLengthMeasure,
        pub semi_imag_axis: PositiveLengthMeasure,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = invisibility)]
    pub struct Invisibility {
        pub invisible_items: Vec<InvisibleItem>,
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum InvisibilityAny {
        ContextDependentInvisibility(Box<ContextDependentInvisibility>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = leader_curve)]
    pub struct LeaderCurve {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = leader_directed_callout)]
    pub struct LeaderDirectedCallout {}
    #[derive(Debug, Clone, PartialEq)]
    pub enum LeaderDirectedCalloutAny {
        LeaderDirectedDimension(Box<LeaderDirectedDimension>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = leader_directed_dimension)]
    pub struct LeaderDirectedDimension {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = leader_terminator)]
    pub struct LeaderTerminator {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = length_measure_with_unit)]
    pub struct LengthMeasureWithUnit {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = length_unit)]
    pub struct LengthUnit {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = line)]
    pub struct Line {
        pub pnt: CartesianPoint,
        pub dir: Vector,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = linear_dimension)]
    pub struct LinearDimension {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = mapped_item)]
    pub struct MappedItem {
        pub mapping_source: RepresentationMapAny,
        pub mapping_target: RepresentationItemAny,
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum MappedItemAny {
        AnnotationSymbol(Box<AnnotationSymbol>),
        AnnotationText(Box<AnnotationText>),
        CameraImage(Box<CameraImage>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = measure_with_unit)]
    pub struct MeasureWithUnit {
        pub value_component: MeasureValue,
        pub unit_component: Unit,
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum MeasureWithUnitAny {
        LengthMeasureWithUnit(Box<LengthMeasureWithUnit>),
        PlaneAngleMeasureWithUnit(Box<PlaneAngleMeasureWithUnit>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = named_unit)]
    pub struct NamedUnit {
        pub dimensions: DimensionalExponents,
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum NamedUnitAny {
        ConversionBasedUnit(Box<ConversionBasedUnit>),
        LengthUnit(Box<LengthUnit>),
        PlaneAngleUnit(Box<PlaneAngleUnit>),
        SiUnit(Box<SiUnit>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = offset_curve_2d)]
    pub struct OffsetCurve2D {
        pub basis_curve: CurveAny,
        pub distance: LengthMeasure,
        pub self_intersect: Logical,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = one_direction_repeat_factor)]
    pub struct OneDirectionRepeatFactor {
        pub repeat_factor: Vector,
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum OneDirectionRepeatFactorAny {
        TwoDirectionRepeatFactor(Box<TwoDirectionRepeatFactor>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = ordinate_dimension)]
    pub struct OrdinateDimension {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = organization)]
    pub struct Organization {
        pub id: Option<Identifier>,
        pub name: Label,
        pub description: Text,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = organization_assignment)]
    pub struct OrganizationAssignment {
        pub assigned_organization: Organization,
        pub role: OrganizationRole,
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum OrganizationAssignmentAny {
        DraughtingOrganizationAssignment(Box<DraughtingOrganizationAssignment>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = organization_role)]
    pub struct OrganizationRole {
        pub name: Label,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = organizational_address)]
    pub struct OrganizationalAddress {
        pub organizations: Vec<Organization>,
        pub description: Text,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = parabola)]
    pub struct Parabola {
        pub focal_dist: LengthMeasure,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = person)]
    pub struct Person {
        pub id: Identifier,
        pub last_name: Option<Label>,
        pub first_name: Option<Label>,
        pub middle_names: Option<Vec<Label>>,
        pub prefix_titles: Option<Vec<Label>>,
        pub suffix_titles: Option<Vec<Label>>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = person_and_organization)]
    pub struct PersonAndOrganization {
        pub the_person: Person,
        pub the_organization: Organization,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = person_and_organization_assignment)]
    pub struct PersonAndOrganizationAssignment {
        pub assigned_person_and_organization: PersonAndOrganization,
        pub role: PersonAndOrganizationRole,
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum PersonAndOrganizationAssignmentAny {
        DraughtingPersonAndOrganizationAssignment(Box<DraughtingPersonAndOrganizationAssignment>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = person_and_organization_role)]
    pub struct PersonAndOrganizationRole {
        pub name: Label,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = person_assignment)]
    pub struct PersonAssignment {
        pub assigned_person: Person,
        pub role: PersonRole,
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum PersonAssignmentAny {
        DraughtingPersonAssignment(Box<DraughtingPersonAssignment>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = person_role)]
    pub struct PersonRole {
        pub name: Label,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = personal_address)]
    pub struct PersonalAddress {
        pub people: Vec<Person>,
        pub description: Text,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = placement)]
    pub struct Placement {
        pub location: CartesianPoint,
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum PlacementAny {
        Axis2Placement2D(Box<Axis2Placement2D>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = planar_box)]
    pub struct PlanarBox {
        pub placement: Axis2Placement,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = planar_extent)]
    pub struct PlanarExtent {
        pub size_in_x: LengthMeasure,
        pub size_in_y: LengthMeasure,
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum PlanarExtentAny {
        PlanarBox(Box<PlanarBox>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = plane_angle_measure_with_unit)]
    pub struct PlaneAngleMeasureWithUnit {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = plane_angle_unit)]
    pub struct PlaneAngleUnit {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = point)]
    pub struct Point {}
    #[derive(Debug, Clone, PartialEq)]
    pub enum PointAny {
        CartesianPoint(Box<CartesianPoint>),
        PointOnCurve(Box<PointOnCurve>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = point_on_curve)]
    pub struct PointOnCurve {
        pub basis_curve: CurveAny,
        pub point_parameter: ParameterValue,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = polyline)]
    pub struct Polyline {
        pub points: Vec<CartesianPoint>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = pre_defined_colour)]
    pub struct PreDefinedColour {}
    #[derive(Debug, Clone, PartialEq)]
    pub enum PreDefinedColourAny {
        DraughtingPreDefinedColour(Box<DraughtingPreDefinedColour>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = pre_defined_curve_font)]
    pub struct PreDefinedCurveFont {}
    #[derive(Debug, Clone, PartialEq)]
    pub enum PreDefinedCurveFontAny {
        DraughtingPreDefinedCurveFont(Box<DraughtingPreDefinedCurveFont>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = pre_defined_dimension_symbol)]
    pub struct PreDefinedDimensionSymbol {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = pre_defined_geometrical_tolerance_symbol)]
    pub struct PreDefinedGeometricalToleranceSymbol {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = pre_defined_item)]
    pub struct PreDefinedItem {
        pub name: Label,
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum PreDefinedItemAny {
        PreDefinedColour(Box<PreDefinedColour>),
        PreDefinedCurveFont(Box<PreDefinedCurveFont>),
        PreDefinedSymbol(Box<PreDefinedSymbol>),
        PreDefinedTextFont(Box<PreDefinedTextFont>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = pre_defined_point_marker_symbol)]
    pub struct PreDefinedPointMarkerSymbol {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = pre_defined_symbol)]
    pub struct PreDefinedSymbol {}
    #[derive(Debug, Clone, PartialEq)]
    pub enum PreDefinedSymbolAny {
        PreDefinedDimensionSymbol(Box<PreDefinedDimensionSymbol>),
        PreDefinedGeometricalToleranceSymbol(Box<PreDefinedGeometricalToleranceSymbol>),
        PreDefinedPointMarkerSymbol(Box<PreDefinedPointMarkerSymbol>),
        PreDefinedTerminatorSymbol(Box<PreDefinedTerminatorSymbol>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = pre_defined_terminator_symbol)]
    pub struct PreDefinedTerminatorSymbol {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = pre_defined_text_font)]
    pub struct PreDefinedTextFont {}
    #[derive(Debug, Clone, PartialEq)]
    pub enum PreDefinedTextFontAny {
        DraughtingPreDefinedTextFont(Box<DraughtingPreDefinedTextFont>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = presentation_area)]
    pub struct PresentationArea {}
    #[derive(Debug, Clone, PartialEq)]
    pub enum PresentationAreaAny {
        DrawingSheetRevision(Box<DrawingSheetRevision>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = presentation_layer_assignment)]
    pub struct PresentationLayerAssignment {
        pub name: Label,
        pub description: Text,
        pub assigned_items: Vec<LayeredItem>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = presentation_layer_usage)]
    pub struct PresentationLayerUsage {
        pub assignment: PresentationLayerAssignment,
        pub presentation: PresentationRepresentationAny,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = presentation_representation)]
    pub struct PresentationRepresentation {}
    #[derive(Debug, Clone, PartialEq)]
    pub enum PresentationRepresentationAny {
        PresentationArea(Box<PresentationArea>),
        PresentationView(Box<PresentationView>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = presentation_set)]
    pub struct PresentationSet {}
    #[derive(Debug, Clone, PartialEq)]
    pub enum PresentationSetAny {
        DrawingRevision(Box<DrawingRevision>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = presentation_size)]
    pub struct PresentationSize {
        pub unit: PresentationSizeAssignmentSelect,
        pub size: PlanarBox,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = presentation_style_assignment)]
    pub struct PresentationStyleAssignment {
        pub styles: Vec<PresentationStyleSelect>,
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum PresentationStyleAssignmentAny {
        PresentationStyleByContext(Box<PresentationStyleByContext>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = presentation_style_by_context)]
    pub struct PresentationStyleByContext {
        pub style_context: StyleContextSelect,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = presentation_view)]
    pub struct PresentationView {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = presented_item)]
    pub struct PresentedItem {}
    #[derive(Debug, Clone, PartialEq)]
    pub enum PresentedItemAny {
        DraughtingPresentedItem(Box<DraughtingPresentedItem>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = presented_item_representation)]
    pub struct PresentedItemRepresentation {
        pub presentation: PresentationRepresentationSelect,
        pub item: PresentedItemAny,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = product)]
    pub struct Product {
        pub id: Identifier,
        pub name: Label,
        pub description: Text,
        pub frame_of_reference: Vec<ProductContext>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = product_context)]
    pub struct ProductContext {
        pub discipline_type: Label,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = product_definition)]
    pub struct ProductDefinition {
        pub id: Identifier,
        pub description: Text,
        pub formation: ProductDefinitionFormation,
        pub frame_of_reference: ProductDefinitionContext,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = product_definition_context)]
    pub struct ProductDefinitionContext {
        pub life_cycle_stage: Label,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = product_definition_formation)]
    pub struct ProductDefinitionFormation {
        pub id: Identifier,
        pub description: Text,
        pub of_product: Product,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = product_definition_shape)]
    pub struct ProductDefinitionShape {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = projection_curve)]
    pub struct ProjectionCurve {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = projection_directed_callout)]
    pub struct ProjectionDirectedCallout {}
    #[derive(Debug, Clone, PartialEq)]
    pub enum ProjectionDirectedCalloutAny {
        OrdinateDimension(Box<OrdinateDimension>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = property_definition)]
    pub struct PropertyDefinition {
        pub name: Label,
        pub description: Text,
        pub definition: CharacterizedDefinition,
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum PropertyDefinitionAny {
        ProductDefinitionShape(Box<ProductDefinitionShape>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = property_definition_representation)]
    pub struct PropertyDefinitionRepresentation {
        pub definition: PropertyDefinitionAny,
        pub used_representation: RepresentationAny,
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum PropertyDefinitionRepresentationAny {
        ShapeDefinitionRepresentation(Box<ShapeDefinitionRepresentation>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = quasi_uniform_curve)]
    pub struct QuasiUniformCurve {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = radius_dimension)]
    pub struct RadiusDimension {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = rational_b_spline_curve)]
    pub struct RationalBSplineCurve {
        pub weights_data: Vec<f64>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = representation)]
    pub struct Representation {
        pub name: Label,
        pub items: Vec<RepresentationItemAny>,
        pub context_of_items: RepresentationContextAny,
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum RepresentationAny {
        DraughtingModel(Box<DraughtingModel>),
        PresentationRepresentation(Box<PresentationRepresentation>),
        ShapeRepresentation(Box<ShapeRepresentation>),
        SymbolRepresentation(Box<SymbolRepresentation>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = representation_context)]
    pub struct RepresentationContext {
        pub context_identifier: Identifier,
        pub context_type: Text,
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum RepresentationContextAny {
        GeometricRepresentationContext(Box<GeometricRepresentationContext>),
        GlobalUnitAssignedContext(Box<GlobalUnitAssignedContext>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = representation_item)]
    pub struct RepresentationItem {
        pub name: Label,
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum RepresentationItemAny {
        GeometricRepresentationItem(Box<GeometricRepresentationItem>),
        MappedItem(Box<MappedItem>),
        StyledItem(Box<StyledItem>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = representation_map)]
    pub struct RepresentationMap {
        pub mapping_origin: RepresentationItemAny,
        pub mapped_representation: RepresentationAny,
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum RepresentationMapAny {
        CameraUsage(Box<CameraUsage>),
        SymbolRepresentationMap(Box<SymbolRepresentationMap>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = security_classification)]
    pub struct SecurityClassification {
        pub name: Label,
        pub purpose: Text,
        pub security_level: SecurityClassificationLevel,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = security_classification_assignment)]
    pub struct SecurityClassificationAssignment {
        pub assigned_security_classification: SecurityClassification,
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum SecurityClassificationAssignmentAny {
        DraughtingSecurityClassificationAssignment(Box<DraughtingSecurityClassificationAssignment>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = security_classification_level)]
    pub struct SecurityClassificationLevel {
        pub name: Label,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = shape_definition_representation)]
    pub struct ShapeDefinitionRepresentation {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = shape_representation)]
    pub struct ShapeRepresentation {}
    #[derive(Debug, Clone, PartialEq)]
    pub enum ShapeRepresentationAny {
        GeometricallyBounded2DWireframeRepresentation(
            Box<GeometricallyBounded2DWireframeRepresentation>,
        ),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = si_unit)]
    pub struct SiUnit {
        pub prefix: Option<SiPrefix>,
        pub name: SiUnitName,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = structured_dimension_callout)]
    pub struct StructuredDimensionCallout {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = styled_item)]
    pub struct StyledItem {
        pub styles: Vec<PresentationStyleAssignmentAny>,
        pub item: RepresentationItemAny,
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum StyledItemAny {
        AnnotationOccurrence(Box<AnnotationOccurrence>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = symbol_colour)]
    pub struct SymbolColour {
        pub colour_of_symbol: ColourAny,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = symbol_representation)]
    pub struct SymbolRepresentation {}
    #[derive(Debug, Clone, PartialEq)]
    pub enum SymbolRepresentationAny {
        DraughtingSubfigureRepresentation(Box<DraughtingSubfigureRepresentation>),
        DraughtingSymbolRepresentation(Box<DraughtingSymbolRepresentation>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = symbol_representation_map)]
    pub struct SymbolRepresentationMap {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = symbol_style)]
    pub struct SymbolStyle {
        pub name: Label,
        pub style_of_symbol: SymbolStyleSelect,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = symbol_target)]
    pub struct SymbolTarget {
        pub placement: Axis2Placement,
        pub x_scale: PositiveRatioMeasure,
        pub y_scale: PositiveRatioMeasure,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = terminator_symbol)]
    pub struct TerminatorSymbol {
        pub annotated_curve: AnnotationCurveOccurrenceAny,
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum TerminatorSymbolAny {
        DimensionCurveTerminator(Box<DimensionCurveTerminator>),
        LeaderTerminator(Box<LeaderTerminator>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = text_literal)]
    pub struct TextLiteral {
        pub literal: PresentableText,
        pub placement: Axis2Placement,
        pub alignment: TextAlignment,
        pub path: TextPath,
        pub font: FontSelect,
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum TextLiteralAny {
        TextLiteralWithAssociatedCurves(Box<TextLiteralWithAssociatedCurves>),
        TextLiteralWithBlankingBox(Box<TextLiteralWithBlankingBox>),
        TextLiteralWithDelineation(Box<TextLiteralWithDelineation>),
        TextLiteralWithExtent(Box<TextLiteralWithExtent>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = text_literal_with_associated_curves)]
    pub struct TextLiteralWithAssociatedCurves {
        pub associated_curves: Vec<CurveAny>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = text_literal_with_blanking_box)]
    pub struct TextLiteralWithBlankingBox {
        pub blanking: PlanarBox,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = text_literal_with_delineation)]
    pub struct TextLiteralWithDelineation {
        pub delineation: TextDelineation,
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum TextLiteralWithDelineationAny {
        DraughtingTextLiteralWithDelineation(Box<DraughtingTextLiteralWithDelineation>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = text_literal_with_extent)]
    pub struct TextLiteralWithExtent {
        pub extent: PlanarExtentAny,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = text_style)]
    pub struct TextStyle {
        pub name: Label,
        pub character_appearance: CharacterStyleSelect,
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum TextStyleAny {
        TextStyleWithBoxCharacteristics(Box<TextStyleWithBoxCharacteristics>),
        TextStyleWithMirror(Box<TextStyleWithMirror>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = text_style_for_defined_font)]
    pub struct TextStyleForDefinedFont {
        pub text_colour: ColourAny,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = text_style_with_box_characteristics)]
    pub struct TextStyleWithBoxCharacteristics {
        pub characteristics: Vec<BoxCharacteristicSelect>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = text_style_with_mirror)]
    pub struct TextStyleWithMirror {
        pub mirror_placement: Axis2Placement,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = trimmed_curve)]
    pub struct TrimmedCurve {
        pub basis_curve: CurveAny,
        pub trim_1: Vec<TrimmingSelect>,
        pub trim_2: Vec<TrimmingSelect>,
        pub sense_agreement: bool,
        pub master_representation: TrimmingPreference,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = two_direction_repeat_factor)]
    pub struct TwoDirectionRepeatFactor {
        pub second_repeat_factor: Vector,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = uniform_curve)]
    pub struct UniformCurve {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables , field = vector)]
    pub struct Vector {
        pub orientation: Direction,
        pub magnitude: LengthMeasure,
    }
}

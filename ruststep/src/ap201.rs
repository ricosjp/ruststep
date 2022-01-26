#![allow(dead_code)]
pub mod explicit_draughting {
    use crate::{as_holder, derive_more::*, primitive::*, Holder, TableInit};
    use std::collections::HashMap;
    #[derive(Debug, Clone, PartialEq, Default, TableInit)]
    pub struct Tables {
        address: HashMap<u64, as_holder!(Address)>,
        angular_dimension: HashMap<u64, as_holder!(AngularDimension)>,
        annotation_curve_occurrence: HashMap<u64, as_holder!(AnnotationCurveOccurrence)>,
        annotation_fill_area: HashMap<u64, as_holder!(AnnotationFillArea)>,
        annotation_fill_area_occurrence: HashMap<u64, as_holder!(AnnotationFillAreaOccurrence)>,
        annotation_occurrence: HashMap<u64, as_holder!(AnnotationOccurrence)>,
        annotation_subfigure_occurrence: HashMap<u64, as_holder!(AnnotationSubfigureOccurrence)>,
        annotation_symbol: HashMap<u64, as_holder!(AnnotationSymbol)>,
        annotation_symbol_occurrence: HashMap<u64, as_holder!(AnnotationSymbolOccurrence)>,
        annotation_text: HashMap<u64, as_holder!(AnnotationText)>,
        annotation_text_occurrence: HashMap<u64, as_holder!(AnnotationTextOccurrence)>,
        application_context: HashMap<u64, as_holder!(ApplicationContext)>,
        application_context_element: HashMap<u64, as_holder!(ApplicationContextElement)>,
        application_protocol_definition: HashMap<u64, as_holder!(ApplicationProtocolDefinition)>,
        approval: HashMap<u64, as_holder!(Approval)>,
        approval_assignment: HashMap<u64, as_holder!(ApprovalAssignment)>,
        approval_date_time: HashMap<u64, as_holder!(ApprovalDateTime)>,
        approval_person_organization: HashMap<u64, as_holder!(ApprovalPersonOrganization)>,
        approval_role: HashMap<u64, as_holder!(ApprovalRole)>,
        approval_status: HashMap<u64, as_holder!(ApprovalStatus)>,
        area_in_set: HashMap<u64, as_holder!(AreaInSet)>,
        axis2_placement_2d: HashMap<u64, as_holder!(Axis2Placement2D)>,
        b_spline_curve: HashMap<u64, as_holder!(BSplineCurve)>,
        b_spline_curve_with_knots: HashMap<u64, as_holder!(BSplineCurveWithKnots)>,
        bezier_curve: HashMap<u64, as_holder!(BezierCurve)>,
        bounded_curve: HashMap<u64, as_holder!(BoundedCurve)>,
        calendar_date: HashMap<u64, as_holder!(CalendarDate)>,
        camera_image: HashMap<u64, as_holder!(CameraImage)>,
        camera_image_2d_with_scale: HashMap<u64, as_holder!(CameraImage2DWithScale)>,
        camera_model: HashMap<u64, as_holder!(CameraModel)>,
        camera_model_d2: HashMap<u64, as_holder!(CameraModelD2)>,
        camera_usage: HashMap<u64, as_holder!(CameraUsage)>,
        cartesian_point: HashMap<u64, as_holder!(CartesianPoint)>,
        circle: HashMap<u64, as_holder!(Circle)>,
        colour: HashMap<u64, as_holder!(Colour)>,
        colour_rgb: HashMap<u64, as_holder!(ColourRgb)>,
        colour_specification: HashMap<u64, as_holder!(ColourSpecification)>,
        composite_curve: HashMap<u64, as_holder!(CompositeCurve)>,
        composite_curve_segment: HashMap<u64, as_holder!(CompositeCurveSegment)>,
        composite_text: HashMap<u64, as_holder!(CompositeText)>,
        composite_text_with_associated_curves:
            HashMap<u64, as_holder!(CompositeTextWithAssociatedCurves)>,
        composite_text_with_blanking_box: HashMap<u64, as_holder!(CompositeTextWithBlankingBox)>,
        composite_text_with_extent: HashMap<u64, as_holder!(CompositeTextWithExtent)>,
        conic: HashMap<u64, as_holder!(Conic)>,
        context_dependent_invisibility: HashMap<u64, as_holder!(ContextDependentInvisibility)>,
        contract: HashMap<u64, as_holder!(Contract)>,
        contract_assignment: HashMap<u64, as_holder!(ContractAssignment)>,
        contract_type: HashMap<u64, as_holder!(ContractType)>,
        conversion_based_unit: HashMap<u64, as_holder!(ConversionBasedUnit)>,
        curve: HashMap<u64, as_holder!(Curve)>,
        curve_dimension: HashMap<u64, as_holder!(CurveDimension)>,
        curve_style: HashMap<u64, as_holder!(CurveStyle)>,
        curve_style_font: HashMap<u64, as_holder!(CurveStyleFont)>,
        curve_style_font_pattern: HashMap<u64, as_holder!(CurveStyleFontPattern)>,
        date: HashMap<u64, as_holder!(Date)>,
        datum_feature_callout: HashMap<u64, as_holder!(DatumFeatureCallout)>,
        datum_target_callout: HashMap<u64, as_holder!(DatumTargetCallout)>,
        defined_symbol: HashMap<u64, as_holder!(DefinedSymbol)>,
        diameter_dimension: HashMap<u64, as_holder!(DiameterDimension)>,
        dimension_callout_component_relationship:
            HashMap<u64, as_holder!(DimensionCalloutComponentRelationship)>,
        dimension_callout_relationship: HashMap<u64, as_holder!(DimensionCalloutRelationship)>,
        dimension_curve: HashMap<u64, as_holder!(DimensionCurve)>,
        dimension_curve_directed_callout: HashMap<u64, as_holder!(DimensionCurveDirectedCallout)>,
        dimension_curve_terminator: HashMap<u64, as_holder!(DimensionCurveTerminator)>,
        dimension_pair: HashMap<u64, as_holder!(DimensionPair)>,
        dimensional_exponents: HashMap<u64, as_holder!(DimensionalExponents)>,
        direction: HashMap<u64, as_holder!(Direction)>,
        document: HashMap<u64, as_holder!(Document)>,
        document_reference: HashMap<u64, as_holder!(DocumentReference)>,
        document_type: HashMap<u64, as_holder!(DocumentType)>,
        draughting_annotation_occurrence: HashMap<u64, as_holder!(DraughtingAnnotationOccurrence)>,
        draughting_approval_assignment: HashMap<u64, as_holder!(DraughtingApprovalAssignment)>,
        draughting_callout: HashMap<u64, as_holder!(DraughtingCallout)>,
        draughting_callout_relationship: HashMap<u64, as_holder!(DraughtingCalloutRelationship)>,
        draughting_contract_assignment: HashMap<u64, as_holder!(DraughtingContractAssignment)>,
        draughting_drawing_revision: HashMap<u64, as_holder!(DraughtingDrawingRevision)>,
        draughting_elements: HashMap<u64, as_holder!(DraughtingElements)>,
        draughting_group_assignment: HashMap<u64, as_holder!(DraughtingGroupAssignment)>,
        draughting_model: HashMap<u64, as_holder!(DraughtingModel)>,
        draughting_organization_assignment:
            HashMap<u64, as_holder!(DraughtingOrganizationAssignment)>,
        draughting_person_and_organization_assignment:
            HashMap<u64, as_holder!(DraughtingPersonAndOrganizationAssignment)>,
        draughting_person_assignment: HashMap<u64, as_holder!(DraughtingPersonAssignment)>,
        draughting_pre_defined_colour: HashMap<u64, as_holder!(DraughtingPreDefinedColour)>,
        draughting_pre_defined_curve_font: HashMap<u64, as_holder!(DraughtingPreDefinedCurveFont)>,
        draughting_pre_defined_text_font: HashMap<u64, as_holder!(DraughtingPreDefinedTextFont)>,
        draughting_presented_item: HashMap<u64, as_holder!(DraughtingPresentedItem)>,
        draughting_security_classification_assignment:
            HashMap<u64, as_holder!(DraughtingSecurityClassificationAssignment)>,
        draughting_specification_reference:
            HashMap<u64, as_holder!(DraughtingSpecificationReference)>,
        draughting_subfigure_representation:
            HashMap<u64, as_holder!(DraughtingSubfigureRepresentation)>,
        draughting_symbol_representation: HashMap<u64, as_holder!(DraughtingSymbolRepresentation)>,
        draughting_text_literal_with_delineation:
            HashMap<u64, as_holder!(DraughtingTextLiteralWithDelineation)>,
        draughting_title: HashMap<u64, as_holder!(DraughtingTitle)>,
        drawing_definition: HashMap<u64, as_holder!(DrawingDefinition)>,
        drawing_revision: HashMap<u64, as_holder!(DrawingRevision)>,
        drawing_sheet_layout: HashMap<u64, as_holder!(DrawingSheetLayout)>,
        drawing_sheet_revision: HashMap<u64, as_holder!(DrawingSheetRevision)>,
        drawing_sheet_revision_usage: HashMap<u64, as_holder!(DrawingSheetRevisionUsage)>,
        ellipse: HashMap<u64, as_holder!(Ellipse)>,
        external_source: HashMap<u64, as_holder!(ExternalSource)>,
        externally_defined_curve_font: HashMap<u64, as_holder!(ExternallyDefinedCurveFont)>,
        externally_defined_hatch_style: HashMap<u64, as_holder!(ExternallyDefinedHatchStyle)>,
        externally_defined_item: HashMap<u64, as_holder!(ExternallyDefinedItem)>,
        externally_defined_symbol: HashMap<u64, as_holder!(ExternallyDefinedSymbol)>,
        externally_defined_text_font: HashMap<u64, as_holder!(ExternallyDefinedTextFont)>,
        externally_defined_tile_style: HashMap<u64, as_holder!(ExternallyDefinedTileStyle)>,
        fill_area_style: HashMap<u64, as_holder!(FillAreaStyle)>,
        fill_area_style_colour: HashMap<u64, as_holder!(FillAreaStyleColour)>,
        fill_area_style_hatching: HashMap<u64, as_holder!(FillAreaStyleHatching)>,
        fill_area_style_tile_symbol_with_style:
            HashMap<u64, as_holder!(FillAreaStyleTileSymbolWithStyle)>,
        fill_area_style_tiles: HashMap<u64, as_holder!(FillAreaStyleTiles)>,
        geometric_curve_set: HashMap<u64, as_holder!(GeometricCurveSet)>,
        geometric_representation_context: HashMap<u64, as_holder!(GeometricRepresentationContext)>,
        geometric_representation_item: HashMap<u64, as_holder!(GeometricRepresentationItem)>,
        geometric_set: HashMap<u64, as_holder!(GeometricSet)>,
        geometrical_tolerance_callout: HashMap<u64, as_holder!(GeometricalToleranceCallout)>,
        geometrically_bounded_2d_wireframe_representation:
            HashMap<u64, as_holder!(GeometricallyBounded2DWireframeRepresentation)>,
        global_unit_assigned_context: HashMap<u64, as_holder!(GlobalUnitAssignedContext)>,
        group: HashMap<u64, as_holder!(Group)>,
        group_assignment: HashMap<u64, as_holder!(GroupAssignment)>,
        group_relationship: HashMap<u64, as_holder!(GroupRelationship)>,
        hyperbola: HashMap<u64, as_holder!(Hyperbola)>,
        invisibility: HashMap<u64, as_holder!(Invisibility)>,
        leader_curve: HashMap<u64, as_holder!(LeaderCurve)>,
        leader_directed_callout: HashMap<u64, as_holder!(LeaderDirectedCallout)>,
        leader_directed_dimension: HashMap<u64, as_holder!(LeaderDirectedDimension)>,
        leader_terminator: HashMap<u64, as_holder!(LeaderTerminator)>,
        length_measure_with_unit: HashMap<u64, as_holder!(LengthMeasureWithUnit)>,
        length_unit: HashMap<u64, as_holder!(LengthUnit)>,
        line: HashMap<u64, as_holder!(Line)>,
        linear_dimension: HashMap<u64, as_holder!(LinearDimension)>,
        mapped_item: HashMap<u64, as_holder!(MappedItem)>,
        measure_with_unit: HashMap<u64, as_holder!(MeasureWithUnit)>,
        named_unit: HashMap<u64, as_holder!(NamedUnit)>,
        offset_curve_2d: HashMap<u64, as_holder!(OffsetCurve2D)>,
        one_direction_repeat_factor: HashMap<u64, as_holder!(OneDirectionRepeatFactor)>,
        ordinate_dimension: HashMap<u64, as_holder!(OrdinateDimension)>,
        organization: HashMap<u64, as_holder!(Organization)>,
        organization_assignment: HashMap<u64, as_holder!(OrganizationAssignment)>,
        organization_role: HashMap<u64, as_holder!(OrganizationRole)>,
        organizational_address: HashMap<u64, as_holder!(OrganizationalAddress)>,
        parabola: HashMap<u64, as_holder!(Parabola)>,
        person: HashMap<u64, as_holder!(Person)>,
        person_and_organization: HashMap<u64, as_holder!(PersonAndOrganization)>,
        person_and_organization_assignment:
            HashMap<u64, as_holder!(PersonAndOrganizationAssignment)>,
        person_and_organization_role: HashMap<u64, as_holder!(PersonAndOrganizationRole)>,
        person_assignment: HashMap<u64, as_holder!(PersonAssignment)>,
        person_role: HashMap<u64, as_holder!(PersonRole)>,
        personal_address: HashMap<u64, as_holder!(PersonalAddress)>,
        placement: HashMap<u64, as_holder!(Placement)>,
        planar_box: HashMap<u64, as_holder!(PlanarBox)>,
        planar_extent: HashMap<u64, as_holder!(PlanarExtent)>,
        plane_angle_measure_with_unit: HashMap<u64, as_holder!(PlaneAngleMeasureWithUnit)>,
        plane_angle_unit: HashMap<u64, as_holder!(PlaneAngleUnit)>,
        point: HashMap<u64, as_holder!(Point)>,
        point_on_curve: HashMap<u64, as_holder!(PointOnCurve)>,
        polyline: HashMap<u64, as_holder!(Polyline)>,
        pre_defined_colour: HashMap<u64, as_holder!(PreDefinedColour)>,
        pre_defined_curve_font: HashMap<u64, as_holder!(PreDefinedCurveFont)>,
        pre_defined_dimension_symbol: HashMap<u64, as_holder!(PreDefinedDimensionSymbol)>,
        pre_defined_geometrical_tolerance_symbol:
            HashMap<u64, as_holder!(PreDefinedGeometricalToleranceSymbol)>,
        pre_defined_item: HashMap<u64, as_holder!(PreDefinedItem)>,
        pre_defined_point_marker_symbol: HashMap<u64, as_holder!(PreDefinedPointMarkerSymbol)>,
        pre_defined_symbol: HashMap<u64, as_holder!(PreDefinedSymbol)>,
        pre_defined_terminator_symbol: HashMap<u64, as_holder!(PreDefinedTerminatorSymbol)>,
        pre_defined_text_font: HashMap<u64, as_holder!(PreDefinedTextFont)>,
        presentation_area: HashMap<u64, as_holder!(PresentationArea)>,
        presentation_layer_assignment: HashMap<u64, as_holder!(PresentationLayerAssignment)>,
        presentation_layer_usage: HashMap<u64, as_holder!(PresentationLayerUsage)>,
        presentation_representation: HashMap<u64, as_holder!(PresentationRepresentation)>,
        presentation_set: HashMap<u64, as_holder!(PresentationSet)>,
        presentation_size: HashMap<u64, as_holder!(PresentationSize)>,
        presentation_style_assignment: HashMap<u64, as_holder!(PresentationStyleAssignment)>,
        presentation_style_by_context: HashMap<u64, as_holder!(PresentationStyleByContext)>,
        presentation_view: HashMap<u64, as_holder!(PresentationView)>,
        presented_item: HashMap<u64, as_holder!(PresentedItem)>,
        presented_item_representation: HashMap<u64, as_holder!(PresentedItemRepresentation)>,
        product: HashMap<u64, as_holder!(Product)>,
        product_context: HashMap<u64, as_holder!(ProductContext)>,
        product_definition: HashMap<u64, as_holder!(ProductDefinition)>,
        product_definition_context: HashMap<u64, as_holder!(ProductDefinitionContext)>,
        product_definition_formation: HashMap<u64, as_holder!(ProductDefinitionFormation)>,
        product_definition_shape: HashMap<u64, as_holder!(ProductDefinitionShape)>,
        projection_curve: HashMap<u64, as_holder!(ProjectionCurve)>,
        projection_directed_callout: HashMap<u64, as_holder!(ProjectionDirectedCallout)>,
        property_definition: HashMap<u64, as_holder!(PropertyDefinition)>,
        property_definition_representation:
            HashMap<u64, as_holder!(PropertyDefinitionRepresentation)>,
        quasi_uniform_curve: HashMap<u64, as_holder!(QuasiUniformCurve)>,
        radius_dimension: HashMap<u64, as_holder!(RadiusDimension)>,
        rational_b_spline_curve: HashMap<u64, as_holder!(RationalBSplineCurve)>,
        representation: HashMap<u64, as_holder!(Representation)>,
        representation_context: HashMap<u64, as_holder!(RepresentationContext)>,
        representation_item: HashMap<u64, as_holder!(RepresentationItem)>,
        representation_map: HashMap<u64, as_holder!(RepresentationMap)>,
        security_classification: HashMap<u64, as_holder!(SecurityClassification)>,
        security_classification_assignment:
            HashMap<u64, as_holder!(SecurityClassificationAssignment)>,
        security_classification_level: HashMap<u64, as_holder!(SecurityClassificationLevel)>,
        shape_definition_representation: HashMap<u64, as_holder!(ShapeDefinitionRepresentation)>,
        shape_representation: HashMap<u64, as_holder!(ShapeRepresentation)>,
        si_unit: HashMap<u64, as_holder!(SiUnit)>,
        structured_dimension_callout: HashMap<u64, as_holder!(StructuredDimensionCallout)>,
        styled_item: HashMap<u64, as_holder!(StyledItem)>,
        symbol_colour: HashMap<u64, as_holder!(SymbolColour)>,
        symbol_representation: HashMap<u64, as_holder!(SymbolRepresentation)>,
        symbol_representation_map: HashMap<u64, as_holder!(SymbolRepresentationMap)>,
        symbol_style: HashMap<u64, as_holder!(SymbolStyle)>,
        symbol_target: HashMap<u64, as_holder!(SymbolTarget)>,
        terminator_symbol: HashMap<u64, as_holder!(TerminatorSymbol)>,
        text_literal: HashMap<u64, as_holder!(TextLiteral)>,
        text_literal_with_associated_curves:
            HashMap<u64, as_holder!(TextLiteralWithAssociatedCurves)>,
        text_literal_with_blanking_box: HashMap<u64, as_holder!(TextLiteralWithBlankingBox)>,
        text_literal_with_delineation: HashMap<u64, as_holder!(TextLiteralWithDelineation)>,
        text_literal_with_extent: HashMap<u64, as_holder!(TextLiteralWithExtent)>,
        text_style: HashMap<u64, as_holder!(TextStyle)>,
        text_style_for_defined_font: HashMap<u64, as_holder!(TextStyleForDefinedFont)>,
        text_style_with_box_characteristics:
            HashMap<u64, as_holder!(TextStyleWithBoxCharacteristics)>,
        text_style_with_mirror: HashMap<u64, as_holder!(TextStyleWithMirror)>,
        trimmed_curve: HashMap<u64, as_holder!(TrimmedCurve)>,
        two_direction_repeat_factor: HashMap<u64, as_holder!(TwoDirectionRepeatFactor)>,
        uniform_curve: HashMap<u64, as_holder!(UniformCurve)>,
        vector: HashMap<u64, as_holder!(Vector)>,
        approved_item: HashMap<u64, as_holder!(ApprovedItem)>,
        area_or_view: HashMap<u64, as_holder!(AreaOrView)>,
        axis2_placement: HashMap<u64, as_holder!(Axis2Placement)>,
        box_characteristic_select: HashMap<u64, as_holder!(BoxCharacteristicSelect)>,
        box_height: HashMap<u64, as_holder!(BoxHeight)>,
        box_rotate_angle: HashMap<u64, as_holder!(BoxRotateAngle)>,
        box_slant_angle: HashMap<u64, as_holder!(BoxSlantAngle)>,
        box_width: HashMap<u64, as_holder!(BoxWidth)>,
        character_spacing_select: HashMap<u64, as_holder!(CharacterSpacingSelect)>,
        character_style_select: HashMap<u64, as_holder!(CharacterStyleSelect)>,
        characterized_definition: HashMap<u64, as_holder!(CharacterizedDefinition)>,
        characterized_product_definition: HashMap<u64, as_holder!(CharacterizedProductDefinition)>,
        classified_item: HashMap<u64, as_holder!(ClassifiedItem)>,
        contracted_item: HashMap<u64, as_holder!(ContractedItem)>,
        curve_font_or_scaled_curve_font_select:
            HashMap<u64, as_holder!(CurveFontOrScaledCurveFontSelect)>,
        curve_or_annotation_curve_occurrence:
            HashMap<u64, as_holder!(CurveOrAnnotationCurveOccurrence)>,
        curve_or_render: HashMap<u64, as_holder!(CurveOrRender)>,
        curve_style_font_select: HashMap<u64, as_holder!(CurveStyleFontSelect)>,
        date_time_select: HashMap<u64, as_holder!(DateTimeSelect)>,
        day_in_month_number: HashMap<u64, as_holder!(DayInMonthNumber)>,
        defined_symbol_select: HashMap<u64, as_holder!(DefinedSymbolSelect)>,
        dimension_count: HashMap<u64, as_holder!(DimensionCount)>,
        draughting_callout_element: HashMap<u64, as_holder!(DraughtingCalloutElement)>,
        draughting_grouped_item: HashMap<u64, as_holder!(DraughtingGroupedItem)>,
        draughting_organization_item: HashMap<u64, as_holder!(DraughtingOrganizationItem)>,
        draughting_presented_item_select: HashMap<u64, as_holder!(DraughtingPresentedItemSelect)>,
        draughting_titled_item: HashMap<u64, as_holder!(DraughtingTitledItem)>,
        fill_area_style_tile_shape_select: HashMap<u64, as_holder!(FillAreaStyleTileShapeSelect)>,
        fill_style_select: HashMap<u64, as_holder!(FillStyleSelect)>,
        font_select: HashMap<u64, as_holder!(FontSelect)>,
        geometric_set_select: HashMap<u64, as_holder!(GeometricSetSelect)>,
        hiding_or_blanking_select: HashMap<u64, as_holder!(HidingOrBlankingSelect)>,
        identifier: HashMap<u64, as_holder!(Identifier)>,
        invisibility_context: HashMap<u64, as_holder!(InvisibilityContext)>,
        invisible_item: HashMap<u64, as_holder!(InvisibleItem)>,
        label: HashMap<u64, as_holder!(Label)>,
        layered_item: HashMap<u64, as_holder!(LayeredItem)>,
        length_measure: HashMap<u64, as_holder!(LengthMeasure)>,
        measure_value: HashMap<u64, as_holder!(MeasureValue)>,
        month_in_year_number: HashMap<u64, as_holder!(MonthInYearNumber)>,
        parameter_value: HashMap<u64, as_holder!(ParameterValue)>,
        person_organization_select: HashMap<u64, as_holder!(PersonOrganizationSelect)>,
        plane_angle_measure: HashMap<u64, as_holder!(PlaneAngleMeasure)>,
        positive_length_measure: HashMap<u64, as_holder!(PositiveLengthMeasure)>,
        positive_ratio_measure: HashMap<u64, as_holder!(PositiveRatioMeasure)>,
        presentable_text: HashMap<u64, as_holder!(PresentableText)>,
        presentation_representation_select:
            HashMap<u64, as_holder!(PresentationRepresentationSelect)>,
        presentation_size_assignment_select:
            HashMap<u64, as_holder!(PresentationSizeAssignmentSelect)>,
        presentation_style_select: HashMap<u64, as_holder!(PresentationStyleSelect)>,
        ratio_measure: HashMap<u64, as_holder!(RatioMeasure)>,
        shape_definition: HashMap<u64, as_holder!(ShapeDefinition)>,
        size_select: HashMap<u64, as_holder!(SizeSelect)>,
        source_item: HashMap<u64, as_holder!(SourceItem)>,
        specified_item: HashMap<u64, as_holder!(SpecifiedItem)>,
        style_context_select: HashMap<u64, as_holder!(StyleContextSelect)>,
        symbol_style_select: HashMap<u64, as_holder!(SymbolStyleSelect)>,
        text: HashMap<u64, as_holder!(Text)>,
        text_alignment: HashMap<u64, as_holder!(TextAlignment)>,
        text_delineation: HashMap<u64, as_holder!(TextDelineation)>,
        text_or_character: HashMap<u64, as_holder!(TextOrCharacter)>,
        trimming_select: HashMap<u64, as_holder!(TrimmingSelect)>,
        unit: HashMap<u64, as_holder!(Unit)>,
        vector_or_direction: HashMap<u64, as_holder!(VectorOrDirection)>,
        year_number: HashMap<u64, as_holder!(YearNumber)>,
    }
    impl Tables {
        pub fn address_holders(&self) -> &HashMap<u64, as_holder!(Address)> {
            &self.address
        }
        pub fn angular_dimension_holders(&self) -> &HashMap<u64, as_holder!(AngularDimension)> {
            &self.angular_dimension
        }
        pub fn annotation_curve_occurrence_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(AnnotationCurveOccurrence)> {
            &self.annotation_curve_occurrence
        }
        pub fn annotation_fill_area_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(AnnotationFillArea)> {
            &self.annotation_fill_area
        }
        pub fn annotation_fill_area_occurrence_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(AnnotationFillAreaOccurrence)> {
            &self.annotation_fill_area_occurrence
        }
        pub fn annotation_occurrence_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(AnnotationOccurrence)> {
            &self.annotation_occurrence
        }
        pub fn annotation_subfigure_occurrence_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(AnnotationSubfigureOccurrence)> {
            &self.annotation_subfigure_occurrence
        }
        pub fn annotation_symbol_holders(&self) -> &HashMap<u64, as_holder!(AnnotationSymbol)> {
            &self.annotation_symbol
        }
        pub fn annotation_symbol_occurrence_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(AnnotationSymbolOccurrence)> {
            &self.annotation_symbol_occurrence
        }
        pub fn annotation_text_holders(&self) -> &HashMap<u64, as_holder!(AnnotationText)> {
            &self.annotation_text
        }
        pub fn annotation_text_occurrence_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(AnnotationTextOccurrence)> {
            &self.annotation_text_occurrence
        }
        pub fn application_context_holders(&self) -> &HashMap<u64, as_holder!(ApplicationContext)> {
            &self.application_context
        }
        pub fn application_context_element_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(ApplicationContextElement)> {
            &self.application_context_element
        }
        pub fn application_protocol_definition_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(ApplicationProtocolDefinition)> {
            &self.application_protocol_definition
        }
        pub fn approval_holders(&self) -> &HashMap<u64, as_holder!(Approval)> {
            &self.approval
        }
        pub fn approval_assignment_holders(&self) -> &HashMap<u64, as_holder!(ApprovalAssignment)> {
            &self.approval_assignment
        }
        pub fn approval_date_time_holders(&self) -> &HashMap<u64, as_holder!(ApprovalDateTime)> {
            &self.approval_date_time
        }
        pub fn approval_person_organization_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(ApprovalPersonOrganization)> {
            &self.approval_person_organization
        }
        pub fn approval_role_holders(&self) -> &HashMap<u64, as_holder!(ApprovalRole)> {
            &self.approval_role
        }
        pub fn approval_status_holders(&self) -> &HashMap<u64, as_holder!(ApprovalStatus)> {
            &self.approval_status
        }
        pub fn area_in_set_holders(&self) -> &HashMap<u64, as_holder!(AreaInSet)> {
            &self.area_in_set
        }
        pub fn axis2_placement_2d_holders(&self) -> &HashMap<u64, as_holder!(Axis2Placement2D)> {
            &self.axis2_placement_2d
        }
        pub fn b_spline_curve_holders(&self) -> &HashMap<u64, as_holder!(BSplineCurve)> {
            &self.b_spline_curve
        }
        pub fn b_spline_curve_with_knots_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(BSplineCurveWithKnots)> {
            &self.b_spline_curve_with_knots
        }
        pub fn bezier_curve_holders(&self) -> &HashMap<u64, as_holder!(BezierCurve)> {
            &self.bezier_curve
        }
        pub fn bounded_curve_holders(&self) -> &HashMap<u64, as_holder!(BoundedCurve)> {
            &self.bounded_curve
        }
        pub fn calendar_date_holders(&self) -> &HashMap<u64, as_holder!(CalendarDate)> {
            &self.calendar_date
        }
        pub fn camera_image_holders(&self) -> &HashMap<u64, as_holder!(CameraImage)> {
            &self.camera_image
        }
        pub fn camera_image_2d_with_scale_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(CameraImage2DWithScale)> {
            &self.camera_image_2d_with_scale
        }
        pub fn camera_model_holders(&self) -> &HashMap<u64, as_holder!(CameraModel)> {
            &self.camera_model
        }
        pub fn camera_model_d2_holders(&self) -> &HashMap<u64, as_holder!(CameraModelD2)> {
            &self.camera_model_d2
        }
        pub fn camera_usage_holders(&self) -> &HashMap<u64, as_holder!(CameraUsage)> {
            &self.camera_usage
        }
        pub fn cartesian_point_holders(&self) -> &HashMap<u64, as_holder!(CartesianPoint)> {
            &self.cartesian_point
        }
        pub fn circle_holders(&self) -> &HashMap<u64, as_holder!(Circle)> {
            &self.circle
        }
        pub fn colour_holders(&self) -> &HashMap<u64, as_holder!(Colour)> {
            &self.colour
        }
        pub fn colour_rgb_holders(&self) -> &HashMap<u64, as_holder!(ColourRgb)> {
            &self.colour_rgb
        }
        pub fn colour_specification_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(ColourSpecification)> {
            &self.colour_specification
        }
        pub fn composite_curve_holders(&self) -> &HashMap<u64, as_holder!(CompositeCurve)> {
            &self.composite_curve
        }
        pub fn composite_curve_segment_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(CompositeCurveSegment)> {
            &self.composite_curve_segment
        }
        pub fn composite_text_holders(&self) -> &HashMap<u64, as_holder!(CompositeText)> {
            &self.composite_text
        }
        pub fn composite_text_with_associated_curves_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(CompositeTextWithAssociatedCurves)> {
            &self.composite_text_with_associated_curves
        }
        pub fn composite_text_with_blanking_box_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(CompositeTextWithBlankingBox)> {
            &self.composite_text_with_blanking_box
        }
        pub fn composite_text_with_extent_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(CompositeTextWithExtent)> {
            &self.composite_text_with_extent
        }
        pub fn conic_holders(&self) -> &HashMap<u64, as_holder!(Conic)> {
            &self.conic
        }
        pub fn context_dependent_invisibility_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(ContextDependentInvisibility)> {
            &self.context_dependent_invisibility
        }
        pub fn contract_holders(&self) -> &HashMap<u64, as_holder!(Contract)> {
            &self.contract
        }
        pub fn contract_assignment_holders(&self) -> &HashMap<u64, as_holder!(ContractAssignment)> {
            &self.contract_assignment
        }
        pub fn contract_type_holders(&self) -> &HashMap<u64, as_holder!(ContractType)> {
            &self.contract_type
        }
        pub fn conversion_based_unit_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(ConversionBasedUnit)> {
            &self.conversion_based_unit
        }
        pub fn curve_holders(&self) -> &HashMap<u64, as_holder!(Curve)> {
            &self.curve
        }
        pub fn curve_dimension_holders(&self) -> &HashMap<u64, as_holder!(CurveDimension)> {
            &self.curve_dimension
        }
        pub fn curve_style_holders(&self) -> &HashMap<u64, as_holder!(CurveStyle)> {
            &self.curve_style
        }
        pub fn curve_style_font_holders(&self) -> &HashMap<u64, as_holder!(CurveStyleFont)> {
            &self.curve_style_font
        }
        pub fn curve_style_font_pattern_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(CurveStyleFontPattern)> {
            &self.curve_style_font_pattern
        }
        pub fn date_holders(&self) -> &HashMap<u64, as_holder!(Date)> {
            &self.date
        }
        pub fn datum_feature_callout_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(DatumFeatureCallout)> {
            &self.datum_feature_callout
        }
        pub fn datum_target_callout_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(DatumTargetCallout)> {
            &self.datum_target_callout
        }
        pub fn defined_symbol_holders(&self) -> &HashMap<u64, as_holder!(DefinedSymbol)> {
            &self.defined_symbol
        }
        pub fn diameter_dimension_holders(&self) -> &HashMap<u64, as_holder!(DiameterDimension)> {
            &self.diameter_dimension
        }
        pub fn dimension_callout_component_relationship_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(DimensionCalloutComponentRelationship)> {
            &self.dimension_callout_component_relationship
        }
        pub fn dimension_callout_relationship_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(DimensionCalloutRelationship)> {
            &self.dimension_callout_relationship
        }
        pub fn dimension_curve_holders(&self) -> &HashMap<u64, as_holder!(DimensionCurve)> {
            &self.dimension_curve
        }
        pub fn dimension_curve_directed_callout_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(DimensionCurveDirectedCallout)> {
            &self.dimension_curve_directed_callout
        }
        pub fn dimension_curve_terminator_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(DimensionCurveTerminator)> {
            &self.dimension_curve_terminator
        }
        pub fn dimension_pair_holders(&self) -> &HashMap<u64, as_holder!(DimensionPair)> {
            &self.dimension_pair
        }
        pub fn dimensional_exponents_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(DimensionalExponents)> {
            &self.dimensional_exponents
        }
        pub fn direction_holders(&self) -> &HashMap<u64, as_holder!(Direction)> {
            &self.direction
        }
        pub fn document_holders(&self) -> &HashMap<u64, as_holder!(Document)> {
            &self.document
        }
        pub fn document_reference_holders(&self) -> &HashMap<u64, as_holder!(DocumentReference)> {
            &self.document_reference
        }
        pub fn document_type_holders(&self) -> &HashMap<u64, as_holder!(DocumentType)> {
            &self.document_type
        }
        pub fn draughting_annotation_occurrence_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(DraughtingAnnotationOccurrence)> {
            &self.draughting_annotation_occurrence
        }
        pub fn draughting_approval_assignment_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(DraughtingApprovalAssignment)> {
            &self.draughting_approval_assignment
        }
        pub fn draughting_callout_holders(&self) -> &HashMap<u64, as_holder!(DraughtingCallout)> {
            &self.draughting_callout
        }
        pub fn draughting_callout_relationship_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(DraughtingCalloutRelationship)> {
            &self.draughting_callout_relationship
        }
        pub fn draughting_contract_assignment_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(DraughtingContractAssignment)> {
            &self.draughting_contract_assignment
        }
        pub fn draughting_drawing_revision_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(DraughtingDrawingRevision)> {
            &self.draughting_drawing_revision
        }
        pub fn draughting_elements_holders(&self) -> &HashMap<u64, as_holder!(DraughtingElements)> {
            &self.draughting_elements
        }
        pub fn draughting_group_assignment_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(DraughtingGroupAssignment)> {
            &self.draughting_group_assignment
        }
        pub fn draughting_model_holders(&self) -> &HashMap<u64, as_holder!(DraughtingModel)> {
            &self.draughting_model
        }
        pub fn draughting_organization_assignment_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(DraughtingOrganizationAssignment)> {
            &self.draughting_organization_assignment
        }
        pub fn draughting_person_and_organization_assignment_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(DraughtingPersonAndOrganizationAssignment)> {
            &self.draughting_person_and_organization_assignment
        }
        pub fn draughting_person_assignment_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(DraughtingPersonAssignment)> {
            &self.draughting_person_assignment
        }
        pub fn draughting_pre_defined_colour_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(DraughtingPreDefinedColour)> {
            &self.draughting_pre_defined_colour
        }
        pub fn draughting_pre_defined_curve_font_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(DraughtingPreDefinedCurveFont)> {
            &self.draughting_pre_defined_curve_font
        }
        pub fn draughting_pre_defined_text_font_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(DraughtingPreDefinedTextFont)> {
            &self.draughting_pre_defined_text_font
        }
        pub fn draughting_presented_item_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(DraughtingPresentedItem)> {
            &self.draughting_presented_item
        }
        pub fn draughting_security_classification_assignment_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(DraughtingSecurityClassificationAssignment)> {
            &self.draughting_security_classification_assignment
        }
        pub fn draughting_specification_reference_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(DraughtingSpecificationReference)> {
            &self.draughting_specification_reference
        }
        pub fn draughting_subfigure_representation_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(DraughtingSubfigureRepresentation)> {
            &self.draughting_subfigure_representation
        }
        pub fn draughting_symbol_representation_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(DraughtingSymbolRepresentation)> {
            &self.draughting_symbol_representation
        }
        pub fn draughting_text_literal_with_delineation_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(DraughtingTextLiteralWithDelineation)> {
            &self.draughting_text_literal_with_delineation
        }
        pub fn draughting_title_holders(&self) -> &HashMap<u64, as_holder!(DraughtingTitle)> {
            &self.draughting_title
        }
        pub fn drawing_definition_holders(&self) -> &HashMap<u64, as_holder!(DrawingDefinition)> {
            &self.drawing_definition
        }
        pub fn drawing_revision_holders(&self) -> &HashMap<u64, as_holder!(DrawingRevision)> {
            &self.drawing_revision
        }
        pub fn drawing_sheet_layout_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(DrawingSheetLayout)> {
            &self.drawing_sheet_layout
        }
        pub fn drawing_sheet_revision_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(DrawingSheetRevision)> {
            &self.drawing_sheet_revision
        }
        pub fn drawing_sheet_revision_usage_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(DrawingSheetRevisionUsage)> {
            &self.drawing_sheet_revision_usage
        }
        pub fn ellipse_holders(&self) -> &HashMap<u64, as_holder!(Ellipse)> {
            &self.ellipse
        }
        pub fn external_source_holders(&self) -> &HashMap<u64, as_holder!(ExternalSource)> {
            &self.external_source
        }
        pub fn externally_defined_curve_font_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(ExternallyDefinedCurveFont)> {
            &self.externally_defined_curve_font
        }
        pub fn externally_defined_hatch_style_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(ExternallyDefinedHatchStyle)> {
            &self.externally_defined_hatch_style
        }
        pub fn externally_defined_item_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(ExternallyDefinedItem)> {
            &self.externally_defined_item
        }
        pub fn externally_defined_symbol_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(ExternallyDefinedSymbol)> {
            &self.externally_defined_symbol
        }
        pub fn externally_defined_text_font_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(ExternallyDefinedTextFont)> {
            &self.externally_defined_text_font
        }
        pub fn externally_defined_tile_style_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(ExternallyDefinedTileStyle)> {
            &self.externally_defined_tile_style
        }
        pub fn fill_area_style_holders(&self) -> &HashMap<u64, as_holder!(FillAreaStyle)> {
            &self.fill_area_style
        }
        pub fn fill_area_style_colour_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(FillAreaStyleColour)> {
            &self.fill_area_style_colour
        }
        pub fn fill_area_style_hatching_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(FillAreaStyleHatching)> {
            &self.fill_area_style_hatching
        }
        pub fn fill_area_style_tile_symbol_with_style_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(FillAreaStyleTileSymbolWithStyle)> {
            &self.fill_area_style_tile_symbol_with_style
        }
        pub fn fill_area_style_tiles_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(FillAreaStyleTiles)> {
            &self.fill_area_style_tiles
        }
        pub fn geometric_curve_set_holders(&self) -> &HashMap<u64, as_holder!(GeometricCurveSet)> {
            &self.geometric_curve_set
        }
        pub fn geometric_representation_context_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(GeometricRepresentationContext)> {
            &self.geometric_representation_context
        }
        pub fn geometric_representation_item_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(GeometricRepresentationItem)> {
            &self.geometric_representation_item
        }
        pub fn geometric_set_holders(&self) -> &HashMap<u64, as_holder!(GeometricSet)> {
            &self.geometric_set
        }
        pub fn geometrical_tolerance_callout_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(GeometricalToleranceCallout)> {
            &self.geometrical_tolerance_callout
        }
        pub fn geometrically_bounded_2d_wireframe_representation_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(GeometricallyBounded2DWireframeRepresentation)> {
            &self.geometrically_bounded_2d_wireframe_representation
        }
        pub fn global_unit_assigned_context_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(GlobalUnitAssignedContext)> {
            &self.global_unit_assigned_context
        }
        pub fn group_holders(&self) -> &HashMap<u64, as_holder!(Group)> {
            &self.group
        }
        pub fn group_assignment_holders(&self) -> &HashMap<u64, as_holder!(GroupAssignment)> {
            &self.group_assignment
        }
        pub fn group_relationship_holders(&self) -> &HashMap<u64, as_holder!(GroupRelationship)> {
            &self.group_relationship
        }
        pub fn hyperbola_holders(&self) -> &HashMap<u64, as_holder!(Hyperbola)> {
            &self.hyperbola
        }
        pub fn invisibility_holders(&self) -> &HashMap<u64, as_holder!(Invisibility)> {
            &self.invisibility
        }
        pub fn leader_curve_holders(&self) -> &HashMap<u64, as_holder!(LeaderCurve)> {
            &self.leader_curve
        }
        pub fn leader_directed_callout_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(LeaderDirectedCallout)> {
            &self.leader_directed_callout
        }
        pub fn leader_directed_dimension_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(LeaderDirectedDimension)> {
            &self.leader_directed_dimension
        }
        pub fn leader_terminator_holders(&self) -> &HashMap<u64, as_holder!(LeaderTerminator)> {
            &self.leader_terminator
        }
        pub fn length_measure_with_unit_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(LengthMeasureWithUnit)> {
            &self.length_measure_with_unit
        }
        pub fn length_unit_holders(&self) -> &HashMap<u64, as_holder!(LengthUnit)> {
            &self.length_unit
        }
        pub fn line_holders(&self) -> &HashMap<u64, as_holder!(Line)> {
            &self.line
        }
        pub fn linear_dimension_holders(&self) -> &HashMap<u64, as_holder!(LinearDimension)> {
            &self.linear_dimension
        }
        pub fn mapped_item_holders(&self) -> &HashMap<u64, as_holder!(MappedItem)> {
            &self.mapped_item
        }
        pub fn measure_with_unit_holders(&self) -> &HashMap<u64, as_holder!(MeasureWithUnit)> {
            &self.measure_with_unit
        }
        pub fn named_unit_holders(&self) -> &HashMap<u64, as_holder!(NamedUnit)> {
            &self.named_unit
        }
        pub fn offset_curve_2d_holders(&self) -> &HashMap<u64, as_holder!(OffsetCurve2D)> {
            &self.offset_curve_2d
        }
        pub fn one_direction_repeat_factor_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(OneDirectionRepeatFactor)> {
            &self.one_direction_repeat_factor
        }
        pub fn ordinate_dimension_holders(&self) -> &HashMap<u64, as_holder!(OrdinateDimension)> {
            &self.ordinate_dimension
        }
        pub fn organization_holders(&self) -> &HashMap<u64, as_holder!(Organization)> {
            &self.organization
        }
        pub fn organization_assignment_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(OrganizationAssignment)> {
            &self.organization_assignment
        }
        pub fn organization_role_holders(&self) -> &HashMap<u64, as_holder!(OrganizationRole)> {
            &self.organization_role
        }
        pub fn organizational_address_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(OrganizationalAddress)> {
            &self.organizational_address
        }
        pub fn parabola_holders(&self) -> &HashMap<u64, as_holder!(Parabola)> {
            &self.parabola
        }
        pub fn person_holders(&self) -> &HashMap<u64, as_holder!(Person)> {
            &self.person
        }
        pub fn person_and_organization_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(PersonAndOrganization)> {
            &self.person_and_organization
        }
        pub fn person_and_organization_assignment_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(PersonAndOrganizationAssignment)> {
            &self.person_and_organization_assignment
        }
        pub fn person_and_organization_role_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(PersonAndOrganizationRole)> {
            &self.person_and_organization_role
        }
        pub fn person_assignment_holders(&self) -> &HashMap<u64, as_holder!(PersonAssignment)> {
            &self.person_assignment
        }
        pub fn person_role_holders(&self) -> &HashMap<u64, as_holder!(PersonRole)> {
            &self.person_role
        }
        pub fn personal_address_holders(&self) -> &HashMap<u64, as_holder!(PersonalAddress)> {
            &self.personal_address
        }
        pub fn placement_holders(&self) -> &HashMap<u64, as_holder!(Placement)> {
            &self.placement
        }
        pub fn planar_box_holders(&self) -> &HashMap<u64, as_holder!(PlanarBox)> {
            &self.planar_box
        }
        pub fn planar_extent_holders(&self) -> &HashMap<u64, as_holder!(PlanarExtent)> {
            &self.planar_extent
        }
        pub fn plane_angle_measure_with_unit_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(PlaneAngleMeasureWithUnit)> {
            &self.plane_angle_measure_with_unit
        }
        pub fn plane_angle_unit_holders(&self) -> &HashMap<u64, as_holder!(PlaneAngleUnit)> {
            &self.plane_angle_unit
        }
        pub fn point_holders(&self) -> &HashMap<u64, as_holder!(Point)> {
            &self.point
        }
        pub fn point_on_curve_holders(&self) -> &HashMap<u64, as_holder!(PointOnCurve)> {
            &self.point_on_curve
        }
        pub fn polyline_holders(&self) -> &HashMap<u64, as_holder!(Polyline)> {
            &self.polyline
        }
        pub fn pre_defined_colour_holders(&self) -> &HashMap<u64, as_holder!(PreDefinedColour)> {
            &self.pre_defined_colour
        }
        pub fn pre_defined_curve_font_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(PreDefinedCurveFont)> {
            &self.pre_defined_curve_font
        }
        pub fn pre_defined_dimension_symbol_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(PreDefinedDimensionSymbol)> {
            &self.pre_defined_dimension_symbol
        }
        pub fn pre_defined_geometrical_tolerance_symbol_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(PreDefinedGeometricalToleranceSymbol)> {
            &self.pre_defined_geometrical_tolerance_symbol
        }
        pub fn pre_defined_item_holders(&self) -> &HashMap<u64, as_holder!(PreDefinedItem)> {
            &self.pre_defined_item
        }
        pub fn pre_defined_point_marker_symbol_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(PreDefinedPointMarkerSymbol)> {
            &self.pre_defined_point_marker_symbol
        }
        pub fn pre_defined_symbol_holders(&self) -> &HashMap<u64, as_holder!(PreDefinedSymbol)> {
            &self.pre_defined_symbol
        }
        pub fn pre_defined_terminator_symbol_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(PreDefinedTerminatorSymbol)> {
            &self.pre_defined_terminator_symbol
        }
        pub fn pre_defined_text_font_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(PreDefinedTextFont)> {
            &self.pre_defined_text_font
        }
        pub fn presentation_area_holders(&self) -> &HashMap<u64, as_holder!(PresentationArea)> {
            &self.presentation_area
        }
        pub fn presentation_layer_assignment_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(PresentationLayerAssignment)> {
            &self.presentation_layer_assignment
        }
        pub fn presentation_layer_usage_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(PresentationLayerUsage)> {
            &self.presentation_layer_usage
        }
        pub fn presentation_representation_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(PresentationRepresentation)> {
            &self.presentation_representation
        }
        pub fn presentation_set_holders(&self) -> &HashMap<u64, as_holder!(PresentationSet)> {
            &self.presentation_set
        }
        pub fn presentation_size_holders(&self) -> &HashMap<u64, as_holder!(PresentationSize)> {
            &self.presentation_size
        }
        pub fn presentation_style_assignment_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(PresentationStyleAssignment)> {
            &self.presentation_style_assignment
        }
        pub fn presentation_style_by_context_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(PresentationStyleByContext)> {
            &self.presentation_style_by_context
        }
        pub fn presentation_view_holders(&self) -> &HashMap<u64, as_holder!(PresentationView)> {
            &self.presentation_view
        }
        pub fn presented_item_holders(&self) -> &HashMap<u64, as_holder!(PresentedItem)> {
            &self.presented_item
        }
        pub fn presented_item_representation_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(PresentedItemRepresentation)> {
            &self.presented_item_representation
        }
        pub fn product_holders(&self) -> &HashMap<u64, as_holder!(Product)> {
            &self.product
        }
        pub fn product_context_holders(&self) -> &HashMap<u64, as_holder!(ProductContext)> {
            &self.product_context
        }
        pub fn product_definition_holders(&self) -> &HashMap<u64, as_holder!(ProductDefinition)> {
            &self.product_definition
        }
        pub fn product_definition_context_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(ProductDefinitionContext)> {
            &self.product_definition_context
        }
        pub fn product_definition_formation_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(ProductDefinitionFormation)> {
            &self.product_definition_formation
        }
        pub fn product_definition_shape_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(ProductDefinitionShape)> {
            &self.product_definition_shape
        }
        pub fn projection_curve_holders(&self) -> &HashMap<u64, as_holder!(ProjectionCurve)> {
            &self.projection_curve
        }
        pub fn projection_directed_callout_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(ProjectionDirectedCallout)> {
            &self.projection_directed_callout
        }
        pub fn property_definition_holders(&self) -> &HashMap<u64, as_holder!(PropertyDefinition)> {
            &self.property_definition
        }
        pub fn property_definition_representation_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(PropertyDefinitionRepresentation)> {
            &self.property_definition_representation
        }
        pub fn quasi_uniform_curve_holders(&self) -> &HashMap<u64, as_holder!(QuasiUniformCurve)> {
            &self.quasi_uniform_curve
        }
        pub fn radius_dimension_holders(&self) -> &HashMap<u64, as_holder!(RadiusDimension)> {
            &self.radius_dimension
        }
        pub fn rational_b_spline_curve_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(RationalBSplineCurve)> {
            &self.rational_b_spline_curve
        }
        pub fn representation_holders(&self) -> &HashMap<u64, as_holder!(Representation)> {
            &self.representation
        }
        pub fn representation_context_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(RepresentationContext)> {
            &self.representation_context
        }
        pub fn representation_item_holders(&self) -> &HashMap<u64, as_holder!(RepresentationItem)> {
            &self.representation_item
        }
        pub fn representation_map_holders(&self) -> &HashMap<u64, as_holder!(RepresentationMap)> {
            &self.representation_map
        }
        pub fn security_classification_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(SecurityClassification)> {
            &self.security_classification
        }
        pub fn security_classification_assignment_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(SecurityClassificationAssignment)> {
            &self.security_classification_assignment
        }
        pub fn security_classification_level_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(SecurityClassificationLevel)> {
            &self.security_classification_level
        }
        pub fn shape_definition_representation_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(ShapeDefinitionRepresentation)> {
            &self.shape_definition_representation
        }
        pub fn shape_representation_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(ShapeRepresentation)> {
            &self.shape_representation
        }
        pub fn si_unit_holders(&self) -> &HashMap<u64, as_holder!(SiUnit)> {
            &self.si_unit
        }
        pub fn structured_dimension_callout_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(StructuredDimensionCallout)> {
            &self.structured_dimension_callout
        }
        pub fn styled_item_holders(&self) -> &HashMap<u64, as_holder!(StyledItem)> {
            &self.styled_item
        }
        pub fn symbol_colour_holders(&self) -> &HashMap<u64, as_holder!(SymbolColour)> {
            &self.symbol_colour
        }
        pub fn symbol_representation_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(SymbolRepresentation)> {
            &self.symbol_representation
        }
        pub fn symbol_representation_map_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(SymbolRepresentationMap)> {
            &self.symbol_representation_map
        }
        pub fn symbol_style_holders(&self) -> &HashMap<u64, as_holder!(SymbolStyle)> {
            &self.symbol_style
        }
        pub fn symbol_target_holders(&self) -> &HashMap<u64, as_holder!(SymbolTarget)> {
            &self.symbol_target
        }
        pub fn terminator_symbol_holders(&self) -> &HashMap<u64, as_holder!(TerminatorSymbol)> {
            &self.terminator_symbol
        }
        pub fn text_literal_holders(&self) -> &HashMap<u64, as_holder!(TextLiteral)> {
            &self.text_literal
        }
        pub fn text_literal_with_associated_curves_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(TextLiteralWithAssociatedCurves)> {
            &self.text_literal_with_associated_curves
        }
        pub fn text_literal_with_blanking_box_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(TextLiteralWithBlankingBox)> {
            &self.text_literal_with_blanking_box
        }
        pub fn text_literal_with_delineation_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(TextLiteralWithDelineation)> {
            &self.text_literal_with_delineation
        }
        pub fn text_literal_with_extent_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(TextLiteralWithExtent)> {
            &self.text_literal_with_extent
        }
        pub fn text_style_holders(&self) -> &HashMap<u64, as_holder!(TextStyle)> {
            &self.text_style
        }
        pub fn text_style_for_defined_font_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(TextStyleForDefinedFont)> {
            &self.text_style_for_defined_font
        }
        pub fn text_style_with_box_characteristics_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(TextStyleWithBoxCharacteristics)> {
            &self.text_style_with_box_characteristics
        }
        pub fn text_style_with_mirror_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(TextStyleWithMirror)> {
            &self.text_style_with_mirror
        }
        pub fn trimmed_curve_holders(&self) -> &HashMap<u64, as_holder!(TrimmedCurve)> {
            &self.trimmed_curve
        }
        pub fn two_direction_repeat_factor_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(TwoDirectionRepeatFactor)> {
            &self.two_direction_repeat_factor
        }
        pub fn uniform_curve_holders(&self) -> &HashMap<u64, as_holder!(UniformCurve)> {
            &self.uniform_curve
        }
        pub fn vector_holders(&self) -> &HashMap<u64, as_holder!(Vector)> {
            &self.vector
        }
        pub fn approved_item_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ApprovedItem>> + 'table {
            self.approved_item
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn area_or_view_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<AreaOrView>> + 'table {
            self.area_or_view
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn axis2_placement_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<Axis2Placement>> + 'table {
            self.axis2_placement
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn box_characteristic_select_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<BoxCharacteristicSelect>> + 'table {
            self.box_characteristic_select
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn box_height_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<BoxHeight>> + 'table {
            self.box_height
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn box_rotate_angle_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<BoxRotateAngle>> + 'table {
            self.box_rotate_angle
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn box_slant_angle_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<BoxSlantAngle>> + 'table {
            self.box_slant_angle
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn box_width_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<BoxWidth>> + 'table {
            self.box_width
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn character_spacing_select_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<CharacterSpacingSelect>> + 'table {
            self.character_spacing_select
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn character_style_select_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<CharacterStyleSelect>> + 'table {
            self.character_style_select
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn characterized_definition_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<CharacterizedDefinition>> + 'table {
            self.characterized_definition
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn characterized_product_definition_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<CharacterizedProductDefinition>> + 'table {
            self.characterized_product_definition
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn classified_item_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ClassifiedItem>> + 'table {
            self.classified_item
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn contracted_item_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ContractedItem>> + 'table {
            self.contracted_item
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn curve_font_or_scaled_curve_font_select_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<CurveFontOrScaledCurveFontSelect>> + 'table {
            self.curve_font_or_scaled_curve_font_select
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn curve_or_annotation_curve_occurrence_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<CurveOrAnnotationCurveOccurrence>> + 'table {
            self.curve_or_annotation_curve_occurrence
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn curve_or_render_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<CurveOrRender>> + 'table {
            self.curve_or_render
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn curve_style_font_select_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<CurveStyleFontSelect>> + 'table {
            self.curve_style_font_select
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn date_time_select_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<DateTimeSelect>> + 'table {
            self.date_time_select
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn day_in_month_number_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<DayInMonthNumber>> + 'table {
            self.day_in_month_number
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn defined_symbol_select_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<DefinedSymbolSelect>> + 'table {
            self.defined_symbol_select
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn dimension_count_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<DimensionCount>> + 'table {
            self.dimension_count
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn draughting_callout_element_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<DraughtingCalloutElement>> + 'table {
            self.draughting_callout_element
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn draughting_grouped_item_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<DraughtingGroupedItem>> + 'table {
            self.draughting_grouped_item
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn draughting_organization_item_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<DraughtingOrganizationItem>> + 'table {
            self.draughting_organization_item
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn draughting_presented_item_select_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<DraughtingPresentedItemSelect>> + 'table {
            self.draughting_presented_item_select
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn draughting_titled_item_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<DraughtingTitledItem>> + 'table {
            self.draughting_titled_item
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn fill_area_style_tile_shape_select_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<FillAreaStyleTileShapeSelect>> + 'table {
            self.fill_area_style_tile_shape_select
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn fill_style_select_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<FillStyleSelect>> + 'table {
            self.fill_style_select
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn font_select_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<FontSelect>> + 'table {
            self.font_select
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn geometric_set_select_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<GeometricSetSelect>> + 'table {
            self.geometric_set_select
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn hiding_or_blanking_select_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<HidingOrBlankingSelect>> + 'table {
            self.hiding_or_blanking_select
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn identifier_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<Identifier>> + 'table {
            self.identifier
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn invisibility_context_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<InvisibilityContext>> + 'table {
            self.invisibility_context
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn invisible_item_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<InvisibleItem>> + 'table {
            self.invisible_item
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn label_iter<'table>(&'table self) -> impl Iterator<Item = Result<Label>> + 'table {
            self.label
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn layered_item_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<LayeredItem>> + 'table {
            self.layered_item
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn length_measure_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<LengthMeasure>> + 'table {
            self.length_measure
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn measure_value_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<MeasureValue>> + 'table {
            self.measure_value
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn month_in_year_number_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<MonthInYearNumber>> + 'table {
            self.month_in_year_number
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn parameter_value_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ParameterValue>> + 'table {
            self.parameter_value
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn person_organization_select_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<PersonOrganizationSelect>> + 'table {
            self.person_organization_select
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn plane_angle_measure_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<PlaneAngleMeasure>> + 'table {
            self.plane_angle_measure
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn positive_length_measure_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<PositiveLengthMeasure>> + 'table {
            self.positive_length_measure
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn positive_ratio_measure_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<PositiveRatioMeasure>> + 'table {
            self.positive_ratio_measure
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn presentable_text_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<PresentableText>> + 'table {
            self.presentable_text
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn presentation_representation_select_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<PresentationRepresentationSelect>> + 'table {
            self.presentation_representation_select
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn presentation_size_assignment_select_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<PresentationSizeAssignmentSelect>> + 'table {
            self.presentation_size_assignment_select
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn presentation_style_select_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<PresentationStyleSelect>> + 'table {
            self.presentation_style_select
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn ratio_measure_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<RatioMeasure>> + 'table {
            self.ratio_measure
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn shape_definition_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ShapeDefinition>> + 'table {
            self.shape_definition
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn size_select_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<SizeSelect>> + 'table {
            self.size_select
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn source_item_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<SourceItem>> + 'table {
            self.source_item
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn specified_item_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<SpecifiedItem>> + 'table {
            self.specified_item
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn style_context_select_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<StyleContextSelect>> + 'table {
            self.style_context_select
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn symbol_style_select_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<SymbolStyleSelect>> + 'table {
            self.symbol_style_select
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn text_iter<'table>(&'table self) -> impl Iterator<Item = Result<Text>> + 'table {
            self.text
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn text_alignment_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<TextAlignment>> + 'table {
            self.text_alignment
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn text_delineation_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<TextDelineation>> + 'table {
            self.text_delineation
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn text_or_character_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<TextOrCharacter>> + 'table {
            self.text_or_character
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn trimming_select_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<TrimmingSelect>> + 'table {
            self.trimming_select
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn unit_iter<'table>(&'table self) -> impl Iterator<Item = Result<Unit>> + 'table {
            self.unit
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn vector_or_direction_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<VectorOrDirection>> + 'table {
            self.vector_or_direction
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn year_number_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<YearNumber>> + 'table {
            self.year_number
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ApprovedItem {
        #[holder(use_place_holder)]
        DrawingRevision(DrawingRevisionAny),
        # [holder (field = drawing_sheet_revision)]
        #[holder(use_place_holder)]
        DrawingSheetRevision(Box<DrawingSheetRevision>),
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum AreaOrView {
        #[holder(use_place_holder)]
        PresentationArea(PresentationAreaAny),
        # [holder (field = presentation_view)]
        #[holder(use_place_holder)]
        PresentationView(Box<PresentationView>),
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum Axis2Placement {
        # [holder (field = axis2_placement_2d)]
        #[holder(use_place_holder)]
        Axis2Placement2D(Box<Axis2Placement2D>),
    }
    #[derive(Debug, Clone, PartialEq, :: serde :: Deserialize)]
    pub enum BSplineCurveForm {
        EllipticArc,
        PolylineForm,
        ParabolicArc,
        CircularArc,
        Unspecified,
        HyperbolicArc,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum BoxCharacteristicSelect {
        #[holder(use_place_holder)]
        BoxHeight(Box<BoxHeight>),
        #[holder(use_place_holder)]
        BoxWidth(Box<BoxWidth>),
        #[holder(use_place_holder)]
        BoxSlantAngle(Box<BoxSlantAngle>),
        #[holder(use_place_holder)]
        BoxRotateAngle(Box<BoxRotateAngle>),
    }
    #[derive(
        Clone, Debug, PartialEq, AsRef, Deref, DerefMut, Into, From, :: ruststep_derive :: Holder,
    )]
    # [holder (table = Tables)]
    # [holder (field = box_height)]
    #[holder(generate_deserialize)]
    pub struct BoxHeight(#[holder(use_place_holder)] pub PositiveRatioMeasure);
    #[derive(
        Clone, Debug, PartialEq, AsRef, Deref, DerefMut, Into, From, :: ruststep_derive :: Holder,
    )]
    # [holder (table = Tables)]
    # [holder (field = box_rotate_angle)]
    #[holder(generate_deserialize)]
    pub struct BoxRotateAngle(#[holder(use_place_holder)] pub PlaneAngleMeasure);
    #[derive(
        Clone, Debug, PartialEq, AsRef, Deref, DerefMut, Into, From, :: ruststep_derive :: Holder,
    )]
    # [holder (table = Tables)]
    # [holder (field = box_slant_angle)]
    #[holder(generate_deserialize)]
    pub struct BoxSlantAngle(#[holder(use_place_holder)] pub PlaneAngleMeasure);
    #[derive(
        Clone, Debug, PartialEq, AsRef, Deref, DerefMut, Into, From, :: ruststep_derive :: Holder,
    )]
    # [holder (table = Tables)]
    # [holder (field = box_width)]
    #[holder(generate_deserialize)]
    pub struct BoxWidth(#[holder(use_place_holder)] pub PositiveRatioMeasure);
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum CharacterSpacingSelect {
        #[holder(use_place_holder)]
        LengthMeasure(Box<LengthMeasure>),
        #[holder(use_place_holder)]
        RatioMeasure(Box<RatioMeasure>),
        #[holder(use_place_holder)]
        MeasureWithUnit(MeasureWithUnitAny),
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum CharacterStyleSelect {
        # [holder (field = text_style_for_defined_font)]
        #[holder(use_place_holder)]
        TextStyleForDefinedFont(Box<TextStyleForDefinedFont>),
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum CharacterizedDefinition {
        #[holder(use_place_holder)]
        CharacterizedProductDefinition(Box<CharacterizedProductDefinition>),
        #[holder(use_place_holder)]
        ShapeDefinition(Box<ShapeDefinition>),
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum CharacterizedProductDefinition {
        # [holder (field = product_definition)]
        #[holder(use_place_holder)]
        ProductDefinition(Box<ProductDefinition>),
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ClassifiedItem {
        #[holder(use_place_holder)]
        DrawingRevision(DrawingRevisionAny),
        # [holder (field = drawing_sheet_revision)]
        #[holder(use_place_holder)]
        DrawingSheetRevision(Box<DrawingSheetRevision>),
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ContractedItem {
        #[holder(use_place_holder)]
        DrawingRevision(DrawingRevisionAny),
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum CurveFontOrScaledCurveFontSelect {
        #[holder(use_place_holder)]
        CurveStyleFontSelect(Box<CurveStyleFontSelect>),
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum CurveOrAnnotationCurveOccurrence {
        #[holder(use_place_holder)]
        Curve(CurveAny),
        #[holder(use_place_holder)]
        AnnotationCurveOccurrence(AnnotationCurveOccurrenceAny),
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum CurveOrRender {
        # [holder (field = curve_style)]
        #[holder(use_place_holder)]
        CurveStyle(Box<CurveStyle>),
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum CurveStyleFontSelect {
        # [holder (field = curve_style_font)]
        #[holder(use_place_holder)]
        CurveStyleFont(Box<CurveStyleFont>),
        #[holder(use_place_holder)]
        PreDefinedCurveFont(PreDefinedCurveFontAny),
        # [holder (field = externally_defined_curve_font)]
        #[holder(use_place_holder)]
        ExternallyDefinedCurveFont(Box<ExternallyDefinedCurveFont>),
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum DateTimeSelect {
        #[holder(use_place_holder)]
        Date(DateAny),
    }
    #[derive(
        Clone, Debug, PartialEq, AsRef, Deref, DerefMut, Into, From, :: ruststep_derive :: Holder,
    )]
    # [holder (table = Tables)]
    # [holder (field = day_in_month_number)]
    #[holder(generate_deserialize)]
    pub struct DayInMonthNumber(pub i64);
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum DefinedSymbolSelect {
        #[holder(use_place_holder)]
        PreDefinedSymbol(PreDefinedSymbolAny),
        # [holder (field = externally_defined_symbol)]
        #[holder(use_place_holder)]
        ExternallyDefinedSymbol(Box<ExternallyDefinedSymbol>),
    }
    #[derive(
        Clone, Debug, PartialEq, AsRef, Deref, DerefMut, Into, From, :: ruststep_derive :: Holder,
    )]
    # [holder (table = Tables)]
    # [holder (field = dimension_count)]
    #[holder(generate_deserialize)]
    pub struct DimensionCount(pub i64);
    #[derive(Debug, Clone, PartialEq, :: serde :: Deserialize)]
    pub enum DimensionExtentUsage {
        Origin,
        Target,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum DraughtingCalloutElement {
        # [holder (field = annotation_text_occurrence)]
        #[holder(use_place_holder)]
        AnnotationTextOccurrence(Box<AnnotationTextOccurrence>),
        #[holder(use_place_holder)]
        AnnotationSymbolOccurrence(AnnotationSymbolOccurrenceAny),
        #[holder(use_place_holder)]
        AnnotationCurveOccurrence(AnnotationCurveOccurrenceAny),
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum DraughtingGroupedItem {
        #[holder(use_place_holder)]
        AnnotationOccurrence(AnnotationOccurrenceAny),
        #[holder(use_place_holder)]
        GeometricSetSelect(Box<GeometricSetSelect>),
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum DraughtingOrganizationItem {
        # [holder (field = product_definition_formation)]
        #[holder(use_place_holder)]
        ProductDefinitionFormation(Box<ProductDefinitionFormation>),
        #[holder(use_place_holder)]
        DrawingRevision(DrawingRevisionAny),
        # [holder (field = drawing_sheet_revision)]
        #[holder(use_place_holder)]
        DrawingSheetRevision(Box<DrawingSheetRevision>),
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum DraughtingPresentedItemSelect {
        # [holder (field = product_definition_formation)]
        #[holder(use_place_holder)]
        ProductDefinitionFormation(Box<ProductDefinitionFormation>),
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum DraughtingTitledItem {
        #[holder(use_place_holder)]
        DrawingRevision(DrawingRevisionAny),
        # [holder (field = drawing_sheet_revision)]
        #[holder(use_place_holder)]
        DrawingSheetRevision(Box<DrawingSheetRevision>),
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum FillAreaStyleTileShapeSelect {
        # [holder (field = fill_area_style_tile_symbol_with_style)]
        #[holder(use_place_holder)]
        FillAreaStyleTileSymbolWithStyle(Box<FillAreaStyleTileSymbolWithStyle>),
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum FillStyleSelect {
        # [holder (field = fill_area_style_colour)]
        #[holder(use_place_holder)]
        FillAreaStyleColour(Box<FillAreaStyleColour>),
        # [holder (field = externally_defined_tile_style)]
        #[holder(use_place_holder)]
        ExternallyDefinedTileStyle(Box<ExternallyDefinedTileStyle>),
        # [holder (field = fill_area_style_tiles)]
        #[holder(use_place_holder)]
        FillAreaStyleTiles(Box<FillAreaStyleTiles>),
        # [holder (field = externally_defined_hatch_style)]
        #[holder(use_place_holder)]
        ExternallyDefinedHatchStyle(Box<ExternallyDefinedHatchStyle>),
        # [holder (field = fill_area_style_hatching)]
        #[holder(use_place_holder)]
        FillAreaStyleHatching(Box<FillAreaStyleHatching>),
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum FontSelect {
        #[holder(use_place_holder)]
        PreDefinedTextFont(PreDefinedTextFontAny),
        # [holder (field = externally_defined_text_font)]
        #[holder(use_place_holder)]
        ExternallyDefinedTextFont(Box<ExternallyDefinedTextFont>),
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum GeometricSetSelect {
        #[holder(use_place_holder)]
        Point(PointAny),
        #[holder(use_place_holder)]
        Curve(CurveAny),
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum HidingOrBlankingSelect {
        #[holder(use_place_holder)]
        PresentationArea(PresentationAreaAny),
        # [holder (field = presentation_view)]
        #[holder(use_place_holder)]
        PresentationView(Box<PresentationView>),
        # [holder (field = annotation_fill_area)]
        #[holder(use_place_holder)]
        AnnotationFillArea(Box<AnnotationFillArea>),
    }
    #[derive(
        Clone, Debug, PartialEq, AsRef, Deref, DerefMut, Into, From, :: ruststep_derive :: Holder,
    )]
    # [holder (table = Tables)]
    # [holder (field = identifier)]
    #[holder(generate_deserialize)]
    pub struct Identifier(pub String);
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum InvisibilityContext {
        # [holder (field = presentation_layer_usage)]
        #[holder(use_place_holder)]
        PresentationLayerUsage(Box<PresentationLayerUsage>),
        #[holder(use_place_holder)]
        PresentationRepresentation(PresentationRepresentationAny),
        #[holder(use_place_holder)]
        PresentationSet(PresentationSetAny),
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum InvisibleItem {
        #[holder(use_place_holder)]
        StyledItem(StyledItemAny),
        # [holder (field = presentation_layer_assignment)]
        #[holder(use_place_holder)]
        PresentationLayerAssignment(Box<PresentationLayerAssignment>),
        #[holder(use_place_holder)]
        PresentationRepresentation(PresentationRepresentationAny),
    }
    #[derive(Debug, Clone, PartialEq, :: serde :: Deserialize)]
    pub enum KnotType {
        UniformKnots,
        QuasiUniformKnots,
        PiecewiseBezierKnots,
        Unspecified,
    }
    #[derive(
        Clone, Debug, PartialEq, AsRef, Deref, DerefMut, Into, From, :: ruststep_derive :: Holder,
    )]
    # [holder (table = Tables)]
    # [holder (field = label)]
    #[holder(generate_deserialize)]
    pub struct Label(pub String);
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum LayeredItem {
        #[holder(use_place_holder)]
        PresentationRepresentation(PresentationRepresentationAny),
        #[holder(use_place_holder)]
        RepresentationItem(RepresentationItemAny),
    }
    #[derive(
        Clone, Debug, PartialEq, AsRef, Deref, DerefMut, Into, From, :: ruststep_derive :: Holder,
    )]
    # [holder (table = Tables)]
    # [holder (field = length_measure)]
    #[holder(generate_deserialize)]
    pub struct LengthMeasure(pub f64);
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum MeasureValue {
        #[holder(use_place_holder)]
        LengthMeasure(Box<LengthMeasure>),
        #[holder(use_place_holder)]
        PlaneAngleMeasure(Box<PlaneAngleMeasure>),
        #[holder(use_place_holder)]
        RatioMeasure(Box<RatioMeasure>),
        #[holder(use_place_holder)]
        ParameterValue(Box<ParameterValue>),
        #[holder(use_place_holder)]
        PositiveLengthMeasure(Box<PositiveLengthMeasure>),
        #[holder(use_place_holder)]
        PositiveRatioMeasure(Box<PositiveRatioMeasure>),
    }
    #[derive(
        Clone, Debug, PartialEq, AsRef, Deref, DerefMut, Into, From, :: ruststep_derive :: Holder,
    )]
    # [holder (table = Tables)]
    # [holder (field = month_in_year_number)]
    #[holder(generate_deserialize)]
    pub struct MonthInYearNumber(pub i64);
    #[derive(Debug, Clone, PartialEq, :: serde :: Deserialize)]
    pub enum NullStyle {
        Null,
    }
    #[derive(
        Clone, Debug, PartialEq, AsRef, Deref, DerefMut, Into, From, :: ruststep_derive :: Holder,
    )]
    # [holder (table = Tables)]
    # [holder (field = parameter_value)]
    #[holder(generate_deserialize)]
    pub struct ParameterValue(pub f64);
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum PersonOrganizationSelect {
        # [holder (field = person)]
        #[holder(use_place_holder)]
        Person(Box<Person>),
        # [holder (field = organization)]
        #[holder(use_place_holder)]
        Organization(Box<Organization>),
        # [holder (field = person_and_organization)]
        #[holder(use_place_holder)]
        PersonAndOrganization(Box<PersonAndOrganization>),
    }
    #[derive(
        Clone, Debug, PartialEq, AsRef, Deref, DerefMut, Into, From, :: ruststep_derive :: Holder,
    )]
    # [holder (table = Tables)]
    # [holder (field = plane_angle_measure)]
    #[holder(generate_deserialize)]
    pub struct PlaneAngleMeasure(pub f64);
    #[derive(
        Clone, Debug, PartialEq, AsRef, Deref, DerefMut, Into, From, :: ruststep_derive :: Holder,
    )]
    # [holder (table = Tables)]
    # [holder (field = positive_length_measure)]
    #[holder(generate_deserialize)]
    pub struct PositiveLengthMeasure(#[holder(use_place_holder)] pub LengthMeasure);
    #[derive(
        Clone, Debug, PartialEq, AsRef, Deref, DerefMut, Into, From, :: ruststep_derive :: Holder,
    )]
    # [holder (table = Tables)]
    # [holder (field = positive_ratio_measure)]
    #[holder(generate_deserialize)]
    pub struct PositiveRatioMeasure(#[holder(use_place_holder)] pub RatioMeasure);
    #[derive(
        Clone, Debug, PartialEq, AsRef, Deref, DerefMut, Into, From, :: ruststep_derive :: Holder,
    )]
    # [holder (table = Tables)]
    # [holder (field = presentable_text)]
    #[holder(generate_deserialize)]
    pub struct PresentableText(pub String);
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum PresentationRepresentationSelect {
        #[holder(use_place_holder)]
        PresentationRepresentation(PresentationRepresentationAny),
        #[holder(use_place_holder)]
        PresentationSet(PresentationSetAny),
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum PresentationSizeAssignmentSelect {
        # [holder (field = presentation_view)]
        #[holder(use_place_holder)]
        PresentationView(Box<PresentationView>),
        #[holder(use_place_holder)]
        PresentationArea(PresentationAreaAny),
        #[holder(use_place_holder)]
        AreaInSet(AreaInSetAny),
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum PresentationStyleSelect {
        # [holder (field = curve_style)]
        #[holder(use_place_holder)]
        CurveStyle(Box<CurveStyle>),
        # [holder (field = symbol_style)]
        #[holder(use_place_holder)]
        SymbolStyle(Box<SymbolStyle>),
        # [holder (field = fill_area_style)]
        #[holder(use_place_holder)]
        FillAreaStyle(Box<FillAreaStyle>),
        #[holder(use_place_holder)]
        TextStyle(TextStyleAny),
        NullStyle(NullStyle),
    }
    #[derive(
        Clone, Debug, PartialEq, AsRef, Deref, DerefMut, Into, From, :: ruststep_derive :: Holder,
    )]
    # [holder (table = Tables)]
    # [holder (field = ratio_measure)]
    #[holder(generate_deserialize)]
    pub struct RatioMeasure(pub f64);
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ShapeDefinition {
        # [holder (field = product_definition_shape)]
        #[holder(use_place_holder)]
        ProductDefinitionShape(Box<ProductDefinitionShape>),
    }
    #[derive(Debug, Clone, PartialEq, :: serde :: Deserialize)]
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
    #[derive(Debug, Clone, PartialEq, :: serde :: Deserialize)]
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
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum SizeSelect {
        #[holder(use_place_holder)]
        PositiveLengthMeasure(Box<PositiveLengthMeasure>),
        #[holder(use_place_holder)]
        MeasureWithUnit(MeasureWithUnitAny),
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum SourceItem {
        #[holder(use_place_holder)]
        Identifier(Box<Identifier>),
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum SpecifiedItem {
        #[holder(use_place_holder)]
        DrawingRevision(DrawingRevisionAny),
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum StyleContextSelect {
        #[holder(use_place_holder)]
        Representation(RepresentationAny),
        #[holder(use_place_holder)]
        RepresentationItem(RepresentationItemAny),
        #[holder(use_place_holder)]
        PresentationSet(PresentationSetAny),
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum SymbolStyleSelect {
        # [holder (field = symbol_colour)]
        #[holder(use_place_holder)]
        SymbolColour(Box<SymbolColour>),
    }
    #[derive(
        Clone, Debug, PartialEq, AsRef, Deref, DerefMut, Into, From, :: ruststep_derive :: Holder,
    )]
    # [holder (table = Tables)]
    # [holder (field = text)]
    #[holder(generate_deserialize)]
    pub struct Text(pub String);
    #[derive(
        Clone, Debug, PartialEq, AsRef, Deref, DerefMut, Into, From, :: ruststep_derive :: Holder,
    )]
    # [holder (table = Tables)]
    # [holder (field = text_alignment)]
    #[holder(generate_deserialize)]
    pub struct TextAlignment(#[holder(use_place_holder)] pub Label);
    #[derive(
        Clone, Debug, PartialEq, AsRef, Deref, DerefMut, Into, From, :: ruststep_derive :: Holder,
    )]
    # [holder (table = Tables)]
    # [holder (field = text_delineation)]
    #[holder(generate_deserialize)]
    pub struct TextDelineation(#[holder(use_place_holder)] pub Label);
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum TextOrCharacter {
        # [holder (field = annotation_text)]
        #[holder(use_place_holder)]
        AnnotationText(Box<AnnotationText>),
        #[holder(use_place_holder)]
        CompositeText(CompositeTextAny),
        #[holder(use_place_holder)]
        TextLiteral(TextLiteralAny),
    }
    #[derive(Debug, Clone, PartialEq, :: serde :: Deserialize)]
    pub enum TextPath {
        Up,
        Right,
        Down,
        Left,
    }
    #[derive(Debug, Clone, PartialEq, :: serde :: Deserialize)]
    pub enum TransitionCode {
        Discontinuous,
        ContSameGradientSameCurvature,
        ContSameGradient,
        Continuous,
    }
    #[derive(Debug, Clone, PartialEq, :: serde :: Deserialize)]
    pub enum TrimmingPreference {
        Parameter,
        Unspecified,
        Cartesian,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum TrimmingSelect {
        # [holder (field = cartesian_point)]
        #[holder(use_place_holder)]
        CartesianPoint(Box<CartesianPoint>),
        #[holder(use_place_holder)]
        ParameterValue(Box<ParameterValue>),
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum Unit {
        #[holder(use_place_holder)]
        NamedUnit(NamedUnitAny),
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum VectorOrDirection {
        # [holder (field = vector)]
        #[holder(use_place_holder)]
        Vector(Box<Vector>),
        # [holder (field = direction)]
        #[holder(use_place_holder)]
        Direction(Box<Direction>),
    }
    #[derive(
        Clone, Debug, PartialEq, AsRef, Deref, DerefMut, Into, From, :: ruststep_derive :: Holder,
    )]
    # [holder (table = Tables)]
    # [holder (field = year_number)]
    #[holder(generate_deserialize)]
    pub struct YearNumber(pub i64);
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = address)]
    #[holder(generate_deserialize)]
    pub struct Address {
        #[holder(use_place_holder)]
        pub internal_location: Option<Label>,
        #[holder(use_place_holder)]
        pub street_number: Option<Label>,
        #[holder(use_place_holder)]
        pub street: Option<Label>,
        #[holder(use_place_holder)]
        pub postal_box: Option<Label>,
        #[holder(use_place_holder)]
        pub town: Option<Label>,
        #[holder(use_place_holder)]
        pub region: Option<Label>,
        #[holder(use_place_holder)]
        pub postal_code: Option<Label>,
        #[holder(use_place_holder)]
        pub country: Option<Label>,
        #[holder(use_place_holder)]
        pub facsimile_number: Option<Label>,
        #[holder(use_place_holder)]
        pub telephone_number: Option<Label>,
        #[holder(use_place_holder)]
        pub electronic_mail_address: Option<Label>,
        #[holder(use_place_holder)]
        pub telex_number: Option<Label>,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum AddressAny {
        #[holder(use_place_holder)]
        # [holder (field = address)]
        Address(Box<Address>),
        #[holder(use_place_holder)]
        # [holder (field = organizational_address)]
        OrganizationalAddress(Box<OrganizationalAddress>),
        #[holder(use_place_holder)]
        # [holder (field = personal_address)]
        PersonalAddress(Box<PersonalAddress>),
    }
    impl Into<AddressAny> for Address {
        fn into(self) -> AddressAny {
            AddressAny::Address(Box::new(self))
        }
    }
    impl Into<AddressAny> for OrganizationalAddress {
        fn into(self) -> AddressAny {
            AddressAny::OrganizationalAddress(Box::new(self.into()))
        }
    }
    impl Into<AddressAny> for PersonalAddress {
        fn into(self) -> AddressAny {
            AddressAny::PersonalAddress(Box::new(self.into()))
        }
    }
    impl AsRef<Address> for AddressAny {
        fn as_ref(&self) -> &Address {
            match self {
                AddressAny::Address(x) => x.as_ref(),
                AddressAny::OrganizationalAddress(x) => (**x).as_ref(),
                AddressAny::PersonalAddress(x) => (**x).as_ref(),
            }
        }
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = angular_dimension)]
    #[holder(generate_deserialize)]
    pub struct AngularDimension {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub dimension_curve_directed_callout: DimensionCurveDirectedCallout,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = annotation_curve_occurrence)]
    #[holder(generate_deserialize)]
    pub struct AnnotationCurveOccurrence {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub annotation_occurrence: AnnotationOccurrence,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum AnnotationCurveOccurrenceAny {
        #[holder(use_place_holder)]
        # [holder (field = annotation_curve_occurrence)]
        AnnotationCurveOccurrence(Box<AnnotationCurveOccurrence>),
        #[holder(use_place_holder)]
        # [holder (field = dimension_curve)]
        DimensionCurve(Box<DimensionCurve>),
        #[holder(use_place_holder)]
        # [holder (field = leader_curve)]
        LeaderCurve(Box<LeaderCurve>),
        #[holder(use_place_holder)]
        # [holder (field = projection_curve)]
        ProjectionCurve(Box<ProjectionCurve>),
    }
    impl Into<AnnotationCurveOccurrenceAny> for AnnotationCurveOccurrence {
        fn into(self) -> AnnotationCurveOccurrenceAny {
            AnnotationCurveOccurrenceAny::AnnotationCurveOccurrence(Box::new(self))
        }
    }
    impl Into<AnnotationCurveOccurrenceAny> for DimensionCurve {
        fn into(self) -> AnnotationCurveOccurrenceAny {
            AnnotationCurveOccurrenceAny::DimensionCurve(Box::new(self.into()))
        }
    }
    impl Into<AnnotationCurveOccurrenceAny> for LeaderCurve {
        fn into(self) -> AnnotationCurveOccurrenceAny {
            AnnotationCurveOccurrenceAny::LeaderCurve(Box::new(self.into()))
        }
    }
    impl Into<AnnotationCurveOccurrenceAny> for ProjectionCurve {
        fn into(self) -> AnnotationCurveOccurrenceAny {
            AnnotationCurveOccurrenceAny::ProjectionCurve(Box::new(self.into()))
        }
    }
    impl AsRef<AnnotationCurveOccurrence> for AnnotationCurveOccurrenceAny {
        fn as_ref(&self) -> &AnnotationCurveOccurrence {
            match self {
                AnnotationCurveOccurrenceAny::AnnotationCurveOccurrence(x) => x.as_ref(),
                AnnotationCurveOccurrenceAny::DimensionCurve(x) => (**x).as_ref(),
                AnnotationCurveOccurrenceAny::LeaderCurve(x) => (**x).as_ref(),
                AnnotationCurveOccurrenceAny::ProjectionCurve(x) => (**x).as_ref(),
            }
        }
    }
    impl AsRef<AnnotationOccurrence> for AnnotationCurveOccurrenceAny {
        fn as_ref(&self) -> &AnnotationOccurrence {
            match self {
                AnnotationCurveOccurrenceAny::AnnotationCurveOccurrence(x) => {
                    AsRef::<AnnotationCurveOccurrence>::as_ref(x).as_ref()
                }
                AnnotationCurveOccurrenceAny::DimensionCurve(x) => {
                    AsRef::<AnnotationCurveOccurrence>::as_ref(x.as_ref()).as_ref()
                }
                AnnotationCurveOccurrenceAny::LeaderCurve(x) => {
                    AsRef::<AnnotationCurveOccurrence>::as_ref(x.as_ref()).as_ref()
                }
                AnnotationCurveOccurrenceAny::ProjectionCurve(x) => {
                    AsRef::<AnnotationCurveOccurrence>::as_ref(x.as_ref()).as_ref()
                }
            }
        }
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = annotation_fill_area)]
    #[holder(generate_deserialize)]
    pub struct AnnotationFillArea {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub geometric_representation_item: GeometricRepresentationItem,
        #[holder(use_place_holder)]
        pub boundaries: Vec<CurveAny>,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = annotation_fill_area_occurrence)]
    #[holder(generate_deserialize)]
    pub struct AnnotationFillAreaOccurrence {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub annotation_occurrence: AnnotationOccurrence,
        #[holder(use_place_holder)]
        pub fill_style_target: PointAny,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = annotation_occurrence)]
    #[holder(generate_deserialize)]
    pub struct AnnotationOccurrence {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub styled_item: StyledItem,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum AnnotationOccurrenceAny {
        #[holder(use_place_holder)]
        # [holder (field = annotation_occurrence)]
        AnnotationOccurrence(Box<AnnotationOccurrence>),
        #[holder(use_place_holder)]
        # [holder (field = annotation_curve_occurrence)]
        AnnotationCurveOccurrence(Box<AnnotationCurveOccurrenceAny>),
        #[holder(use_place_holder)]
        # [holder (field = annotation_fill_area_occurrence)]
        AnnotationFillAreaOccurrence(Box<AnnotationFillAreaOccurrence>),
        #[holder(use_place_holder)]
        # [holder (field = annotation_symbol_occurrence)]
        AnnotationSymbolOccurrence(Box<AnnotationSymbolOccurrenceAny>),
        #[holder(use_place_holder)]
        # [holder (field = annotation_text_occurrence)]
        AnnotationTextOccurrence(Box<AnnotationTextOccurrence>),
        #[holder(use_place_holder)]
        # [holder (field = draughting_annotation_occurrence)]
        DraughtingAnnotationOccurrence(Box<DraughtingAnnotationOccurrence>),
    }
    impl Into<AnnotationOccurrenceAny> for AnnotationOccurrence {
        fn into(self) -> AnnotationOccurrenceAny {
            AnnotationOccurrenceAny::AnnotationOccurrence(Box::new(self))
        }
    }
    impl Into<AnnotationOccurrenceAny> for AnnotationCurveOccurrence {
        fn into(self) -> AnnotationOccurrenceAny {
            AnnotationOccurrenceAny::AnnotationCurveOccurrence(Box::new(self.into()))
        }
    }
    impl Into<AnnotationOccurrenceAny> for AnnotationFillAreaOccurrence {
        fn into(self) -> AnnotationOccurrenceAny {
            AnnotationOccurrenceAny::AnnotationFillAreaOccurrence(Box::new(self.into()))
        }
    }
    impl Into<AnnotationOccurrenceAny> for AnnotationSymbolOccurrence {
        fn into(self) -> AnnotationOccurrenceAny {
            AnnotationOccurrenceAny::AnnotationSymbolOccurrence(Box::new(self.into()))
        }
    }
    impl Into<AnnotationOccurrenceAny> for AnnotationTextOccurrence {
        fn into(self) -> AnnotationOccurrenceAny {
            AnnotationOccurrenceAny::AnnotationTextOccurrence(Box::new(self.into()))
        }
    }
    impl Into<AnnotationOccurrenceAny> for DraughtingAnnotationOccurrence {
        fn into(self) -> AnnotationOccurrenceAny {
            AnnotationOccurrenceAny::DraughtingAnnotationOccurrence(Box::new(self.into()))
        }
    }
    impl AsRef<AnnotationOccurrence> for AnnotationOccurrenceAny {
        fn as_ref(&self) -> &AnnotationOccurrence {
            match self {
                AnnotationOccurrenceAny::AnnotationOccurrence(x) => x.as_ref(),
                AnnotationOccurrenceAny::AnnotationCurveOccurrence(x) => (**x).as_ref(),
                AnnotationOccurrenceAny::AnnotationFillAreaOccurrence(x) => (**x).as_ref(),
                AnnotationOccurrenceAny::AnnotationSymbolOccurrence(x) => (**x).as_ref(),
                AnnotationOccurrenceAny::AnnotationTextOccurrence(x) => (**x).as_ref(),
                AnnotationOccurrenceAny::DraughtingAnnotationOccurrence(x) => (**x).as_ref(),
            }
        }
    }
    impl AsRef<StyledItem> for AnnotationOccurrenceAny {
        fn as_ref(&self) -> &StyledItem {
            match self {
                AnnotationOccurrenceAny::AnnotationOccurrence(x) => {
                    AsRef::<AnnotationOccurrence>::as_ref(x).as_ref()
                }
                AnnotationOccurrenceAny::AnnotationCurveOccurrence(x) => {
                    AsRef::<AnnotationOccurrence>::as_ref(x.as_ref()).as_ref()
                }
                AnnotationOccurrenceAny::AnnotationFillAreaOccurrence(x) => {
                    AsRef::<AnnotationOccurrence>::as_ref(x.as_ref()).as_ref()
                }
                AnnotationOccurrenceAny::AnnotationSymbolOccurrence(x) => {
                    AsRef::<AnnotationOccurrence>::as_ref(x.as_ref()).as_ref()
                }
                AnnotationOccurrenceAny::AnnotationTextOccurrence(x) => {
                    AsRef::<AnnotationOccurrence>::as_ref(x.as_ref()).as_ref()
                }
                AnnotationOccurrenceAny::DraughtingAnnotationOccurrence(x) => {
                    AsRef::<AnnotationOccurrence>::as_ref(x.as_ref()).as_ref()
                }
            }
        }
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = annotation_subfigure_occurrence)]
    #[holder(generate_deserialize)]
    pub struct AnnotationSubfigureOccurrence {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub annotation_symbol_occurrence: AnnotationSymbolOccurrence,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = annotation_symbol)]
    #[holder(generate_deserialize)]
    pub struct AnnotationSymbol {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub mapped_item: MappedItem,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = annotation_symbol_occurrence)]
    #[holder(generate_deserialize)]
    pub struct AnnotationSymbolOccurrence {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub annotation_occurrence: AnnotationOccurrence,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum AnnotationSymbolOccurrenceAny {
        #[holder(use_place_holder)]
        # [holder (field = annotation_symbol_occurrence)]
        AnnotationSymbolOccurrence(Box<AnnotationSymbolOccurrence>),
        #[holder(use_place_holder)]
        # [holder (field = annotation_subfigure_occurrence)]
        AnnotationSubfigureOccurrence(Box<AnnotationSubfigureOccurrence>),
        #[holder(use_place_holder)]
        # [holder (field = terminator_symbol)]
        TerminatorSymbol(Box<TerminatorSymbolAny>),
    }
    impl Into<AnnotationSymbolOccurrenceAny> for AnnotationSymbolOccurrence {
        fn into(self) -> AnnotationSymbolOccurrenceAny {
            AnnotationSymbolOccurrenceAny::AnnotationSymbolOccurrence(Box::new(self))
        }
    }
    impl Into<AnnotationSymbolOccurrenceAny> for AnnotationSubfigureOccurrence {
        fn into(self) -> AnnotationSymbolOccurrenceAny {
            AnnotationSymbolOccurrenceAny::AnnotationSubfigureOccurrence(Box::new(self.into()))
        }
    }
    impl Into<AnnotationSymbolOccurrenceAny> for TerminatorSymbol {
        fn into(self) -> AnnotationSymbolOccurrenceAny {
            AnnotationSymbolOccurrenceAny::TerminatorSymbol(Box::new(self.into()))
        }
    }
    impl AsRef<AnnotationSymbolOccurrence> for AnnotationSymbolOccurrenceAny {
        fn as_ref(&self) -> &AnnotationSymbolOccurrence {
            match self {
                AnnotationSymbolOccurrenceAny::AnnotationSymbolOccurrence(x) => x.as_ref(),
                AnnotationSymbolOccurrenceAny::AnnotationSubfigureOccurrence(x) => (**x).as_ref(),
                AnnotationSymbolOccurrenceAny::TerminatorSymbol(x) => (**x).as_ref(),
            }
        }
    }
    impl AsRef<AnnotationOccurrence> for AnnotationSymbolOccurrenceAny {
        fn as_ref(&self) -> &AnnotationOccurrence {
            match self {
                AnnotationSymbolOccurrenceAny::AnnotationSymbolOccurrence(x) => {
                    AsRef::<AnnotationSymbolOccurrence>::as_ref(x).as_ref()
                }
                AnnotationSymbolOccurrenceAny::AnnotationSubfigureOccurrence(x) => {
                    AsRef::<AnnotationSymbolOccurrence>::as_ref(x.as_ref()).as_ref()
                }
                AnnotationSymbolOccurrenceAny::TerminatorSymbol(x) => {
                    AsRef::<AnnotationSymbolOccurrence>::as_ref(x.as_ref()).as_ref()
                }
            }
        }
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = annotation_text)]
    #[holder(generate_deserialize)]
    pub struct AnnotationText {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub mapped_item: MappedItem,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = annotation_text_occurrence)]
    #[holder(generate_deserialize)]
    pub struct AnnotationTextOccurrence {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub annotation_occurrence: AnnotationOccurrence,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = application_context)]
    #[holder(generate_deserialize)]
    pub struct ApplicationContext {
        #[holder(use_place_holder)]
        pub application: Text,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = application_context_element)]
    #[holder(generate_deserialize)]
    pub struct ApplicationContextElement {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub frame_of_reference: ApplicationContext,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ApplicationContextElementAny {
        #[holder(use_place_holder)]
        # [holder (field = application_context_element)]
        ApplicationContextElement(Box<ApplicationContextElement>),
        #[holder(use_place_holder)]
        # [holder (field = product_context)]
        ProductContext(Box<ProductContext>),
        #[holder(use_place_holder)]
        # [holder (field = product_definition_context)]
        ProductDefinitionContext(Box<ProductDefinitionContext>),
    }
    impl Into<ApplicationContextElementAny> for ApplicationContextElement {
        fn into(self) -> ApplicationContextElementAny {
            ApplicationContextElementAny::ApplicationContextElement(Box::new(self))
        }
    }
    impl Into<ApplicationContextElementAny> for ProductContext {
        fn into(self) -> ApplicationContextElementAny {
            ApplicationContextElementAny::ProductContext(Box::new(self.into()))
        }
    }
    impl Into<ApplicationContextElementAny> for ProductDefinitionContext {
        fn into(self) -> ApplicationContextElementAny {
            ApplicationContextElementAny::ProductDefinitionContext(Box::new(self.into()))
        }
    }
    impl AsRef<ApplicationContextElement> for ApplicationContextElementAny {
        fn as_ref(&self) -> &ApplicationContextElement {
            match self {
                ApplicationContextElementAny::ApplicationContextElement(x) => x.as_ref(),
                ApplicationContextElementAny::ProductContext(x) => (**x).as_ref(),
                ApplicationContextElementAny::ProductDefinitionContext(x) => (**x).as_ref(),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = application_protocol_definition)]
    #[holder(generate_deserialize)]
    pub struct ApplicationProtocolDefinition {
        #[holder(use_place_holder)]
        pub status: Label,
        #[holder(use_place_holder)]
        pub application_interpreted_model_schema_name: Label,
        #[holder(use_place_holder)]
        pub application_protocol_year: YearNumber,
        #[holder(use_place_holder)]
        pub application: ApplicationContext,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = approval)]
    #[holder(generate_deserialize)]
    pub struct Approval {
        #[holder(use_place_holder)]
        pub status: ApprovalStatus,
        #[holder(use_place_holder)]
        pub level: Label,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = approval_assignment)]
    #[holder(generate_deserialize)]
    pub struct ApprovalAssignment {
        #[holder(use_place_holder)]
        pub assigned_approval: Approval,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ApprovalAssignmentAny {
        #[holder(use_place_holder)]
        # [holder (field = approval_assignment)]
        ApprovalAssignment(Box<ApprovalAssignment>),
        #[holder(use_place_holder)]
        # [holder (field = draughting_approval_assignment)]
        DraughtingApprovalAssignment(Box<DraughtingApprovalAssignment>),
    }
    impl Into<ApprovalAssignmentAny> for ApprovalAssignment {
        fn into(self) -> ApprovalAssignmentAny {
            ApprovalAssignmentAny::ApprovalAssignment(Box::new(self))
        }
    }
    impl Into<ApprovalAssignmentAny> for DraughtingApprovalAssignment {
        fn into(self) -> ApprovalAssignmentAny {
            ApprovalAssignmentAny::DraughtingApprovalAssignment(Box::new(self.into()))
        }
    }
    impl AsRef<ApprovalAssignment> for ApprovalAssignmentAny {
        fn as_ref(&self) -> &ApprovalAssignment {
            match self {
                ApprovalAssignmentAny::ApprovalAssignment(x) => x.as_ref(),
                ApprovalAssignmentAny::DraughtingApprovalAssignment(x) => (**x).as_ref(),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = approval_date_time)]
    #[holder(generate_deserialize)]
    pub struct ApprovalDateTime {
        #[holder(use_place_holder)]
        pub date_time: DateTimeSelect,
        #[holder(use_place_holder)]
        pub dated_approval: Approval,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = approval_person_organization)]
    #[holder(generate_deserialize)]
    pub struct ApprovalPersonOrganization {
        #[holder(use_place_holder)]
        pub person_organization: PersonOrganizationSelect,
        #[holder(use_place_holder)]
        pub authorized_approval: Approval,
        #[holder(use_place_holder)]
        pub role: ApprovalRole,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = approval_role)]
    #[holder(generate_deserialize)]
    pub struct ApprovalRole {
        #[holder(use_place_holder)]
        pub role: Label,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = approval_status)]
    #[holder(generate_deserialize)]
    pub struct ApprovalStatus {
        #[holder(use_place_holder)]
        pub name: Label,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = area_in_set)]
    #[holder(generate_deserialize)]
    pub struct AreaInSet {
        #[holder(use_place_holder)]
        pub area: PresentationAreaAny,
        #[holder(use_place_holder)]
        pub in_set: PresentationSetAny,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum AreaInSetAny {
        #[holder(use_place_holder)]
        # [holder (field = area_in_set)]
        AreaInSet(Box<AreaInSet>),
        #[holder(use_place_holder)]
        # [holder (field = drawing_sheet_revision_usage)]
        DrawingSheetRevisionUsage(Box<DrawingSheetRevisionUsage>),
    }
    impl Into<AreaInSetAny> for AreaInSet {
        fn into(self) -> AreaInSetAny {
            AreaInSetAny::AreaInSet(Box::new(self))
        }
    }
    impl Into<AreaInSetAny> for DrawingSheetRevisionUsage {
        fn into(self) -> AreaInSetAny {
            AreaInSetAny::DrawingSheetRevisionUsage(Box::new(self.into()))
        }
    }
    impl AsRef<AreaInSet> for AreaInSetAny {
        fn as_ref(&self) -> &AreaInSet {
            match self {
                AreaInSetAny::AreaInSet(x) => x.as_ref(),
                AreaInSetAny::DrawingSheetRevisionUsage(x) => (**x).as_ref(),
            }
        }
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = axis2_placement_2d)]
    #[holder(generate_deserialize)]
    pub struct Axis2Placement2D {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub placement: Placement,
        #[holder(use_place_holder)]
        pub ref_direction: Option<Direction>,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = b_spline_curve)]
    #[holder(generate_deserialize)]
    pub struct BSplineCurve {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub bounded_curve: BoundedCurve,
        pub degree: i64,
        #[holder(use_place_holder)]
        pub control_points_list: Vec<CartesianPoint>,
        pub curve_form: BSplineCurveForm,
        pub closed_curve: Logical,
        pub self_intersect: Logical,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum BSplineCurveAny {
        #[holder(use_place_holder)]
        # [holder (field = b_spline_curve)]
        BSplineCurve(Box<BSplineCurve>),
        #[holder(use_place_holder)]
        # [holder (field = b_spline_curve_with_knots)]
        BSplineCurveWithKnots(Box<BSplineCurveWithKnots>),
        #[holder(use_place_holder)]
        # [holder (field = bezier_curve)]
        BezierCurve(Box<BezierCurve>),
        #[holder(use_place_holder)]
        # [holder (field = quasi_uniform_curve)]
        QuasiUniformCurve(Box<QuasiUniformCurve>),
        #[holder(use_place_holder)]
        # [holder (field = rational_b_spline_curve)]
        RationalBSplineCurve(Box<RationalBSplineCurve>),
        #[holder(use_place_holder)]
        # [holder (field = uniform_curve)]
        UniformCurve(Box<UniformCurve>),
    }
    impl Into<BSplineCurveAny> for BSplineCurve {
        fn into(self) -> BSplineCurveAny {
            BSplineCurveAny::BSplineCurve(Box::new(self))
        }
    }
    impl Into<BSplineCurveAny> for BSplineCurveWithKnots {
        fn into(self) -> BSplineCurveAny {
            BSplineCurveAny::BSplineCurveWithKnots(Box::new(self.into()))
        }
    }
    impl Into<BSplineCurveAny> for BezierCurve {
        fn into(self) -> BSplineCurveAny {
            BSplineCurveAny::BezierCurve(Box::new(self.into()))
        }
    }
    impl Into<BSplineCurveAny> for QuasiUniformCurve {
        fn into(self) -> BSplineCurveAny {
            BSplineCurveAny::QuasiUniformCurve(Box::new(self.into()))
        }
    }
    impl Into<BSplineCurveAny> for RationalBSplineCurve {
        fn into(self) -> BSplineCurveAny {
            BSplineCurveAny::RationalBSplineCurve(Box::new(self.into()))
        }
    }
    impl Into<BSplineCurveAny> for UniformCurve {
        fn into(self) -> BSplineCurveAny {
            BSplineCurveAny::UniformCurve(Box::new(self.into()))
        }
    }
    impl AsRef<BSplineCurve> for BSplineCurveAny {
        fn as_ref(&self) -> &BSplineCurve {
            match self {
                BSplineCurveAny::BSplineCurve(x) => x.as_ref(),
                BSplineCurveAny::BSplineCurveWithKnots(x) => (**x).as_ref(),
                BSplineCurveAny::BezierCurve(x) => (**x).as_ref(),
                BSplineCurveAny::QuasiUniformCurve(x) => (**x).as_ref(),
                BSplineCurveAny::RationalBSplineCurve(x) => (**x).as_ref(),
                BSplineCurveAny::UniformCurve(x) => (**x).as_ref(),
            }
        }
    }
    impl AsRef<BoundedCurve> for BSplineCurveAny {
        fn as_ref(&self) -> &BoundedCurve {
            match self {
                BSplineCurveAny::BSplineCurve(x) => AsRef::<BSplineCurve>::as_ref(x).as_ref(),
                BSplineCurveAny::BSplineCurveWithKnots(x) => {
                    AsRef::<BSplineCurve>::as_ref(x.as_ref()).as_ref()
                }
                BSplineCurveAny::BezierCurve(x) => {
                    AsRef::<BSplineCurve>::as_ref(x.as_ref()).as_ref()
                }
                BSplineCurveAny::QuasiUniformCurve(x) => {
                    AsRef::<BSplineCurve>::as_ref(x.as_ref()).as_ref()
                }
                BSplineCurveAny::RationalBSplineCurve(x) => {
                    AsRef::<BSplineCurve>::as_ref(x.as_ref()).as_ref()
                }
                BSplineCurveAny::UniformCurve(x) => {
                    AsRef::<BSplineCurve>::as_ref(x.as_ref()).as_ref()
                }
            }
        }
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = b_spline_curve_with_knots)]
    #[holder(generate_deserialize)]
    pub struct BSplineCurveWithKnots {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub b_spline_curve: BSplineCurve,
        pub knot_multiplicities: Vec<i64>,
        #[holder(use_place_holder)]
        pub knots: Vec<ParameterValue>,
        pub knot_spec: KnotType,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = bezier_curve)]
    #[holder(generate_deserialize)]
    pub struct BezierCurve {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub b_spline_curve: BSplineCurve,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = bounded_curve)]
    #[holder(generate_deserialize)]
    pub struct BoundedCurve {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub curve: Curve,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum BoundedCurveAny {
        #[holder(use_place_holder)]
        # [holder (field = bounded_curve)]
        BoundedCurve(Box<BoundedCurve>),
        #[holder(use_place_holder)]
        # [holder (field = b_spline_curve)]
        BSplineCurve(Box<BSplineCurveAny>),
        #[holder(use_place_holder)]
        # [holder (field = composite_curve)]
        CompositeCurve(Box<CompositeCurve>),
        #[holder(use_place_holder)]
        # [holder (field = polyline)]
        Polyline(Box<Polyline>),
        #[holder(use_place_holder)]
        # [holder (field = trimmed_curve)]
        TrimmedCurve(Box<TrimmedCurve>),
    }
    impl Into<BoundedCurveAny> for BoundedCurve {
        fn into(self) -> BoundedCurveAny {
            BoundedCurveAny::BoundedCurve(Box::new(self))
        }
    }
    impl Into<BoundedCurveAny> for BSplineCurve {
        fn into(self) -> BoundedCurveAny {
            BoundedCurveAny::BSplineCurve(Box::new(self.into()))
        }
    }
    impl Into<BoundedCurveAny> for CompositeCurve {
        fn into(self) -> BoundedCurveAny {
            BoundedCurveAny::CompositeCurve(Box::new(self.into()))
        }
    }
    impl Into<BoundedCurveAny> for Polyline {
        fn into(self) -> BoundedCurveAny {
            BoundedCurveAny::Polyline(Box::new(self.into()))
        }
    }
    impl Into<BoundedCurveAny> for TrimmedCurve {
        fn into(self) -> BoundedCurveAny {
            BoundedCurveAny::TrimmedCurve(Box::new(self.into()))
        }
    }
    impl AsRef<BoundedCurve> for BoundedCurveAny {
        fn as_ref(&self) -> &BoundedCurve {
            match self {
                BoundedCurveAny::BoundedCurve(x) => x.as_ref(),
                BoundedCurveAny::BSplineCurve(x) => (**x).as_ref(),
                BoundedCurveAny::CompositeCurve(x) => (**x).as_ref(),
                BoundedCurveAny::Polyline(x) => (**x).as_ref(),
                BoundedCurveAny::TrimmedCurve(x) => (**x).as_ref(),
            }
        }
    }
    impl AsRef<Curve> for BoundedCurveAny {
        fn as_ref(&self) -> &Curve {
            match self {
                BoundedCurveAny::BoundedCurve(x) => AsRef::<BoundedCurve>::as_ref(x).as_ref(),
                BoundedCurveAny::BSplineCurve(x) => {
                    AsRef::<BoundedCurve>::as_ref(x.as_ref()).as_ref()
                }
                BoundedCurveAny::CompositeCurve(x) => {
                    AsRef::<BoundedCurve>::as_ref(x.as_ref()).as_ref()
                }
                BoundedCurveAny::Polyline(x) => AsRef::<BoundedCurve>::as_ref(x.as_ref()).as_ref(),
                BoundedCurveAny::TrimmedCurve(x) => {
                    AsRef::<BoundedCurve>::as_ref(x.as_ref()).as_ref()
                }
            }
        }
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = calendar_date)]
    #[holder(generate_deserialize)]
    pub struct CalendarDate {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub date: Date,
        #[holder(use_place_holder)]
        pub day_component: DayInMonthNumber,
        #[holder(use_place_holder)]
        pub month_component: MonthInYearNumber,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = camera_image)]
    #[holder(generate_deserialize)]
    pub struct CameraImage {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub mapped_item: MappedItem,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum CameraImageAny {
        #[holder(use_place_holder)]
        # [holder (field = camera_image)]
        CameraImage(Box<CameraImage>),
        #[holder(use_place_holder)]
        # [holder (field = camera_image_2d_with_scale)]
        CameraImage2DWithScale(Box<CameraImage2DWithScale>),
    }
    impl Into<CameraImageAny> for CameraImage {
        fn into(self) -> CameraImageAny {
            CameraImageAny::CameraImage(Box::new(self))
        }
    }
    impl Into<CameraImageAny> for CameraImage2DWithScale {
        fn into(self) -> CameraImageAny {
            CameraImageAny::CameraImage2DWithScale(Box::new(self.into()))
        }
    }
    impl AsRef<CameraImage> for CameraImageAny {
        fn as_ref(&self) -> &CameraImage {
            match self {
                CameraImageAny::CameraImage(x) => x.as_ref(),
                CameraImageAny::CameraImage2DWithScale(x) => (**x).as_ref(),
            }
        }
    }
    impl AsRef<MappedItem> for CameraImageAny {
        fn as_ref(&self) -> &MappedItem {
            match self {
                CameraImageAny::CameraImage(x) => AsRef::<CameraImage>::as_ref(x).as_ref(),
                CameraImageAny::CameraImage2DWithScale(x) => {
                    AsRef::<CameraImage>::as_ref(x.as_ref()).as_ref()
                }
            }
        }
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = camera_image_2d_with_scale)]
    #[holder(generate_deserialize)]
    pub struct CameraImage2DWithScale {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub camera_image: CameraImage,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = camera_model)]
    #[holder(generate_deserialize)]
    pub struct CameraModel {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub geometric_representation_item: GeometricRepresentationItem,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum CameraModelAny {
        #[holder(use_place_holder)]
        # [holder (field = camera_model)]
        CameraModel(Box<CameraModel>),
        #[holder(use_place_holder)]
        # [holder (field = camera_model_d2)]
        CameraModelD2(Box<CameraModelD2>),
    }
    impl Into<CameraModelAny> for CameraModel {
        fn into(self) -> CameraModelAny {
            CameraModelAny::CameraModel(Box::new(self))
        }
    }
    impl Into<CameraModelAny> for CameraModelD2 {
        fn into(self) -> CameraModelAny {
            CameraModelAny::CameraModelD2(Box::new(self.into()))
        }
    }
    impl AsRef<CameraModel> for CameraModelAny {
        fn as_ref(&self) -> &CameraModel {
            match self {
                CameraModelAny::CameraModel(x) => x.as_ref(),
                CameraModelAny::CameraModelD2(x) => (**x).as_ref(),
            }
        }
    }
    impl AsRef<GeometricRepresentationItem> for CameraModelAny {
        fn as_ref(&self) -> &GeometricRepresentationItem {
            match self {
                CameraModelAny::CameraModel(x) => AsRef::<CameraModel>::as_ref(x).as_ref(),
                CameraModelAny::CameraModelD2(x) => {
                    AsRef::<CameraModel>::as_ref(x.as_ref()).as_ref()
                }
            }
        }
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = camera_model_d2)]
    #[holder(generate_deserialize)]
    pub struct CameraModelD2 {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub camera_model: CameraModel,
        #[holder(use_place_holder)]
        pub view_window: PlanarBox,
        pub view_window_clipping: bool,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = camera_usage)]
    #[holder(generate_deserialize)]
    pub struct CameraUsage {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub representation_map: RepresentationMap,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = cartesian_point)]
    #[holder(generate_deserialize)]
    pub struct CartesianPoint {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub point: Point,
        #[holder(use_place_holder)]
        pub coordinates: Vec<LengthMeasure>,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = circle)]
    #[holder(generate_deserialize)]
    pub struct Circle {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub conic: Conic,
        #[holder(use_place_holder)]
        pub radius: PositiveLengthMeasure,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = colour)]
    #[holder(generate_deserialize)]
    pub struct Colour {}
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ColourAny {
        #[holder(use_place_holder)]
        # [holder (field = colour)]
        Colour(Box<Colour>),
        #[holder(use_place_holder)]
        # [holder (field = colour_specification)]
        ColourSpecification(Box<ColourSpecificationAny>),
        #[holder(use_place_holder)]
        # [holder (field = pre_defined_colour)]
        PreDefinedColour(Box<PreDefinedColourAny>),
    }
    impl Into<ColourAny> for Colour {
        fn into(self) -> ColourAny {
            ColourAny::Colour(Box::new(self))
        }
    }
    impl Into<ColourAny> for ColourSpecification {
        fn into(self) -> ColourAny {
            ColourAny::ColourSpecification(Box::new(self.into()))
        }
    }
    impl Into<ColourAny> for PreDefinedColour {
        fn into(self) -> ColourAny {
            ColourAny::PreDefinedColour(Box::new(self.into()))
        }
    }
    impl AsRef<Colour> for ColourAny {
        fn as_ref(&self) -> &Colour {
            match self {
                ColourAny::Colour(x) => x.as_ref(),
                ColourAny::ColourSpecification(x) => (**x).as_ref(),
                ColourAny::PreDefinedColour(x) => (**x).as_ref(),
            }
        }
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = colour_rgb)]
    #[holder(generate_deserialize)]
    pub struct ColourRgb {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub colour_specification: ColourSpecification,
        pub red: f64,
        pub green: f64,
        pub blue: f64,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = colour_specification)]
    #[holder(generate_deserialize)]
    pub struct ColourSpecification {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub colour: Colour,
        #[holder(use_place_holder)]
        pub name: ColourAny,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ColourSpecificationAny {
        #[holder(use_place_holder)]
        # [holder (field = colour_specification)]
        ColourSpecification(Box<ColourSpecification>),
        #[holder(use_place_holder)]
        # [holder (field = colour_rgb)]
        ColourRgb(Box<ColourRgb>),
    }
    impl Into<ColourSpecificationAny> for ColourSpecification {
        fn into(self) -> ColourSpecificationAny {
            ColourSpecificationAny::ColourSpecification(Box::new(self))
        }
    }
    impl Into<ColourSpecificationAny> for ColourRgb {
        fn into(self) -> ColourSpecificationAny {
            ColourSpecificationAny::ColourRgb(Box::new(self.into()))
        }
    }
    impl AsRef<ColourSpecification> for ColourSpecificationAny {
        fn as_ref(&self) -> &ColourSpecification {
            match self {
                ColourSpecificationAny::ColourSpecification(x) => x.as_ref(),
                ColourSpecificationAny::ColourRgb(x) => (**x).as_ref(),
            }
        }
    }
    impl AsRef<Colour> for ColourSpecificationAny {
        fn as_ref(&self) -> &Colour {
            match self {
                ColourSpecificationAny::ColourSpecification(x) => {
                    AsRef::<ColourSpecification>::as_ref(x).as_ref()
                }
                ColourSpecificationAny::ColourRgb(x) => {
                    AsRef::<ColourSpecification>::as_ref(x.as_ref()).as_ref()
                }
            }
        }
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = composite_curve)]
    #[holder(generate_deserialize)]
    pub struct CompositeCurve {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub bounded_curve: BoundedCurve,
        #[holder(use_place_holder)]
        pub segments: Vec<CompositeCurveSegment>,
        pub self_intersect: Logical,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = composite_curve_segment)]
    #[holder(generate_deserialize)]
    pub struct CompositeCurveSegment {
        pub transition: TransitionCode,
        pub same_sense: bool,
        #[holder(use_place_holder)]
        pub parent_curve: CurveAny,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = composite_text)]
    #[holder(generate_deserialize)]
    pub struct CompositeText {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub geometric_representation_item: GeometricRepresentationItem,
        #[holder(use_place_holder)]
        pub collected_text: Vec<TextOrCharacter>,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum CompositeTextAny {
        #[holder(use_place_holder)]
        # [holder (field = composite_text)]
        CompositeText(Box<CompositeText>),
        #[holder(use_place_holder)]
        # [holder (field = composite_text_with_associated_curves)]
        CompositeTextWithAssociatedCurves(Box<CompositeTextWithAssociatedCurves>),
        #[holder(use_place_holder)]
        # [holder (field = composite_text_with_blanking_box)]
        CompositeTextWithBlankingBox(Box<CompositeTextWithBlankingBox>),
        #[holder(use_place_holder)]
        # [holder (field = composite_text_with_extent)]
        CompositeTextWithExtent(Box<CompositeTextWithExtent>),
    }
    impl Into<CompositeTextAny> for CompositeText {
        fn into(self) -> CompositeTextAny {
            CompositeTextAny::CompositeText(Box::new(self))
        }
    }
    impl Into<CompositeTextAny> for CompositeTextWithAssociatedCurves {
        fn into(self) -> CompositeTextAny {
            CompositeTextAny::CompositeTextWithAssociatedCurves(Box::new(self.into()))
        }
    }
    impl Into<CompositeTextAny> for CompositeTextWithBlankingBox {
        fn into(self) -> CompositeTextAny {
            CompositeTextAny::CompositeTextWithBlankingBox(Box::new(self.into()))
        }
    }
    impl Into<CompositeTextAny> for CompositeTextWithExtent {
        fn into(self) -> CompositeTextAny {
            CompositeTextAny::CompositeTextWithExtent(Box::new(self.into()))
        }
    }
    impl AsRef<CompositeText> for CompositeTextAny {
        fn as_ref(&self) -> &CompositeText {
            match self {
                CompositeTextAny::CompositeText(x) => x.as_ref(),
                CompositeTextAny::CompositeTextWithAssociatedCurves(x) => (**x).as_ref(),
                CompositeTextAny::CompositeTextWithBlankingBox(x) => (**x).as_ref(),
                CompositeTextAny::CompositeTextWithExtent(x) => (**x).as_ref(),
            }
        }
    }
    impl AsRef<GeometricRepresentationItem> for CompositeTextAny {
        fn as_ref(&self) -> &GeometricRepresentationItem {
            match self {
                CompositeTextAny::CompositeText(x) => AsRef::<CompositeText>::as_ref(x).as_ref(),
                CompositeTextAny::CompositeTextWithAssociatedCurves(x) => {
                    AsRef::<CompositeText>::as_ref(x.as_ref()).as_ref()
                }
                CompositeTextAny::CompositeTextWithBlankingBox(x) => {
                    AsRef::<CompositeText>::as_ref(x.as_ref()).as_ref()
                }
                CompositeTextAny::CompositeTextWithExtent(x) => {
                    AsRef::<CompositeText>::as_ref(x.as_ref()).as_ref()
                }
            }
        }
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = composite_text_with_associated_curves)]
    #[holder(generate_deserialize)]
    pub struct CompositeTextWithAssociatedCurves {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub composite_text: CompositeText,
        #[holder(use_place_holder)]
        pub associated_curves: Vec<CurveAny>,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = composite_text_with_blanking_box)]
    #[holder(generate_deserialize)]
    pub struct CompositeTextWithBlankingBox {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub composite_text: CompositeText,
        #[holder(use_place_holder)]
        pub blanking: PlanarBox,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = composite_text_with_extent)]
    #[holder(generate_deserialize)]
    pub struct CompositeTextWithExtent {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub composite_text: CompositeText,
        #[holder(use_place_holder)]
        pub extent: PlanarExtentAny,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = conic)]
    #[holder(generate_deserialize)]
    pub struct Conic {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub curve: Curve,
        #[holder(use_place_holder)]
        pub position: Axis2Placement,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ConicAny {
        #[holder(use_place_holder)]
        # [holder (field = conic)]
        Conic(Box<Conic>),
        #[holder(use_place_holder)]
        # [holder (field = circle)]
        Circle(Box<Circle>),
        #[holder(use_place_holder)]
        # [holder (field = ellipse)]
        Ellipse(Box<Ellipse>),
        #[holder(use_place_holder)]
        # [holder (field = hyperbola)]
        Hyperbola(Box<Hyperbola>),
        #[holder(use_place_holder)]
        # [holder (field = parabola)]
        Parabola(Box<Parabola>),
    }
    impl Into<ConicAny> for Conic {
        fn into(self) -> ConicAny {
            ConicAny::Conic(Box::new(self))
        }
    }
    impl Into<ConicAny> for Circle {
        fn into(self) -> ConicAny {
            ConicAny::Circle(Box::new(self.into()))
        }
    }
    impl Into<ConicAny> for Ellipse {
        fn into(self) -> ConicAny {
            ConicAny::Ellipse(Box::new(self.into()))
        }
    }
    impl Into<ConicAny> for Hyperbola {
        fn into(self) -> ConicAny {
            ConicAny::Hyperbola(Box::new(self.into()))
        }
    }
    impl Into<ConicAny> for Parabola {
        fn into(self) -> ConicAny {
            ConicAny::Parabola(Box::new(self.into()))
        }
    }
    impl AsRef<Conic> for ConicAny {
        fn as_ref(&self) -> &Conic {
            match self {
                ConicAny::Conic(x) => x.as_ref(),
                ConicAny::Circle(x) => (**x).as_ref(),
                ConicAny::Ellipse(x) => (**x).as_ref(),
                ConicAny::Hyperbola(x) => (**x).as_ref(),
                ConicAny::Parabola(x) => (**x).as_ref(),
            }
        }
    }
    impl AsRef<Curve> for ConicAny {
        fn as_ref(&self) -> &Curve {
            match self {
                ConicAny::Conic(x) => AsRef::<Conic>::as_ref(x).as_ref(),
                ConicAny::Circle(x) => AsRef::<Conic>::as_ref(x.as_ref()).as_ref(),
                ConicAny::Ellipse(x) => AsRef::<Conic>::as_ref(x.as_ref()).as_ref(),
                ConicAny::Hyperbola(x) => AsRef::<Conic>::as_ref(x.as_ref()).as_ref(),
                ConicAny::Parabola(x) => AsRef::<Conic>::as_ref(x.as_ref()).as_ref(),
            }
        }
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = context_dependent_invisibility)]
    #[holder(generate_deserialize)]
    pub struct ContextDependentInvisibility {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub invisibility: Invisibility,
        #[holder(use_place_holder)]
        pub presentation_context: InvisibilityContext,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = contract)]
    #[holder(generate_deserialize)]
    pub struct Contract {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub purpose: Text,
        #[holder(use_place_holder)]
        pub kind: ContractType,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = contract_assignment)]
    #[holder(generate_deserialize)]
    pub struct ContractAssignment {
        #[holder(use_place_holder)]
        pub assigned_contract: Contract,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ContractAssignmentAny {
        #[holder(use_place_holder)]
        # [holder (field = contract_assignment)]
        ContractAssignment(Box<ContractAssignment>),
        #[holder(use_place_holder)]
        # [holder (field = draughting_contract_assignment)]
        DraughtingContractAssignment(Box<DraughtingContractAssignment>),
    }
    impl Into<ContractAssignmentAny> for ContractAssignment {
        fn into(self) -> ContractAssignmentAny {
            ContractAssignmentAny::ContractAssignment(Box::new(self))
        }
    }
    impl Into<ContractAssignmentAny> for DraughtingContractAssignment {
        fn into(self) -> ContractAssignmentAny {
            ContractAssignmentAny::DraughtingContractAssignment(Box::new(self.into()))
        }
    }
    impl AsRef<ContractAssignment> for ContractAssignmentAny {
        fn as_ref(&self) -> &ContractAssignment {
            match self {
                ContractAssignmentAny::ContractAssignment(x) => x.as_ref(),
                ContractAssignmentAny::DraughtingContractAssignment(x) => (**x).as_ref(),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = contract_type)]
    #[holder(generate_deserialize)]
    pub struct ContractType {
        #[holder(use_place_holder)]
        pub description: Label,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = conversion_based_unit)]
    #[holder(generate_deserialize)]
    pub struct ConversionBasedUnit {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub named_unit: NamedUnit,
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub conversion_factor: MeasureWithUnitAny,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = curve)]
    #[holder(generate_deserialize)]
    pub struct Curve {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub geometric_representation_item: GeometricRepresentationItem,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum CurveAny {
        #[holder(use_place_holder)]
        # [holder (field = curve)]
        Curve(Box<Curve>),
        #[holder(use_place_holder)]
        # [holder (field = bounded_curve)]
        BoundedCurve(Box<BoundedCurveAny>),
        #[holder(use_place_holder)]
        # [holder (field = conic)]
        Conic(Box<ConicAny>),
        #[holder(use_place_holder)]
        # [holder (field = line)]
        Line(Box<Line>),
        #[holder(use_place_holder)]
        # [holder (field = offset_curve_2d)]
        OffsetCurve2D(Box<OffsetCurve2D>),
    }
    impl Into<CurveAny> for Curve {
        fn into(self) -> CurveAny {
            CurveAny::Curve(Box::new(self))
        }
    }
    impl Into<CurveAny> for BoundedCurve {
        fn into(self) -> CurveAny {
            CurveAny::BoundedCurve(Box::new(self.into()))
        }
    }
    impl Into<CurveAny> for Conic {
        fn into(self) -> CurveAny {
            CurveAny::Conic(Box::new(self.into()))
        }
    }
    impl Into<CurveAny> for Line {
        fn into(self) -> CurveAny {
            CurveAny::Line(Box::new(self.into()))
        }
    }
    impl Into<CurveAny> for OffsetCurve2D {
        fn into(self) -> CurveAny {
            CurveAny::OffsetCurve2D(Box::new(self.into()))
        }
    }
    impl AsRef<Curve> for CurveAny {
        fn as_ref(&self) -> &Curve {
            match self {
                CurveAny::Curve(x) => x.as_ref(),
                CurveAny::BoundedCurve(x) => (**x).as_ref(),
                CurveAny::Conic(x) => (**x).as_ref(),
                CurveAny::Line(x) => (**x).as_ref(),
                CurveAny::OffsetCurve2D(x) => (**x).as_ref(),
            }
        }
    }
    impl AsRef<GeometricRepresentationItem> for CurveAny {
        fn as_ref(&self) -> &GeometricRepresentationItem {
            match self {
                CurveAny::Curve(x) => AsRef::<Curve>::as_ref(x).as_ref(),
                CurveAny::BoundedCurve(x) => AsRef::<Curve>::as_ref(x.as_ref()).as_ref(),
                CurveAny::Conic(x) => AsRef::<Curve>::as_ref(x.as_ref()).as_ref(),
                CurveAny::Line(x) => AsRef::<Curve>::as_ref(x.as_ref()).as_ref(),
                CurveAny::OffsetCurve2D(x) => AsRef::<Curve>::as_ref(x.as_ref()).as_ref(),
            }
        }
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = curve_dimension)]
    #[holder(generate_deserialize)]
    pub struct CurveDimension {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub dimension_curve_directed_callout: DimensionCurveDirectedCallout,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = curve_style)]
    #[holder(generate_deserialize)]
    pub struct CurveStyle {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub curve_font: CurveFontOrScaledCurveFontSelect,
        #[holder(use_place_holder)]
        pub curve_width: SizeSelect,
        #[holder(use_place_holder)]
        pub curve_colour: ColourAny,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = curve_style_font)]
    #[holder(generate_deserialize)]
    pub struct CurveStyleFont {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub pattern_list: Vec<CurveStyleFontPattern>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = curve_style_font_pattern)]
    #[holder(generate_deserialize)]
    pub struct CurveStyleFontPattern {
        #[holder(use_place_holder)]
        pub visible_segment_length: PositiveLengthMeasure,
        #[holder(use_place_holder)]
        pub invisible_segment_length: PositiveLengthMeasure,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = date)]
    #[holder(generate_deserialize)]
    pub struct Date {
        #[holder(use_place_holder)]
        pub year_component: YearNumber,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum DateAny {
        #[holder(use_place_holder)]
        # [holder (field = date)]
        Date(Box<Date>),
        #[holder(use_place_holder)]
        # [holder (field = calendar_date)]
        CalendarDate(Box<CalendarDate>),
    }
    impl Into<DateAny> for Date {
        fn into(self) -> DateAny {
            DateAny::Date(Box::new(self))
        }
    }
    impl Into<DateAny> for CalendarDate {
        fn into(self) -> DateAny {
            DateAny::CalendarDate(Box::new(self.into()))
        }
    }
    impl AsRef<Date> for DateAny {
        fn as_ref(&self) -> &Date {
            match self {
                DateAny::Date(x) => x.as_ref(),
                DateAny::CalendarDate(x) => (**x).as_ref(),
            }
        }
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = datum_feature_callout)]
    #[holder(generate_deserialize)]
    pub struct DatumFeatureCallout {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub draughting_callout: DraughtingCallout,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = datum_target_callout)]
    #[holder(generate_deserialize)]
    pub struct DatumTargetCallout {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub draughting_callout: DraughtingCallout,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = defined_symbol)]
    #[holder(generate_deserialize)]
    pub struct DefinedSymbol {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub geometric_representation_item: GeometricRepresentationItem,
        #[holder(use_place_holder)]
        pub definition: DefinedSymbolSelect,
        #[holder(use_place_holder)]
        pub target: SymbolTarget,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = diameter_dimension)]
    #[holder(generate_deserialize)]
    pub struct DiameterDimension {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub dimension_curve_directed_callout: DimensionCurveDirectedCallout,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = dimension_callout_component_relationship)]
    #[holder(generate_deserialize)]
    pub struct DimensionCalloutComponentRelationship {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub draughting_callout_relationship: DraughtingCalloutRelationship,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = dimension_callout_relationship)]
    #[holder(generate_deserialize)]
    pub struct DimensionCalloutRelationship {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub draughting_callout_relationship: DraughtingCalloutRelationship,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = dimension_curve)]
    #[holder(generate_deserialize)]
    pub struct DimensionCurve {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub annotation_curve_occurrence: AnnotationCurveOccurrence,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = dimension_curve_directed_callout)]
    #[holder(generate_deserialize)]
    pub struct DimensionCurveDirectedCallout {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub draughting_callout: DraughtingCallout,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum DimensionCurveDirectedCalloutAny {
        #[holder(use_place_holder)]
        # [holder (field = dimension_curve_directed_callout)]
        DimensionCurveDirectedCallout(Box<DimensionCurveDirectedCallout>),
        #[holder(use_place_holder)]
        # [holder (field = angular_dimension)]
        AngularDimension(Box<AngularDimension>),
        #[holder(use_place_holder)]
        # [holder (field = curve_dimension)]
        CurveDimension(Box<CurveDimension>),
        #[holder(use_place_holder)]
        # [holder (field = diameter_dimension)]
        DiameterDimension(Box<DiameterDimension>),
        #[holder(use_place_holder)]
        # [holder (field = linear_dimension)]
        LinearDimension(Box<LinearDimension>),
        #[holder(use_place_holder)]
        # [holder (field = radius_dimension)]
        RadiusDimension(Box<RadiusDimension>),
    }
    impl Into<DimensionCurveDirectedCalloutAny> for DimensionCurveDirectedCallout {
        fn into(self) -> DimensionCurveDirectedCalloutAny {
            DimensionCurveDirectedCalloutAny::DimensionCurveDirectedCallout(Box::new(self))
        }
    }
    impl Into<DimensionCurveDirectedCalloutAny> for AngularDimension {
        fn into(self) -> DimensionCurveDirectedCalloutAny {
            DimensionCurveDirectedCalloutAny::AngularDimension(Box::new(self.into()))
        }
    }
    impl Into<DimensionCurveDirectedCalloutAny> for CurveDimension {
        fn into(self) -> DimensionCurveDirectedCalloutAny {
            DimensionCurveDirectedCalloutAny::CurveDimension(Box::new(self.into()))
        }
    }
    impl Into<DimensionCurveDirectedCalloutAny> for DiameterDimension {
        fn into(self) -> DimensionCurveDirectedCalloutAny {
            DimensionCurveDirectedCalloutAny::DiameterDimension(Box::new(self.into()))
        }
    }
    impl Into<DimensionCurveDirectedCalloutAny> for LinearDimension {
        fn into(self) -> DimensionCurveDirectedCalloutAny {
            DimensionCurveDirectedCalloutAny::LinearDimension(Box::new(self.into()))
        }
    }
    impl Into<DimensionCurveDirectedCalloutAny> for RadiusDimension {
        fn into(self) -> DimensionCurveDirectedCalloutAny {
            DimensionCurveDirectedCalloutAny::RadiusDimension(Box::new(self.into()))
        }
    }
    impl AsRef<DimensionCurveDirectedCallout> for DimensionCurveDirectedCalloutAny {
        fn as_ref(&self) -> &DimensionCurveDirectedCallout {
            match self {
                DimensionCurveDirectedCalloutAny::DimensionCurveDirectedCallout(x) => x.as_ref(),
                DimensionCurveDirectedCalloutAny::AngularDimension(x) => (**x).as_ref(),
                DimensionCurveDirectedCalloutAny::CurveDimension(x) => (**x).as_ref(),
                DimensionCurveDirectedCalloutAny::DiameterDimension(x) => (**x).as_ref(),
                DimensionCurveDirectedCalloutAny::LinearDimension(x) => (**x).as_ref(),
                DimensionCurveDirectedCalloutAny::RadiusDimension(x) => (**x).as_ref(),
            }
        }
    }
    impl AsRef<DraughtingCallout> for DimensionCurveDirectedCalloutAny {
        fn as_ref(&self) -> &DraughtingCallout {
            match self {
                DimensionCurveDirectedCalloutAny::DimensionCurveDirectedCallout(x) => {
                    AsRef::<DimensionCurveDirectedCallout>::as_ref(x).as_ref()
                }
                DimensionCurveDirectedCalloutAny::AngularDimension(x) => {
                    AsRef::<DimensionCurveDirectedCallout>::as_ref(x.as_ref()).as_ref()
                }
                DimensionCurveDirectedCalloutAny::CurveDimension(x) => {
                    AsRef::<DimensionCurveDirectedCallout>::as_ref(x.as_ref()).as_ref()
                }
                DimensionCurveDirectedCalloutAny::DiameterDimension(x) => {
                    AsRef::<DimensionCurveDirectedCallout>::as_ref(x.as_ref()).as_ref()
                }
                DimensionCurveDirectedCalloutAny::LinearDimension(x) => {
                    AsRef::<DimensionCurveDirectedCallout>::as_ref(x.as_ref()).as_ref()
                }
                DimensionCurveDirectedCalloutAny::RadiusDimension(x) => {
                    AsRef::<DimensionCurveDirectedCallout>::as_ref(x.as_ref()).as_ref()
                }
            }
        }
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = dimension_curve_terminator)]
    #[holder(generate_deserialize)]
    pub struct DimensionCurveTerminator {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub terminator_symbol: TerminatorSymbol,
        pub role: DimensionExtentUsage,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = dimension_pair)]
    #[holder(generate_deserialize)]
    pub struct DimensionPair {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub draughting_callout_relationship: DraughtingCalloutRelationship,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = dimensional_exponents)]
    #[holder(generate_deserialize)]
    pub struct DimensionalExponents {
        pub length_exponent: f64,
        pub mass_exponent: f64,
        pub time_exponent: f64,
        pub electric_current_exponent: f64,
        pub thermodynamic_temperature_exponent: f64,
        pub amount_of_substance_exponent: f64,
        pub luminous_intensity_exponent: f64,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = direction)]
    #[holder(generate_deserialize)]
    pub struct Direction {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub geometric_representation_item: GeometricRepresentationItem,
        pub direction_ratios: Vec<f64>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = document)]
    #[holder(generate_deserialize)]
    pub struct Document {
        #[holder(use_place_holder)]
        pub id: Identifier,
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub description: Text,
        #[holder(use_place_holder)]
        pub kind: DocumentType,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = document_reference)]
    #[holder(generate_deserialize)]
    pub struct DocumentReference {
        #[holder(use_place_holder)]
        pub assigned_document: Document,
        #[holder(use_place_holder)]
        pub source: Label,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum DocumentReferenceAny {
        #[holder(use_place_holder)]
        # [holder (field = document_reference)]
        DocumentReference(Box<DocumentReference>),
        #[holder(use_place_holder)]
        # [holder (field = draughting_specification_reference)]
        DraughtingSpecificationReference(Box<DraughtingSpecificationReference>),
    }
    impl Into<DocumentReferenceAny> for DocumentReference {
        fn into(self) -> DocumentReferenceAny {
            DocumentReferenceAny::DocumentReference(Box::new(self))
        }
    }
    impl Into<DocumentReferenceAny> for DraughtingSpecificationReference {
        fn into(self) -> DocumentReferenceAny {
            DocumentReferenceAny::DraughtingSpecificationReference(Box::new(self.into()))
        }
    }
    impl AsRef<DocumentReference> for DocumentReferenceAny {
        fn as_ref(&self) -> &DocumentReference {
            match self {
                DocumentReferenceAny::DocumentReference(x) => x.as_ref(),
                DocumentReferenceAny::DraughtingSpecificationReference(x) => (**x).as_ref(),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = document_type)]
    #[holder(generate_deserialize)]
    pub struct DocumentType {
        #[holder(use_place_holder)]
        pub product_data_type: Label,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = draughting_annotation_occurrence)]
    #[holder(generate_deserialize)]
    pub struct DraughtingAnnotationOccurrence {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub annotation_occurrence: AnnotationOccurrence,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = draughting_approval_assignment)]
    #[holder(generate_deserialize)]
    pub struct DraughtingApprovalAssignment {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub approval_assignment: ApprovalAssignment,
        #[holder(use_place_holder)]
        pub approved_items: Vec<ApprovedItem>,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = draughting_callout)]
    #[holder(generate_deserialize)]
    pub struct DraughtingCallout {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub geometric_representation_item: GeometricRepresentationItem,
        #[holder(use_place_holder)]
        pub contents: Vec<DraughtingCalloutElement>,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum DraughtingCalloutAny {
        #[holder(use_place_holder)]
        # [holder (field = draughting_callout)]
        DraughtingCallout(Box<DraughtingCallout>),
        #[holder(use_place_holder)]
        # [holder (field = datum_feature_callout)]
        DatumFeatureCallout(Box<DatumFeatureCallout>),
        #[holder(use_place_holder)]
        # [holder (field = datum_target_callout)]
        DatumTargetCallout(Box<DatumTargetCallout>),
        #[holder(use_place_holder)]
        # [holder (field = dimension_curve_directed_callout)]
        DimensionCurveDirectedCallout(Box<DimensionCurveDirectedCalloutAny>),
        #[holder(use_place_holder)]
        # [holder (field = draughting_elements)]
        DraughtingElements(Box<DraughtingElements>),
        #[holder(use_place_holder)]
        # [holder (field = geometrical_tolerance_callout)]
        GeometricalToleranceCallout(Box<GeometricalToleranceCallout>),
        #[holder(use_place_holder)]
        # [holder (field = leader_directed_callout)]
        LeaderDirectedCallout(Box<LeaderDirectedCalloutAny>),
        #[holder(use_place_holder)]
        # [holder (field = projection_directed_callout)]
        ProjectionDirectedCallout(Box<ProjectionDirectedCalloutAny>),
        #[holder(use_place_holder)]
        # [holder (field = structured_dimension_callout)]
        StructuredDimensionCallout(Box<StructuredDimensionCallout>),
    }
    impl Into<DraughtingCalloutAny> for DraughtingCallout {
        fn into(self) -> DraughtingCalloutAny {
            DraughtingCalloutAny::DraughtingCallout(Box::new(self))
        }
    }
    impl Into<DraughtingCalloutAny> for DatumFeatureCallout {
        fn into(self) -> DraughtingCalloutAny {
            DraughtingCalloutAny::DatumFeatureCallout(Box::new(self.into()))
        }
    }
    impl Into<DraughtingCalloutAny> for DatumTargetCallout {
        fn into(self) -> DraughtingCalloutAny {
            DraughtingCalloutAny::DatumTargetCallout(Box::new(self.into()))
        }
    }
    impl Into<DraughtingCalloutAny> for DimensionCurveDirectedCallout {
        fn into(self) -> DraughtingCalloutAny {
            DraughtingCalloutAny::DimensionCurveDirectedCallout(Box::new(self.into()))
        }
    }
    impl Into<DraughtingCalloutAny> for DraughtingElements {
        fn into(self) -> DraughtingCalloutAny {
            DraughtingCalloutAny::DraughtingElements(Box::new(self.into()))
        }
    }
    impl Into<DraughtingCalloutAny> for GeometricalToleranceCallout {
        fn into(self) -> DraughtingCalloutAny {
            DraughtingCalloutAny::GeometricalToleranceCallout(Box::new(self.into()))
        }
    }
    impl Into<DraughtingCalloutAny> for LeaderDirectedCallout {
        fn into(self) -> DraughtingCalloutAny {
            DraughtingCalloutAny::LeaderDirectedCallout(Box::new(self.into()))
        }
    }
    impl Into<DraughtingCalloutAny> for ProjectionDirectedCallout {
        fn into(self) -> DraughtingCalloutAny {
            DraughtingCalloutAny::ProjectionDirectedCallout(Box::new(self.into()))
        }
    }
    impl Into<DraughtingCalloutAny> for StructuredDimensionCallout {
        fn into(self) -> DraughtingCalloutAny {
            DraughtingCalloutAny::StructuredDimensionCallout(Box::new(self.into()))
        }
    }
    impl AsRef<DraughtingCallout> for DraughtingCalloutAny {
        fn as_ref(&self) -> &DraughtingCallout {
            match self {
                DraughtingCalloutAny::DraughtingCallout(x) => x.as_ref(),
                DraughtingCalloutAny::DatumFeatureCallout(x) => (**x).as_ref(),
                DraughtingCalloutAny::DatumTargetCallout(x) => (**x).as_ref(),
                DraughtingCalloutAny::DimensionCurveDirectedCallout(x) => (**x).as_ref(),
                DraughtingCalloutAny::DraughtingElements(x) => (**x).as_ref(),
                DraughtingCalloutAny::GeometricalToleranceCallout(x) => (**x).as_ref(),
                DraughtingCalloutAny::LeaderDirectedCallout(x) => (**x).as_ref(),
                DraughtingCalloutAny::ProjectionDirectedCallout(x) => (**x).as_ref(),
                DraughtingCalloutAny::StructuredDimensionCallout(x) => (**x).as_ref(),
            }
        }
    }
    impl AsRef<GeometricRepresentationItem> for DraughtingCalloutAny {
        fn as_ref(&self) -> &GeometricRepresentationItem {
            match self {
                DraughtingCalloutAny::DraughtingCallout(x) => {
                    AsRef::<DraughtingCallout>::as_ref(x).as_ref()
                }
                DraughtingCalloutAny::DatumFeatureCallout(x) => {
                    AsRef::<DraughtingCallout>::as_ref(x.as_ref()).as_ref()
                }
                DraughtingCalloutAny::DatumTargetCallout(x) => {
                    AsRef::<DraughtingCallout>::as_ref(x.as_ref()).as_ref()
                }
                DraughtingCalloutAny::DimensionCurveDirectedCallout(x) => {
                    AsRef::<DraughtingCallout>::as_ref(x.as_ref()).as_ref()
                }
                DraughtingCalloutAny::DraughtingElements(x) => {
                    AsRef::<DraughtingCallout>::as_ref(x.as_ref()).as_ref()
                }
                DraughtingCalloutAny::GeometricalToleranceCallout(x) => {
                    AsRef::<DraughtingCallout>::as_ref(x.as_ref()).as_ref()
                }
                DraughtingCalloutAny::LeaderDirectedCallout(x) => {
                    AsRef::<DraughtingCallout>::as_ref(x.as_ref()).as_ref()
                }
                DraughtingCalloutAny::ProjectionDirectedCallout(x) => {
                    AsRef::<DraughtingCallout>::as_ref(x.as_ref()).as_ref()
                }
                DraughtingCalloutAny::StructuredDimensionCallout(x) => {
                    AsRef::<DraughtingCallout>::as_ref(x.as_ref()).as_ref()
                }
            }
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = draughting_callout_relationship)]
    #[holder(generate_deserialize)]
    pub struct DraughtingCalloutRelationship {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub description: Text,
        #[holder(use_place_holder)]
        pub relating_draughting_callout: DraughtingCalloutAny,
        #[holder(use_place_holder)]
        pub related_draughting_callout: DraughtingCalloutAny,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum DraughtingCalloutRelationshipAny {
        #[holder(use_place_holder)]
        # [holder (field = draughting_callout_relationship)]
        DraughtingCalloutRelationship(Box<DraughtingCalloutRelationship>),
        #[holder(use_place_holder)]
        # [holder (field = dimension_callout_component_relationship)]
        DimensionCalloutComponentRelationship(Box<DimensionCalloutComponentRelationship>),
        #[holder(use_place_holder)]
        # [holder (field = dimension_callout_relationship)]
        DimensionCalloutRelationship(Box<DimensionCalloutRelationship>),
        #[holder(use_place_holder)]
        # [holder (field = dimension_pair)]
        DimensionPair(Box<DimensionPair>),
    }
    impl Into<DraughtingCalloutRelationshipAny> for DraughtingCalloutRelationship {
        fn into(self) -> DraughtingCalloutRelationshipAny {
            DraughtingCalloutRelationshipAny::DraughtingCalloutRelationship(Box::new(self))
        }
    }
    impl Into<DraughtingCalloutRelationshipAny> for DimensionCalloutComponentRelationship {
        fn into(self) -> DraughtingCalloutRelationshipAny {
            DraughtingCalloutRelationshipAny::DimensionCalloutComponentRelationship(Box::new(
                self.into(),
            ))
        }
    }
    impl Into<DraughtingCalloutRelationshipAny> for DimensionCalloutRelationship {
        fn into(self) -> DraughtingCalloutRelationshipAny {
            DraughtingCalloutRelationshipAny::DimensionCalloutRelationship(Box::new(self.into()))
        }
    }
    impl Into<DraughtingCalloutRelationshipAny> for DimensionPair {
        fn into(self) -> DraughtingCalloutRelationshipAny {
            DraughtingCalloutRelationshipAny::DimensionPair(Box::new(self.into()))
        }
    }
    impl AsRef<DraughtingCalloutRelationship> for DraughtingCalloutRelationshipAny {
        fn as_ref(&self) -> &DraughtingCalloutRelationship {
            match self {
                DraughtingCalloutRelationshipAny::DraughtingCalloutRelationship(x) => x.as_ref(),
                DraughtingCalloutRelationshipAny::DimensionCalloutComponentRelationship(x) => {
                    (**x).as_ref()
                }
                DraughtingCalloutRelationshipAny::DimensionCalloutRelationship(x) => (**x).as_ref(),
                DraughtingCalloutRelationshipAny::DimensionPair(x) => (**x).as_ref(),
            }
        }
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = draughting_contract_assignment)]
    #[holder(generate_deserialize)]
    pub struct DraughtingContractAssignment {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub contract_assignment: ContractAssignment,
        #[holder(use_place_holder)]
        pub items: Vec<ContractedItem>,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = draughting_drawing_revision)]
    #[holder(generate_deserialize)]
    pub struct DraughtingDrawingRevision {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub drawing_revision: DrawingRevision,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = draughting_elements)]
    #[holder(generate_deserialize)]
    pub struct DraughtingElements {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub draughting_callout: DraughtingCallout,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = draughting_group_assignment)]
    #[holder(generate_deserialize)]
    pub struct DraughtingGroupAssignment {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub group_assignment: GroupAssignment,
        #[holder(use_place_holder)]
        pub items: Vec<DraughtingGroupedItem>,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = draughting_model)]
    #[holder(generate_deserialize)]
    pub struct DraughtingModel {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub representation: Representation,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = draughting_organization_assignment)]
    #[holder(generate_deserialize)]
    pub struct DraughtingOrganizationAssignment {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub organization_assignment: OrganizationAssignment,
        #[holder(use_place_holder)]
        pub assigned_items: Vec<DraughtingOrganizationItem>,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = draughting_person_and_organization_assignment)]
    #[holder(generate_deserialize)]
    pub struct DraughtingPersonAndOrganizationAssignment {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub person_and_organization_assignment: PersonAndOrganizationAssignment,
        #[holder(use_place_holder)]
        pub assigned_items: Vec<DraughtingOrganizationItem>,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = draughting_person_assignment)]
    #[holder(generate_deserialize)]
    pub struct DraughtingPersonAssignment {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub person_assignment: PersonAssignment,
        #[holder(use_place_holder)]
        pub assigned_items: Vec<DraughtingOrganizationItem>,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = draughting_pre_defined_colour)]
    #[holder(generate_deserialize)]
    pub struct DraughtingPreDefinedColour {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub pre_defined_colour: PreDefinedColour,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = draughting_pre_defined_curve_font)]
    #[holder(generate_deserialize)]
    pub struct DraughtingPreDefinedCurveFont {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub pre_defined_curve_font: PreDefinedCurveFont,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = draughting_pre_defined_text_font)]
    #[holder(generate_deserialize)]
    pub struct DraughtingPreDefinedTextFont {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub pre_defined_text_font: PreDefinedTextFont,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = draughting_presented_item)]
    #[holder(generate_deserialize)]
    pub struct DraughtingPresentedItem {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub presented_item: PresentedItem,
        #[holder(use_place_holder)]
        pub items: Vec<DraughtingPresentedItemSelect>,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = draughting_security_classification_assignment)]
    #[holder(generate_deserialize)]
    pub struct DraughtingSecurityClassificationAssignment {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub security_classification_assignment: SecurityClassificationAssignment,
        #[holder(use_place_holder)]
        pub assigned_items: Vec<ClassifiedItem>,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = draughting_specification_reference)]
    #[holder(generate_deserialize)]
    pub struct DraughtingSpecificationReference {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub document_reference: DocumentReference,
        #[holder(use_place_holder)]
        pub specified_items: Vec<SpecifiedItem>,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = draughting_subfigure_representation)]
    #[holder(generate_deserialize)]
    pub struct DraughtingSubfigureRepresentation {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub symbol_representation: SymbolRepresentation,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = draughting_symbol_representation)]
    #[holder(generate_deserialize)]
    pub struct DraughtingSymbolRepresentation {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub symbol_representation: SymbolRepresentation,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum DraughtingSymbolRepresentationAny {
        #[holder(use_place_holder)]
        # [holder (field = draughting_symbol_representation)]
        DraughtingSymbolRepresentation(Box<DraughtingSymbolRepresentation>),
        #[holder(use_place_holder)]
        # [holder (field = drawing_sheet_layout)]
        DrawingSheetLayout(Box<DrawingSheetLayout>),
    }
    impl Into<DraughtingSymbolRepresentationAny> for DraughtingSymbolRepresentation {
        fn into(self) -> DraughtingSymbolRepresentationAny {
            DraughtingSymbolRepresentationAny::DraughtingSymbolRepresentation(Box::new(self))
        }
    }
    impl Into<DraughtingSymbolRepresentationAny> for DrawingSheetLayout {
        fn into(self) -> DraughtingSymbolRepresentationAny {
            DraughtingSymbolRepresentationAny::DrawingSheetLayout(Box::new(self.into()))
        }
    }
    impl AsRef<DraughtingSymbolRepresentation> for DraughtingSymbolRepresentationAny {
        fn as_ref(&self) -> &DraughtingSymbolRepresentation {
            match self {
                DraughtingSymbolRepresentationAny::DraughtingSymbolRepresentation(x) => x.as_ref(),
                DraughtingSymbolRepresentationAny::DrawingSheetLayout(x) => (**x).as_ref(),
            }
        }
    }
    impl AsRef<SymbolRepresentation> for DraughtingSymbolRepresentationAny {
        fn as_ref(&self) -> &SymbolRepresentation {
            match self {
                DraughtingSymbolRepresentationAny::DraughtingSymbolRepresentation(x) => {
                    AsRef::<DraughtingSymbolRepresentation>::as_ref(x).as_ref()
                }
                DraughtingSymbolRepresentationAny::DrawingSheetLayout(x) => {
                    AsRef::<DraughtingSymbolRepresentation>::as_ref(x.as_ref()).as_ref()
                }
            }
        }
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = draughting_text_literal_with_delineation)]
    #[holder(generate_deserialize)]
    pub struct DraughtingTextLiteralWithDelineation {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub text_literal_with_delineation: TextLiteralWithDelineation,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = draughting_title)]
    #[holder(generate_deserialize)]
    pub struct DraughtingTitle {
        #[holder(use_place_holder)]
        pub items: Vec<DraughtingTitledItem>,
        #[holder(use_place_holder)]
        pub language: Label,
        #[holder(use_place_holder)]
        pub contents: Text,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = drawing_definition)]
    #[holder(generate_deserialize)]
    pub struct DrawingDefinition {
        #[holder(use_place_holder)]
        pub drawing_number: Identifier,
        #[holder(use_place_holder)]
        pub drawing_type: Option<Label>,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = drawing_revision)]
    #[holder(generate_deserialize)]
    pub struct DrawingRevision {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub presentation_set: PresentationSet,
        #[holder(use_place_holder)]
        pub revision_identifier: Identifier,
        #[holder(use_place_holder)]
        pub drawing_identifier: DrawingDefinition,
        #[holder(use_place_holder)]
        pub intended_scale: Option<Text>,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum DrawingRevisionAny {
        #[holder(use_place_holder)]
        # [holder (field = drawing_revision)]
        DrawingRevision(Box<DrawingRevision>),
        #[holder(use_place_holder)]
        # [holder (field = draughting_drawing_revision)]
        DraughtingDrawingRevision(Box<DraughtingDrawingRevision>),
    }
    impl Into<DrawingRevisionAny> for DrawingRevision {
        fn into(self) -> DrawingRevisionAny {
            DrawingRevisionAny::DrawingRevision(Box::new(self))
        }
    }
    impl Into<DrawingRevisionAny> for DraughtingDrawingRevision {
        fn into(self) -> DrawingRevisionAny {
            DrawingRevisionAny::DraughtingDrawingRevision(Box::new(self.into()))
        }
    }
    impl AsRef<DrawingRevision> for DrawingRevisionAny {
        fn as_ref(&self) -> &DrawingRevision {
            match self {
                DrawingRevisionAny::DrawingRevision(x) => x.as_ref(),
                DrawingRevisionAny::DraughtingDrawingRevision(x) => (**x).as_ref(),
            }
        }
    }
    impl AsRef<PresentationSet> for DrawingRevisionAny {
        fn as_ref(&self) -> &PresentationSet {
            match self {
                DrawingRevisionAny::DrawingRevision(x) => {
                    AsRef::<DrawingRevision>::as_ref(x).as_ref()
                }
                DrawingRevisionAny::DraughtingDrawingRevision(x) => {
                    AsRef::<DrawingRevision>::as_ref(x.as_ref()).as_ref()
                }
            }
        }
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = drawing_sheet_layout)]
    #[holder(generate_deserialize)]
    pub struct DrawingSheetLayout {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub draughting_symbol_representation: DraughtingSymbolRepresentation,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = drawing_sheet_revision)]
    #[holder(generate_deserialize)]
    pub struct DrawingSheetRevision {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub presentation_area: PresentationArea,
        #[holder(use_place_holder)]
        pub revision_identifier: Identifier,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = drawing_sheet_revision_usage)]
    #[holder(generate_deserialize)]
    pub struct DrawingSheetRevisionUsage {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub area_in_set: AreaInSet,
        #[holder(use_place_holder)]
        pub sheet_number: Identifier,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = ellipse)]
    #[holder(generate_deserialize)]
    pub struct Ellipse {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub conic: Conic,
        #[holder(use_place_holder)]
        pub semi_axis_1: PositiveLengthMeasure,
        #[holder(use_place_holder)]
        pub semi_axis_2: PositiveLengthMeasure,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = external_source)]
    #[holder(generate_deserialize)]
    pub struct ExternalSource {
        #[holder(use_place_holder)]
        pub source_id: SourceItem,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = externally_defined_curve_font)]
    #[holder(generate_deserialize)]
    pub struct ExternallyDefinedCurveFont {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub externally_defined_item: ExternallyDefinedItem,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut)]
    # [holder (table = Tables)]
    # [holder (field = externally_defined_hatch_style)]
    #[holder(generate_deserialize)]
    pub struct ExternallyDefinedHatchStyle {
        #[as_ref]
        #[as_mut]
        #[holder(use_place_holder)]
        pub externally_defined_item: ExternallyDefinedItem,
        #[as_ref]
        #[as_mut]
        #[holder(use_place_holder)]
        pub geometric_representation_item: GeometricRepresentationItem,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = externally_defined_item)]
    #[holder(generate_deserialize)]
    pub struct ExternallyDefinedItem {
        #[holder(use_place_holder)]
        pub item_id: SourceItem,
        #[holder(use_place_holder)]
        pub source: ExternalSource,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ExternallyDefinedItemAny {
        #[holder(use_place_holder)]
        # [holder (field = externally_defined_item)]
        ExternallyDefinedItem(Box<ExternallyDefinedItem>),
        #[holder(use_place_holder)]
        # [holder (field = externally_defined_curve_font)]
        ExternallyDefinedCurveFont(Box<ExternallyDefinedCurveFont>),
        #[holder(use_place_holder)]
        # [holder (field = externally_defined_hatch_style)]
        ExternallyDefinedHatchStyle(Box<ExternallyDefinedHatchStyle>),
        #[holder(use_place_holder)]
        # [holder (field = externally_defined_symbol)]
        ExternallyDefinedSymbol(Box<ExternallyDefinedSymbol>),
        #[holder(use_place_holder)]
        # [holder (field = externally_defined_text_font)]
        ExternallyDefinedTextFont(Box<ExternallyDefinedTextFont>),
        #[holder(use_place_holder)]
        # [holder (field = externally_defined_tile_style)]
        ExternallyDefinedTileStyle(Box<ExternallyDefinedTileStyle>),
    }
    impl Into<ExternallyDefinedItemAny> for ExternallyDefinedItem {
        fn into(self) -> ExternallyDefinedItemAny {
            ExternallyDefinedItemAny::ExternallyDefinedItem(Box::new(self))
        }
    }
    impl Into<ExternallyDefinedItemAny> for ExternallyDefinedCurveFont {
        fn into(self) -> ExternallyDefinedItemAny {
            ExternallyDefinedItemAny::ExternallyDefinedCurveFont(Box::new(self.into()))
        }
    }
    impl Into<ExternallyDefinedItemAny> for ExternallyDefinedHatchStyle {
        fn into(self) -> ExternallyDefinedItemAny {
            ExternallyDefinedItemAny::ExternallyDefinedHatchStyle(Box::new(self.into()))
        }
    }
    impl Into<ExternallyDefinedItemAny> for ExternallyDefinedSymbol {
        fn into(self) -> ExternallyDefinedItemAny {
            ExternallyDefinedItemAny::ExternallyDefinedSymbol(Box::new(self.into()))
        }
    }
    impl Into<ExternallyDefinedItemAny> for ExternallyDefinedTextFont {
        fn into(self) -> ExternallyDefinedItemAny {
            ExternallyDefinedItemAny::ExternallyDefinedTextFont(Box::new(self.into()))
        }
    }
    impl Into<ExternallyDefinedItemAny> for ExternallyDefinedTileStyle {
        fn into(self) -> ExternallyDefinedItemAny {
            ExternallyDefinedItemAny::ExternallyDefinedTileStyle(Box::new(self.into()))
        }
    }
    impl AsRef<ExternallyDefinedItem> for ExternallyDefinedItemAny {
        fn as_ref(&self) -> &ExternallyDefinedItem {
            match self {
                ExternallyDefinedItemAny::ExternallyDefinedItem(x) => x.as_ref(),
                ExternallyDefinedItemAny::ExternallyDefinedCurveFont(x) => (**x).as_ref(),
                ExternallyDefinedItemAny::ExternallyDefinedHatchStyle(x) => (**x).as_ref(),
                ExternallyDefinedItemAny::ExternallyDefinedSymbol(x) => (**x).as_ref(),
                ExternallyDefinedItemAny::ExternallyDefinedTextFont(x) => (**x).as_ref(),
                ExternallyDefinedItemAny::ExternallyDefinedTileStyle(x) => (**x).as_ref(),
            }
        }
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = externally_defined_symbol)]
    #[holder(generate_deserialize)]
    pub struct ExternallyDefinedSymbol {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub externally_defined_item: ExternallyDefinedItem,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = externally_defined_text_font)]
    #[holder(generate_deserialize)]
    pub struct ExternallyDefinedTextFont {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub externally_defined_item: ExternallyDefinedItem,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut)]
    # [holder (table = Tables)]
    # [holder (field = externally_defined_tile_style)]
    #[holder(generate_deserialize)]
    pub struct ExternallyDefinedTileStyle {
        #[as_ref]
        #[as_mut]
        #[holder(use_place_holder)]
        pub externally_defined_item: ExternallyDefinedItem,
        #[as_ref]
        #[as_mut]
        #[holder(use_place_holder)]
        pub geometric_representation_item: GeometricRepresentationItem,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = fill_area_style)]
    #[holder(generate_deserialize)]
    pub struct FillAreaStyle {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub fill_styles: Vec<FillStyleSelect>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = fill_area_style_colour)]
    #[holder(generate_deserialize)]
    pub struct FillAreaStyleColour {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub fill_colour: ColourAny,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = fill_area_style_hatching)]
    #[holder(generate_deserialize)]
    pub struct FillAreaStyleHatching {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub geometric_representation_item: GeometricRepresentationItem,
        #[holder(use_place_holder)]
        pub hatch_line_appearance: CurveStyle,
        #[holder(use_place_holder)]
        pub start_of_next_hatch_line: OneDirectionRepeatFactorAny,
        #[holder(use_place_holder)]
        pub point_of_reference_hatch_line: CartesianPoint,
        #[holder(use_place_holder)]
        pub pattern_start: CartesianPoint,
        #[holder(use_place_holder)]
        pub hatch_line_angle: PlaneAngleMeasure,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = fill_area_style_tile_symbol_with_style)]
    #[holder(generate_deserialize)]
    pub struct FillAreaStyleTileSymbolWithStyle {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub geometric_representation_item: GeometricRepresentationItem,
        #[holder(use_place_holder)]
        pub symbol: AnnotationSymbolOccurrenceAny,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = fill_area_style_tiles)]
    #[holder(generate_deserialize)]
    pub struct FillAreaStyleTiles {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub geometric_representation_item: GeometricRepresentationItem,
        #[holder(use_place_holder)]
        pub tiling_pattern: TwoDirectionRepeatFactor,
        #[holder(use_place_holder)]
        pub tiles: Vec<FillAreaStyleTileShapeSelect>,
        #[holder(use_place_holder)]
        pub tiling_scale: PositiveRatioMeasure,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = geometric_curve_set)]
    #[holder(generate_deserialize)]
    pub struct GeometricCurveSet {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub geometric_set: GeometricSet,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = geometric_representation_context)]
    #[holder(generate_deserialize)]
    pub struct GeometricRepresentationContext {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub representation_context: RepresentationContext,
        #[holder(use_place_holder)]
        pub coordinate_space_dimension: DimensionCount,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = geometric_representation_item)]
    #[holder(generate_deserialize)]
    pub struct GeometricRepresentationItem {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub representation_item: RepresentationItem,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum GeometricRepresentationItemAny {
        #[holder(use_place_holder)]
        # [holder (field = geometric_representation_item)]
        GeometricRepresentationItem(Box<GeometricRepresentationItem>),
        #[holder(use_place_holder)]
        # [holder (field = annotation_fill_area)]
        AnnotationFillArea(Box<AnnotationFillArea>),
        #[holder(use_place_holder)]
        # [holder (field = camera_model)]
        CameraModel(Box<CameraModelAny>),
        #[holder(use_place_holder)]
        # [holder (field = composite_text)]
        CompositeText(Box<CompositeTextAny>),
        #[holder(use_place_holder)]
        # [holder (field = curve)]
        Curve(Box<CurveAny>),
        #[holder(use_place_holder)]
        # [holder (field = defined_symbol)]
        DefinedSymbol(Box<DefinedSymbol>),
        #[holder(use_place_holder)]
        # [holder (field = direction)]
        Direction(Box<Direction>),
        #[holder(use_place_holder)]
        # [holder (field = draughting_callout)]
        DraughtingCallout(Box<DraughtingCalloutAny>),
        #[holder(use_place_holder)]
        # [holder (field = externally_defined_hatch_style)]
        ExternallyDefinedHatchStyle(Box<ExternallyDefinedHatchStyle>),
        #[holder(use_place_holder)]
        # [holder (field = externally_defined_tile_style)]
        ExternallyDefinedTileStyle(Box<ExternallyDefinedTileStyle>),
        #[holder(use_place_holder)]
        # [holder (field = fill_area_style_hatching)]
        FillAreaStyleHatching(Box<FillAreaStyleHatching>),
        #[holder(use_place_holder)]
        # [holder (field = fill_area_style_tile_symbol_with_style)]
        FillAreaStyleTileSymbolWithStyle(Box<FillAreaStyleTileSymbolWithStyle>),
        #[holder(use_place_holder)]
        # [holder (field = fill_area_style_tiles)]
        FillAreaStyleTiles(Box<FillAreaStyleTiles>),
        #[holder(use_place_holder)]
        # [holder (field = geometric_set)]
        GeometricSet(Box<GeometricSetAny>),
        #[holder(use_place_holder)]
        # [holder (field = one_direction_repeat_factor)]
        OneDirectionRepeatFactor(Box<OneDirectionRepeatFactorAny>),
        #[holder(use_place_holder)]
        # [holder (field = placement)]
        Placement(Box<PlacementAny>),
        #[holder(use_place_holder)]
        # [holder (field = planar_extent)]
        PlanarExtent(Box<PlanarExtentAny>),
        #[holder(use_place_holder)]
        # [holder (field = point)]
        Point(Box<PointAny>),
        #[holder(use_place_holder)]
        # [holder (field = symbol_target)]
        SymbolTarget(Box<SymbolTarget>),
        #[holder(use_place_holder)]
        # [holder (field = text_literal)]
        TextLiteral(Box<TextLiteralAny>),
        #[holder(use_place_holder)]
        # [holder (field = vector)]
        Vector(Box<Vector>),
    }
    impl Into<GeometricRepresentationItemAny> for GeometricRepresentationItem {
        fn into(self) -> GeometricRepresentationItemAny {
            GeometricRepresentationItemAny::GeometricRepresentationItem(Box::new(self))
        }
    }
    impl Into<GeometricRepresentationItemAny> for AnnotationFillArea {
        fn into(self) -> GeometricRepresentationItemAny {
            GeometricRepresentationItemAny::AnnotationFillArea(Box::new(self.into()))
        }
    }
    impl Into<GeometricRepresentationItemAny> for CameraModel {
        fn into(self) -> GeometricRepresentationItemAny {
            GeometricRepresentationItemAny::CameraModel(Box::new(self.into()))
        }
    }
    impl Into<GeometricRepresentationItemAny> for CompositeText {
        fn into(self) -> GeometricRepresentationItemAny {
            GeometricRepresentationItemAny::CompositeText(Box::new(self.into()))
        }
    }
    impl Into<GeometricRepresentationItemAny> for Curve {
        fn into(self) -> GeometricRepresentationItemAny {
            GeometricRepresentationItemAny::Curve(Box::new(self.into()))
        }
    }
    impl Into<GeometricRepresentationItemAny> for DefinedSymbol {
        fn into(self) -> GeometricRepresentationItemAny {
            GeometricRepresentationItemAny::DefinedSymbol(Box::new(self.into()))
        }
    }
    impl Into<GeometricRepresentationItemAny> for Direction {
        fn into(self) -> GeometricRepresentationItemAny {
            GeometricRepresentationItemAny::Direction(Box::new(self.into()))
        }
    }
    impl Into<GeometricRepresentationItemAny> for DraughtingCallout {
        fn into(self) -> GeometricRepresentationItemAny {
            GeometricRepresentationItemAny::DraughtingCallout(Box::new(self.into()))
        }
    }
    impl Into<GeometricRepresentationItemAny> for ExternallyDefinedHatchStyle {
        fn into(self) -> GeometricRepresentationItemAny {
            GeometricRepresentationItemAny::ExternallyDefinedHatchStyle(Box::new(self.into()))
        }
    }
    impl Into<GeometricRepresentationItemAny> for ExternallyDefinedTileStyle {
        fn into(self) -> GeometricRepresentationItemAny {
            GeometricRepresentationItemAny::ExternallyDefinedTileStyle(Box::new(self.into()))
        }
    }
    impl Into<GeometricRepresentationItemAny> for FillAreaStyleHatching {
        fn into(self) -> GeometricRepresentationItemAny {
            GeometricRepresentationItemAny::FillAreaStyleHatching(Box::new(self.into()))
        }
    }
    impl Into<GeometricRepresentationItemAny> for FillAreaStyleTileSymbolWithStyle {
        fn into(self) -> GeometricRepresentationItemAny {
            GeometricRepresentationItemAny::FillAreaStyleTileSymbolWithStyle(Box::new(self.into()))
        }
    }
    impl Into<GeometricRepresentationItemAny> for FillAreaStyleTiles {
        fn into(self) -> GeometricRepresentationItemAny {
            GeometricRepresentationItemAny::FillAreaStyleTiles(Box::new(self.into()))
        }
    }
    impl Into<GeometricRepresentationItemAny> for GeometricSet {
        fn into(self) -> GeometricRepresentationItemAny {
            GeometricRepresentationItemAny::GeometricSet(Box::new(self.into()))
        }
    }
    impl Into<GeometricRepresentationItemAny> for OneDirectionRepeatFactor {
        fn into(self) -> GeometricRepresentationItemAny {
            GeometricRepresentationItemAny::OneDirectionRepeatFactor(Box::new(self.into()))
        }
    }
    impl Into<GeometricRepresentationItemAny> for Placement {
        fn into(self) -> GeometricRepresentationItemAny {
            GeometricRepresentationItemAny::Placement(Box::new(self.into()))
        }
    }
    impl Into<GeometricRepresentationItemAny> for PlanarExtent {
        fn into(self) -> GeometricRepresentationItemAny {
            GeometricRepresentationItemAny::PlanarExtent(Box::new(self.into()))
        }
    }
    impl Into<GeometricRepresentationItemAny> for Point {
        fn into(self) -> GeometricRepresentationItemAny {
            GeometricRepresentationItemAny::Point(Box::new(self.into()))
        }
    }
    impl Into<GeometricRepresentationItemAny> for SymbolTarget {
        fn into(self) -> GeometricRepresentationItemAny {
            GeometricRepresentationItemAny::SymbolTarget(Box::new(self.into()))
        }
    }
    impl Into<GeometricRepresentationItemAny> for TextLiteral {
        fn into(self) -> GeometricRepresentationItemAny {
            GeometricRepresentationItemAny::TextLiteral(Box::new(self.into()))
        }
    }
    impl Into<GeometricRepresentationItemAny> for Vector {
        fn into(self) -> GeometricRepresentationItemAny {
            GeometricRepresentationItemAny::Vector(Box::new(self.into()))
        }
    }
    impl AsRef<GeometricRepresentationItem> for GeometricRepresentationItemAny {
        fn as_ref(&self) -> &GeometricRepresentationItem {
            match self {
                GeometricRepresentationItemAny::GeometricRepresentationItem(x) => x.as_ref(),
                GeometricRepresentationItemAny::AnnotationFillArea(x) => (**x).as_ref(),
                GeometricRepresentationItemAny::CameraModel(x) => (**x).as_ref(),
                GeometricRepresentationItemAny::CompositeText(x) => (**x).as_ref(),
                GeometricRepresentationItemAny::Curve(x) => (**x).as_ref(),
                GeometricRepresentationItemAny::DefinedSymbol(x) => (**x).as_ref(),
                GeometricRepresentationItemAny::Direction(x) => (**x).as_ref(),
                GeometricRepresentationItemAny::DraughtingCallout(x) => (**x).as_ref(),
                GeometricRepresentationItemAny::ExternallyDefinedHatchStyle(x) => (**x).as_ref(),
                GeometricRepresentationItemAny::ExternallyDefinedTileStyle(x) => (**x).as_ref(),
                GeometricRepresentationItemAny::FillAreaStyleHatching(x) => (**x).as_ref(),
                GeometricRepresentationItemAny::FillAreaStyleTileSymbolWithStyle(x) => {
                    (**x).as_ref()
                }
                GeometricRepresentationItemAny::FillAreaStyleTiles(x) => (**x).as_ref(),
                GeometricRepresentationItemAny::GeometricSet(x) => (**x).as_ref(),
                GeometricRepresentationItemAny::OneDirectionRepeatFactor(x) => (**x).as_ref(),
                GeometricRepresentationItemAny::Placement(x) => (**x).as_ref(),
                GeometricRepresentationItemAny::PlanarExtent(x) => (**x).as_ref(),
                GeometricRepresentationItemAny::Point(x) => (**x).as_ref(),
                GeometricRepresentationItemAny::SymbolTarget(x) => (**x).as_ref(),
                GeometricRepresentationItemAny::TextLiteral(x) => (**x).as_ref(),
                GeometricRepresentationItemAny::Vector(x) => (**x).as_ref(),
            }
        }
    }
    impl AsRef<RepresentationItem> for GeometricRepresentationItemAny {
        fn as_ref(&self) -> &RepresentationItem {
            match self {
                GeometricRepresentationItemAny::GeometricRepresentationItem(x) => {
                    AsRef::<GeometricRepresentationItem>::as_ref(x).as_ref()
                }
                GeometricRepresentationItemAny::AnnotationFillArea(x) => {
                    AsRef::<GeometricRepresentationItem>::as_ref(x.as_ref()).as_ref()
                }
                GeometricRepresentationItemAny::CameraModel(x) => {
                    AsRef::<GeometricRepresentationItem>::as_ref(x.as_ref()).as_ref()
                }
                GeometricRepresentationItemAny::CompositeText(x) => {
                    AsRef::<GeometricRepresentationItem>::as_ref(x.as_ref()).as_ref()
                }
                GeometricRepresentationItemAny::Curve(x) => {
                    AsRef::<GeometricRepresentationItem>::as_ref(x.as_ref()).as_ref()
                }
                GeometricRepresentationItemAny::DefinedSymbol(x) => {
                    AsRef::<GeometricRepresentationItem>::as_ref(x.as_ref()).as_ref()
                }
                GeometricRepresentationItemAny::Direction(x) => {
                    AsRef::<GeometricRepresentationItem>::as_ref(x.as_ref()).as_ref()
                }
                GeometricRepresentationItemAny::DraughtingCallout(x) => {
                    AsRef::<GeometricRepresentationItem>::as_ref(x.as_ref()).as_ref()
                }
                GeometricRepresentationItemAny::ExternallyDefinedHatchStyle(x) => {
                    AsRef::<GeometricRepresentationItem>::as_ref(x.as_ref()).as_ref()
                }
                GeometricRepresentationItemAny::ExternallyDefinedTileStyle(x) => {
                    AsRef::<GeometricRepresentationItem>::as_ref(x.as_ref()).as_ref()
                }
                GeometricRepresentationItemAny::FillAreaStyleHatching(x) => {
                    AsRef::<GeometricRepresentationItem>::as_ref(x.as_ref()).as_ref()
                }
                GeometricRepresentationItemAny::FillAreaStyleTileSymbolWithStyle(x) => {
                    AsRef::<GeometricRepresentationItem>::as_ref(x.as_ref()).as_ref()
                }
                GeometricRepresentationItemAny::FillAreaStyleTiles(x) => {
                    AsRef::<GeometricRepresentationItem>::as_ref(x.as_ref()).as_ref()
                }
                GeometricRepresentationItemAny::GeometricSet(x) => {
                    AsRef::<GeometricRepresentationItem>::as_ref(x.as_ref()).as_ref()
                }
                GeometricRepresentationItemAny::OneDirectionRepeatFactor(x) => {
                    AsRef::<GeometricRepresentationItem>::as_ref(x.as_ref()).as_ref()
                }
                GeometricRepresentationItemAny::Placement(x) => {
                    AsRef::<GeometricRepresentationItem>::as_ref(x.as_ref()).as_ref()
                }
                GeometricRepresentationItemAny::PlanarExtent(x) => {
                    AsRef::<GeometricRepresentationItem>::as_ref(x.as_ref()).as_ref()
                }
                GeometricRepresentationItemAny::Point(x) => {
                    AsRef::<GeometricRepresentationItem>::as_ref(x.as_ref()).as_ref()
                }
                GeometricRepresentationItemAny::SymbolTarget(x) => {
                    AsRef::<GeometricRepresentationItem>::as_ref(x.as_ref()).as_ref()
                }
                GeometricRepresentationItemAny::TextLiteral(x) => {
                    AsRef::<GeometricRepresentationItem>::as_ref(x.as_ref()).as_ref()
                }
                GeometricRepresentationItemAny::Vector(x) => {
                    AsRef::<GeometricRepresentationItem>::as_ref(x.as_ref()).as_ref()
                }
            }
        }
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = geometric_set)]
    #[holder(generate_deserialize)]
    pub struct GeometricSet {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub geometric_representation_item: GeometricRepresentationItem,
        #[holder(use_place_holder)]
        pub elements: Vec<GeometricSetSelect>,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum GeometricSetAny {
        #[holder(use_place_holder)]
        # [holder (field = geometric_set)]
        GeometricSet(Box<GeometricSet>),
        #[holder(use_place_holder)]
        # [holder (field = geometric_curve_set)]
        GeometricCurveSet(Box<GeometricCurveSet>),
    }
    impl Into<GeometricSetAny> for GeometricSet {
        fn into(self) -> GeometricSetAny {
            GeometricSetAny::GeometricSet(Box::new(self))
        }
    }
    impl Into<GeometricSetAny> for GeometricCurveSet {
        fn into(self) -> GeometricSetAny {
            GeometricSetAny::GeometricCurveSet(Box::new(self.into()))
        }
    }
    impl AsRef<GeometricSet> for GeometricSetAny {
        fn as_ref(&self) -> &GeometricSet {
            match self {
                GeometricSetAny::GeometricSet(x) => x.as_ref(),
                GeometricSetAny::GeometricCurveSet(x) => (**x).as_ref(),
            }
        }
    }
    impl AsRef<GeometricRepresentationItem> for GeometricSetAny {
        fn as_ref(&self) -> &GeometricRepresentationItem {
            match self {
                GeometricSetAny::GeometricSet(x) => AsRef::<GeometricSet>::as_ref(x).as_ref(),
                GeometricSetAny::GeometricCurveSet(x) => {
                    AsRef::<GeometricSet>::as_ref(x.as_ref()).as_ref()
                }
            }
        }
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = geometrical_tolerance_callout)]
    #[holder(generate_deserialize)]
    pub struct GeometricalToleranceCallout {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub draughting_callout: DraughtingCallout,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = geometrically_bounded_2d_wireframe_representation)]
    #[holder(generate_deserialize)]
    pub struct GeometricallyBounded2DWireframeRepresentation {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub shape_representation: ShapeRepresentation,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = global_unit_assigned_context)]
    #[holder(generate_deserialize)]
    pub struct GlobalUnitAssignedContext {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub representation_context: RepresentationContext,
        #[holder(use_place_holder)]
        pub units: Vec<Unit>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = group)]
    #[holder(generate_deserialize)]
    pub struct Group {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub description: Text,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = group_assignment)]
    #[holder(generate_deserialize)]
    pub struct GroupAssignment {
        #[holder(use_place_holder)]
        pub assigned_group: Group,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum GroupAssignmentAny {
        #[holder(use_place_holder)]
        # [holder (field = group_assignment)]
        GroupAssignment(Box<GroupAssignment>),
        #[holder(use_place_holder)]
        # [holder (field = draughting_group_assignment)]
        DraughtingGroupAssignment(Box<DraughtingGroupAssignment>),
    }
    impl Into<GroupAssignmentAny> for GroupAssignment {
        fn into(self) -> GroupAssignmentAny {
            GroupAssignmentAny::GroupAssignment(Box::new(self))
        }
    }
    impl Into<GroupAssignmentAny> for DraughtingGroupAssignment {
        fn into(self) -> GroupAssignmentAny {
            GroupAssignmentAny::DraughtingGroupAssignment(Box::new(self.into()))
        }
    }
    impl AsRef<GroupAssignment> for GroupAssignmentAny {
        fn as_ref(&self) -> &GroupAssignment {
            match self {
                GroupAssignmentAny::GroupAssignment(x) => x.as_ref(),
                GroupAssignmentAny::DraughtingGroupAssignment(x) => (**x).as_ref(),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = group_relationship)]
    #[holder(generate_deserialize)]
    pub struct GroupRelationship {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub description: Text,
        #[holder(use_place_holder)]
        pub relating_group: Group,
        #[holder(use_place_holder)]
        pub related_group: Group,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = hyperbola)]
    #[holder(generate_deserialize)]
    pub struct Hyperbola {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub conic: Conic,
        #[holder(use_place_holder)]
        pub semi_axis: PositiveLengthMeasure,
        #[holder(use_place_holder)]
        pub semi_imag_axis: PositiveLengthMeasure,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = invisibility)]
    #[holder(generate_deserialize)]
    pub struct Invisibility {
        #[holder(use_place_holder)]
        pub invisible_items: Vec<InvisibleItem>,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum InvisibilityAny {
        #[holder(use_place_holder)]
        # [holder (field = invisibility)]
        Invisibility(Box<Invisibility>),
        #[holder(use_place_holder)]
        # [holder (field = context_dependent_invisibility)]
        ContextDependentInvisibility(Box<ContextDependentInvisibility>),
    }
    impl Into<InvisibilityAny> for Invisibility {
        fn into(self) -> InvisibilityAny {
            InvisibilityAny::Invisibility(Box::new(self))
        }
    }
    impl Into<InvisibilityAny> for ContextDependentInvisibility {
        fn into(self) -> InvisibilityAny {
            InvisibilityAny::ContextDependentInvisibility(Box::new(self.into()))
        }
    }
    impl AsRef<Invisibility> for InvisibilityAny {
        fn as_ref(&self) -> &Invisibility {
            match self {
                InvisibilityAny::Invisibility(x) => x.as_ref(),
                InvisibilityAny::ContextDependentInvisibility(x) => (**x).as_ref(),
            }
        }
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = leader_curve)]
    #[holder(generate_deserialize)]
    pub struct LeaderCurve {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub annotation_curve_occurrence: AnnotationCurveOccurrence,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = leader_directed_callout)]
    #[holder(generate_deserialize)]
    pub struct LeaderDirectedCallout {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub draughting_callout: DraughtingCallout,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum LeaderDirectedCalloutAny {
        #[holder(use_place_holder)]
        # [holder (field = leader_directed_callout)]
        LeaderDirectedCallout(Box<LeaderDirectedCallout>),
        #[holder(use_place_holder)]
        # [holder (field = leader_directed_dimension)]
        LeaderDirectedDimension(Box<LeaderDirectedDimension>),
    }
    impl Into<LeaderDirectedCalloutAny> for LeaderDirectedCallout {
        fn into(self) -> LeaderDirectedCalloutAny {
            LeaderDirectedCalloutAny::LeaderDirectedCallout(Box::new(self))
        }
    }
    impl Into<LeaderDirectedCalloutAny> for LeaderDirectedDimension {
        fn into(self) -> LeaderDirectedCalloutAny {
            LeaderDirectedCalloutAny::LeaderDirectedDimension(Box::new(self.into()))
        }
    }
    impl AsRef<LeaderDirectedCallout> for LeaderDirectedCalloutAny {
        fn as_ref(&self) -> &LeaderDirectedCallout {
            match self {
                LeaderDirectedCalloutAny::LeaderDirectedCallout(x) => x.as_ref(),
                LeaderDirectedCalloutAny::LeaderDirectedDimension(x) => (**x).as_ref(),
            }
        }
    }
    impl AsRef<DraughtingCallout> for LeaderDirectedCalloutAny {
        fn as_ref(&self) -> &DraughtingCallout {
            match self {
                LeaderDirectedCalloutAny::LeaderDirectedCallout(x) => {
                    AsRef::<LeaderDirectedCallout>::as_ref(x).as_ref()
                }
                LeaderDirectedCalloutAny::LeaderDirectedDimension(x) => {
                    AsRef::<LeaderDirectedCallout>::as_ref(x.as_ref()).as_ref()
                }
            }
        }
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = leader_directed_dimension)]
    #[holder(generate_deserialize)]
    pub struct LeaderDirectedDimension {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub leader_directed_callout: LeaderDirectedCallout,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = leader_terminator)]
    #[holder(generate_deserialize)]
    pub struct LeaderTerminator {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub terminator_symbol: TerminatorSymbol,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = length_measure_with_unit)]
    #[holder(generate_deserialize)]
    pub struct LengthMeasureWithUnit {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub measure_with_unit: MeasureWithUnit,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = length_unit)]
    #[holder(generate_deserialize)]
    pub struct LengthUnit {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub named_unit: NamedUnit,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = line)]
    #[holder(generate_deserialize)]
    pub struct Line {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub curve: Curve,
        #[holder(use_place_holder)]
        pub pnt: CartesianPoint,
        #[holder(use_place_holder)]
        pub dir: Vector,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = linear_dimension)]
    #[holder(generate_deserialize)]
    pub struct LinearDimension {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub dimension_curve_directed_callout: DimensionCurveDirectedCallout,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = mapped_item)]
    #[holder(generate_deserialize)]
    pub struct MappedItem {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub representation_item: RepresentationItem,
        #[holder(use_place_holder)]
        pub mapping_source: RepresentationMapAny,
        #[holder(use_place_holder)]
        pub mapping_target: RepresentationItemAny,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum MappedItemAny {
        #[holder(use_place_holder)]
        # [holder (field = mapped_item)]
        MappedItem(Box<MappedItem>),
        #[holder(use_place_holder)]
        # [holder (field = annotation_symbol)]
        AnnotationSymbol(Box<AnnotationSymbol>),
        #[holder(use_place_holder)]
        # [holder (field = annotation_text)]
        AnnotationText(Box<AnnotationText>),
        #[holder(use_place_holder)]
        # [holder (field = camera_image)]
        CameraImage(Box<CameraImageAny>),
    }
    impl Into<MappedItemAny> for MappedItem {
        fn into(self) -> MappedItemAny {
            MappedItemAny::MappedItem(Box::new(self))
        }
    }
    impl Into<MappedItemAny> for AnnotationSymbol {
        fn into(self) -> MappedItemAny {
            MappedItemAny::AnnotationSymbol(Box::new(self.into()))
        }
    }
    impl Into<MappedItemAny> for AnnotationText {
        fn into(self) -> MappedItemAny {
            MappedItemAny::AnnotationText(Box::new(self.into()))
        }
    }
    impl Into<MappedItemAny> for CameraImage {
        fn into(self) -> MappedItemAny {
            MappedItemAny::CameraImage(Box::new(self.into()))
        }
    }
    impl AsRef<MappedItem> for MappedItemAny {
        fn as_ref(&self) -> &MappedItem {
            match self {
                MappedItemAny::MappedItem(x) => x.as_ref(),
                MappedItemAny::AnnotationSymbol(x) => (**x).as_ref(),
                MappedItemAny::AnnotationText(x) => (**x).as_ref(),
                MappedItemAny::CameraImage(x) => (**x).as_ref(),
            }
        }
    }
    impl AsRef<RepresentationItem> for MappedItemAny {
        fn as_ref(&self) -> &RepresentationItem {
            match self {
                MappedItemAny::MappedItem(x) => AsRef::<MappedItem>::as_ref(x).as_ref(),
                MappedItemAny::AnnotationSymbol(x) => {
                    AsRef::<MappedItem>::as_ref(x.as_ref()).as_ref()
                }
                MappedItemAny::AnnotationText(x) => {
                    AsRef::<MappedItem>::as_ref(x.as_ref()).as_ref()
                }
                MappedItemAny::CameraImage(x) => AsRef::<MappedItem>::as_ref(x.as_ref()).as_ref(),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = measure_with_unit)]
    #[holder(generate_deserialize)]
    pub struct MeasureWithUnit {
        #[holder(use_place_holder)]
        pub value_component: MeasureValue,
        #[holder(use_place_holder)]
        pub unit_component: Unit,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum MeasureWithUnitAny {
        #[holder(use_place_holder)]
        # [holder (field = measure_with_unit)]
        MeasureWithUnit(Box<MeasureWithUnit>),
        #[holder(use_place_holder)]
        # [holder (field = length_measure_with_unit)]
        LengthMeasureWithUnit(Box<LengthMeasureWithUnit>),
        #[holder(use_place_holder)]
        # [holder (field = plane_angle_measure_with_unit)]
        PlaneAngleMeasureWithUnit(Box<PlaneAngleMeasureWithUnit>),
    }
    impl Into<MeasureWithUnitAny> for MeasureWithUnit {
        fn into(self) -> MeasureWithUnitAny {
            MeasureWithUnitAny::MeasureWithUnit(Box::new(self))
        }
    }
    impl Into<MeasureWithUnitAny> for LengthMeasureWithUnit {
        fn into(self) -> MeasureWithUnitAny {
            MeasureWithUnitAny::LengthMeasureWithUnit(Box::new(self.into()))
        }
    }
    impl Into<MeasureWithUnitAny> for PlaneAngleMeasureWithUnit {
        fn into(self) -> MeasureWithUnitAny {
            MeasureWithUnitAny::PlaneAngleMeasureWithUnit(Box::new(self.into()))
        }
    }
    impl AsRef<MeasureWithUnit> for MeasureWithUnitAny {
        fn as_ref(&self) -> &MeasureWithUnit {
            match self {
                MeasureWithUnitAny::MeasureWithUnit(x) => x.as_ref(),
                MeasureWithUnitAny::LengthMeasureWithUnit(x) => (**x).as_ref(),
                MeasureWithUnitAny::PlaneAngleMeasureWithUnit(x) => (**x).as_ref(),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = named_unit)]
    #[holder(generate_deserialize)]
    pub struct NamedUnit {
        #[holder(use_place_holder)]
        pub dimensions: DimensionalExponents,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum NamedUnitAny {
        #[holder(use_place_holder)]
        # [holder (field = named_unit)]
        NamedUnit(Box<NamedUnit>),
        #[holder(use_place_holder)]
        # [holder (field = conversion_based_unit)]
        ConversionBasedUnit(Box<ConversionBasedUnit>),
        #[holder(use_place_holder)]
        # [holder (field = length_unit)]
        LengthUnit(Box<LengthUnit>),
        #[holder(use_place_holder)]
        # [holder (field = plane_angle_unit)]
        PlaneAngleUnit(Box<PlaneAngleUnit>),
        #[holder(use_place_holder)]
        # [holder (field = si_unit)]
        SiUnit(Box<SiUnit>),
    }
    impl Into<NamedUnitAny> for NamedUnit {
        fn into(self) -> NamedUnitAny {
            NamedUnitAny::NamedUnit(Box::new(self))
        }
    }
    impl Into<NamedUnitAny> for ConversionBasedUnit {
        fn into(self) -> NamedUnitAny {
            NamedUnitAny::ConversionBasedUnit(Box::new(self.into()))
        }
    }
    impl Into<NamedUnitAny> for LengthUnit {
        fn into(self) -> NamedUnitAny {
            NamedUnitAny::LengthUnit(Box::new(self.into()))
        }
    }
    impl Into<NamedUnitAny> for PlaneAngleUnit {
        fn into(self) -> NamedUnitAny {
            NamedUnitAny::PlaneAngleUnit(Box::new(self.into()))
        }
    }
    impl Into<NamedUnitAny> for SiUnit {
        fn into(self) -> NamedUnitAny {
            NamedUnitAny::SiUnit(Box::new(self.into()))
        }
    }
    impl AsRef<NamedUnit> for NamedUnitAny {
        fn as_ref(&self) -> &NamedUnit {
            match self {
                NamedUnitAny::NamedUnit(x) => x.as_ref(),
                NamedUnitAny::ConversionBasedUnit(x) => (**x).as_ref(),
                NamedUnitAny::LengthUnit(x) => (**x).as_ref(),
                NamedUnitAny::PlaneAngleUnit(x) => (**x).as_ref(),
                NamedUnitAny::SiUnit(x) => (**x).as_ref(),
            }
        }
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = offset_curve_2d)]
    #[holder(generate_deserialize)]
    pub struct OffsetCurve2D {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub curve: Curve,
        #[holder(use_place_holder)]
        pub basis_curve: CurveAny,
        #[holder(use_place_holder)]
        pub distance: LengthMeasure,
        pub self_intersect: Logical,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = one_direction_repeat_factor)]
    #[holder(generate_deserialize)]
    pub struct OneDirectionRepeatFactor {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub geometric_representation_item: GeometricRepresentationItem,
        #[holder(use_place_holder)]
        pub repeat_factor: Vector,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum OneDirectionRepeatFactorAny {
        #[holder(use_place_holder)]
        # [holder (field = one_direction_repeat_factor)]
        OneDirectionRepeatFactor(Box<OneDirectionRepeatFactor>),
        #[holder(use_place_holder)]
        # [holder (field = two_direction_repeat_factor)]
        TwoDirectionRepeatFactor(Box<TwoDirectionRepeatFactor>),
    }
    impl Into<OneDirectionRepeatFactorAny> for OneDirectionRepeatFactor {
        fn into(self) -> OneDirectionRepeatFactorAny {
            OneDirectionRepeatFactorAny::OneDirectionRepeatFactor(Box::new(self))
        }
    }
    impl Into<OneDirectionRepeatFactorAny> for TwoDirectionRepeatFactor {
        fn into(self) -> OneDirectionRepeatFactorAny {
            OneDirectionRepeatFactorAny::TwoDirectionRepeatFactor(Box::new(self.into()))
        }
    }
    impl AsRef<OneDirectionRepeatFactor> for OneDirectionRepeatFactorAny {
        fn as_ref(&self) -> &OneDirectionRepeatFactor {
            match self {
                OneDirectionRepeatFactorAny::OneDirectionRepeatFactor(x) => x.as_ref(),
                OneDirectionRepeatFactorAny::TwoDirectionRepeatFactor(x) => (**x).as_ref(),
            }
        }
    }
    impl AsRef<GeometricRepresentationItem> for OneDirectionRepeatFactorAny {
        fn as_ref(&self) -> &GeometricRepresentationItem {
            match self {
                OneDirectionRepeatFactorAny::OneDirectionRepeatFactor(x) => {
                    AsRef::<OneDirectionRepeatFactor>::as_ref(x).as_ref()
                }
                OneDirectionRepeatFactorAny::TwoDirectionRepeatFactor(x) => {
                    AsRef::<OneDirectionRepeatFactor>::as_ref(x.as_ref()).as_ref()
                }
            }
        }
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = ordinate_dimension)]
    #[holder(generate_deserialize)]
    pub struct OrdinateDimension {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub projection_directed_callout: ProjectionDirectedCallout,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = organization)]
    #[holder(generate_deserialize)]
    pub struct Organization {
        #[holder(use_place_holder)]
        pub id: Option<Identifier>,
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub description: Text,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = organization_assignment)]
    #[holder(generate_deserialize)]
    pub struct OrganizationAssignment {
        #[holder(use_place_holder)]
        pub assigned_organization: Organization,
        #[holder(use_place_holder)]
        pub role: OrganizationRole,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum OrganizationAssignmentAny {
        #[holder(use_place_holder)]
        # [holder (field = organization_assignment)]
        OrganizationAssignment(Box<OrganizationAssignment>),
        #[holder(use_place_holder)]
        # [holder (field = draughting_organization_assignment)]
        DraughtingOrganizationAssignment(Box<DraughtingOrganizationAssignment>),
    }
    impl Into<OrganizationAssignmentAny> for OrganizationAssignment {
        fn into(self) -> OrganizationAssignmentAny {
            OrganizationAssignmentAny::OrganizationAssignment(Box::new(self))
        }
    }
    impl Into<OrganizationAssignmentAny> for DraughtingOrganizationAssignment {
        fn into(self) -> OrganizationAssignmentAny {
            OrganizationAssignmentAny::DraughtingOrganizationAssignment(Box::new(self.into()))
        }
    }
    impl AsRef<OrganizationAssignment> for OrganizationAssignmentAny {
        fn as_ref(&self) -> &OrganizationAssignment {
            match self {
                OrganizationAssignmentAny::OrganizationAssignment(x) => x.as_ref(),
                OrganizationAssignmentAny::DraughtingOrganizationAssignment(x) => (**x).as_ref(),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = organization_role)]
    #[holder(generate_deserialize)]
    pub struct OrganizationRole {
        #[holder(use_place_holder)]
        pub name: Label,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = organizational_address)]
    #[holder(generate_deserialize)]
    pub struct OrganizationalAddress {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub address: Address,
        #[holder(use_place_holder)]
        pub organizations: Vec<Organization>,
        #[holder(use_place_holder)]
        pub description: Text,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = parabola)]
    #[holder(generate_deserialize)]
    pub struct Parabola {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub conic: Conic,
        #[holder(use_place_holder)]
        pub focal_dist: LengthMeasure,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = person)]
    #[holder(generate_deserialize)]
    pub struct Person {
        #[holder(use_place_holder)]
        pub id: Identifier,
        #[holder(use_place_holder)]
        pub last_name: Option<Label>,
        #[holder(use_place_holder)]
        pub first_name: Option<Label>,
        #[holder(use_place_holder)]
        pub middle_names: Option<Vec<Label>>,
        #[holder(use_place_holder)]
        pub prefix_titles: Option<Vec<Label>>,
        #[holder(use_place_holder)]
        pub suffix_titles: Option<Vec<Label>>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = person_and_organization)]
    #[holder(generate_deserialize)]
    pub struct PersonAndOrganization {
        #[holder(use_place_holder)]
        pub the_person: Person,
        #[holder(use_place_holder)]
        pub the_organization: Organization,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = person_and_organization_assignment)]
    #[holder(generate_deserialize)]
    pub struct PersonAndOrganizationAssignment {
        #[holder(use_place_holder)]
        pub assigned_person_and_organization: PersonAndOrganization,
        #[holder(use_place_holder)]
        pub role: PersonAndOrganizationRole,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum PersonAndOrganizationAssignmentAny {
        #[holder(use_place_holder)]
        # [holder (field = person_and_organization_assignment)]
        PersonAndOrganizationAssignment(Box<PersonAndOrganizationAssignment>),
        #[holder(use_place_holder)]
        # [holder (field = draughting_person_and_organization_assignment)]
        DraughtingPersonAndOrganizationAssignment(Box<DraughtingPersonAndOrganizationAssignment>),
    }
    impl Into<PersonAndOrganizationAssignmentAny> for PersonAndOrganizationAssignment {
        fn into(self) -> PersonAndOrganizationAssignmentAny {
            PersonAndOrganizationAssignmentAny::PersonAndOrganizationAssignment(Box::new(self))
        }
    }
    impl Into<PersonAndOrganizationAssignmentAny> for DraughtingPersonAndOrganizationAssignment {
        fn into(self) -> PersonAndOrganizationAssignmentAny {
            PersonAndOrganizationAssignmentAny::DraughtingPersonAndOrganizationAssignment(Box::new(
                self.into(),
            ))
        }
    }
    impl AsRef<PersonAndOrganizationAssignment> for PersonAndOrganizationAssignmentAny {
        fn as_ref(&self) -> &PersonAndOrganizationAssignment {
            match self {
                PersonAndOrganizationAssignmentAny::PersonAndOrganizationAssignment(x) => {
                    x.as_ref()
                }
                PersonAndOrganizationAssignmentAny::DraughtingPersonAndOrganizationAssignment(
                    x,
                ) => (**x).as_ref(),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = person_and_organization_role)]
    #[holder(generate_deserialize)]
    pub struct PersonAndOrganizationRole {
        #[holder(use_place_holder)]
        pub name: Label,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = person_assignment)]
    #[holder(generate_deserialize)]
    pub struct PersonAssignment {
        #[holder(use_place_holder)]
        pub assigned_person: Person,
        #[holder(use_place_holder)]
        pub role: PersonRole,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum PersonAssignmentAny {
        #[holder(use_place_holder)]
        # [holder (field = person_assignment)]
        PersonAssignment(Box<PersonAssignment>),
        #[holder(use_place_holder)]
        # [holder (field = draughting_person_assignment)]
        DraughtingPersonAssignment(Box<DraughtingPersonAssignment>),
    }
    impl Into<PersonAssignmentAny> for PersonAssignment {
        fn into(self) -> PersonAssignmentAny {
            PersonAssignmentAny::PersonAssignment(Box::new(self))
        }
    }
    impl Into<PersonAssignmentAny> for DraughtingPersonAssignment {
        fn into(self) -> PersonAssignmentAny {
            PersonAssignmentAny::DraughtingPersonAssignment(Box::new(self.into()))
        }
    }
    impl AsRef<PersonAssignment> for PersonAssignmentAny {
        fn as_ref(&self) -> &PersonAssignment {
            match self {
                PersonAssignmentAny::PersonAssignment(x) => x.as_ref(),
                PersonAssignmentAny::DraughtingPersonAssignment(x) => (**x).as_ref(),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = person_role)]
    #[holder(generate_deserialize)]
    pub struct PersonRole {
        #[holder(use_place_holder)]
        pub name: Label,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = personal_address)]
    #[holder(generate_deserialize)]
    pub struct PersonalAddress {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub address: Address,
        #[holder(use_place_holder)]
        pub people: Vec<Person>,
        #[holder(use_place_holder)]
        pub description: Text,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = placement)]
    #[holder(generate_deserialize)]
    pub struct Placement {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub geometric_representation_item: GeometricRepresentationItem,
        #[holder(use_place_holder)]
        pub location: CartesianPoint,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum PlacementAny {
        #[holder(use_place_holder)]
        # [holder (field = placement)]
        Placement(Box<Placement>),
        #[holder(use_place_holder)]
        # [holder (field = axis2_placement_2d)]
        Axis2Placement2D(Box<Axis2Placement2D>),
    }
    impl Into<PlacementAny> for Placement {
        fn into(self) -> PlacementAny {
            PlacementAny::Placement(Box::new(self))
        }
    }
    impl Into<PlacementAny> for Axis2Placement2D {
        fn into(self) -> PlacementAny {
            PlacementAny::Axis2Placement2D(Box::new(self.into()))
        }
    }
    impl AsRef<Placement> for PlacementAny {
        fn as_ref(&self) -> &Placement {
            match self {
                PlacementAny::Placement(x) => x.as_ref(),
                PlacementAny::Axis2Placement2D(x) => (**x).as_ref(),
            }
        }
    }
    impl AsRef<GeometricRepresentationItem> for PlacementAny {
        fn as_ref(&self) -> &GeometricRepresentationItem {
            match self {
                PlacementAny::Placement(x) => AsRef::<Placement>::as_ref(x).as_ref(),
                PlacementAny::Axis2Placement2D(x) => {
                    AsRef::<Placement>::as_ref(x.as_ref()).as_ref()
                }
            }
        }
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = planar_box)]
    #[holder(generate_deserialize)]
    pub struct PlanarBox {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub planar_extent: PlanarExtent,
        #[holder(use_place_holder)]
        pub placement: Axis2Placement,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = planar_extent)]
    #[holder(generate_deserialize)]
    pub struct PlanarExtent {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub geometric_representation_item: GeometricRepresentationItem,
        #[holder(use_place_holder)]
        pub size_in_x: LengthMeasure,
        #[holder(use_place_holder)]
        pub size_in_y: LengthMeasure,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum PlanarExtentAny {
        #[holder(use_place_holder)]
        # [holder (field = planar_extent)]
        PlanarExtent(Box<PlanarExtent>),
        #[holder(use_place_holder)]
        # [holder (field = planar_box)]
        PlanarBox(Box<PlanarBox>),
    }
    impl Into<PlanarExtentAny> for PlanarExtent {
        fn into(self) -> PlanarExtentAny {
            PlanarExtentAny::PlanarExtent(Box::new(self))
        }
    }
    impl Into<PlanarExtentAny> for PlanarBox {
        fn into(self) -> PlanarExtentAny {
            PlanarExtentAny::PlanarBox(Box::new(self.into()))
        }
    }
    impl AsRef<PlanarExtent> for PlanarExtentAny {
        fn as_ref(&self) -> &PlanarExtent {
            match self {
                PlanarExtentAny::PlanarExtent(x) => x.as_ref(),
                PlanarExtentAny::PlanarBox(x) => (**x).as_ref(),
            }
        }
    }
    impl AsRef<GeometricRepresentationItem> for PlanarExtentAny {
        fn as_ref(&self) -> &GeometricRepresentationItem {
            match self {
                PlanarExtentAny::PlanarExtent(x) => AsRef::<PlanarExtent>::as_ref(x).as_ref(),
                PlanarExtentAny::PlanarBox(x) => AsRef::<PlanarExtent>::as_ref(x.as_ref()).as_ref(),
            }
        }
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = plane_angle_measure_with_unit)]
    #[holder(generate_deserialize)]
    pub struct PlaneAngleMeasureWithUnit {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub measure_with_unit: MeasureWithUnit,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = plane_angle_unit)]
    #[holder(generate_deserialize)]
    pub struct PlaneAngleUnit {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub named_unit: NamedUnit,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = point)]
    #[holder(generate_deserialize)]
    pub struct Point {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub geometric_representation_item: GeometricRepresentationItem,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum PointAny {
        #[holder(use_place_holder)]
        # [holder (field = point)]
        Point(Box<Point>),
        #[holder(use_place_holder)]
        # [holder (field = cartesian_point)]
        CartesianPoint(Box<CartesianPoint>),
        #[holder(use_place_holder)]
        # [holder (field = point_on_curve)]
        PointOnCurve(Box<PointOnCurve>),
    }
    impl Into<PointAny> for Point {
        fn into(self) -> PointAny {
            PointAny::Point(Box::new(self))
        }
    }
    impl Into<PointAny> for CartesianPoint {
        fn into(self) -> PointAny {
            PointAny::CartesianPoint(Box::new(self.into()))
        }
    }
    impl Into<PointAny> for PointOnCurve {
        fn into(self) -> PointAny {
            PointAny::PointOnCurve(Box::new(self.into()))
        }
    }
    impl AsRef<Point> for PointAny {
        fn as_ref(&self) -> &Point {
            match self {
                PointAny::Point(x) => x.as_ref(),
                PointAny::CartesianPoint(x) => (**x).as_ref(),
                PointAny::PointOnCurve(x) => (**x).as_ref(),
            }
        }
    }
    impl AsRef<GeometricRepresentationItem> for PointAny {
        fn as_ref(&self) -> &GeometricRepresentationItem {
            match self {
                PointAny::Point(x) => AsRef::<Point>::as_ref(x).as_ref(),
                PointAny::CartesianPoint(x) => AsRef::<Point>::as_ref(x.as_ref()).as_ref(),
                PointAny::PointOnCurve(x) => AsRef::<Point>::as_ref(x.as_ref()).as_ref(),
            }
        }
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = point_on_curve)]
    #[holder(generate_deserialize)]
    pub struct PointOnCurve {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub point: Point,
        #[holder(use_place_holder)]
        pub basis_curve: CurveAny,
        #[holder(use_place_holder)]
        pub point_parameter: ParameterValue,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = polyline)]
    #[holder(generate_deserialize)]
    pub struct Polyline {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub bounded_curve: BoundedCurve,
        #[holder(use_place_holder)]
        pub points: Vec<CartesianPoint>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut)]
    # [holder (table = Tables)]
    # [holder (field = pre_defined_colour)]
    #[holder(generate_deserialize)]
    pub struct PreDefinedColour {
        #[as_ref]
        #[as_mut]
        #[holder(use_place_holder)]
        pub pre_defined_item: PreDefinedItem,
        #[as_ref]
        #[as_mut]
        #[holder(use_place_holder)]
        pub colour: Colour,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum PreDefinedColourAny {
        #[holder(use_place_holder)]
        # [holder (field = pre_defined_colour)]
        PreDefinedColour(Box<PreDefinedColour>),
        #[holder(use_place_holder)]
        # [holder (field = draughting_pre_defined_colour)]
        DraughtingPreDefinedColour(Box<DraughtingPreDefinedColour>),
    }
    impl Into<PreDefinedColourAny> for PreDefinedColour {
        fn into(self) -> PreDefinedColourAny {
            PreDefinedColourAny::PreDefinedColour(Box::new(self))
        }
    }
    impl Into<PreDefinedColourAny> for DraughtingPreDefinedColour {
        fn into(self) -> PreDefinedColourAny {
            PreDefinedColourAny::DraughtingPreDefinedColour(Box::new(self.into()))
        }
    }
    impl AsRef<PreDefinedColour> for PreDefinedColourAny {
        fn as_ref(&self) -> &PreDefinedColour {
            match self {
                PreDefinedColourAny::PreDefinedColour(x) => x.as_ref(),
                PreDefinedColourAny::DraughtingPreDefinedColour(x) => (**x).as_ref(),
            }
        }
    }
    impl AsRef<PreDefinedItem> for PreDefinedColourAny {
        fn as_ref(&self) -> &PreDefinedItem {
            match self {
                PreDefinedColourAny::PreDefinedColour(x) => {
                    AsRef::<PreDefinedColour>::as_ref(x).as_ref()
                }
                PreDefinedColourAny::DraughtingPreDefinedColour(x) => {
                    AsRef::<PreDefinedColour>::as_ref(x.as_ref()).as_ref()
                }
            }
        }
    }
    impl AsRef<Colour> for PreDefinedColourAny {
        fn as_ref(&self) -> &Colour {
            match self {
                PreDefinedColourAny::PreDefinedColour(x) => {
                    AsRef::<PreDefinedColour>::as_ref(x).as_ref()
                }
                PreDefinedColourAny::DraughtingPreDefinedColour(x) => {
                    AsRef::<PreDefinedColour>::as_ref(x.as_ref()).as_ref()
                }
            }
        }
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = pre_defined_curve_font)]
    #[holder(generate_deserialize)]
    pub struct PreDefinedCurveFont {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub pre_defined_item: PreDefinedItem,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum PreDefinedCurveFontAny {
        #[holder(use_place_holder)]
        # [holder (field = pre_defined_curve_font)]
        PreDefinedCurveFont(Box<PreDefinedCurveFont>),
        #[holder(use_place_holder)]
        # [holder (field = draughting_pre_defined_curve_font)]
        DraughtingPreDefinedCurveFont(Box<DraughtingPreDefinedCurveFont>),
    }
    impl Into<PreDefinedCurveFontAny> for PreDefinedCurveFont {
        fn into(self) -> PreDefinedCurveFontAny {
            PreDefinedCurveFontAny::PreDefinedCurveFont(Box::new(self))
        }
    }
    impl Into<PreDefinedCurveFontAny> for DraughtingPreDefinedCurveFont {
        fn into(self) -> PreDefinedCurveFontAny {
            PreDefinedCurveFontAny::DraughtingPreDefinedCurveFont(Box::new(self.into()))
        }
    }
    impl AsRef<PreDefinedCurveFont> for PreDefinedCurveFontAny {
        fn as_ref(&self) -> &PreDefinedCurveFont {
            match self {
                PreDefinedCurveFontAny::PreDefinedCurveFont(x) => x.as_ref(),
                PreDefinedCurveFontAny::DraughtingPreDefinedCurveFont(x) => (**x).as_ref(),
            }
        }
    }
    impl AsRef<PreDefinedItem> for PreDefinedCurveFontAny {
        fn as_ref(&self) -> &PreDefinedItem {
            match self {
                PreDefinedCurveFontAny::PreDefinedCurveFont(x) => {
                    AsRef::<PreDefinedCurveFont>::as_ref(x).as_ref()
                }
                PreDefinedCurveFontAny::DraughtingPreDefinedCurveFont(x) => {
                    AsRef::<PreDefinedCurveFont>::as_ref(x.as_ref()).as_ref()
                }
            }
        }
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = pre_defined_dimension_symbol)]
    #[holder(generate_deserialize)]
    pub struct PreDefinedDimensionSymbol {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub pre_defined_symbol: PreDefinedSymbol,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = pre_defined_geometrical_tolerance_symbol)]
    #[holder(generate_deserialize)]
    pub struct PreDefinedGeometricalToleranceSymbol {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub pre_defined_symbol: PreDefinedSymbol,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = pre_defined_item)]
    #[holder(generate_deserialize)]
    pub struct PreDefinedItem {
        #[holder(use_place_holder)]
        pub name: Label,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum PreDefinedItemAny {
        #[holder(use_place_holder)]
        # [holder (field = pre_defined_item)]
        PreDefinedItem(Box<PreDefinedItem>),
        #[holder(use_place_holder)]
        # [holder (field = pre_defined_colour)]
        PreDefinedColour(Box<PreDefinedColourAny>),
        #[holder(use_place_holder)]
        # [holder (field = pre_defined_curve_font)]
        PreDefinedCurveFont(Box<PreDefinedCurveFontAny>),
        #[holder(use_place_holder)]
        # [holder (field = pre_defined_symbol)]
        PreDefinedSymbol(Box<PreDefinedSymbolAny>),
        #[holder(use_place_holder)]
        # [holder (field = pre_defined_text_font)]
        PreDefinedTextFont(Box<PreDefinedTextFontAny>),
    }
    impl Into<PreDefinedItemAny> for PreDefinedItem {
        fn into(self) -> PreDefinedItemAny {
            PreDefinedItemAny::PreDefinedItem(Box::new(self))
        }
    }
    impl Into<PreDefinedItemAny> for PreDefinedColour {
        fn into(self) -> PreDefinedItemAny {
            PreDefinedItemAny::PreDefinedColour(Box::new(self.into()))
        }
    }
    impl Into<PreDefinedItemAny> for PreDefinedCurveFont {
        fn into(self) -> PreDefinedItemAny {
            PreDefinedItemAny::PreDefinedCurveFont(Box::new(self.into()))
        }
    }
    impl Into<PreDefinedItemAny> for PreDefinedSymbol {
        fn into(self) -> PreDefinedItemAny {
            PreDefinedItemAny::PreDefinedSymbol(Box::new(self.into()))
        }
    }
    impl Into<PreDefinedItemAny> for PreDefinedTextFont {
        fn into(self) -> PreDefinedItemAny {
            PreDefinedItemAny::PreDefinedTextFont(Box::new(self.into()))
        }
    }
    impl AsRef<PreDefinedItem> for PreDefinedItemAny {
        fn as_ref(&self) -> &PreDefinedItem {
            match self {
                PreDefinedItemAny::PreDefinedItem(x) => x.as_ref(),
                PreDefinedItemAny::PreDefinedColour(x) => (**x).as_ref(),
                PreDefinedItemAny::PreDefinedCurveFont(x) => (**x).as_ref(),
                PreDefinedItemAny::PreDefinedSymbol(x) => (**x).as_ref(),
                PreDefinedItemAny::PreDefinedTextFont(x) => (**x).as_ref(),
            }
        }
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = pre_defined_point_marker_symbol)]
    #[holder(generate_deserialize)]
    pub struct PreDefinedPointMarkerSymbol {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub pre_defined_symbol: PreDefinedSymbol,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = pre_defined_symbol)]
    #[holder(generate_deserialize)]
    pub struct PreDefinedSymbol {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub pre_defined_item: PreDefinedItem,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum PreDefinedSymbolAny {
        #[holder(use_place_holder)]
        # [holder (field = pre_defined_symbol)]
        PreDefinedSymbol(Box<PreDefinedSymbol>),
        #[holder(use_place_holder)]
        # [holder (field = pre_defined_dimension_symbol)]
        PreDefinedDimensionSymbol(Box<PreDefinedDimensionSymbol>),
        #[holder(use_place_holder)]
        # [holder (field = pre_defined_geometrical_tolerance_symbol)]
        PreDefinedGeometricalToleranceSymbol(Box<PreDefinedGeometricalToleranceSymbol>),
        #[holder(use_place_holder)]
        # [holder (field = pre_defined_point_marker_symbol)]
        PreDefinedPointMarkerSymbol(Box<PreDefinedPointMarkerSymbol>),
        #[holder(use_place_holder)]
        # [holder (field = pre_defined_terminator_symbol)]
        PreDefinedTerminatorSymbol(Box<PreDefinedTerminatorSymbol>),
    }
    impl Into<PreDefinedSymbolAny> for PreDefinedSymbol {
        fn into(self) -> PreDefinedSymbolAny {
            PreDefinedSymbolAny::PreDefinedSymbol(Box::new(self))
        }
    }
    impl Into<PreDefinedSymbolAny> for PreDefinedDimensionSymbol {
        fn into(self) -> PreDefinedSymbolAny {
            PreDefinedSymbolAny::PreDefinedDimensionSymbol(Box::new(self.into()))
        }
    }
    impl Into<PreDefinedSymbolAny> for PreDefinedGeometricalToleranceSymbol {
        fn into(self) -> PreDefinedSymbolAny {
            PreDefinedSymbolAny::PreDefinedGeometricalToleranceSymbol(Box::new(self.into()))
        }
    }
    impl Into<PreDefinedSymbolAny> for PreDefinedPointMarkerSymbol {
        fn into(self) -> PreDefinedSymbolAny {
            PreDefinedSymbolAny::PreDefinedPointMarkerSymbol(Box::new(self.into()))
        }
    }
    impl Into<PreDefinedSymbolAny> for PreDefinedTerminatorSymbol {
        fn into(self) -> PreDefinedSymbolAny {
            PreDefinedSymbolAny::PreDefinedTerminatorSymbol(Box::new(self.into()))
        }
    }
    impl AsRef<PreDefinedSymbol> for PreDefinedSymbolAny {
        fn as_ref(&self) -> &PreDefinedSymbol {
            match self {
                PreDefinedSymbolAny::PreDefinedSymbol(x) => x.as_ref(),
                PreDefinedSymbolAny::PreDefinedDimensionSymbol(x) => (**x).as_ref(),
                PreDefinedSymbolAny::PreDefinedGeometricalToleranceSymbol(x) => (**x).as_ref(),
                PreDefinedSymbolAny::PreDefinedPointMarkerSymbol(x) => (**x).as_ref(),
                PreDefinedSymbolAny::PreDefinedTerminatorSymbol(x) => (**x).as_ref(),
            }
        }
    }
    impl AsRef<PreDefinedItem> for PreDefinedSymbolAny {
        fn as_ref(&self) -> &PreDefinedItem {
            match self {
                PreDefinedSymbolAny::PreDefinedSymbol(x) => {
                    AsRef::<PreDefinedSymbol>::as_ref(x).as_ref()
                }
                PreDefinedSymbolAny::PreDefinedDimensionSymbol(x) => {
                    AsRef::<PreDefinedSymbol>::as_ref(x.as_ref()).as_ref()
                }
                PreDefinedSymbolAny::PreDefinedGeometricalToleranceSymbol(x) => {
                    AsRef::<PreDefinedSymbol>::as_ref(x.as_ref()).as_ref()
                }
                PreDefinedSymbolAny::PreDefinedPointMarkerSymbol(x) => {
                    AsRef::<PreDefinedSymbol>::as_ref(x.as_ref()).as_ref()
                }
                PreDefinedSymbolAny::PreDefinedTerminatorSymbol(x) => {
                    AsRef::<PreDefinedSymbol>::as_ref(x.as_ref()).as_ref()
                }
            }
        }
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = pre_defined_terminator_symbol)]
    #[holder(generate_deserialize)]
    pub struct PreDefinedTerminatorSymbol {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub pre_defined_symbol: PreDefinedSymbol,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = pre_defined_text_font)]
    #[holder(generate_deserialize)]
    pub struct PreDefinedTextFont {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub pre_defined_item: PreDefinedItem,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum PreDefinedTextFontAny {
        #[holder(use_place_holder)]
        # [holder (field = pre_defined_text_font)]
        PreDefinedTextFont(Box<PreDefinedTextFont>),
        #[holder(use_place_holder)]
        # [holder (field = draughting_pre_defined_text_font)]
        DraughtingPreDefinedTextFont(Box<DraughtingPreDefinedTextFont>),
    }
    impl Into<PreDefinedTextFontAny> for PreDefinedTextFont {
        fn into(self) -> PreDefinedTextFontAny {
            PreDefinedTextFontAny::PreDefinedTextFont(Box::new(self))
        }
    }
    impl Into<PreDefinedTextFontAny> for DraughtingPreDefinedTextFont {
        fn into(self) -> PreDefinedTextFontAny {
            PreDefinedTextFontAny::DraughtingPreDefinedTextFont(Box::new(self.into()))
        }
    }
    impl AsRef<PreDefinedTextFont> for PreDefinedTextFontAny {
        fn as_ref(&self) -> &PreDefinedTextFont {
            match self {
                PreDefinedTextFontAny::PreDefinedTextFont(x) => x.as_ref(),
                PreDefinedTextFontAny::DraughtingPreDefinedTextFont(x) => (**x).as_ref(),
            }
        }
    }
    impl AsRef<PreDefinedItem> for PreDefinedTextFontAny {
        fn as_ref(&self) -> &PreDefinedItem {
            match self {
                PreDefinedTextFontAny::PreDefinedTextFont(x) => {
                    AsRef::<PreDefinedTextFont>::as_ref(x).as_ref()
                }
                PreDefinedTextFontAny::DraughtingPreDefinedTextFont(x) => {
                    AsRef::<PreDefinedTextFont>::as_ref(x.as_ref()).as_ref()
                }
            }
        }
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = presentation_area)]
    #[holder(generate_deserialize)]
    pub struct PresentationArea {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub presentation_representation: PresentationRepresentation,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum PresentationAreaAny {
        #[holder(use_place_holder)]
        # [holder (field = presentation_area)]
        PresentationArea(Box<PresentationArea>),
        #[holder(use_place_holder)]
        # [holder (field = drawing_sheet_revision)]
        DrawingSheetRevision(Box<DrawingSheetRevision>),
    }
    impl Into<PresentationAreaAny> for PresentationArea {
        fn into(self) -> PresentationAreaAny {
            PresentationAreaAny::PresentationArea(Box::new(self))
        }
    }
    impl Into<PresentationAreaAny> for DrawingSheetRevision {
        fn into(self) -> PresentationAreaAny {
            PresentationAreaAny::DrawingSheetRevision(Box::new(self.into()))
        }
    }
    impl AsRef<PresentationArea> for PresentationAreaAny {
        fn as_ref(&self) -> &PresentationArea {
            match self {
                PresentationAreaAny::PresentationArea(x) => x.as_ref(),
                PresentationAreaAny::DrawingSheetRevision(x) => (**x).as_ref(),
            }
        }
    }
    impl AsRef<PresentationRepresentation> for PresentationAreaAny {
        fn as_ref(&self) -> &PresentationRepresentation {
            match self {
                PresentationAreaAny::PresentationArea(x) => {
                    AsRef::<PresentationArea>::as_ref(x).as_ref()
                }
                PresentationAreaAny::DrawingSheetRevision(x) => {
                    AsRef::<PresentationArea>::as_ref(x.as_ref()).as_ref()
                }
            }
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = presentation_layer_assignment)]
    #[holder(generate_deserialize)]
    pub struct PresentationLayerAssignment {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub description: Text,
        #[holder(use_place_holder)]
        pub assigned_items: Vec<LayeredItem>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = presentation_layer_usage)]
    #[holder(generate_deserialize)]
    pub struct PresentationLayerUsage {
        #[holder(use_place_holder)]
        pub assignment: PresentationLayerAssignment,
        #[holder(use_place_holder)]
        pub presentation: PresentationRepresentationAny,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = presentation_representation)]
    #[holder(generate_deserialize)]
    pub struct PresentationRepresentation {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub representation: Representation,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum PresentationRepresentationAny {
        #[holder(use_place_holder)]
        # [holder (field = presentation_representation)]
        PresentationRepresentation(Box<PresentationRepresentation>),
        #[holder(use_place_holder)]
        # [holder (field = presentation_area)]
        PresentationArea(Box<PresentationAreaAny>),
        #[holder(use_place_holder)]
        # [holder (field = presentation_view)]
        PresentationView(Box<PresentationView>),
    }
    impl Into<PresentationRepresentationAny> for PresentationRepresentation {
        fn into(self) -> PresentationRepresentationAny {
            PresentationRepresentationAny::PresentationRepresentation(Box::new(self))
        }
    }
    impl Into<PresentationRepresentationAny> for PresentationArea {
        fn into(self) -> PresentationRepresentationAny {
            PresentationRepresentationAny::PresentationArea(Box::new(self.into()))
        }
    }
    impl Into<PresentationRepresentationAny> for PresentationView {
        fn into(self) -> PresentationRepresentationAny {
            PresentationRepresentationAny::PresentationView(Box::new(self.into()))
        }
    }
    impl AsRef<PresentationRepresentation> for PresentationRepresentationAny {
        fn as_ref(&self) -> &PresentationRepresentation {
            match self {
                PresentationRepresentationAny::PresentationRepresentation(x) => x.as_ref(),
                PresentationRepresentationAny::PresentationArea(x) => (**x).as_ref(),
                PresentationRepresentationAny::PresentationView(x) => (**x).as_ref(),
            }
        }
    }
    impl AsRef<Representation> for PresentationRepresentationAny {
        fn as_ref(&self) -> &Representation {
            match self {
                PresentationRepresentationAny::PresentationRepresentation(x) => {
                    AsRef::<PresentationRepresentation>::as_ref(x).as_ref()
                }
                PresentationRepresentationAny::PresentationArea(x) => {
                    AsRef::<PresentationRepresentation>::as_ref(x.as_ref()).as_ref()
                }
                PresentationRepresentationAny::PresentationView(x) => {
                    AsRef::<PresentationRepresentation>::as_ref(x.as_ref()).as_ref()
                }
            }
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = presentation_set)]
    #[holder(generate_deserialize)]
    pub struct PresentationSet {}
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum PresentationSetAny {
        #[holder(use_place_holder)]
        # [holder (field = presentation_set)]
        PresentationSet(Box<PresentationSet>),
        #[holder(use_place_holder)]
        # [holder (field = drawing_revision)]
        DrawingRevision(Box<DrawingRevisionAny>),
    }
    impl Into<PresentationSetAny> for PresentationSet {
        fn into(self) -> PresentationSetAny {
            PresentationSetAny::PresentationSet(Box::new(self))
        }
    }
    impl Into<PresentationSetAny> for DrawingRevision {
        fn into(self) -> PresentationSetAny {
            PresentationSetAny::DrawingRevision(Box::new(self.into()))
        }
    }
    impl AsRef<PresentationSet> for PresentationSetAny {
        fn as_ref(&self) -> &PresentationSet {
            match self {
                PresentationSetAny::PresentationSet(x) => x.as_ref(),
                PresentationSetAny::DrawingRevision(x) => (**x).as_ref(),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = presentation_size)]
    #[holder(generate_deserialize)]
    pub struct PresentationSize {
        #[holder(use_place_holder)]
        pub unit: PresentationSizeAssignmentSelect,
        #[holder(use_place_holder)]
        pub size: PlanarBox,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = presentation_style_assignment)]
    #[holder(generate_deserialize)]
    pub struct PresentationStyleAssignment {
        #[holder(use_place_holder)]
        pub styles: Vec<PresentationStyleSelect>,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum PresentationStyleAssignmentAny {
        #[holder(use_place_holder)]
        # [holder (field = presentation_style_assignment)]
        PresentationStyleAssignment(Box<PresentationStyleAssignment>),
        #[holder(use_place_holder)]
        # [holder (field = presentation_style_by_context)]
        PresentationStyleByContext(Box<PresentationStyleByContext>),
    }
    impl Into<PresentationStyleAssignmentAny> for PresentationStyleAssignment {
        fn into(self) -> PresentationStyleAssignmentAny {
            PresentationStyleAssignmentAny::PresentationStyleAssignment(Box::new(self))
        }
    }
    impl Into<PresentationStyleAssignmentAny> for PresentationStyleByContext {
        fn into(self) -> PresentationStyleAssignmentAny {
            PresentationStyleAssignmentAny::PresentationStyleByContext(Box::new(self.into()))
        }
    }
    impl AsRef<PresentationStyleAssignment> for PresentationStyleAssignmentAny {
        fn as_ref(&self) -> &PresentationStyleAssignment {
            match self {
                PresentationStyleAssignmentAny::PresentationStyleAssignment(x) => x.as_ref(),
                PresentationStyleAssignmentAny::PresentationStyleByContext(x) => (**x).as_ref(),
            }
        }
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = presentation_style_by_context)]
    #[holder(generate_deserialize)]
    pub struct PresentationStyleByContext {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub presentation_style_assignment: PresentationStyleAssignment,
        #[holder(use_place_holder)]
        pub style_context: StyleContextSelect,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = presentation_view)]
    #[holder(generate_deserialize)]
    pub struct PresentationView {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub presentation_representation: PresentationRepresentation,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = presented_item)]
    #[holder(generate_deserialize)]
    pub struct PresentedItem {}
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum PresentedItemAny {
        #[holder(use_place_holder)]
        # [holder (field = presented_item)]
        PresentedItem(Box<PresentedItem>),
        #[holder(use_place_holder)]
        # [holder (field = draughting_presented_item)]
        DraughtingPresentedItem(Box<DraughtingPresentedItem>),
    }
    impl Into<PresentedItemAny> for PresentedItem {
        fn into(self) -> PresentedItemAny {
            PresentedItemAny::PresentedItem(Box::new(self))
        }
    }
    impl Into<PresentedItemAny> for DraughtingPresentedItem {
        fn into(self) -> PresentedItemAny {
            PresentedItemAny::DraughtingPresentedItem(Box::new(self.into()))
        }
    }
    impl AsRef<PresentedItem> for PresentedItemAny {
        fn as_ref(&self) -> &PresentedItem {
            match self {
                PresentedItemAny::PresentedItem(x) => x.as_ref(),
                PresentedItemAny::DraughtingPresentedItem(x) => (**x).as_ref(),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = presented_item_representation)]
    #[holder(generate_deserialize)]
    pub struct PresentedItemRepresentation {
        #[holder(use_place_holder)]
        pub presentation: PresentationRepresentationSelect,
        #[holder(use_place_holder)]
        pub item: PresentedItemAny,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = product)]
    #[holder(generate_deserialize)]
    pub struct Product {
        #[holder(use_place_holder)]
        pub id: Identifier,
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub description: Text,
        #[holder(use_place_holder)]
        pub frame_of_reference: Vec<ProductContext>,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = product_context)]
    #[holder(generate_deserialize)]
    pub struct ProductContext {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub application_context_element: ApplicationContextElement,
        #[holder(use_place_holder)]
        pub discipline_type: Label,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = product_definition)]
    #[holder(generate_deserialize)]
    pub struct ProductDefinition {
        #[holder(use_place_holder)]
        pub id: Identifier,
        #[holder(use_place_holder)]
        pub description: Text,
        #[holder(use_place_holder)]
        pub formation: ProductDefinitionFormation,
        #[holder(use_place_holder)]
        pub frame_of_reference: ProductDefinitionContext,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = product_definition_context)]
    #[holder(generate_deserialize)]
    pub struct ProductDefinitionContext {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub application_context_element: ApplicationContextElement,
        #[holder(use_place_holder)]
        pub life_cycle_stage: Label,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = product_definition_formation)]
    #[holder(generate_deserialize)]
    pub struct ProductDefinitionFormation {
        #[holder(use_place_holder)]
        pub id: Identifier,
        #[holder(use_place_holder)]
        pub description: Text,
        #[holder(use_place_holder)]
        pub of_product: Product,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = product_definition_shape)]
    #[holder(generate_deserialize)]
    pub struct ProductDefinitionShape {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub property_definition: PropertyDefinition,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = projection_curve)]
    #[holder(generate_deserialize)]
    pub struct ProjectionCurve {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub annotation_curve_occurrence: AnnotationCurveOccurrence,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = projection_directed_callout)]
    #[holder(generate_deserialize)]
    pub struct ProjectionDirectedCallout {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub draughting_callout: DraughtingCallout,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ProjectionDirectedCalloutAny {
        #[holder(use_place_holder)]
        # [holder (field = projection_directed_callout)]
        ProjectionDirectedCallout(Box<ProjectionDirectedCallout>),
        #[holder(use_place_holder)]
        # [holder (field = ordinate_dimension)]
        OrdinateDimension(Box<OrdinateDimension>),
    }
    impl Into<ProjectionDirectedCalloutAny> for ProjectionDirectedCallout {
        fn into(self) -> ProjectionDirectedCalloutAny {
            ProjectionDirectedCalloutAny::ProjectionDirectedCallout(Box::new(self))
        }
    }
    impl Into<ProjectionDirectedCalloutAny> for OrdinateDimension {
        fn into(self) -> ProjectionDirectedCalloutAny {
            ProjectionDirectedCalloutAny::OrdinateDimension(Box::new(self.into()))
        }
    }
    impl AsRef<ProjectionDirectedCallout> for ProjectionDirectedCalloutAny {
        fn as_ref(&self) -> &ProjectionDirectedCallout {
            match self {
                ProjectionDirectedCalloutAny::ProjectionDirectedCallout(x) => x.as_ref(),
                ProjectionDirectedCalloutAny::OrdinateDimension(x) => (**x).as_ref(),
            }
        }
    }
    impl AsRef<DraughtingCallout> for ProjectionDirectedCalloutAny {
        fn as_ref(&self) -> &DraughtingCallout {
            match self {
                ProjectionDirectedCalloutAny::ProjectionDirectedCallout(x) => {
                    AsRef::<ProjectionDirectedCallout>::as_ref(x).as_ref()
                }
                ProjectionDirectedCalloutAny::OrdinateDimension(x) => {
                    AsRef::<ProjectionDirectedCallout>::as_ref(x.as_ref()).as_ref()
                }
            }
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = property_definition)]
    #[holder(generate_deserialize)]
    pub struct PropertyDefinition {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub description: Text,
        #[holder(use_place_holder)]
        pub definition: CharacterizedDefinition,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum PropertyDefinitionAny {
        #[holder(use_place_holder)]
        # [holder (field = property_definition)]
        PropertyDefinition(Box<PropertyDefinition>),
        #[holder(use_place_holder)]
        # [holder (field = product_definition_shape)]
        ProductDefinitionShape(Box<ProductDefinitionShape>),
    }
    impl Into<PropertyDefinitionAny> for PropertyDefinition {
        fn into(self) -> PropertyDefinitionAny {
            PropertyDefinitionAny::PropertyDefinition(Box::new(self))
        }
    }
    impl Into<PropertyDefinitionAny> for ProductDefinitionShape {
        fn into(self) -> PropertyDefinitionAny {
            PropertyDefinitionAny::ProductDefinitionShape(Box::new(self.into()))
        }
    }
    impl AsRef<PropertyDefinition> for PropertyDefinitionAny {
        fn as_ref(&self) -> &PropertyDefinition {
            match self {
                PropertyDefinitionAny::PropertyDefinition(x) => x.as_ref(),
                PropertyDefinitionAny::ProductDefinitionShape(x) => (**x).as_ref(),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = property_definition_representation)]
    #[holder(generate_deserialize)]
    pub struct PropertyDefinitionRepresentation {
        #[holder(use_place_holder)]
        pub definition: PropertyDefinitionAny,
        #[holder(use_place_holder)]
        pub used_representation: RepresentationAny,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum PropertyDefinitionRepresentationAny {
        #[holder(use_place_holder)]
        # [holder (field = property_definition_representation)]
        PropertyDefinitionRepresentation(Box<PropertyDefinitionRepresentation>),
        #[holder(use_place_holder)]
        # [holder (field = shape_definition_representation)]
        ShapeDefinitionRepresentation(Box<ShapeDefinitionRepresentation>),
    }
    impl Into<PropertyDefinitionRepresentationAny> for PropertyDefinitionRepresentation {
        fn into(self) -> PropertyDefinitionRepresentationAny {
            PropertyDefinitionRepresentationAny::PropertyDefinitionRepresentation(Box::new(self))
        }
    }
    impl Into<PropertyDefinitionRepresentationAny> for ShapeDefinitionRepresentation {
        fn into(self) -> PropertyDefinitionRepresentationAny {
            PropertyDefinitionRepresentationAny::ShapeDefinitionRepresentation(Box::new(
                self.into(),
            ))
        }
    }
    impl AsRef<PropertyDefinitionRepresentation> for PropertyDefinitionRepresentationAny {
        fn as_ref(&self) -> &PropertyDefinitionRepresentation {
            match self {
                PropertyDefinitionRepresentationAny::PropertyDefinitionRepresentation(x) => {
                    x.as_ref()
                }
                PropertyDefinitionRepresentationAny::ShapeDefinitionRepresentation(x) => {
                    (**x).as_ref()
                }
            }
        }
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = quasi_uniform_curve)]
    #[holder(generate_deserialize)]
    pub struct QuasiUniformCurve {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub b_spline_curve: BSplineCurve,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = radius_dimension)]
    #[holder(generate_deserialize)]
    pub struct RadiusDimension {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub dimension_curve_directed_callout: DimensionCurveDirectedCallout,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = rational_b_spline_curve)]
    #[holder(generate_deserialize)]
    pub struct RationalBSplineCurve {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub b_spline_curve: BSplineCurve,
        pub weights_data: Vec<f64>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = representation)]
    #[holder(generate_deserialize)]
    pub struct Representation {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub items: Vec<RepresentationItemAny>,
        #[holder(use_place_holder)]
        pub context_of_items: RepresentationContextAny,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum RepresentationAny {
        #[holder(use_place_holder)]
        # [holder (field = representation)]
        Representation(Box<Representation>),
        #[holder(use_place_holder)]
        # [holder (field = draughting_model)]
        DraughtingModel(Box<DraughtingModel>),
        #[holder(use_place_holder)]
        # [holder (field = presentation_representation)]
        PresentationRepresentation(Box<PresentationRepresentationAny>),
        #[holder(use_place_holder)]
        # [holder (field = shape_representation)]
        ShapeRepresentation(Box<ShapeRepresentationAny>),
        #[holder(use_place_holder)]
        # [holder (field = symbol_representation)]
        SymbolRepresentation(Box<SymbolRepresentationAny>),
    }
    impl Into<RepresentationAny> for Representation {
        fn into(self) -> RepresentationAny {
            RepresentationAny::Representation(Box::new(self))
        }
    }
    impl Into<RepresentationAny> for DraughtingModel {
        fn into(self) -> RepresentationAny {
            RepresentationAny::DraughtingModel(Box::new(self.into()))
        }
    }
    impl Into<RepresentationAny> for PresentationRepresentation {
        fn into(self) -> RepresentationAny {
            RepresentationAny::PresentationRepresentation(Box::new(self.into()))
        }
    }
    impl Into<RepresentationAny> for ShapeRepresentation {
        fn into(self) -> RepresentationAny {
            RepresentationAny::ShapeRepresentation(Box::new(self.into()))
        }
    }
    impl Into<RepresentationAny> for SymbolRepresentation {
        fn into(self) -> RepresentationAny {
            RepresentationAny::SymbolRepresentation(Box::new(self.into()))
        }
    }
    impl AsRef<Representation> for RepresentationAny {
        fn as_ref(&self) -> &Representation {
            match self {
                RepresentationAny::Representation(x) => x.as_ref(),
                RepresentationAny::DraughtingModel(x) => (**x).as_ref(),
                RepresentationAny::PresentationRepresentation(x) => (**x).as_ref(),
                RepresentationAny::ShapeRepresentation(x) => (**x).as_ref(),
                RepresentationAny::SymbolRepresentation(x) => (**x).as_ref(),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = representation_context)]
    #[holder(generate_deserialize)]
    pub struct RepresentationContext {
        #[holder(use_place_holder)]
        pub context_identifier: Identifier,
        #[holder(use_place_holder)]
        pub context_type: Text,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum RepresentationContextAny {
        #[holder(use_place_holder)]
        # [holder (field = representation_context)]
        RepresentationContext(Box<RepresentationContext>),
        #[holder(use_place_holder)]
        # [holder (field = geometric_representation_context)]
        GeometricRepresentationContext(Box<GeometricRepresentationContext>),
        #[holder(use_place_holder)]
        # [holder (field = global_unit_assigned_context)]
        GlobalUnitAssignedContext(Box<GlobalUnitAssignedContext>),
    }
    impl Into<RepresentationContextAny> for RepresentationContext {
        fn into(self) -> RepresentationContextAny {
            RepresentationContextAny::RepresentationContext(Box::new(self))
        }
    }
    impl Into<RepresentationContextAny> for GeometricRepresentationContext {
        fn into(self) -> RepresentationContextAny {
            RepresentationContextAny::GeometricRepresentationContext(Box::new(self.into()))
        }
    }
    impl Into<RepresentationContextAny> for GlobalUnitAssignedContext {
        fn into(self) -> RepresentationContextAny {
            RepresentationContextAny::GlobalUnitAssignedContext(Box::new(self.into()))
        }
    }
    impl AsRef<RepresentationContext> for RepresentationContextAny {
        fn as_ref(&self) -> &RepresentationContext {
            match self {
                RepresentationContextAny::RepresentationContext(x) => x.as_ref(),
                RepresentationContextAny::GeometricRepresentationContext(x) => (**x).as_ref(),
                RepresentationContextAny::GlobalUnitAssignedContext(x) => (**x).as_ref(),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = representation_item)]
    #[holder(generate_deserialize)]
    pub struct RepresentationItem {
        #[holder(use_place_holder)]
        pub name: Label,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum RepresentationItemAny {
        #[holder(use_place_holder)]
        # [holder (field = representation_item)]
        RepresentationItem(Box<RepresentationItem>),
        #[holder(use_place_holder)]
        # [holder (field = geometric_representation_item)]
        GeometricRepresentationItem(Box<GeometricRepresentationItemAny>),
        #[holder(use_place_holder)]
        # [holder (field = mapped_item)]
        MappedItem(Box<MappedItemAny>),
        #[holder(use_place_holder)]
        # [holder (field = styled_item)]
        StyledItem(Box<StyledItemAny>),
    }
    impl Into<RepresentationItemAny> for RepresentationItem {
        fn into(self) -> RepresentationItemAny {
            RepresentationItemAny::RepresentationItem(Box::new(self))
        }
    }
    impl Into<RepresentationItemAny> for GeometricRepresentationItem {
        fn into(self) -> RepresentationItemAny {
            RepresentationItemAny::GeometricRepresentationItem(Box::new(self.into()))
        }
    }
    impl Into<RepresentationItemAny> for MappedItem {
        fn into(self) -> RepresentationItemAny {
            RepresentationItemAny::MappedItem(Box::new(self.into()))
        }
    }
    impl Into<RepresentationItemAny> for StyledItem {
        fn into(self) -> RepresentationItemAny {
            RepresentationItemAny::StyledItem(Box::new(self.into()))
        }
    }
    impl AsRef<RepresentationItem> for RepresentationItemAny {
        fn as_ref(&self) -> &RepresentationItem {
            match self {
                RepresentationItemAny::RepresentationItem(x) => x.as_ref(),
                RepresentationItemAny::GeometricRepresentationItem(x) => (**x).as_ref(),
                RepresentationItemAny::MappedItem(x) => (**x).as_ref(),
                RepresentationItemAny::StyledItem(x) => (**x).as_ref(),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = representation_map)]
    #[holder(generate_deserialize)]
    pub struct RepresentationMap {
        #[holder(use_place_holder)]
        pub mapping_origin: RepresentationItemAny,
        #[holder(use_place_holder)]
        pub mapped_representation: RepresentationAny,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum RepresentationMapAny {
        #[holder(use_place_holder)]
        # [holder (field = representation_map)]
        RepresentationMap(Box<RepresentationMap>),
        #[holder(use_place_holder)]
        # [holder (field = camera_usage)]
        CameraUsage(Box<CameraUsage>),
        #[holder(use_place_holder)]
        # [holder (field = symbol_representation_map)]
        SymbolRepresentationMap(Box<SymbolRepresentationMap>),
    }
    impl Into<RepresentationMapAny> for RepresentationMap {
        fn into(self) -> RepresentationMapAny {
            RepresentationMapAny::RepresentationMap(Box::new(self))
        }
    }
    impl Into<RepresentationMapAny> for CameraUsage {
        fn into(self) -> RepresentationMapAny {
            RepresentationMapAny::CameraUsage(Box::new(self.into()))
        }
    }
    impl Into<RepresentationMapAny> for SymbolRepresentationMap {
        fn into(self) -> RepresentationMapAny {
            RepresentationMapAny::SymbolRepresentationMap(Box::new(self.into()))
        }
    }
    impl AsRef<RepresentationMap> for RepresentationMapAny {
        fn as_ref(&self) -> &RepresentationMap {
            match self {
                RepresentationMapAny::RepresentationMap(x) => x.as_ref(),
                RepresentationMapAny::CameraUsage(x) => (**x).as_ref(),
                RepresentationMapAny::SymbolRepresentationMap(x) => (**x).as_ref(),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = security_classification)]
    #[holder(generate_deserialize)]
    pub struct SecurityClassification {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub purpose: Text,
        #[holder(use_place_holder)]
        pub security_level: SecurityClassificationLevel,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = security_classification_assignment)]
    #[holder(generate_deserialize)]
    pub struct SecurityClassificationAssignment {
        #[holder(use_place_holder)]
        pub assigned_security_classification: SecurityClassification,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum SecurityClassificationAssignmentAny {
        #[holder(use_place_holder)]
        # [holder (field = security_classification_assignment)]
        SecurityClassificationAssignment(Box<SecurityClassificationAssignment>),
        #[holder(use_place_holder)]
        # [holder (field = draughting_security_classification_assignment)]
        DraughtingSecurityClassificationAssignment(Box<DraughtingSecurityClassificationAssignment>),
    }
    impl Into<SecurityClassificationAssignmentAny> for SecurityClassificationAssignment {
        fn into(self) -> SecurityClassificationAssignmentAny {
            SecurityClassificationAssignmentAny::SecurityClassificationAssignment(Box::new(self))
        }
    }
    impl Into<SecurityClassificationAssignmentAny> for DraughtingSecurityClassificationAssignment {
        fn into(self) -> SecurityClassificationAssignmentAny {
            SecurityClassificationAssignmentAny::DraughtingSecurityClassificationAssignment(
                Box::new(self.into()),
            )
        }
    }
    impl AsRef<SecurityClassificationAssignment> for SecurityClassificationAssignmentAny {
        fn as_ref(&self) -> &SecurityClassificationAssignment {
            match self {
                SecurityClassificationAssignmentAny::SecurityClassificationAssignment(x) => {
                    x.as_ref()
                }
                SecurityClassificationAssignmentAny::DraughtingSecurityClassificationAssignment(
                    x,
                ) => (**x).as_ref(),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = security_classification_level)]
    #[holder(generate_deserialize)]
    pub struct SecurityClassificationLevel {
        #[holder(use_place_holder)]
        pub name: Label,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = shape_definition_representation)]
    #[holder(generate_deserialize)]
    pub struct ShapeDefinitionRepresentation {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub property_definition_representation: PropertyDefinitionRepresentation,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = shape_representation)]
    #[holder(generate_deserialize)]
    pub struct ShapeRepresentation {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub representation: Representation,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ShapeRepresentationAny {
        #[holder(use_place_holder)]
        # [holder (field = shape_representation)]
        ShapeRepresentation(Box<ShapeRepresentation>),
        #[holder(use_place_holder)]
        # [holder (field = geometrically_bounded_2d_wireframe_representation)]
        GeometricallyBounded2DWireframeRepresentation(
            Box<GeometricallyBounded2DWireframeRepresentation>,
        ),
    }
    impl Into<ShapeRepresentationAny> for ShapeRepresentation {
        fn into(self) -> ShapeRepresentationAny {
            ShapeRepresentationAny::ShapeRepresentation(Box::new(self))
        }
    }
    impl Into<ShapeRepresentationAny> for GeometricallyBounded2DWireframeRepresentation {
        fn into(self) -> ShapeRepresentationAny {
            ShapeRepresentationAny::GeometricallyBounded2DWireframeRepresentation(Box::new(
                self.into(),
            ))
        }
    }
    impl AsRef<ShapeRepresentation> for ShapeRepresentationAny {
        fn as_ref(&self) -> &ShapeRepresentation {
            match self {
                ShapeRepresentationAny::ShapeRepresentation(x) => x.as_ref(),
                ShapeRepresentationAny::GeometricallyBounded2DWireframeRepresentation(x) => {
                    (**x).as_ref()
                }
            }
        }
    }
    impl AsRef<Representation> for ShapeRepresentationAny {
        fn as_ref(&self) -> &Representation {
            match self {
                ShapeRepresentationAny::ShapeRepresentation(x) => {
                    AsRef::<ShapeRepresentation>::as_ref(x).as_ref()
                }
                ShapeRepresentationAny::GeometricallyBounded2DWireframeRepresentation(x) => {
                    AsRef::<ShapeRepresentation>::as_ref(x.as_ref()).as_ref()
                }
            }
        }
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = si_unit)]
    #[holder(generate_deserialize)]
    pub struct SiUnit {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub named_unit: NamedUnit,
        pub prefix: Option<SiPrefix>,
        pub name: SiUnitName,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = structured_dimension_callout)]
    #[holder(generate_deserialize)]
    pub struct StructuredDimensionCallout {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub draughting_callout: DraughtingCallout,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = styled_item)]
    #[holder(generate_deserialize)]
    pub struct StyledItem {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub representation_item: RepresentationItem,
        #[holder(use_place_holder)]
        pub styles: Vec<PresentationStyleAssignmentAny>,
        #[holder(use_place_holder)]
        pub item: RepresentationItemAny,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum StyledItemAny {
        #[holder(use_place_holder)]
        # [holder (field = styled_item)]
        StyledItem(Box<StyledItem>),
        #[holder(use_place_holder)]
        # [holder (field = annotation_occurrence)]
        AnnotationOccurrence(Box<AnnotationOccurrenceAny>),
    }
    impl Into<StyledItemAny> for StyledItem {
        fn into(self) -> StyledItemAny {
            StyledItemAny::StyledItem(Box::new(self))
        }
    }
    impl Into<StyledItemAny> for AnnotationOccurrence {
        fn into(self) -> StyledItemAny {
            StyledItemAny::AnnotationOccurrence(Box::new(self.into()))
        }
    }
    impl AsRef<StyledItem> for StyledItemAny {
        fn as_ref(&self) -> &StyledItem {
            match self {
                StyledItemAny::StyledItem(x) => x.as_ref(),
                StyledItemAny::AnnotationOccurrence(x) => (**x).as_ref(),
            }
        }
    }
    impl AsRef<RepresentationItem> for StyledItemAny {
        fn as_ref(&self) -> &RepresentationItem {
            match self {
                StyledItemAny::StyledItem(x) => AsRef::<StyledItem>::as_ref(x).as_ref(),
                StyledItemAny::AnnotationOccurrence(x) => {
                    AsRef::<StyledItem>::as_ref(x.as_ref()).as_ref()
                }
            }
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = symbol_colour)]
    #[holder(generate_deserialize)]
    pub struct SymbolColour {
        #[holder(use_place_holder)]
        pub colour_of_symbol: ColourAny,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = symbol_representation)]
    #[holder(generate_deserialize)]
    pub struct SymbolRepresentation {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub representation: Representation,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum SymbolRepresentationAny {
        #[holder(use_place_holder)]
        # [holder (field = symbol_representation)]
        SymbolRepresentation(Box<SymbolRepresentation>),
        #[holder(use_place_holder)]
        # [holder (field = draughting_subfigure_representation)]
        DraughtingSubfigureRepresentation(Box<DraughtingSubfigureRepresentation>),
        #[holder(use_place_holder)]
        # [holder (field = draughting_symbol_representation)]
        DraughtingSymbolRepresentation(Box<DraughtingSymbolRepresentationAny>),
    }
    impl Into<SymbolRepresentationAny> for SymbolRepresentation {
        fn into(self) -> SymbolRepresentationAny {
            SymbolRepresentationAny::SymbolRepresentation(Box::new(self))
        }
    }
    impl Into<SymbolRepresentationAny> for DraughtingSubfigureRepresentation {
        fn into(self) -> SymbolRepresentationAny {
            SymbolRepresentationAny::DraughtingSubfigureRepresentation(Box::new(self.into()))
        }
    }
    impl Into<SymbolRepresentationAny> for DraughtingSymbolRepresentation {
        fn into(self) -> SymbolRepresentationAny {
            SymbolRepresentationAny::DraughtingSymbolRepresentation(Box::new(self.into()))
        }
    }
    impl AsRef<SymbolRepresentation> for SymbolRepresentationAny {
        fn as_ref(&self) -> &SymbolRepresentation {
            match self {
                SymbolRepresentationAny::SymbolRepresentation(x) => x.as_ref(),
                SymbolRepresentationAny::DraughtingSubfigureRepresentation(x) => (**x).as_ref(),
                SymbolRepresentationAny::DraughtingSymbolRepresentation(x) => (**x).as_ref(),
            }
        }
    }
    impl AsRef<Representation> for SymbolRepresentationAny {
        fn as_ref(&self) -> &Representation {
            match self {
                SymbolRepresentationAny::SymbolRepresentation(x) => {
                    AsRef::<SymbolRepresentation>::as_ref(x).as_ref()
                }
                SymbolRepresentationAny::DraughtingSubfigureRepresentation(x) => {
                    AsRef::<SymbolRepresentation>::as_ref(x.as_ref()).as_ref()
                }
                SymbolRepresentationAny::DraughtingSymbolRepresentation(x) => {
                    AsRef::<SymbolRepresentation>::as_ref(x.as_ref()).as_ref()
                }
            }
        }
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = symbol_representation_map)]
    #[holder(generate_deserialize)]
    pub struct SymbolRepresentationMap {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub representation_map: RepresentationMap,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = symbol_style)]
    #[holder(generate_deserialize)]
    pub struct SymbolStyle {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub style_of_symbol: SymbolStyleSelect,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = symbol_target)]
    #[holder(generate_deserialize)]
    pub struct SymbolTarget {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub geometric_representation_item: GeometricRepresentationItem,
        #[holder(use_place_holder)]
        pub placement: Axis2Placement,
        #[holder(use_place_holder)]
        pub x_scale: PositiveRatioMeasure,
        #[holder(use_place_holder)]
        pub y_scale: PositiveRatioMeasure,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = terminator_symbol)]
    #[holder(generate_deserialize)]
    pub struct TerminatorSymbol {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub annotation_symbol_occurrence: AnnotationSymbolOccurrence,
        #[holder(use_place_holder)]
        pub annotated_curve: AnnotationCurveOccurrenceAny,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum TerminatorSymbolAny {
        #[holder(use_place_holder)]
        # [holder (field = terminator_symbol)]
        TerminatorSymbol(Box<TerminatorSymbol>),
        #[holder(use_place_holder)]
        # [holder (field = dimension_curve_terminator)]
        DimensionCurveTerminator(Box<DimensionCurveTerminator>),
        #[holder(use_place_holder)]
        # [holder (field = leader_terminator)]
        LeaderTerminator(Box<LeaderTerminator>),
    }
    impl Into<TerminatorSymbolAny> for TerminatorSymbol {
        fn into(self) -> TerminatorSymbolAny {
            TerminatorSymbolAny::TerminatorSymbol(Box::new(self))
        }
    }
    impl Into<TerminatorSymbolAny> for DimensionCurveTerminator {
        fn into(self) -> TerminatorSymbolAny {
            TerminatorSymbolAny::DimensionCurveTerminator(Box::new(self.into()))
        }
    }
    impl Into<TerminatorSymbolAny> for LeaderTerminator {
        fn into(self) -> TerminatorSymbolAny {
            TerminatorSymbolAny::LeaderTerminator(Box::new(self.into()))
        }
    }
    impl AsRef<TerminatorSymbol> for TerminatorSymbolAny {
        fn as_ref(&self) -> &TerminatorSymbol {
            match self {
                TerminatorSymbolAny::TerminatorSymbol(x) => x.as_ref(),
                TerminatorSymbolAny::DimensionCurveTerminator(x) => (**x).as_ref(),
                TerminatorSymbolAny::LeaderTerminator(x) => (**x).as_ref(),
            }
        }
    }
    impl AsRef<AnnotationSymbolOccurrence> for TerminatorSymbolAny {
        fn as_ref(&self) -> &AnnotationSymbolOccurrence {
            match self {
                TerminatorSymbolAny::TerminatorSymbol(x) => {
                    AsRef::<TerminatorSymbol>::as_ref(x).as_ref()
                }
                TerminatorSymbolAny::DimensionCurveTerminator(x) => {
                    AsRef::<TerminatorSymbol>::as_ref(x.as_ref()).as_ref()
                }
                TerminatorSymbolAny::LeaderTerminator(x) => {
                    AsRef::<TerminatorSymbol>::as_ref(x.as_ref()).as_ref()
                }
            }
        }
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = text_literal)]
    #[holder(generate_deserialize)]
    pub struct TextLiteral {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub geometric_representation_item: GeometricRepresentationItem,
        #[holder(use_place_holder)]
        pub literal: PresentableText,
        #[holder(use_place_holder)]
        pub placement: Axis2Placement,
        #[holder(use_place_holder)]
        pub alignment: TextAlignment,
        pub path: TextPath,
        #[holder(use_place_holder)]
        pub font: FontSelect,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum TextLiteralAny {
        #[holder(use_place_holder)]
        # [holder (field = text_literal)]
        TextLiteral(Box<TextLiteral>),
        #[holder(use_place_holder)]
        # [holder (field = text_literal_with_associated_curves)]
        TextLiteralWithAssociatedCurves(Box<TextLiteralWithAssociatedCurves>),
        #[holder(use_place_holder)]
        # [holder (field = text_literal_with_blanking_box)]
        TextLiteralWithBlankingBox(Box<TextLiteralWithBlankingBox>),
        #[holder(use_place_holder)]
        # [holder (field = text_literal_with_delineation)]
        TextLiteralWithDelineation(Box<TextLiteralWithDelineationAny>),
        #[holder(use_place_holder)]
        # [holder (field = text_literal_with_extent)]
        TextLiteralWithExtent(Box<TextLiteralWithExtent>),
    }
    impl Into<TextLiteralAny> for TextLiteral {
        fn into(self) -> TextLiteralAny {
            TextLiteralAny::TextLiteral(Box::new(self))
        }
    }
    impl Into<TextLiteralAny> for TextLiteralWithAssociatedCurves {
        fn into(self) -> TextLiteralAny {
            TextLiteralAny::TextLiteralWithAssociatedCurves(Box::new(self.into()))
        }
    }
    impl Into<TextLiteralAny> for TextLiteralWithBlankingBox {
        fn into(self) -> TextLiteralAny {
            TextLiteralAny::TextLiteralWithBlankingBox(Box::new(self.into()))
        }
    }
    impl Into<TextLiteralAny> for TextLiteralWithDelineation {
        fn into(self) -> TextLiteralAny {
            TextLiteralAny::TextLiteralWithDelineation(Box::new(self.into()))
        }
    }
    impl Into<TextLiteralAny> for TextLiteralWithExtent {
        fn into(self) -> TextLiteralAny {
            TextLiteralAny::TextLiteralWithExtent(Box::new(self.into()))
        }
    }
    impl AsRef<TextLiteral> for TextLiteralAny {
        fn as_ref(&self) -> &TextLiteral {
            match self {
                TextLiteralAny::TextLiteral(x) => x.as_ref(),
                TextLiteralAny::TextLiteralWithAssociatedCurves(x) => (**x).as_ref(),
                TextLiteralAny::TextLiteralWithBlankingBox(x) => (**x).as_ref(),
                TextLiteralAny::TextLiteralWithDelineation(x) => (**x).as_ref(),
                TextLiteralAny::TextLiteralWithExtent(x) => (**x).as_ref(),
            }
        }
    }
    impl AsRef<GeometricRepresentationItem> for TextLiteralAny {
        fn as_ref(&self) -> &GeometricRepresentationItem {
            match self {
                TextLiteralAny::TextLiteral(x) => AsRef::<TextLiteral>::as_ref(x).as_ref(),
                TextLiteralAny::TextLiteralWithAssociatedCurves(x) => {
                    AsRef::<TextLiteral>::as_ref(x.as_ref()).as_ref()
                }
                TextLiteralAny::TextLiteralWithBlankingBox(x) => {
                    AsRef::<TextLiteral>::as_ref(x.as_ref()).as_ref()
                }
                TextLiteralAny::TextLiteralWithDelineation(x) => {
                    AsRef::<TextLiteral>::as_ref(x.as_ref()).as_ref()
                }
                TextLiteralAny::TextLiteralWithExtent(x) => {
                    AsRef::<TextLiteral>::as_ref(x.as_ref()).as_ref()
                }
            }
        }
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = text_literal_with_associated_curves)]
    #[holder(generate_deserialize)]
    pub struct TextLiteralWithAssociatedCurves {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub text_literal: TextLiteral,
        #[holder(use_place_holder)]
        pub associated_curves: Vec<CurveAny>,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = text_literal_with_blanking_box)]
    #[holder(generate_deserialize)]
    pub struct TextLiteralWithBlankingBox {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub text_literal: TextLiteral,
        #[holder(use_place_holder)]
        pub blanking: PlanarBox,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = text_literal_with_delineation)]
    #[holder(generate_deserialize)]
    pub struct TextLiteralWithDelineation {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub text_literal: TextLiteral,
        #[holder(use_place_holder)]
        pub delineation: TextDelineation,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum TextLiteralWithDelineationAny {
        #[holder(use_place_holder)]
        # [holder (field = text_literal_with_delineation)]
        TextLiteralWithDelineation(Box<TextLiteralWithDelineation>),
        #[holder(use_place_holder)]
        # [holder (field = draughting_text_literal_with_delineation)]
        DraughtingTextLiteralWithDelineation(Box<DraughtingTextLiteralWithDelineation>),
    }
    impl Into<TextLiteralWithDelineationAny> for TextLiteralWithDelineation {
        fn into(self) -> TextLiteralWithDelineationAny {
            TextLiteralWithDelineationAny::TextLiteralWithDelineation(Box::new(self))
        }
    }
    impl Into<TextLiteralWithDelineationAny> for DraughtingTextLiteralWithDelineation {
        fn into(self) -> TextLiteralWithDelineationAny {
            TextLiteralWithDelineationAny::DraughtingTextLiteralWithDelineation(Box::new(
                self.into(),
            ))
        }
    }
    impl AsRef<TextLiteralWithDelineation> for TextLiteralWithDelineationAny {
        fn as_ref(&self) -> &TextLiteralWithDelineation {
            match self {
                TextLiteralWithDelineationAny::TextLiteralWithDelineation(x) => x.as_ref(),
                TextLiteralWithDelineationAny::DraughtingTextLiteralWithDelineation(x) => {
                    (**x).as_ref()
                }
            }
        }
    }
    impl AsRef<TextLiteral> for TextLiteralWithDelineationAny {
        fn as_ref(&self) -> &TextLiteral {
            match self {
                TextLiteralWithDelineationAny::TextLiteralWithDelineation(x) => {
                    AsRef::<TextLiteralWithDelineation>::as_ref(x).as_ref()
                }
                TextLiteralWithDelineationAny::DraughtingTextLiteralWithDelineation(x) => {
                    AsRef::<TextLiteralWithDelineation>::as_ref(x.as_ref()).as_ref()
                }
            }
        }
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = text_literal_with_extent)]
    #[holder(generate_deserialize)]
    pub struct TextLiteralWithExtent {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub text_literal: TextLiteral,
        #[holder(use_place_holder)]
        pub extent: PlanarExtentAny,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = text_style)]
    #[holder(generate_deserialize)]
    pub struct TextStyle {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub character_appearance: CharacterStyleSelect,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum TextStyleAny {
        #[holder(use_place_holder)]
        # [holder (field = text_style)]
        TextStyle(Box<TextStyle>),
        #[holder(use_place_holder)]
        # [holder (field = text_style_with_box_characteristics)]
        TextStyleWithBoxCharacteristics(Box<TextStyleWithBoxCharacteristics>),
        #[holder(use_place_holder)]
        # [holder (field = text_style_with_mirror)]
        TextStyleWithMirror(Box<TextStyleWithMirror>),
    }
    impl Into<TextStyleAny> for TextStyle {
        fn into(self) -> TextStyleAny {
            TextStyleAny::TextStyle(Box::new(self))
        }
    }
    impl Into<TextStyleAny> for TextStyleWithBoxCharacteristics {
        fn into(self) -> TextStyleAny {
            TextStyleAny::TextStyleWithBoxCharacteristics(Box::new(self.into()))
        }
    }
    impl Into<TextStyleAny> for TextStyleWithMirror {
        fn into(self) -> TextStyleAny {
            TextStyleAny::TextStyleWithMirror(Box::new(self.into()))
        }
    }
    impl AsRef<TextStyle> for TextStyleAny {
        fn as_ref(&self) -> &TextStyle {
            match self {
                TextStyleAny::TextStyle(x) => x.as_ref(),
                TextStyleAny::TextStyleWithBoxCharacteristics(x) => (**x).as_ref(),
                TextStyleAny::TextStyleWithMirror(x) => (**x).as_ref(),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = text_style_for_defined_font)]
    #[holder(generate_deserialize)]
    pub struct TextStyleForDefinedFont {
        #[holder(use_place_holder)]
        pub text_colour: ColourAny,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = text_style_with_box_characteristics)]
    #[holder(generate_deserialize)]
    pub struct TextStyleWithBoxCharacteristics {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub text_style: TextStyle,
        #[holder(use_place_holder)]
        pub characteristics: Vec<BoxCharacteristicSelect>,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = text_style_with_mirror)]
    #[holder(generate_deserialize)]
    pub struct TextStyleWithMirror {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub text_style: TextStyle,
        #[holder(use_place_holder)]
        pub mirror_placement: Axis2Placement,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = trimmed_curve)]
    #[holder(generate_deserialize)]
    pub struct TrimmedCurve {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub bounded_curve: BoundedCurve,
        #[holder(use_place_holder)]
        pub basis_curve: CurveAny,
        #[holder(use_place_holder)]
        pub trim_1: Vec<TrimmingSelect>,
        #[holder(use_place_holder)]
        pub trim_2: Vec<TrimmingSelect>,
        pub sense_agreement: bool,
        pub master_representation: TrimmingPreference,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = two_direction_repeat_factor)]
    #[holder(generate_deserialize)]
    pub struct TwoDirectionRepeatFactor {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub one_direction_repeat_factor: OneDirectionRepeatFactor,
        #[holder(use_place_holder)]
        pub second_repeat_factor: Vector,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = uniform_curve)]
    #[holder(generate_deserialize)]
    pub struct UniformCurve {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub b_spline_curve: BSplineCurve,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = vector)]
    #[holder(generate_deserialize)]
    pub struct Vector {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub geometric_representation_item: GeometricRepresentationItem,
        #[holder(use_place_holder)]
        pub orientation: Direction,
        #[holder(use_place_holder)]
        pub magnitude: LengthMeasure,
    }
}

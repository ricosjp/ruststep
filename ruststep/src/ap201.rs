#![allow(dead_code)]
pub mod explicit_draughting {
    use crate::{as_holder, error::Result, primitive::*, tables::*, Holder, TableInit};
    use derive_more::*;
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
        BoxHeight(BoxHeight),
        BoxWidth(BoxWidth),
        BoxSlantAngle(BoxSlantAngle),
        BoxRotateAngle(BoxRotateAngle),
    }
    #[derive(
        Clone,
        Debug,
        PartialEq,
        AsRef,
        Deref,
        DerefMut,
        :: serde :: Serialize,
        :: serde :: Deserialize,
    )]
    pub struct BoxHeight(pub PositiveRatioMeasure);
    #[derive(
        Clone,
        Debug,
        PartialEq,
        AsRef,
        Deref,
        DerefMut,
        :: serde :: Serialize,
        :: serde :: Deserialize,
    )]
    pub struct BoxRotateAngle(pub PlaneAngleMeasure);
    #[derive(
        Clone,
        Debug,
        PartialEq,
        AsRef,
        Deref,
        DerefMut,
        :: serde :: Serialize,
        :: serde :: Deserialize,
    )]
    pub struct BoxSlantAngle(pub PlaneAngleMeasure);
    #[derive(
        Clone,
        Debug,
        PartialEq,
        AsRef,
        Deref,
        DerefMut,
        :: serde :: Serialize,
        :: serde :: Deserialize,
    )]
    pub struct BoxWidth(pub PositiveRatioMeasure);
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum CharacterSpacingSelect {
        LengthMeasure(LengthMeasure),
        RatioMeasure(RatioMeasure),
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
        Clone,
        Debug,
        PartialEq,
        AsRef,
        Deref,
        DerefMut,
        :: serde :: Serialize,
        :: serde :: Deserialize,
    )]
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
        Clone,
        Debug,
        PartialEq,
        AsRef,
        Deref,
        DerefMut,
        :: serde :: Serialize,
        :: serde :: Deserialize,
    )]
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
        Clone,
        Debug,
        PartialEq,
        AsRef,
        Deref,
        DerefMut,
        :: serde :: Serialize,
        :: serde :: Deserialize,
    )]
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
        Clone,
        Debug,
        PartialEq,
        AsRef,
        Deref,
        DerefMut,
        :: serde :: Serialize,
        :: serde :: Deserialize,
    )]
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
        Clone,
        Debug,
        PartialEq,
        AsRef,
        Deref,
        DerefMut,
        :: serde :: Serialize,
        :: serde :: Deserialize,
    )]
    pub struct LengthMeasure(pub f64);
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum MeasureValue {
        LengthMeasure(LengthMeasure),
        PlaneAngleMeasure(PlaneAngleMeasure),
        RatioMeasure(RatioMeasure),
        ParameterValue(ParameterValue),
        PositiveLengthMeasure(PositiveLengthMeasure),
        PositiveRatioMeasure(PositiveRatioMeasure),
    }
    #[derive(
        Clone,
        Debug,
        PartialEq,
        AsRef,
        Deref,
        DerefMut,
        :: serde :: Serialize,
        :: serde :: Deserialize,
    )]
    pub struct MonthInYearNumber(pub i64);
    #[derive(Debug, Clone, PartialEq, :: serde :: Deserialize)]
    pub enum NullStyle {
        Null,
    }
    #[derive(
        Clone,
        Debug,
        PartialEq,
        AsRef,
        Deref,
        DerefMut,
        :: serde :: Serialize,
        :: serde :: Deserialize,
    )]
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
        Clone,
        Debug,
        PartialEq,
        AsRef,
        Deref,
        DerefMut,
        :: serde :: Serialize,
        :: serde :: Deserialize,
    )]
    pub struct PlaneAngleMeasure(pub f64);
    #[derive(
        Clone,
        Debug,
        PartialEq,
        AsRef,
        Deref,
        DerefMut,
        :: serde :: Serialize,
        :: serde :: Deserialize,
    )]
    pub struct PositiveLengthMeasure(pub LengthMeasure);
    #[derive(
        Clone,
        Debug,
        PartialEq,
        AsRef,
        Deref,
        DerefMut,
        :: serde :: Serialize,
        :: serde :: Deserialize,
    )]
    pub struct PositiveRatioMeasure(pub RatioMeasure);
    #[derive(
        Clone,
        Debug,
        PartialEq,
        AsRef,
        Deref,
        DerefMut,
        :: serde :: Serialize,
        :: serde :: Deserialize,
    )]
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
        Clone,
        Debug,
        PartialEq,
        AsRef,
        Deref,
        DerefMut,
        :: serde :: Serialize,
        :: serde :: Deserialize,
    )]
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
        PositiveLengthMeasure(PositiveLengthMeasure),
        #[holder(use_place_holder)]
        MeasureWithUnit(MeasureWithUnitAny),
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum SourceItem {
        Identifier(Identifier),
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
        Clone,
        Debug,
        PartialEq,
        AsRef,
        Deref,
        DerefMut,
        :: serde :: Serialize,
        :: serde :: Deserialize,
    )]
    pub struct Text(pub String);
    #[derive(
        Clone,
        Debug,
        PartialEq,
        AsRef,
        Deref,
        DerefMut,
        :: serde :: Serialize,
        :: serde :: Deserialize,
    )]
    pub struct TextAlignment(pub Label);
    #[derive(
        Clone,
        Debug,
        PartialEq,
        AsRef,
        Deref,
        DerefMut,
        :: serde :: Serialize,
        :: serde :: Deserialize,
    )]
    pub struct TextDelineation(pub Label);
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
        ParameterValue(ParameterValue),
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
        Clone,
        Debug,
        PartialEq,
        AsRef,
        Deref,
        DerefMut,
        :: serde :: Serialize,
        :: serde :: Deserialize,
    )]
    pub struct YearNumber(pub i64);
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = address)]
    #[holder(generate_deserialize)]
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
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum AddressAny {
        #[holder(use_place_holder)]
        # [holder (field = organizational_address)]
        OrganizationalAddress(Box<OrganizationalAddress>),
        #[holder(use_place_holder)]
        # [holder (field = personal_address)]
        PersonalAddress(Box<PersonalAddress>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = angular_dimension)]
    #[holder(generate_deserialize)]
    pub struct AngularDimension {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = annotation_curve_occurrence)]
    #[holder(generate_deserialize)]
    pub struct AnnotationCurveOccurrence {}
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum AnnotationCurveOccurrenceAny {
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
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = annotation_fill_area)]
    #[holder(generate_deserialize)]
    pub struct AnnotationFillArea {
        #[holder(use_place_holder)]
        pub boundaries: Vec<CurveAny>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = annotation_fill_area_occurrence)]
    #[holder(generate_deserialize)]
    pub struct AnnotationFillAreaOccurrence {
        #[holder(use_place_holder)]
        pub fill_style_target: PointAny,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = annotation_occurrence)]
    #[holder(generate_deserialize)]
    pub struct AnnotationOccurrence {}
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum AnnotationOccurrenceAny {
        #[holder(use_place_holder)]
        # [holder (field = annotation_curve_occurrence)]
        AnnotationCurveOccurrence(Box<AnnotationCurveOccurrence>),
        #[holder(use_place_holder)]
        # [holder (field = annotation_fill_area_occurrence)]
        AnnotationFillAreaOccurrence(Box<AnnotationFillAreaOccurrence>),
        #[holder(use_place_holder)]
        # [holder (field = annotation_symbol_occurrence)]
        AnnotationSymbolOccurrence(Box<AnnotationSymbolOccurrence>),
        #[holder(use_place_holder)]
        # [holder (field = annotation_text_occurrence)]
        AnnotationTextOccurrence(Box<AnnotationTextOccurrence>),
        #[holder(use_place_holder)]
        # [holder (field = draughting_annotation_occurrence)]
        DraughtingAnnotationOccurrence(Box<DraughtingAnnotationOccurrence>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = annotation_subfigure_occurrence)]
    #[holder(generate_deserialize)]
    pub struct AnnotationSubfigureOccurrence {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = annotation_symbol)]
    #[holder(generate_deserialize)]
    pub struct AnnotationSymbol {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = annotation_symbol_occurrence)]
    #[holder(generate_deserialize)]
    pub struct AnnotationSymbolOccurrence {}
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum AnnotationSymbolOccurrenceAny {
        #[holder(use_place_holder)]
        # [holder (field = annotation_subfigure_occurrence)]
        AnnotationSubfigureOccurrence(Box<AnnotationSubfigureOccurrence>),
        #[holder(use_place_holder)]
        # [holder (field = terminator_symbol)]
        TerminatorSymbol(Box<TerminatorSymbol>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = annotation_text)]
    #[holder(generate_deserialize)]
    pub struct AnnotationText {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = annotation_text_occurrence)]
    #[holder(generate_deserialize)]
    pub struct AnnotationTextOccurrence {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = application_context)]
    #[holder(generate_deserialize)]
    pub struct ApplicationContext {
        pub application: Text,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = application_context_element)]
    #[holder(generate_deserialize)]
    pub struct ApplicationContextElement {
        pub name: Label,
        #[holder(use_place_holder)]
        pub frame_of_reference: ApplicationContext,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ApplicationContextElementAny {
        #[holder(use_place_holder)]
        # [holder (field = product_context)]
        ProductContext(Box<ProductContext>),
        #[holder(use_place_holder)]
        # [holder (field = product_definition_context)]
        ProductDefinitionContext(Box<ProductDefinitionContext>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = application_protocol_definition)]
    #[holder(generate_deserialize)]
    pub struct ApplicationProtocolDefinition {
        pub status: Label,
        pub application_interpreted_model_schema_name: Label,
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
        # [holder (field = draughting_approval_assignment)]
        DraughtingApprovalAssignment(Box<DraughtingApprovalAssignment>),
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
        pub role: Label,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = approval_status)]
    #[holder(generate_deserialize)]
    pub struct ApprovalStatus {
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
        # [holder (field = drawing_sheet_revision_usage)]
        DrawingSheetRevisionUsage(Box<DrawingSheetRevisionUsage>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = axis2_placement_2d)]
    #[holder(generate_deserialize)]
    pub struct Axis2Placement2D {
        #[holder(use_place_holder)]
        pub ref_direction: Option<Direction>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = b_spline_curve)]
    #[holder(generate_deserialize)]
    pub struct BSplineCurve {
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
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = b_spline_curve_with_knots)]
    #[holder(generate_deserialize)]
    pub struct BSplineCurveWithKnots {
        pub knot_multiplicities: Vec<i64>,
        pub knots: Vec<ParameterValue>,
        pub knot_spec: KnotType,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = bezier_curve)]
    #[holder(generate_deserialize)]
    pub struct BezierCurve {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = bounded_curve)]
    #[holder(generate_deserialize)]
    pub struct BoundedCurve {}
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum BoundedCurveAny {
        #[holder(use_place_holder)]
        # [holder (field = b_spline_curve)]
        BSplineCurve(Box<BSplineCurve>),
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
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = calendar_date)]
    #[holder(generate_deserialize)]
    pub struct CalendarDate {
        pub day_component: DayInMonthNumber,
        pub month_component: MonthInYearNumber,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = camera_image)]
    #[holder(generate_deserialize)]
    pub struct CameraImage {}
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum CameraImageAny {
        #[holder(use_place_holder)]
        # [holder (field = camera_image_2d_with_scale)]
        CameraImage2DWithScale(Box<CameraImage2DWithScale>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = camera_image_2d_with_scale)]
    #[holder(generate_deserialize)]
    pub struct CameraImage2DWithScale {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = camera_model)]
    #[holder(generate_deserialize)]
    pub struct CameraModel {}
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum CameraModelAny {
        #[holder(use_place_holder)]
        # [holder (field = camera_model_d2)]
        CameraModelD2(Box<CameraModelD2>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = camera_model_d2)]
    #[holder(generate_deserialize)]
    pub struct CameraModelD2 {
        #[holder(use_place_holder)]
        pub view_window: PlanarBox,
        pub view_window_clipping: bool,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = camera_usage)]
    #[holder(generate_deserialize)]
    pub struct CameraUsage {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = cartesian_point)]
    #[holder(generate_deserialize)]
    pub struct CartesianPoint {
        pub coordinates: Vec<LengthMeasure>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = circle)]
    #[holder(generate_deserialize)]
    pub struct Circle {
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
        # [holder (field = colour_specification)]
        ColourSpecification(Box<ColourSpecification>),
        #[holder(use_place_holder)]
        # [holder (field = pre_defined_colour)]
        PreDefinedColour(Box<PreDefinedColour>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = colour_rgb)]
    #[holder(generate_deserialize)]
    pub struct ColourRgb {
        pub red: f64,
        pub green: f64,
        pub blue: f64,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = colour_specification)]
    #[holder(generate_deserialize)]
    pub struct ColourSpecification {
        #[holder(use_place_holder)]
        pub name: ColourAny,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ColourSpecificationAny {
        #[holder(use_place_holder)]
        # [holder (field = colour_rgb)]
        ColourRgb(Box<ColourRgb>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = composite_curve)]
    #[holder(generate_deserialize)]
    pub struct CompositeCurve {
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
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = composite_text)]
    #[holder(generate_deserialize)]
    pub struct CompositeText {
        #[holder(use_place_holder)]
        pub collected_text: Vec<TextOrCharacter>,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum CompositeTextAny {
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
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = composite_text_with_associated_curves)]
    #[holder(generate_deserialize)]
    pub struct CompositeTextWithAssociatedCurves {
        #[holder(use_place_holder)]
        pub associated_curves: Vec<CurveAny>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = composite_text_with_blanking_box)]
    #[holder(generate_deserialize)]
    pub struct CompositeTextWithBlankingBox {
        #[holder(use_place_holder)]
        pub blanking: PlanarBox,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = composite_text_with_extent)]
    #[holder(generate_deserialize)]
    pub struct CompositeTextWithExtent {
        #[holder(use_place_holder)]
        pub extent: PlanarExtentAny,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = conic)]
    #[holder(generate_deserialize)]
    pub struct Conic {
        #[holder(use_place_holder)]
        pub position: Axis2Placement,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ConicAny {
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
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = context_dependent_invisibility)]
    #[holder(generate_deserialize)]
    pub struct ContextDependentInvisibility {
        #[holder(use_place_holder)]
        pub presentation_context: InvisibilityContext,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = contract)]
    #[holder(generate_deserialize)]
    pub struct Contract {
        pub name: Label,
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
        # [holder (field = draughting_contract_assignment)]
        DraughtingContractAssignment(Box<DraughtingContractAssignment>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = contract_type)]
    #[holder(generate_deserialize)]
    pub struct ContractType {
        pub description: Label,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = conversion_based_unit)]
    #[holder(generate_deserialize)]
    pub struct ConversionBasedUnit {
        pub name: Label,
        #[holder(use_place_holder)]
        pub conversion_factor: MeasureWithUnitAny,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = curve)]
    #[holder(generate_deserialize)]
    pub struct Curve {}
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum CurveAny {
        #[holder(use_place_holder)]
        # [holder (field = bounded_curve)]
        BoundedCurve(Box<BoundedCurve>),
        #[holder(use_place_holder)]
        # [holder (field = conic)]
        Conic(Box<Conic>),
        #[holder(use_place_holder)]
        # [holder (field = line)]
        Line(Box<Line>),
        #[holder(use_place_holder)]
        # [holder (field = offset_curve_2d)]
        OffsetCurve2D(Box<OffsetCurve2D>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = curve_dimension)]
    #[holder(generate_deserialize)]
    pub struct CurveDimension {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = curve_style)]
    #[holder(generate_deserialize)]
    pub struct CurveStyle {
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
        pub name: Label,
        #[holder(use_place_holder)]
        pub pattern_list: Vec<CurveStyleFontPattern>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = curve_style_font_pattern)]
    #[holder(generate_deserialize)]
    pub struct CurveStyleFontPattern {
        pub visible_segment_length: PositiveLengthMeasure,
        pub invisible_segment_length: PositiveLengthMeasure,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = date)]
    #[holder(generate_deserialize)]
    pub struct Date {
        pub year_component: YearNumber,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum DateAny {
        #[holder(use_place_holder)]
        # [holder (field = calendar_date)]
        CalendarDate(Box<CalendarDate>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = datum_feature_callout)]
    #[holder(generate_deserialize)]
    pub struct DatumFeatureCallout {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = datum_target_callout)]
    #[holder(generate_deserialize)]
    pub struct DatumTargetCallout {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = defined_symbol)]
    #[holder(generate_deserialize)]
    pub struct DefinedSymbol {
        #[holder(use_place_holder)]
        pub definition: DefinedSymbolSelect,
        #[holder(use_place_holder)]
        pub target: SymbolTarget,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = diameter_dimension)]
    #[holder(generate_deserialize)]
    pub struct DiameterDimension {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = dimension_callout_component_relationship)]
    #[holder(generate_deserialize)]
    pub struct DimensionCalloutComponentRelationship {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = dimension_callout_relationship)]
    #[holder(generate_deserialize)]
    pub struct DimensionCalloutRelationship {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = dimension_curve)]
    #[holder(generate_deserialize)]
    pub struct DimensionCurve {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = dimension_curve_directed_callout)]
    #[holder(generate_deserialize)]
    pub struct DimensionCurveDirectedCallout {}
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum DimensionCurveDirectedCalloutAny {
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
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = dimension_curve_terminator)]
    #[holder(generate_deserialize)]
    pub struct DimensionCurveTerminator {
        pub role: DimensionExtentUsage,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = dimension_pair)]
    #[holder(generate_deserialize)]
    pub struct DimensionPair {}
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
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = direction)]
    #[holder(generate_deserialize)]
    pub struct Direction {
        pub direction_ratios: Vec<f64>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = document)]
    #[holder(generate_deserialize)]
    pub struct Document {
        pub id: Identifier,
        pub name: Label,
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
        pub source: Label,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum DocumentReferenceAny {
        #[holder(use_place_holder)]
        # [holder (field = draughting_specification_reference)]
        DraughtingSpecificationReference(Box<DraughtingSpecificationReference>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = document_type)]
    #[holder(generate_deserialize)]
    pub struct DocumentType {
        pub product_data_type: Label,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = draughting_annotation_occurrence)]
    #[holder(generate_deserialize)]
    pub struct DraughtingAnnotationOccurrence {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = draughting_approval_assignment)]
    #[holder(generate_deserialize)]
    pub struct DraughtingApprovalAssignment {
        #[holder(use_place_holder)]
        pub approved_items: Vec<ApprovedItem>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = draughting_callout)]
    #[holder(generate_deserialize)]
    pub struct DraughtingCallout {
        #[holder(use_place_holder)]
        pub contents: Vec<DraughtingCalloutElement>,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum DraughtingCalloutAny {
        #[holder(use_place_holder)]
        # [holder (field = datum_feature_callout)]
        DatumFeatureCallout(Box<DatumFeatureCallout>),
        #[holder(use_place_holder)]
        # [holder (field = datum_target_callout)]
        DatumTargetCallout(Box<DatumTargetCallout>),
        #[holder(use_place_holder)]
        # [holder (field = dimension_curve_directed_callout)]
        DimensionCurveDirectedCallout(Box<DimensionCurveDirectedCallout>),
        #[holder(use_place_holder)]
        # [holder (field = draughting_elements)]
        DraughtingElements(Box<DraughtingElements>),
        #[holder(use_place_holder)]
        # [holder (field = geometrical_tolerance_callout)]
        GeometricalToleranceCallout(Box<GeometricalToleranceCallout>),
        #[holder(use_place_holder)]
        # [holder (field = leader_directed_callout)]
        LeaderDirectedCallout(Box<LeaderDirectedCallout>),
        #[holder(use_place_holder)]
        # [holder (field = projection_directed_callout)]
        ProjectionDirectedCallout(Box<ProjectionDirectedCallout>),
        #[holder(use_place_holder)]
        # [holder (field = structured_dimension_callout)]
        StructuredDimensionCallout(Box<StructuredDimensionCallout>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = draughting_callout_relationship)]
    #[holder(generate_deserialize)]
    pub struct DraughtingCalloutRelationship {
        pub name: Label,
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
        # [holder (field = dimension_callout_component_relationship)]
        DimensionCalloutComponentRelationship(Box<DimensionCalloutComponentRelationship>),
        #[holder(use_place_holder)]
        # [holder (field = dimension_callout_relationship)]
        DimensionCalloutRelationship(Box<DimensionCalloutRelationship>),
        #[holder(use_place_holder)]
        # [holder (field = dimension_pair)]
        DimensionPair(Box<DimensionPair>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = draughting_contract_assignment)]
    #[holder(generate_deserialize)]
    pub struct DraughtingContractAssignment {
        #[holder(use_place_holder)]
        pub items: Vec<ContractedItem>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = draughting_drawing_revision)]
    #[holder(generate_deserialize)]
    pub struct DraughtingDrawingRevision {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = draughting_elements)]
    #[holder(generate_deserialize)]
    pub struct DraughtingElements {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = draughting_group_assignment)]
    #[holder(generate_deserialize)]
    pub struct DraughtingGroupAssignment {
        #[holder(use_place_holder)]
        pub items: Vec<DraughtingGroupedItem>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = draughting_model)]
    #[holder(generate_deserialize)]
    pub struct DraughtingModel {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = draughting_organization_assignment)]
    #[holder(generate_deserialize)]
    pub struct DraughtingOrganizationAssignment {
        #[holder(use_place_holder)]
        pub assigned_items: Vec<DraughtingOrganizationItem>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = draughting_person_and_organization_assignment)]
    #[holder(generate_deserialize)]
    pub struct DraughtingPersonAndOrganizationAssignment {
        #[holder(use_place_holder)]
        pub assigned_items: Vec<DraughtingOrganizationItem>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = draughting_person_assignment)]
    #[holder(generate_deserialize)]
    pub struct DraughtingPersonAssignment {
        #[holder(use_place_holder)]
        pub assigned_items: Vec<DraughtingOrganizationItem>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = draughting_pre_defined_colour)]
    #[holder(generate_deserialize)]
    pub struct DraughtingPreDefinedColour {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = draughting_pre_defined_curve_font)]
    #[holder(generate_deserialize)]
    pub struct DraughtingPreDefinedCurveFont {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = draughting_pre_defined_text_font)]
    #[holder(generate_deserialize)]
    pub struct DraughtingPreDefinedTextFont {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = draughting_presented_item)]
    #[holder(generate_deserialize)]
    pub struct DraughtingPresentedItem {
        #[holder(use_place_holder)]
        pub items: Vec<DraughtingPresentedItemSelect>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = draughting_security_classification_assignment)]
    #[holder(generate_deserialize)]
    pub struct DraughtingSecurityClassificationAssignment {
        #[holder(use_place_holder)]
        pub assigned_items: Vec<ClassifiedItem>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = draughting_specification_reference)]
    #[holder(generate_deserialize)]
    pub struct DraughtingSpecificationReference {
        #[holder(use_place_holder)]
        pub specified_items: Vec<SpecifiedItem>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = draughting_subfigure_representation)]
    #[holder(generate_deserialize)]
    pub struct DraughtingSubfigureRepresentation {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = draughting_symbol_representation)]
    #[holder(generate_deserialize)]
    pub struct DraughtingSymbolRepresentation {}
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum DraughtingSymbolRepresentationAny {
        #[holder(use_place_holder)]
        # [holder (field = drawing_sheet_layout)]
        DrawingSheetLayout(Box<DrawingSheetLayout>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = draughting_text_literal_with_delineation)]
    #[holder(generate_deserialize)]
    pub struct DraughtingTextLiteralWithDelineation {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = draughting_title)]
    #[holder(generate_deserialize)]
    pub struct DraughtingTitle {
        #[holder(use_place_holder)]
        pub items: Vec<DraughtingTitledItem>,
        pub language: Label,
        pub contents: Text,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = drawing_definition)]
    #[holder(generate_deserialize)]
    pub struct DrawingDefinition {
        pub drawing_number: Identifier,
        pub drawing_type: Option<Label>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = drawing_revision)]
    #[holder(generate_deserialize)]
    pub struct DrawingRevision {
        pub revision_identifier: Identifier,
        #[holder(use_place_holder)]
        pub drawing_identifier: DrawingDefinition,
        pub intended_scale: Option<Text>,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum DrawingRevisionAny {
        #[holder(use_place_holder)]
        # [holder (field = draughting_drawing_revision)]
        DraughtingDrawingRevision(Box<DraughtingDrawingRevision>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = drawing_sheet_layout)]
    #[holder(generate_deserialize)]
    pub struct DrawingSheetLayout {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = drawing_sheet_revision)]
    #[holder(generate_deserialize)]
    pub struct DrawingSheetRevision {
        pub revision_identifier: Identifier,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = drawing_sheet_revision_usage)]
    #[holder(generate_deserialize)]
    pub struct DrawingSheetRevisionUsage {
        pub sheet_number: Identifier,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = ellipse)]
    #[holder(generate_deserialize)]
    pub struct Ellipse {
        pub semi_axis_1: PositiveLengthMeasure,
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
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = externally_defined_curve_font)]
    #[holder(generate_deserialize)]
    pub struct ExternallyDefinedCurveFont {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = externally_defined_hatch_style)]
    #[holder(generate_deserialize)]
    pub struct ExternallyDefinedHatchStyle {}
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
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = externally_defined_symbol)]
    #[holder(generate_deserialize)]
    pub struct ExternallyDefinedSymbol {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = externally_defined_text_font)]
    #[holder(generate_deserialize)]
    pub struct ExternallyDefinedTextFont {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = externally_defined_tile_style)]
    #[holder(generate_deserialize)]
    pub struct ExternallyDefinedTileStyle {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = fill_area_style)]
    #[holder(generate_deserialize)]
    pub struct FillAreaStyle {
        pub name: Label,
        #[holder(use_place_holder)]
        pub fill_styles: Vec<FillStyleSelect>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = fill_area_style_colour)]
    #[holder(generate_deserialize)]
    pub struct FillAreaStyleColour {
        pub name: Label,
        #[holder(use_place_holder)]
        pub fill_colour: ColourAny,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = fill_area_style_hatching)]
    #[holder(generate_deserialize)]
    pub struct FillAreaStyleHatching {
        #[holder(use_place_holder)]
        pub hatch_line_appearance: CurveStyle,
        #[holder(use_place_holder)]
        pub start_of_next_hatch_line: OneDirectionRepeatFactorAny,
        #[holder(use_place_holder)]
        pub point_of_reference_hatch_line: CartesianPoint,
        #[holder(use_place_holder)]
        pub pattern_start: CartesianPoint,
        pub hatch_line_angle: PlaneAngleMeasure,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = fill_area_style_tile_symbol_with_style)]
    #[holder(generate_deserialize)]
    pub struct FillAreaStyleTileSymbolWithStyle {
        #[holder(use_place_holder)]
        pub symbol: AnnotationSymbolOccurrenceAny,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = fill_area_style_tiles)]
    #[holder(generate_deserialize)]
    pub struct FillAreaStyleTiles {
        #[holder(use_place_holder)]
        pub tiling_pattern: TwoDirectionRepeatFactor,
        #[holder(use_place_holder)]
        pub tiles: Vec<FillAreaStyleTileShapeSelect>,
        pub tiling_scale: PositiveRatioMeasure,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = geometric_curve_set)]
    #[holder(generate_deserialize)]
    pub struct GeometricCurveSet {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = geometric_representation_context)]
    #[holder(generate_deserialize)]
    pub struct GeometricRepresentationContext {
        pub coordinate_space_dimension: DimensionCount,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = geometric_representation_item)]
    #[holder(generate_deserialize)]
    pub struct GeometricRepresentationItem {}
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum GeometricRepresentationItemAny {
        #[holder(use_place_holder)]
        # [holder (field = annotation_fill_area)]
        AnnotationFillArea(Box<AnnotationFillArea>),
        #[holder(use_place_holder)]
        # [holder (field = camera_model)]
        CameraModel(Box<CameraModel>),
        #[holder(use_place_holder)]
        # [holder (field = composite_text)]
        CompositeText(Box<CompositeText>),
        #[holder(use_place_holder)]
        # [holder (field = curve)]
        Curve(Box<Curve>),
        #[holder(use_place_holder)]
        # [holder (field = defined_symbol)]
        DefinedSymbol(Box<DefinedSymbol>),
        #[holder(use_place_holder)]
        # [holder (field = direction)]
        Direction(Box<Direction>),
        #[holder(use_place_holder)]
        # [holder (field = draughting_callout)]
        DraughtingCallout(Box<DraughtingCallout>),
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
        GeometricSet(Box<GeometricSet>),
        #[holder(use_place_holder)]
        # [holder (field = one_direction_repeat_factor)]
        OneDirectionRepeatFactor(Box<OneDirectionRepeatFactor>),
        #[holder(use_place_holder)]
        # [holder (field = placement)]
        Placement(Box<Placement>),
        #[holder(use_place_holder)]
        # [holder (field = planar_extent)]
        PlanarExtent(Box<PlanarExtent>),
        #[holder(use_place_holder)]
        # [holder (field = point)]
        Point(Box<Point>),
        #[holder(use_place_holder)]
        # [holder (field = symbol_target)]
        SymbolTarget(Box<SymbolTarget>),
        #[holder(use_place_holder)]
        # [holder (field = text_literal)]
        TextLiteral(Box<TextLiteral>),
        #[holder(use_place_holder)]
        # [holder (field = vector)]
        Vector(Box<Vector>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = geometric_set)]
    #[holder(generate_deserialize)]
    pub struct GeometricSet {
        #[holder(use_place_holder)]
        pub elements: Vec<GeometricSetSelect>,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum GeometricSetAny {
        #[holder(use_place_holder)]
        # [holder (field = geometric_curve_set)]
        GeometricCurveSet(Box<GeometricCurveSet>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = geometrical_tolerance_callout)]
    #[holder(generate_deserialize)]
    pub struct GeometricalToleranceCallout {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = geometrically_bounded_2d_wireframe_representation)]
    #[holder(generate_deserialize)]
    pub struct GeometricallyBounded2DWireframeRepresentation {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = global_unit_assigned_context)]
    #[holder(generate_deserialize)]
    pub struct GlobalUnitAssignedContext {
        #[holder(use_place_holder)]
        pub units: Vec<Unit>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = group)]
    #[holder(generate_deserialize)]
    pub struct Group {
        pub name: Label,
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
        # [holder (field = draughting_group_assignment)]
        DraughtingGroupAssignment(Box<DraughtingGroupAssignment>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = group_relationship)]
    #[holder(generate_deserialize)]
    pub struct GroupRelationship {
        pub name: Label,
        pub description: Text,
        #[holder(use_place_holder)]
        pub relating_group: Group,
        #[holder(use_place_holder)]
        pub related_group: Group,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = hyperbola)]
    #[holder(generate_deserialize)]
    pub struct Hyperbola {
        pub semi_axis: PositiveLengthMeasure,
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
        # [holder (field = context_dependent_invisibility)]
        ContextDependentInvisibility(Box<ContextDependentInvisibility>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = leader_curve)]
    #[holder(generate_deserialize)]
    pub struct LeaderCurve {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = leader_directed_callout)]
    #[holder(generate_deserialize)]
    pub struct LeaderDirectedCallout {}
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum LeaderDirectedCalloutAny {
        #[holder(use_place_holder)]
        # [holder (field = leader_directed_dimension)]
        LeaderDirectedDimension(Box<LeaderDirectedDimension>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = leader_directed_dimension)]
    #[holder(generate_deserialize)]
    pub struct LeaderDirectedDimension {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = leader_terminator)]
    #[holder(generate_deserialize)]
    pub struct LeaderTerminator {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = length_measure_with_unit)]
    #[holder(generate_deserialize)]
    pub struct LengthMeasureWithUnit {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = length_unit)]
    #[holder(generate_deserialize)]
    pub struct LengthUnit {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = line)]
    #[holder(generate_deserialize)]
    pub struct Line {
        #[holder(use_place_holder)]
        pub pnt: CartesianPoint,
        #[holder(use_place_holder)]
        pub dir: Vector,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = linear_dimension)]
    #[holder(generate_deserialize)]
    pub struct LinearDimension {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = mapped_item)]
    #[holder(generate_deserialize)]
    pub struct MappedItem {
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
        # [holder (field = annotation_symbol)]
        AnnotationSymbol(Box<AnnotationSymbol>),
        #[holder(use_place_holder)]
        # [holder (field = annotation_text)]
        AnnotationText(Box<AnnotationText>),
        #[holder(use_place_holder)]
        # [holder (field = camera_image)]
        CameraImage(Box<CameraImage>),
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
        # [holder (field = length_measure_with_unit)]
        LengthMeasureWithUnit(Box<LengthMeasureWithUnit>),
        #[holder(use_place_holder)]
        # [holder (field = plane_angle_measure_with_unit)]
        PlaneAngleMeasureWithUnit(Box<PlaneAngleMeasureWithUnit>),
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
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = offset_curve_2d)]
    #[holder(generate_deserialize)]
    pub struct OffsetCurve2D {
        #[holder(use_place_holder)]
        pub basis_curve: CurveAny,
        pub distance: LengthMeasure,
        pub self_intersect: Logical,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = one_direction_repeat_factor)]
    #[holder(generate_deserialize)]
    pub struct OneDirectionRepeatFactor {
        #[holder(use_place_holder)]
        pub repeat_factor: Vector,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum OneDirectionRepeatFactorAny {
        #[holder(use_place_holder)]
        # [holder (field = two_direction_repeat_factor)]
        TwoDirectionRepeatFactor(Box<TwoDirectionRepeatFactor>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = ordinate_dimension)]
    #[holder(generate_deserialize)]
    pub struct OrdinateDimension {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = organization)]
    #[holder(generate_deserialize)]
    pub struct Organization {
        pub id: Option<Identifier>,
        pub name: Label,
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
        # [holder (field = draughting_organization_assignment)]
        DraughtingOrganizationAssignment(Box<DraughtingOrganizationAssignment>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = organization_role)]
    #[holder(generate_deserialize)]
    pub struct OrganizationRole {
        pub name: Label,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = organizational_address)]
    #[holder(generate_deserialize)]
    pub struct OrganizationalAddress {
        #[holder(use_place_holder)]
        pub organizations: Vec<Organization>,
        pub description: Text,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = parabola)]
    #[holder(generate_deserialize)]
    pub struct Parabola {
        pub focal_dist: LengthMeasure,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = person)]
    #[holder(generate_deserialize)]
    pub struct Person {
        pub id: Identifier,
        pub last_name: Option<Label>,
        pub first_name: Option<Label>,
        pub middle_names: Option<Vec<Label>>,
        pub prefix_titles: Option<Vec<Label>>,
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
        # [holder (field = draughting_person_and_organization_assignment)]
        DraughtingPersonAndOrganizationAssignment(Box<DraughtingPersonAndOrganizationAssignment>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = person_and_organization_role)]
    #[holder(generate_deserialize)]
    pub struct PersonAndOrganizationRole {
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
        # [holder (field = draughting_person_assignment)]
        DraughtingPersonAssignment(Box<DraughtingPersonAssignment>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = person_role)]
    #[holder(generate_deserialize)]
    pub struct PersonRole {
        pub name: Label,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = personal_address)]
    #[holder(generate_deserialize)]
    pub struct PersonalAddress {
        #[holder(use_place_holder)]
        pub people: Vec<Person>,
        pub description: Text,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = placement)]
    #[holder(generate_deserialize)]
    pub struct Placement {
        #[holder(use_place_holder)]
        pub location: CartesianPoint,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum PlacementAny {
        #[holder(use_place_holder)]
        # [holder (field = axis2_placement_2d)]
        Axis2Placement2D(Box<Axis2Placement2D>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = planar_box)]
    #[holder(generate_deserialize)]
    pub struct PlanarBox {
        #[holder(use_place_holder)]
        pub placement: Axis2Placement,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = planar_extent)]
    #[holder(generate_deserialize)]
    pub struct PlanarExtent {
        pub size_in_x: LengthMeasure,
        pub size_in_y: LengthMeasure,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum PlanarExtentAny {
        #[holder(use_place_holder)]
        # [holder (field = planar_box)]
        PlanarBox(Box<PlanarBox>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = plane_angle_measure_with_unit)]
    #[holder(generate_deserialize)]
    pub struct PlaneAngleMeasureWithUnit {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = plane_angle_unit)]
    #[holder(generate_deserialize)]
    pub struct PlaneAngleUnit {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = point)]
    #[holder(generate_deserialize)]
    pub struct Point {}
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum PointAny {
        #[holder(use_place_holder)]
        # [holder (field = cartesian_point)]
        CartesianPoint(Box<CartesianPoint>),
        #[holder(use_place_holder)]
        # [holder (field = point_on_curve)]
        PointOnCurve(Box<PointOnCurve>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = point_on_curve)]
    #[holder(generate_deserialize)]
    pub struct PointOnCurve {
        #[holder(use_place_holder)]
        pub basis_curve: CurveAny,
        pub point_parameter: ParameterValue,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = polyline)]
    #[holder(generate_deserialize)]
    pub struct Polyline {
        #[holder(use_place_holder)]
        pub points: Vec<CartesianPoint>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = pre_defined_colour)]
    #[holder(generate_deserialize)]
    pub struct PreDefinedColour {}
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum PreDefinedColourAny {
        #[holder(use_place_holder)]
        # [holder (field = draughting_pre_defined_colour)]
        DraughtingPreDefinedColour(Box<DraughtingPreDefinedColour>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = pre_defined_curve_font)]
    #[holder(generate_deserialize)]
    pub struct PreDefinedCurveFont {}
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum PreDefinedCurveFontAny {
        #[holder(use_place_holder)]
        # [holder (field = draughting_pre_defined_curve_font)]
        DraughtingPreDefinedCurveFont(Box<DraughtingPreDefinedCurveFont>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = pre_defined_dimension_symbol)]
    #[holder(generate_deserialize)]
    pub struct PreDefinedDimensionSymbol {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = pre_defined_geometrical_tolerance_symbol)]
    #[holder(generate_deserialize)]
    pub struct PreDefinedGeometricalToleranceSymbol {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = pre_defined_item)]
    #[holder(generate_deserialize)]
    pub struct PreDefinedItem {
        pub name: Label,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum PreDefinedItemAny {
        #[holder(use_place_holder)]
        # [holder (field = pre_defined_colour)]
        PreDefinedColour(Box<PreDefinedColour>),
        #[holder(use_place_holder)]
        # [holder (field = pre_defined_curve_font)]
        PreDefinedCurveFont(Box<PreDefinedCurveFont>),
        #[holder(use_place_holder)]
        # [holder (field = pre_defined_symbol)]
        PreDefinedSymbol(Box<PreDefinedSymbol>),
        #[holder(use_place_holder)]
        # [holder (field = pre_defined_text_font)]
        PreDefinedTextFont(Box<PreDefinedTextFont>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = pre_defined_point_marker_symbol)]
    #[holder(generate_deserialize)]
    pub struct PreDefinedPointMarkerSymbol {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = pre_defined_symbol)]
    #[holder(generate_deserialize)]
    pub struct PreDefinedSymbol {}
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum PreDefinedSymbolAny {
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
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = pre_defined_terminator_symbol)]
    #[holder(generate_deserialize)]
    pub struct PreDefinedTerminatorSymbol {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = pre_defined_text_font)]
    #[holder(generate_deserialize)]
    pub struct PreDefinedTextFont {}
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum PreDefinedTextFontAny {
        #[holder(use_place_holder)]
        # [holder (field = draughting_pre_defined_text_font)]
        DraughtingPreDefinedTextFont(Box<DraughtingPreDefinedTextFont>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = presentation_area)]
    #[holder(generate_deserialize)]
    pub struct PresentationArea {}
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum PresentationAreaAny {
        #[holder(use_place_holder)]
        # [holder (field = drawing_sheet_revision)]
        DrawingSheetRevision(Box<DrawingSheetRevision>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = presentation_layer_assignment)]
    #[holder(generate_deserialize)]
    pub struct PresentationLayerAssignment {
        pub name: Label,
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
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = presentation_representation)]
    #[holder(generate_deserialize)]
    pub struct PresentationRepresentation {}
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum PresentationRepresentationAny {
        #[holder(use_place_holder)]
        # [holder (field = presentation_area)]
        PresentationArea(Box<PresentationArea>),
        #[holder(use_place_holder)]
        # [holder (field = presentation_view)]
        PresentationView(Box<PresentationView>),
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
        # [holder (field = drawing_revision)]
        DrawingRevision(Box<DrawingRevision>),
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
        # [holder (field = presentation_style_by_context)]
        PresentationStyleByContext(Box<PresentationStyleByContext>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = presentation_style_by_context)]
    #[holder(generate_deserialize)]
    pub struct PresentationStyleByContext {
        #[holder(use_place_holder)]
        pub style_context: StyleContextSelect,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = presentation_view)]
    #[holder(generate_deserialize)]
    pub struct PresentationView {}
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
        # [holder (field = draughting_presented_item)]
        DraughtingPresentedItem(Box<DraughtingPresentedItem>),
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
        pub id: Identifier,
        pub name: Label,
        pub description: Text,
        #[holder(use_place_holder)]
        pub frame_of_reference: Vec<ProductContext>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = product_context)]
    #[holder(generate_deserialize)]
    pub struct ProductContext {
        pub discipline_type: Label,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = product_definition)]
    #[holder(generate_deserialize)]
    pub struct ProductDefinition {
        pub id: Identifier,
        pub description: Text,
        #[holder(use_place_holder)]
        pub formation: ProductDefinitionFormation,
        #[holder(use_place_holder)]
        pub frame_of_reference: ProductDefinitionContext,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = product_definition_context)]
    #[holder(generate_deserialize)]
    pub struct ProductDefinitionContext {
        pub life_cycle_stage: Label,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = product_definition_formation)]
    #[holder(generate_deserialize)]
    pub struct ProductDefinitionFormation {
        pub id: Identifier,
        pub description: Text,
        #[holder(use_place_holder)]
        pub of_product: Product,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = product_definition_shape)]
    #[holder(generate_deserialize)]
    pub struct ProductDefinitionShape {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = projection_curve)]
    #[holder(generate_deserialize)]
    pub struct ProjectionCurve {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = projection_directed_callout)]
    #[holder(generate_deserialize)]
    pub struct ProjectionDirectedCallout {}
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ProjectionDirectedCalloutAny {
        #[holder(use_place_holder)]
        # [holder (field = ordinate_dimension)]
        OrdinateDimension(Box<OrdinateDimension>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = property_definition)]
    #[holder(generate_deserialize)]
    pub struct PropertyDefinition {
        pub name: Label,
        pub description: Text,
        #[holder(use_place_holder)]
        pub definition: CharacterizedDefinition,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum PropertyDefinitionAny {
        #[holder(use_place_holder)]
        # [holder (field = product_definition_shape)]
        ProductDefinitionShape(Box<ProductDefinitionShape>),
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
        # [holder (field = shape_definition_representation)]
        ShapeDefinitionRepresentation(Box<ShapeDefinitionRepresentation>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = quasi_uniform_curve)]
    #[holder(generate_deserialize)]
    pub struct QuasiUniformCurve {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = radius_dimension)]
    #[holder(generate_deserialize)]
    pub struct RadiusDimension {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = rational_b_spline_curve)]
    #[holder(generate_deserialize)]
    pub struct RationalBSplineCurve {
        pub weights_data: Vec<f64>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = representation)]
    #[holder(generate_deserialize)]
    pub struct Representation {
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
        # [holder (field = draughting_model)]
        DraughtingModel(Box<DraughtingModel>),
        #[holder(use_place_holder)]
        # [holder (field = presentation_representation)]
        PresentationRepresentation(Box<PresentationRepresentation>),
        #[holder(use_place_holder)]
        # [holder (field = shape_representation)]
        ShapeRepresentation(Box<ShapeRepresentation>),
        #[holder(use_place_holder)]
        # [holder (field = symbol_representation)]
        SymbolRepresentation(Box<SymbolRepresentation>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = representation_context)]
    #[holder(generate_deserialize)]
    pub struct RepresentationContext {
        pub context_identifier: Identifier,
        pub context_type: Text,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum RepresentationContextAny {
        #[holder(use_place_holder)]
        # [holder (field = geometric_representation_context)]
        GeometricRepresentationContext(Box<GeometricRepresentationContext>),
        #[holder(use_place_holder)]
        # [holder (field = global_unit_assigned_context)]
        GlobalUnitAssignedContext(Box<GlobalUnitAssignedContext>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = representation_item)]
    #[holder(generate_deserialize)]
    pub struct RepresentationItem {
        pub name: Label,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum RepresentationItemAny {
        #[holder(use_place_holder)]
        # [holder (field = geometric_representation_item)]
        GeometricRepresentationItem(Box<GeometricRepresentationItem>),
        #[holder(use_place_holder)]
        # [holder (field = mapped_item)]
        MappedItem(Box<MappedItem>),
        #[holder(use_place_holder)]
        # [holder (field = styled_item)]
        StyledItem(Box<StyledItem>),
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
        # [holder (field = camera_usage)]
        CameraUsage(Box<CameraUsage>),
        #[holder(use_place_holder)]
        # [holder (field = symbol_representation_map)]
        SymbolRepresentationMap(Box<SymbolRepresentationMap>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = security_classification)]
    #[holder(generate_deserialize)]
    pub struct SecurityClassification {
        pub name: Label,
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
        # [holder (field = draughting_security_classification_assignment)]
        DraughtingSecurityClassificationAssignment(Box<DraughtingSecurityClassificationAssignment>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = security_classification_level)]
    #[holder(generate_deserialize)]
    pub struct SecurityClassificationLevel {
        pub name: Label,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = shape_definition_representation)]
    #[holder(generate_deserialize)]
    pub struct ShapeDefinitionRepresentation {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = shape_representation)]
    #[holder(generate_deserialize)]
    pub struct ShapeRepresentation {}
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ShapeRepresentationAny {
        #[holder(use_place_holder)]
        # [holder (field = geometrically_bounded_2d_wireframe_representation)]
        GeometricallyBounded2DWireframeRepresentation(
            Box<GeometricallyBounded2DWireframeRepresentation>,
        ),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = si_unit)]
    #[holder(generate_deserialize)]
    pub struct SiUnit {
        pub prefix: Option<SiPrefix>,
        pub name: SiUnitName,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = structured_dimension_callout)]
    #[holder(generate_deserialize)]
    pub struct StructuredDimensionCallout {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = styled_item)]
    #[holder(generate_deserialize)]
    pub struct StyledItem {
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
        # [holder (field = annotation_occurrence)]
        AnnotationOccurrence(Box<AnnotationOccurrence>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = symbol_colour)]
    #[holder(generate_deserialize)]
    pub struct SymbolColour {
        #[holder(use_place_holder)]
        pub colour_of_symbol: ColourAny,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = symbol_representation)]
    #[holder(generate_deserialize)]
    pub struct SymbolRepresentation {}
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum SymbolRepresentationAny {
        #[holder(use_place_holder)]
        # [holder (field = draughting_subfigure_representation)]
        DraughtingSubfigureRepresentation(Box<DraughtingSubfigureRepresentation>),
        #[holder(use_place_holder)]
        # [holder (field = draughting_symbol_representation)]
        DraughtingSymbolRepresentation(Box<DraughtingSymbolRepresentation>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = symbol_representation_map)]
    #[holder(generate_deserialize)]
    pub struct SymbolRepresentationMap {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = symbol_style)]
    #[holder(generate_deserialize)]
    pub struct SymbolStyle {
        pub name: Label,
        #[holder(use_place_holder)]
        pub style_of_symbol: SymbolStyleSelect,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = symbol_target)]
    #[holder(generate_deserialize)]
    pub struct SymbolTarget {
        #[holder(use_place_holder)]
        pub placement: Axis2Placement,
        pub x_scale: PositiveRatioMeasure,
        pub y_scale: PositiveRatioMeasure,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = terminator_symbol)]
    #[holder(generate_deserialize)]
    pub struct TerminatorSymbol {
        #[holder(use_place_holder)]
        pub annotated_curve: AnnotationCurveOccurrenceAny,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum TerminatorSymbolAny {
        #[holder(use_place_holder)]
        # [holder (field = dimension_curve_terminator)]
        DimensionCurveTerminator(Box<DimensionCurveTerminator>),
        #[holder(use_place_holder)]
        # [holder (field = leader_terminator)]
        LeaderTerminator(Box<LeaderTerminator>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = text_literal)]
    #[holder(generate_deserialize)]
    pub struct TextLiteral {
        pub literal: PresentableText,
        #[holder(use_place_holder)]
        pub placement: Axis2Placement,
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
        # [holder (field = text_literal_with_associated_curves)]
        TextLiteralWithAssociatedCurves(Box<TextLiteralWithAssociatedCurves>),
        #[holder(use_place_holder)]
        # [holder (field = text_literal_with_blanking_box)]
        TextLiteralWithBlankingBox(Box<TextLiteralWithBlankingBox>),
        #[holder(use_place_holder)]
        # [holder (field = text_literal_with_delineation)]
        TextLiteralWithDelineation(Box<TextLiteralWithDelineation>),
        #[holder(use_place_holder)]
        # [holder (field = text_literal_with_extent)]
        TextLiteralWithExtent(Box<TextLiteralWithExtent>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = text_literal_with_associated_curves)]
    #[holder(generate_deserialize)]
    pub struct TextLiteralWithAssociatedCurves {
        #[holder(use_place_holder)]
        pub associated_curves: Vec<CurveAny>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = text_literal_with_blanking_box)]
    #[holder(generate_deserialize)]
    pub struct TextLiteralWithBlankingBox {
        #[holder(use_place_holder)]
        pub blanking: PlanarBox,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = text_literal_with_delineation)]
    #[holder(generate_deserialize)]
    pub struct TextLiteralWithDelineation {
        pub delineation: TextDelineation,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum TextLiteralWithDelineationAny {
        #[holder(use_place_holder)]
        # [holder (field = draughting_text_literal_with_delineation)]
        DraughtingTextLiteralWithDelineation(Box<DraughtingTextLiteralWithDelineation>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = text_literal_with_extent)]
    #[holder(generate_deserialize)]
    pub struct TextLiteralWithExtent {
        #[holder(use_place_holder)]
        pub extent: PlanarExtentAny,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = text_style)]
    #[holder(generate_deserialize)]
    pub struct TextStyle {
        pub name: Label,
        #[holder(use_place_holder)]
        pub character_appearance: CharacterStyleSelect,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum TextStyleAny {
        #[holder(use_place_holder)]
        # [holder (field = text_style_with_box_characteristics)]
        TextStyleWithBoxCharacteristics(Box<TextStyleWithBoxCharacteristics>),
        #[holder(use_place_holder)]
        # [holder (field = text_style_with_mirror)]
        TextStyleWithMirror(Box<TextStyleWithMirror>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = text_style_for_defined_font)]
    #[holder(generate_deserialize)]
    pub struct TextStyleForDefinedFont {
        #[holder(use_place_holder)]
        pub text_colour: ColourAny,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = text_style_with_box_characteristics)]
    #[holder(generate_deserialize)]
    pub struct TextStyleWithBoxCharacteristics {
        #[holder(use_place_holder)]
        pub characteristics: Vec<BoxCharacteristicSelect>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = text_style_with_mirror)]
    #[holder(generate_deserialize)]
    pub struct TextStyleWithMirror {
        #[holder(use_place_holder)]
        pub mirror_placement: Axis2Placement,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = trimmed_curve)]
    #[holder(generate_deserialize)]
    pub struct TrimmedCurve {
        #[holder(use_place_holder)]
        pub basis_curve: CurveAny,
        #[holder(use_place_holder)]
        pub trim_1: Vec<TrimmingSelect>,
        #[holder(use_place_holder)]
        pub trim_2: Vec<TrimmingSelect>,
        pub sense_agreement: bool,
        pub master_representation: TrimmingPreference,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = two_direction_repeat_factor)]
    #[holder(generate_deserialize)]
    pub struct TwoDirectionRepeatFactor {
        #[holder(use_place_holder)]
        pub second_repeat_factor: Vector,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = uniform_curve)]
    #[holder(generate_deserialize)]
    pub struct UniformCurve {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = vector)]
    #[holder(generate_deserialize)]
    pub struct Vector {
        #[holder(use_place_holder)]
        pub orientation: Direction,
        pub magnitude: LengthMeasure,
    }
}

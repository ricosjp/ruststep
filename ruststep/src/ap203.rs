#![allow(dead_code)]
pub mod config_control_design {
    use crate::{as_holder, derive_more::*, primitive::*, Holder, TableInit};
    use std::collections::HashMap;
    #[derive(Debug, Clone, PartialEq, Default, TableInit)]
    pub struct Tables {
        action: HashMap<u64, as_holder!(Action)>,
        action_assignment: HashMap<u64, as_holder!(ActionAssignment)>,
        action_directive: HashMap<u64, as_holder!(ActionDirective)>,
        action_method: HashMap<u64, as_holder!(ActionMethod)>,
        action_request_assignment: HashMap<u64, as_holder!(ActionRequestAssignment)>,
        action_request_solution: HashMap<u64, as_holder!(ActionRequestSolution)>,
        action_request_status: HashMap<u64, as_holder!(ActionRequestStatus)>,
        action_status: HashMap<u64, as_holder!(ActionStatus)>,
        address: HashMap<u64, as_holder!(Address)>,
        advanced_brep_shape_representation:
            HashMap<u64, as_holder!(AdvancedBrepShapeRepresentation)>,
        advanced_face: HashMap<u64, as_holder!(AdvancedFace)>,
        alternate_product_relationship: HashMap<u64, as_holder!(AlternateProductRelationship)>,
        application_context: HashMap<u64, as_holder!(ApplicationContext)>,
        application_context_element: HashMap<u64, as_holder!(ApplicationContextElement)>,
        application_protocol_definition: HashMap<u64, as_holder!(ApplicationProtocolDefinition)>,
        approval: HashMap<u64, as_holder!(Approval)>,
        approval_assignment: HashMap<u64, as_holder!(ApprovalAssignment)>,
        approval_date_time: HashMap<u64, as_holder!(ApprovalDateTime)>,
        approval_person_organization: HashMap<u64, as_holder!(ApprovalPersonOrganization)>,
        approval_relationship: HashMap<u64, as_holder!(ApprovalRelationship)>,
        approval_role: HashMap<u64, as_holder!(ApprovalRole)>,
        approval_status: HashMap<u64, as_holder!(ApprovalStatus)>,
        area_measure_with_unit: HashMap<u64, as_holder!(AreaMeasureWithUnit)>,
        area_unit: HashMap<u64, as_holder!(AreaUnit)>,
        assembly_component_usage: HashMap<u64, as_holder!(AssemblyComponentUsage)>,
        assembly_component_usage_substitute:
            HashMap<u64, as_holder!(AssemblyComponentUsageSubstitute)>,
        axis1_placement: HashMap<u64, as_holder!(Axis1Placement)>,
        axis2_placement_2d: HashMap<u64, as_holder!(Axis2Placement2D)>,
        axis2_placement_3d: HashMap<u64, as_holder!(Axis2Placement3D)>,
        b_spline_curve: HashMap<u64, as_holder!(BSplineCurve)>,
        b_spline_curve_with_knots: HashMap<u64, as_holder!(BSplineCurveWithKnots)>,
        b_spline_surface: HashMap<u64, as_holder!(BSplineSurface)>,
        b_spline_surface_with_knots: HashMap<u64, as_holder!(BSplineSurfaceWithKnots)>,
        bezier_curve: HashMap<u64, as_holder!(BezierCurve)>,
        bezier_surface: HashMap<u64, as_holder!(BezierSurface)>,
        boundary_curve: HashMap<u64, as_holder!(BoundaryCurve)>,
        bounded_curve: HashMap<u64, as_holder!(BoundedCurve)>,
        bounded_pcurve: HashMap<u64, as_holder!(BoundedPcurve)>,
        bounded_surface: HashMap<u64, as_holder!(BoundedSurface)>,
        bounded_surface_curve: HashMap<u64, as_holder!(BoundedSurfaceCurve)>,
        brep_with_voids: HashMap<u64, as_holder!(BrepWithVoids)>,
        calendar_date: HashMap<u64, as_holder!(CalendarDate)>,
        cartesian_point: HashMap<u64, as_holder!(CartesianPoint)>,
        cartesian_transformation_operator:
            HashMap<u64, as_holder!(CartesianTransformationOperator)>,
        cartesian_transformation_operator_3d:
            HashMap<u64, as_holder!(CartesianTransformationOperator3D)>,
        cc_design_approval: HashMap<u64, as_holder!(CcDesignApproval)>,
        cc_design_certification: HashMap<u64, as_holder!(CcDesignCertification)>,
        cc_design_contract: HashMap<u64, as_holder!(CcDesignContract)>,
        cc_design_date_and_time_assignment: HashMap<u64, as_holder!(CcDesignDateAndTimeAssignment)>,
        cc_design_person_and_organization_assignment:
            HashMap<u64, as_holder!(CcDesignPersonAndOrganizationAssignment)>,
        cc_design_security_classification: HashMap<u64, as_holder!(CcDesignSecurityClassification)>,
        cc_design_specification_reference: HashMap<u64, as_holder!(CcDesignSpecificationReference)>,
        certification: HashMap<u64, as_holder!(Certification)>,
        certification_assignment: HashMap<u64, as_holder!(CertificationAssignment)>,
        certification_type: HashMap<u64, as_holder!(CertificationType)>,
        change: HashMap<u64, as_holder!(Change)>,
        change_request: HashMap<u64, as_holder!(ChangeRequest)>,
        circle: HashMap<u64, as_holder!(Circle)>,
        closed_shell: HashMap<u64, as_holder!(ClosedShell)>,
        composite_curve: HashMap<u64, as_holder!(CompositeCurve)>,
        composite_curve_on_surface: HashMap<u64, as_holder!(CompositeCurveOnSurface)>,
        composite_curve_segment: HashMap<u64, as_holder!(CompositeCurveSegment)>,
        configuration_design: HashMap<u64, as_holder!(ConfigurationDesign)>,
        configuration_effectivity: HashMap<u64, as_holder!(ConfigurationEffectivity)>,
        configuration_item: HashMap<u64, as_holder!(ConfigurationItem)>,
        conic: HashMap<u64, as_holder!(Conic)>,
        conical_surface: HashMap<u64, as_holder!(ConicalSurface)>,
        connected_edge_set: HashMap<u64, as_holder!(ConnectedEdgeSet)>,
        connected_face_set: HashMap<u64, as_holder!(ConnectedFaceSet)>,
        context_dependent_shape_representation:
            HashMap<u64, as_holder!(ContextDependentShapeRepresentation)>,
        context_dependent_unit: HashMap<u64, as_holder!(ContextDependentUnit)>,
        contract: HashMap<u64, as_holder!(Contract)>,
        contract_assignment: HashMap<u64, as_holder!(ContractAssignment)>,
        contract_type: HashMap<u64, as_holder!(ContractType)>,
        conversion_based_unit: HashMap<u64, as_holder!(ConversionBasedUnit)>,
        coordinated_universal_time_offset: HashMap<u64, as_holder!(CoordinatedUniversalTimeOffset)>,
        curve: HashMap<u64, as_holder!(Curve)>,
        curve_bounded_surface: HashMap<u64, as_holder!(CurveBoundedSurface)>,
        curve_replica: HashMap<u64, as_holder!(CurveReplica)>,
        cylindrical_surface: HashMap<u64, as_holder!(CylindricalSurface)>,
        date: HashMap<u64, as_holder!(Date)>,
        date_and_time: HashMap<u64, as_holder!(DateAndTime)>,
        date_and_time_assignment: HashMap<u64, as_holder!(DateAndTimeAssignment)>,
        date_time_role: HashMap<u64, as_holder!(DateTimeRole)>,
        dated_effectivity: HashMap<u64, as_holder!(DatedEffectivity)>,
        definitional_representation: HashMap<u64, as_holder!(DefinitionalRepresentation)>,
        degenerate_pcurve: HashMap<u64, as_holder!(DegeneratePcurve)>,
        degenerate_toroidal_surface: HashMap<u64, as_holder!(DegenerateToroidalSurface)>,
        design_context: HashMap<u64, as_holder!(DesignContext)>,
        design_make_from_relationship: HashMap<u64, as_holder!(DesignMakeFromRelationship)>,
        dimensional_exponents: HashMap<u64, as_holder!(DimensionalExponents)>,
        directed_action: HashMap<u64, as_holder!(DirectedAction)>,
        direction: HashMap<u64, as_holder!(Direction)>,
        document: HashMap<u64, as_holder!(Document)>,
        document_reference: HashMap<u64, as_holder!(DocumentReference)>,
        document_relationship: HashMap<u64, as_holder!(DocumentRelationship)>,
        document_type: HashMap<u64, as_holder!(DocumentType)>,
        document_usage_constraint: HashMap<u64, as_holder!(DocumentUsageConstraint)>,
        document_with_class: HashMap<u64, as_holder!(DocumentWithClass)>,
        edge: HashMap<u64, as_holder!(Edge)>,
        edge_based_wireframe_model: HashMap<u64, as_holder!(EdgeBasedWireframeModel)>,
        edge_based_wireframe_shape_representation:
            HashMap<u64, as_holder!(EdgeBasedWireframeShapeRepresentation)>,
        edge_curve: HashMap<u64, as_holder!(EdgeCurve)>,
        edge_loop: HashMap<u64, as_holder!(EdgeLoop)>,
        effectivity: HashMap<u64, as_holder!(Effectivity)>,
        elementary_surface: HashMap<u64, as_holder!(ElementarySurface)>,
        ellipse: HashMap<u64, as_holder!(Ellipse)>,
        evaluated_degenerate_pcurve: HashMap<u64, as_holder!(EvaluatedDegeneratePcurve)>,
        executed_action: HashMap<u64, as_holder!(ExecutedAction)>,
        face: HashMap<u64, as_holder!(Face)>,
        face_bound: HashMap<u64, as_holder!(FaceBound)>,
        face_outer_bound: HashMap<u64, as_holder!(FaceOuterBound)>,
        face_surface: HashMap<u64, as_holder!(FaceSurface)>,
        faceted_brep: HashMap<u64, as_holder!(FacetedBrep)>,
        faceted_brep_shape_representation: HashMap<u64, as_holder!(FacetedBrepShapeRepresentation)>,
        founded_item: HashMap<u64, as_holder!(FoundedItem)>,
        functionally_defined_transformation:
            HashMap<u64, as_holder!(FunctionallyDefinedTransformation)>,
        geometric_curve_set: HashMap<u64, as_holder!(GeometricCurveSet)>,
        geometric_representation_context: HashMap<u64, as_holder!(GeometricRepresentationContext)>,
        geometric_representation_item: HashMap<u64, as_holder!(GeometricRepresentationItem)>,
        geometric_set: HashMap<u64, as_holder!(GeometricSet)>,
        geometrically_bounded_surface_shape_representation:
            HashMap<u64, as_holder!(GeometricallyBoundedSurfaceShapeRepresentation)>,
        geometrically_bounded_wireframe_shape_representation:
            HashMap<u64, as_holder!(GeometricallyBoundedWireframeShapeRepresentation)>,
        global_uncertainty_assigned_context:
            HashMap<u64, as_holder!(GlobalUncertaintyAssignedContext)>,
        global_unit_assigned_context: HashMap<u64, as_holder!(GlobalUnitAssignedContext)>,
        hyperbola: HashMap<u64, as_holder!(Hyperbola)>,
        intersection_curve: HashMap<u64, as_holder!(IntersectionCurve)>,
        item_defined_transformation: HashMap<u64, as_holder!(ItemDefinedTransformation)>,
        length_measure_with_unit: HashMap<u64, as_holder!(LengthMeasureWithUnit)>,
        length_unit: HashMap<u64, as_holder!(LengthUnit)>,
        line: HashMap<u64, as_holder!(Line)>,
        local_time: HashMap<u64, as_holder!(LocalTime)>,
        r#loop: HashMap<u64, as_holder!(Loop)>,
        lot_effectivity: HashMap<u64, as_holder!(LotEffectivity)>,
        manifold_solid_brep: HashMap<u64, as_holder!(ManifoldSolidBrep)>,
        manifold_surface_shape_representation:
            HashMap<u64, as_holder!(ManifoldSurfaceShapeRepresentation)>,
        mapped_item: HashMap<u64, as_holder!(MappedItem)>,
        mass_measure_with_unit: HashMap<u64, as_holder!(MassMeasureWithUnit)>,
        mass_unit: HashMap<u64, as_holder!(MassUnit)>,
        measure_with_unit: HashMap<u64, as_holder!(MeasureWithUnit)>,
        mechanical_context: HashMap<u64, as_holder!(MechanicalContext)>,
        named_unit: HashMap<u64, as_holder!(NamedUnit)>,
        next_assembly_usage_occurrence: HashMap<u64, as_holder!(NextAssemblyUsageOccurrence)>,
        offset_curve_3d: HashMap<u64, as_holder!(OffsetCurve3D)>,
        offset_surface: HashMap<u64, as_holder!(OffsetSurface)>,
        open_shell: HashMap<u64, as_holder!(OpenShell)>,
        ordinal_date: HashMap<u64, as_holder!(OrdinalDate)>,
        organization: HashMap<u64, as_holder!(Organization)>,
        organization_relationship: HashMap<u64, as_holder!(OrganizationRelationship)>,
        organizational_address: HashMap<u64, as_holder!(OrganizationalAddress)>,
        organizational_project: HashMap<u64, as_holder!(OrganizationalProject)>,
        oriented_closed_shell: HashMap<u64, as_holder!(OrientedClosedShell)>,
        oriented_edge: HashMap<u64, as_holder!(OrientedEdge)>,
        oriented_face: HashMap<u64, as_holder!(OrientedFace)>,
        oriented_open_shell: HashMap<u64, as_holder!(OrientedOpenShell)>,
        oriented_path: HashMap<u64, as_holder!(OrientedPath)>,
        outer_boundary_curve: HashMap<u64, as_holder!(OuterBoundaryCurve)>,
        parabola: HashMap<u64, as_holder!(Parabola)>,
        parametric_representation_context:
            HashMap<u64, as_holder!(ParametricRepresentationContext)>,
        path: HashMap<u64, as_holder!(Path)>,
        pcurve: HashMap<u64, as_holder!(Pcurve)>,
        person: HashMap<u64, as_holder!(Person)>,
        person_and_organization: HashMap<u64, as_holder!(PersonAndOrganization)>,
        person_and_organization_assignment:
            HashMap<u64, as_holder!(PersonAndOrganizationAssignment)>,
        person_and_organization_role: HashMap<u64, as_holder!(PersonAndOrganizationRole)>,
        personal_address: HashMap<u64, as_holder!(PersonalAddress)>,
        placement: HashMap<u64, as_holder!(Placement)>,
        plane: HashMap<u64, as_holder!(Plane)>,
        plane_angle_measure_with_unit: HashMap<u64, as_holder!(PlaneAngleMeasureWithUnit)>,
        plane_angle_unit: HashMap<u64, as_holder!(PlaneAngleUnit)>,
        point: HashMap<u64, as_holder!(Point)>,
        point_on_curve: HashMap<u64, as_holder!(PointOnCurve)>,
        point_on_surface: HashMap<u64, as_holder!(PointOnSurface)>,
        point_replica: HashMap<u64, as_holder!(PointReplica)>,
        poly_loop: HashMap<u64, as_holder!(PolyLoop)>,
        polyline: HashMap<u64, as_holder!(Polyline)>,
        product: HashMap<u64, as_holder!(Product)>,
        product_category: HashMap<u64, as_holder!(ProductCategory)>,
        product_category_relationship: HashMap<u64, as_holder!(ProductCategoryRelationship)>,
        product_concept: HashMap<u64, as_holder!(ProductConcept)>,
        product_concept_context: HashMap<u64, as_holder!(ProductConceptContext)>,
        product_context: HashMap<u64, as_holder!(ProductContext)>,
        product_definition: HashMap<u64, as_holder!(ProductDefinition)>,
        product_definition_context: HashMap<u64, as_holder!(ProductDefinitionContext)>,
        product_definition_effectivity: HashMap<u64, as_holder!(ProductDefinitionEffectivity)>,
        product_definition_formation: HashMap<u64, as_holder!(ProductDefinitionFormation)>,
        product_definition_formation_with_specified_source:
            HashMap<u64, as_holder!(ProductDefinitionFormationWithSpecifiedSource)>,
        product_definition_relationship: HashMap<u64, as_holder!(ProductDefinitionRelationship)>,
        product_definition_shape: HashMap<u64, as_holder!(ProductDefinitionShape)>,
        product_definition_usage: HashMap<u64, as_holder!(ProductDefinitionUsage)>,
        product_definition_with_associated_documents:
            HashMap<u64, as_holder!(ProductDefinitionWithAssociatedDocuments)>,
        product_related_product_category: HashMap<u64, as_holder!(ProductRelatedProductCategory)>,
        promissory_usage_occurrence: HashMap<u64, as_holder!(PromissoryUsageOccurrence)>,
        property_definition: HashMap<u64, as_holder!(PropertyDefinition)>,
        property_definition_representation:
            HashMap<u64, as_holder!(PropertyDefinitionRepresentation)>,
        quantified_assembly_component_usage:
            HashMap<u64, as_holder!(QuantifiedAssemblyComponentUsage)>,
        quasi_uniform_curve: HashMap<u64, as_holder!(QuasiUniformCurve)>,
        quasi_uniform_surface: HashMap<u64, as_holder!(QuasiUniformSurface)>,
        rational_b_spline_curve: HashMap<u64, as_holder!(RationalBSplineCurve)>,
        rational_b_spline_surface: HashMap<u64, as_holder!(RationalBSplineSurface)>,
        rectangular_composite_surface: HashMap<u64, as_holder!(RectangularCompositeSurface)>,
        rectangular_trimmed_surface: HashMap<u64, as_holder!(RectangularTrimmedSurface)>,
        reparametrised_composite_curve_segment:
            HashMap<u64, as_holder!(ReparametrisedCompositeCurveSegment)>,
        representation: HashMap<u64, as_holder!(Representation)>,
        representation_context: HashMap<u64, as_holder!(RepresentationContext)>,
        representation_item: HashMap<u64, as_holder!(RepresentationItem)>,
        representation_map: HashMap<u64, as_holder!(RepresentationMap)>,
        representation_relationship: HashMap<u64, as_holder!(RepresentationRelationship)>,
        representation_relationship_with_transformation:
            HashMap<u64, as_holder!(RepresentationRelationshipWithTransformation)>,
        seam_curve: HashMap<u64, as_holder!(SeamCurve)>,
        security_classification: HashMap<u64, as_holder!(SecurityClassification)>,
        security_classification_assignment:
            HashMap<u64, as_holder!(SecurityClassificationAssignment)>,
        security_classification_level: HashMap<u64, as_holder!(SecurityClassificationLevel)>,
        serial_numbered_effectivity: HashMap<u64, as_holder!(SerialNumberedEffectivity)>,
        shape_aspect: HashMap<u64, as_holder!(ShapeAspect)>,
        shape_aspect_relationship: HashMap<u64, as_holder!(ShapeAspectRelationship)>,
        shape_definition_representation: HashMap<u64, as_holder!(ShapeDefinitionRepresentation)>,
        shape_representation: HashMap<u64, as_holder!(ShapeRepresentation)>,
        shape_representation_relationship:
            HashMap<u64, as_holder!(ShapeRepresentationRelationship)>,
        shell_based_surface_model: HashMap<u64, as_holder!(ShellBasedSurfaceModel)>,
        shell_based_wireframe_model: HashMap<u64, as_holder!(ShellBasedWireframeModel)>,
        shell_based_wireframe_shape_representation:
            HashMap<u64, as_holder!(ShellBasedWireframeShapeRepresentation)>,
        si_unit: HashMap<u64, as_holder!(SiUnit)>,
        solid_angle_measure_with_unit: HashMap<u64, as_holder!(SolidAngleMeasureWithUnit)>,
        solid_angle_unit: HashMap<u64, as_holder!(SolidAngleUnit)>,
        solid_model: HashMap<u64, as_holder!(SolidModel)>,
        specified_higher_usage_occurrence: HashMap<u64, as_holder!(SpecifiedHigherUsageOccurrence)>,
        spherical_surface: HashMap<u64, as_holder!(SphericalSurface)>,
        start_request: HashMap<u64, as_holder!(StartRequest)>,
        start_work: HashMap<u64, as_holder!(StartWork)>,
        supplied_part_relationship: HashMap<u64, as_holder!(SuppliedPartRelationship)>,
        surface: HashMap<u64, as_holder!(Surface)>,
        surface_curve: HashMap<u64, as_holder!(SurfaceCurve)>,
        surface_of_linear_extrusion: HashMap<u64, as_holder!(SurfaceOfLinearExtrusion)>,
        surface_of_revolution: HashMap<u64, as_holder!(SurfaceOfRevolution)>,
        surface_patch: HashMap<u64, as_holder!(SurfacePatch)>,
        surface_replica: HashMap<u64, as_holder!(SurfaceReplica)>,
        swept_surface: HashMap<u64, as_holder!(SweptSurface)>,
        topological_representation_item: HashMap<u64, as_holder!(TopologicalRepresentationItem)>,
        toroidal_surface: HashMap<u64, as_holder!(ToroidalSurface)>,
        trimmed_curve: HashMap<u64, as_holder!(TrimmedCurve)>,
        uncertainty_measure_with_unit: HashMap<u64, as_holder!(UncertaintyMeasureWithUnit)>,
        uniform_curve: HashMap<u64, as_holder!(UniformCurve)>,
        uniform_surface: HashMap<u64, as_holder!(UniformSurface)>,
        vector: HashMap<u64, as_holder!(Vector)>,
        versioned_action_request: HashMap<u64, as_holder!(VersionedActionRequest)>,
        vertex: HashMap<u64, as_holder!(Vertex)>,
        vertex_loop: HashMap<u64, as_holder!(VertexLoop)>,
        vertex_point: HashMap<u64, as_holder!(VertexPoint)>,
        vertex_shell: HashMap<u64, as_holder!(VertexShell)>,
        volume_measure_with_unit: HashMap<u64, as_holder!(VolumeMeasureWithUnit)>,
        volume_unit: HashMap<u64, as_holder!(VolumeUnit)>,
        week_of_year_and_day_date: HashMap<u64, as_holder!(WeekOfYearAndDayDate)>,
        wire_shell: HashMap<u64, as_holder!(WireShell)>,
        approved_item: HashMap<u64, as_holder!(ApprovedItem)>,
        area_measure: HashMap<u64, as_holder!(AreaMeasure)>,
        axis2_placement: HashMap<u64, as_holder!(Axis2Placement)>,
        boolean_operand: HashMap<u64, as_holder!(BooleanOperand)>,
        certified_item: HashMap<u64, as_holder!(CertifiedItem)>,
        change_request_item: HashMap<u64, as_holder!(ChangeRequestItem)>,
        characterized_definition: HashMap<u64, as_holder!(CharacterizedDefinition)>,
        characterized_product_definition: HashMap<u64, as_holder!(CharacterizedProductDefinition)>,
        classified_item: HashMap<u64, as_holder!(ClassifiedItem)>,
        context_dependent_measure: HashMap<u64, as_holder!(ContextDependentMeasure)>,
        contracted_item: HashMap<u64, as_holder!(ContractedItem)>,
        count_measure: HashMap<u64, as_holder!(CountMeasure)>,
        curve_on_surface: HashMap<u64, as_holder!(CurveOnSurface)>,
        date_time_item: HashMap<u64, as_holder!(DateTimeItem)>,
        date_time_select: HashMap<u64, as_holder!(DateTimeSelect)>,
        day_in_month_number: HashMap<u64, as_holder!(DayInMonthNumber)>,
        day_in_week_number: HashMap<u64, as_holder!(DayInWeekNumber)>,
        day_in_year_number: HashMap<u64, as_holder!(DayInYearNumber)>,
        descriptive_measure: HashMap<u64, as_holder!(DescriptiveMeasure)>,
        dimension_count: HashMap<u64, as_holder!(DimensionCount)>,
        founded_item_select: HashMap<u64, as_holder!(FoundedItemSelect)>,
        geometric_set_select: HashMap<u64, as_holder!(GeometricSetSelect)>,
        hour_in_day: HashMap<u64, as_holder!(HourInDay)>,
        identifier: HashMap<u64, as_holder!(Identifier)>,
        label: HashMap<u64, as_holder!(Label)>,
        length_measure: HashMap<u64, as_holder!(LengthMeasure)>,
        list_of_reversible_topology_item: HashMap<u64, as_holder!(ListOfReversibleTopologyItem)>,
        mass_measure: HashMap<u64, as_holder!(MassMeasure)>,
        measure_value: HashMap<u64, as_holder!(MeasureValue)>,
        minute_in_hour: HashMap<u64, as_holder!(MinuteInHour)>,
        month_in_year_number: HashMap<u64, as_holder!(MonthInYearNumber)>,
        parameter_value: HashMap<u64, as_holder!(ParameterValue)>,
        pcurve_or_surface: HashMap<u64, as_holder!(PcurveOrSurface)>,
        person_organization_item: HashMap<u64, as_holder!(PersonOrganizationItem)>,
        person_organization_select: HashMap<u64, as_holder!(PersonOrganizationSelect)>,
        plane_angle_measure: HashMap<u64, as_holder!(PlaneAngleMeasure)>,
        positive_length_measure: HashMap<u64, as_holder!(PositiveLengthMeasure)>,
        positive_plane_angle_measure: HashMap<u64, as_holder!(PositivePlaneAngleMeasure)>,
        reversible_topology: HashMap<u64, as_holder!(ReversibleTopology)>,
        reversible_topology_item: HashMap<u64, as_holder!(ReversibleTopologyItem)>,
        second_in_minute: HashMap<u64, as_holder!(SecondInMinute)>,
        set_of_reversible_topology_item: HashMap<u64, as_holder!(SetOfReversibleTopologyItem)>,
        shape_definition: HashMap<u64, as_holder!(ShapeDefinition)>,
        shell: HashMap<u64, as_holder!(Shell)>,
        solid_angle_measure: HashMap<u64, as_holder!(SolidAngleMeasure)>,
        specified_item: HashMap<u64, as_holder!(SpecifiedItem)>,
        start_request_item: HashMap<u64, as_holder!(StartRequestItem)>,
        supported_item: HashMap<u64, as_holder!(SupportedItem)>,
        surface_model: HashMap<u64, as_holder!(SurfaceModel)>,
        text: HashMap<u64, as_holder!(Text)>,
        transformation: HashMap<u64, as_holder!(Transformation)>,
        trimming_select: HashMap<u64, as_holder!(TrimmingSelect)>,
        unit: HashMap<u64, as_holder!(Unit)>,
        vector_or_direction: HashMap<u64, as_holder!(VectorOrDirection)>,
        volume_measure: HashMap<u64, as_holder!(VolumeMeasure)>,
        week_in_year_number: HashMap<u64, as_holder!(WeekInYearNumber)>,
        wireframe_model: HashMap<u64, as_holder!(WireframeModel)>,
        work_item: HashMap<u64, as_holder!(WorkItem)>,
        year_number: HashMap<u64, as_holder!(YearNumber)>,
    }
    impl Tables {
        pub fn action_holders(&self) -> &HashMap<u64, as_holder!(Action)> {
            &self.action
        }
        pub fn action_assignment_holders(&self) -> &HashMap<u64, as_holder!(ActionAssignment)> {
            &self.action_assignment
        }
        pub fn action_directive_holders(&self) -> &HashMap<u64, as_holder!(ActionDirective)> {
            &self.action_directive
        }
        pub fn action_method_holders(&self) -> &HashMap<u64, as_holder!(ActionMethod)> {
            &self.action_method
        }
        pub fn action_request_assignment_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(ActionRequestAssignment)> {
            &self.action_request_assignment
        }
        pub fn action_request_solution_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(ActionRequestSolution)> {
            &self.action_request_solution
        }
        pub fn action_request_status_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(ActionRequestStatus)> {
            &self.action_request_status
        }
        pub fn action_status_holders(&self) -> &HashMap<u64, as_holder!(ActionStatus)> {
            &self.action_status
        }
        pub fn address_holders(&self) -> &HashMap<u64, as_holder!(Address)> {
            &self.address
        }
        pub fn advanced_brep_shape_representation_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(AdvancedBrepShapeRepresentation)> {
            &self.advanced_brep_shape_representation
        }
        pub fn advanced_face_holders(&self) -> &HashMap<u64, as_holder!(AdvancedFace)> {
            &self.advanced_face
        }
        pub fn alternate_product_relationship_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(AlternateProductRelationship)> {
            &self.alternate_product_relationship
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
        pub fn approval_relationship_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(ApprovalRelationship)> {
            &self.approval_relationship
        }
        pub fn approval_role_holders(&self) -> &HashMap<u64, as_holder!(ApprovalRole)> {
            &self.approval_role
        }
        pub fn approval_status_holders(&self) -> &HashMap<u64, as_holder!(ApprovalStatus)> {
            &self.approval_status
        }
        pub fn area_measure_with_unit_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(AreaMeasureWithUnit)> {
            &self.area_measure_with_unit
        }
        pub fn area_unit_holders(&self) -> &HashMap<u64, as_holder!(AreaUnit)> {
            &self.area_unit
        }
        pub fn assembly_component_usage_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(AssemblyComponentUsage)> {
            &self.assembly_component_usage
        }
        pub fn assembly_component_usage_substitute_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(AssemblyComponentUsageSubstitute)> {
            &self.assembly_component_usage_substitute
        }
        pub fn axis1_placement_holders(&self) -> &HashMap<u64, as_holder!(Axis1Placement)> {
            &self.axis1_placement
        }
        pub fn axis2_placement_2d_holders(&self) -> &HashMap<u64, as_holder!(Axis2Placement2D)> {
            &self.axis2_placement_2d
        }
        pub fn axis2_placement_3d_holders(&self) -> &HashMap<u64, as_holder!(Axis2Placement3D)> {
            &self.axis2_placement_3d
        }
        pub fn b_spline_curve_holders(&self) -> &HashMap<u64, as_holder!(BSplineCurve)> {
            &self.b_spline_curve
        }
        pub fn b_spline_curve_with_knots_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(BSplineCurveWithKnots)> {
            &self.b_spline_curve_with_knots
        }
        pub fn b_spline_surface_holders(&self) -> &HashMap<u64, as_holder!(BSplineSurface)> {
            &self.b_spline_surface
        }
        pub fn b_spline_surface_with_knots_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(BSplineSurfaceWithKnots)> {
            &self.b_spline_surface_with_knots
        }
        pub fn bezier_curve_holders(&self) -> &HashMap<u64, as_holder!(BezierCurve)> {
            &self.bezier_curve
        }
        pub fn bezier_surface_holders(&self) -> &HashMap<u64, as_holder!(BezierSurface)> {
            &self.bezier_surface
        }
        pub fn boundary_curve_holders(&self) -> &HashMap<u64, as_holder!(BoundaryCurve)> {
            &self.boundary_curve
        }
        pub fn bounded_curve_holders(&self) -> &HashMap<u64, as_holder!(BoundedCurve)> {
            &self.bounded_curve
        }
        pub fn bounded_pcurve_holders(&self) -> &HashMap<u64, as_holder!(BoundedPcurve)> {
            &self.bounded_pcurve
        }
        pub fn bounded_surface_holders(&self) -> &HashMap<u64, as_holder!(BoundedSurface)> {
            &self.bounded_surface
        }
        pub fn bounded_surface_curve_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(BoundedSurfaceCurve)> {
            &self.bounded_surface_curve
        }
        pub fn brep_with_voids_holders(&self) -> &HashMap<u64, as_holder!(BrepWithVoids)> {
            &self.brep_with_voids
        }
        pub fn calendar_date_holders(&self) -> &HashMap<u64, as_holder!(CalendarDate)> {
            &self.calendar_date
        }
        pub fn cartesian_point_holders(&self) -> &HashMap<u64, as_holder!(CartesianPoint)> {
            &self.cartesian_point
        }
        pub fn cartesian_transformation_operator_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(CartesianTransformationOperator)> {
            &self.cartesian_transformation_operator
        }
        pub fn cartesian_transformation_operator_3d_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(CartesianTransformationOperator3D)> {
            &self.cartesian_transformation_operator_3d
        }
        pub fn cc_design_approval_holders(&self) -> &HashMap<u64, as_holder!(CcDesignApproval)> {
            &self.cc_design_approval
        }
        pub fn cc_design_certification_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(CcDesignCertification)> {
            &self.cc_design_certification
        }
        pub fn cc_design_contract_holders(&self) -> &HashMap<u64, as_holder!(CcDesignContract)> {
            &self.cc_design_contract
        }
        pub fn cc_design_date_and_time_assignment_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(CcDesignDateAndTimeAssignment)> {
            &self.cc_design_date_and_time_assignment
        }
        pub fn cc_design_person_and_organization_assignment_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(CcDesignPersonAndOrganizationAssignment)> {
            &self.cc_design_person_and_organization_assignment
        }
        pub fn cc_design_security_classification_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(CcDesignSecurityClassification)> {
            &self.cc_design_security_classification
        }
        pub fn cc_design_specification_reference_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(CcDesignSpecificationReference)> {
            &self.cc_design_specification_reference
        }
        pub fn certification_holders(&self) -> &HashMap<u64, as_holder!(Certification)> {
            &self.certification
        }
        pub fn certification_assignment_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(CertificationAssignment)> {
            &self.certification_assignment
        }
        pub fn certification_type_holders(&self) -> &HashMap<u64, as_holder!(CertificationType)> {
            &self.certification_type
        }
        pub fn change_holders(&self) -> &HashMap<u64, as_holder!(Change)> {
            &self.change
        }
        pub fn change_request_holders(&self) -> &HashMap<u64, as_holder!(ChangeRequest)> {
            &self.change_request
        }
        pub fn circle_holders(&self) -> &HashMap<u64, as_holder!(Circle)> {
            &self.circle
        }
        pub fn closed_shell_holders(&self) -> &HashMap<u64, as_holder!(ClosedShell)> {
            &self.closed_shell
        }
        pub fn composite_curve_holders(&self) -> &HashMap<u64, as_holder!(CompositeCurve)> {
            &self.composite_curve
        }
        pub fn composite_curve_on_surface_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(CompositeCurveOnSurface)> {
            &self.composite_curve_on_surface
        }
        pub fn composite_curve_segment_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(CompositeCurveSegment)> {
            &self.composite_curve_segment
        }
        pub fn configuration_design_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(ConfigurationDesign)> {
            &self.configuration_design
        }
        pub fn configuration_effectivity_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(ConfigurationEffectivity)> {
            &self.configuration_effectivity
        }
        pub fn configuration_item_holders(&self) -> &HashMap<u64, as_holder!(ConfigurationItem)> {
            &self.configuration_item
        }
        pub fn conic_holders(&self) -> &HashMap<u64, as_holder!(Conic)> {
            &self.conic
        }
        pub fn conical_surface_holders(&self) -> &HashMap<u64, as_holder!(ConicalSurface)> {
            &self.conical_surface
        }
        pub fn connected_edge_set_holders(&self) -> &HashMap<u64, as_holder!(ConnectedEdgeSet)> {
            &self.connected_edge_set
        }
        pub fn connected_face_set_holders(&self) -> &HashMap<u64, as_holder!(ConnectedFaceSet)> {
            &self.connected_face_set
        }
        pub fn context_dependent_shape_representation_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(ContextDependentShapeRepresentation)> {
            &self.context_dependent_shape_representation
        }
        pub fn context_dependent_unit_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(ContextDependentUnit)> {
            &self.context_dependent_unit
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
        pub fn coordinated_universal_time_offset_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(CoordinatedUniversalTimeOffset)> {
            &self.coordinated_universal_time_offset
        }
        pub fn curve_holders(&self) -> &HashMap<u64, as_holder!(Curve)> {
            &self.curve
        }
        pub fn curve_bounded_surface_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(CurveBoundedSurface)> {
            &self.curve_bounded_surface
        }
        pub fn curve_replica_holders(&self) -> &HashMap<u64, as_holder!(CurveReplica)> {
            &self.curve_replica
        }
        pub fn cylindrical_surface_holders(&self) -> &HashMap<u64, as_holder!(CylindricalSurface)> {
            &self.cylindrical_surface
        }
        pub fn date_holders(&self) -> &HashMap<u64, as_holder!(Date)> {
            &self.date
        }
        pub fn date_and_time_holders(&self) -> &HashMap<u64, as_holder!(DateAndTime)> {
            &self.date_and_time
        }
        pub fn date_and_time_assignment_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(DateAndTimeAssignment)> {
            &self.date_and_time_assignment
        }
        pub fn date_time_role_holders(&self) -> &HashMap<u64, as_holder!(DateTimeRole)> {
            &self.date_time_role
        }
        pub fn dated_effectivity_holders(&self) -> &HashMap<u64, as_holder!(DatedEffectivity)> {
            &self.dated_effectivity
        }
        pub fn definitional_representation_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(DefinitionalRepresentation)> {
            &self.definitional_representation
        }
        pub fn degenerate_pcurve_holders(&self) -> &HashMap<u64, as_holder!(DegeneratePcurve)> {
            &self.degenerate_pcurve
        }
        pub fn degenerate_toroidal_surface_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(DegenerateToroidalSurface)> {
            &self.degenerate_toroidal_surface
        }
        pub fn design_context_holders(&self) -> &HashMap<u64, as_holder!(DesignContext)> {
            &self.design_context
        }
        pub fn design_make_from_relationship_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(DesignMakeFromRelationship)> {
            &self.design_make_from_relationship
        }
        pub fn dimensional_exponents_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(DimensionalExponents)> {
            &self.dimensional_exponents
        }
        pub fn directed_action_holders(&self) -> &HashMap<u64, as_holder!(DirectedAction)> {
            &self.directed_action
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
        pub fn document_relationship_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(DocumentRelationship)> {
            &self.document_relationship
        }
        pub fn document_type_holders(&self) -> &HashMap<u64, as_holder!(DocumentType)> {
            &self.document_type
        }
        pub fn document_usage_constraint_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(DocumentUsageConstraint)> {
            &self.document_usage_constraint
        }
        pub fn document_with_class_holders(&self) -> &HashMap<u64, as_holder!(DocumentWithClass)> {
            &self.document_with_class
        }
        pub fn edge_holders(&self) -> &HashMap<u64, as_holder!(Edge)> {
            &self.edge
        }
        pub fn edge_based_wireframe_model_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(EdgeBasedWireframeModel)> {
            &self.edge_based_wireframe_model
        }
        pub fn edge_based_wireframe_shape_representation_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(EdgeBasedWireframeShapeRepresentation)> {
            &self.edge_based_wireframe_shape_representation
        }
        pub fn edge_curve_holders(&self) -> &HashMap<u64, as_holder!(EdgeCurve)> {
            &self.edge_curve
        }
        pub fn edge_loop_holders(&self) -> &HashMap<u64, as_holder!(EdgeLoop)> {
            &self.edge_loop
        }
        pub fn effectivity_holders(&self) -> &HashMap<u64, as_holder!(Effectivity)> {
            &self.effectivity
        }
        pub fn elementary_surface_holders(&self) -> &HashMap<u64, as_holder!(ElementarySurface)> {
            &self.elementary_surface
        }
        pub fn ellipse_holders(&self) -> &HashMap<u64, as_holder!(Ellipse)> {
            &self.ellipse
        }
        pub fn evaluated_degenerate_pcurve_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(EvaluatedDegeneratePcurve)> {
            &self.evaluated_degenerate_pcurve
        }
        pub fn executed_action_holders(&self) -> &HashMap<u64, as_holder!(ExecutedAction)> {
            &self.executed_action
        }
        pub fn face_holders(&self) -> &HashMap<u64, as_holder!(Face)> {
            &self.face
        }
        pub fn face_bound_holders(&self) -> &HashMap<u64, as_holder!(FaceBound)> {
            &self.face_bound
        }
        pub fn face_outer_bound_holders(&self) -> &HashMap<u64, as_holder!(FaceOuterBound)> {
            &self.face_outer_bound
        }
        pub fn face_surface_holders(&self) -> &HashMap<u64, as_holder!(FaceSurface)> {
            &self.face_surface
        }
        pub fn faceted_brep_holders(&self) -> &HashMap<u64, as_holder!(FacetedBrep)> {
            &self.faceted_brep
        }
        pub fn faceted_brep_shape_representation_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(FacetedBrepShapeRepresentation)> {
            &self.faceted_brep_shape_representation
        }
        pub fn founded_item_holders(&self) -> &HashMap<u64, as_holder!(FoundedItem)> {
            &self.founded_item
        }
        pub fn functionally_defined_transformation_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(FunctionallyDefinedTransformation)> {
            &self.functionally_defined_transformation
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
        pub fn geometrically_bounded_surface_shape_representation_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(GeometricallyBoundedSurfaceShapeRepresentation)> {
            &self.geometrically_bounded_surface_shape_representation
        }
        pub fn geometrically_bounded_wireframe_shape_representation_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(GeometricallyBoundedWireframeShapeRepresentation)> {
            &self.geometrically_bounded_wireframe_shape_representation
        }
        pub fn global_uncertainty_assigned_context_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(GlobalUncertaintyAssignedContext)> {
            &self.global_uncertainty_assigned_context
        }
        pub fn global_unit_assigned_context_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(GlobalUnitAssignedContext)> {
            &self.global_unit_assigned_context
        }
        pub fn hyperbola_holders(&self) -> &HashMap<u64, as_holder!(Hyperbola)> {
            &self.hyperbola
        }
        pub fn intersection_curve_holders(&self) -> &HashMap<u64, as_holder!(IntersectionCurve)> {
            &self.intersection_curve
        }
        pub fn item_defined_transformation_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(ItemDefinedTransformation)> {
            &self.item_defined_transformation
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
        pub fn local_time_holders(&self) -> &HashMap<u64, as_holder!(LocalTime)> {
            &self.local_time
        }
        pub fn loop_holders(&self) -> &HashMap<u64, as_holder!(Loop)> {
            &self.r#loop
        }
        pub fn lot_effectivity_holders(&self) -> &HashMap<u64, as_holder!(LotEffectivity)> {
            &self.lot_effectivity
        }
        pub fn manifold_solid_brep_holders(&self) -> &HashMap<u64, as_holder!(ManifoldSolidBrep)> {
            &self.manifold_solid_brep
        }
        pub fn manifold_surface_shape_representation_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(ManifoldSurfaceShapeRepresentation)> {
            &self.manifold_surface_shape_representation
        }
        pub fn mapped_item_holders(&self) -> &HashMap<u64, as_holder!(MappedItem)> {
            &self.mapped_item
        }
        pub fn mass_measure_with_unit_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(MassMeasureWithUnit)> {
            &self.mass_measure_with_unit
        }
        pub fn mass_unit_holders(&self) -> &HashMap<u64, as_holder!(MassUnit)> {
            &self.mass_unit
        }
        pub fn measure_with_unit_holders(&self) -> &HashMap<u64, as_holder!(MeasureWithUnit)> {
            &self.measure_with_unit
        }
        pub fn mechanical_context_holders(&self) -> &HashMap<u64, as_holder!(MechanicalContext)> {
            &self.mechanical_context
        }
        pub fn named_unit_holders(&self) -> &HashMap<u64, as_holder!(NamedUnit)> {
            &self.named_unit
        }
        pub fn next_assembly_usage_occurrence_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(NextAssemblyUsageOccurrence)> {
            &self.next_assembly_usage_occurrence
        }
        pub fn offset_curve_3d_holders(&self) -> &HashMap<u64, as_holder!(OffsetCurve3D)> {
            &self.offset_curve_3d
        }
        pub fn offset_surface_holders(&self) -> &HashMap<u64, as_holder!(OffsetSurface)> {
            &self.offset_surface
        }
        pub fn open_shell_holders(&self) -> &HashMap<u64, as_holder!(OpenShell)> {
            &self.open_shell
        }
        pub fn ordinal_date_holders(&self) -> &HashMap<u64, as_holder!(OrdinalDate)> {
            &self.ordinal_date
        }
        pub fn organization_holders(&self) -> &HashMap<u64, as_holder!(Organization)> {
            &self.organization
        }
        pub fn organization_relationship_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(OrganizationRelationship)> {
            &self.organization_relationship
        }
        pub fn organizational_address_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(OrganizationalAddress)> {
            &self.organizational_address
        }
        pub fn organizational_project_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(OrganizationalProject)> {
            &self.organizational_project
        }
        pub fn oriented_closed_shell_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(OrientedClosedShell)> {
            &self.oriented_closed_shell
        }
        pub fn oriented_edge_holders(&self) -> &HashMap<u64, as_holder!(OrientedEdge)> {
            &self.oriented_edge
        }
        pub fn oriented_face_holders(&self) -> &HashMap<u64, as_holder!(OrientedFace)> {
            &self.oriented_face
        }
        pub fn oriented_open_shell_holders(&self) -> &HashMap<u64, as_holder!(OrientedOpenShell)> {
            &self.oriented_open_shell
        }
        pub fn oriented_path_holders(&self) -> &HashMap<u64, as_holder!(OrientedPath)> {
            &self.oriented_path
        }
        pub fn outer_boundary_curve_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(OuterBoundaryCurve)> {
            &self.outer_boundary_curve
        }
        pub fn parabola_holders(&self) -> &HashMap<u64, as_holder!(Parabola)> {
            &self.parabola
        }
        pub fn parametric_representation_context_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(ParametricRepresentationContext)> {
            &self.parametric_representation_context
        }
        pub fn path_holders(&self) -> &HashMap<u64, as_holder!(Path)> {
            &self.path
        }
        pub fn pcurve_holders(&self) -> &HashMap<u64, as_holder!(Pcurve)> {
            &self.pcurve
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
        pub fn personal_address_holders(&self) -> &HashMap<u64, as_holder!(PersonalAddress)> {
            &self.personal_address
        }
        pub fn placement_holders(&self) -> &HashMap<u64, as_holder!(Placement)> {
            &self.placement
        }
        pub fn plane_holders(&self) -> &HashMap<u64, as_holder!(Plane)> {
            &self.plane
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
        pub fn point_on_surface_holders(&self) -> &HashMap<u64, as_holder!(PointOnSurface)> {
            &self.point_on_surface
        }
        pub fn point_replica_holders(&self) -> &HashMap<u64, as_holder!(PointReplica)> {
            &self.point_replica
        }
        pub fn poly_loop_holders(&self) -> &HashMap<u64, as_holder!(PolyLoop)> {
            &self.poly_loop
        }
        pub fn polyline_holders(&self) -> &HashMap<u64, as_holder!(Polyline)> {
            &self.polyline
        }
        pub fn product_holders(&self) -> &HashMap<u64, as_holder!(Product)> {
            &self.product
        }
        pub fn product_category_holders(&self) -> &HashMap<u64, as_holder!(ProductCategory)> {
            &self.product_category
        }
        pub fn product_category_relationship_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(ProductCategoryRelationship)> {
            &self.product_category_relationship
        }
        pub fn product_concept_holders(&self) -> &HashMap<u64, as_holder!(ProductConcept)> {
            &self.product_concept
        }
        pub fn product_concept_context_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(ProductConceptContext)> {
            &self.product_concept_context
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
        pub fn product_definition_effectivity_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(ProductDefinitionEffectivity)> {
            &self.product_definition_effectivity
        }
        pub fn product_definition_formation_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(ProductDefinitionFormation)> {
            &self.product_definition_formation
        }
        pub fn product_definition_formation_with_specified_source_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(ProductDefinitionFormationWithSpecifiedSource)> {
            &self.product_definition_formation_with_specified_source
        }
        pub fn product_definition_relationship_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(ProductDefinitionRelationship)> {
            &self.product_definition_relationship
        }
        pub fn product_definition_shape_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(ProductDefinitionShape)> {
            &self.product_definition_shape
        }
        pub fn product_definition_usage_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(ProductDefinitionUsage)> {
            &self.product_definition_usage
        }
        pub fn product_definition_with_associated_documents_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(ProductDefinitionWithAssociatedDocuments)> {
            &self.product_definition_with_associated_documents
        }
        pub fn product_related_product_category_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(ProductRelatedProductCategory)> {
            &self.product_related_product_category
        }
        pub fn promissory_usage_occurrence_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(PromissoryUsageOccurrence)> {
            &self.promissory_usage_occurrence
        }
        pub fn property_definition_holders(&self) -> &HashMap<u64, as_holder!(PropertyDefinition)> {
            &self.property_definition
        }
        pub fn property_definition_representation_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(PropertyDefinitionRepresentation)> {
            &self.property_definition_representation
        }
        pub fn quantified_assembly_component_usage_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(QuantifiedAssemblyComponentUsage)> {
            &self.quantified_assembly_component_usage
        }
        pub fn quasi_uniform_curve_holders(&self) -> &HashMap<u64, as_holder!(QuasiUniformCurve)> {
            &self.quasi_uniform_curve
        }
        pub fn quasi_uniform_surface_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(QuasiUniformSurface)> {
            &self.quasi_uniform_surface
        }
        pub fn rational_b_spline_curve_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(RationalBSplineCurve)> {
            &self.rational_b_spline_curve
        }
        pub fn rational_b_spline_surface_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(RationalBSplineSurface)> {
            &self.rational_b_spline_surface
        }
        pub fn rectangular_composite_surface_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(RectangularCompositeSurface)> {
            &self.rectangular_composite_surface
        }
        pub fn rectangular_trimmed_surface_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(RectangularTrimmedSurface)> {
            &self.rectangular_trimmed_surface
        }
        pub fn reparametrised_composite_curve_segment_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(ReparametrisedCompositeCurveSegment)> {
            &self.reparametrised_composite_curve_segment
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
        pub fn representation_relationship_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(RepresentationRelationship)> {
            &self.representation_relationship
        }
        pub fn representation_relationship_with_transformation_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(RepresentationRelationshipWithTransformation)> {
            &self.representation_relationship_with_transformation
        }
        pub fn seam_curve_holders(&self) -> &HashMap<u64, as_holder!(SeamCurve)> {
            &self.seam_curve
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
        pub fn serial_numbered_effectivity_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(SerialNumberedEffectivity)> {
            &self.serial_numbered_effectivity
        }
        pub fn shape_aspect_holders(&self) -> &HashMap<u64, as_holder!(ShapeAspect)> {
            &self.shape_aspect
        }
        pub fn shape_aspect_relationship_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(ShapeAspectRelationship)> {
            &self.shape_aspect_relationship
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
        pub fn shape_representation_relationship_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(ShapeRepresentationRelationship)> {
            &self.shape_representation_relationship
        }
        pub fn shell_based_surface_model_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(ShellBasedSurfaceModel)> {
            &self.shell_based_surface_model
        }
        pub fn shell_based_wireframe_model_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(ShellBasedWireframeModel)> {
            &self.shell_based_wireframe_model
        }
        pub fn shell_based_wireframe_shape_representation_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(ShellBasedWireframeShapeRepresentation)> {
            &self.shell_based_wireframe_shape_representation
        }
        pub fn si_unit_holders(&self) -> &HashMap<u64, as_holder!(SiUnit)> {
            &self.si_unit
        }
        pub fn solid_angle_measure_with_unit_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(SolidAngleMeasureWithUnit)> {
            &self.solid_angle_measure_with_unit
        }
        pub fn solid_angle_unit_holders(&self) -> &HashMap<u64, as_holder!(SolidAngleUnit)> {
            &self.solid_angle_unit
        }
        pub fn solid_model_holders(&self) -> &HashMap<u64, as_holder!(SolidModel)> {
            &self.solid_model
        }
        pub fn specified_higher_usage_occurrence_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(SpecifiedHigherUsageOccurrence)> {
            &self.specified_higher_usage_occurrence
        }
        pub fn spherical_surface_holders(&self) -> &HashMap<u64, as_holder!(SphericalSurface)> {
            &self.spherical_surface
        }
        pub fn start_request_holders(&self) -> &HashMap<u64, as_holder!(StartRequest)> {
            &self.start_request
        }
        pub fn start_work_holders(&self) -> &HashMap<u64, as_holder!(StartWork)> {
            &self.start_work
        }
        pub fn supplied_part_relationship_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(SuppliedPartRelationship)> {
            &self.supplied_part_relationship
        }
        pub fn surface_holders(&self) -> &HashMap<u64, as_holder!(Surface)> {
            &self.surface
        }
        pub fn surface_curve_holders(&self) -> &HashMap<u64, as_holder!(SurfaceCurve)> {
            &self.surface_curve
        }
        pub fn surface_of_linear_extrusion_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(SurfaceOfLinearExtrusion)> {
            &self.surface_of_linear_extrusion
        }
        pub fn surface_of_revolution_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(SurfaceOfRevolution)> {
            &self.surface_of_revolution
        }
        pub fn surface_patch_holders(&self) -> &HashMap<u64, as_holder!(SurfacePatch)> {
            &self.surface_patch
        }
        pub fn surface_replica_holders(&self) -> &HashMap<u64, as_holder!(SurfaceReplica)> {
            &self.surface_replica
        }
        pub fn swept_surface_holders(&self) -> &HashMap<u64, as_holder!(SweptSurface)> {
            &self.swept_surface
        }
        pub fn topological_representation_item_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(TopologicalRepresentationItem)> {
            &self.topological_representation_item
        }
        pub fn toroidal_surface_holders(&self) -> &HashMap<u64, as_holder!(ToroidalSurface)> {
            &self.toroidal_surface
        }
        pub fn trimmed_curve_holders(&self) -> &HashMap<u64, as_holder!(TrimmedCurve)> {
            &self.trimmed_curve
        }
        pub fn uncertainty_measure_with_unit_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(UncertaintyMeasureWithUnit)> {
            &self.uncertainty_measure_with_unit
        }
        pub fn uniform_curve_holders(&self) -> &HashMap<u64, as_holder!(UniformCurve)> {
            &self.uniform_curve
        }
        pub fn uniform_surface_holders(&self) -> &HashMap<u64, as_holder!(UniformSurface)> {
            &self.uniform_surface
        }
        pub fn vector_holders(&self) -> &HashMap<u64, as_holder!(Vector)> {
            &self.vector
        }
        pub fn versioned_action_request_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(VersionedActionRequest)> {
            &self.versioned_action_request
        }
        pub fn vertex_holders(&self) -> &HashMap<u64, as_holder!(Vertex)> {
            &self.vertex
        }
        pub fn vertex_loop_holders(&self) -> &HashMap<u64, as_holder!(VertexLoop)> {
            &self.vertex_loop
        }
        pub fn vertex_point_holders(&self) -> &HashMap<u64, as_holder!(VertexPoint)> {
            &self.vertex_point
        }
        pub fn vertex_shell_holders(&self) -> &HashMap<u64, as_holder!(VertexShell)> {
            &self.vertex_shell
        }
        pub fn volume_measure_with_unit_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(VolumeMeasureWithUnit)> {
            &self.volume_measure_with_unit
        }
        pub fn volume_unit_holders(&self) -> &HashMap<u64, as_holder!(VolumeUnit)> {
            &self.volume_unit
        }
        pub fn week_of_year_and_day_date_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(WeekOfYearAndDayDate)> {
            &self.week_of_year_and_day_date
        }
        pub fn wire_shell_holders(&self) -> &HashMap<u64, as_holder!(WireShell)> {
            &self.wire_shell
        }
        pub fn approved_item_holders(&self) -> &HashMap<u64, as_holder!(ApprovedItem)> {
            &self.approved_item
        }
        pub fn area_measure_holders(&self) -> &HashMap<u64, as_holder!(AreaMeasure)> {
            &self.area_measure
        }
        pub fn axis2_placement_holders(&self) -> &HashMap<u64, as_holder!(Axis2Placement)> {
            &self.axis2_placement
        }
        pub fn boolean_operand_holders(&self) -> &HashMap<u64, as_holder!(BooleanOperand)> {
            &self.boolean_operand
        }
        pub fn certified_item_holders(&self) -> &HashMap<u64, as_holder!(CertifiedItem)> {
            &self.certified_item
        }
        pub fn change_request_item_holders(&self) -> &HashMap<u64, as_holder!(ChangeRequestItem)> {
            &self.change_request_item
        }
        pub fn characterized_definition_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(CharacterizedDefinition)> {
            &self.characterized_definition
        }
        pub fn characterized_product_definition_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(CharacterizedProductDefinition)> {
            &self.characterized_product_definition
        }
        pub fn classified_item_holders(&self) -> &HashMap<u64, as_holder!(ClassifiedItem)> {
            &self.classified_item
        }
        pub fn context_dependent_measure_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(ContextDependentMeasure)> {
            &self.context_dependent_measure
        }
        pub fn contracted_item_holders(&self) -> &HashMap<u64, as_holder!(ContractedItem)> {
            &self.contracted_item
        }
        pub fn count_measure_holders(&self) -> &HashMap<u64, as_holder!(CountMeasure)> {
            &self.count_measure
        }
        pub fn curve_on_surface_holders(&self) -> &HashMap<u64, as_holder!(CurveOnSurface)> {
            &self.curve_on_surface
        }
        pub fn date_time_item_holders(&self) -> &HashMap<u64, as_holder!(DateTimeItem)> {
            &self.date_time_item
        }
        pub fn date_time_select_holders(&self) -> &HashMap<u64, as_holder!(DateTimeSelect)> {
            &self.date_time_select
        }
        pub fn day_in_month_number_holders(&self) -> &HashMap<u64, as_holder!(DayInMonthNumber)> {
            &self.day_in_month_number
        }
        pub fn day_in_week_number_holders(&self) -> &HashMap<u64, as_holder!(DayInWeekNumber)> {
            &self.day_in_week_number
        }
        pub fn day_in_year_number_holders(&self) -> &HashMap<u64, as_holder!(DayInYearNumber)> {
            &self.day_in_year_number
        }
        pub fn descriptive_measure_holders(&self) -> &HashMap<u64, as_holder!(DescriptiveMeasure)> {
            &self.descriptive_measure
        }
        pub fn dimension_count_holders(&self) -> &HashMap<u64, as_holder!(DimensionCount)> {
            &self.dimension_count
        }
        pub fn founded_item_select_holders(&self) -> &HashMap<u64, as_holder!(FoundedItemSelect)> {
            &self.founded_item_select
        }
        pub fn geometric_set_select_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(GeometricSetSelect)> {
            &self.geometric_set_select
        }
        pub fn hour_in_day_holders(&self) -> &HashMap<u64, as_holder!(HourInDay)> {
            &self.hour_in_day
        }
        pub fn identifier_holders(&self) -> &HashMap<u64, as_holder!(Identifier)> {
            &self.identifier
        }
        pub fn label_holders(&self) -> &HashMap<u64, as_holder!(Label)> {
            &self.label
        }
        pub fn length_measure_holders(&self) -> &HashMap<u64, as_holder!(LengthMeasure)> {
            &self.length_measure
        }
        pub fn list_of_reversible_topology_item_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(ListOfReversibleTopologyItem)> {
            &self.list_of_reversible_topology_item
        }
        pub fn mass_measure_holders(&self) -> &HashMap<u64, as_holder!(MassMeasure)> {
            &self.mass_measure
        }
        pub fn measure_value_holders(&self) -> &HashMap<u64, as_holder!(MeasureValue)> {
            &self.measure_value
        }
        pub fn minute_in_hour_holders(&self) -> &HashMap<u64, as_holder!(MinuteInHour)> {
            &self.minute_in_hour
        }
        pub fn month_in_year_number_holders(&self) -> &HashMap<u64, as_holder!(MonthInYearNumber)> {
            &self.month_in_year_number
        }
        pub fn parameter_value_holders(&self) -> &HashMap<u64, as_holder!(ParameterValue)> {
            &self.parameter_value
        }
        pub fn pcurve_or_surface_holders(&self) -> &HashMap<u64, as_holder!(PcurveOrSurface)> {
            &self.pcurve_or_surface
        }
        pub fn person_organization_item_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(PersonOrganizationItem)> {
            &self.person_organization_item
        }
        pub fn person_organization_select_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(PersonOrganizationSelect)> {
            &self.person_organization_select
        }
        pub fn plane_angle_measure_holders(&self) -> &HashMap<u64, as_holder!(PlaneAngleMeasure)> {
            &self.plane_angle_measure
        }
        pub fn positive_length_measure_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(PositiveLengthMeasure)> {
            &self.positive_length_measure
        }
        pub fn positive_plane_angle_measure_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(PositivePlaneAngleMeasure)> {
            &self.positive_plane_angle_measure
        }
        pub fn reversible_topology_holders(&self) -> &HashMap<u64, as_holder!(ReversibleTopology)> {
            &self.reversible_topology
        }
        pub fn reversible_topology_item_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(ReversibleTopologyItem)> {
            &self.reversible_topology_item
        }
        pub fn second_in_minute_holders(&self) -> &HashMap<u64, as_holder!(SecondInMinute)> {
            &self.second_in_minute
        }
        pub fn set_of_reversible_topology_item_holders(
            &self,
        ) -> &HashMap<u64, as_holder!(SetOfReversibleTopologyItem)> {
            &self.set_of_reversible_topology_item
        }
        pub fn shape_definition_holders(&self) -> &HashMap<u64, as_holder!(ShapeDefinition)> {
            &self.shape_definition
        }
        pub fn shell_holders(&self) -> &HashMap<u64, as_holder!(Shell)> {
            &self.shell
        }
        pub fn solid_angle_measure_holders(&self) -> &HashMap<u64, as_holder!(SolidAngleMeasure)> {
            &self.solid_angle_measure
        }
        pub fn specified_item_holders(&self) -> &HashMap<u64, as_holder!(SpecifiedItem)> {
            &self.specified_item
        }
        pub fn start_request_item_holders(&self) -> &HashMap<u64, as_holder!(StartRequestItem)> {
            &self.start_request_item
        }
        pub fn supported_item_holders(&self) -> &HashMap<u64, as_holder!(SupportedItem)> {
            &self.supported_item
        }
        pub fn surface_model_holders(&self) -> &HashMap<u64, as_holder!(SurfaceModel)> {
            &self.surface_model
        }
        pub fn text_holders(&self) -> &HashMap<u64, as_holder!(Text)> {
            &self.text
        }
        pub fn transformation_holders(&self) -> &HashMap<u64, as_holder!(Transformation)> {
            &self.transformation
        }
        pub fn trimming_select_holders(&self) -> &HashMap<u64, as_holder!(TrimmingSelect)> {
            &self.trimming_select
        }
        pub fn unit_holders(&self) -> &HashMap<u64, as_holder!(Unit)> {
            &self.unit
        }
        pub fn vector_or_direction_holders(&self) -> &HashMap<u64, as_holder!(VectorOrDirection)> {
            &self.vector_or_direction
        }
        pub fn volume_measure_holders(&self) -> &HashMap<u64, as_holder!(VolumeMeasure)> {
            &self.volume_measure
        }
        pub fn week_in_year_number_holders(&self) -> &HashMap<u64, as_holder!(WeekInYearNumber)> {
            &self.week_in_year_number
        }
        pub fn wireframe_model_holders(&self) -> &HashMap<u64, as_holder!(WireframeModel)> {
            &self.wireframe_model
        }
        pub fn work_item_holders(&self) -> &HashMap<u64, as_holder!(WorkItem)> {
            &self.work_item
        }
        pub fn year_number_holders(&self) -> &HashMap<u64, as_holder!(YearNumber)> {
            &self.year_number
        }
    }
    #[derive(Debug, Clone, PartialEq, :: serde :: Deserialize)]
    pub enum AheadOrBehind {
        Ahead,
        Behind,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ApprovedItem {
        #[holder(use_place_holder)]
        ProductDefinitionFormation(ProductDefinitionFormationAny),
        #[holder(use_place_holder)]
        ProductDefinition(ProductDefinitionAny),
        #[holder(use_place_holder)]
        ConfigurationEffectivity(Box<ConfigurationEffectivity>),
        #[holder(use_place_holder)]
        ConfigurationItem(Box<ConfigurationItem>),
        #[holder(use_place_holder)]
        SecurityClassification(Box<SecurityClassification>),
        #[holder(use_place_holder)]
        ChangeRequest(Box<ChangeRequest>),
        #[holder(use_place_holder)]
        Change(Box<Change>),
        #[holder(use_place_holder)]
        StartRequest(Box<StartRequest>),
        #[holder(use_place_holder)]
        StartWork(Box<StartWork>),
        #[holder(use_place_holder)]
        Certification(Box<Certification>),
        #[holder(use_place_holder)]
        Contract(Box<Contract>),
    }
    #[derive(
        Clone, Debug, PartialEq, AsRef, Deref, DerefMut, Into, From, :: ruststep_derive :: Holder,
    )]
    # [holder (table = Tables)]
    # [holder (field = area_measure)]
    #[holder(generate_deserialize)]
    pub struct AreaMeasure(pub f64);
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum Axis2Placement {
        #[holder(use_place_holder)]
        Axis2Placement2D(Box<Axis2Placement2D>),
        #[holder(use_place_holder)]
        Axis2Placement3D(Box<Axis2Placement3D>),
    }
    #[derive(Debug, Clone, PartialEq, :: serde :: Deserialize)]
    pub enum BSplineCurveForm {
        PolylineForm,
        CircularArc,
        EllipticArc,
        ParabolicArc,
        HyperbolicArc,
        Unspecified,
    }
    #[derive(Debug, Clone, PartialEq, :: serde :: Deserialize)]
    pub enum BSplineSurfaceForm {
        PlaneSurf,
        CylindricalSurf,
        ConicalSurf,
        SphericalSurf,
        ToroidalSurf,
        SurfOfRevolution,
        RuledSurf,
        GeneralisedCone,
        QuadricSurf,
        SurfOfLinearExtrusion,
        Unspecified,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum BooleanOperand {
        #[holder(use_place_holder)]
        SolidModel(SolidModelAny),
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum CertifiedItem {
        #[holder(use_place_holder)]
        SuppliedPartRelationship(Box<SuppliedPartRelationship>),
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ChangeRequestItem {
        #[holder(use_place_holder)]
        ProductDefinitionFormation(ProductDefinitionFormationAny),
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
        #[holder(use_place_holder)]
        ProductDefinition(ProductDefinitionAny),
        #[holder(use_place_holder)]
        ProductDefinitionRelationship(ProductDefinitionRelationshipAny),
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ClassifiedItem {
        #[holder(use_place_holder)]
        ProductDefinitionFormation(ProductDefinitionFormationAny),
        #[holder(use_place_holder)]
        AssemblyComponentUsage(AssemblyComponentUsageAny),
    }
    #[derive(
        Clone, Debug, PartialEq, AsRef, Deref, DerefMut, Into, From, :: ruststep_derive :: Holder,
    )]
    # [holder (table = Tables)]
    # [holder (field = context_dependent_measure)]
    #[holder(generate_deserialize)]
    pub struct ContextDependentMeasure(pub f64);
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ContractedItem {
        #[holder(use_place_holder)]
        ProductDefinitionFormation(ProductDefinitionFormationAny),
    }
    #[derive(
        Clone, Debug, PartialEq, AsRef, Deref, DerefMut, Into, From, :: ruststep_derive :: Holder,
    )]
    # [holder (table = Tables)]
    # [holder (field = count_measure)]
    #[holder(generate_deserialize)]
    pub struct CountMeasure(pub f64);
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum CurveOnSurface {
        #[holder(use_place_holder)]
        Pcurve(PcurveAny),
        #[holder(use_place_holder)]
        SurfaceCurve(SurfaceCurveAny),
        #[holder(use_place_holder)]
        CompositeCurveOnSurface(CompositeCurveOnSurfaceAny),
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum DateTimeItem {
        #[holder(use_place_holder)]
        ProductDefinition(ProductDefinitionAny),
        #[holder(use_place_holder)]
        ChangeRequest(Box<ChangeRequest>),
        #[holder(use_place_holder)]
        StartRequest(Box<StartRequest>),
        #[holder(use_place_holder)]
        Change(Box<Change>),
        #[holder(use_place_holder)]
        StartWork(Box<StartWork>),
        #[holder(use_place_holder)]
        ApprovalPersonOrganization(Box<ApprovalPersonOrganization>),
        #[holder(use_place_holder)]
        Contract(Box<Contract>),
        #[holder(use_place_holder)]
        SecurityClassification(Box<SecurityClassification>),
        #[holder(use_place_holder)]
        Certification(Box<Certification>),
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum DateTimeSelect {
        #[holder(use_place_holder)]
        Date(DateAny),
        #[holder(use_place_holder)]
        LocalTime(Box<LocalTime>),
        #[holder(use_place_holder)]
        DateAndTime(Box<DateAndTime>),
    }
    #[derive(
        Clone, Debug, PartialEq, AsRef, Deref, DerefMut, Into, From, :: ruststep_derive :: Holder,
    )]
    # [holder (table = Tables)]
    # [holder (field = day_in_month_number)]
    #[holder(generate_deserialize)]
    pub struct DayInMonthNumber(pub i64);
    #[derive(
        Clone, Debug, PartialEq, AsRef, Deref, DerefMut, Into, From, :: ruststep_derive :: Holder,
    )]
    # [holder (table = Tables)]
    # [holder (field = day_in_week_number)]
    #[holder(generate_deserialize)]
    pub struct DayInWeekNumber(pub i64);
    #[derive(
        Clone, Debug, PartialEq, AsRef, Deref, DerefMut, Into, From, :: ruststep_derive :: Holder,
    )]
    # [holder (table = Tables)]
    # [holder (field = day_in_year_number)]
    #[holder(generate_deserialize)]
    pub struct DayInYearNumber(pub i64);
    #[derive(
        Clone, Debug, PartialEq, AsRef, Deref, DerefMut, Into, From, :: ruststep_derive :: Holder,
    )]
    # [holder (table = Tables)]
    # [holder (field = descriptive_measure)]
    #[holder(generate_deserialize)]
    pub struct DescriptiveMeasure(pub String);
    #[derive(
        Clone, Debug, PartialEq, AsRef, Deref, DerefMut, Into, From, :: ruststep_derive :: Holder,
    )]
    # [holder (table = Tables)]
    # [holder (field = dimension_count)]
    #[holder(generate_deserialize)]
    pub struct DimensionCount(pub i64);
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum FoundedItemSelect {
        #[holder(use_place_holder)]
        FoundedItem(FoundedItemAny),
        #[holder(use_place_holder)]
        RepresentationItem(RepresentationItemAny),
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum GeometricSetSelect {
        #[holder(use_place_holder)]
        Point(PointAny),
        #[holder(use_place_holder)]
        Curve(CurveAny),
        #[holder(use_place_holder)]
        Surface(SurfaceAny),
    }
    #[derive(
        Clone, Debug, PartialEq, AsRef, Deref, DerefMut, Into, From, :: ruststep_derive :: Holder,
    )]
    # [holder (table = Tables)]
    # [holder (field = hour_in_day)]
    #[holder(generate_deserialize)]
    pub struct HourInDay(pub i64);
    #[derive(
        Clone, Debug, PartialEq, AsRef, Deref, DerefMut, Into, From, :: ruststep_derive :: Holder,
    )]
    # [holder (table = Tables)]
    # [holder (field = identifier)]
    #[holder(generate_deserialize)]
    pub struct Identifier(pub String);
    #[derive(Debug, Clone, PartialEq, :: serde :: Deserialize)]
    pub enum KnotType {
        UniformKnots,
        Unspecified,
        QuasiUniformKnots,
        PiecewiseBezierKnots,
    }
    #[derive(
        Clone, Debug, PartialEq, AsRef, Deref, DerefMut, Into, From, :: ruststep_derive :: Holder,
    )]
    # [holder (table = Tables)]
    # [holder (field = label)]
    #[holder(generate_deserialize)]
    pub struct Label(pub String);
    #[derive(
        Clone, Debug, PartialEq, AsRef, Deref, DerefMut, Into, From, :: ruststep_derive :: Holder,
    )]
    # [holder (table = Tables)]
    # [holder (field = length_measure)]
    #[holder(generate_deserialize)]
    pub struct LengthMeasure(pub f64);
    #[derive(
        Clone, Debug, PartialEq, AsRef, Deref, DerefMut, Into, From, :: ruststep_derive :: Holder,
    )]
    # [holder (table = Tables)]
    # [holder (field = list_of_reversible_topology_item)]
    #[holder(generate_deserialize)]
    pub struct ListOfReversibleTopologyItem(
        #[holder(use_place_holder)] pub Vec<ReversibleTopologyItem>,
    );
    #[derive(
        Clone, Debug, PartialEq, AsRef, Deref, DerefMut, Into, From, :: ruststep_derive :: Holder,
    )]
    # [holder (table = Tables)]
    # [holder (field = mass_measure)]
    #[holder(generate_deserialize)]
    pub struct MassMeasure(pub f64);
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum MeasureValue {
        #[holder(use_place_holder)]
        LengthMeasure(Box<LengthMeasure>),
        #[holder(use_place_holder)]
        MassMeasure(Box<MassMeasure>),
        #[holder(use_place_holder)]
        PlaneAngleMeasure(Box<PlaneAngleMeasure>),
        #[holder(use_place_holder)]
        SolidAngleMeasure(Box<SolidAngleMeasure>),
        #[holder(use_place_holder)]
        AreaMeasure(Box<AreaMeasure>),
        #[holder(use_place_holder)]
        VolumeMeasure(Box<VolumeMeasure>),
        #[holder(use_place_holder)]
        ParameterValue(Box<ParameterValue>),
        #[holder(use_place_holder)]
        ContextDependentMeasure(Box<ContextDependentMeasure>),
        #[holder(use_place_holder)]
        DescriptiveMeasure(Box<DescriptiveMeasure>),
        #[holder(use_place_holder)]
        PositiveLengthMeasure(Box<PositiveLengthMeasure>),
        #[holder(use_place_holder)]
        PositivePlaneAngleMeasure(Box<PositivePlaneAngleMeasure>),
        #[holder(use_place_holder)]
        CountMeasure(Box<CountMeasure>),
    }
    #[derive(
        Clone, Debug, PartialEq, AsRef, Deref, DerefMut, Into, From, :: ruststep_derive :: Holder,
    )]
    # [holder (table = Tables)]
    # [holder (field = minute_in_hour)]
    #[holder(generate_deserialize)]
    pub struct MinuteInHour(pub i64);
    #[derive(
        Clone, Debug, PartialEq, AsRef, Deref, DerefMut, Into, From, :: ruststep_derive :: Holder,
    )]
    # [holder (table = Tables)]
    # [holder (field = month_in_year_number)]
    #[holder(generate_deserialize)]
    pub struct MonthInYearNumber(pub i64);
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
    pub enum PcurveOrSurface {
        #[holder(use_place_holder)]
        Pcurve(PcurveAny),
        #[holder(use_place_holder)]
        Surface(SurfaceAny),
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum PersonOrganizationItem {
        #[holder(use_place_holder)]
        Change(Box<Change>),
        #[holder(use_place_holder)]
        StartWork(Box<StartWork>),
        #[holder(use_place_holder)]
        ChangeRequest(Box<ChangeRequest>),
        #[holder(use_place_holder)]
        StartRequest(Box<StartRequest>),
        #[holder(use_place_holder)]
        ConfigurationItem(Box<ConfigurationItem>),
        #[holder(use_place_holder)]
        Product(Box<Product>),
        #[holder(use_place_holder)]
        ProductDefinitionFormation(ProductDefinitionFormationAny),
        #[holder(use_place_holder)]
        ProductDefinition(ProductDefinitionAny),
        #[holder(use_place_holder)]
        Contract(Box<Contract>),
        #[holder(use_place_holder)]
        SecurityClassification(Box<SecurityClassification>),
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum PersonOrganizationSelect {
        #[holder(use_place_holder)]
        Person(Box<Person>),
        #[holder(use_place_holder)]
        Organization(Box<Organization>),
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
    # [holder (field = positive_plane_angle_measure)]
    #[holder(generate_deserialize)]
    pub struct PositivePlaneAngleMeasure(#[holder(use_place_holder)] pub PlaneAngleMeasure);
    #[derive(Debug, Clone, PartialEq, :: serde :: Deserialize)]
    pub enum PreferredSurfaceCurveRepresentation {
        Curve3D,
        PcurveS1,
        PcurveS2,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ReversibleTopology {
        #[holder(use_place_holder)]
        ReversibleTopologyItem(Box<ReversibleTopologyItem>),
        #[holder(use_place_holder)]
        ListOfReversibleTopologyItem(Box<ListOfReversibleTopologyItem>),
        #[holder(use_place_holder)]
        SetOfReversibleTopologyItem(Box<SetOfReversibleTopologyItem>),
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ReversibleTopologyItem {
        #[holder(use_place_holder)]
        Edge(EdgeAny),
        #[holder(use_place_holder)]
        Path(PathAny),
        #[holder(use_place_holder)]
        Face(FaceAny),
        #[holder(use_place_holder)]
        FaceBound(FaceBoundAny),
        #[holder(use_place_holder)]
        ClosedShell(ClosedShellAny),
        #[holder(use_place_holder)]
        OpenShell(OpenShellAny),
    }
    #[derive(
        Clone, Debug, PartialEq, AsRef, Deref, DerefMut, Into, From, :: ruststep_derive :: Holder,
    )]
    # [holder (table = Tables)]
    # [holder (field = second_in_minute)]
    #[holder(generate_deserialize)]
    pub struct SecondInMinute(pub f64);
    #[derive(
        Clone, Debug, PartialEq, AsRef, Deref, DerefMut, Into, From, :: ruststep_derive :: Holder,
    )]
    # [holder (table = Tables)]
    # [holder (field = set_of_reversible_topology_item)]
    #[holder(generate_deserialize)]
    pub struct SetOfReversibleTopologyItem(
        #[holder(use_place_holder)] pub Vec<ReversibleTopologyItem>,
    );
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ShapeDefinition {
        #[holder(use_place_holder)]
        ProductDefinitionShape(Box<ProductDefinitionShape>),
        #[holder(use_place_holder)]
        ShapeAspect(Box<ShapeAspect>),
        #[holder(use_place_holder)]
        ShapeAspectRelationship(Box<ShapeAspectRelationship>),
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum Shell {
        #[holder(use_place_holder)]
        VertexShell(Box<VertexShell>),
        #[holder(use_place_holder)]
        WireShell(Box<WireShell>),
        #[holder(use_place_holder)]
        OpenShell(OpenShellAny),
        #[holder(use_place_holder)]
        ClosedShell(ClosedShellAny),
    }
    #[derive(Debug, Clone, PartialEq, :: serde :: Deserialize)]
    pub enum SiPrefix {
        Exa,
        Peta,
        Tera,
        Giga,
        Mega,
        Kilo,
        Hecto,
        Deca,
        Deci,
        Centi,
        Milli,
        Micro,
        Nano,
        Pico,
        Femto,
        Atto,
    }
    #[derive(Debug, Clone, PartialEq, :: serde :: Deserialize)]
    pub enum SiUnitName {
        Metre,
        Gram,
        Second,
        Ampere,
        Kelvin,
        Mole,
        Candela,
        Radian,
        Steradian,
        Hertz,
        Newton,
        Pascal,
        Joule,
        Watt,
        Coulomb,
        Volt,
        Farad,
        Ohm,
        Siemens,
        Weber,
        Tesla,
        Henry,
        DegreeCelsius,
        Lumen,
        Lux,
        Becquerel,
        Gray,
        Sievert,
    }
    #[derive(
        Clone, Debug, PartialEq, AsRef, Deref, DerefMut, Into, From, :: ruststep_derive :: Holder,
    )]
    # [holder (table = Tables)]
    # [holder (field = solid_angle_measure)]
    #[holder(generate_deserialize)]
    pub struct SolidAngleMeasure(pub f64);
    #[derive(Debug, Clone, PartialEq, :: serde :: Deserialize)]
    pub enum Source {
        Made,
        Bought,
        NotKnown,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum SpecifiedItem {
        #[holder(use_place_holder)]
        ProductDefinition(ProductDefinitionAny),
        #[holder(use_place_holder)]
        ShapeAspect(Box<ShapeAspect>),
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum StartRequestItem {
        #[holder(use_place_holder)]
        ProductDefinitionFormation(ProductDefinitionFormationAny),
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum SupportedItem {
        #[holder(use_place_holder)]
        ActionDirective(Box<ActionDirective>),
        #[holder(use_place_holder)]
        Action(ActionAny),
        #[holder(use_place_holder)]
        ActionMethod(Box<ActionMethod>),
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum SurfaceModel {
        #[holder(use_place_holder)]
        ShellBasedSurfaceModel(Box<ShellBasedSurfaceModel>),
    }
    #[derive(
        Clone, Debug, PartialEq, AsRef, Deref, DerefMut, Into, From, :: ruststep_derive :: Holder,
    )]
    # [holder (table = Tables)]
    # [holder (field = text)]
    #[holder(generate_deserialize)]
    pub struct Text(pub String);
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum Transformation {
        #[holder(use_place_holder)]
        ItemDefinedTransformation(Box<ItemDefinedTransformation>),
        #[holder(use_place_holder)]
        FunctionallyDefinedTransformation(FunctionallyDefinedTransformationAny),
    }
    #[derive(Debug, Clone, PartialEq, :: serde :: Deserialize)]
    pub enum TransitionCode {
        Discontinuous,
        Continuous,
        ContSameGradient,
        ContSameGradientSameCurvature,
    }
    #[derive(Debug, Clone, PartialEq, :: serde :: Deserialize)]
    pub enum TrimmingPreference {
        Cartesian,
        Parameter,
        Unspecified,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum TrimmingSelect {
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
        #[holder(use_place_holder)]
        Vector(Box<Vector>),
        #[holder(use_place_holder)]
        Direction(Box<Direction>),
    }
    #[derive(
        Clone, Debug, PartialEq, AsRef, Deref, DerefMut, Into, From, :: ruststep_derive :: Holder,
    )]
    # [holder (table = Tables)]
    # [holder (field = volume_measure)]
    #[holder(generate_deserialize)]
    pub struct VolumeMeasure(pub f64);
    #[derive(
        Clone, Debug, PartialEq, AsRef, Deref, DerefMut, Into, From, :: ruststep_derive :: Holder,
    )]
    # [holder (table = Tables)]
    # [holder (field = week_in_year_number)]
    #[holder(generate_deserialize)]
    pub struct WeekInYearNumber(pub i64);
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum WireframeModel {
        #[holder(use_place_holder)]
        ShellBasedWireframeModel(Box<ShellBasedWireframeModel>),
        #[holder(use_place_holder)]
        EdgeBasedWireframeModel(Box<EdgeBasedWireframeModel>),
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum WorkItem {
        #[holder(use_place_holder)]
        ProductDefinitionFormation(ProductDefinitionFormationAny),
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
    # [holder (field = action)]
    #[holder(generate_deserialize)]
    pub struct Action {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub description: Text,
        #[holder(use_place_holder)]
        pub chosen_method: ActionMethod,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ActionAny {
        #[holder(use_place_holder)]
        Action(Box<Action>),
        #[holder(use_place_holder)]
        ExecutedAction(Box<ExecutedActionAny>),
    }
    impl Into<ActionAny> for Action {
        fn into(self) -> ActionAny {
            ActionAny::Action(Box::new(self))
        }
    }
    impl Into<ActionAny> for ExecutedAction {
        fn into(self) -> ActionAny {
            ActionAny::ExecutedAction(Box::new(self.into()))
        }
    }
    impl AsRef<Action> for ActionAny {
        fn as_ref(&self) -> &Action {
            match self {
                ActionAny::Action(x) => x.as_ref(),
                ActionAny::ExecutedAction(x) => (**x).as_ref(),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = action_assignment)]
    #[holder(generate_deserialize)]
    pub struct ActionAssignment {
        #[holder(use_place_holder)]
        pub assigned_action: ActionAny,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ActionAssignmentAny {
        #[holder(use_place_holder)]
        ActionAssignment(Box<ActionAssignment>),
        #[holder(use_place_holder)]
        Change(Box<Change>),
        #[holder(use_place_holder)]
        StartWork(Box<StartWork>),
    }
    impl Into<ActionAssignmentAny> for ActionAssignment {
        fn into(self) -> ActionAssignmentAny {
            ActionAssignmentAny::ActionAssignment(Box::new(self))
        }
    }
    impl Into<ActionAssignmentAny> for Change {
        fn into(self) -> ActionAssignmentAny {
            ActionAssignmentAny::Change(Box::new(self.into()))
        }
    }
    impl Into<ActionAssignmentAny> for StartWork {
        fn into(self) -> ActionAssignmentAny {
            ActionAssignmentAny::StartWork(Box::new(self.into()))
        }
    }
    impl AsRef<ActionAssignment> for ActionAssignmentAny {
        fn as_ref(&self) -> &ActionAssignment {
            match self {
                ActionAssignmentAny::ActionAssignment(x) => x.as_ref(),
                ActionAssignmentAny::Change(x) => (**x).as_ref(),
                ActionAssignmentAny::StartWork(x) => (**x).as_ref(),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = action_directive)]
    #[holder(generate_deserialize)]
    pub struct ActionDirective {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub description: Text,
        #[holder(use_place_holder)]
        pub analysis: Text,
        #[holder(use_place_holder)]
        pub comment: Text,
        #[holder(use_place_holder)]
        pub requests: Vec<VersionedActionRequest>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = action_method)]
    #[holder(generate_deserialize)]
    pub struct ActionMethod {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub description: Text,
        #[holder(use_place_holder)]
        pub consequence: Text,
        #[holder(use_place_holder)]
        pub purpose: Text,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = action_request_assignment)]
    #[holder(generate_deserialize)]
    pub struct ActionRequestAssignment {
        #[holder(use_place_holder)]
        pub assigned_action_request: VersionedActionRequest,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ActionRequestAssignmentAny {
        #[holder(use_place_holder)]
        ActionRequestAssignment(Box<ActionRequestAssignment>),
        #[holder(use_place_holder)]
        ChangeRequest(Box<ChangeRequest>),
        #[holder(use_place_holder)]
        StartRequest(Box<StartRequest>),
    }
    impl Into<ActionRequestAssignmentAny> for ActionRequestAssignment {
        fn into(self) -> ActionRequestAssignmentAny {
            ActionRequestAssignmentAny::ActionRequestAssignment(Box::new(self))
        }
    }
    impl Into<ActionRequestAssignmentAny> for ChangeRequest {
        fn into(self) -> ActionRequestAssignmentAny {
            ActionRequestAssignmentAny::ChangeRequest(Box::new(self.into()))
        }
    }
    impl Into<ActionRequestAssignmentAny> for StartRequest {
        fn into(self) -> ActionRequestAssignmentAny {
            ActionRequestAssignmentAny::StartRequest(Box::new(self.into()))
        }
    }
    impl AsRef<ActionRequestAssignment> for ActionRequestAssignmentAny {
        fn as_ref(&self) -> &ActionRequestAssignment {
            match self {
                ActionRequestAssignmentAny::ActionRequestAssignment(x) => x.as_ref(),
                ActionRequestAssignmentAny::ChangeRequest(x) => (**x).as_ref(),
                ActionRequestAssignmentAny::StartRequest(x) => (**x).as_ref(),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = action_request_solution)]
    #[holder(generate_deserialize)]
    pub struct ActionRequestSolution {
        #[holder(use_place_holder)]
        pub method: ActionMethod,
        #[holder(use_place_holder)]
        pub request: VersionedActionRequest,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = action_request_status)]
    #[holder(generate_deserialize)]
    pub struct ActionRequestStatus {
        #[holder(use_place_holder)]
        pub status: Label,
        #[holder(use_place_holder)]
        pub assigned_request: VersionedActionRequest,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = action_status)]
    #[holder(generate_deserialize)]
    pub struct ActionStatus {
        #[holder(use_place_holder)]
        pub status: Label,
        #[holder(use_place_holder)]
        pub assigned_action: ExecutedActionAny,
    }
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
        Address(Box<Address>),
        #[holder(use_place_holder)]
        OrganizationalAddress(Box<OrganizationalAddress>),
        #[holder(use_place_holder)]
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
    # [holder (field = advanced_brep_shape_representation)]
    #[holder(generate_deserialize)]
    pub struct AdvancedBrepShapeRepresentation {
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
    # [holder (field = advanced_face)]
    #[holder(generate_deserialize)]
    pub struct AdvancedFace {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub face_surface: FaceSurface,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = alternate_product_relationship)]
    #[holder(generate_deserialize)]
    pub struct AlternateProductRelationship {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub definition: Text,
        #[holder(use_place_holder)]
        pub alternate: Product,
        #[holder(use_place_holder)]
        pub base: Product,
        #[holder(use_place_holder)]
        pub basis: Text,
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
        ApplicationContextElement(Box<ApplicationContextElement>),
        #[holder(use_place_holder)]
        ProductConceptContext(Box<ProductConceptContext>),
        #[holder(use_place_holder)]
        ProductContext(Box<ProductContextAny>),
        #[holder(use_place_holder)]
        ProductDefinitionContext(Box<ProductDefinitionContextAny>),
    }
    impl Into<ApplicationContextElementAny> for ApplicationContextElement {
        fn into(self) -> ApplicationContextElementAny {
            ApplicationContextElementAny::ApplicationContextElement(Box::new(self))
        }
    }
    impl Into<ApplicationContextElementAny> for ProductConceptContext {
        fn into(self) -> ApplicationContextElementAny {
            ApplicationContextElementAny::ProductConceptContext(Box::new(self.into()))
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
                ApplicationContextElementAny::ProductConceptContext(x) => (**x).as_ref(),
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
        ApprovalAssignment(Box<ApprovalAssignment>),
        #[holder(use_place_holder)]
        CcDesignApproval(Box<CcDesignApproval>),
    }
    impl Into<ApprovalAssignmentAny> for ApprovalAssignment {
        fn into(self) -> ApprovalAssignmentAny {
            ApprovalAssignmentAny::ApprovalAssignment(Box::new(self))
        }
    }
    impl Into<ApprovalAssignmentAny> for CcDesignApproval {
        fn into(self) -> ApprovalAssignmentAny {
            ApprovalAssignmentAny::CcDesignApproval(Box::new(self.into()))
        }
    }
    impl AsRef<ApprovalAssignment> for ApprovalAssignmentAny {
        fn as_ref(&self) -> &ApprovalAssignment {
            match self {
                ApprovalAssignmentAny::ApprovalAssignment(x) => x.as_ref(),
                ApprovalAssignmentAny::CcDesignApproval(x) => (**x).as_ref(),
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
    # [holder (field = approval_relationship)]
    #[holder(generate_deserialize)]
    pub struct ApprovalRelationship {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub description: Text,
        #[holder(use_place_holder)]
        pub relating_approval: Approval,
        #[holder(use_place_holder)]
        pub related_approval: Approval,
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
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = area_measure_with_unit)]
    #[holder(generate_deserialize)]
    pub struct AreaMeasureWithUnit {
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
    # [holder (field = area_unit)]
    #[holder(generate_deserialize)]
    pub struct AreaUnit {
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
    # [holder (field = assembly_component_usage)]
    #[holder(generate_deserialize)]
    pub struct AssemblyComponentUsage {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub product_definition_usage: ProductDefinitionUsage,
        #[holder(use_place_holder)]
        pub reference_designator: Option<Identifier>,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum AssemblyComponentUsageAny {
        #[holder(use_place_holder)]
        AssemblyComponentUsage(Box<AssemblyComponentUsage>),
        #[holder(use_place_holder)]
        NextAssemblyUsageOccurrence(Box<NextAssemblyUsageOccurrence>),
        #[holder(use_place_holder)]
        PromissoryUsageOccurrence(Box<PromissoryUsageOccurrence>),
        #[holder(use_place_holder)]
        QuantifiedAssemblyComponentUsage(Box<QuantifiedAssemblyComponentUsage>),
        #[holder(use_place_holder)]
        SpecifiedHigherUsageOccurrence(Box<SpecifiedHigherUsageOccurrence>),
    }
    impl Into<AssemblyComponentUsageAny> for AssemblyComponentUsage {
        fn into(self) -> AssemblyComponentUsageAny {
            AssemblyComponentUsageAny::AssemblyComponentUsage(Box::new(self))
        }
    }
    impl Into<AssemblyComponentUsageAny> for NextAssemblyUsageOccurrence {
        fn into(self) -> AssemblyComponentUsageAny {
            AssemblyComponentUsageAny::NextAssemblyUsageOccurrence(Box::new(self.into()))
        }
    }
    impl Into<AssemblyComponentUsageAny> for PromissoryUsageOccurrence {
        fn into(self) -> AssemblyComponentUsageAny {
            AssemblyComponentUsageAny::PromissoryUsageOccurrence(Box::new(self.into()))
        }
    }
    impl Into<AssemblyComponentUsageAny> for QuantifiedAssemblyComponentUsage {
        fn into(self) -> AssemblyComponentUsageAny {
            AssemblyComponentUsageAny::QuantifiedAssemblyComponentUsage(Box::new(self.into()))
        }
    }
    impl Into<AssemblyComponentUsageAny> for SpecifiedHigherUsageOccurrence {
        fn into(self) -> AssemblyComponentUsageAny {
            AssemblyComponentUsageAny::SpecifiedHigherUsageOccurrence(Box::new(self.into()))
        }
    }
    impl AsRef<AssemblyComponentUsage> for AssemblyComponentUsageAny {
        fn as_ref(&self) -> &AssemblyComponentUsage {
            match self {
                AssemblyComponentUsageAny::AssemblyComponentUsage(x) => x.as_ref(),
                AssemblyComponentUsageAny::NextAssemblyUsageOccurrence(x) => (**x).as_ref(),
                AssemblyComponentUsageAny::PromissoryUsageOccurrence(x) => (**x).as_ref(),
                AssemblyComponentUsageAny::QuantifiedAssemblyComponentUsage(x) => (**x).as_ref(),
                AssemblyComponentUsageAny::SpecifiedHigherUsageOccurrence(x) => (**x).as_ref(),
            }
        }
    }
    impl AsRef<ProductDefinitionUsage> for AssemblyComponentUsageAny {
        fn as_ref(&self) -> &ProductDefinitionUsage {
            match self {
                AssemblyComponentUsageAny::AssemblyComponentUsage(x) => {
                    AsRef::<AssemblyComponentUsage>::as_ref(x).as_ref()
                }
                AssemblyComponentUsageAny::NextAssemblyUsageOccurrence(x) => {
                    AsRef::<AssemblyComponentUsage>::as_ref(x.as_ref()).as_ref()
                }
                AssemblyComponentUsageAny::PromissoryUsageOccurrence(x) => {
                    AsRef::<AssemblyComponentUsage>::as_ref(x.as_ref()).as_ref()
                }
                AssemblyComponentUsageAny::QuantifiedAssemblyComponentUsage(x) => {
                    AsRef::<AssemblyComponentUsage>::as_ref(x.as_ref()).as_ref()
                }
                AssemblyComponentUsageAny::SpecifiedHigherUsageOccurrence(x) => {
                    AsRef::<AssemblyComponentUsage>::as_ref(x.as_ref()).as_ref()
                }
            }
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = assembly_component_usage_substitute)]
    #[holder(generate_deserialize)]
    pub struct AssemblyComponentUsageSubstitute {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub definition: Text,
        #[holder(use_place_holder)]
        pub base: AssemblyComponentUsageAny,
        #[holder(use_place_holder)]
        pub substitute: AssemblyComponentUsageAny,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = axis1_placement)]
    #[holder(generate_deserialize)]
    pub struct Axis1Placement {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub placement: Placement,
        #[holder(use_place_holder)]
        pub axis: Option<Direction>,
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
    # [holder (field = axis2_placement_3d)]
    #[holder(generate_deserialize)]
    pub struct Axis2Placement3D {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub placement: Placement,
        #[holder(use_place_holder)]
        pub axis: Option<Direction>,
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
        BSplineCurve(Box<BSplineCurve>),
        #[holder(use_place_holder)]
        BSplineCurveWithKnots(Box<BSplineCurveWithKnots>),
        #[holder(use_place_holder)]
        BezierCurve(Box<BezierCurve>),
        #[holder(use_place_holder)]
        QuasiUniformCurve(Box<QuasiUniformCurve>),
        #[holder(use_place_holder)]
        RationalBSplineCurve(Box<RationalBSplineCurve>),
        #[holder(use_place_holder)]
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
    # [holder (field = b_spline_surface)]
    #[holder(generate_deserialize)]
    pub struct BSplineSurface {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub bounded_surface: BoundedSurface,
        pub u_degree: i64,
        pub v_degree: i64,
        #[holder(use_place_holder)]
        pub control_points_list: Vec<Vec<CartesianPoint>>,
        pub surface_form: BSplineSurfaceForm,
        pub u_closed: Logical,
        pub v_closed: Logical,
        pub self_intersect: Logical,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum BSplineSurfaceAny {
        #[holder(use_place_holder)]
        BSplineSurface(Box<BSplineSurface>),
        #[holder(use_place_holder)]
        BSplineSurfaceWithKnots(Box<BSplineSurfaceWithKnots>),
        #[holder(use_place_holder)]
        BezierSurface(Box<BezierSurface>),
        #[holder(use_place_holder)]
        QuasiUniformSurface(Box<QuasiUniformSurface>),
        #[holder(use_place_holder)]
        RationalBSplineSurface(Box<RationalBSplineSurface>),
        #[holder(use_place_holder)]
        UniformSurface(Box<UniformSurface>),
    }
    impl Into<BSplineSurfaceAny> for BSplineSurface {
        fn into(self) -> BSplineSurfaceAny {
            BSplineSurfaceAny::BSplineSurface(Box::new(self))
        }
    }
    impl Into<BSplineSurfaceAny> for BSplineSurfaceWithKnots {
        fn into(self) -> BSplineSurfaceAny {
            BSplineSurfaceAny::BSplineSurfaceWithKnots(Box::new(self.into()))
        }
    }
    impl Into<BSplineSurfaceAny> for BezierSurface {
        fn into(self) -> BSplineSurfaceAny {
            BSplineSurfaceAny::BezierSurface(Box::new(self.into()))
        }
    }
    impl Into<BSplineSurfaceAny> for QuasiUniformSurface {
        fn into(self) -> BSplineSurfaceAny {
            BSplineSurfaceAny::QuasiUniformSurface(Box::new(self.into()))
        }
    }
    impl Into<BSplineSurfaceAny> for RationalBSplineSurface {
        fn into(self) -> BSplineSurfaceAny {
            BSplineSurfaceAny::RationalBSplineSurface(Box::new(self.into()))
        }
    }
    impl Into<BSplineSurfaceAny> for UniformSurface {
        fn into(self) -> BSplineSurfaceAny {
            BSplineSurfaceAny::UniformSurface(Box::new(self.into()))
        }
    }
    impl AsRef<BSplineSurface> for BSplineSurfaceAny {
        fn as_ref(&self) -> &BSplineSurface {
            match self {
                BSplineSurfaceAny::BSplineSurface(x) => x.as_ref(),
                BSplineSurfaceAny::BSplineSurfaceWithKnots(x) => (**x).as_ref(),
                BSplineSurfaceAny::BezierSurface(x) => (**x).as_ref(),
                BSplineSurfaceAny::QuasiUniformSurface(x) => (**x).as_ref(),
                BSplineSurfaceAny::RationalBSplineSurface(x) => (**x).as_ref(),
                BSplineSurfaceAny::UniformSurface(x) => (**x).as_ref(),
            }
        }
    }
    impl AsRef<BoundedSurface> for BSplineSurfaceAny {
        fn as_ref(&self) -> &BoundedSurface {
            match self {
                BSplineSurfaceAny::BSplineSurface(x) => AsRef::<BSplineSurface>::as_ref(x).as_ref(),
                BSplineSurfaceAny::BSplineSurfaceWithKnots(x) => {
                    AsRef::<BSplineSurface>::as_ref(x.as_ref()).as_ref()
                }
                BSplineSurfaceAny::BezierSurface(x) => {
                    AsRef::<BSplineSurface>::as_ref(x.as_ref()).as_ref()
                }
                BSplineSurfaceAny::QuasiUniformSurface(x) => {
                    AsRef::<BSplineSurface>::as_ref(x.as_ref()).as_ref()
                }
                BSplineSurfaceAny::RationalBSplineSurface(x) => {
                    AsRef::<BSplineSurface>::as_ref(x.as_ref()).as_ref()
                }
                BSplineSurfaceAny::UniformSurface(x) => {
                    AsRef::<BSplineSurface>::as_ref(x.as_ref()).as_ref()
                }
            }
        }
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = b_spline_surface_with_knots)]
    #[holder(generate_deserialize)]
    pub struct BSplineSurfaceWithKnots {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub b_spline_surface: BSplineSurface,
        pub u_multiplicities: Vec<i64>,
        pub v_multiplicities: Vec<i64>,
        #[holder(use_place_holder)]
        pub u_knots: Vec<ParameterValue>,
        #[holder(use_place_holder)]
        pub v_knots: Vec<ParameterValue>,
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
    # [holder (field = bezier_surface)]
    #[holder(generate_deserialize)]
    pub struct BezierSurface {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub b_spline_surface: BSplineSurface,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = boundary_curve)]
    #[holder(generate_deserialize)]
    pub struct BoundaryCurve {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub composite_curve_on_surface: CompositeCurveOnSurface,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum BoundaryCurveAny {
        #[holder(use_place_holder)]
        BoundaryCurve(Box<BoundaryCurve>),
        #[holder(use_place_holder)]
        OuterBoundaryCurve(Box<OuterBoundaryCurve>),
    }
    impl Into<BoundaryCurveAny> for BoundaryCurve {
        fn into(self) -> BoundaryCurveAny {
            BoundaryCurveAny::BoundaryCurve(Box::new(self))
        }
    }
    impl Into<BoundaryCurveAny> for OuterBoundaryCurve {
        fn into(self) -> BoundaryCurveAny {
            BoundaryCurveAny::OuterBoundaryCurve(Box::new(self.into()))
        }
    }
    impl AsRef<BoundaryCurve> for BoundaryCurveAny {
        fn as_ref(&self) -> &BoundaryCurve {
            match self {
                BoundaryCurveAny::BoundaryCurve(x) => x.as_ref(),
                BoundaryCurveAny::OuterBoundaryCurve(x) => (**x).as_ref(),
            }
        }
    }
    impl AsRef<CompositeCurveOnSurface> for BoundaryCurveAny {
        fn as_ref(&self) -> &CompositeCurveOnSurface {
            match self {
                BoundaryCurveAny::BoundaryCurve(x) => AsRef::<BoundaryCurve>::as_ref(x).as_ref(),
                BoundaryCurveAny::OuterBoundaryCurve(x) => {
                    AsRef::<BoundaryCurve>::as_ref(x.as_ref()).as_ref()
                }
            }
        }
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
        BoundedCurve(Box<BoundedCurve>),
        #[holder(use_place_holder)]
        BSplineCurve(Box<BSplineCurveAny>),
        #[holder(use_place_holder)]
        BoundedPcurve(Box<BoundedPcurve>),
        #[holder(use_place_holder)]
        BoundedSurfaceCurve(Box<BoundedSurfaceCurve>),
        #[holder(use_place_holder)]
        CompositeCurve(Box<CompositeCurveAny>),
        #[holder(use_place_holder)]
        Polyline(Box<Polyline>),
        #[holder(use_place_holder)]
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
    impl Into<BoundedCurveAny> for BoundedPcurve {
        fn into(self) -> BoundedCurveAny {
            BoundedCurveAny::BoundedPcurve(Box::new(self.into()))
        }
    }
    impl Into<BoundedCurveAny> for BoundedSurfaceCurve {
        fn into(self) -> BoundedCurveAny {
            BoundedCurveAny::BoundedSurfaceCurve(Box::new(self.into()))
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
                BoundedCurveAny::BoundedPcurve(x) => (**x).as_ref(),
                BoundedCurveAny::BoundedSurfaceCurve(x) => (**x).as_ref(),
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
                BoundedCurveAny::BoundedPcurve(x) => {
                    AsRef::<BoundedCurve>::as_ref(x.as_ref()).as_ref()
                }
                BoundedCurveAny::BoundedSurfaceCurve(x) => {
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
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut)]
    # [holder (table = Tables)]
    # [holder (field = bounded_pcurve)]
    #[holder(generate_deserialize)]
    pub struct BoundedPcurve {
        #[as_ref]
        #[as_mut]
        #[holder(use_place_holder)]
        pub pcurve: Pcurve,
        #[as_ref]
        #[as_mut]
        #[holder(use_place_holder)]
        pub bounded_curve: BoundedCurve,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = bounded_surface)]
    #[holder(generate_deserialize)]
    pub struct BoundedSurface {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub surface: Surface,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum BoundedSurfaceAny {
        #[holder(use_place_holder)]
        BoundedSurface(Box<BoundedSurface>),
        #[holder(use_place_holder)]
        BSplineSurface(Box<BSplineSurfaceAny>),
        #[holder(use_place_holder)]
        CurveBoundedSurface(Box<CurveBoundedSurface>),
        #[holder(use_place_holder)]
        RectangularCompositeSurface(Box<RectangularCompositeSurface>),
        #[holder(use_place_holder)]
        RectangularTrimmedSurface(Box<RectangularTrimmedSurface>),
    }
    impl Into<BoundedSurfaceAny> for BoundedSurface {
        fn into(self) -> BoundedSurfaceAny {
            BoundedSurfaceAny::BoundedSurface(Box::new(self))
        }
    }
    impl Into<BoundedSurfaceAny> for BSplineSurface {
        fn into(self) -> BoundedSurfaceAny {
            BoundedSurfaceAny::BSplineSurface(Box::new(self.into()))
        }
    }
    impl Into<BoundedSurfaceAny> for CurveBoundedSurface {
        fn into(self) -> BoundedSurfaceAny {
            BoundedSurfaceAny::CurveBoundedSurface(Box::new(self.into()))
        }
    }
    impl Into<BoundedSurfaceAny> for RectangularCompositeSurface {
        fn into(self) -> BoundedSurfaceAny {
            BoundedSurfaceAny::RectangularCompositeSurface(Box::new(self.into()))
        }
    }
    impl Into<BoundedSurfaceAny> for RectangularTrimmedSurface {
        fn into(self) -> BoundedSurfaceAny {
            BoundedSurfaceAny::RectangularTrimmedSurface(Box::new(self.into()))
        }
    }
    impl AsRef<BoundedSurface> for BoundedSurfaceAny {
        fn as_ref(&self) -> &BoundedSurface {
            match self {
                BoundedSurfaceAny::BoundedSurface(x) => x.as_ref(),
                BoundedSurfaceAny::BSplineSurface(x) => (**x).as_ref(),
                BoundedSurfaceAny::CurveBoundedSurface(x) => (**x).as_ref(),
                BoundedSurfaceAny::RectangularCompositeSurface(x) => (**x).as_ref(),
                BoundedSurfaceAny::RectangularTrimmedSurface(x) => (**x).as_ref(),
            }
        }
    }
    impl AsRef<Surface> for BoundedSurfaceAny {
        fn as_ref(&self) -> &Surface {
            match self {
                BoundedSurfaceAny::BoundedSurface(x) => AsRef::<BoundedSurface>::as_ref(x).as_ref(),
                BoundedSurfaceAny::BSplineSurface(x) => {
                    AsRef::<BoundedSurface>::as_ref(x.as_ref()).as_ref()
                }
                BoundedSurfaceAny::CurveBoundedSurface(x) => {
                    AsRef::<BoundedSurface>::as_ref(x.as_ref()).as_ref()
                }
                BoundedSurfaceAny::RectangularCompositeSurface(x) => {
                    AsRef::<BoundedSurface>::as_ref(x.as_ref()).as_ref()
                }
                BoundedSurfaceAny::RectangularTrimmedSurface(x) => {
                    AsRef::<BoundedSurface>::as_ref(x.as_ref()).as_ref()
                }
            }
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut)]
    # [holder (table = Tables)]
    # [holder (field = bounded_surface_curve)]
    #[holder(generate_deserialize)]
    pub struct BoundedSurfaceCurve {
        #[as_ref]
        #[as_mut]
        #[holder(use_place_holder)]
        pub surface_curve: SurfaceCurve,
        #[as_ref]
        #[as_mut]
        #[holder(use_place_holder)]
        pub bounded_curve: BoundedCurve,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = brep_with_voids)]
    #[holder(generate_deserialize)]
    pub struct BrepWithVoids {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub manifold_solid_brep: ManifoldSolidBrep,
        #[holder(use_place_holder)]
        pub voids: Vec<OrientedClosedShell>,
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
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut)]
    # [holder (table = Tables)]
    # [holder (field = cartesian_transformation_operator)]
    #[holder(generate_deserialize)]
    pub struct CartesianTransformationOperator {
        #[as_ref]
        #[as_mut]
        #[holder(use_place_holder)]
        pub geometric_representation_item: GeometricRepresentationItem,
        #[as_ref]
        #[as_mut]
        #[holder(use_place_holder)]
        pub functionally_defined_transformation: FunctionallyDefinedTransformation,
        #[holder(use_place_holder)]
        pub axis1: Option<Direction>,
        #[holder(use_place_holder)]
        pub axis2: Option<Direction>,
        #[holder(use_place_holder)]
        pub local_origin: CartesianPoint,
        pub scale: Option<f64>,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum CartesianTransformationOperatorAny {
        #[holder(use_place_holder)]
        CartesianTransformationOperator(Box<CartesianTransformationOperator>),
        #[holder(use_place_holder)]
        CartesianTransformationOperator3D(Box<CartesianTransformationOperator3D>),
    }
    impl Into<CartesianTransformationOperatorAny> for CartesianTransformationOperator {
        fn into(self) -> CartesianTransformationOperatorAny {
            CartesianTransformationOperatorAny::CartesianTransformationOperator(Box::new(self))
        }
    }
    impl Into<CartesianTransformationOperatorAny> for CartesianTransformationOperator3D {
        fn into(self) -> CartesianTransformationOperatorAny {
            CartesianTransformationOperatorAny::CartesianTransformationOperator3D(Box::new(
                self.into(),
            ))
        }
    }
    impl AsRef<CartesianTransformationOperator> for CartesianTransformationOperatorAny {
        fn as_ref(&self) -> &CartesianTransformationOperator {
            match self {
                CartesianTransformationOperatorAny::CartesianTransformationOperator(x) => {
                    x.as_ref()
                }
                CartesianTransformationOperatorAny::CartesianTransformationOperator3D(x) => {
                    (**x).as_ref()
                }
            }
        }
    }
    impl AsRef<GeometricRepresentationItem> for CartesianTransformationOperatorAny {
        fn as_ref(&self) -> &GeometricRepresentationItem {
            match self {
                CartesianTransformationOperatorAny::CartesianTransformationOperator(x) => {
                    AsRef::<CartesianTransformationOperator>::as_ref(x).as_ref()
                }
                CartesianTransformationOperatorAny::CartesianTransformationOperator3D(x) => {
                    AsRef::<CartesianTransformationOperator>::as_ref(x.as_ref()).as_ref()
                }
            }
        }
    }
    impl AsRef<FunctionallyDefinedTransformation> for CartesianTransformationOperatorAny {
        fn as_ref(&self) -> &FunctionallyDefinedTransformation {
            match self {
                CartesianTransformationOperatorAny::CartesianTransformationOperator(x) => {
                    AsRef::<CartesianTransformationOperator>::as_ref(x).as_ref()
                }
                CartesianTransformationOperatorAny::CartesianTransformationOperator3D(x) => {
                    AsRef::<CartesianTransformationOperator>::as_ref(x.as_ref()).as_ref()
                }
            }
        }
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = cartesian_transformation_operator_3d)]
    #[holder(generate_deserialize)]
    pub struct CartesianTransformationOperator3D {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub cartesian_transformation_operator: CartesianTransformationOperator,
        #[holder(use_place_holder)]
        pub axis3: Option<Direction>,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = cc_design_approval)]
    #[holder(generate_deserialize)]
    pub struct CcDesignApproval {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub approval_assignment: ApprovalAssignment,
        #[holder(use_place_holder)]
        pub items: Vec<ApprovedItem>,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = cc_design_certification)]
    #[holder(generate_deserialize)]
    pub struct CcDesignCertification {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub certification_assignment: CertificationAssignment,
        #[holder(use_place_holder)]
        pub items: Vec<CertifiedItem>,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = cc_design_contract)]
    #[holder(generate_deserialize)]
    pub struct CcDesignContract {
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
    # [holder (field = cc_design_date_and_time_assignment)]
    #[holder(generate_deserialize)]
    pub struct CcDesignDateAndTimeAssignment {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub date_and_time_assignment: DateAndTimeAssignment,
        #[holder(use_place_holder)]
        pub items: Vec<DateTimeItem>,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = cc_design_person_and_organization_assignment)]
    #[holder(generate_deserialize)]
    pub struct CcDesignPersonAndOrganizationAssignment {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub person_and_organization_assignment: PersonAndOrganizationAssignment,
        #[holder(use_place_holder)]
        pub items: Vec<PersonOrganizationItem>,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = cc_design_security_classification)]
    #[holder(generate_deserialize)]
    pub struct CcDesignSecurityClassification {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub security_classification_assignment: SecurityClassificationAssignment,
        #[holder(use_place_holder)]
        pub items: Vec<ClassifiedItem>,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = cc_design_specification_reference)]
    #[holder(generate_deserialize)]
    pub struct CcDesignSpecificationReference {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub document_reference: DocumentReference,
        #[holder(use_place_holder)]
        pub items: Vec<SpecifiedItem>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = certification)]
    #[holder(generate_deserialize)]
    pub struct Certification {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub purpose: Text,
        #[holder(use_place_holder)]
        pub kind: CertificationType,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = certification_assignment)]
    #[holder(generate_deserialize)]
    pub struct CertificationAssignment {
        #[holder(use_place_holder)]
        pub assigned_certification: Certification,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum CertificationAssignmentAny {
        #[holder(use_place_holder)]
        CertificationAssignment(Box<CertificationAssignment>),
        #[holder(use_place_holder)]
        CcDesignCertification(Box<CcDesignCertification>),
    }
    impl Into<CertificationAssignmentAny> for CertificationAssignment {
        fn into(self) -> CertificationAssignmentAny {
            CertificationAssignmentAny::CertificationAssignment(Box::new(self))
        }
    }
    impl Into<CertificationAssignmentAny> for CcDesignCertification {
        fn into(self) -> CertificationAssignmentAny {
            CertificationAssignmentAny::CcDesignCertification(Box::new(self.into()))
        }
    }
    impl AsRef<CertificationAssignment> for CertificationAssignmentAny {
        fn as_ref(&self) -> &CertificationAssignment {
            match self {
                CertificationAssignmentAny::CertificationAssignment(x) => x.as_ref(),
                CertificationAssignmentAny::CcDesignCertification(x) => (**x).as_ref(),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = certification_type)]
    #[holder(generate_deserialize)]
    pub struct CertificationType {
        #[holder(use_place_holder)]
        pub description: Label,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = change)]
    #[holder(generate_deserialize)]
    pub struct Change {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub action_assignment: ActionAssignment,
        #[holder(use_place_holder)]
        pub items: Vec<WorkItem>,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = change_request)]
    #[holder(generate_deserialize)]
    pub struct ChangeRequest {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub action_request_assignment: ActionRequestAssignment,
        #[holder(use_place_holder)]
        pub items: Vec<ChangeRequestItem>,
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
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = closed_shell)]
    #[holder(generate_deserialize)]
    pub struct ClosedShell {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub connected_face_set: ConnectedFaceSet,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ClosedShellAny {
        #[holder(use_place_holder)]
        ClosedShell(Box<ClosedShell>),
        #[holder(use_place_holder)]
        OrientedClosedShell(Box<OrientedClosedShell>),
    }
    impl Into<ClosedShellAny> for ClosedShell {
        fn into(self) -> ClosedShellAny {
            ClosedShellAny::ClosedShell(Box::new(self))
        }
    }
    impl Into<ClosedShellAny> for OrientedClosedShell {
        fn into(self) -> ClosedShellAny {
            ClosedShellAny::OrientedClosedShell(Box::new(self.into()))
        }
    }
    impl AsRef<ClosedShell> for ClosedShellAny {
        fn as_ref(&self) -> &ClosedShell {
            match self {
                ClosedShellAny::ClosedShell(x) => x.as_ref(),
                ClosedShellAny::OrientedClosedShell(x) => (**x).as_ref(),
            }
        }
    }
    impl AsRef<ConnectedFaceSet> for ClosedShellAny {
        fn as_ref(&self) -> &ConnectedFaceSet {
            match self {
                ClosedShellAny::ClosedShell(x) => AsRef::<ClosedShell>::as_ref(x).as_ref(),
                ClosedShellAny::OrientedClosedShell(x) => {
                    AsRef::<ClosedShell>::as_ref(x.as_ref()).as_ref()
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
        pub segments: Vec<CompositeCurveSegmentAny>,
        pub self_intersect: Logical,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum CompositeCurveAny {
        #[holder(use_place_holder)]
        CompositeCurve(Box<CompositeCurve>),
        #[holder(use_place_holder)]
        CompositeCurveOnSurface(Box<CompositeCurveOnSurfaceAny>),
    }
    impl Into<CompositeCurveAny> for CompositeCurve {
        fn into(self) -> CompositeCurveAny {
            CompositeCurveAny::CompositeCurve(Box::new(self))
        }
    }
    impl Into<CompositeCurveAny> for CompositeCurveOnSurface {
        fn into(self) -> CompositeCurveAny {
            CompositeCurveAny::CompositeCurveOnSurface(Box::new(self.into()))
        }
    }
    impl AsRef<CompositeCurve> for CompositeCurveAny {
        fn as_ref(&self) -> &CompositeCurve {
            match self {
                CompositeCurveAny::CompositeCurve(x) => x.as_ref(),
                CompositeCurveAny::CompositeCurveOnSurface(x) => (**x).as_ref(),
            }
        }
    }
    impl AsRef<BoundedCurve> for CompositeCurveAny {
        fn as_ref(&self) -> &BoundedCurve {
            match self {
                CompositeCurveAny::CompositeCurve(x) => AsRef::<CompositeCurve>::as_ref(x).as_ref(),
                CompositeCurveAny::CompositeCurveOnSurface(x) => {
                    AsRef::<CompositeCurve>::as_ref(x.as_ref()).as_ref()
                }
            }
        }
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = composite_curve_on_surface)]
    #[holder(generate_deserialize)]
    pub struct CompositeCurveOnSurface {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub composite_curve: CompositeCurve,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum CompositeCurveOnSurfaceAny {
        #[holder(use_place_holder)]
        CompositeCurveOnSurface(Box<CompositeCurveOnSurface>),
        #[holder(use_place_holder)]
        BoundaryCurve(Box<BoundaryCurveAny>),
    }
    impl Into<CompositeCurveOnSurfaceAny> for CompositeCurveOnSurface {
        fn into(self) -> CompositeCurveOnSurfaceAny {
            CompositeCurveOnSurfaceAny::CompositeCurveOnSurface(Box::new(self))
        }
    }
    impl Into<CompositeCurveOnSurfaceAny> for BoundaryCurve {
        fn into(self) -> CompositeCurveOnSurfaceAny {
            CompositeCurveOnSurfaceAny::BoundaryCurve(Box::new(self.into()))
        }
    }
    impl AsRef<CompositeCurveOnSurface> for CompositeCurveOnSurfaceAny {
        fn as_ref(&self) -> &CompositeCurveOnSurface {
            match self {
                CompositeCurveOnSurfaceAny::CompositeCurveOnSurface(x) => x.as_ref(),
                CompositeCurveOnSurfaceAny::BoundaryCurve(x) => (**x).as_ref(),
            }
        }
    }
    impl AsRef<CompositeCurve> for CompositeCurveOnSurfaceAny {
        fn as_ref(&self) -> &CompositeCurve {
            match self {
                CompositeCurveOnSurfaceAny::CompositeCurveOnSurface(x) => {
                    AsRef::<CompositeCurveOnSurface>::as_ref(x).as_ref()
                }
                CompositeCurveOnSurfaceAny::BoundaryCurve(x) => {
                    AsRef::<CompositeCurveOnSurface>::as_ref(x.as_ref()).as_ref()
                }
            }
        }
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = composite_curve_segment)]
    #[holder(generate_deserialize)]
    pub struct CompositeCurveSegment {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub founded_item: FoundedItem,
        pub transition: TransitionCode,
        pub same_sense: bool,
        #[holder(use_place_holder)]
        pub parent_curve: CurveAny,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum CompositeCurveSegmentAny {
        #[holder(use_place_holder)]
        CompositeCurveSegment(Box<CompositeCurveSegment>),
        #[holder(use_place_holder)]
        ReparametrisedCompositeCurveSegment(Box<ReparametrisedCompositeCurveSegment>),
    }
    impl Into<CompositeCurveSegmentAny> for CompositeCurveSegment {
        fn into(self) -> CompositeCurveSegmentAny {
            CompositeCurveSegmentAny::CompositeCurveSegment(Box::new(self))
        }
    }
    impl Into<CompositeCurveSegmentAny> for ReparametrisedCompositeCurveSegment {
        fn into(self) -> CompositeCurveSegmentAny {
            CompositeCurveSegmentAny::ReparametrisedCompositeCurveSegment(Box::new(self.into()))
        }
    }
    impl AsRef<CompositeCurveSegment> for CompositeCurveSegmentAny {
        fn as_ref(&self) -> &CompositeCurveSegment {
            match self {
                CompositeCurveSegmentAny::CompositeCurveSegment(x) => x.as_ref(),
                CompositeCurveSegmentAny::ReparametrisedCompositeCurveSegment(x) => (**x).as_ref(),
            }
        }
    }
    impl AsRef<FoundedItem> for CompositeCurveSegmentAny {
        fn as_ref(&self) -> &FoundedItem {
            match self {
                CompositeCurveSegmentAny::CompositeCurveSegment(x) => {
                    AsRef::<CompositeCurveSegment>::as_ref(x).as_ref()
                }
                CompositeCurveSegmentAny::ReparametrisedCompositeCurveSegment(x) => {
                    AsRef::<CompositeCurveSegment>::as_ref(x.as_ref()).as_ref()
                }
            }
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = configuration_design)]
    #[holder(generate_deserialize)]
    pub struct ConfigurationDesign {
        #[holder(use_place_holder)]
        pub configuration: ConfigurationItem,
        #[holder(use_place_holder)]
        pub design: ProductDefinitionFormationAny,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = configuration_effectivity)]
    #[holder(generate_deserialize)]
    pub struct ConfigurationEffectivity {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub product_definition_effectivity: ProductDefinitionEffectivity,
        #[holder(use_place_holder)]
        pub configuration: ConfigurationDesign,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = configuration_item)]
    #[holder(generate_deserialize)]
    pub struct ConfigurationItem {
        #[holder(use_place_holder)]
        pub id: Identifier,
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub description: Option<Text>,
        #[holder(use_place_holder)]
        pub item_concept: ProductConcept,
        #[holder(use_place_holder)]
        pub purpose: Option<Label>,
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
        Conic(Box<Conic>),
        #[holder(use_place_holder)]
        Circle(Box<Circle>),
        #[holder(use_place_holder)]
        Ellipse(Box<Ellipse>),
        #[holder(use_place_holder)]
        Hyperbola(Box<Hyperbola>),
        #[holder(use_place_holder)]
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
    # [holder (field = conical_surface)]
    #[holder(generate_deserialize)]
    pub struct ConicalSurface {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub elementary_surface: ElementarySurface,
        #[holder(use_place_holder)]
        pub radius: LengthMeasure,
        #[holder(use_place_holder)]
        pub semi_angle: PlaneAngleMeasure,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = connected_edge_set)]
    #[holder(generate_deserialize)]
    pub struct ConnectedEdgeSet {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub topological_representation_item: TopologicalRepresentationItem,
        #[holder(use_place_holder)]
        pub ces_edges: Vec<EdgeAny>,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = connected_face_set)]
    #[holder(generate_deserialize)]
    pub struct ConnectedFaceSet {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub topological_representation_item: TopologicalRepresentationItem,
        #[holder(use_place_holder)]
        pub cfs_faces: Vec<FaceAny>,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ConnectedFaceSetAny {
        #[holder(use_place_holder)]
        ConnectedFaceSet(Box<ConnectedFaceSet>),
        #[holder(use_place_holder)]
        ClosedShell(Box<ClosedShellAny>),
        #[holder(use_place_holder)]
        OpenShell(Box<OpenShellAny>),
    }
    impl Into<ConnectedFaceSetAny> for ConnectedFaceSet {
        fn into(self) -> ConnectedFaceSetAny {
            ConnectedFaceSetAny::ConnectedFaceSet(Box::new(self))
        }
    }
    impl Into<ConnectedFaceSetAny> for ClosedShell {
        fn into(self) -> ConnectedFaceSetAny {
            ConnectedFaceSetAny::ClosedShell(Box::new(self.into()))
        }
    }
    impl Into<ConnectedFaceSetAny> for OpenShell {
        fn into(self) -> ConnectedFaceSetAny {
            ConnectedFaceSetAny::OpenShell(Box::new(self.into()))
        }
    }
    impl AsRef<ConnectedFaceSet> for ConnectedFaceSetAny {
        fn as_ref(&self) -> &ConnectedFaceSet {
            match self {
                ConnectedFaceSetAny::ConnectedFaceSet(x) => x.as_ref(),
                ConnectedFaceSetAny::ClosedShell(x) => (**x).as_ref(),
                ConnectedFaceSetAny::OpenShell(x) => (**x).as_ref(),
            }
        }
    }
    impl AsRef<TopologicalRepresentationItem> for ConnectedFaceSetAny {
        fn as_ref(&self) -> &TopologicalRepresentationItem {
            match self {
                ConnectedFaceSetAny::ConnectedFaceSet(x) => {
                    AsRef::<ConnectedFaceSet>::as_ref(x).as_ref()
                }
                ConnectedFaceSetAny::ClosedShell(x) => {
                    AsRef::<ConnectedFaceSet>::as_ref(x.as_ref()).as_ref()
                }
                ConnectedFaceSetAny::OpenShell(x) => {
                    AsRef::<ConnectedFaceSet>::as_ref(x.as_ref()).as_ref()
                }
            }
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = context_dependent_shape_representation)]
    #[holder(generate_deserialize)]
    pub struct ContextDependentShapeRepresentation {
        #[holder(use_place_holder)]
        pub representation_relation: ShapeRepresentationRelationship,
        #[holder(use_place_holder)]
        pub represented_product_relation: ProductDefinitionShape,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = context_dependent_unit)]
    #[holder(generate_deserialize)]
    pub struct ContextDependentUnit {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub named_unit: NamedUnit,
        #[holder(use_place_holder)]
        pub name: Label,
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
        ContractAssignment(Box<ContractAssignment>),
        #[holder(use_place_holder)]
        CcDesignContract(Box<CcDesignContract>),
    }
    impl Into<ContractAssignmentAny> for ContractAssignment {
        fn into(self) -> ContractAssignmentAny {
            ContractAssignmentAny::ContractAssignment(Box::new(self))
        }
    }
    impl Into<ContractAssignmentAny> for CcDesignContract {
        fn into(self) -> ContractAssignmentAny {
            ContractAssignmentAny::CcDesignContract(Box::new(self.into()))
        }
    }
    impl AsRef<ContractAssignment> for ContractAssignmentAny {
        fn as_ref(&self) -> &ContractAssignment {
            match self {
                ContractAssignmentAny::ContractAssignment(x) => x.as_ref(),
                ContractAssignmentAny::CcDesignContract(x) => (**x).as_ref(),
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
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = coordinated_universal_time_offset)]
    #[holder(generate_deserialize)]
    pub struct CoordinatedUniversalTimeOffset {
        #[holder(use_place_holder)]
        pub hour_offset: HourInDay,
        #[holder(use_place_holder)]
        pub minute_offset: Option<MinuteInHour>,
        pub sense: AheadOrBehind,
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
        Curve(Box<Curve>),
        #[holder(use_place_holder)]
        BoundedCurve(Box<BoundedCurveAny>),
        #[holder(use_place_holder)]
        Conic(Box<ConicAny>),
        #[holder(use_place_holder)]
        CurveReplica(Box<CurveReplica>),
        #[holder(use_place_holder)]
        Line(Box<Line>),
        #[holder(use_place_holder)]
        OffsetCurve3D(Box<OffsetCurve3D>),
        #[holder(use_place_holder)]
        Pcurve(Box<PcurveAny>),
        #[holder(use_place_holder)]
        SurfaceCurve(Box<SurfaceCurveAny>),
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
    impl Into<CurveAny> for CurveReplica {
        fn into(self) -> CurveAny {
            CurveAny::CurveReplica(Box::new(self.into()))
        }
    }
    impl Into<CurveAny> for Line {
        fn into(self) -> CurveAny {
            CurveAny::Line(Box::new(self.into()))
        }
    }
    impl Into<CurveAny> for OffsetCurve3D {
        fn into(self) -> CurveAny {
            CurveAny::OffsetCurve3D(Box::new(self.into()))
        }
    }
    impl Into<CurveAny> for Pcurve {
        fn into(self) -> CurveAny {
            CurveAny::Pcurve(Box::new(self.into()))
        }
    }
    impl Into<CurveAny> for SurfaceCurve {
        fn into(self) -> CurveAny {
            CurveAny::SurfaceCurve(Box::new(self.into()))
        }
    }
    impl AsRef<Curve> for CurveAny {
        fn as_ref(&self) -> &Curve {
            match self {
                CurveAny::Curve(x) => x.as_ref(),
                CurveAny::BoundedCurve(x) => (**x).as_ref(),
                CurveAny::Conic(x) => (**x).as_ref(),
                CurveAny::CurveReplica(x) => (**x).as_ref(),
                CurveAny::Line(x) => (**x).as_ref(),
                CurveAny::OffsetCurve3D(x) => (**x).as_ref(),
                CurveAny::Pcurve(x) => (**x).as_ref(),
                CurveAny::SurfaceCurve(x) => (**x).as_ref(),
            }
        }
    }
    impl AsRef<GeometricRepresentationItem> for CurveAny {
        fn as_ref(&self) -> &GeometricRepresentationItem {
            match self {
                CurveAny::Curve(x) => AsRef::<Curve>::as_ref(x).as_ref(),
                CurveAny::BoundedCurve(x) => AsRef::<Curve>::as_ref(x.as_ref()).as_ref(),
                CurveAny::Conic(x) => AsRef::<Curve>::as_ref(x.as_ref()).as_ref(),
                CurveAny::CurveReplica(x) => AsRef::<Curve>::as_ref(x.as_ref()).as_ref(),
                CurveAny::Line(x) => AsRef::<Curve>::as_ref(x.as_ref()).as_ref(),
                CurveAny::OffsetCurve3D(x) => AsRef::<Curve>::as_ref(x.as_ref()).as_ref(),
                CurveAny::Pcurve(x) => AsRef::<Curve>::as_ref(x.as_ref()).as_ref(),
                CurveAny::SurfaceCurve(x) => AsRef::<Curve>::as_ref(x.as_ref()).as_ref(),
            }
        }
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = curve_bounded_surface)]
    #[holder(generate_deserialize)]
    pub struct CurveBoundedSurface {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub bounded_surface: BoundedSurface,
        #[holder(use_place_holder)]
        pub basis_surface: SurfaceAny,
        #[holder(use_place_holder)]
        pub boundaries: Vec<BoundaryCurveAny>,
        pub implicit_outer: bool,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = curve_replica)]
    #[holder(generate_deserialize)]
    pub struct CurveReplica {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub curve: Curve,
        #[holder(use_place_holder)]
        pub parent_curve: CurveAny,
        #[holder(use_place_holder)]
        pub transformation: CartesianTransformationOperatorAny,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = cylindrical_surface)]
    #[holder(generate_deserialize)]
    pub struct CylindricalSurface {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub elementary_surface: ElementarySurface,
        #[holder(use_place_holder)]
        pub radius: PositiveLengthMeasure,
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
        Date(Box<Date>),
        #[holder(use_place_holder)]
        CalendarDate(Box<CalendarDate>),
        #[holder(use_place_holder)]
        OrdinalDate(Box<OrdinalDate>),
        #[holder(use_place_holder)]
        WeekOfYearAndDayDate(Box<WeekOfYearAndDayDate>),
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
    impl Into<DateAny> for OrdinalDate {
        fn into(self) -> DateAny {
            DateAny::OrdinalDate(Box::new(self.into()))
        }
    }
    impl Into<DateAny> for WeekOfYearAndDayDate {
        fn into(self) -> DateAny {
            DateAny::WeekOfYearAndDayDate(Box::new(self.into()))
        }
    }
    impl AsRef<Date> for DateAny {
        fn as_ref(&self) -> &Date {
            match self {
                DateAny::Date(x) => x.as_ref(),
                DateAny::CalendarDate(x) => (**x).as_ref(),
                DateAny::OrdinalDate(x) => (**x).as_ref(),
                DateAny::WeekOfYearAndDayDate(x) => (**x).as_ref(),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = date_and_time)]
    #[holder(generate_deserialize)]
    pub struct DateAndTime {
        #[holder(use_place_holder)]
        pub date_component: DateAny,
        #[holder(use_place_holder)]
        pub time_component: LocalTime,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = date_and_time_assignment)]
    #[holder(generate_deserialize)]
    pub struct DateAndTimeAssignment {
        #[holder(use_place_holder)]
        pub assigned_date_and_time: DateAndTime,
        #[holder(use_place_holder)]
        pub role: DateTimeRole,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum DateAndTimeAssignmentAny {
        #[holder(use_place_holder)]
        DateAndTimeAssignment(Box<DateAndTimeAssignment>),
        #[holder(use_place_holder)]
        CcDesignDateAndTimeAssignment(Box<CcDesignDateAndTimeAssignment>),
    }
    impl Into<DateAndTimeAssignmentAny> for DateAndTimeAssignment {
        fn into(self) -> DateAndTimeAssignmentAny {
            DateAndTimeAssignmentAny::DateAndTimeAssignment(Box::new(self))
        }
    }
    impl Into<DateAndTimeAssignmentAny> for CcDesignDateAndTimeAssignment {
        fn into(self) -> DateAndTimeAssignmentAny {
            DateAndTimeAssignmentAny::CcDesignDateAndTimeAssignment(Box::new(self.into()))
        }
    }
    impl AsRef<DateAndTimeAssignment> for DateAndTimeAssignmentAny {
        fn as_ref(&self) -> &DateAndTimeAssignment {
            match self {
                DateAndTimeAssignmentAny::DateAndTimeAssignment(x) => x.as_ref(),
                DateAndTimeAssignmentAny::CcDesignDateAndTimeAssignment(x) => (**x).as_ref(),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = date_time_role)]
    #[holder(generate_deserialize)]
    pub struct DateTimeRole {
        #[holder(use_place_holder)]
        pub name: Label,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = dated_effectivity)]
    #[holder(generate_deserialize)]
    pub struct DatedEffectivity {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub effectivity: Effectivity,
        #[holder(use_place_holder)]
        pub effectivity_start_date: DateAndTime,
        #[holder(use_place_holder)]
        pub effectivity_end_date: Option<DateAndTime>,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = definitional_representation)]
    #[holder(generate_deserialize)]
    pub struct DefinitionalRepresentation {
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
    # [holder (field = degenerate_pcurve)]
    #[holder(generate_deserialize)]
    pub struct DegeneratePcurve {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub point: Point,
        #[holder(use_place_holder)]
        pub basis_surface: SurfaceAny,
        #[holder(use_place_holder)]
        pub reference_to_curve: DefinitionalRepresentation,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum DegeneratePcurveAny {
        #[holder(use_place_holder)]
        DegeneratePcurve(Box<DegeneratePcurve>),
        #[holder(use_place_holder)]
        EvaluatedDegeneratePcurve(Box<EvaluatedDegeneratePcurve>),
    }
    impl Into<DegeneratePcurveAny> for DegeneratePcurve {
        fn into(self) -> DegeneratePcurveAny {
            DegeneratePcurveAny::DegeneratePcurve(Box::new(self))
        }
    }
    impl Into<DegeneratePcurveAny> for EvaluatedDegeneratePcurve {
        fn into(self) -> DegeneratePcurveAny {
            DegeneratePcurveAny::EvaluatedDegeneratePcurve(Box::new(self.into()))
        }
    }
    impl AsRef<DegeneratePcurve> for DegeneratePcurveAny {
        fn as_ref(&self) -> &DegeneratePcurve {
            match self {
                DegeneratePcurveAny::DegeneratePcurve(x) => x.as_ref(),
                DegeneratePcurveAny::EvaluatedDegeneratePcurve(x) => (**x).as_ref(),
            }
        }
    }
    impl AsRef<Point> for DegeneratePcurveAny {
        fn as_ref(&self) -> &Point {
            match self {
                DegeneratePcurveAny::DegeneratePcurve(x) => {
                    AsRef::<DegeneratePcurve>::as_ref(x).as_ref()
                }
                DegeneratePcurveAny::EvaluatedDegeneratePcurve(x) => {
                    AsRef::<DegeneratePcurve>::as_ref(x.as_ref()).as_ref()
                }
            }
        }
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = degenerate_toroidal_surface)]
    #[holder(generate_deserialize)]
    pub struct DegenerateToroidalSurface {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub toroidal_surface: ToroidalSurface,
        pub select_outer: bool,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = design_context)]
    #[holder(generate_deserialize)]
    pub struct DesignContext {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub product_definition_context: ProductDefinitionContext,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = design_make_from_relationship)]
    #[holder(generate_deserialize)]
    pub struct DesignMakeFromRelationship {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub product_definition_relationship: ProductDefinitionRelationship,
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
    # [holder (field = directed_action)]
    #[holder(generate_deserialize)]
    pub struct DirectedAction {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub executed_action: ExecutedAction,
        #[holder(use_place_holder)]
        pub directive: ActionDirective,
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
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum DocumentAny {
        #[holder(use_place_holder)]
        Document(Box<Document>),
        #[holder(use_place_holder)]
        DocumentWithClass(Box<DocumentWithClass>),
    }
    impl Into<DocumentAny> for Document {
        fn into(self) -> DocumentAny {
            DocumentAny::Document(Box::new(self))
        }
    }
    impl Into<DocumentAny> for DocumentWithClass {
        fn into(self) -> DocumentAny {
            DocumentAny::DocumentWithClass(Box::new(self.into()))
        }
    }
    impl AsRef<Document> for DocumentAny {
        fn as_ref(&self) -> &Document {
            match self {
                DocumentAny::Document(x) => x.as_ref(),
                DocumentAny::DocumentWithClass(x) => (**x).as_ref(),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = document_reference)]
    #[holder(generate_deserialize)]
    pub struct DocumentReference {
        #[holder(use_place_holder)]
        pub assigned_document: DocumentAny,
        #[holder(use_place_holder)]
        pub source: Label,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum DocumentReferenceAny {
        #[holder(use_place_holder)]
        DocumentReference(Box<DocumentReference>),
        #[holder(use_place_holder)]
        CcDesignSpecificationReference(Box<CcDesignSpecificationReference>),
    }
    impl Into<DocumentReferenceAny> for DocumentReference {
        fn into(self) -> DocumentReferenceAny {
            DocumentReferenceAny::DocumentReference(Box::new(self))
        }
    }
    impl Into<DocumentReferenceAny> for CcDesignSpecificationReference {
        fn into(self) -> DocumentReferenceAny {
            DocumentReferenceAny::CcDesignSpecificationReference(Box::new(self.into()))
        }
    }
    impl AsRef<DocumentReference> for DocumentReferenceAny {
        fn as_ref(&self) -> &DocumentReference {
            match self {
                DocumentReferenceAny::DocumentReference(x) => x.as_ref(),
                DocumentReferenceAny::CcDesignSpecificationReference(x) => (**x).as_ref(),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = document_relationship)]
    #[holder(generate_deserialize)]
    pub struct DocumentRelationship {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub description: Text,
        #[holder(use_place_holder)]
        pub relating_document: DocumentAny,
        #[holder(use_place_holder)]
        pub related_document: DocumentAny,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = document_type)]
    #[holder(generate_deserialize)]
    pub struct DocumentType {
        #[holder(use_place_holder)]
        pub product_data_type: Label,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = document_usage_constraint)]
    #[holder(generate_deserialize)]
    pub struct DocumentUsageConstraint {
        #[holder(use_place_holder)]
        pub source: DocumentAny,
        #[holder(use_place_holder)]
        pub subject_element: Label,
        #[holder(use_place_holder)]
        pub subject_element_value: Text,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = document_with_class)]
    #[holder(generate_deserialize)]
    pub struct DocumentWithClass {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub document: Document,
        #[holder(use_place_holder)]
        pub class: Identifier,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = edge)]
    #[holder(generate_deserialize)]
    pub struct Edge {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub topological_representation_item: TopologicalRepresentationItem,
        #[holder(use_place_holder)]
        pub edge_start: VertexAny,
        #[holder(use_place_holder)]
        pub edge_end: VertexAny,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum EdgeAny {
        #[holder(use_place_holder)]
        Edge(Box<Edge>),
        #[holder(use_place_holder)]
        EdgeCurve(Box<EdgeCurve>),
        #[holder(use_place_holder)]
        OrientedEdge(Box<OrientedEdge>),
    }
    impl Into<EdgeAny> for Edge {
        fn into(self) -> EdgeAny {
            EdgeAny::Edge(Box::new(self))
        }
    }
    impl Into<EdgeAny> for EdgeCurve {
        fn into(self) -> EdgeAny {
            EdgeAny::EdgeCurve(Box::new(self.into()))
        }
    }
    impl Into<EdgeAny> for OrientedEdge {
        fn into(self) -> EdgeAny {
            EdgeAny::OrientedEdge(Box::new(self.into()))
        }
    }
    impl AsRef<Edge> for EdgeAny {
        fn as_ref(&self) -> &Edge {
            match self {
                EdgeAny::Edge(x) => x.as_ref(),
                EdgeAny::EdgeCurve(x) => (**x).as_ref(),
                EdgeAny::OrientedEdge(x) => (**x).as_ref(),
            }
        }
    }
    impl AsRef<TopologicalRepresentationItem> for EdgeAny {
        fn as_ref(&self) -> &TopologicalRepresentationItem {
            match self {
                EdgeAny::Edge(x) => AsRef::<Edge>::as_ref(x).as_ref(),
                EdgeAny::EdgeCurve(x) => AsRef::<Edge>::as_ref(x.as_ref()).as_ref(),
                EdgeAny::OrientedEdge(x) => AsRef::<Edge>::as_ref(x.as_ref()).as_ref(),
            }
        }
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = edge_based_wireframe_model)]
    #[holder(generate_deserialize)]
    pub struct EdgeBasedWireframeModel {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub geometric_representation_item: GeometricRepresentationItem,
        #[holder(use_place_holder)]
        pub ebwm_boundary: Vec<ConnectedEdgeSet>,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = edge_based_wireframe_shape_representation)]
    #[holder(generate_deserialize)]
    pub struct EdgeBasedWireframeShapeRepresentation {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub shape_representation: ShapeRepresentation,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut)]
    # [holder (table = Tables)]
    # [holder (field = edge_curve)]
    #[holder(generate_deserialize)]
    pub struct EdgeCurve {
        #[as_ref]
        #[as_mut]
        #[holder(use_place_holder)]
        pub edge: Edge,
        #[as_ref]
        #[as_mut]
        #[holder(use_place_holder)]
        pub geometric_representation_item: GeometricRepresentationItem,
        #[holder(use_place_holder)]
        pub edge_geometry: CurveAny,
        pub same_sense: bool,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut)]
    # [holder (table = Tables)]
    # [holder (field = edge_loop)]
    #[holder(generate_deserialize)]
    pub struct EdgeLoop {
        #[as_ref]
        #[as_mut]
        #[holder(use_place_holder)]
        pub r#loop: Loop,
        #[as_ref]
        #[as_mut]
        #[holder(use_place_holder)]
        pub path: Path,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = effectivity)]
    #[holder(generate_deserialize)]
    pub struct Effectivity {
        #[holder(use_place_holder)]
        pub id: Identifier,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum EffectivityAny {
        #[holder(use_place_holder)]
        Effectivity(Box<Effectivity>),
        #[holder(use_place_holder)]
        DatedEffectivity(Box<DatedEffectivity>),
        #[holder(use_place_holder)]
        LotEffectivity(Box<LotEffectivity>),
        #[holder(use_place_holder)]
        ProductDefinitionEffectivity(Box<ProductDefinitionEffectivityAny>),
        #[holder(use_place_holder)]
        SerialNumberedEffectivity(Box<SerialNumberedEffectivity>),
    }
    impl Into<EffectivityAny> for Effectivity {
        fn into(self) -> EffectivityAny {
            EffectivityAny::Effectivity(Box::new(self))
        }
    }
    impl Into<EffectivityAny> for DatedEffectivity {
        fn into(self) -> EffectivityAny {
            EffectivityAny::DatedEffectivity(Box::new(self.into()))
        }
    }
    impl Into<EffectivityAny> for LotEffectivity {
        fn into(self) -> EffectivityAny {
            EffectivityAny::LotEffectivity(Box::new(self.into()))
        }
    }
    impl Into<EffectivityAny> for ProductDefinitionEffectivity {
        fn into(self) -> EffectivityAny {
            EffectivityAny::ProductDefinitionEffectivity(Box::new(self.into()))
        }
    }
    impl Into<EffectivityAny> for SerialNumberedEffectivity {
        fn into(self) -> EffectivityAny {
            EffectivityAny::SerialNumberedEffectivity(Box::new(self.into()))
        }
    }
    impl AsRef<Effectivity> for EffectivityAny {
        fn as_ref(&self) -> &Effectivity {
            match self {
                EffectivityAny::Effectivity(x) => x.as_ref(),
                EffectivityAny::DatedEffectivity(x) => (**x).as_ref(),
                EffectivityAny::LotEffectivity(x) => (**x).as_ref(),
                EffectivityAny::ProductDefinitionEffectivity(x) => (**x).as_ref(),
                EffectivityAny::SerialNumberedEffectivity(x) => (**x).as_ref(),
            }
        }
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = elementary_surface)]
    #[holder(generate_deserialize)]
    pub struct ElementarySurface {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub surface: Surface,
        #[holder(use_place_holder)]
        pub position: Axis2Placement3D,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ElementarySurfaceAny {
        #[holder(use_place_holder)]
        ElementarySurface(Box<ElementarySurface>),
        #[holder(use_place_holder)]
        ConicalSurface(Box<ConicalSurface>),
        #[holder(use_place_holder)]
        CylindricalSurface(Box<CylindricalSurface>),
        #[holder(use_place_holder)]
        Plane(Box<Plane>),
        #[holder(use_place_holder)]
        SphericalSurface(Box<SphericalSurface>),
        #[holder(use_place_holder)]
        ToroidalSurface(Box<ToroidalSurfaceAny>),
    }
    impl Into<ElementarySurfaceAny> for ElementarySurface {
        fn into(self) -> ElementarySurfaceAny {
            ElementarySurfaceAny::ElementarySurface(Box::new(self))
        }
    }
    impl Into<ElementarySurfaceAny> for ConicalSurface {
        fn into(self) -> ElementarySurfaceAny {
            ElementarySurfaceAny::ConicalSurface(Box::new(self.into()))
        }
    }
    impl Into<ElementarySurfaceAny> for CylindricalSurface {
        fn into(self) -> ElementarySurfaceAny {
            ElementarySurfaceAny::CylindricalSurface(Box::new(self.into()))
        }
    }
    impl Into<ElementarySurfaceAny> for Plane {
        fn into(self) -> ElementarySurfaceAny {
            ElementarySurfaceAny::Plane(Box::new(self.into()))
        }
    }
    impl Into<ElementarySurfaceAny> for SphericalSurface {
        fn into(self) -> ElementarySurfaceAny {
            ElementarySurfaceAny::SphericalSurface(Box::new(self.into()))
        }
    }
    impl Into<ElementarySurfaceAny> for ToroidalSurface {
        fn into(self) -> ElementarySurfaceAny {
            ElementarySurfaceAny::ToroidalSurface(Box::new(self.into()))
        }
    }
    impl AsRef<ElementarySurface> for ElementarySurfaceAny {
        fn as_ref(&self) -> &ElementarySurface {
            match self {
                ElementarySurfaceAny::ElementarySurface(x) => x.as_ref(),
                ElementarySurfaceAny::ConicalSurface(x) => (**x).as_ref(),
                ElementarySurfaceAny::CylindricalSurface(x) => (**x).as_ref(),
                ElementarySurfaceAny::Plane(x) => (**x).as_ref(),
                ElementarySurfaceAny::SphericalSurface(x) => (**x).as_ref(),
                ElementarySurfaceAny::ToroidalSurface(x) => (**x).as_ref(),
            }
        }
    }
    impl AsRef<Surface> for ElementarySurfaceAny {
        fn as_ref(&self) -> &Surface {
            match self {
                ElementarySurfaceAny::ElementarySurface(x) => {
                    AsRef::<ElementarySurface>::as_ref(x).as_ref()
                }
                ElementarySurfaceAny::ConicalSurface(x) => {
                    AsRef::<ElementarySurface>::as_ref(x.as_ref()).as_ref()
                }
                ElementarySurfaceAny::CylindricalSurface(x) => {
                    AsRef::<ElementarySurface>::as_ref(x.as_ref()).as_ref()
                }
                ElementarySurfaceAny::Plane(x) => {
                    AsRef::<ElementarySurface>::as_ref(x.as_ref()).as_ref()
                }
                ElementarySurfaceAny::SphericalSurface(x) => {
                    AsRef::<ElementarySurface>::as_ref(x.as_ref()).as_ref()
                }
                ElementarySurfaceAny::ToroidalSurface(x) => {
                    AsRef::<ElementarySurface>::as_ref(x.as_ref()).as_ref()
                }
            }
        }
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
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = evaluated_degenerate_pcurve)]
    #[holder(generate_deserialize)]
    pub struct EvaluatedDegeneratePcurve {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub degenerate_pcurve: DegeneratePcurve,
        #[holder(use_place_holder)]
        pub equivalent_point: CartesianPoint,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = executed_action)]
    #[holder(generate_deserialize)]
    pub struct ExecutedAction {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub action: Action,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ExecutedActionAny {
        #[holder(use_place_holder)]
        ExecutedAction(Box<ExecutedAction>),
        #[holder(use_place_holder)]
        DirectedAction(Box<DirectedAction>),
    }
    impl Into<ExecutedActionAny> for ExecutedAction {
        fn into(self) -> ExecutedActionAny {
            ExecutedActionAny::ExecutedAction(Box::new(self))
        }
    }
    impl Into<ExecutedActionAny> for DirectedAction {
        fn into(self) -> ExecutedActionAny {
            ExecutedActionAny::DirectedAction(Box::new(self.into()))
        }
    }
    impl AsRef<ExecutedAction> for ExecutedActionAny {
        fn as_ref(&self) -> &ExecutedAction {
            match self {
                ExecutedActionAny::ExecutedAction(x) => x.as_ref(),
                ExecutedActionAny::DirectedAction(x) => (**x).as_ref(),
            }
        }
    }
    impl AsRef<Action> for ExecutedActionAny {
        fn as_ref(&self) -> &Action {
            match self {
                ExecutedActionAny::ExecutedAction(x) => AsRef::<ExecutedAction>::as_ref(x).as_ref(),
                ExecutedActionAny::DirectedAction(x) => {
                    AsRef::<ExecutedAction>::as_ref(x.as_ref()).as_ref()
                }
            }
        }
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = face)]
    #[holder(generate_deserialize)]
    pub struct Face {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub topological_representation_item: TopologicalRepresentationItem,
        #[holder(use_place_holder)]
        pub bounds: Vec<FaceBoundAny>,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum FaceAny {
        #[holder(use_place_holder)]
        Face(Box<Face>),
        #[holder(use_place_holder)]
        FaceSurface(Box<FaceSurfaceAny>),
        #[holder(use_place_holder)]
        OrientedFace(Box<OrientedFace>),
    }
    impl Into<FaceAny> for Face {
        fn into(self) -> FaceAny {
            FaceAny::Face(Box::new(self))
        }
    }
    impl Into<FaceAny> for FaceSurface {
        fn into(self) -> FaceAny {
            FaceAny::FaceSurface(Box::new(self.into()))
        }
    }
    impl Into<FaceAny> for OrientedFace {
        fn into(self) -> FaceAny {
            FaceAny::OrientedFace(Box::new(self.into()))
        }
    }
    impl AsRef<Face> for FaceAny {
        fn as_ref(&self) -> &Face {
            match self {
                FaceAny::Face(x) => x.as_ref(),
                FaceAny::FaceSurface(x) => (**x).as_ref(),
                FaceAny::OrientedFace(x) => (**x).as_ref(),
            }
        }
    }
    impl AsRef<TopologicalRepresentationItem> for FaceAny {
        fn as_ref(&self) -> &TopologicalRepresentationItem {
            match self {
                FaceAny::Face(x) => AsRef::<Face>::as_ref(x).as_ref(),
                FaceAny::FaceSurface(x) => AsRef::<Face>::as_ref(x.as_ref()).as_ref(),
                FaceAny::OrientedFace(x) => AsRef::<Face>::as_ref(x.as_ref()).as_ref(),
            }
        }
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = face_bound)]
    #[holder(generate_deserialize)]
    pub struct FaceBound {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub topological_representation_item: TopologicalRepresentationItem,
        #[holder(use_place_holder)]
        pub bound: LoopAny,
        pub orientation: bool,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum FaceBoundAny {
        #[holder(use_place_holder)]
        FaceBound(Box<FaceBound>),
        #[holder(use_place_holder)]
        FaceOuterBound(Box<FaceOuterBound>),
    }
    impl Into<FaceBoundAny> for FaceBound {
        fn into(self) -> FaceBoundAny {
            FaceBoundAny::FaceBound(Box::new(self))
        }
    }
    impl Into<FaceBoundAny> for FaceOuterBound {
        fn into(self) -> FaceBoundAny {
            FaceBoundAny::FaceOuterBound(Box::new(self.into()))
        }
    }
    impl AsRef<FaceBound> for FaceBoundAny {
        fn as_ref(&self) -> &FaceBound {
            match self {
                FaceBoundAny::FaceBound(x) => x.as_ref(),
                FaceBoundAny::FaceOuterBound(x) => (**x).as_ref(),
            }
        }
    }
    impl AsRef<TopologicalRepresentationItem> for FaceBoundAny {
        fn as_ref(&self) -> &TopologicalRepresentationItem {
            match self {
                FaceBoundAny::FaceBound(x) => AsRef::<FaceBound>::as_ref(x).as_ref(),
                FaceBoundAny::FaceOuterBound(x) => AsRef::<FaceBound>::as_ref(x.as_ref()).as_ref(),
            }
        }
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = face_outer_bound)]
    #[holder(generate_deserialize)]
    pub struct FaceOuterBound {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub face_bound: FaceBound,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut)]
    # [holder (table = Tables)]
    # [holder (field = face_surface)]
    #[holder(generate_deserialize)]
    pub struct FaceSurface {
        #[as_ref]
        #[as_mut]
        #[holder(use_place_holder)]
        pub face: Face,
        #[as_ref]
        #[as_mut]
        #[holder(use_place_holder)]
        pub geometric_representation_item: GeometricRepresentationItem,
        #[holder(use_place_holder)]
        pub face_geometry: SurfaceAny,
        pub same_sense: bool,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum FaceSurfaceAny {
        #[holder(use_place_holder)]
        FaceSurface(Box<FaceSurface>),
        #[holder(use_place_holder)]
        AdvancedFace(Box<AdvancedFace>),
    }
    impl Into<FaceSurfaceAny> for FaceSurface {
        fn into(self) -> FaceSurfaceAny {
            FaceSurfaceAny::FaceSurface(Box::new(self))
        }
    }
    impl Into<FaceSurfaceAny> for AdvancedFace {
        fn into(self) -> FaceSurfaceAny {
            FaceSurfaceAny::AdvancedFace(Box::new(self.into()))
        }
    }
    impl AsRef<FaceSurface> for FaceSurfaceAny {
        fn as_ref(&self) -> &FaceSurface {
            match self {
                FaceSurfaceAny::FaceSurface(x) => x.as_ref(),
                FaceSurfaceAny::AdvancedFace(x) => (**x).as_ref(),
            }
        }
    }
    impl AsRef<Face> for FaceSurfaceAny {
        fn as_ref(&self) -> &Face {
            match self {
                FaceSurfaceAny::FaceSurface(x) => AsRef::<FaceSurface>::as_ref(x).as_ref(),
                FaceSurfaceAny::AdvancedFace(x) => {
                    AsRef::<FaceSurface>::as_ref(x.as_ref()).as_ref()
                }
            }
        }
    }
    impl AsRef<GeometricRepresentationItem> for FaceSurfaceAny {
        fn as_ref(&self) -> &GeometricRepresentationItem {
            match self {
                FaceSurfaceAny::FaceSurface(x) => AsRef::<FaceSurface>::as_ref(x).as_ref(),
                FaceSurfaceAny::AdvancedFace(x) => {
                    AsRef::<FaceSurface>::as_ref(x.as_ref()).as_ref()
                }
            }
        }
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = faceted_brep)]
    #[holder(generate_deserialize)]
    pub struct FacetedBrep {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub manifold_solid_brep: ManifoldSolidBrep,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = faceted_brep_shape_representation)]
    #[holder(generate_deserialize)]
    pub struct FacetedBrepShapeRepresentation {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub shape_representation: ShapeRepresentation,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = founded_item)]
    #[holder(generate_deserialize)]
    pub struct FoundedItem {}
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum FoundedItemAny {
        #[holder(use_place_holder)]
        FoundedItem(Box<FoundedItem>),
        #[holder(use_place_holder)]
        CompositeCurveSegment(Box<CompositeCurveSegmentAny>),
        #[holder(use_place_holder)]
        SurfacePatch(Box<SurfacePatch>),
    }
    impl Into<FoundedItemAny> for FoundedItem {
        fn into(self) -> FoundedItemAny {
            FoundedItemAny::FoundedItem(Box::new(self))
        }
    }
    impl Into<FoundedItemAny> for CompositeCurveSegment {
        fn into(self) -> FoundedItemAny {
            FoundedItemAny::CompositeCurveSegment(Box::new(self.into()))
        }
    }
    impl Into<FoundedItemAny> for SurfacePatch {
        fn into(self) -> FoundedItemAny {
            FoundedItemAny::SurfacePatch(Box::new(self.into()))
        }
    }
    impl AsRef<FoundedItem> for FoundedItemAny {
        fn as_ref(&self) -> &FoundedItem {
            match self {
                FoundedItemAny::FoundedItem(x) => x.as_ref(),
                FoundedItemAny::CompositeCurveSegment(x) => (**x).as_ref(),
                FoundedItemAny::SurfacePatch(x) => (**x).as_ref(),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = functionally_defined_transformation)]
    #[holder(generate_deserialize)]
    pub struct FunctionallyDefinedTransformation {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub description: Text,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum FunctionallyDefinedTransformationAny {
        #[holder(use_place_holder)]
        FunctionallyDefinedTransformation(Box<FunctionallyDefinedTransformation>),
        #[holder(use_place_holder)]
        CartesianTransformationOperator(Box<CartesianTransformationOperatorAny>),
    }
    impl Into<FunctionallyDefinedTransformationAny> for FunctionallyDefinedTransformation {
        fn into(self) -> FunctionallyDefinedTransformationAny {
            FunctionallyDefinedTransformationAny::FunctionallyDefinedTransformation(Box::new(self))
        }
    }
    impl Into<FunctionallyDefinedTransformationAny> for CartesianTransformationOperator {
        fn into(self) -> FunctionallyDefinedTransformationAny {
            FunctionallyDefinedTransformationAny::CartesianTransformationOperator(Box::new(
                self.into(),
            ))
        }
    }
    impl AsRef<FunctionallyDefinedTransformation> for FunctionallyDefinedTransformationAny {
        fn as_ref(&self) -> &FunctionallyDefinedTransformation {
            match self {
                FunctionallyDefinedTransformationAny::FunctionallyDefinedTransformation(x) => {
                    x.as_ref()
                }
                FunctionallyDefinedTransformationAny::CartesianTransformationOperator(x) => {
                    (**x).as_ref()
                }
            }
        }
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
        GeometricRepresentationItem(Box<GeometricRepresentationItem>),
        #[holder(use_place_holder)]
        CartesianTransformationOperator(Box<CartesianTransformationOperatorAny>),
        #[holder(use_place_holder)]
        Curve(Box<CurveAny>),
        #[holder(use_place_holder)]
        Direction(Box<Direction>),
        #[holder(use_place_holder)]
        EdgeBasedWireframeModel(Box<EdgeBasedWireframeModel>),
        #[holder(use_place_holder)]
        EdgeCurve(Box<EdgeCurve>),
        #[holder(use_place_holder)]
        FaceSurface(Box<FaceSurfaceAny>),
        #[holder(use_place_holder)]
        GeometricSet(Box<GeometricSetAny>),
        #[holder(use_place_holder)]
        Placement(Box<PlacementAny>),
        #[holder(use_place_holder)]
        Point(Box<PointAny>),
        #[holder(use_place_holder)]
        PolyLoop(Box<PolyLoop>),
        #[holder(use_place_holder)]
        ShellBasedSurfaceModel(Box<ShellBasedSurfaceModel>),
        #[holder(use_place_holder)]
        ShellBasedWireframeModel(Box<ShellBasedWireframeModel>),
        #[holder(use_place_holder)]
        SolidModel(Box<SolidModelAny>),
        #[holder(use_place_holder)]
        Surface(Box<SurfaceAny>),
        #[holder(use_place_holder)]
        Vector(Box<Vector>),
        #[holder(use_place_holder)]
        VertexPoint(Box<VertexPoint>),
    }
    impl Into<GeometricRepresentationItemAny> for GeometricRepresentationItem {
        fn into(self) -> GeometricRepresentationItemAny {
            GeometricRepresentationItemAny::GeometricRepresentationItem(Box::new(self))
        }
    }
    impl Into<GeometricRepresentationItemAny> for CartesianTransformationOperator {
        fn into(self) -> GeometricRepresentationItemAny {
            GeometricRepresentationItemAny::CartesianTransformationOperator(Box::new(self.into()))
        }
    }
    impl Into<GeometricRepresentationItemAny> for Curve {
        fn into(self) -> GeometricRepresentationItemAny {
            GeometricRepresentationItemAny::Curve(Box::new(self.into()))
        }
    }
    impl Into<GeometricRepresentationItemAny> for Direction {
        fn into(self) -> GeometricRepresentationItemAny {
            GeometricRepresentationItemAny::Direction(Box::new(self.into()))
        }
    }
    impl Into<GeometricRepresentationItemAny> for EdgeBasedWireframeModel {
        fn into(self) -> GeometricRepresentationItemAny {
            GeometricRepresentationItemAny::EdgeBasedWireframeModel(Box::new(self.into()))
        }
    }
    impl Into<GeometricRepresentationItemAny> for EdgeCurve {
        fn into(self) -> GeometricRepresentationItemAny {
            GeometricRepresentationItemAny::EdgeCurve(Box::new(self.into()))
        }
    }
    impl Into<GeometricRepresentationItemAny> for FaceSurface {
        fn into(self) -> GeometricRepresentationItemAny {
            GeometricRepresentationItemAny::FaceSurface(Box::new(self.into()))
        }
    }
    impl Into<GeometricRepresentationItemAny> for GeometricSet {
        fn into(self) -> GeometricRepresentationItemAny {
            GeometricRepresentationItemAny::GeometricSet(Box::new(self.into()))
        }
    }
    impl Into<GeometricRepresentationItemAny> for Placement {
        fn into(self) -> GeometricRepresentationItemAny {
            GeometricRepresentationItemAny::Placement(Box::new(self.into()))
        }
    }
    impl Into<GeometricRepresentationItemAny> for Point {
        fn into(self) -> GeometricRepresentationItemAny {
            GeometricRepresentationItemAny::Point(Box::new(self.into()))
        }
    }
    impl Into<GeometricRepresentationItemAny> for PolyLoop {
        fn into(self) -> GeometricRepresentationItemAny {
            GeometricRepresentationItemAny::PolyLoop(Box::new(self.into()))
        }
    }
    impl Into<GeometricRepresentationItemAny> for ShellBasedSurfaceModel {
        fn into(self) -> GeometricRepresentationItemAny {
            GeometricRepresentationItemAny::ShellBasedSurfaceModel(Box::new(self.into()))
        }
    }
    impl Into<GeometricRepresentationItemAny> for ShellBasedWireframeModel {
        fn into(self) -> GeometricRepresentationItemAny {
            GeometricRepresentationItemAny::ShellBasedWireframeModel(Box::new(self.into()))
        }
    }
    impl Into<GeometricRepresentationItemAny> for SolidModel {
        fn into(self) -> GeometricRepresentationItemAny {
            GeometricRepresentationItemAny::SolidModel(Box::new(self.into()))
        }
    }
    impl Into<GeometricRepresentationItemAny> for Surface {
        fn into(self) -> GeometricRepresentationItemAny {
            GeometricRepresentationItemAny::Surface(Box::new(self.into()))
        }
    }
    impl Into<GeometricRepresentationItemAny> for Vector {
        fn into(self) -> GeometricRepresentationItemAny {
            GeometricRepresentationItemAny::Vector(Box::new(self.into()))
        }
    }
    impl Into<GeometricRepresentationItemAny> for VertexPoint {
        fn into(self) -> GeometricRepresentationItemAny {
            GeometricRepresentationItemAny::VertexPoint(Box::new(self.into()))
        }
    }
    impl AsRef<GeometricRepresentationItem> for GeometricRepresentationItemAny {
        fn as_ref(&self) -> &GeometricRepresentationItem {
            match self {
                GeometricRepresentationItemAny::GeometricRepresentationItem(x) => x.as_ref(),
                GeometricRepresentationItemAny::CartesianTransformationOperator(x) => {
                    (**x).as_ref()
                }
                GeometricRepresentationItemAny::Curve(x) => (**x).as_ref(),
                GeometricRepresentationItemAny::Direction(x) => (**x).as_ref(),
                GeometricRepresentationItemAny::EdgeBasedWireframeModel(x) => (**x).as_ref(),
                GeometricRepresentationItemAny::EdgeCurve(x) => (**x).as_ref(),
                GeometricRepresentationItemAny::FaceSurface(x) => (**x).as_ref(),
                GeometricRepresentationItemAny::GeometricSet(x) => (**x).as_ref(),
                GeometricRepresentationItemAny::Placement(x) => (**x).as_ref(),
                GeometricRepresentationItemAny::Point(x) => (**x).as_ref(),
                GeometricRepresentationItemAny::PolyLoop(x) => (**x).as_ref(),
                GeometricRepresentationItemAny::ShellBasedSurfaceModel(x) => (**x).as_ref(),
                GeometricRepresentationItemAny::ShellBasedWireframeModel(x) => (**x).as_ref(),
                GeometricRepresentationItemAny::SolidModel(x) => (**x).as_ref(),
                GeometricRepresentationItemAny::Surface(x) => (**x).as_ref(),
                GeometricRepresentationItemAny::Vector(x) => (**x).as_ref(),
                GeometricRepresentationItemAny::VertexPoint(x) => (**x).as_ref(),
            }
        }
    }
    impl AsRef<RepresentationItem> for GeometricRepresentationItemAny {
        fn as_ref(&self) -> &RepresentationItem {
            match self {
                GeometricRepresentationItemAny::GeometricRepresentationItem(x) => {
                    AsRef::<GeometricRepresentationItem>::as_ref(x).as_ref()
                }
                GeometricRepresentationItemAny::CartesianTransformationOperator(x) => {
                    AsRef::<GeometricRepresentationItem>::as_ref(x.as_ref()).as_ref()
                }
                GeometricRepresentationItemAny::Curve(x) => {
                    AsRef::<GeometricRepresentationItem>::as_ref(x.as_ref()).as_ref()
                }
                GeometricRepresentationItemAny::Direction(x) => {
                    AsRef::<GeometricRepresentationItem>::as_ref(x.as_ref()).as_ref()
                }
                GeometricRepresentationItemAny::EdgeBasedWireframeModel(x) => {
                    AsRef::<GeometricRepresentationItem>::as_ref(x.as_ref()).as_ref()
                }
                GeometricRepresentationItemAny::EdgeCurve(x) => {
                    AsRef::<GeometricRepresentationItem>::as_ref(x.as_ref()).as_ref()
                }
                GeometricRepresentationItemAny::FaceSurface(x) => {
                    AsRef::<GeometricRepresentationItem>::as_ref(x.as_ref()).as_ref()
                }
                GeometricRepresentationItemAny::GeometricSet(x) => {
                    AsRef::<GeometricRepresentationItem>::as_ref(x.as_ref()).as_ref()
                }
                GeometricRepresentationItemAny::Placement(x) => {
                    AsRef::<GeometricRepresentationItem>::as_ref(x.as_ref()).as_ref()
                }
                GeometricRepresentationItemAny::Point(x) => {
                    AsRef::<GeometricRepresentationItem>::as_ref(x.as_ref()).as_ref()
                }
                GeometricRepresentationItemAny::PolyLoop(x) => {
                    AsRef::<GeometricRepresentationItem>::as_ref(x.as_ref()).as_ref()
                }
                GeometricRepresentationItemAny::ShellBasedSurfaceModel(x) => {
                    AsRef::<GeometricRepresentationItem>::as_ref(x.as_ref()).as_ref()
                }
                GeometricRepresentationItemAny::ShellBasedWireframeModel(x) => {
                    AsRef::<GeometricRepresentationItem>::as_ref(x.as_ref()).as_ref()
                }
                GeometricRepresentationItemAny::SolidModel(x) => {
                    AsRef::<GeometricRepresentationItem>::as_ref(x.as_ref()).as_ref()
                }
                GeometricRepresentationItemAny::Surface(x) => {
                    AsRef::<GeometricRepresentationItem>::as_ref(x.as_ref()).as_ref()
                }
                GeometricRepresentationItemAny::Vector(x) => {
                    AsRef::<GeometricRepresentationItem>::as_ref(x.as_ref()).as_ref()
                }
                GeometricRepresentationItemAny::VertexPoint(x) => {
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
        GeometricSet(Box<GeometricSet>),
        #[holder(use_place_holder)]
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
    # [holder (field = geometrically_bounded_surface_shape_representation)]
    #[holder(generate_deserialize)]
    pub struct GeometricallyBoundedSurfaceShapeRepresentation {
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
    # [holder (field = geometrically_bounded_wireframe_shape_representation)]
    #[holder(generate_deserialize)]
    pub struct GeometricallyBoundedWireframeShapeRepresentation {
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
    # [holder (field = global_uncertainty_assigned_context)]
    #[holder(generate_deserialize)]
    pub struct GlobalUncertaintyAssignedContext {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub representation_context: RepresentationContext,
        #[holder(use_place_holder)]
        pub uncertainty: Vec<UncertaintyMeasureWithUnit>,
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
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = intersection_curve)]
    #[holder(generate_deserialize)]
    pub struct IntersectionCurve {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub surface_curve: SurfaceCurve,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = item_defined_transformation)]
    #[holder(generate_deserialize)]
    pub struct ItemDefinedTransformation {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub description: Text,
        #[holder(use_place_holder)]
        pub transform_item_1: RepresentationItemAny,
        #[holder(use_place_holder)]
        pub transform_item_2: RepresentationItemAny,
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
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = local_time)]
    #[holder(generate_deserialize)]
    pub struct LocalTime {
        #[holder(use_place_holder)]
        pub hour_component: HourInDay,
        #[holder(use_place_holder)]
        pub minute_component: Option<MinuteInHour>,
        #[holder(use_place_holder)]
        pub second_component: Option<SecondInMinute>,
        #[holder(use_place_holder)]
        pub zone: CoordinatedUniversalTimeOffset,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = r#loop)]
    #[holder(generate_deserialize)]
    pub struct Loop {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub topological_representation_item: TopologicalRepresentationItem,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum LoopAny {
        #[holder(use_place_holder)]
        Loop(Box<Loop>),
        #[holder(use_place_holder)]
        EdgeLoop(Box<EdgeLoop>),
        #[holder(use_place_holder)]
        PolyLoop(Box<PolyLoop>),
        #[holder(use_place_holder)]
        VertexLoop(Box<VertexLoop>),
    }
    impl Into<LoopAny> for Loop {
        fn into(self) -> LoopAny {
            LoopAny::Loop(Box::new(self))
        }
    }
    impl Into<LoopAny> for EdgeLoop {
        fn into(self) -> LoopAny {
            LoopAny::EdgeLoop(Box::new(self.into()))
        }
    }
    impl Into<LoopAny> for PolyLoop {
        fn into(self) -> LoopAny {
            LoopAny::PolyLoop(Box::new(self.into()))
        }
    }
    impl Into<LoopAny> for VertexLoop {
        fn into(self) -> LoopAny {
            LoopAny::VertexLoop(Box::new(self.into()))
        }
    }
    impl AsRef<Loop> for LoopAny {
        fn as_ref(&self) -> &Loop {
            match self {
                LoopAny::Loop(x) => x.as_ref(),
                LoopAny::EdgeLoop(x) => (**x).as_ref(),
                LoopAny::PolyLoop(x) => (**x).as_ref(),
                LoopAny::VertexLoop(x) => (**x).as_ref(),
            }
        }
    }
    impl AsRef<TopologicalRepresentationItem> for LoopAny {
        fn as_ref(&self) -> &TopologicalRepresentationItem {
            match self {
                LoopAny::Loop(x) => AsRef::<Loop>::as_ref(x).as_ref(),
                LoopAny::EdgeLoop(x) => AsRef::<Loop>::as_ref(x.as_ref()).as_ref(),
                LoopAny::PolyLoop(x) => AsRef::<Loop>::as_ref(x.as_ref()).as_ref(),
                LoopAny::VertexLoop(x) => AsRef::<Loop>::as_ref(x.as_ref()).as_ref(),
            }
        }
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = lot_effectivity)]
    #[holder(generate_deserialize)]
    pub struct LotEffectivity {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub effectivity: Effectivity,
        #[holder(use_place_holder)]
        pub effectivity_lot_id: Identifier,
        #[holder(use_place_holder)]
        pub effectivity_lot_size: MeasureWithUnitAny,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = manifold_solid_brep)]
    #[holder(generate_deserialize)]
    pub struct ManifoldSolidBrep {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub solid_model: SolidModel,
        #[holder(use_place_holder)]
        pub outer: ClosedShellAny,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ManifoldSolidBrepAny {
        #[holder(use_place_holder)]
        ManifoldSolidBrep(Box<ManifoldSolidBrep>),
        #[holder(use_place_holder)]
        BrepWithVoids(Box<BrepWithVoids>),
        #[holder(use_place_holder)]
        FacetedBrep(Box<FacetedBrep>),
    }
    impl Into<ManifoldSolidBrepAny> for ManifoldSolidBrep {
        fn into(self) -> ManifoldSolidBrepAny {
            ManifoldSolidBrepAny::ManifoldSolidBrep(Box::new(self))
        }
    }
    impl Into<ManifoldSolidBrepAny> for BrepWithVoids {
        fn into(self) -> ManifoldSolidBrepAny {
            ManifoldSolidBrepAny::BrepWithVoids(Box::new(self.into()))
        }
    }
    impl Into<ManifoldSolidBrepAny> for FacetedBrep {
        fn into(self) -> ManifoldSolidBrepAny {
            ManifoldSolidBrepAny::FacetedBrep(Box::new(self.into()))
        }
    }
    impl AsRef<ManifoldSolidBrep> for ManifoldSolidBrepAny {
        fn as_ref(&self) -> &ManifoldSolidBrep {
            match self {
                ManifoldSolidBrepAny::ManifoldSolidBrep(x) => x.as_ref(),
                ManifoldSolidBrepAny::BrepWithVoids(x) => (**x).as_ref(),
                ManifoldSolidBrepAny::FacetedBrep(x) => (**x).as_ref(),
            }
        }
    }
    impl AsRef<SolidModel> for ManifoldSolidBrepAny {
        fn as_ref(&self) -> &SolidModel {
            match self {
                ManifoldSolidBrepAny::ManifoldSolidBrep(x) => {
                    AsRef::<ManifoldSolidBrep>::as_ref(x).as_ref()
                }
                ManifoldSolidBrepAny::BrepWithVoids(x) => {
                    AsRef::<ManifoldSolidBrep>::as_ref(x.as_ref()).as_ref()
                }
                ManifoldSolidBrepAny::FacetedBrep(x) => {
                    AsRef::<ManifoldSolidBrep>::as_ref(x.as_ref()).as_ref()
                }
            }
        }
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = manifold_surface_shape_representation)]
    #[holder(generate_deserialize)]
    pub struct ManifoldSurfaceShapeRepresentation {
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
        pub mapping_source: RepresentationMap,
        #[holder(use_place_holder)]
        pub mapping_target: RepresentationItemAny,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = mass_measure_with_unit)]
    #[holder(generate_deserialize)]
    pub struct MassMeasureWithUnit {
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
    # [holder (field = mass_unit)]
    #[holder(generate_deserialize)]
    pub struct MassUnit {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub named_unit: NamedUnit,
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
        MeasureWithUnit(Box<MeasureWithUnit>),
        #[holder(use_place_holder)]
        AreaMeasureWithUnit(Box<AreaMeasureWithUnit>),
        #[holder(use_place_holder)]
        LengthMeasureWithUnit(Box<LengthMeasureWithUnit>),
        #[holder(use_place_holder)]
        MassMeasureWithUnit(Box<MassMeasureWithUnit>),
        #[holder(use_place_holder)]
        PlaneAngleMeasureWithUnit(Box<PlaneAngleMeasureWithUnit>),
        #[holder(use_place_holder)]
        SolidAngleMeasureWithUnit(Box<SolidAngleMeasureWithUnit>),
        #[holder(use_place_holder)]
        UncertaintyMeasureWithUnit(Box<UncertaintyMeasureWithUnit>),
        #[holder(use_place_holder)]
        VolumeMeasureWithUnit(Box<VolumeMeasureWithUnit>),
    }
    impl Into<MeasureWithUnitAny> for MeasureWithUnit {
        fn into(self) -> MeasureWithUnitAny {
            MeasureWithUnitAny::MeasureWithUnit(Box::new(self))
        }
    }
    impl Into<MeasureWithUnitAny> for AreaMeasureWithUnit {
        fn into(self) -> MeasureWithUnitAny {
            MeasureWithUnitAny::AreaMeasureWithUnit(Box::new(self.into()))
        }
    }
    impl Into<MeasureWithUnitAny> for LengthMeasureWithUnit {
        fn into(self) -> MeasureWithUnitAny {
            MeasureWithUnitAny::LengthMeasureWithUnit(Box::new(self.into()))
        }
    }
    impl Into<MeasureWithUnitAny> for MassMeasureWithUnit {
        fn into(self) -> MeasureWithUnitAny {
            MeasureWithUnitAny::MassMeasureWithUnit(Box::new(self.into()))
        }
    }
    impl Into<MeasureWithUnitAny> for PlaneAngleMeasureWithUnit {
        fn into(self) -> MeasureWithUnitAny {
            MeasureWithUnitAny::PlaneAngleMeasureWithUnit(Box::new(self.into()))
        }
    }
    impl Into<MeasureWithUnitAny> for SolidAngleMeasureWithUnit {
        fn into(self) -> MeasureWithUnitAny {
            MeasureWithUnitAny::SolidAngleMeasureWithUnit(Box::new(self.into()))
        }
    }
    impl Into<MeasureWithUnitAny> for UncertaintyMeasureWithUnit {
        fn into(self) -> MeasureWithUnitAny {
            MeasureWithUnitAny::UncertaintyMeasureWithUnit(Box::new(self.into()))
        }
    }
    impl Into<MeasureWithUnitAny> for VolumeMeasureWithUnit {
        fn into(self) -> MeasureWithUnitAny {
            MeasureWithUnitAny::VolumeMeasureWithUnit(Box::new(self.into()))
        }
    }
    impl AsRef<MeasureWithUnit> for MeasureWithUnitAny {
        fn as_ref(&self) -> &MeasureWithUnit {
            match self {
                MeasureWithUnitAny::MeasureWithUnit(x) => x.as_ref(),
                MeasureWithUnitAny::AreaMeasureWithUnit(x) => (**x).as_ref(),
                MeasureWithUnitAny::LengthMeasureWithUnit(x) => (**x).as_ref(),
                MeasureWithUnitAny::MassMeasureWithUnit(x) => (**x).as_ref(),
                MeasureWithUnitAny::PlaneAngleMeasureWithUnit(x) => (**x).as_ref(),
                MeasureWithUnitAny::SolidAngleMeasureWithUnit(x) => (**x).as_ref(),
                MeasureWithUnitAny::UncertaintyMeasureWithUnit(x) => (**x).as_ref(),
                MeasureWithUnitAny::VolumeMeasureWithUnit(x) => (**x).as_ref(),
            }
        }
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = mechanical_context)]
    #[holder(generate_deserialize)]
    pub struct MechanicalContext {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub product_context: ProductContext,
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
        NamedUnit(Box<NamedUnit>),
        #[holder(use_place_holder)]
        AreaUnit(Box<AreaUnit>),
        #[holder(use_place_holder)]
        ContextDependentUnit(Box<ContextDependentUnit>),
        #[holder(use_place_holder)]
        ConversionBasedUnit(Box<ConversionBasedUnit>),
        #[holder(use_place_holder)]
        LengthUnit(Box<LengthUnit>),
        #[holder(use_place_holder)]
        MassUnit(Box<MassUnit>),
        #[holder(use_place_holder)]
        PlaneAngleUnit(Box<PlaneAngleUnit>),
        #[holder(use_place_holder)]
        SiUnit(Box<SiUnit>),
        #[holder(use_place_holder)]
        SolidAngleUnit(Box<SolidAngleUnit>),
        #[holder(use_place_holder)]
        VolumeUnit(Box<VolumeUnit>),
    }
    impl Into<NamedUnitAny> for NamedUnit {
        fn into(self) -> NamedUnitAny {
            NamedUnitAny::NamedUnit(Box::new(self))
        }
    }
    impl Into<NamedUnitAny> for AreaUnit {
        fn into(self) -> NamedUnitAny {
            NamedUnitAny::AreaUnit(Box::new(self.into()))
        }
    }
    impl Into<NamedUnitAny> for ContextDependentUnit {
        fn into(self) -> NamedUnitAny {
            NamedUnitAny::ContextDependentUnit(Box::new(self.into()))
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
    impl Into<NamedUnitAny> for MassUnit {
        fn into(self) -> NamedUnitAny {
            NamedUnitAny::MassUnit(Box::new(self.into()))
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
    impl Into<NamedUnitAny> for SolidAngleUnit {
        fn into(self) -> NamedUnitAny {
            NamedUnitAny::SolidAngleUnit(Box::new(self.into()))
        }
    }
    impl Into<NamedUnitAny> for VolumeUnit {
        fn into(self) -> NamedUnitAny {
            NamedUnitAny::VolumeUnit(Box::new(self.into()))
        }
    }
    impl AsRef<NamedUnit> for NamedUnitAny {
        fn as_ref(&self) -> &NamedUnit {
            match self {
                NamedUnitAny::NamedUnit(x) => x.as_ref(),
                NamedUnitAny::AreaUnit(x) => (**x).as_ref(),
                NamedUnitAny::ContextDependentUnit(x) => (**x).as_ref(),
                NamedUnitAny::ConversionBasedUnit(x) => (**x).as_ref(),
                NamedUnitAny::LengthUnit(x) => (**x).as_ref(),
                NamedUnitAny::MassUnit(x) => (**x).as_ref(),
                NamedUnitAny::PlaneAngleUnit(x) => (**x).as_ref(),
                NamedUnitAny::SiUnit(x) => (**x).as_ref(),
                NamedUnitAny::SolidAngleUnit(x) => (**x).as_ref(),
                NamedUnitAny::VolumeUnit(x) => (**x).as_ref(),
            }
        }
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = next_assembly_usage_occurrence)]
    #[holder(generate_deserialize)]
    pub struct NextAssemblyUsageOccurrence {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub assembly_component_usage: AssemblyComponentUsage,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = offset_curve_3d)]
    #[holder(generate_deserialize)]
    pub struct OffsetCurve3D {
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
        #[holder(use_place_holder)]
        pub ref_direction: Direction,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = offset_surface)]
    #[holder(generate_deserialize)]
    pub struct OffsetSurface {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub surface: Surface,
        #[holder(use_place_holder)]
        pub basis_surface: SurfaceAny,
        #[holder(use_place_holder)]
        pub distance: LengthMeasure,
        pub self_intersect: Logical,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = open_shell)]
    #[holder(generate_deserialize)]
    pub struct OpenShell {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub connected_face_set: ConnectedFaceSet,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum OpenShellAny {
        #[holder(use_place_holder)]
        OpenShell(Box<OpenShell>),
        #[holder(use_place_holder)]
        OrientedOpenShell(Box<OrientedOpenShell>),
    }
    impl Into<OpenShellAny> for OpenShell {
        fn into(self) -> OpenShellAny {
            OpenShellAny::OpenShell(Box::new(self))
        }
    }
    impl Into<OpenShellAny> for OrientedOpenShell {
        fn into(self) -> OpenShellAny {
            OpenShellAny::OrientedOpenShell(Box::new(self.into()))
        }
    }
    impl AsRef<OpenShell> for OpenShellAny {
        fn as_ref(&self) -> &OpenShell {
            match self {
                OpenShellAny::OpenShell(x) => x.as_ref(),
                OpenShellAny::OrientedOpenShell(x) => (**x).as_ref(),
            }
        }
    }
    impl AsRef<ConnectedFaceSet> for OpenShellAny {
        fn as_ref(&self) -> &ConnectedFaceSet {
            match self {
                OpenShellAny::OpenShell(x) => AsRef::<OpenShell>::as_ref(x).as_ref(),
                OpenShellAny::OrientedOpenShell(x) => {
                    AsRef::<OpenShell>::as_ref(x.as_ref()).as_ref()
                }
            }
        }
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = ordinal_date)]
    #[holder(generate_deserialize)]
    pub struct OrdinalDate {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub date: Date,
        #[holder(use_place_holder)]
        pub day_component: DayInYearNumber,
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
    # [holder (field = organization_relationship)]
    #[holder(generate_deserialize)]
    pub struct OrganizationRelationship {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub description: Text,
        #[holder(use_place_holder)]
        pub relating_organization: Organization,
        #[holder(use_place_holder)]
        pub related_organization: Organization,
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
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = organizational_project)]
    #[holder(generate_deserialize)]
    pub struct OrganizationalProject {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub description: Text,
        #[holder(use_place_holder)]
        pub responsible_organizations: Vec<Organization>,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = oriented_closed_shell)]
    #[holder(generate_deserialize)]
    pub struct OrientedClosedShell {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub closed_shell: ClosedShell,
        #[holder(use_place_holder)]
        pub closed_shell_element: ClosedShellAny,
        pub orientation: bool,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = oriented_edge)]
    #[holder(generate_deserialize)]
    pub struct OrientedEdge {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub edge: Edge,
        #[holder(use_place_holder)]
        pub edge_element: EdgeAny,
        pub orientation: bool,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = oriented_face)]
    #[holder(generate_deserialize)]
    pub struct OrientedFace {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub face: Face,
        #[holder(use_place_holder)]
        pub face_element: FaceAny,
        pub orientation: bool,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = oriented_open_shell)]
    #[holder(generate_deserialize)]
    pub struct OrientedOpenShell {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub open_shell: OpenShell,
        #[holder(use_place_holder)]
        pub open_shell_element: OpenShellAny,
        pub orientation: bool,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = oriented_path)]
    #[holder(generate_deserialize)]
    pub struct OrientedPath {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub path: Path,
        #[holder(use_place_holder)]
        pub path_element: PathAny,
        pub orientation: bool,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = outer_boundary_curve)]
    #[holder(generate_deserialize)]
    pub struct OuterBoundaryCurve {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub boundary_curve: BoundaryCurve,
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
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = parametric_representation_context)]
    #[holder(generate_deserialize)]
    pub struct ParametricRepresentationContext {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub representation_context: RepresentationContext,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = path)]
    #[holder(generate_deserialize)]
    pub struct Path {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub topological_representation_item: TopologicalRepresentationItem,
        #[holder(use_place_holder)]
        pub edge_list: Vec<OrientedEdge>,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum PathAny {
        #[holder(use_place_holder)]
        Path(Box<Path>),
        #[holder(use_place_holder)]
        EdgeLoop(Box<EdgeLoop>),
        #[holder(use_place_holder)]
        OrientedPath(Box<OrientedPath>),
    }
    impl Into<PathAny> for Path {
        fn into(self) -> PathAny {
            PathAny::Path(Box::new(self))
        }
    }
    impl Into<PathAny> for EdgeLoop {
        fn into(self) -> PathAny {
            PathAny::EdgeLoop(Box::new(self.into()))
        }
    }
    impl Into<PathAny> for OrientedPath {
        fn into(self) -> PathAny {
            PathAny::OrientedPath(Box::new(self.into()))
        }
    }
    impl AsRef<Path> for PathAny {
        fn as_ref(&self) -> &Path {
            match self {
                PathAny::Path(x) => x.as_ref(),
                PathAny::EdgeLoop(x) => (**x).as_ref(),
                PathAny::OrientedPath(x) => (**x).as_ref(),
            }
        }
    }
    impl AsRef<TopologicalRepresentationItem> for PathAny {
        fn as_ref(&self) -> &TopologicalRepresentationItem {
            match self {
                PathAny::Path(x) => AsRef::<Path>::as_ref(x).as_ref(),
                PathAny::EdgeLoop(x) => AsRef::<Path>::as_ref(x.as_ref()).as_ref(),
                PathAny::OrientedPath(x) => AsRef::<Path>::as_ref(x.as_ref()).as_ref(),
            }
        }
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = pcurve)]
    #[holder(generate_deserialize)]
    pub struct Pcurve {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub curve: Curve,
        #[holder(use_place_holder)]
        pub basis_surface: SurfaceAny,
        #[holder(use_place_holder)]
        pub reference_to_curve: DefinitionalRepresentation,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum PcurveAny {
        #[holder(use_place_holder)]
        Pcurve(Box<Pcurve>),
        #[holder(use_place_holder)]
        BoundedPcurve(Box<BoundedPcurve>),
    }
    impl Into<PcurveAny> for Pcurve {
        fn into(self) -> PcurveAny {
            PcurveAny::Pcurve(Box::new(self))
        }
    }
    impl Into<PcurveAny> for BoundedPcurve {
        fn into(self) -> PcurveAny {
            PcurveAny::BoundedPcurve(Box::new(self.into()))
        }
    }
    impl AsRef<Pcurve> for PcurveAny {
        fn as_ref(&self) -> &Pcurve {
            match self {
                PcurveAny::Pcurve(x) => x.as_ref(),
                PcurveAny::BoundedPcurve(x) => (**x).as_ref(),
            }
        }
    }
    impl AsRef<Curve> for PcurveAny {
        fn as_ref(&self) -> &Curve {
            match self {
                PcurveAny::Pcurve(x) => AsRef::<Pcurve>::as_ref(x).as_ref(),
                PcurveAny::BoundedPcurve(x) => AsRef::<Pcurve>::as_ref(x.as_ref()).as_ref(),
            }
        }
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
        PersonAndOrganizationAssignment(Box<PersonAndOrganizationAssignment>),
        #[holder(use_place_holder)]
        CcDesignPersonAndOrganizationAssignment(Box<CcDesignPersonAndOrganizationAssignment>),
    }
    impl Into<PersonAndOrganizationAssignmentAny> for PersonAndOrganizationAssignment {
        fn into(self) -> PersonAndOrganizationAssignmentAny {
            PersonAndOrganizationAssignmentAny::PersonAndOrganizationAssignment(Box::new(self))
        }
    }
    impl Into<PersonAndOrganizationAssignmentAny> for CcDesignPersonAndOrganizationAssignment {
        fn into(self) -> PersonAndOrganizationAssignmentAny {
            PersonAndOrganizationAssignmentAny::CcDesignPersonAndOrganizationAssignment(Box::new(
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
                PersonAndOrganizationAssignmentAny::CcDesignPersonAndOrganizationAssignment(x) => {
                    (**x).as_ref()
                }
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
        Placement(Box<Placement>),
        #[holder(use_place_holder)]
        Axis1Placement(Box<Axis1Placement>),
        #[holder(use_place_holder)]
        Axis2Placement2D(Box<Axis2Placement2D>),
        #[holder(use_place_holder)]
        Axis2Placement3D(Box<Axis2Placement3D>),
    }
    impl Into<PlacementAny> for Placement {
        fn into(self) -> PlacementAny {
            PlacementAny::Placement(Box::new(self))
        }
    }
    impl Into<PlacementAny> for Axis1Placement {
        fn into(self) -> PlacementAny {
            PlacementAny::Axis1Placement(Box::new(self.into()))
        }
    }
    impl Into<PlacementAny> for Axis2Placement2D {
        fn into(self) -> PlacementAny {
            PlacementAny::Axis2Placement2D(Box::new(self.into()))
        }
    }
    impl Into<PlacementAny> for Axis2Placement3D {
        fn into(self) -> PlacementAny {
            PlacementAny::Axis2Placement3D(Box::new(self.into()))
        }
    }
    impl AsRef<Placement> for PlacementAny {
        fn as_ref(&self) -> &Placement {
            match self {
                PlacementAny::Placement(x) => x.as_ref(),
                PlacementAny::Axis1Placement(x) => (**x).as_ref(),
                PlacementAny::Axis2Placement2D(x) => (**x).as_ref(),
                PlacementAny::Axis2Placement3D(x) => (**x).as_ref(),
            }
        }
    }
    impl AsRef<GeometricRepresentationItem> for PlacementAny {
        fn as_ref(&self) -> &GeometricRepresentationItem {
            match self {
                PlacementAny::Placement(x) => AsRef::<Placement>::as_ref(x).as_ref(),
                PlacementAny::Axis1Placement(x) => AsRef::<Placement>::as_ref(x.as_ref()).as_ref(),
                PlacementAny::Axis2Placement2D(x) => {
                    AsRef::<Placement>::as_ref(x.as_ref()).as_ref()
                }
                PlacementAny::Axis2Placement3D(x) => {
                    AsRef::<Placement>::as_ref(x.as_ref()).as_ref()
                }
            }
        }
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = plane)]
    #[holder(generate_deserialize)]
    pub struct Plane {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub elementary_surface: ElementarySurface,
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
        Point(Box<Point>),
        #[holder(use_place_holder)]
        CartesianPoint(Box<CartesianPoint>),
        #[holder(use_place_holder)]
        DegeneratePcurve(Box<DegeneratePcurveAny>),
        #[holder(use_place_holder)]
        PointOnCurve(Box<PointOnCurve>),
        #[holder(use_place_holder)]
        PointOnSurface(Box<PointOnSurface>),
        #[holder(use_place_holder)]
        PointReplica(Box<PointReplica>),
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
    impl Into<PointAny> for DegeneratePcurve {
        fn into(self) -> PointAny {
            PointAny::DegeneratePcurve(Box::new(self.into()))
        }
    }
    impl Into<PointAny> for PointOnCurve {
        fn into(self) -> PointAny {
            PointAny::PointOnCurve(Box::new(self.into()))
        }
    }
    impl Into<PointAny> for PointOnSurface {
        fn into(self) -> PointAny {
            PointAny::PointOnSurface(Box::new(self.into()))
        }
    }
    impl Into<PointAny> for PointReplica {
        fn into(self) -> PointAny {
            PointAny::PointReplica(Box::new(self.into()))
        }
    }
    impl AsRef<Point> for PointAny {
        fn as_ref(&self) -> &Point {
            match self {
                PointAny::Point(x) => x.as_ref(),
                PointAny::CartesianPoint(x) => (**x).as_ref(),
                PointAny::DegeneratePcurve(x) => (**x).as_ref(),
                PointAny::PointOnCurve(x) => (**x).as_ref(),
                PointAny::PointOnSurface(x) => (**x).as_ref(),
                PointAny::PointReplica(x) => (**x).as_ref(),
            }
        }
    }
    impl AsRef<GeometricRepresentationItem> for PointAny {
        fn as_ref(&self) -> &GeometricRepresentationItem {
            match self {
                PointAny::Point(x) => AsRef::<Point>::as_ref(x).as_ref(),
                PointAny::CartesianPoint(x) => AsRef::<Point>::as_ref(x.as_ref()).as_ref(),
                PointAny::DegeneratePcurve(x) => AsRef::<Point>::as_ref(x.as_ref()).as_ref(),
                PointAny::PointOnCurve(x) => AsRef::<Point>::as_ref(x.as_ref()).as_ref(),
                PointAny::PointOnSurface(x) => AsRef::<Point>::as_ref(x.as_ref()).as_ref(),
                PointAny::PointReplica(x) => AsRef::<Point>::as_ref(x.as_ref()).as_ref(),
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
    # [holder (field = point_on_surface)]
    #[holder(generate_deserialize)]
    pub struct PointOnSurface {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub point: Point,
        #[holder(use_place_holder)]
        pub basis_surface: SurfaceAny,
        #[holder(use_place_holder)]
        pub point_parameter_u: ParameterValue,
        #[holder(use_place_holder)]
        pub point_parameter_v: ParameterValue,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = point_replica)]
    #[holder(generate_deserialize)]
    pub struct PointReplica {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub point: Point,
        #[holder(use_place_holder)]
        pub parent_pt: PointAny,
        #[holder(use_place_holder)]
        pub transformation: CartesianTransformationOperatorAny,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut)]
    # [holder (table = Tables)]
    # [holder (field = poly_loop)]
    #[holder(generate_deserialize)]
    pub struct PolyLoop {
        #[as_ref]
        #[as_mut]
        #[holder(use_place_holder)]
        pub r#loop: Loop,
        #[as_ref]
        #[as_mut]
        #[holder(use_place_holder)]
        pub geometric_representation_item: GeometricRepresentationItem,
        #[holder(use_place_holder)]
        pub polygon: Vec<CartesianPoint>,
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
        pub frame_of_reference: Vec<ProductContextAny>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = product_category)]
    #[holder(generate_deserialize)]
    pub struct ProductCategory {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub description: Option<Text>,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ProductCategoryAny {
        #[holder(use_place_holder)]
        ProductCategory(Box<ProductCategory>),
        #[holder(use_place_holder)]
        ProductRelatedProductCategory(Box<ProductRelatedProductCategory>),
    }
    impl Into<ProductCategoryAny> for ProductCategory {
        fn into(self) -> ProductCategoryAny {
            ProductCategoryAny::ProductCategory(Box::new(self))
        }
    }
    impl Into<ProductCategoryAny> for ProductRelatedProductCategory {
        fn into(self) -> ProductCategoryAny {
            ProductCategoryAny::ProductRelatedProductCategory(Box::new(self.into()))
        }
    }
    impl AsRef<ProductCategory> for ProductCategoryAny {
        fn as_ref(&self) -> &ProductCategory {
            match self {
                ProductCategoryAny::ProductCategory(x) => x.as_ref(),
                ProductCategoryAny::ProductRelatedProductCategory(x) => (**x).as_ref(),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = product_category_relationship)]
    #[holder(generate_deserialize)]
    pub struct ProductCategoryRelationship {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub description: Text,
        #[holder(use_place_holder)]
        pub category: ProductCategoryAny,
        #[holder(use_place_holder)]
        pub sub_category: ProductCategoryAny,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = product_concept)]
    #[holder(generate_deserialize)]
    pub struct ProductConcept {
        #[holder(use_place_holder)]
        pub id: Identifier,
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub description: Text,
        #[holder(use_place_holder)]
        pub market_context: ProductConceptContext,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = product_concept_context)]
    #[holder(generate_deserialize)]
    pub struct ProductConceptContext {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub application_context_element: ApplicationContextElement,
        #[holder(use_place_holder)]
        pub market_segment_type: Label,
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
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ProductContextAny {
        #[holder(use_place_holder)]
        ProductContext(Box<ProductContext>),
        #[holder(use_place_holder)]
        MechanicalContext(Box<MechanicalContext>),
    }
    impl Into<ProductContextAny> for ProductContext {
        fn into(self) -> ProductContextAny {
            ProductContextAny::ProductContext(Box::new(self))
        }
    }
    impl Into<ProductContextAny> for MechanicalContext {
        fn into(self) -> ProductContextAny {
            ProductContextAny::MechanicalContext(Box::new(self.into()))
        }
    }
    impl AsRef<ProductContext> for ProductContextAny {
        fn as_ref(&self) -> &ProductContext {
            match self {
                ProductContextAny::ProductContext(x) => x.as_ref(),
                ProductContextAny::MechanicalContext(x) => (**x).as_ref(),
            }
        }
    }
    impl AsRef<ApplicationContextElement> for ProductContextAny {
        fn as_ref(&self) -> &ApplicationContextElement {
            match self {
                ProductContextAny::ProductContext(x) => AsRef::<ProductContext>::as_ref(x).as_ref(),
                ProductContextAny::MechanicalContext(x) => {
                    AsRef::<ProductContext>::as_ref(x.as_ref()).as_ref()
                }
            }
        }
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
        pub formation: ProductDefinitionFormationAny,
        #[holder(use_place_holder)]
        pub frame_of_reference: ProductDefinitionContextAny,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ProductDefinitionAny {
        #[holder(use_place_holder)]
        ProductDefinition(Box<ProductDefinition>),
        #[holder(use_place_holder)]
        ProductDefinitionWithAssociatedDocuments(Box<ProductDefinitionWithAssociatedDocuments>),
    }
    impl Into<ProductDefinitionAny> for ProductDefinition {
        fn into(self) -> ProductDefinitionAny {
            ProductDefinitionAny::ProductDefinition(Box::new(self))
        }
    }
    impl Into<ProductDefinitionAny> for ProductDefinitionWithAssociatedDocuments {
        fn into(self) -> ProductDefinitionAny {
            ProductDefinitionAny::ProductDefinitionWithAssociatedDocuments(Box::new(self.into()))
        }
    }
    impl AsRef<ProductDefinition> for ProductDefinitionAny {
        fn as_ref(&self) -> &ProductDefinition {
            match self {
                ProductDefinitionAny::ProductDefinition(x) => x.as_ref(),
                ProductDefinitionAny::ProductDefinitionWithAssociatedDocuments(x) => (**x).as_ref(),
            }
        }
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
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ProductDefinitionContextAny {
        #[holder(use_place_holder)]
        ProductDefinitionContext(Box<ProductDefinitionContext>),
        #[holder(use_place_holder)]
        DesignContext(Box<DesignContext>),
    }
    impl Into<ProductDefinitionContextAny> for ProductDefinitionContext {
        fn into(self) -> ProductDefinitionContextAny {
            ProductDefinitionContextAny::ProductDefinitionContext(Box::new(self))
        }
    }
    impl Into<ProductDefinitionContextAny> for DesignContext {
        fn into(self) -> ProductDefinitionContextAny {
            ProductDefinitionContextAny::DesignContext(Box::new(self.into()))
        }
    }
    impl AsRef<ProductDefinitionContext> for ProductDefinitionContextAny {
        fn as_ref(&self) -> &ProductDefinitionContext {
            match self {
                ProductDefinitionContextAny::ProductDefinitionContext(x) => x.as_ref(),
                ProductDefinitionContextAny::DesignContext(x) => (**x).as_ref(),
            }
        }
    }
    impl AsRef<ApplicationContextElement> for ProductDefinitionContextAny {
        fn as_ref(&self) -> &ApplicationContextElement {
            match self {
                ProductDefinitionContextAny::ProductDefinitionContext(x) => {
                    AsRef::<ProductDefinitionContext>::as_ref(x).as_ref()
                }
                ProductDefinitionContextAny::DesignContext(x) => {
                    AsRef::<ProductDefinitionContext>::as_ref(x.as_ref()).as_ref()
                }
            }
        }
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = product_definition_effectivity)]
    #[holder(generate_deserialize)]
    pub struct ProductDefinitionEffectivity {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub effectivity: Effectivity,
        #[holder(use_place_holder)]
        pub usage: ProductDefinitionRelationshipAny,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ProductDefinitionEffectivityAny {
        #[holder(use_place_holder)]
        ProductDefinitionEffectivity(Box<ProductDefinitionEffectivity>),
        #[holder(use_place_holder)]
        ConfigurationEffectivity(Box<ConfigurationEffectivity>),
    }
    impl Into<ProductDefinitionEffectivityAny> for ProductDefinitionEffectivity {
        fn into(self) -> ProductDefinitionEffectivityAny {
            ProductDefinitionEffectivityAny::ProductDefinitionEffectivity(Box::new(self))
        }
    }
    impl Into<ProductDefinitionEffectivityAny> for ConfigurationEffectivity {
        fn into(self) -> ProductDefinitionEffectivityAny {
            ProductDefinitionEffectivityAny::ConfigurationEffectivity(Box::new(self.into()))
        }
    }
    impl AsRef<ProductDefinitionEffectivity> for ProductDefinitionEffectivityAny {
        fn as_ref(&self) -> &ProductDefinitionEffectivity {
            match self {
                ProductDefinitionEffectivityAny::ProductDefinitionEffectivity(x) => x.as_ref(),
                ProductDefinitionEffectivityAny::ConfigurationEffectivity(x) => (**x).as_ref(),
            }
        }
    }
    impl AsRef<Effectivity> for ProductDefinitionEffectivityAny {
        fn as_ref(&self) -> &Effectivity {
            match self {
                ProductDefinitionEffectivityAny::ProductDefinitionEffectivity(x) => {
                    AsRef::<ProductDefinitionEffectivity>::as_ref(x).as_ref()
                }
                ProductDefinitionEffectivityAny::ConfigurationEffectivity(x) => {
                    AsRef::<ProductDefinitionEffectivity>::as_ref(x.as_ref()).as_ref()
                }
            }
        }
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
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ProductDefinitionFormationAny {
        #[holder(use_place_holder)]
        ProductDefinitionFormation(Box<ProductDefinitionFormation>),
        #[holder(use_place_holder)]
        ProductDefinitionFormationWithSpecifiedSource(
            Box<ProductDefinitionFormationWithSpecifiedSource>,
        ),
    }
    impl Into<ProductDefinitionFormationAny> for ProductDefinitionFormation {
        fn into(self) -> ProductDefinitionFormationAny {
            ProductDefinitionFormationAny::ProductDefinitionFormation(Box::new(self))
        }
    }
    impl Into<ProductDefinitionFormationAny> for ProductDefinitionFormationWithSpecifiedSource {
        fn into(self) -> ProductDefinitionFormationAny {
            ProductDefinitionFormationAny::ProductDefinitionFormationWithSpecifiedSource(Box::new(
                self.into(),
            ))
        }
    }
    impl AsRef<ProductDefinitionFormation> for ProductDefinitionFormationAny {
        fn as_ref(&self) -> &ProductDefinitionFormation {
            match self {
                ProductDefinitionFormationAny::ProductDefinitionFormation(x) => x.as_ref(),
                ProductDefinitionFormationAny::ProductDefinitionFormationWithSpecifiedSource(x) => {
                    (**x).as_ref()
                }
            }
        }
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = product_definition_formation_with_specified_source)]
    #[holder(generate_deserialize)]
    pub struct ProductDefinitionFormationWithSpecifiedSource {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub product_definition_formation: ProductDefinitionFormation,
        pub make_or_buy: Source,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = product_definition_relationship)]
    #[holder(generate_deserialize)]
    pub struct ProductDefinitionRelationship {
        #[holder(use_place_holder)]
        pub id: Identifier,
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub description: Text,
        #[holder(use_place_holder)]
        pub relating_product_definition: ProductDefinitionAny,
        #[holder(use_place_holder)]
        pub related_product_definition: ProductDefinitionAny,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ProductDefinitionRelationshipAny {
        #[holder(use_place_holder)]
        ProductDefinitionRelationship(Box<ProductDefinitionRelationship>),
        #[holder(use_place_holder)]
        DesignMakeFromRelationship(Box<DesignMakeFromRelationship>),
        #[holder(use_place_holder)]
        ProductDefinitionUsage(Box<ProductDefinitionUsageAny>),
        #[holder(use_place_holder)]
        SuppliedPartRelationship(Box<SuppliedPartRelationship>),
    }
    impl Into<ProductDefinitionRelationshipAny> for ProductDefinitionRelationship {
        fn into(self) -> ProductDefinitionRelationshipAny {
            ProductDefinitionRelationshipAny::ProductDefinitionRelationship(Box::new(self))
        }
    }
    impl Into<ProductDefinitionRelationshipAny> for DesignMakeFromRelationship {
        fn into(self) -> ProductDefinitionRelationshipAny {
            ProductDefinitionRelationshipAny::DesignMakeFromRelationship(Box::new(self.into()))
        }
    }
    impl Into<ProductDefinitionRelationshipAny> for ProductDefinitionUsage {
        fn into(self) -> ProductDefinitionRelationshipAny {
            ProductDefinitionRelationshipAny::ProductDefinitionUsage(Box::new(self.into()))
        }
    }
    impl Into<ProductDefinitionRelationshipAny> for SuppliedPartRelationship {
        fn into(self) -> ProductDefinitionRelationshipAny {
            ProductDefinitionRelationshipAny::SuppliedPartRelationship(Box::new(self.into()))
        }
    }
    impl AsRef<ProductDefinitionRelationship> for ProductDefinitionRelationshipAny {
        fn as_ref(&self) -> &ProductDefinitionRelationship {
            match self {
                ProductDefinitionRelationshipAny::ProductDefinitionRelationship(x) => x.as_ref(),
                ProductDefinitionRelationshipAny::DesignMakeFromRelationship(x) => (**x).as_ref(),
                ProductDefinitionRelationshipAny::ProductDefinitionUsage(x) => (**x).as_ref(),
                ProductDefinitionRelationshipAny::SuppliedPartRelationship(x) => (**x).as_ref(),
            }
        }
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
    # [holder (field = product_definition_usage)]
    #[holder(generate_deserialize)]
    pub struct ProductDefinitionUsage {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub product_definition_relationship: ProductDefinitionRelationship,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ProductDefinitionUsageAny {
        #[holder(use_place_holder)]
        ProductDefinitionUsage(Box<ProductDefinitionUsage>),
        #[holder(use_place_holder)]
        AssemblyComponentUsage(Box<AssemblyComponentUsageAny>),
    }
    impl Into<ProductDefinitionUsageAny> for ProductDefinitionUsage {
        fn into(self) -> ProductDefinitionUsageAny {
            ProductDefinitionUsageAny::ProductDefinitionUsage(Box::new(self))
        }
    }
    impl Into<ProductDefinitionUsageAny> for AssemblyComponentUsage {
        fn into(self) -> ProductDefinitionUsageAny {
            ProductDefinitionUsageAny::AssemblyComponentUsage(Box::new(self.into()))
        }
    }
    impl AsRef<ProductDefinitionUsage> for ProductDefinitionUsageAny {
        fn as_ref(&self) -> &ProductDefinitionUsage {
            match self {
                ProductDefinitionUsageAny::ProductDefinitionUsage(x) => x.as_ref(),
                ProductDefinitionUsageAny::AssemblyComponentUsage(x) => (**x).as_ref(),
            }
        }
    }
    impl AsRef<ProductDefinitionRelationship> for ProductDefinitionUsageAny {
        fn as_ref(&self) -> &ProductDefinitionRelationship {
            match self {
                ProductDefinitionUsageAny::ProductDefinitionUsage(x) => {
                    AsRef::<ProductDefinitionUsage>::as_ref(x).as_ref()
                }
                ProductDefinitionUsageAny::AssemblyComponentUsage(x) => {
                    AsRef::<ProductDefinitionUsage>::as_ref(x.as_ref()).as_ref()
                }
            }
        }
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = product_definition_with_associated_documents)]
    #[holder(generate_deserialize)]
    pub struct ProductDefinitionWithAssociatedDocuments {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub product_definition: ProductDefinition,
        #[holder(use_place_holder)]
        pub documentation_ids: Vec<DocumentAny>,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = product_related_product_category)]
    #[holder(generate_deserialize)]
    pub struct ProductRelatedProductCategory {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub product_category: ProductCategory,
        #[holder(use_place_holder)]
        pub products: Vec<Product>,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = promissory_usage_occurrence)]
    #[holder(generate_deserialize)]
    pub struct PromissoryUsageOccurrence {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub assembly_component_usage: AssemblyComponentUsage,
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
        PropertyDefinition(Box<PropertyDefinition>),
        #[holder(use_place_holder)]
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
        PropertyDefinitionRepresentation(Box<PropertyDefinitionRepresentation>),
        #[holder(use_place_holder)]
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
    # [holder (field = quantified_assembly_component_usage)]
    #[holder(generate_deserialize)]
    pub struct QuantifiedAssemblyComponentUsage {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub assembly_component_usage: AssemblyComponentUsage,
        #[holder(use_place_holder)]
        pub quantity: MeasureWithUnitAny,
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
    # [holder (field = quasi_uniform_surface)]
    #[holder(generate_deserialize)]
    pub struct QuasiUniformSurface {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub b_spline_surface: BSplineSurface,
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
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = rational_b_spline_surface)]
    #[holder(generate_deserialize)]
    pub struct RationalBSplineSurface {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub b_spline_surface: BSplineSurface,
        pub weights_data: Vec<Vec<f64>>,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = rectangular_composite_surface)]
    #[holder(generate_deserialize)]
    pub struct RectangularCompositeSurface {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub bounded_surface: BoundedSurface,
        #[holder(use_place_holder)]
        pub segments: Vec<Vec<SurfacePatch>>,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = rectangular_trimmed_surface)]
    #[holder(generate_deserialize)]
    pub struct RectangularTrimmedSurface {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub bounded_surface: BoundedSurface,
        #[holder(use_place_holder)]
        pub basis_surface: SurfaceAny,
        #[holder(use_place_holder)]
        pub u1: ParameterValue,
        #[holder(use_place_holder)]
        pub u2: ParameterValue,
        #[holder(use_place_holder)]
        pub v1: ParameterValue,
        #[holder(use_place_holder)]
        pub v2: ParameterValue,
        pub usense: bool,
        pub vsense: bool,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = reparametrised_composite_curve_segment)]
    #[holder(generate_deserialize)]
    pub struct ReparametrisedCompositeCurveSegment {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub composite_curve_segment: CompositeCurveSegment,
        #[holder(use_place_holder)]
        pub param_length: ParameterValue,
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
        Representation(Box<Representation>),
        #[holder(use_place_holder)]
        DefinitionalRepresentation(Box<DefinitionalRepresentation>),
        #[holder(use_place_holder)]
        ShapeRepresentation(Box<ShapeRepresentationAny>),
    }
    impl Into<RepresentationAny> for Representation {
        fn into(self) -> RepresentationAny {
            RepresentationAny::Representation(Box::new(self))
        }
    }
    impl Into<RepresentationAny> for DefinitionalRepresentation {
        fn into(self) -> RepresentationAny {
            RepresentationAny::DefinitionalRepresentation(Box::new(self.into()))
        }
    }
    impl Into<RepresentationAny> for ShapeRepresentation {
        fn into(self) -> RepresentationAny {
            RepresentationAny::ShapeRepresentation(Box::new(self.into()))
        }
    }
    impl AsRef<Representation> for RepresentationAny {
        fn as_ref(&self) -> &Representation {
            match self {
                RepresentationAny::Representation(x) => x.as_ref(),
                RepresentationAny::DefinitionalRepresentation(x) => (**x).as_ref(),
                RepresentationAny::ShapeRepresentation(x) => (**x).as_ref(),
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
        RepresentationContext(Box<RepresentationContext>),
        #[holder(use_place_holder)]
        GeometricRepresentationContext(Box<GeometricRepresentationContext>),
        #[holder(use_place_holder)]
        GlobalUncertaintyAssignedContext(Box<GlobalUncertaintyAssignedContext>),
        #[holder(use_place_holder)]
        GlobalUnitAssignedContext(Box<GlobalUnitAssignedContext>),
        #[holder(use_place_holder)]
        ParametricRepresentationContext(Box<ParametricRepresentationContext>),
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
    impl Into<RepresentationContextAny> for GlobalUncertaintyAssignedContext {
        fn into(self) -> RepresentationContextAny {
            RepresentationContextAny::GlobalUncertaintyAssignedContext(Box::new(self.into()))
        }
    }
    impl Into<RepresentationContextAny> for GlobalUnitAssignedContext {
        fn into(self) -> RepresentationContextAny {
            RepresentationContextAny::GlobalUnitAssignedContext(Box::new(self.into()))
        }
    }
    impl Into<RepresentationContextAny> for ParametricRepresentationContext {
        fn into(self) -> RepresentationContextAny {
            RepresentationContextAny::ParametricRepresentationContext(Box::new(self.into()))
        }
    }
    impl AsRef<RepresentationContext> for RepresentationContextAny {
        fn as_ref(&self) -> &RepresentationContext {
            match self {
                RepresentationContextAny::RepresentationContext(x) => x.as_ref(),
                RepresentationContextAny::GeometricRepresentationContext(x) => (**x).as_ref(),
                RepresentationContextAny::GlobalUncertaintyAssignedContext(x) => (**x).as_ref(),
                RepresentationContextAny::GlobalUnitAssignedContext(x) => (**x).as_ref(),
                RepresentationContextAny::ParametricRepresentationContext(x) => (**x).as_ref(),
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
        RepresentationItem(Box<RepresentationItem>),
        #[holder(use_place_holder)]
        GeometricRepresentationItem(Box<GeometricRepresentationItemAny>),
        #[holder(use_place_holder)]
        MappedItem(Box<MappedItem>),
        #[holder(use_place_holder)]
        TopologicalRepresentationItem(Box<TopologicalRepresentationItemAny>),
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
    impl Into<RepresentationItemAny> for TopologicalRepresentationItem {
        fn into(self) -> RepresentationItemAny {
            RepresentationItemAny::TopologicalRepresentationItem(Box::new(self.into()))
        }
    }
    impl AsRef<RepresentationItem> for RepresentationItemAny {
        fn as_ref(&self) -> &RepresentationItem {
            match self {
                RepresentationItemAny::RepresentationItem(x) => x.as_ref(),
                RepresentationItemAny::GeometricRepresentationItem(x) => (**x).as_ref(),
                RepresentationItemAny::MappedItem(x) => (**x).as_ref(),
                RepresentationItemAny::TopologicalRepresentationItem(x) => (**x).as_ref(),
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
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = representation_relationship)]
    #[holder(generate_deserialize)]
    pub struct RepresentationRelationship {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub description: Text,
        #[holder(use_place_holder)]
        pub rep_1: RepresentationAny,
        #[holder(use_place_holder)]
        pub rep_2: RepresentationAny,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum RepresentationRelationshipAny {
        #[holder(use_place_holder)]
        RepresentationRelationship(Box<RepresentationRelationship>),
        #[holder(use_place_holder)]
        RepresentationRelationshipWithTransformation(
            Box<RepresentationRelationshipWithTransformation>,
        ),
        #[holder(use_place_holder)]
        ShapeRepresentationRelationship(Box<ShapeRepresentationRelationship>),
    }
    impl Into<RepresentationRelationshipAny> for RepresentationRelationship {
        fn into(self) -> RepresentationRelationshipAny {
            RepresentationRelationshipAny::RepresentationRelationship(Box::new(self))
        }
    }
    impl Into<RepresentationRelationshipAny> for RepresentationRelationshipWithTransformation {
        fn into(self) -> RepresentationRelationshipAny {
            RepresentationRelationshipAny::RepresentationRelationshipWithTransformation(Box::new(
                self.into(),
            ))
        }
    }
    impl Into<RepresentationRelationshipAny> for ShapeRepresentationRelationship {
        fn into(self) -> RepresentationRelationshipAny {
            RepresentationRelationshipAny::ShapeRepresentationRelationship(Box::new(self.into()))
        }
    }
    impl AsRef<RepresentationRelationship> for RepresentationRelationshipAny {
        fn as_ref(&self) -> &RepresentationRelationship {
            match self {
                RepresentationRelationshipAny::RepresentationRelationship(x) => x.as_ref(),
                RepresentationRelationshipAny::RepresentationRelationshipWithTransformation(x) => {
                    (**x).as_ref()
                }
                RepresentationRelationshipAny::ShapeRepresentationRelationship(x) => (**x).as_ref(),
            }
        }
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = representation_relationship_with_transformation)]
    #[holder(generate_deserialize)]
    pub struct RepresentationRelationshipWithTransformation {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub representation_relationship: RepresentationRelationship,
        #[holder(use_place_holder)]
        pub transformation_operator: Transformation,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = seam_curve)]
    #[holder(generate_deserialize)]
    pub struct SeamCurve {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub surface_curve: SurfaceCurve,
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
        SecurityClassificationAssignment(Box<SecurityClassificationAssignment>),
        #[holder(use_place_holder)]
        CcDesignSecurityClassification(Box<CcDesignSecurityClassification>),
    }
    impl Into<SecurityClassificationAssignmentAny> for SecurityClassificationAssignment {
        fn into(self) -> SecurityClassificationAssignmentAny {
            SecurityClassificationAssignmentAny::SecurityClassificationAssignment(Box::new(self))
        }
    }
    impl Into<SecurityClassificationAssignmentAny> for CcDesignSecurityClassification {
        fn into(self) -> SecurityClassificationAssignmentAny {
            SecurityClassificationAssignmentAny::CcDesignSecurityClassification(Box::new(
                self.into(),
            ))
        }
    }
    impl AsRef<SecurityClassificationAssignment> for SecurityClassificationAssignmentAny {
        fn as_ref(&self) -> &SecurityClassificationAssignment {
            match self {
                SecurityClassificationAssignmentAny::SecurityClassificationAssignment(x) => {
                    x.as_ref()
                }
                SecurityClassificationAssignmentAny::CcDesignSecurityClassification(x) => {
                    (**x).as_ref()
                }
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
    # [holder (field = serial_numbered_effectivity)]
    #[holder(generate_deserialize)]
    pub struct SerialNumberedEffectivity {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub effectivity: Effectivity,
        #[holder(use_place_holder)]
        pub effectivity_start_id: Identifier,
        #[holder(use_place_holder)]
        pub effectivity_end_id: Option<Identifier>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = shape_aspect)]
    #[holder(generate_deserialize)]
    pub struct ShapeAspect {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub description: Text,
        #[holder(use_place_holder)]
        pub of_shape: ProductDefinitionShape,
        pub product_definitional: Logical,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = shape_aspect_relationship)]
    #[holder(generate_deserialize)]
    pub struct ShapeAspectRelationship {
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub description: Text,
        #[holder(use_place_holder)]
        pub relating_shape_aspect: ShapeAspect,
        #[holder(use_place_holder)]
        pub related_shape_aspect: ShapeAspect,
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
        ShapeRepresentation(Box<ShapeRepresentation>),
        #[holder(use_place_holder)]
        AdvancedBrepShapeRepresentation(Box<AdvancedBrepShapeRepresentation>),
        #[holder(use_place_holder)]
        EdgeBasedWireframeShapeRepresentation(Box<EdgeBasedWireframeShapeRepresentation>),
        #[holder(use_place_holder)]
        FacetedBrepShapeRepresentation(Box<FacetedBrepShapeRepresentation>),
        #[holder(use_place_holder)]
        GeometricallyBoundedSurfaceShapeRepresentation(
            Box<GeometricallyBoundedSurfaceShapeRepresentation>,
        ),
        #[holder(use_place_holder)]
        GeometricallyBoundedWireframeShapeRepresentation(
            Box<GeometricallyBoundedWireframeShapeRepresentation>,
        ),
        #[holder(use_place_holder)]
        ManifoldSurfaceShapeRepresentation(Box<ManifoldSurfaceShapeRepresentation>),
        #[holder(use_place_holder)]
        ShellBasedWireframeShapeRepresentation(Box<ShellBasedWireframeShapeRepresentation>),
    }
    impl Into<ShapeRepresentationAny> for ShapeRepresentation {
        fn into(self) -> ShapeRepresentationAny {
            ShapeRepresentationAny::ShapeRepresentation(Box::new(self))
        }
    }
    impl Into<ShapeRepresentationAny> for AdvancedBrepShapeRepresentation {
        fn into(self) -> ShapeRepresentationAny {
            ShapeRepresentationAny::AdvancedBrepShapeRepresentation(Box::new(self.into()))
        }
    }
    impl Into<ShapeRepresentationAny> for EdgeBasedWireframeShapeRepresentation {
        fn into(self) -> ShapeRepresentationAny {
            ShapeRepresentationAny::EdgeBasedWireframeShapeRepresentation(Box::new(self.into()))
        }
    }
    impl Into<ShapeRepresentationAny> for FacetedBrepShapeRepresentation {
        fn into(self) -> ShapeRepresentationAny {
            ShapeRepresentationAny::FacetedBrepShapeRepresentation(Box::new(self.into()))
        }
    }
    impl Into<ShapeRepresentationAny> for GeometricallyBoundedSurfaceShapeRepresentation {
        fn into(self) -> ShapeRepresentationAny {
            ShapeRepresentationAny::GeometricallyBoundedSurfaceShapeRepresentation(Box::new(
                self.into(),
            ))
        }
    }
    impl Into<ShapeRepresentationAny> for GeometricallyBoundedWireframeShapeRepresentation {
        fn into(self) -> ShapeRepresentationAny {
            ShapeRepresentationAny::GeometricallyBoundedWireframeShapeRepresentation(Box::new(
                self.into(),
            ))
        }
    }
    impl Into<ShapeRepresentationAny> for ManifoldSurfaceShapeRepresentation {
        fn into(self) -> ShapeRepresentationAny {
            ShapeRepresentationAny::ManifoldSurfaceShapeRepresentation(Box::new(self.into()))
        }
    }
    impl Into<ShapeRepresentationAny> for ShellBasedWireframeShapeRepresentation {
        fn into(self) -> ShapeRepresentationAny {
            ShapeRepresentationAny::ShellBasedWireframeShapeRepresentation(Box::new(self.into()))
        }
    }
    impl AsRef<ShapeRepresentation> for ShapeRepresentationAny {
        fn as_ref(&self) -> &ShapeRepresentation {
            match self {
                ShapeRepresentationAny::ShapeRepresentation(x) => x.as_ref(),
                ShapeRepresentationAny::AdvancedBrepShapeRepresentation(x) => (**x).as_ref(),
                ShapeRepresentationAny::EdgeBasedWireframeShapeRepresentation(x) => (**x).as_ref(),
                ShapeRepresentationAny::FacetedBrepShapeRepresentation(x) => (**x).as_ref(),
                ShapeRepresentationAny::GeometricallyBoundedSurfaceShapeRepresentation(x) => {
                    (**x).as_ref()
                }
                ShapeRepresentationAny::GeometricallyBoundedWireframeShapeRepresentation(x) => {
                    (**x).as_ref()
                }
                ShapeRepresentationAny::ManifoldSurfaceShapeRepresentation(x) => (**x).as_ref(),
                ShapeRepresentationAny::ShellBasedWireframeShapeRepresentation(x) => (**x).as_ref(),
            }
        }
    }
    impl AsRef<Representation> for ShapeRepresentationAny {
        fn as_ref(&self) -> &Representation {
            match self {
                ShapeRepresentationAny::ShapeRepresentation(x) => {
                    AsRef::<ShapeRepresentation>::as_ref(x).as_ref()
                }
                ShapeRepresentationAny::AdvancedBrepShapeRepresentation(x) => {
                    AsRef::<ShapeRepresentation>::as_ref(x.as_ref()).as_ref()
                }
                ShapeRepresentationAny::EdgeBasedWireframeShapeRepresentation(x) => {
                    AsRef::<ShapeRepresentation>::as_ref(x.as_ref()).as_ref()
                }
                ShapeRepresentationAny::FacetedBrepShapeRepresentation(x) => {
                    AsRef::<ShapeRepresentation>::as_ref(x.as_ref()).as_ref()
                }
                ShapeRepresentationAny::GeometricallyBoundedSurfaceShapeRepresentation(x) => {
                    AsRef::<ShapeRepresentation>::as_ref(x.as_ref()).as_ref()
                }
                ShapeRepresentationAny::GeometricallyBoundedWireframeShapeRepresentation(x) => {
                    AsRef::<ShapeRepresentation>::as_ref(x.as_ref()).as_ref()
                }
                ShapeRepresentationAny::ManifoldSurfaceShapeRepresentation(x) => {
                    AsRef::<ShapeRepresentation>::as_ref(x.as_ref()).as_ref()
                }
                ShapeRepresentationAny::ShellBasedWireframeShapeRepresentation(x) => {
                    AsRef::<ShapeRepresentation>::as_ref(x.as_ref()).as_ref()
                }
            }
        }
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = shape_representation_relationship)]
    #[holder(generate_deserialize)]
    pub struct ShapeRepresentationRelationship {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub representation_relationship: RepresentationRelationship,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = shell_based_surface_model)]
    #[holder(generate_deserialize)]
    pub struct ShellBasedSurfaceModel {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub geometric_representation_item: GeometricRepresentationItem,
        #[holder(use_place_holder)]
        pub sbsm_boundary: Vec<Shell>,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = shell_based_wireframe_model)]
    #[holder(generate_deserialize)]
    pub struct ShellBasedWireframeModel {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub geometric_representation_item: GeometricRepresentationItem,
        #[holder(use_place_holder)]
        pub sbwm_boundary: Vec<Shell>,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = shell_based_wireframe_shape_representation)]
    #[holder(generate_deserialize)]
    pub struct ShellBasedWireframeShapeRepresentation {
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
    # [holder (field = solid_angle_measure_with_unit)]
    #[holder(generate_deserialize)]
    pub struct SolidAngleMeasureWithUnit {
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
    # [holder (field = solid_angle_unit)]
    #[holder(generate_deserialize)]
    pub struct SolidAngleUnit {
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
    # [holder (field = solid_model)]
    #[holder(generate_deserialize)]
    pub struct SolidModel {
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
    pub enum SolidModelAny {
        #[holder(use_place_holder)]
        SolidModel(Box<SolidModel>),
        #[holder(use_place_holder)]
        ManifoldSolidBrep(Box<ManifoldSolidBrepAny>),
    }
    impl Into<SolidModelAny> for SolidModel {
        fn into(self) -> SolidModelAny {
            SolidModelAny::SolidModel(Box::new(self))
        }
    }
    impl Into<SolidModelAny> for ManifoldSolidBrep {
        fn into(self) -> SolidModelAny {
            SolidModelAny::ManifoldSolidBrep(Box::new(self.into()))
        }
    }
    impl AsRef<SolidModel> for SolidModelAny {
        fn as_ref(&self) -> &SolidModel {
            match self {
                SolidModelAny::SolidModel(x) => x.as_ref(),
                SolidModelAny::ManifoldSolidBrep(x) => (**x).as_ref(),
            }
        }
    }
    impl AsRef<GeometricRepresentationItem> for SolidModelAny {
        fn as_ref(&self) -> &GeometricRepresentationItem {
            match self {
                SolidModelAny::SolidModel(x) => AsRef::<SolidModel>::as_ref(x).as_ref(),
                SolidModelAny::ManifoldSolidBrep(x) => {
                    AsRef::<SolidModel>::as_ref(x.as_ref()).as_ref()
                }
            }
        }
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = specified_higher_usage_occurrence)]
    #[holder(generate_deserialize)]
    pub struct SpecifiedHigherUsageOccurrence {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub assembly_component_usage: AssemblyComponentUsage,
        #[holder(use_place_holder)]
        pub upper_usage: AssemblyComponentUsageAny,
        #[holder(use_place_holder)]
        pub next_usage: NextAssemblyUsageOccurrence,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = spherical_surface)]
    #[holder(generate_deserialize)]
    pub struct SphericalSurface {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub elementary_surface: ElementarySurface,
        #[holder(use_place_holder)]
        pub radius: PositiveLengthMeasure,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = start_request)]
    #[holder(generate_deserialize)]
    pub struct StartRequest {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub action_request_assignment: ActionRequestAssignment,
        #[holder(use_place_holder)]
        pub items: Vec<StartRequestItem>,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = start_work)]
    #[holder(generate_deserialize)]
    pub struct StartWork {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub action_assignment: ActionAssignment,
        #[holder(use_place_holder)]
        pub items: Vec<WorkItem>,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = supplied_part_relationship)]
    #[holder(generate_deserialize)]
    pub struct SuppliedPartRelationship {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub product_definition_relationship: ProductDefinitionRelationship,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = surface)]
    #[holder(generate_deserialize)]
    pub struct Surface {
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
    pub enum SurfaceAny {
        #[holder(use_place_holder)]
        Surface(Box<Surface>),
        #[holder(use_place_holder)]
        BoundedSurface(Box<BoundedSurfaceAny>),
        #[holder(use_place_holder)]
        ElementarySurface(Box<ElementarySurfaceAny>),
        #[holder(use_place_holder)]
        OffsetSurface(Box<OffsetSurface>),
        #[holder(use_place_holder)]
        SurfaceReplica(Box<SurfaceReplica>),
        #[holder(use_place_holder)]
        SweptSurface(Box<SweptSurfaceAny>),
    }
    impl Into<SurfaceAny> for Surface {
        fn into(self) -> SurfaceAny {
            SurfaceAny::Surface(Box::new(self))
        }
    }
    impl Into<SurfaceAny> for BoundedSurface {
        fn into(self) -> SurfaceAny {
            SurfaceAny::BoundedSurface(Box::new(self.into()))
        }
    }
    impl Into<SurfaceAny> for ElementarySurface {
        fn into(self) -> SurfaceAny {
            SurfaceAny::ElementarySurface(Box::new(self.into()))
        }
    }
    impl Into<SurfaceAny> for OffsetSurface {
        fn into(self) -> SurfaceAny {
            SurfaceAny::OffsetSurface(Box::new(self.into()))
        }
    }
    impl Into<SurfaceAny> for SurfaceReplica {
        fn into(self) -> SurfaceAny {
            SurfaceAny::SurfaceReplica(Box::new(self.into()))
        }
    }
    impl Into<SurfaceAny> for SweptSurface {
        fn into(self) -> SurfaceAny {
            SurfaceAny::SweptSurface(Box::new(self.into()))
        }
    }
    impl AsRef<Surface> for SurfaceAny {
        fn as_ref(&self) -> &Surface {
            match self {
                SurfaceAny::Surface(x) => x.as_ref(),
                SurfaceAny::BoundedSurface(x) => (**x).as_ref(),
                SurfaceAny::ElementarySurface(x) => (**x).as_ref(),
                SurfaceAny::OffsetSurface(x) => (**x).as_ref(),
                SurfaceAny::SurfaceReplica(x) => (**x).as_ref(),
                SurfaceAny::SweptSurface(x) => (**x).as_ref(),
            }
        }
    }
    impl AsRef<GeometricRepresentationItem> for SurfaceAny {
        fn as_ref(&self) -> &GeometricRepresentationItem {
            match self {
                SurfaceAny::Surface(x) => AsRef::<Surface>::as_ref(x).as_ref(),
                SurfaceAny::BoundedSurface(x) => AsRef::<Surface>::as_ref(x.as_ref()).as_ref(),
                SurfaceAny::ElementarySurface(x) => AsRef::<Surface>::as_ref(x.as_ref()).as_ref(),
                SurfaceAny::OffsetSurface(x) => AsRef::<Surface>::as_ref(x.as_ref()).as_ref(),
                SurfaceAny::SurfaceReplica(x) => AsRef::<Surface>::as_ref(x.as_ref()).as_ref(),
                SurfaceAny::SweptSurface(x) => AsRef::<Surface>::as_ref(x.as_ref()).as_ref(),
            }
        }
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = surface_curve)]
    #[holder(generate_deserialize)]
    pub struct SurfaceCurve {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub curve: Curve,
        #[holder(use_place_holder)]
        pub curve_3d: CurveAny,
        #[holder(use_place_holder)]
        pub associated_geometry: Vec<PcurveOrSurface>,
        pub master_representation: PreferredSurfaceCurveRepresentation,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum SurfaceCurveAny {
        #[holder(use_place_holder)]
        SurfaceCurve(Box<SurfaceCurve>),
        #[holder(use_place_holder)]
        BoundedSurfaceCurve(Box<BoundedSurfaceCurve>),
        #[holder(use_place_holder)]
        IntersectionCurve(Box<IntersectionCurve>),
        #[holder(use_place_holder)]
        SeamCurve(Box<SeamCurve>),
    }
    impl Into<SurfaceCurveAny> for SurfaceCurve {
        fn into(self) -> SurfaceCurveAny {
            SurfaceCurveAny::SurfaceCurve(Box::new(self))
        }
    }
    impl Into<SurfaceCurveAny> for BoundedSurfaceCurve {
        fn into(self) -> SurfaceCurveAny {
            SurfaceCurveAny::BoundedSurfaceCurve(Box::new(self.into()))
        }
    }
    impl Into<SurfaceCurveAny> for IntersectionCurve {
        fn into(self) -> SurfaceCurveAny {
            SurfaceCurveAny::IntersectionCurve(Box::new(self.into()))
        }
    }
    impl Into<SurfaceCurveAny> for SeamCurve {
        fn into(self) -> SurfaceCurveAny {
            SurfaceCurveAny::SeamCurve(Box::new(self.into()))
        }
    }
    impl AsRef<SurfaceCurve> for SurfaceCurveAny {
        fn as_ref(&self) -> &SurfaceCurve {
            match self {
                SurfaceCurveAny::SurfaceCurve(x) => x.as_ref(),
                SurfaceCurveAny::BoundedSurfaceCurve(x) => (**x).as_ref(),
                SurfaceCurveAny::IntersectionCurve(x) => (**x).as_ref(),
                SurfaceCurveAny::SeamCurve(x) => (**x).as_ref(),
            }
        }
    }
    impl AsRef<Curve> for SurfaceCurveAny {
        fn as_ref(&self) -> &Curve {
            match self {
                SurfaceCurveAny::SurfaceCurve(x) => AsRef::<SurfaceCurve>::as_ref(x).as_ref(),
                SurfaceCurveAny::BoundedSurfaceCurve(x) => {
                    AsRef::<SurfaceCurve>::as_ref(x.as_ref()).as_ref()
                }
                SurfaceCurveAny::IntersectionCurve(x) => {
                    AsRef::<SurfaceCurve>::as_ref(x.as_ref()).as_ref()
                }
                SurfaceCurveAny::SeamCurve(x) => AsRef::<SurfaceCurve>::as_ref(x.as_ref()).as_ref(),
            }
        }
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = surface_of_linear_extrusion)]
    #[holder(generate_deserialize)]
    pub struct SurfaceOfLinearExtrusion {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub swept_surface: SweptSurface,
        #[holder(use_place_holder)]
        pub extrusion_axis: Vector,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = surface_of_revolution)]
    #[holder(generate_deserialize)]
    pub struct SurfaceOfRevolution {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub swept_surface: SweptSurface,
        #[holder(use_place_holder)]
        pub axis_position: Axis1Placement,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = surface_patch)]
    #[holder(generate_deserialize)]
    pub struct SurfacePatch {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub founded_item: FoundedItem,
        #[holder(use_place_holder)]
        pub parent_surface: BoundedSurfaceAny,
        pub u_transition: TransitionCode,
        pub v_transition: TransitionCode,
        pub u_sense: bool,
        pub v_sense: bool,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = surface_replica)]
    #[holder(generate_deserialize)]
    pub struct SurfaceReplica {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub surface: Surface,
        #[holder(use_place_holder)]
        pub parent_surface: SurfaceAny,
        #[holder(use_place_holder)]
        pub transformation: CartesianTransformationOperator3D,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = swept_surface)]
    #[holder(generate_deserialize)]
    pub struct SweptSurface {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub surface: Surface,
        #[holder(use_place_holder)]
        pub swept_curve: CurveAny,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum SweptSurfaceAny {
        #[holder(use_place_holder)]
        SweptSurface(Box<SweptSurface>),
        #[holder(use_place_holder)]
        SurfaceOfLinearExtrusion(Box<SurfaceOfLinearExtrusion>),
        #[holder(use_place_holder)]
        SurfaceOfRevolution(Box<SurfaceOfRevolution>),
    }
    impl Into<SweptSurfaceAny> for SweptSurface {
        fn into(self) -> SweptSurfaceAny {
            SweptSurfaceAny::SweptSurface(Box::new(self))
        }
    }
    impl Into<SweptSurfaceAny> for SurfaceOfLinearExtrusion {
        fn into(self) -> SweptSurfaceAny {
            SweptSurfaceAny::SurfaceOfLinearExtrusion(Box::new(self.into()))
        }
    }
    impl Into<SweptSurfaceAny> for SurfaceOfRevolution {
        fn into(self) -> SweptSurfaceAny {
            SweptSurfaceAny::SurfaceOfRevolution(Box::new(self.into()))
        }
    }
    impl AsRef<SweptSurface> for SweptSurfaceAny {
        fn as_ref(&self) -> &SweptSurface {
            match self {
                SweptSurfaceAny::SweptSurface(x) => x.as_ref(),
                SweptSurfaceAny::SurfaceOfLinearExtrusion(x) => (**x).as_ref(),
                SweptSurfaceAny::SurfaceOfRevolution(x) => (**x).as_ref(),
            }
        }
    }
    impl AsRef<Surface> for SweptSurfaceAny {
        fn as_ref(&self) -> &Surface {
            match self {
                SweptSurfaceAny::SweptSurface(x) => AsRef::<SweptSurface>::as_ref(x).as_ref(),
                SweptSurfaceAny::SurfaceOfLinearExtrusion(x) => {
                    AsRef::<SweptSurface>::as_ref(x.as_ref()).as_ref()
                }
                SweptSurfaceAny::SurfaceOfRevolution(x) => {
                    AsRef::<SweptSurface>::as_ref(x.as_ref()).as_ref()
                }
            }
        }
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = topological_representation_item)]
    #[holder(generate_deserialize)]
    pub struct TopologicalRepresentationItem {
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
    pub enum TopologicalRepresentationItemAny {
        #[holder(use_place_holder)]
        TopologicalRepresentationItem(Box<TopologicalRepresentationItem>),
        #[holder(use_place_holder)]
        ConnectedEdgeSet(Box<ConnectedEdgeSet>),
        #[holder(use_place_holder)]
        ConnectedFaceSet(Box<ConnectedFaceSetAny>),
        #[holder(use_place_holder)]
        Edge(Box<EdgeAny>),
        #[holder(use_place_holder)]
        Face(Box<FaceAny>),
        #[holder(use_place_holder)]
        FaceBound(Box<FaceBoundAny>),
        #[holder(use_place_holder)]
        Loop(Box<LoopAny>),
        #[holder(use_place_holder)]
        Path(Box<PathAny>),
        #[holder(use_place_holder)]
        Vertex(Box<VertexAny>),
        #[holder(use_place_holder)]
        VertexShell(Box<VertexShell>),
        #[holder(use_place_holder)]
        WireShell(Box<WireShell>),
    }
    impl Into<TopologicalRepresentationItemAny> for TopologicalRepresentationItem {
        fn into(self) -> TopologicalRepresentationItemAny {
            TopologicalRepresentationItemAny::TopologicalRepresentationItem(Box::new(self))
        }
    }
    impl Into<TopologicalRepresentationItemAny> for ConnectedEdgeSet {
        fn into(self) -> TopologicalRepresentationItemAny {
            TopologicalRepresentationItemAny::ConnectedEdgeSet(Box::new(self.into()))
        }
    }
    impl Into<TopologicalRepresentationItemAny> for ConnectedFaceSet {
        fn into(self) -> TopologicalRepresentationItemAny {
            TopologicalRepresentationItemAny::ConnectedFaceSet(Box::new(self.into()))
        }
    }
    impl Into<TopologicalRepresentationItemAny> for Edge {
        fn into(self) -> TopologicalRepresentationItemAny {
            TopologicalRepresentationItemAny::Edge(Box::new(self.into()))
        }
    }
    impl Into<TopologicalRepresentationItemAny> for Face {
        fn into(self) -> TopologicalRepresentationItemAny {
            TopologicalRepresentationItemAny::Face(Box::new(self.into()))
        }
    }
    impl Into<TopologicalRepresentationItemAny> for FaceBound {
        fn into(self) -> TopologicalRepresentationItemAny {
            TopologicalRepresentationItemAny::FaceBound(Box::new(self.into()))
        }
    }
    impl Into<TopologicalRepresentationItemAny> for Loop {
        fn into(self) -> TopologicalRepresentationItemAny {
            TopologicalRepresentationItemAny::Loop(Box::new(self.into()))
        }
    }
    impl Into<TopologicalRepresentationItemAny> for Path {
        fn into(self) -> TopologicalRepresentationItemAny {
            TopologicalRepresentationItemAny::Path(Box::new(self.into()))
        }
    }
    impl Into<TopologicalRepresentationItemAny> for Vertex {
        fn into(self) -> TopologicalRepresentationItemAny {
            TopologicalRepresentationItemAny::Vertex(Box::new(self.into()))
        }
    }
    impl Into<TopologicalRepresentationItemAny> for VertexShell {
        fn into(self) -> TopologicalRepresentationItemAny {
            TopologicalRepresentationItemAny::VertexShell(Box::new(self.into()))
        }
    }
    impl Into<TopologicalRepresentationItemAny> for WireShell {
        fn into(self) -> TopologicalRepresentationItemAny {
            TopologicalRepresentationItemAny::WireShell(Box::new(self.into()))
        }
    }
    impl AsRef<TopologicalRepresentationItem> for TopologicalRepresentationItemAny {
        fn as_ref(&self) -> &TopologicalRepresentationItem {
            match self {
                TopologicalRepresentationItemAny::TopologicalRepresentationItem(x) => x.as_ref(),
                TopologicalRepresentationItemAny::ConnectedEdgeSet(x) => (**x).as_ref(),
                TopologicalRepresentationItemAny::ConnectedFaceSet(x) => (**x).as_ref(),
                TopologicalRepresentationItemAny::Edge(x) => (**x).as_ref(),
                TopologicalRepresentationItemAny::Face(x) => (**x).as_ref(),
                TopologicalRepresentationItemAny::FaceBound(x) => (**x).as_ref(),
                TopologicalRepresentationItemAny::Loop(x) => (**x).as_ref(),
                TopologicalRepresentationItemAny::Path(x) => (**x).as_ref(),
                TopologicalRepresentationItemAny::Vertex(x) => (**x).as_ref(),
                TopologicalRepresentationItemAny::VertexShell(x) => (**x).as_ref(),
                TopologicalRepresentationItemAny::WireShell(x) => (**x).as_ref(),
            }
        }
    }
    impl AsRef<RepresentationItem> for TopologicalRepresentationItemAny {
        fn as_ref(&self) -> &RepresentationItem {
            match self {
                TopologicalRepresentationItemAny::TopologicalRepresentationItem(x) => {
                    AsRef::<TopologicalRepresentationItem>::as_ref(x).as_ref()
                }
                TopologicalRepresentationItemAny::ConnectedEdgeSet(x) => {
                    AsRef::<TopologicalRepresentationItem>::as_ref(x.as_ref()).as_ref()
                }
                TopologicalRepresentationItemAny::ConnectedFaceSet(x) => {
                    AsRef::<TopologicalRepresentationItem>::as_ref(x.as_ref()).as_ref()
                }
                TopologicalRepresentationItemAny::Edge(x) => {
                    AsRef::<TopologicalRepresentationItem>::as_ref(x.as_ref()).as_ref()
                }
                TopologicalRepresentationItemAny::Face(x) => {
                    AsRef::<TopologicalRepresentationItem>::as_ref(x.as_ref()).as_ref()
                }
                TopologicalRepresentationItemAny::FaceBound(x) => {
                    AsRef::<TopologicalRepresentationItem>::as_ref(x.as_ref()).as_ref()
                }
                TopologicalRepresentationItemAny::Loop(x) => {
                    AsRef::<TopologicalRepresentationItem>::as_ref(x.as_ref()).as_ref()
                }
                TopologicalRepresentationItemAny::Path(x) => {
                    AsRef::<TopologicalRepresentationItem>::as_ref(x.as_ref()).as_ref()
                }
                TopologicalRepresentationItemAny::Vertex(x) => {
                    AsRef::<TopologicalRepresentationItem>::as_ref(x.as_ref()).as_ref()
                }
                TopologicalRepresentationItemAny::VertexShell(x) => {
                    AsRef::<TopologicalRepresentationItem>::as_ref(x.as_ref()).as_ref()
                }
                TopologicalRepresentationItemAny::WireShell(x) => {
                    AsRef::<TopologicalRepresentationItem>::as_ref(x.as_ref()).as_ref()
                }
            }
        }
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = toroidal_surface)]
    #[holder(generate_deserialize)]
    pub struct ToroidalSurface {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub elementary_surface: ElementarySurface,
        #[holder(use_place_holder)]
        pub major_radius: PositiveLengthMeasure,
        #[holder(use_place_holder)]
        pub minor_radius: PositiveLengthMeasure,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ToroidalSurfaceAny {
        #[holder(use_place_holder)]
        ToroidalSurface(Box<ToroidalSurface>),
        #[holder(use_place_holder)]
        DegenerateToroidalSurface(Box<DegenerateToroidalSurface>),
    }
    impl Into<ToroidalSurfaceAny> for ToroidalSurface {
        fn into(self) -> ToroidalSurfaceAny {
            ToroidalSurfaceAny::ToroidalSurface(Box::new(self))
        }
    }
    impl Into<ToroidalSurfaceAny> for DegenerateToroidalSurface {
        fn into(self) -> ToroidalSurfaceAny {
            ToroidalSurfaceAny::DegenerateToroidalSurface(Box::new(self.into()))
        }
    }
    impl AsRef<ToroidalSurface> for ToroidalSurfaceAny {
        fn as_ref(&self) -> &ToroidalSurface {
            match self {
                ToroidalSurfaceAny::ToroidalSurface(x) => x.as_ref(),
                ToroidalSurfaceAny::DegenerateToroidalSurface(x) => (**x).as_ref(),
            }
        }
    }
    impl AsRef<ElementarySurface> for ToroidalSurfaceAny {
        fn as_ref(&self) -> &ElementarySurface {
            match self {
                ToroidalSurfaceAny::ToroidalSurface(x) => {
                    AsRef::<ToroidalSurface>::as_ref(x).as_ref()
                }
                ToroidalSurfaceAny::DegenerateToroidalSurface(x) => {
                    AsRef::<ToroidalSurface>::as_ref(x.as_ref()).as_ref()
                }
            }
        }
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
    # [holder (field = uncertainty_measure_with_unit)]
    #[holder(generate_deserialize)]
    pub struct UncertaintyMeasureWithUnit {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub measure_with_unit: MeasureWithUnit,
        #[holder(use_place_holder)]
        pub name: Label,
        #[holder(use_place_holder)]
        pub description: Text,
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
    # [holder (field = uniform_surface)]
    #[holder(generate_deserialize)]
    pub struct UniformSurface {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub b_spline_surface: BSplineSurface,
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
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = versioned_action_request)]
    #[holder(generate_deserialize)]
    pub struct VersionedActionRequest {
        #[holder(use_place_holder)]
        pub id: Identifier,
        #[holder(use_place_holder)]
        pub version: Label,
        #[holder(use_place_holder)]
        pub purpose: Text,
        #[holder(use_place_holder)]
        pub description: Text,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = vertex)]
    #[holder(generate_deserialize)]
    pub struct Vertex {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub topological_representation_item: TopologicalRepresentationItem,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum VertexAny {
        #[holder(use_place_holder)]
        Vertex(Box<Vertex>),
        #[holder(use_place_holder)]
        VertexPoint(Box<VertexPoint>),
    }
    impl Into<VertexAny> for Vertex {
        fn into(self) -> VertexAny {
            VertexAny::Vertex(Box::new(self))
        }
    }
    impl Into<VertexAny> for VertexPoint {
        fn into(self) -> VertexAny {
            VertexAny::VertexPoint(Box::new(self.into()))
        }
    }
    impl AsRef<Vertex> for VertexAny {
        fn as_ref(&self) -> &Vertex {
            match self {
                VertexAny::Vertex(x) => x.as_ref(),
                VertexAny::VertexPoint(x) => (**x).as_ref(),
            }
        }
    }
    impl AsRef<TopologicalRepresentationItem> for VertexAny {
        fn as_ref(&self) -> &TopologicalRepresentationItem {
            match self {
                VertexAny::Vertex(x) => AsRef::<Vertex>::as_ref(x).as_ref(),
                VertexAny::VertexPoint(x) => AsRef::<Vertex>::as_ref(x.as_ref()).as_ref(),
            }
        }
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = vertex_loop)]
    #[holder(generate_deserialize)]
    pub struct VertexLoop {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub r#loop: Loop,
        #[holder(use_place_holder)]
        pub loop_vertex: VertexAny,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut)]
    # [holder (table = Tables)]
    # [holder (field = vertex_point)]
    #[holder(generate_deserialize)]
    pub struct VertexPoint {
        #[as_ref]
        #[as_mut]
        #[holder(use_place_holder)]
        pub vertex: Vertex,
        #[as_ref]
        #[as_mut]
        #[holder(use_place_holder)]
        pub geometric_representation_item: GeometricRepresentationItem,
        #[holder(use_place_holder)]
        pub vertex_geometry: PointAny,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = vertex_shell)]
    #[holder(generate_deserialize)]
    pub struct VertexShell {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub topological_representation_item: TopologicalRepresentationItem,
        #[holder(use_place_holder)]
        pub vertex_shell_extent: VertexLoop,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = volume_measure_with_unit)]
    #[holder(generate_deserialize)]
    pub struct VolumeMeasureWithUnit {
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
    # [holder (field = volume_unit)]
    #[holder(generate_deserialize)]
    pub struct VolumeUnit {
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
    # [holder (field = week_of_year_and_day_date)]
    #[holder(generate_deserialize)]
    pub struct WeekOfYearAndDayDate {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub date: Date,
        #[holder(use_place_holder)]
        pub week_component: WeekInYearNumber,
        #[holder(use_place_holder)]
        pub day_component: Option<DayInWeekNumber>,
    }
    #[derive(
        Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
    )]
    # [holder (table = Tables)]
    # [holder (field = wire_shell)]
    #[holder(generate_deserialize)]
    pub struct WireShell {
        #[as_ref]
        #[as_mut]
        #[deref]
        #[deref_mut]
        #[holder(use_place_holder)]
        pub topological_representation_item: TopologicalRepresentationItem,
        #[holder(use_place_holder)]
        pub wire_shell_extent: Vec<LoopAny>,
    }
}

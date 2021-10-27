#![allow(dead_code)]
pub mod config_control_design {
    use crate::{error::Result, primitive::*, tables::*};
    use ruststep_derive::as_holder;
    use std::collections::HashMap;
    #[derive(Debug, Clone, PartialEq, Default)]
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
    }
    impl Tables {
        pub fn action_iter<'table>(&'table self) -> impl Iterator<Item = Result<Action>> + 'table {
            self.action
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn action_assignment_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ActionAssignment>> + 'table {
            self.action_assignment
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn action_directive_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ActionDirective>> + 'table {
            self.action_directive
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn action_method_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ActionMethod>> + 'table {
            self.action_method
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn action_request_assignment_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ActionRequestAssignment>> + 'table {
            self.action_request_assignment
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn action_request_solution_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ActionRequestSolution>> + 'table {
            self.action_request_solution
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn action_request_status_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ActionRequestStatus>> + 'table {
            self.action_request_status
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn action_status_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ActionStatus>> + 'table {
            self.action_status
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn address_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<Address>> + 'table {
            self.address
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn advanced_brep_shape_representation_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<AdvancedBrepShapeRepresentation>> + 'table {
            self.advanced_brep_shape_representation
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn advanced_face_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<AdvancedFace>> + 'table {
            self.advanced_face
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn alternate_product_relationship_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<AlternateProductRelationship>> + 'table {
            self.alternate_product_relationship
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
        pub fn approval_relationship_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ApprovalRelationship>> + 'table {
            self.approval_relationship
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
        pub fn area_measure_with_unit_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<AreaMeasureWithUnit>> + 'table {
            self.area_measure_with_unit
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn area_unit_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<AreaUnit>> + 'table {
            self.area_unit
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn assembly_component_usage_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<AssemblyComponentUsage>> + 'table {
            self.assembly_component_usage
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn assembly_component_usage_substitute_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<AssemblyComponentUsageSubstitute>> + 'table {
            self.assembly_component_usage_substitute
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn axis1_placement_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<Axis1Placement>> + 'table {
            self.axis1_placement
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
        pub fn axis2_placement_3d_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<Axis2Placement3D>> + 'table {
            self.axis2_placement_3d
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
        pub fn b_spline_surface_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<BSplineSurface>> + 'table {
            self.b_spline_surface
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn b_spline_surface_with_knots_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<BSplineSurfaceWithKnots>> + 'table {
            self.b_spline_surface_with_knots
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
        pub fn bezier_surface_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<BezierSurface>> + 'table {
            self.bezier_surface
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn boundary_curve_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<BoundaryCurve>> + 'table {
            self.boundary_curve
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
        pub fn bounded_pcurve_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<BoundedPcurve>> + 'table {
            self.bounded_pcurve
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn bounded_surface_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<BoundedSurface>> + 'table {
            self.bounded_surface
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn bounded_surface_curve_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<BoundedSurfaceCurve>> + 'table {
            self.bounded_surface_curve
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn brep_with_voids_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<BrepWithVoids>> + 'table {
            self.brep_with_voids
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
        pub fn cartesian_point_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<CartesianPoint>> + 'table {
            self.cartesian_point
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn cartesian_transformation_operator_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<CartesianTransformationOperator>> + 'table {
            self.cartesian_transformation_operator
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn cartesian_transformation_operator_3d_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<CartesianTransformationOperator3D>> + 'table {
            self.cartesian_transformation_operator_3d
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn cc_design_approval_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<CcDesignApproval>> + 'table {
            self.cc_design_approval
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn cc_design_certification_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<CcDesignCertification>> + 'table {
            self.cc_design_certification
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn cc_design_contract_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<CcDesignContract>> + 'table {
            self.cc_design_contract
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn cc_design_date_and_time_assignment_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<CcDesignDateAndTimeAssignment>> + 'table {
            self.cc_design_date_and_time_assignment
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn cc_design_person_and_organization_assignment_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<CcDesignPersonAndOrganizationAssignment>> + 'table
        {
            self.cc_design_person_and_organization_assignment
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn cc_design_security_classification_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<CcDesignSecurityClassification>> + 'table {
            self.cc_design_security_classification
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn cc_design_specification_reference_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<CcDesignSpecificationReference>> + 'table {
            self.cc_design_specification_reference
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn certification_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<Certification>> + 'table {
            self.certification
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn certification_assignment_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<CertificationAssignment>> + 'table {
            self.certification_assignment
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn certification_type_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<CertificationType>> + 'table {
            self.certification_type
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn change_iter<'table>(&'table self) -> impl Iterator<Item = Result<Change>> + 'table {
            self.change
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn change_request_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ChangeRequest>> + 'table {
            self.change_request
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
        pub fn closed_shell_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ClosedShell>> + 'table {
            self.closed_shell
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
        pub fn composite_curve_on_surface_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<CompositeCurveOnSurface>> + 'table {
            self.composite_curve_on_surface
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
        pub fn configuration_design_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ConfigurationDesign>> + 'table {
            self.configuration_design
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn configuration_effectivity_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ConfigurationEffectivity>> + 'table {
            self.configuration_effectivity
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn configuration_item_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ConfigurationItem>> + 'table {
            self.configuration_item
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
        pub fn conical_surface_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ConicalSurface>> + 'table {
            self.conical_surface
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn connected_edge_set_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ConnectedEdgeSet>> + 'table {
            self.connected_edge_set
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn connected_face_set_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ConnectedFaceSet>> + 'table {
            self.connected_face_set
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn context_dependent_shape_representation_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ContextDependentShapeRepresentation>> + 'table {
            self.context_dependent_shape_representation
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn context_dependent_unit_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ContextDependentUnit>> + 'table {
            self.context_dependent_unit
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
        pub fn coordinated_universal_time_offset_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<CoordinatedUniversalTimeOffset>> + 'table {
            self.coordinated_universal_time_offset
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
        pub fn curve_bounded_surface_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<CurveBoundedSurface>> + 'table {
            self.curve_bounded_surface
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn curve_replica_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<CurveReplica>> + 'table {
            self.curve_replica
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn cylindrical_surface_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<CylindricalSurface>> + 'table {
            self.cylindrical_surface
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
        pub fn date_and_time_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<DateAndTime>> + 'table {
            self.date_and_time
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn date_and_time_assignment_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<DateAndTimeAssignment>> + 'table {
            self.date_and_time_assignment
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn date_time_role_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<DateTimeRole>> + 'table {
            self.date_time_role
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn dated_effectivity_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<DatedEffectivity>> + 'table {
            self.dated_effectivity
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn definitional_representation_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<DefinitionalRepresentation>> + 'table {
            self.definitional_representation
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn degenerate_pcurve_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<DegeneratePcurve>> + 'table {
            self.degenerate_pcurve
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn degenerate_toroidal_surface_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<DegenerateToroidalSurface>> + 'table {
            self.degenerate_toroidal_surface
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn design_context_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<DesignContext>> + 'table {
            self.design_context
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn design_make_from_relationship_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<DesignMakeFromRelationship>> + 'table {
            self.design_make_from_relationship
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
        pub fn directed_action_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<DirectedAction>> + 'table {
            self.directed_action
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
        pub fn document_relationship_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<DocumentRelationship>> + 'table {
            self.document_relationship
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
        pub fn document_usage_constraint_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<DocumentUsageConstraint>> + 'table {
            self.document_usage_constraint
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn document_with_class_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<DocumentWithClass>> + 'table {
            self.document_with_class
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn edge_iter<'table>(&'table self) -> impl Iterator<Item = Result<Edge>> + 'table {
            self.edge
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn edge_based_wireframe_model_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<EdgeBasedWireframeModel>> + 'table {
            self.edge_based_wireframe_model
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn edge_based_wireframe_shape_representation_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<EdgeBasedWireframeShapeRepresentation>> + 'table {
            self.edge_based_wireframe_shape_representation
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn edge_curve_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<EdgeCurve>> + 'table {
            self.edge_curve
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn edge_loop_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<EdgeLoop>> + 'table {
            self.edge_loop
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn effectivity_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<Effectivity>> + 'table {
            self.effectivity
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn elementary_surface_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ElementarySurface>> + 'table {
            self.elementary_surface
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
        pub fn evaluated_degenerate_pcurve_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<EvaluatedDegeneratePcurve>> + 'table {
            self.evaluated_degenerate_pcurve
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn executed_action_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ExecutedAction>> + 'table {
            self.executed_action
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn face_iter<'table>(&'table self) -> impl Iterator<Item = Result<Face>> + 'table {
            self.face
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn face_bound_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<FaceBound>> + 'table {
            self.face_bound
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn face_outer_bound_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<FaceOuterBound>> + 'table {
            self.face_outer_bound
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn face_surface_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<FaceSurface>> + 'table {
            self.face_surface
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn faceted_brep_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<FacetedBrep>> + 'table {
            self.faceted_brep
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn faceted_brep_shape_representation_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<FacetedBrepShapeRepresentation>> + 'table {
            self.faceted_brep_shape_representation
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn founded_item_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<FoundedItem>> + 'table {
            self.founded_item
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn functionally_defined_transformation_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<FunctionallyDefinedTransformation>> + 'table {
            self.functionally_defined_transformation
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
        pub fn geometrically_bounded_surface_shape_representation_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<GeometricallyBoundedSurfaceShapeRepresentation>> + 'table
        {
            self.geometrically_bounded_surface_shape_representation
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn geometrically_bounded_wireframe_shape_representation_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<GeometricallyBoundedWireframeShapeRepresentation>> + 'table
        {
            self.geometrically_bounded_wireframe_shape_representation
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn global_uncertainty_assigned_context_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<GlobalUncertaintyAssignedContext>> + 'table {
            self.global_uncertainty_assigned_context
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
        pub fn hyperbola_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<Hyperbola>> + 'table {
            self.hyperbola
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn intersection_curve_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<IntersectionCurve>> + 'table {
            self.intersection_curve
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn item_defined_transformation_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ItemDefinedTransformation>> + 'table {
            self.item_defined_transformation
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
        pub fn local_time_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<LocalTime>> + 'table {
            self.local_time
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn loop_iter<'table>(&'table self) -> impl Iterator<Item = Result<Loop>> + 'table {
            self.r#loop
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn lot_effectivity_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<LotEffectivity>> + 'table {
            self.lot_effectivity
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn manifold_solid_brep_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ManifoldSolidBrep>> + 'table {
            self.manifold_solid_brep
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn manifold_surface_shape_representation_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ManifoldSurfaceShapeRepresentation>> + 'table {
            self.manifold_surface_shape_representation
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
        pub fn mass_measure_with_unit_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<MassMeasureWithUnit>> + 'table {
            self.mass_measure_with_unit
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn mass_unit_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<MassUnit>> + 'table {
            self.mass_unit
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
        pub fn mechanical_context_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<MechanicalContext>> + 'table {
            self.mechanical_context
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
        pub fn next_assembly_usage_occurrence_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<NextAssemblyUsageOccurrence>> + 'table {
            self.next_assembly_usage_occurrence
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn offset_curve_3d_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<OffsetCurve3D>> + 'table {
            self.offset_curve_3d
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn offset_surface_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<OffsetSurface>> + 'table {
            self.offset_surface
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn open_shell_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<OpenShell>> + 'table {
            self.open_shell
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn ordinal_date_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<OrdinalDate>> + 'table {
            self.ordinal_date
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
        pub fn organization_relationship_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<OrganizationRelationship>> + 'table {
            self.organization_relationship
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
        pub fn organizational_project_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<OrganizationalProject>> + 'table {
            self.organizational_project
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn oriented_closed_shell_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<OrientedClosedShell>> + 'table {
            self.oriented_closed_shell
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn oriented_edge_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<OrientedEdge>> + 'table {
            self.oriented_edge
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn oriented_face_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<OrientedFace>> + 'table {
            self.oriented_face
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn oriented_open_shell_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<OrientedOpenShell>> + 'table {
            self.oriented_open_shell
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn oriented_path_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<OrientedPath>> + 'table {
            self.oriented_path
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn outer_boundary_curve_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<OuterBoundaryCurve>> + 'table {
            self.outer_boundary_curve
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
        pub fn parametric_representation_context_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ParametricRepresentationContext>> + 'table {
            self.parametric_representation_context
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn path_iter<'table>(&'table self) -> impl Iterator<Item = Result<Path>> + 'table {
            self.path
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn pcurve_iter<'table>(&'table self) -> impl Iterator<Item = Result<Pcurve>> + 'table {
            self.pcurve
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
        pub fn plane_iter<'table>(&'table self) -> impl Iterator<Item = Result<Plane>> + 'table {
            self.plane
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
        pub fn point_on_surface_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<PointOnSurface>> + 'table {
            self.point_on_surface
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn point_replica_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<PointReplica>> + 'table {
            self.point_replica
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn poly_loop_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<PolyLoop>> + 'table {
            self.poly_loop
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
        pub fn product_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<Product>> + 'table {
            self.product
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn product_category_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ProductCategory>> + 'table {
            self.product_category
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn product_category_relationship_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ProductCategoryRelationship>> + 'table {
            self.product_category_relationship
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn product_concept_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ProductConcept>> + 'table {
            self.product_concept
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn product_concept_context_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ProductConceptContext>> + 'table {
            self.product_concept_context
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
        pub fn product_definition_effectivity_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ProductDefinitionEffectivity>> + 'table {
            self.product_definition_effectivity
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
        pub fn product_definition_formation_with_specified_source_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ProductDefinitionFormationWithSpecifiedSource>> + 'table
        {
            self.product_definition_formation_with_specified_source
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn product_definition_relationship_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ProductDefinitionRelationship>> + 'table {
            self.product_definition_relationship
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
        pub fn product_definition_usage_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ProductDefinitionUsage>> + 'table {
            self.product_definition_usage
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn product_definition_with_associated_documents_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ProductDefinitionWithAssociatedDocuments>> + 'table
        {
            self.product_definition_with_associated_documents
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn product_related_product_category_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ProductRelatedProductCategory>> + 'table {
            self.product_related_product_category
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn promissory_usage_occurrence_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<PromissoryUsageOccurrence>> + 'table {
            self.promissory_usage_occurrence
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
        pub fn quantified_assembly_component_usage_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<QuantifiedAssemblyComponentUsage>> + 'table {
            self.quantified_assembly_component_usage
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
        pub fn quasi_uniform_surface_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<QuasiUniformSurface>> + 'table {
            self.quasi_uniform_surface
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
        pub fn rational_b_spline_surface_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<RationalBSplineSurface>> + 'table {
            self.rational_b_spline_surface
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn rectangular_composite_surface_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<RectangularCompositeSurface>> + 'table {
            self.rectangular_composite_surface
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn rectangular_trimmed_surface_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<RectangularTrimmedSurface>> + 'table {
            self.rectangular_trimmed_surface
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn reparametrised_composite_curve_segment_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ReparametrisedCompositeCurveSegment>> + 'table {
            self.reparametrised_composite_curve_segment
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
        pub fn representation_relationship_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<RepresentationRelationship>> + 'table {
            self.representation_relationship
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn representation_relationship_with_transformation_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<RepresentationRelationshipWithTransformation>> + 'table
        {
            self.representation_relationship_with_transformation
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn seam_curve_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<SeamCurve>> + 'table {
            self.seam_curve
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
        pub fn serial_numbered_effectivity_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<SerialNumberedEffectivity>> + 'table {
            self.serial_numbered_effectivity
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn shape_aspect_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ShapeAspect>> + 'table {
            self.shape_aspect
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn shape_aspect_relationship_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ShapeAspectRelationship>> + 'table {
            self.shape_aspect_relationship
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
        pub fn shape_representation_relationship_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ShapeRepresentationRelationship>> + 'table {
            self.shape_representation_relationship
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn shell_based_surface_model_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ShellBasedSurfaceModel>> + 'table {
            self.shell_based_surface_model
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn shell_based_wireframe_model_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ShellBasedWireframeModel>> + 'table {
            self.shell_based_wireframe_model
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn shell_based_wireframe_shape_representation_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ShellBasedWireframeShapeRepresentation>> + 'table {
            self.shell_based_wireframe_shape_representation
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
        pub fn solid_angle_measure_with_unit_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<SolidAngleMeasureWithUnit>> + 'table {
            self.solid_angle_measure_with_unit
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn solid_angle_unit_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<SolidAngleUnit>> + 'table {
            self.solid_angle_unit
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn solid_model_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<SolidModel>> + 'table {
            self.solid_model
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn specified_higher_usage_occurrence_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<SpecifiedHigherUsageOccurrence>> + 'table {
            self.specified_higher_usage_occurrence
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn spherical_surface_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<SphericalSurface>> + 'table {
            self.spherical_surface
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn start_request_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<StartRequest>> + 'table {
            self.start_request
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn start_work_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<StartWork>> + 'table {
            self.start_work
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn supplied_part_relationship_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<SuppliedPartRelationship>> + 'table {
            self.supplied_part_relationship
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn surface_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<Surface>> + 'table {
            self.surface
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn surface_curve_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<SurfaceCurve>> + 'table {
            self.surface_curve
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn surface_of_linear_extrusion_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<SurfaceOfLinearExtrusion>> + 'table {
            self.surface_of_linear_extrusion
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn surface_of_revolution_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<SurfaceOfRevolution>> + 'table {
            self.surface_of_revolution
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn surface_patch_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<SurfacePatch>> + 'table {
            self.surface_patch
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn surface_replica_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<SurfaceReplica>> + 'table {
            self.surface_replica
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn swept_surface_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<SweptSurface>> + 'table {
            self.swept_surface
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn topological_representation_item_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<TopologicalRepresentationItem>> + 'table {
            self.topological_representation_item
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn toroidal_surface_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ToroidalSurface>> + 'table {
            self.toroidal_surface
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
        pub fn uncertainty_measure_with_unit_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<UncertaintyMeasureWithUnit>> + 'table {
            self.uncertainty_measure_with_unit
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
        pub fn uniform_surface_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<UniformSurface>> + 'table {
            self.uniform_surface
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
        pub fn versioned_action_request_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<VersionedActionRequest>> + 'table {
            self.versioned_action_request
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn vertex_iter<'table>(&'table self) -> impl Iterator<Item = Result<Vertex>> + 'table {
            self.vertex
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn vertex_loop_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<VertexLoop>> + 'table {
            self.vertex_loop
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn vertex_point_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<VertexPoint>> + 'table {
            self.vertex_point
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn vertex_shell_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<VertexShell>> + 'table {
            self.vertex_shell
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn volume_measure_with_unit_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<VolumeMeasureWithUnit>> + 'table {
            self.volume_measure_with_unit
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn volume_unit_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<VolumeUnit>> + 'table {
            self.volume_unit
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn week_of_year_and_day_date_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<WeekOfYearAndDayDate>> + 'table {
            self.week_of_year_and_day_date
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn wire_shell_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<WireShell>> + 'table {
            self.wire_shell
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: serde :: Deserialize)]
    pub enum AheadOrBehind {
        Ahead,
        Behind,
    }
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ApprovedItem {
        #[holder(use_place_holder)]
        ProductDefinitionFormation(ProductDefinitionFormationAny),
        #[holder(use_place_holder)]
        ProductDefinition(ProductDefinitionAny),
        # [holder (field = configuration_effectivity)]
        #[holder(use_place_holder)]
        ConfigurationEffectivity(Box<ConfigurationEffectivity>),
        # [holder (field = configuration_item)]
        #[holder(use_place_holder)]
        ConfigurationItem(Box<ConfigurationItem>),
        # [holder (field = security_classification)]
        #[holder(use_place_holder)]
        SecurityClassification(Box<SecurityClassification>),
        # [holder (field = change_request)]
        #[holder(use_place_holder)]
        ChangeRequest(Box<ChangeRequest>),
        # [holder (field = change)]
        #[holder(use_place_holder)]
        Change(Box<Change>),
        # [holder (field = start_request)]
        #[holder(use_place_holder)]
        StartRequest(Box<StartRequest>),
        # [holder (field = start_work)]
        #[holder(use_place_holder)]
        StartWork(Box<StartWork>),
        # [holder (field = certification)]
        #[holder(use_place_holder)]
        Certification(Box<Certification>),
        # [holder (field = contract)]
        #[holder(use_place_holder)]
        Contract(Box<Contract>),
    }
    pub type AreaMeasure = f64;
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum Axis2Placement {
        # [holder (field = axis2_placement_2d)]
        #[holder(use_place_holder)]
        Axis2Placement2D(Box<Axis2Placement2D>),
        # [holder (field = axis2_placement_3d)]
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
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum BooleanOperand {
        #[holder(use_place_holder)]
        SolidModel(SolidModelAny),
    }
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum CertifiedItem {
        # [holder (field = supplied_part_relationship)]
        #[holder(use_place_holder)]
        SuppliedPartRelationship(Box<SuppliedPartRelationship>),
    }
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ChangeRequestItem {
        #[holder(use_place_holder)]
        ProductDefinitionFormation(ProductDefinitionFormationAny),
    }
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum CharacterizedDefinition {
        #[holder(use_place_holder)]
        CharacterizedProductDefinition(Box<CharacterizedProductDefinition>),
        #[holder(use_place_holder)]
        ShapeDefinition(Box<ShapeDefinition>),
    }
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum CharacterizedProductDefinition {
        #[holder(use_place_holder)]
        ProductDefinition(ProductDefinitionAny),
        #[holder(use_place_holder)]
        ProductDefinitionRelationship(ProductDefinitionRelationshipAny),
    }
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ClassifiedItem {
        #[holder(use_place_holder)]
        ProductDefinitionFormation(ProductDefinitionFormationAny),
        #[holder(use_place_holder)]
        AssemblyComponentUsage(AssemblyComponentUsageAny),
    }
    pub type ContextDependentMeasure = f64;
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ContractedItem {
        #[holder(use_place_holder)]
        ProductDefinitionFormation(ProductDefinitionFormationAny),
    }
    pub type CountMeasure = f64;
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
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
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum DateTimeItem {
        #[holder(use_place_holder)]
        ProductDefinition(ProductDefinitionAny),
        # [holder (field = change_request)]
        #[holder(use_place_holder)]
        ChangeRequest(Box<ChangeRequest>),
        # [holder (field = start_request)]
        #[holder(use_place_holder)]
        StartRequest(Box<StartRequest>),
        # [holder (field = change)]
        #[holder(use_place_holder)]
        Change(Box<Change>),
        # [holder (field = start_work)]
        #[holder(use_place_holder)]
        StartWork(Box<StartWork>),
        # [holder (field = approval_person_organization)]
        #[holder(use_place_holder)]
        ApprovalPersonOrganization(Box<ApprovalPersonOrganization>),
        # [holder (field = contract)]
        #[holder(use_place_holder)]
        Contract(Box<Contract>),
        # [holder (field = security_classification)]
        #[holder(use_place_holder)]
        SecurityClassification(Box<SecurityClassification>),
        # [holder (field = certification)]
        #[holder(use_place_holder)]
        Certification(Box<Certification>),
    }
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum DateTimeSelect {
        #[holder(use_place_holder)]
        Date(DateAny),
        # [holder (field = local_time)]
        #[holder(use_place_holder)]
        LocalTime(Box<LocalTime>),
        # [holder (field = date_and_time)]
        #[holder(use_place_holder)]
        DateAndTime(Box<DateAndTime>),
    }
    pub type DayInMonthNumber = i64;
    pub type DayInWeekNumber = i64;
    pub type DayInYearNumber = i64;
    pub type DescriptiveMeasure = String;
    pub type DimensionCount = i64;
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum FoundedItemSelect {
        #[holder(use_place_holder)]
        FoundedItem(FoundedItemAny),
        #[holder(use_place_holder)]
        RepresentationItem(RepresentationItemAny),
    }
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
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
    pub type HourInDay = i64;
    pub type Identifier = String;
    #[derive(Debug, Clone, PartialEq, :: serde :: Deserialize)]
    pub enum KnotType {
        UniformKnots,
        Unspecified,
        QuasiUniformKnots,
        PiecewiseBezierKnots,
    }
    pub type Label = String;
    pub type LengthMeasure = f64;
    pub type ListOfReversibleTopologyItem = Vec<ReversibleTopologyItem>;
    pub type MassMeasure = f64;
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum MeasureValue {
        LengthMeasure(LengthMeasure),
        MassMeasure(MassMeasure),
        PlaneAngleMeasure(PlaneAngleMeasure),
        SolidAngleMeasure(SolidAngleMeasure),
        AreaMeasure(AreaMeasure),
        VolumeMeasure(VolumeMeasure),
        ParameterValue(ParameterValue),
        ContextDependentMeasure(ContextDependentMeasure),
        DescriptiveMeasure(DescriptiveMeasure),
        PositiveLengthMeasure(PositiveLengthMeasure),
        PositivePlaneAngleMeasure(PositivePlaneAngleMeasure),
        CountMeasure(CountMeasure),
    }
    pub type MinuteInHour = i64;
    pub type MonthInYearNumber = i64;
    pub type ParameterValue = f64;
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum PcurveOrSurface {
        #[holder(use_place_holder)]
        Pcurve(PcurveAny),
        #[holder(use_place_holder)]
        Surface(SurfaceAny),
    }
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum PersonOrganizationItem {
        # [holder (field = change)]
        #[holder(use_place_holder)]
        Change(Box<Change>),
        # [holder (field = start_work)]
        #[holder(use_place_holder)]
        StartWork(Box<StartWork>),
        # [holder (field = change_request)]
        #[holder(use_place_holder)]
        ChangeRequest(Box<ChangeRequest>),
        # [holder (field = start_request)]
        #[holder(use_place_holder)]
        StartRequest(Box<StartRequest>),
        # [holder (field = configuration_item)]
        #[holder(use_place_holder)]
        ConfigurationItem(Box<ConfigurationItem>),
        # [holder (field = product)]
        #[holder(use_place_holder)]
        Product(Box<Product>),
        #[holder(use_place_holder)]
        ProductDefinitionFormation(ProductDefinitionFormationAny),
        #[holder(use_place_holder)]
        ProductDefinition(ProductDefinitionAny),
        # [holder (field = contract)]
        #[holder(use_place_holder)]
        Contract(Box<Contract>),
        # [holder (field = security_classification)]
        #[holder(use_place_holder)]
        SecurityClassification(Box<SecurityClassification>),
    }
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
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
    pub type PlaneAngleMeasure = f64;
    pub type PositiveLengthMeasure = LengthMeasure;
    pub type PositivePlaneAngleMeasure = PlaneAngleMeasure;
    #[derive(Debug, Clone, PartialEq, :: serde :: Deserialize)]
    pub enum PreferredSurfaceCurveRepresentation {
        Curve3D,
        PcurveS1,
        PcurveS2,
    }
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
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
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
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
    pub type SecondInMinute = f64;
    pub type SetOfReversibleTopologyItem = Vec<ReversibleTopologyItem>;
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ShapeDefinition {
        # [holder (field = product_definition_shape)]
        #[holder(use_place_holder)]
        ProductDefinitionShape(Box<ProductDefinitionShape>),
        # [holder (field = shape_aspect)]
        #[holder(use_place_holder)]
        ShapeAspect(Box<ShapeAspect>),
        # [holder (field = shape_aspect_relationship)]
        #[holder(use_place_holder)]
        ShapeAspectRelationship(Box<ShapeAspectRelationship>),
    }
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum Shell {
        # [holder (field = vertex_shell)]
        #[holder(use_place_holder)]
        VertexShell(Box<VertexShell>),
        # [holder (field = wire_shell)]
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
    pub type SolidAngleMeasure = f64;
    #[derive(Debug, Clone, PartialEq, :: serde :: Deserialize)]
    pub enum Source {
        Made,
        Bought,
        NotKnown,
    }
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum SpecifiedItem {
        #[holder(use_place_holder)]
        ProductDefinition(ProductDefinitionAny),
        # [holder (field = shape_aspect)]
        #[holder(use_place_holder)]
        ShapeAspect(Box<ShapeAspect>),
    }
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum StartRequestItem {
        #[holder(use_place_holder)]
        ProductDefinitionFormation(ProductDefinitionFormationAny),
    }
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum SupportedItem {
        # [holder (field = action_directive)]
        #[holder(use_place_holder)]
        ActionDirective(Box<ActionDirective>),
        #[holder(use_place_holder)]
        Action(ActionAny),
        # [holder (field = action_method)]
        #[holder(use_place_holder)]
        ActionMethod(Box<ActionMethod>),
    }
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum SurfaceModel {
        # [holder (field = shell_based_surface_model)]
        #[holder(use_place_holder)]
        ShellBasedSurfaceModel(Box<ShellBasedSurfaceModel>),
    }
    pub type Text = String;
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum Transformation {
        # [holder (field = item_defined_transformation)]
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
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum TrimmingSelect {
        # [holder (field = cartesian_point)]
        #[holder(use_place_holder)]
        CartesianPoint(Box<CartesianPoint>),
        ParameterValue(ParameterValue),
    }
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum Unit {
        #[holder(use_place_holder)]
        NamedUnit(NamedUnitAny),
    }
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
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
    pub type VolumeMeasure = f64;
    pub type WeekInYearNumber = i64;
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum WireframeModel {
        # [holder (field = shell_based_wireframe_model)]
        #[holder(use_place_holder)]
        ShellBasedWireframeModel(Box<ShellBasedWireframeModel>),
        # [holder (field = edge_based_wireframe_model)]
        #[holder(use_place_holder)]
        EdgeBasedWireframeModel(Box<EdgeBasedWireframeModel>),
    }
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum WorkItem {
        #[holder(use_place_holder)]
        ProductDefinitionFormation(ProductDefinitionFormationAny),
    }
    pub type YearNumber = i64;
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = action)]
    #[holder(generate_deserialize)]
    pub struct Action {
        pub name: Label,
        pub description: Text,
        #[holder(use_place_holder)]
        pub chosen_method: ActionMethod,
    }
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ActionAny {
        #[holder(use_place_holder)]
        # [holder (field = executed_action)]
        ExecutedAction(Box<ExecutedAction>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = action_assignment)]
    #[holder(generate_deserialize)]
    pub struct ActionAssignment {
        #[holder(use_place_holder)]
        pub assigned_action: ActionAny,
    }
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ActionAssignmentAny {
        #[holder(use_place_holder)]
        # [holder (field = change)]
        Change(Box<Change>),
        #[holder(use_place_holder)]
        # [holder (field = start_work)]
        StartWork(Box<StartWork>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = action_directive)]
    #[holder(generate_deserialize)]
    pub struct ActionDirective {
        pub name: Label,
        pub description: Text,
        pub analysis: Text,
        pub comment: Text,
        #[holder(use_place_holder)]
        pub requests: Vec<VersionedActionRequest>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = action_method)]
    #[holder(generate_deserialize)]
    pub struct ActionMethod {
        pub name: Label,
        pub description: Text,
        pub consequence: Text,
        pub purpose: Text,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = action_request_assignment)]
    #[holder(generate_deserialize)]
    pub struct ActionRequestAssignment {
        #[holder(use_place_holder)]
        pub assigned_action_request: VersionedActionRequest,
    }
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ActionRequestAssignmentAny {
        #[holder(use_place_holder)]
        # [holder (field = change_request)]
        ChangeRequest(Box<ChangeRequest>),
        #[holder(use_place_holder)]
        # [holder (field = start_request)]
        StartRequest(Box<StartRequest>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = action_request_solution)]
    #[holder(generate_deserialize)]
    pub struct ActionRequestSolution {
        #[holder(use_place_holder)]
        pub method: ActionMethod,
        #[holder(use_place_holder)]
        pub request: VersionedActionRequest,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = action_request_status)]
    #[holder(generate_deserialize)]
    pub struct ActionRequestStatus {
        pub status: Label,
        #[holder(use_place_holder)]
        pub assigned_request: VersionedActionRequest,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = action_status)]
    #[holder(generate_deserialize)]
    pub struct ActionStatus {
        pub status: Label,
        #[holder(use_place_holder)]
        pub assigned_action: ExecutedActionAny,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
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
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
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
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = advanced_brep_shape_representation)]
    #[holder(generate_deserialize)]
    pub struct AdvancedBrepShapeRepresentation {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = advanced_face)]
    #[holder(generate_deserialize)]
    pub struct AdvancedFace {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = alternate_product_relationship)]
    #[holder(generate_deserialize)]
    pub struct AlternateProductRelationship {
        pub name: Label,
        pub definition: Text,
        #[holder(use_place_holder)]
        pub alternate: Product,
        #[holder(use_place_holder)]
        pub base: Product,
        pub basis: Text,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = application_context)]
    #[holder(generate_deserialize)]
    pub struct ApplicationContext {
        pub application: Text,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = application_context_element)]
    #[holder(generate_deserialize)]
    pub struct ApplicationContextElement {
        pub name: Label,
        #[holder(use_place_holder)]
        pub frame_of_reference: ApplicationContext,
    }
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ApplicationContextElementAny {
        #[holder(use_place_holder)]
        # [holder (field = product_concept_context)]
        ProductConceptContext(Box<ProductConceptContext>),
        #[holder(use_place_holder)]
        # [holder (field = product_context)]
        ProductContext(Box<ProductContext>),
        #[holder(use_place_holder)]
        # [holder (field = product_definition_context)]
        ProductDefinitionContext(Box<ProductDefinitionContext>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
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
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = approval)]
    #[holder(generate_deserialize)]
    pub struct Approval {
        #[holder(use_place_holder)]
        pub status: ApprovalStatus,
        pub level: Label,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = approval_assignment)]
    #[holder(generate_deserialize)]
    pub struct ApprovalAssignment {
        #[holder(use_place_holder)]
        pub assigned_approval: Approval,
    }
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ApprovalAssignmentAny {
        #[holder(use_place_holder)]
        # [holder (field = cc_design_approval)]
        CcDesignApproval(Box<CcDesignApproval>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = approval_date_time)]
    #[holder(generate_deserialize)]
    pub struct ApprovalDateTime {
        #[holder(use_place_holder)]
        pub date_time: DateTimeSelect,
        #[holder(use_place_holder)]
        pub dated_approval: Approval,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
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
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = approval_relationship)]
    #[holder(generate_deserialize)]
    pub struct ApprovalRelationship {
        pub name: Label,
        pub description: Text,
        #[holder(use_place_holder)]
        pub relating_approval: Approval,
        #[holder(use_place_holder)]
        pub related_approval: Approval,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = approval_role)]
    #[holder(generate_deserialize)]
    pub struct ApprovalRole {
        pub role: Label,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = approval_status)]
    #[holder(generate_deserialize)]
    pub struct ApprovalStatus {
        pub name: Label,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = area_measure_with_unit)]
    #[holder(generate_deserialize)]
    pub struct AreaMeasureWithUnit {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = area_unit)]
    #[holder(generate_deserialize)]
    pub struct AreaUnit {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = assembly_component_usage)]
    #[holder(generate_deserialize)]
    pub struct AssemblyComponentUsage {
        pub reference_designator: Option<Identifier>,
    }
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum AssemblyComponentUsageAny {
        #[holder(use_place_holder)]
        # [holder (field = next_assembly_usage_occurrence)]
        NextAssemblyUsageOccurrence(Box<NextAssemblyUsageOccurrence>),
        #[holder(use_place_holder)]
        # [holder (field = promissory_usage_occurrence)]
        PromissoryUsageOccurrence(Box<PromissoryUsageOccurrence>),
        #[holder(use_place_holder)]
        # [holder (field = quantified_assembly_component_usage)]
        QuantifiedAssemblyComponentUsage(Box<QuantifiedAssemblyComponentUsage>),
        #[holder(use_place_holder)]
        # [holder (field = specified_higher_usage_occurrence)]
        SpecifiedHigherUsageOccurrence(Box<SpecifiedHigherUsageOccurrence>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = assembly_component_usage_substitute)]
    #[holder(generate_deserialize)]
    pub struct AssemblyComponentUsageSubstitute {
        pub name: Label,
        pub definition: Text,
        #[holder(use_place_holder)]
        pub base: AssemblyComponentUsageAny,
        #[holder(use_place_holder)]
        pub substitute: AssemblyComponentUsageAny,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = axis1_placement)]
    #[holder(generate_deserialize)]
    pub struct Axis1Placement {
        #[holder(use_place_holder)]
        pub axis: Option<Direction>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = axis2_placement_2d)]
    #[holder(generate_deserialize)]
    pub struct Axis2Placement2D {
        #[holder(use_place_holder)]
        pub ref_direction: Option<Direction>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = axis2_placement_3d)]
    #[holder(generate_deserialize)]
    pub struct Axis2Placement3D {
        #[holder(use_place_holder)]
        pub axis: Option<Direction>,
        #[holder(use_place_holder)]
        pub ref_direction: Option<Direction>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
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
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
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
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = b_spline_curve_with_knots)]
    #[holder(generate_deserialize)]
    pub struct BSplineCurveWithKnots {
        pub knot_multiplicities: Vec<i64>,
        pub knots: Vec<ParameterValue>,
        pub knot_spec: KnotType,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = b_spline_surface)]
    #[holder(generate_deserialize)]
    pub struct BSplineSurface {
        pub u_degree: i64,
        pub v_degree: i64,
        #[holder(use_place_holder)]
        pub control_points_list: Vec<Vec<CartesianPoint>>,
        pub surface_form: BSplineSurfaceForm,
        pub u_closed: Logical,
        pub v_closed: Logical,
        pub self_intersect: Logical,
    }
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum BSplineSurfaceAny {
        #[holder(use_place_holder)]
        # [holder (field = b_spline_surface_with_knots)]
        BSplineSurfaceWithKnots(Box<BSplineSurfaceWithKnots>),
        #[holder(use_place_holder)]
        # [holder (field = bezier_surface)]
        BezierSurface(Box<BezierSurface>),
        #[holder(use_place_holder)]
        # [holder (field = quasi_uniform_surface)]
        QuasiUniformSurface(Box<QuasiUniformSurface>),
        #[holder(use_place_holder)]
        # [holder (field = rational_b_spline_surface)]
        RationalBSplineSurface(Box<RationalBSplineSurface>),
        #[holder(use_place_holder)]
        # [holder (field = uniform_surface)]
        UniformSurface(Box<UniformSurface>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = b_spline_surface_with_knots)]
    #[holder(generate_deserialize)]
    pub struct BSplineSurfaceWithKnots {
        pub u_multiplicities: Vec<i64>,
        pub v_multiplicities: Vec<i64>,
        pub u_knots: Vec<ParameterValue>,
        pub v_knots: Vec<ParameterValue>,
        pub knot_spec: KnotType,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = bezier_curve)]
    #[holder(generate_deserialize)]
    pub struct BezierCurve {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = bezier_surface)]
    #[holder(generate_deserialize)]
    pub struct BezierSurface {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = boundary_curve)]
    #[holder(generate_deserialize)]
    pub struct BoundaryCurve {}
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum BoundaryCurveAny {
        #[holder(use_place_holder)]
        # [holder (field = outer_boundary_curve)]
        OuterBoundaryCurve(Box<OuterBoundaryCurve>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = bounded_curve)]
    #[holder(generate_deserialize)]
    pub struct BoundedCurve {}
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum BoundedCurveAny {
        #[holder(use_place_holder)]
        # [holder (field = b_spline_curve)]
        BSplineCurve(Box<BSplineCurve>),
        #[holder(use_place_holder)]
        # [holder (field = bounded_pcurve)]
        BoundedPcurve(Box<BoundedPcurve>),
        #[holder(use_place_holder)]
        # [holder (field = bounded_surface_curve)]
        BoundedSurfaceCurve(Box<BoundedSurfaceCurve>),
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
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = bounded_pcurve)]
    #[holder(generate_deserialize)]
    pub struct BoundedPcurve {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = bounded_surface)]
    #[holder(generate_deserialize)]
    pub struct BoundedSurface {}
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum BoundedSurfaceAny {
        #[holder(use_place_holder)]
        # [holder (field = b_spline_surface)]
        BSplineSurface(Box<BSplineSurface>),
        #[holder(use_place_holder)]
        # [holder (field = curve_bounded_surface)]
        CurveBoundedSurface(Box<CurveBoundedSurface>),
        #[holder(use_place_holder)]
        # [holder (field = rectangular_composite_surface)]
        RectangularCompositeSurface(Box<RectangularCompositeSurface>),
        #[holder(use_place_holder)]
        # [holder (field = rectangular_trimmed_surface)]
        RectangularTrimmedSurface(Box<RectangularTrimmedSurface>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = bounded_surface_curve)]
    #[holder(generate_deserialize)]
    pub struct BoundedSurfaceCurve {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = brep_with_voids)]
    #[holder(generate_deserialize)]
    pub struct BrepWithVoids {
        #[holder(use_place_holder)]
        pub voids: Vec<OrientedClosedShell>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = calendar_date)]
    #[holder(generate_deserialize)]
    pub struct CalendarDate {
        pub day_component: DayInMonthNumber,
        pub month_component: MonthInYearNumber,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = cartesian_point)]
    #[holder(generate_deserialize)]
    pub struct CartesianPoint {
        pub coordinates: Vec<LengthMeasure>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = cartesian_transformation_operator)]
    #[holder(generate_deserialize)]
    pub struct CartesianTransformationOperator {
        #[holder(use_place_holder)]
        pub axis1: Option<Direction>,
        #[holder(use_place_holder)]
        pub axis2: Option<Direction>,
        #[holder(use_place_holder)]
        pub local_origin: CartesianPoint,
        pub scale: Option<f64>,
    }
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum CartesianTransformationOperatorAny {
        #[holder(use_place_holder)]
        # [holder (field = cartesian_transformation_operator_3d)]
        CartesianTransformationOperator3D(Box<CartesianTransformationOperator3D>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = cartesian_transformation_operator_3d)]
    #[holder(generate_deserialize)]
    pub struct CartesianTransformationOperator3D {
        #[holder(use_place_holder)]
        pub axis3: Option<Direction>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = cc_design_approval)]
    #[holder(generate_deserialize)]
    pub struct CcDesignApproval {
        #[holder(use_place_holder)]
        pub items: Vec<ApprovedItem>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = cc_design_certification)]
    #[holder(generate_deserialize)]
    pub struct CcDesignCertification {
        #[holder(use_place_holder)]
        pub items: Vec<CertifiedItem>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = cc_design_contract)]
    #[holder(generate_deserialize)]
    pub struct CcDesignContract {
        #[holder(use_place_holder)]
        pub items: Vec<ContractedItem>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = cc_design_date_and_time_assignment)]
    #[holder(generate_deserialize)]
    pub struct CcDesignDateAndTimeAssignment {
        #[holder(use_place_holder)]
        pub items: Vec<DateTimeItem>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = cc_design_person_and_organization_assignment)]
    #[holder(generate_deserialize)]
    pub struct CcDesignPersonAndOrganizationAssignment {
        #[holder(use_place_holder)]
        pub items: Vec<PersonOrganizationItem>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = cc_design_security_classification)]
    #[holder(generate_deserialize)]
    pub struct CcDesignSecurityClassification {
        #[holder(use_place_holder)]
        pub items: Vec<ClassifiedItem>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = cc_design_specification_reference)]
    #[holder(generate_deserialize)]
    pub struct CcDesignSpecificationReference {
        #[holder(use_place_holder)]
        pub items: Vec<SpecifiedItem>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = certification)]
    #[holder(generate_deserialize)]
    pub struct Certification {
        pub name: Label,
        pub purpose: Text,
        #[holder(use_place_holder)]
        pub kind: CertificationType,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = certification_assignment)]
    #[holder(generate_deserialize)]
    pub struct CertificationAssignment {
        #[holder(use_place_holder)]
        pub assigned_certification: Certification,
    }
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum CertificationAssignmentAny {
        #[holder(use_place_holder)]
        # [holder (field = cc_design_certification)]
        CcDesignCertification(Box<CcDesignCertification>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = certification_type)]
    #[holder(generate_deserialize)]
    pub struct CertificationType {
        pub description: Label,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = change)]
    #[holder(generate_deserialize)]
    pub struct Change {
        #[holder(use_place_holder)]
        pub items: Vec<WorkItem>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = change_request)]
    #[holder(generate_deserialize)]
    pub struct ChangeRequest {
        #[holder(use_place_holder)]
        pub items: Vec<ChangeRequestItem>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = circle)]
    #[holder(generate_deserialize)]
    pub struct Circle {
        pub radius: PositiveLengthMeasure,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = closed_shell)]
    #[holder(generate_deserialize)]
    pub struct ClosedShell {}
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ClosedShellAny {
        #[holder(use_place_holder)]
        # [holder (field = oriented_closed_shell)]
        OrientedClosedShell(Box<OrientedClosedShell>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = composite_curve)]
    #[holder(generate_deserialize)]
    pub struct CompositeCurve {
        #[holder(use_place_holder)]
        pub segments: Vec<CompositeCurveSegmentAny>,
        pub self_intersect: Logical,
    }
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum CompositeCurveAny {
        #[holder(use_place_holder)]
        # [holder (field = composite_curve_on_surface)]
        CompositeCurveOnSurface(Box<CompositeCurveOnSurface>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = composite_curve_on_surface)]
    #[holder(generate_deserialize)]
    pub struct CompositeCurveOnSurface {}
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum CompositeCurveOnSurfaceAny {
        #[holder(use_place_holder)]
        # [holder (field = boundary_curve)]
        BoundaryCurve(Box<BoundaryCurve>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = composite_curve_segment)]
    #[holder(generate_deserialize)]
    pub struct CompositeCurveSegment {
        pub transition: TransitionCode,
        pub same_sense: bool,
        #[holder(use_place_holder)]
        pub parent_curve: CurveAny,
    }
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum CompositeCurveSegmentAny {
        #[holder(use_place_holder)]
        # [holder (field = reparametrised_composite_curve_segment)]
        ReparametrisedCompositeCurveSegment(Box<ReparametrisedCompositeCurveSegment>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = configuration_design)]
    #[holder(generate_deserialize)]
    pub struct ConfigurationDesign {
        #[holder(use_place_holder)]
        pub configuration: ConfigurationItem,
        #[holder(use_place_holder)]
        pub design: ProductDefinitionFormationAny,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = configuration_effectivity)]
    #[holder(generate_deserialize)]
    pub struct ConfigurationEffectivity {
        #[holder(use_place_holder)]
        pub configuration: ConfigurationDesign,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = configuration_item)]
    #[holder(generate_deserialize)]
    pub struct ConfigurationItem {
        pub id: Identifier,
        pub name: Label,
        pub description: Option<Text>,
        #[holder(use_place_holder)]
        pub item_concept: ProductConcept,
        pub purpose: Option<Label>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = conic)]
    #[holder(generate_deserialize)]
    pub struct Conic {
        #[holder(use_place_holder)]
        pub position: Axis2Placement,
    }
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
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
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = conical_surface)]
    #[holder(generate_deserialize)]
    pub struct ConicalSurface {
        pub radius: LengthMeasure,
        pub semi_angle: PlaneAngleMeasure,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = connected_edge_set)]
    #[holder(generate_deserialize)]
    pub struct ConnectedEdgeSet {
        #[holder(use_place_holder)]
        pub ces_edges: Vec<EdgeAny>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = connected_face_set)]
    #[holder(generate_deserialize)]
    pub struct ConnectedFaceSet {
        #[holder(use_place_holder)]
        pub cfs_faces: Vec<FaceAny>,
    }
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ConnectedFaceSetAny {
        #[holder(use_place_holder)]
        # [holder (field = closed_shell)]
        ClosedShell(Box<ClosedShell>),
        #[holder(use_place_holder)]
        # [holder (field = open_shell)]
        OpenShell(Box<OpenShell>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = context_dependent_shape_representation)]
    #[holder(generate_deserialize)]
    pub struct ContextDependentShapeRepresentation {
        #[holder(use_place_holder)]
        pub representation_relation: ShapeRepresentationRelationship,
        #[holder(use_place_holder)]
        pub represented_product_relation: ProductDefinitionShape,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = context_dependent_unit)]
    #[holder(generate_deserialize)]
    pub struct ContextDependentUnit {
        pub name: Label,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = contract)]
    #[holder(generate_deserialize)]
    pub struct Contract {
        pub name: Label,
        pub purpose: Text,
        #[holder(use_place_holder)]
        pub kind: ContractType,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = contract_assignment)]
    #[holder(generate_deserialize)]
    pub struct ContractAssignment {
        #[holder(use_place_holder)]
        pub assigned_contract: Contract,
    }
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ContractAssignmentAny {
        #[holder(use_place_holder)]
        # [holder (field = cc_design_contract)]
        CcDesignContract(Box<CcDesignContract>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = contract_type)]
    #[holder(generate_deserialize)]
    pub struct ContractType {
        pub description: Label,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = conversion_based_unit)]
    #[holder(generate_deserialize)]
    pub struct ConversionBasedUnit {
        pub name: Label,
        #[holder(use_place_holder)]
        pub conversion_factor: MeasureWithUnitAny,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = coordinated_universal_time_offset)]
    #[holder(generate_deserialize)]
    pub struct CoordinatedUniversalTimeOffset {
        pub hour_offset: HourInDay,
        pub minute_offset: Option<MinuteInHour>,
        pub sense: AheadOrBehind,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = curve)]
    #[holder(generate_deserialize)]
    pub struct Curve {}
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
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
        # [holder (field = curve_replica)]
        CurveReplica(Box<CurveReplica>),
        #[holder(use_place_holder)]
        # [holder (field = line)]
        Line(Box<Line>),
        #[holder(use_place_holder)]
        # [holder (field = offset_curve_3d)]
        OffsetCurve3D(Box<OffsetCurve3D>),
        #[holder(use_place_holder)]
        # [holder (field = pcurve)]
        Pcurve(Box<Pcurve>),
        #[holder(use_place_holder)]
        # [holder (field = surface_curve)]
        SurfaceCurve(Box<SurfaceCurve>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = curve_bounded_surface)]
    #[holder(generate_deserialize)]
    pub struct CurveBoundedSurface {
        #[holder(use_place_holder)]
        pub basis_surface: SurfaceAny,
        #[holder(use_place_holder)]
        pub boundaries: Vec<BoundaryCurveAny>,
        pub implicit_outer: bool,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = curve_replica)]
    #[holder(generate_deserialize)]
    pub struct CurveReplica {
        #[holder(use_place_holder)]
        pub parent_curve: CurveAny,
        #[holder(use_place_holder)]
        pub transformation: CartesianTransformationOperatorAny,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = cylindrical_surface)]
    #[holder(generate_deserialize)]
    pub struct CylindricalSurface {
        pub radius: PositiveLengthMeasure,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = date)]
    #[holder(generate_deserialize)]
    pub struct Date {
        pub year_component: YearNumber,
    }
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum DateAny {
        #[holder(use_place_holder)]
        # [holder (field = calendar_date)]
        CalendarDate(Box<CalendarDate>),
        #[holder(use_place_holder)]
        # [holder (field = ordinal_date)]
        OrdinalDate(Box<OrdinalDate>),
        #[holder(use_place_holder)]
        # [holder (field = week_of_year_and_day_date)]
        WeekOfYearAndDayDate(Box<WeekOfYearAndDayDate>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = date_and_time)]
    #[holder(generate_deserialize)]
    pub struct DateAndTime {
        #[holder(use_place_holder)]
        pub date_component: DateAny,
        #[holder(use_place_holder)]
        pub time_component: LocalTime,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = date_and_time_assignment)]
    #[holder(generate_deserialize)]
    pub struct DateAndTimeAssignment {
        #[holder(use_place_holder)]
        pub assigned_date_and_time: DateAndTime,
        #[holder(use_place_holder)]
        pub role: DateTimeRole,
    }
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum DateAndTimeAssignmentAny {
        #[holder(use_place_holder)]
        # [holder (field = cc_design_date_and_time_assignment)]
        CcDesignDateAndTimeAssignment(Box<CcDesignDateAndTimeAssignment>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = date_time_role)]
    #[holder(generate_deserialize)]
    pub struct DateTimeRole {
        pub name: Label,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = dated_effectivity)]
    #[holder(generate_deserialize)]
    pub struct DatedEffectivity {
        #[holder(use_place_holder)]
        pub effectivity_start_date: DateAndTime,
        #[holder(use_place_holder)]
        pub effectivity_end_date: Option<DateAndTime>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = definitional_representation)]
    #[holder(generate_deserialize)]
    pub struct DefinitionalRepresentation {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = degenerate_pcurve)]
    #[holder(generate_deserialize)]
    pub struct DegeneratePcurve {
        #[holder(use_place_holder)]
        pub basis_surface: SurfaceAny,
        #[holder(use_place_holder)]
        pub reference_to_curve: DefinitionalRepresentation,
    }
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum DegeneratePcurveAny {
        #[holder(use_place_holder)]
        # [holder (field = evaluated_degenerate_pcurve)]
        EvaluatedDegeneratePcurve(Box<EvaluatedDegeneratePcurve>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = degenerate_toroidal_surface)]
    #[holder(generate_deserialize)]
    pub struct DegenerateToroidalSurface {
        pub select_outer: bool,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = design_context)]
    #[holder(generate_deserialize)]
    pub struct DesignContext {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = design_make_from_relationship)]
    #[holder(generate_deserialize)]
    pub struct DesignMakeFromRelationship {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
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
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = directed_action)]
    #[holder(generate_deserialize)]
    pub struct DirectedAction {
        #[holder(use_place_holder)]
        pub directive: ActionDirective,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = direction)]
    #[holder(generate_deserialize)]
    pub struct Direction {
        pub direction_ratios: Vec<f64>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
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
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum DocumentAny {
        #[holder(use_place_holder)]
        # [holder (field = document_with_class)]
        DocumentWithClass(Box<DocumentWithClass>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = document_reference)]
    #[holder(generate_deserialize)]
    pub struct DocumentReference {
        #[holder(use_place_holder)]
        pub assigned_document: DocumentAny,
        pub source: Label,
    }
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum DocumentReferenceAny {
        #[holder(use_place_holder)]
        # [holder (field = cc_design_specification_reference)]
        CcDesignSpecificationReference(Box<CcDesignSpecificationReference>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = document_relationship)]
    #[holder(generate_deserialize)]
    pub struct DocumentRelationship {
        pub name: Label,
        pub description: Text,
        #[holder(use_place_holder)]
        pub relating_document: DocumentAny,
        #[holder(use_place_holder)]
        pub related_document: DocumentAny,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = document_type)]
    #[holder(generate_deserialize)]
    pub struct DocumentType {
        pub product_data_type: Label,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = document_usage_constraint)]
    #[holder(generate_deserialize)]
    pub struct DocumentUsageConstraint {
        #[holder(use_place_holder)]
        pub source: DocumentAny,
        pub subject_element: Label,
        pub subject_element_value: Text,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = document_with_class)]
    #[holder(generate_deserialize)]
    pub struct DocumentWithClass {
        pub class: Identifier,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = edge)]
    #[holder(generate_deserialize)]
    pub struct Edge {
        #[holder(use_place_holder)]
        pub edge_start: VertexAny,
        #[holder(use_place_holder)]
        pub edge_end: VertexAny,
    }
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum EdgeAny {
        #[holder(use_place_holder)]
        # [holder (field = edge_curve)]
        EdgeCurve(Box<EdgeCurve>),
        #[holder(use_place_holder)]
        # [holder (field = oriented_edge)]
        OrientedEdge(Box<OrientedEdge>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = edge_based_wireframe_model)]
    #[holder(generate_deserialize)]
    pub struct EdgeBasedWireframeModel {
        #[holder(use_place_holder)]
        pub ebwm_boundary: Vec<ConnectedEdgeSet>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = edge_based_wireframe_shape_representation)]
    #[holder(generate_deserialize)]
    pub struct EdgeBasedWireframeShapeRepresentation {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = edge_curve)]
    #[holder(generate_deserialize)]
    pub struct EdgeCurve {
        #[holder(use_place_holder)]
        pub edge_geometry: CurveAny,
        pub same_sense: bool,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = edge_loop)]
    #[holder(generate_deserialize)]
    pub struct EdgeLoop {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = effectivity)]
    #[holder(generate_deserialize)]
    pub struct Effectivity {
        pub id: Identifier,
    }
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum EffectivityAny {
        #[holder(use_place_holder)]
        # [holder (field = dated_effectivity)]
        DatedEffectivity(Box<DatedEffectivity>),
        #[holder(use_place_holder)]
        # [holder (field = lot_effectivity)]
        LotEffectivity(Box<LotEffectivity>),
        #[holder(use_place_holder)]
        # [holder (field = product_definition_effectivity)]
        ProductDefinitionEffectivity(Box<ProductDefinitionEffectivity>),
        #[holder(use_place_holder)]
        # [holder (field = serial_numbered_effectivity)]
        SerialNumberedEffectivity(Box<SerialNumberedEffectivity>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = elementary_surface)]
    #[holder(generate_deserialize)]
    pub struct ElementarySurface {
        #[holder(use_place_holder)]
        pub position: Axis2Placement3D,
    }
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ElementarySurfaceAny {
        #[holder(use_place_holder)]
        # [holder (field = conical_surface)]
        ConicalSurface(Box<ConicalSurface>),
        #[holder(use_place_holder)]
        # [holder (field = cylindrical_surface)]
        CylindricalSurface(Box<CylindricalSurface>),
        #[holder(use_place_holder)]
        # [holder (field = plane)]
        Plane(Box<Plane>),
        #[holder(use_place_holder)]
        # [holder (field = spherical_surface)]
        SphericalSurface(Box<SphericalSurface>),
        #[holder(use_place_holder)]
        # [holder (field = toroidal_surface)]
        ToroidalSurface(Box<ToroidalSurface>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = ellipse)]
    #[holder(generate_deserialize)]
    pub struct Ellipse {
        pub semi_axis_1: PositiveLengthMeasure,
        pub semi_axis_2: PositiveLengthMeasure,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = evaluated_degenerate_pcurve)]
    #[holder(generate_deserialize)]
    pub struct EvaluatedDegeneratePcurve {
        #[holder(use_place_holder)]
        pub equivalent_point: CartesianPoint,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = executed_action)]
    #[holder(generate_deserialize)]
    pub struct ExecutedAction {}
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ExecutedActionAny {
        #[holder(use_place_holder)]
        # [holder (field = directed_action)]
        DirectedAction(Box<DirectedAction>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = face)]
    #[holder(generate_deserialize)]
    pub struct Face {
        #[holder(use_place_holder)]
        pub bounds: Vec<FaceBoundAny>,
    }
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum FaceAny {
        #[holder(use_place_holder)]
        # [holder (field = face_surface)]
        FaceSurface(Box<FaceSurface>),
        #[holder(use_place_holder)]
        # [holder (field = oriented_face)]
        OrientedFace(Box<OrientedFace>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = face_bound)]
    #[holder(generate_deserialize)]
    pub struct FaceBound {
        #[holder(use_place_holder)]
        pub bound: LoopAny,
        pub orientation: bool,
    }
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum FaceBoundAny {
        #[holder(use_place_holder)]
        # [holder (field = face_outer_bound)]
        FaceOuterBound(Box<FaceOuterBound>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = face_outer_bound)]
    #[holder(generate_deserialize)]
    pub struct FaceOuterBound {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = face_surface)]
    #[holder(generate_deserialize)]
    pub struct FaceSurface {
        #[holder(use_place_holder)]
        pub face_geometry: SurfaceAny,
        pub same_sense: bool,
    }
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum FaceSurfaceAny {
        #[holder(use_place_holder)]
        # [holder (field = advanced_face)]
        AdvancedFace(Box<AdvancedFace>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = faceted_brep)]
    #[holder(generate_deserialize)]
    pub struct FacetedBrep {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = faceted_brep_shape_representation)]
    #[holder(generate_deserialize)]
    pub struct FacetedBrepShapeRepresentation {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = founded_item)]
    #[holder(generate_deserialize)]
    pub struct FoundedItem {}
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum FoundedItemAny {
        #[holder(use_place_holder)]
        # [holder (field = composite_curve_segment)]
        CompositeCurveSegment(Box<CompositeCurveSegment>),
        #[holder(use_place_holder)]
        # [holder (field = surface_patch)]
        SurfacePatch(Box<SurfacePatch>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = functionally_defined_transformation)]
    #[holder(generate_deserialize)]
    pub struct FunctionallyDefinedTransformation {
        pub name: Label,
        pub description: Text,
    }
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum FunctionallyDefinedTransformationAny {
        #[holder(use_place_holder)]
        # [holder (field = cartesian_transformation_operator)]
        CartesianTransformationOperator(Box<CartesianTransformationOperator>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = geometric_curve_set)]
    #[holder(generate_deserialize)]
    pub struct GeometricCurveSet {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = geometric_representation_context)]
    #[holder(generate_deserialize)]
    pub struct GeometricRepresentationContext {
        pub coordinate_space_dimension: DimensionCount,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = geometric_representation_item)]
    #[holder(generate_deserialize)]
    pub struct GeometricRepresentationItem {}
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum GeometricRepresentationItemAny {
        #[holder(use_place_holder)]
        # [holder (field = cartesian_transformation_operator)]
        CartesianTransformationOperator(Box<CartesianTransformationOperator>),
        #[holder(use_place_holder)]
        # [holder (field = curve)]
        Curve(Box<Curve>),
        #[holder(use_place_holder)]
        # [holder (field = direction)]
        Direction(Box<Direction>),
        #[holder(use_place_holder)]
        # [holder (field = edge_based_wireframe_model)]
        EdgeBasedWireframeModel(Box<EdgeBasedWireframeModel>),
        #[holder(use_place_holder)]
        # [holder (field = edge_curve)]
        EdgeCurve(Box<EdgeCurve>),
        #[holder(use_place_holder)]
        # [holder (field = face_surface)]
        FaceSurface(Box<FaceSurface>),
        #[holder(use_place_holder)]
        # [holder (field = geometric_set)]
        GeometricSet(Box<GeometricSet>),
        #[holder(use_place_holder)]
        # [holder (field = placement)]
        Placement(Box<Placement>),
        #[holder(use_place_holder)]
        # [holder (field = point)]
        Point(Box<Point>),
        #[holder(use_place_holder)]
        # [holder (field = poly_loop)]
        PolyLoop(Box<PolyLoop>),
        #[holder(use_place_holder)]
        # [holder (field = shell_based_surface_model)]
        ShellBasedSurfaceModel(Box<ShellBasedSurfaceModel>),
        #[holder(use_place_holder)]
        # [holder (field = shell_based_wireframe_model)]
        ShellBasedWireframeModel(Box<ShellBasedWireframeModel>),
        #[holder(use_place_holder)]
        # [holder (field = solid_model)]
        SolidModel(Box<SolidModel>),
        #[holder(use_place_holder)]
        # [holder (field = surface)]
        Surface(Box<Surface>),
        #[holder(use_place_holder)]
        # [holder (field = vector)]
        Vector(Box<Vector>),
        #[holder(use_place_holder)]
        # [holder (field = vertex_point)]
        VertexPoint(Box<VertexPoint>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = geometric_set)]
    #[holder(generate_deserialize)]
    pub struct GeometricSet {
        #[holder(use_place_holder)]
        pub elements: Vec<GeometricSetSelect>,
    }
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum GeometricSetAny {
        #[holder(use_place_holder)]
        # [holder (field = geometric_curve_set)]
        GeometricCurveSet(Box<GeometricCurveSet>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = geometrically_bounded_surface_shape_representation)]
    #[holder(generate_deserialize)]
    pub struct GeometricallyBoundedSurfaceShapeRepresentation {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = geometrically_bounded_wireframe_shape_representation)]
    #[holder(generate_deserialize)]
    pub struct GeometricallyBoundedWireframeShapeRepresentation {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = global_uncertainty_assigned_context)]
    #[holder(generate_deserialize)]
    pub struct GlobalUncertaintyAssignedContext {
        #[holder(use_place_holder)]
        pub uncertainty: Vec<UncertaintyMeasureWithUnit>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = global_unit_assigned_context)]
    #[holder(generate_deserialize)]
    pub struct GlobalUnitAssignedContext {
        #[holder(use_place_holder)]
        pub units: Vec<Unit>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = hyperbola)]
    #[holder(generate_deserialize)]
    pub struct Hyperbola {
        pub semi_axis: PositiveLengthMeasure,
        pub semi_imag_axis: PositiveLengthMeasure,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = intersection_curve)]
    #[holder(generate_deserialize)]
    pub struct IntersectionCurve {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = item_defined_transformation)]
    #[holder(generate_deserialize)]
    pub struct ItemDefinedTransformation {
        pub name: Label,
        pub description: Text,
        #[holder(use_place_holder)]
        pub transform_item_1: RepresentationItemAny,
        #[holder(use_place_holder)]
        pub transform_item_2: RepresentationItemAny,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = length_measure_with_unit)]
    #[holder(generate_deserialize)]
    pub struct LengthMeasureWithUnit {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = length_unit)]
    #[holder(generate_deserialize)]
    pub struct LengthUnit {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = line)]
    #[holder(generate_deserialize)]
    pub struct Line {
        #[holder(use_place_holder)]
        pub pnt: CartesianPoint,
        #[holder(use_place_holder)]
        pub dir: Vector,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = local_time)]
    #[holder(generate_deserialize)]
    pub struct LocalTime {
        pub hour_component: HourInDay,
        pub minute_component: Option<MinuteInHour>,
        pub second_component: Option<SecondInMinute>,
        #[holder(use_place_holder)]
        pub zone: CoordinatedUniversalTimeOffset,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = r#loop)]
    #[holder(generate_deserialize)]
    pub struct Loop {}
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum LoopAny {
        #[holder(use_place_holder)]
        # [holder (field = edge_loop)]
        EdgeLoop(Box<EdgeLoop>),
        #[holder(use_place_holder)]
        # [holder (field = poly_loop)]
        PolyLoop(Box<PolyLoop>),
        #[holder(use_place_holder)]
        # [holder (field = vertex_loop)]
        VertexLoop(Box<VertexLoop>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = lot_effectivity)]
    #[holder(generate_deserialize)]
    pub struct LotEffectivity {
        pub effectivity_lot_id: Identifier,
        #[holder(use_place_holder)]
        pub effectivity_lot_size: MeasureWithUnitAny,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = manifold_solid_brep)]
    #[holder(generate_deserialize)]
    pub struct ManifoldSolidBrep {
        #[holder(use_place_holder)]
        pub outer: ClosedShellAny,
    }
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ManifoldSolidBrepAny {
        #[holder(use_place_holder)]
        # [holder (field = brep_with_voids)]
        BrepWithVoids(Box<BrepWithVoids>),
        #[holder(use_place_holder)]
        # [holder (field = faceted_brep)]
        FacetedBrep(Box<FacetedBrep>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = manifold_surface_shape_representation)]
    #[holder(generate_deserialize)]
    pub struct ManifoldSurfaceShapeRepresentation {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = mapped_item)]
    #[holder(generate_deserialize)]
    pub struct MappedItem {
        #[holder(use_place_holder)]
        pub mapping_source: RepresentationMap,
        #[holder(use_place_holder)]
        pub mapping_target: RepresentationItemAny,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = mass_measure_with_unit)]
    #[holder(generate_deserialize)]
    pub struct MassMeasureWithUnit {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = mass_unit)]
    #[holder(generate_deserialize)]
    pub struct MassUnit {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = measure_with_unit)]
    #[holder(generate_deserialize)]
    pub struct MeasureWithUnit {
        #[holder(use_place_holder)]
        pub value_component: MeasureValue,
        #[holder(use_place_holder)]
        pub unit_component: Unit,
    }
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum MeasureWithUnitAny {
        #[holder(use_place_holder)]
        # [holder (field = area_measure_with_unit)]
        AreaMeasureWithUnit(Box<AreaMeasureWithUnit>),
        #[holder(use_place_holder)]
        # [holder (field = length_measure_with_unit)]
        LengthMeasureWithUnit(Box<LengthMeasureWithUnit>),
        #[holder(use_place_holder)]
        # [holder (field = mass_measure_with_unit)]
        MassMeasureWithUnit(Box<MassMeasureWithUnit>),
        #[holder(use_place_holder)]
        # [holder (field = plane_angle_measure_with_unit)]
        PlaneAngleMeasureWithUnit(Box<PlaneAngleMeasureWithUnit>),
        #[holder(use_place_holder)]
        # [holder (field = solid_angle_measure_with_unit)]
        SolidAngleMeasureWithUnit(Box<SolidAngleMeasureWithUnit>),
        #[holder(use_place_holder)]
        # [holder (field = uncertainty_measure_with_unit)]
        UncertaintyMeasureWithUnit(Box<UncertaintyMeasureWithUnit>),
        #[holder(use_place_holder)]
        # [holder (field = volume_measure_with_unit)]
        VolumeMeasureWithUnit(Box<VolumeMeasureWithUnit>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = mechanical_context)]
    #[holder(generate_deserialize)]
    pub struct MechanicalContext {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = named_unit)]
    #[holder(generate_deserialize)]
    pub struct NamedUnit {
        #[holder(use_place_holder)]
        pub dimensions: DimensionalExponents,
    }
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum NamedUnitAny {
        #[holder(use_place_holder)]
        # [holder (field = area_unit)]
        AreaUnit(Box<AreaUnit>),
        #[holder(use_place_holder)]
        # [holder (field = context_dependent_unit)]
        ContextDependentUnit(Box<ContextDependentUnit>),
        #[holder(use_place_holder)]
        # [holder (field = conversion_based_unit)]
        ConversionBasedUnit(Box<ConversionBasedUnit>),
        #[holder(use_place_holder)]
        # [holder (field = length_unit)]
        LengthUnit(Box<LengthUnit>),
        #[holder(use_place_holder)]
        # [holder (field = mass_unit)]
        MassUnit(Box<MassUnit>),
        #[holder(use_place_holder)]
        # [holder (field = plane_angle_unit)]
        PlaneAngleUnit(Box<PlaneAngleUnit>),
        #[holder(use_place_holder)]
        # [holder (field = si_unit)]
        SiUnit(Box<SiUnit>),
        #[holder(use_place_holder)]
        # [holder (field = solid_angle_unit)]
        SolidAngleUnit(Box<SolidAngleUnit>),
        #[holder(use_place_holder)]
        # [holder (field = volume_unit)]
        VolumeUnit(Box<VolumeUnit>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = next_assembly_usage_occurrence)]
    #[holder(generate_deserialize)]
    pub struct NextAssemblyUsageOccurrence {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = offset_curve_3d)]
    #[holder(generate_deserialize)]
    pub struct OffsetCurve3D {
        #[holder(use_place_holder)]
        pub basis_curve: CurveAny,
        pub distance: LengthMeasure,
        pub self_intersect: Logical,
        #[holder(use_place_holder)]
        pub ref_direction: Direction,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = offset_surface)]
    #[holder(generate_deserialize)]
    pub struct OffsetSurface {
        #[holder(use_place_holder)]
        pub basis_surface: SurfaceAny,
        pub distance: LengthMeasure,
        pub self_intersect: Logical,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = open_shell)]
    #[holder(generate_deserialize)]
    pub struct OpenShell {}
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum OpenShellAny {
        #[holder(use_place_holder)]
        # [holder (field = oriented_open_shell)]
        OrientedOpenShell(Box<OrientedOpenShell>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = ordinal_date)]
    #[holder(generate_deserialize)]
    pub struct OrdinalDate {
        pub day_component: DayInYearNumber,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = organization)]
    #[holder(generate_deserialize)]
    pub struct Organization {
        pub id: Option<Identifier>,
        pub name: Label,
        pub description: Text,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = organization_relationship)]
    #[holder(generate_deserialize)]
    pub struct OrganizationRelationship {
        pub name: Label,
        pub description: Text,
        #[holder(use_place_holder)]
        pub relating_organization: Organization,
        #[holder(use_place_holder)]
        pub related_organization: Organization,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = organizational_address)]
    #[holder(generate_deserialize)]
    pub struct OrganizationalAddress {
        #[holder(use_place_holder)]
        pub organizations: Vec<Organization>,
        pub description: Text,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = organizational_project)]
    #[holder(generate_deserialize)]
    pub struct OrganizationalProject {
        pub name: Label,
        pub description: Text,
        #[holder(use_place_holder)]
        pub responsible_organizations: Vec<Organization>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = oriented_closed_shell)]
    #[holder(generate_deserialize)]
    pub struct OrientedClosedShell {
        #[holder(use_place_holder)]
        pub closed_shell_element: ClosedShellAny,
        pub orientation: bool,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = oriented_edge)]
    #[holder(generate_deserialize)]
    pub struct OrientedEdge {
        #[holder(use_place_holder)]
        pub edge_element: EdgeAny,
        pub orientation: bool,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = oriented_face)]
    #[holder(generate_deserialize)]
    pub struct OrientedFace {
        #[holder(use_place_holder)]
        pub face_element: FaceAny,
        pub orientation: bool,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = oriented_open_shell)]
    #[holder(generate_deserialize)]
    pub struct OrientedOpenShell {
        #[holder(use_place_holder)]
        pub open_shell_element: OpenShellAny,
        pub orientation: bool,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = oriented_path)]
    #[holder(generate_deserialize)]
    pub struct OrientedPath {
        #[holder(use_place_holder)]
        pub path_element: PathAny,
        pub orientation: bool,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = outer_boundary_curve)]
    #[holder(generate_deserialize)]
    pub struct OuterBoundaryCurve {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = parabola)]
    #[holder(generate_deserialize)]
    pub struct Parabola {
        pub focal_dist: LengthMeasure,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = parametric_representation_context)]
    #[holder(generate_deserialize)]
    pub struct ParametricRepresentationContext {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = path)]
    #[holder(generate_deserialize)]
    pub struct Path {
        #[holder(use_place_holder)]
        pub edge_list: Vec<OrientedEdge>,
    }
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum PathAny {
        #[holder(use_place_holder)]
        # [holder (field = edge_loop)]
        EdgeLoop(Box<EdgeLoop>),
        #[holder(use_place_holder)]
        # [holder (field = oriented_path)]
        OrientedPath(Box<OrientedPath>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = pcurve)]
    #[holder(generate_deserialize)]
    pub struct Pcurve {
        #[holder(use_place_holder)]
        pub basis_surface: SurfaceAny,
        #[holder(use_place_holder)]
        pub reference_to_curve: DefinitionalRepresentation,
    }
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum PcurveAny {
        #[holder(use_place_holder)]
        # [holder (field = bounded_pcurve)]
        BoundedPcurve(Box<BoundedPcurve>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
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
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = person_and_organization)]
    #[holder(generate_deserialize)]
    pub struct PersonAndOrganization {
        #[holder(use_place_holder)]
        pub the_person: Person,
        #[holder(use_place_holder)]
        pub the_organization: Organization,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = person_and_organization_assignment)]
    #[holder(generate_deserialize)]
    pub struct PersonAndOrganizationAssignment {
        #[holder(use_place_holder)]
        pub assigned_person_and_organization: PersonAndOrganization,
        #[holder(use_place_holder)]
        pub role: PersonAndOrganizationRole,
    }
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum PersonAndOrganizationAssignmentAny {
        #[holder(use_place_holder)]
        # [holder (field = cc_design_person_and_organization_assignment)]
        CcDesignPersonAndOrganizationAssignment(Box<CcDesignPersonAndOrganizationAssignment>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = person_and_organization_role)]
    #[holder(generate_deserialize)]
    pub struct PersonAndOrganizationRole {
        pub name: Label,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = personal_address)]
    #[holder(generate_deserialize)]
    pub struct PersonalAddress {
        #[holder(use_place_holder)]
        pub people: Vec<Person>,
        pub description: Text,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = placement)]
    #[holder(generate_deserialize)]
    pub struct Placement {
        #[holder(use_place_holder)]
        pub location: CartesianPoint,
    }
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum PlacementAny {
        #[holder(use_place_holder)]
        # [holder (field = axis1_placement)]
        Axis1Placement(Box<Axis1Placement>),
        #[holder(use_place_holder)]
        # [holder (field = axis2_placement_2d)]
        Axis2Placement2D(Box<Axis2Placement2D>),
        #[holder(use_place_holder)]
        # [holder (field = axis2_placement_3d)]
        Axis2Placement3D(Box<Axis2Placement3D>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = plane)]
    #[holder(generate_deserialize)]
    pub struct Plane {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = plane_angle_measure_with_unit)]
    #[holder(generate_deserialize)]
    pub struct PlaneAngleMeasureWithUnit {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = plane_angle_unit)]
    #[holder(generate_deserialize)]
    pub struct PlaneAngleUnit {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = point)]
    #[holder(generate_deserialize)]
    pub struct Point {}
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum PointAny {
        #[holder(use_place_holder)]
        # [holder (field = cartesian_point)]
        CartesianPoint(Box<CartesianPoint>),
        #[holder(use_place_holder)]
        # [holder (field = degenerate_pcurve)]
        DegeneratePcurve(Box<DegeneratePcurve>),
        #[holder(use_place_holder)]
        # [holder (field = point_on_curve)]
        PointOnCurve(Box<PointOnCurve>),
        #[holder(use_place_holder)]
        # [holder (field = point_on_surface)]
        PointOnSurface(Box<PointOnSurface>),
        #[holder(use_place_holder)]
        # [holder (field = point_replica)]
        PointReplica(Box<PointReplica>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = point_on_curve)]
    #[holder(generate_deserialize)]
    pub struct PointOnCurve {
        #[holder(use_place_holder)]
        pub basis_curve: CurveAny,
        pub point_parameter: ParameterValue,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = point_on_surface)]
    #[holder(generate_deserialize)]
    pub struct PointOnSurface {
        #[holder(use_place_holder)]
        pub basis_surface: SurfaceAny,
        pub point_parameter_u: ParameterValue,
        pub point_parameter_v: ParameterValue,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = point_replica)]
    #[holder(generate_deserialize)]
    pub struct PointReplica {
        #[holder(use_place_holder)]
        pub parent_pt: PointAny,
        #[holder(use_place_holder)]
        pub transformation: CartesianTransformationOperatorAny,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = poly_loop)]
    #[holder(generate_deserialize)]
    pub struct PolyLoop {
        #[holder(use_place_holder)]
        pub polygon: Vec<CartesianPoint>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = polyline)]
    #[holder(generate_deserialize)]
    pub struct Polyline {
        #[holder(use_place_holder)]
        pub points: Vec<CartesianPoint>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = product)]
    #[holder(generate_deserialize)]
    pub struct Product {
        pub id: Identifier,
        pub name: Label,
        pub description: Text,
        #[holder(use_place_holder)]
        pub frame_of_reference: Vec<ProductContextAny>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = product_category)]
    #[holder(generate_deserialize)]
    pub struct ProductCategory {
        pub name: Label,
        pub description: Option<Text>,
    }
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ProductCategoryAny {
        #[holder(use_place_holder)]
        # [holder (field = product_related_product_category)]
        ProductRelatedProductCategory(Box<ProductRelatedProductCategory>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = product_category_relationship)]
    #[holder(generate_deserialize)]
    pub struct ProductCategoryRelationship {
        pub name: Label,
        pub description: Text,
        #[holder(use_place_holder)]
        pub category: ProductCategoryAny,
        #[holder(use_place_holder)]
        pub sub_category: ProductCategoryAny,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = product_concept)]
    #[holder(generate_deserialize)]
    pub struct ProductConcept {
        pub id: Identifier,
        pub name: Label,
        pub description: Text,
        #[holder(use_place_holder)]
        pub market_context: ProductConceptContext,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = product_concept_context)]
    #[holder(generate_deserialize)]
    pub struct ProductConceptContext {
        pub market_segment_type: Label,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = product_context)]
    #[holder(generate_deserialize)]
    pub struct ProductContext {
        pub discipline_type: Label,
    }
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ProductContextAny {
        #[holder(use_place_holder)]
        # [holder (field = mechanical_context)]
        MechanicalContext(Box<MechanicalContext>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = product_definition)]
    #[holder(generate_deserialize)]
    pub struct ProductDefinition {
        pub id: Identifier,
        pub description: Text,
        #[holder(use_place_holder)]
        pub formation: ProductDefinitionFormationAny,
        #[holder(use_place_holder)]
        pub frame_of_reference: ProductDefinitionContextAny,
    }
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ProductDefinitionAny {
        #[holder(use_place_holder)]
        # [holder (field = product_definition_with_associated_documents)]
        ProductDefinitionWithAssociatedDocuments(Box<ProductDefinitionWithAssociatedDocuments>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = product_definition_context)]
    #[holder(generate_deserialize)]
    pub struct ProductDefinitionContext {
        pub life_cycle_stage: Label,
    }
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ProductDefinitionContextAny {
        #[holder(use_place_holder)]
        # [holder (field = design_context)]
        DesignContext(Box<DesignContext>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = product_definition_effectivity)]
    #[holder(generate_deserialize)]
    pub struct ProductDefinitionEffectivity {
        #[holder(use_place_holder)]
        pub usage: ProductDefinitionRelationshipAny,
    }
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ProductDefinitionEffectivityAny {
        #[holder(use_place_holder)]
        # [holder (field = configuration_effectivity)]
        ConfigurationEffectivity(Box<ConfigurationEffectivity>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = product_definition_formation)]
    #[holder(generate_deserialize)]
    pub struct ProductDefinitionFormation {
        pub id: Identifier,
        pub description: Text,
        #[holder(use_place_holder)]
        pub of_product: Product,
    }
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ProductDefinitionFormationAny {
        #[holder(use_place_holder)]
        # [holder (field = product_definition_formation_with_specified_source)]
        ProductDefinitionFormationWithSpecifiedSource(
            Box<ProductDefinitionFormationWithSpecifiedSource>,
        ),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = product_definition_formation_with_specified_source)]
    #[holder(generate_deserialize)]
    pub struct ProductDefinitionFormationWithSpecifiedSource {
        pub make_or_buy: Source,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = product_definition_relationship)]
    #[holder(generate_deserialize)]
    pub struct ProductDefinitionRelationship {
        pub id: Identifier,
        pub name: Label,
        pub description: Text,
        #[holder(use_place_holder)]
        pub relating_product_definition: ProductDefinitionAny,
        #[holder(use_place_holder)]
        pub related_product_definition: ProductDefinitionAny,
    }
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ProductDefinitionRelationshipAny {
        #[holder(use_place_holder)]
        # [holder (field = design_make_from_relationship)]
        DesignMakeFromRelationship(Box<DesignMakeFromRelationship>),
        #[holder(use_place_holder)]
        # [holder (field = product_definition_usage)]
        ProductDefinitionUsage(Box<ProductDefinitionUsage>),
        #[holder(use_place_holder)]
        # [holder (field = supplied_part_relationship)]
        SuppliedPartRelationship(Box<SuppliedPartRelationship>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = product_definition_shape)]
    #[holder(generate_deserialize)]
    pub struct ProductDefinitionShape {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = product_definition_usage)]
    #[holder(generate_deserialize)]
    pub struct ProductDefinitionUsage {}
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ProductDefinitionUsageAny {
        #[holder(use_place_holder)]
        # [holder (field = assembly_component_usage)]
        AssemblyComponentUsage(Box<AssemblyComponentUsage>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = product_definition_with_associated_documents)]
    #[holder(generate_deserialize)]
    pub struct ProductDefinitionWithAssociatedDocuments {
        #[holder(use_place_holder)]
        pub documentation_ids: Vec<DocumentAny>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = product_related_product_category)]
    #[holder(generate_deserialize)]
    pub struct ProductRelatedProductCategory {
        #[holder(use_place_holder)]
        pub products: Vec<Product>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = promissory_usage_occurrence)]
    #[holder(generate_deserialize)]
    pub struct PromissoryUsageOccurrence {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = property_definition)]
    #[holder(generate_deserialize)]
    pub struct PropertyDefinition {
        pub name: Label,
        pub description: Text,
        #[holder(use_place_holder)]
        pub definition: CharacterizedDefinition,
    }
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum PropertyDefinitionAny {
        #[holder(use_place_holder)]
        # [holder (field = product_definition_shape)]
        ProductDefinitionShape(Box<ProductDefinitionShape>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = property_definition_representation)]
    #[holder(generate_deserialize)]
    pub struct PropertyDefinitionRepresentation {
        #[holder(use_place_holder)]
        pub definition: PropertyDefinitionAny,
        #[holder(use_place_holder)]
        pub used_representation: RepresentationAny,
    }
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum PropertyDefinitionRepresentationAny {
        #[holder(use_place_holder)]
        # [holder (field = shape_definition_representation)]
        ShapeDefinitionRepresentation(Box<ShapeDefinitionRepresentation>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = quantified_assembly_component_usage)]
    #[holder(generate_deserialize)]
    pub struct QuantifiedAssemblyComponentUsage {
        #[holder(use_place_holder)]
        pub quantity: MeasureWithUnitAny,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = quasi_uniform_curve)]
    #[holder(generate_deserialize)]
    pub struct QuasiUniformCurve {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = quasi_uniform_surface)]
    #[holder(generate_deserialize)]
    pub struct QuasiUniformSurface {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = rational_b_spline_curve)]
    #[holder(generate_deserialize)]
    pub struct RationalBSplineCurve {
        pub weights_data: Vec<f64>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = rational_b_spline_surface)]
    #[holder(generate_deserialize)]
    pub struct RationalBSplineSurface {
        pub weights_data: Vec<Vec<f64>>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = rectangular_composite_surface)]
    #[holder(generate_deserialize)]
    pub struct RectangularCompositeSurface {
        #[holder(use_place_holder)]
        pub segments: Vec<Vec<SurfacePatch>>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = rectangular_trimmed_surface)]
    #[holder(generate_deserialize)]
    pub struct RectangularTrimmedSurface {
        #[holder(use_place_holder)]
        pub basis_surface: SurfaceAny,
        pub u1: ParameterValue,
        pub u2: ParameterValue,
        pub v1: ParameterValue,
        pub v2: ParameterValue,
        pub usense: bool,
        pub vsense: bool,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = reparametrised_composite_curve_segment)]
    #[holder(generate_deserialize)]
    pub struct ReparametrisedCompositeCurveSegment {
        pub param_length: ParameterValue,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
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
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum RepresentationAny {
        #[holder(use_place_holder)]
        # [holder (field = definitional_representation)]
        DefinitionalRepresentation(Box<DefinitionalRepresentation>),
        #[holder(use_place_holder)]
        # [holder (field = shape_representation)]
        ShapeRepresentation(Box<ShapeRepresentation>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = representation_context)]
    #[holder(generate_deserialize)]
    pub struct RepresentationContext {
        pub context_identifier: Identifier,
        pub context_type: Text,
    }
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum RepresentationContextAny {
        #[holder(use_place_holder)]
        # [holder (field = geometric_representation_context)]
        GeometricRepresentationContext(Box<GeometricRepresentationContext>),
        #[holder(use_place_holder)]
        # [holder (field = global_uncertainty_assigned_context)]
        GlobalUncertaintyAssignedContext(Box<GlobalUncertaintyAssignedContext>),
        #[holder(use_place_holder)]
        # [holder (field = global_unit_assigned_context)]
        GlobalUnitAssignedContext(Box<GlobalUnitAssignedContext>),
        #[holder(use_place_holder)]
        # [holder (field = parametric_representation_context)]
        ParametricRepresentationContext(Box<ParametricRepresentationContext>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = representation_item)]
    #[holder(generate_deserialize)]
    pub struct RepresentationItem {
        pub name: Label,
    }
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
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
        # [holder (field = topological_representation_item)]
        TopologicalRepresentationItem(Box<TopologicalRepresentationItem>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = representation_map)]
    #[holder(generate_deserialize)]
    pub struct RepresentationMap {
        #[holder(use_place_holder)]
        pub mapping_origin: RepresentationItemAny,
        #[holder(use_place_holder)]
        pub mapped_representation: RepresentationAny,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = representation_relationship)]
    #[holder(generate_deserialize)]
    pub struct RepresentationRelationship {
        pub name: Label,
        pub description: Text,
        #[holder(use_place_holder)]
        pub rep_1: RepresentationAny,
        #[holder(use_place_holder)]
        pub rep_2: RepresentationAny,
    }
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum RepresentationRelationshipAny {
        #[holder(use_place_holder)]
        # [holder (field = representation_relationship_with_transformation)]
        RepresentationRelationshipWithTransformation(
            Box<RepresentationRelationshipWithTransformation>,
        ),
        #[holder(use_place_holder)]
        # [holder (field = shape_representation_relationship)]
        ShapeRepresentationRelationship(Box<ShapeRepresentationRelationship>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = representation_relationship_with_transformation)]
    #[holder(generate_deserialize)]
    pub struct RepresentationRelationshipWithTransformation {
        #[holder(use_place_holder)]
        pub transformation_operator: Transformation,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = seam_curve)]
    #[holder(generate_deserialize)]
    pub struct SeamCurve {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = security_classification)]
    #[holder(generate_deserialize)]
    pub struct SecurityClassification {
        pub name: Label,
        pub purpose: Text,
        #[holder(use_place_holder)]
        pub security_level: SecurityClassificationLevel,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = security_classification_assignment)]
    #[holder(generate_deserialize)]
    pub struct SecurityClassificationAssignment {
        #[holder(use_place_holder)]
        pub assigned_security_classification: SecurityClassification,
    }
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum SecurityClassificationAssignmentAny {
        #[holder(use_place_holder)]
        # [holder (field = cc_design_security_classification)]
        CcDesignSecurityClassification(Box<CcDesignSecurityClassification>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = security_classification_level)]
    #[holder(generate_deserialize)]
    pub struct SecurityClassificationLevel {
        pub name: Label,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = serial_numbered_effectivity)]
    #[holder(generate_deserialize)]
    pub struct SerialNumberedEffectivity {
        pub effectivity_start_id: Identifier,
        pub effectivity_end_id: Option<Identifier>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = shape_aspect)]
    #[holder(generate_deserialize)]
    pub struct ShapeAspect {
        pub name: Label,
        pub description: Text,
        #[holder(use_place_holder)]
        pub of_shape: ProductDefinitionShape,
        pub product_definitional: Logical,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = shape_aspect_relationship)]
    #[holder(generate_deserialize)]
    pub struct ShapeAspectRelationship {
        pub name: Label,
        pub description: Text,
        #[holder(use_place_holder)]
        pub relating_shape_aspect: ShapeAspect,
        #[holder(use_place_holder)]
        pub related_shape_aspect: ShapeAspect,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = shape_definition_representation)]
    #[holder(generate_deserialize)]
    pub struct ShapeDefinitionRepresentation {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = shape_representation)]
    #[holder(generate_deserialize)]
    pub struct ShapeRepresentation {}
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ShapeRepresentationAny {
        #[holder(use_place_holder)]
        # [holder (field = advanced_brep_shape_representation)]
        AdvancedBrepShapeRepresentation(Box<AdvancedBrepShapeRepresentation>),
        #[holder(use_place_holder)]
        # [holder (field = edge_based_wireframe_shape_representation)]
        EdgeBasedWireframeShapeRepresentation(Box<EdgeBasedWireframeShapeRepresentation>),
        #[holder(use_place_holder)]
        # [holder (field = faceted_brep_shape_representation)]
        FacetedBrepShapeRepresentation(Box<FacetedBrepShapeRepresentation>),
        #[holder(use_place_holder)]
        # [holder (field = geometrically_bounded_surface_shape_representation)]
        GeometricallyBoundedSurfaceShapeRepresentation(
            Box<GeometricallyBoundedSurfaceShapeRepresentation>,
        ),
        #[holder(use_place_holder)]
        # [holder (field = geometrically_bounded_wireframe_shape_representation)]
        GeometricallyBoundedWireframeShapeRepresentation(
            Box<GeometricallyBoundedWireframeShapeRepresentation>,
        ),
        #[holder(use_place_holder)]
        # [holder (field = manifold_surface_shape_representation)]
        ManifoldSurfaceShapeRepresentation(Box<ManifoldSurfaceShapeRepresentation>),
        #[holder(use_place_holder)]
        # [holder (field = shell_based_wireframe_shape_representation)]
        ShellBasedWireframeShapeRepresentation(Box<ShellBasedWireframeShapeRepresentation>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = shape_representation_relationship)]
    #[holder(generate_deserialize)]
    pub struct ShapeRepresentationRelationship {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = shell_based_surface_model)]
    #[holder(generate_deserialize)]
    pub struct ShellBasedSurfaceModel {
        #[holder(use_place_holder)]
        pub sbsm_boundary: Vec<Shell>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = shell_based_wireframe_model)]
    #[holder(generate_deserialize)]
    pub struct ShellBasedWireframeModel {
        #[holder(use_place_holder)]
        pub sbwm_boundary: Vec<Shell>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = shell_based_wireframe_shape_representation)]
    #[holder(generate_deserialize)]
    pub struct ShellBasedWireframeShapeRepresentation {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = si_unit)]
    #[holder(generate_deserialize)]
    pub struct SiUnit {
        pub prefix: Option<SiPrefix>,
        pub name: SiUnitName,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = solid_angle_measure_with_unit)]
    #[holder(generate_deserialize)]
    pub struct SolidAngleMeasureWithUnit {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = solid_angle_unit)]
    #[holder(generate_deserialize)]
    pub struct SolidAngleUnit {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = solid_model)]
    #[holder(generate_deserialize)]
    pub struct SolidModel {}
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum SolidModelAny {
        #[holder(use_place_holder)]
        # [holder (field = manifold_solid_brep)]
        ManifoldSolidBrep(Box<ManifoldSolidBrep>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = specified_higher_usage_occurrence)]
    #[holder(generate_deserialize)]
    pub struct SpecifiedHigherUsageOccurrence {
        #[holder(use_place_holder)]
        pub upper_usage: AssemblyComponentUsageAny,
        #[holder(use_place_holder)]
        pub next_usage: NextAssemblyUsageOccurrence,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = spherical_surface)]
    #[holder(generate_deserialize)]
    pub struct SphericalSurface {
        pub radius: PositiveLengthMeasure,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = start_request)]
    #[holder(generate_deserialize)]
    pub struct StartRequest {
        #[holder(use_place_holder)]
        pub items: Vec<StartRequestItem>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = start_work)]
    #[holder(generate_deserialize)]
    pub struct StartWork {
        #[holder(use_place_holder)]
        pub items: Vec<WorkItem>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = supplied_part_relationship)]
    #[holder(generate_deserialize)]
    pub struct SuppliedPartRelationship {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = surface)]
    #[holder(generate_deserialize)]
    pub struct Surface {}
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum SurfaceAny {
        #[holder(use_place_holder)]
        # [holder (field = bounded_surface)]
        BoundedSurface(Box<BoundedSurface>),
        #[holder(use_place_holder)]
        # [holder (field = elementary_surface)]
        ElementarySurface(Box<ElementarySurface>),
        #[holder(use_place_holder)]
        # [holder (field = offset_surface)]
        OffsetSurface(Box<OffsetSurface>),
        #[holder(use_place_holder)]
        # [holder (field = surface_replica)]
        SurfaceReplica(Box<SurfaceReplica>),
        #[holder(use_place_holder)]
        # [holder (field = swept_surface)]
        SweptSurface(Box<SweptSurface>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = surface_curve)]
    #[holder(generate_deserialize)]
    pub struct SurfaceCurve {
        #[holder(use_place_holder)]
        pub curve_3d: CurveAny,
        #[holder(use_place_holder)]
        pub associated_geometry: Vec<PcurveOrSurface>,
        pub master_representation: PreferredSurfaceCurveRepresentation,
    }
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum SurfaceCurveAny {
        #[holder(use_place_holder)]
        # [holder (field = bounded_surface_curve)]
        BoundedSurfaceCurve(Box<BoundedSurfaceCurve>),
        #[holder(use_place_holder)]
        # [holder (field = intersection_curve)]
        IntersectionCurve(Box<IntersectionCurve>),
        #[holder(use_place_holder)]
        # [holder (field = seam_curve)]
        SeamCurve(Box<SeamCurve>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = surface_of_linear_extrusion)]
    #[holder(generate_deserialize)]
    pub struct SurfaceOfLinearExtrusion {
        #[holder(use_place_holder)]
        pub extrusion_axis: Vector,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = surface_of_revolution)]
    #[holder(generate_deserialize)]
    pub struct SurfaceOfRevolution {
        #[holder(use_place_holder)]
        pub axis_position: Axis1Placement,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = surface_patch)]
    #[holder(generate_deserialize)]
    pub struct SurfacePatch {
        #[holder(use_place_holder)]
        pub parent_surface: BoundedSurfaceAny,
        pub u_transition: TransitionCode,
        pub v_transition: TransitionCode,
        pub u_sense: bool,
        pub v_sense: bool,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = surface_replica)]
    #[holder(generate_deserialize)]
    pub struct SurfaceReplica {
        #[holder(use_place_holder)]
        pub parent_surface: SurfaceAny,
        #[holder(use_place_holder)]
        pub transformation: CartesianTransformationOperator3D,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = swept_surface)]
    #[holder(generate_deserialize)]
    pub struct SweptSurface {
        #[holder(use_place_holder)]
        pub swept_curve: CurveAny,
    }
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum SweptSurfaceAny {
        #[holder(use_place_holder)]
        # [holder (field = surface_of_linear_extrusion)]
        SurfaceOfLinearExtrusion(Box<SurfaceOfLinearExtrusion>),
        #[holder(use_place_holder)]
        # [holder (field = surface_of_revolution)]
        SurfaceOfRevolution(Box<SurfaceOfRevolution>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = topological_representation_item)]
    #[holder(generate_deserialize)]
    pub struct TopologicalRepresentationItem {}
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum TopologicalRepresentationItemAny {
        #[holder(use_place_holder)]
        # [holder (field = connected_edge_set)]
        ConnectedEdgeSet(Box<ConnectedEdgeSet>),
        #[holder(use_place_holder)]
        # [holder (field = connected_face_set)]
        ConnectedFaceSet(Box<ConnectedFaceSet>),
        #[holder(use_place_holder)]
        # [holder (field = edge)]
        Edge(Box<Edge>),
        #[holder(use_place_holder)]
        # [holder (field = face)]
        Face(Box<Face>),
        #[holder(use_place_holder)]
        # [holder (field = face_bound)]
        FaceBound(Box<FaceBound>),
        #[holder(use_place_holder)]
        # [holder (field = loop)]
        Loop(Box<Loop>),
        #[holder(use_place_holder)]
        # [holder (field = path)]
        Path(Box<Path>),
        #[holder(use_place_holder)]
        # [holder (field = vertex)]
        Vertex(Box<Vertex>),
        #[holder(use_place_holder)]
        # [holder (field = vertex_shell)]
        VertexShell(Box<VertexShell>),
        #[holder(use_place_holder)]
        # [holder (field = wire_shell)]
        WireShell(Box<WireShell>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = toroidal_surface)]
    #[holder(generate_deserialize)]
    pub struct ToroidalSurface {
        pub major_radius: PositiveLengthMeasure,
        pub minor_radius: PositiveLengthMeasure,
    }
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ToroidalSurfaceAny {
        #[holder(use_place_holder)]
        # [holder (field = degenerate_toroidal_surface)]
        DegenerateToroidalSurface(Box<DegenerateToroidalSurface>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
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
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = uncertainty_measure_with_unit)]
    #[holder(generate_deserialize)]
    pub struct UncertaintyMeasureWithUnit {
        pub name: Label,
        pub description: Text,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = uniform_curve)]
    #[holder(generate_deserialize)]
    pub struct UniformCurve {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = uniform_surface)]
    #[holder(generate_deserialize)]
    pub struct UniformSurface {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = vector)]
    #[holder(generate_deserialize)]
    pub struct Vector {
        #[holder(use_place_holder)]
        pub orientation: Direction,
        pub magnitude: LengthMeasure,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = versioned_action_request)]
    #[holder(generate_deserialize)]
    pub struct VersionedActionRequest {
        pub id: Identifier,
        pub version: Label,
        pub purpose: Text,
        pub description: Text,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = vertex)]
    #[holder(generate_deserialize)]
    pub struct Vertex {}
    #[derive(Debug, Clone, PartialEq, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum VertexAny {
        #[holder(use_place_holder)]
        # [holder (field = vertex_point)]
        VertexPoint(Box<VertexPoint>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = vertex_loop)]
    #[holder(generate_deserialize)]
    pub struct VertexLoop {
        #[holder(use_place_holder)]
        pub loop_vertex: VertexAny,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = vertex_point)]
    #[holder(generate_deserialize)]
    pub struct VertexPoint {
        #[holder(use_place_holder)]
        pub vertex_geometry: PointAny,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = vertex_shell)]
    #[holder(generate_deserialize)]
    pub struct VertexShell {
        #[holder(use_place_holder)]
        pub vertex_shell_extent: VertexLoop,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = volume_measure_with_unit)]
    #[holder(generate_deserialize)]
    pub struct VolumeMeasureWithUnit {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = volume_unit)]
    #[holder(generate_deserialize)]
    pub struct VolumeUnit {}
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = week_of_year_and_day_date)]
    #[holder(generate_deserialize)]
    pub struct WeekOfYearAndDayDate {
        pub week_component: WeekInYearNumber,
        pub day_component: Option<DayInWeekNumber>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = wire_shell)]
    #[holder(generate_deserialize)]
    pub struct WireShell {
        #[holder(use_place_holder)]
        pub wire_shell_extent: Vec<LoopAny>,
    }
}

pub enum PrimitiveType {
    Integer = 0x0001,
    Real = 0x0002,
    Boolean = 0x0004,
    Logical = 0x0008,
    String = 0x0010,
    Binary = 0x0020,
    Enumeration = 0x0040,
    Select = 0x0080,
    Instance = 0x0100,
    Aggr = 0x0200,
    Number = 0x0400,
}

pub enum AccessType {
    ReadOnly,
    ReadWrite,
    Unset,
}

pub enum CommitMode {
    Commit,
    Abort,
}

pub enum AttrFlag {
    Set,
    Unset,
}

pub enum ImplementationClass {
    Class1,
    Class2,
    Class3,
    Class4,
    Class5,
}

/// error codes taken from 10303-23, Jan 28, 1997.
/// ISO TC184/SC4/WG11 N 004
pub enum ErrorKind {
    /// No error
    NoError = 0,
    /// Session open
    SessionOpen = 10,
    /// Session not available
    SessionNotAvailable = 20,
    /// Session is not open
    SessionClosed = 30,
    /// Repository does not exist
    RepositoryNonExistent = 40,
    /// Repository not available
    RepositoryNonAvailable = 50,
    /// Repository already opened
    RepositoryOpen = 60,
    /// Repository is not open
    RepositoryClosed = 70,
    /// Transaction ended abnormally so it no longer exists
    TransactionEndedAbnormally = 80,
    /// Transaction exists
    TransactionExists = 90,
    /// Transaction not available
    TransactionNonAvailable = 100,
    /// Transaction read-write
    TransactionReadWrite = 110,
    /// Transaction not read-write
    TransactionNonReadWrite = 120,
    /// Transaction does not exist
    TransactionNonExistent = 130,
    /// SDAI-model not domain-equivalent
    ModelNonDomainEquevalent = 140,
    /// SDAI-model does not exist
    ModelNonExistent = 150,
    /// SDAI-model invalid
    ModelInvalid = 160,
    /// SDAI-model duplicate
    ModelDuplicate = 170,
    /// SDAI-model access not read-write
    ModelAccessNonReadWrite = 180,
    /// SDAI-model access is not defined
    ModelAccessNonDefined = 190,
    /// SDAI-model access read-write
    ModelAccessReadWrite = 200,
    /// SDAI-model access read-only
    ModelAccessReadOnly = 210,
    /// Schema definition is not defined
    SchemaDefinitionNonDefined = 220,
    /// Entity definition not defined
    EntityDefinitionNonDefined = 230,
    /// Entity definition not domain equivalent
    EntityDefinitionNonDomainEquevalent = 240,
    /// Entity definition invalid
    EntityDefinitionInvalid = 250,
    // /// Entity definition not available
    //  EntityDefinitionNonAvailable =  250,
    /// Rule not defined
    RuleNonDefined = 260,
    /// Expression evaluation not supported
    ExpressionEvaluationNonSupported = 270,
    /// Attribute invalid
    AttributeInvalid = 280,
    /// Attribute not defined
    AttributeNonDefined = 290,
    /// Schema instance duplicate
    SchemaInstanceDuplicate = 300,
    /// Schema instance does not exist
    SchemaInstanceNonExistent = 310,
    /// Entity instance does not exist
    EntityInstanceNonExistent = 320,
    /// Entity instance not available
    EntityInstanceNonAvailable = 330,
    /// Entity instance invalid
    EntityInstanceInvalid = 340,
    // Entity instance not exported
    EntityInstanceNonExported = 350,
    /// Scope does not exist
    ScopeNonExistent = 360,
    /// Scope exists
    ScopeExists = 370,
    /// Aggregate instance does not exist
    AggregateInstanceNonExistent = 380,
    /// Aggregate instance invalid
    AggregateInstaceInvalid = 390,
    /// Aggregate instance is empty
    AggregateInstanceEmpty = 400,
    /// Value invalid
    ValueInvalid = 410,
    /// Value does not exist
    ValueNonExistent = 420,
    /// Value not set
    ValueNotSet = 430,
    /// Value type invalid
    ValueTypeInvalid = 440,
    /// Iterator does not exist
    IteratorNonExistent = 450,
    /// Current member is not defined
    CurrentMemberNonDefined = 460,
    /// Index invalid
    IndexInvalid = 470,
    /// Event recording not set
    EventRecordingNotSet = 480,
    /// Operator invalid
    OperatorInvalid = 490,
    /// Function not available
    FunctionNonAvailable = 500,
    /// Underlying system error
    UnderlyingSystemError = 1000,
}

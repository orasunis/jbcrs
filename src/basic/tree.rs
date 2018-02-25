//! The tree package provides the basic structure of a basic class file

use std::collections::{BTreeMap, HashMap};

/// A java class file.
#[derive(Debug)]
pub struct Class {
    pub minor_version: u16,
    pub major_version: u16,

    pub access_flags: AccessFlags,
    pub name: u16,
    pub super_name: u16,
    pub interfaces: Vec<u16>,

    pub fields: Vec<Field>,
    pub methods: Vec<Method>,

    pub attributes: Vec<Attribute>,
}

/// A field.
#[derive(Debug)]
pub struct Field {
    pub access_flags: AccessFlags,
    pub name: u16,
    pub desc: u16,
    pub attributes: Vec<Attribute>,
}

/// A method.
#[derive(Debug)]
pub struct Method {
    pub access_flags: AccessFlags,
    pub name: u16,
    pub desc: u16,
    pub attributes: Vec<Attribute>,
}

/// An Attribute.
#[derive(Debug)]
pub enum Attribute {
    AnnotationDefault(ElementValue),
    BootstrapMethods(Vec<BootstrapMethod>),
    Code {
        max_stack: u16,
        max_locals: u16,
        instructions: HashMap<u32, Instruction>,
        exceptions: Vec<Exception>,
        attributes: Vec<Attribute>,
    },
    ConstantValue(u16),
    Deprecated,
    EnclosingMethod {
        class_index: u16,
        method_index: u16,
    },
    Exceptions(Vec<u16>),
    InnerClasses(Vec<InnerClass>),
    LineNumberTable(Vec<LineNumber>),
    LocalVariableTable(Vec<LocalVariable>),
    LocalVariableTypeTable(Vec<LocalVariableType>),
    MethodParameters(Vec<MethodParameter>),
    Module {
        name: u16,
        flags: AccessFlags,
        version: u16,

        requires: Vec<Requirement>,
        exports: Vec<Export>,
        opens: Vec<Opening>,
        uses: Vec<u16>,
        provides: Vec<Provider>,
    },
    ModuleMainClass(u16),
    ModulePackages(Vec<u16>),
    RuntimeVisibleAnnotations(Vec<Annotation>),
    RuntimeInvisibleAnnotations(Vec<Annotation>),
    RuntimeVisibleParameterAnnotations(Vec<Vec<Annotation>>),
    RuntimeInvisibleParameterAnnotations(Vec<Vec<Annotation>>),
    RuntimeVisibleTypeAnnotations(Vec<TypeAnnotation>),
    RuntimeInvisibleTypeAnnotations(Vec<TypeAnnotation>),
    Signature(u16),
    Synthetic,
    SourceFile(u16),
    SourceDebugExtension(String),
    StackMapTable(Vec<StackMapFrame>),
    Unknown(u16, Vec<u8>),
}

bitflags! {
    /// The access flags of a part of the class
    pub struct AccessFlags: u16 {
        const PUBLIC       = 0b0000_0000_0000_0001;
        const PRIVATE      = 0b0000_0000_0000_0010;
        const PROTECTED    = 0b0000_0000_0000_0100;
        const STATIC       = 0b0000_0000_0000_1000;
        const FINAL        = 0b0000_0000_0001_0000;
        const SUPER        = 0b0000_0000_0010_0000;
        const SYNCHRONIZED = 0b0000_0000_0010_0000;
        const VOLATILE     = 0b0000_0000_0100_0000;
        const BRIDGE       = 0b0000_0000_0100_0000;
        const STATIC_PHASE = 0b0000_0000_0100_0000;
        const TRANSIENT    = 0b0000_0000_1000_0000;
        const VARARGS      = 0b0000_0000_1000_0000;
        const NATIVE       = 0b0000_0001_0000_0000;
        const INTERFACE    = 0b0000_0010_0000_0000;
        const ABSTRACT     = 0b0000_0100_0000_0000;
        const STRICT       = 0b0000_1000_0000_0000;
        const SYNTHETIC    = 0b0001_0000_0000_0000;
        const ANNOTATION   = 0b0010_0000_0000_0000;
        const ENUM         = 0b0100_0000_0000_0000;
        const MODULE       = 0b1000_0000_0000_0001;
        const MANDATED     = 0b1000_0000_0000_0001;
    }
}

#[derive(Debug)]
pub struct Exception {
    pub start: u16,
    pub end: u16,
    pub handler: u16,
    pub catch_type: u16,
}

#[derive(Debug)]
pub struct BootstrapMethod {
    pub method_ref: u16,
    pub arguments: Vec<u16>,
}

#[derive(Debug)]
pub struct LineNumber {
    pub start: u16,
    pub line_number: u16,
}

#[derive(Debug)]
pub enum Instruction {
    /// No operation
    NOP,

    /// Pushes null on the stack
    AConstNull,

    /// Pushes the int -1 on the stack
    IConstM1,

    /// Pushes the int 0 on the stack
    IConst0,
    /// Pushes the int 1 on the stack
    IConst1,
    /// Pushes the int 2 on the stack
    IConst2,
    /// Pushes the int 3 on the stack
    IConst3,
    /// Pushes the int 4 on the stack
    IConst4,
    /// Pushes the int 5 on the stack
    IConst5,

    /// Pushes the long 0 on the stack
    LConst0,
    /// Pushes the long 1 on the stack
    LConst1,

    /// Pushes the float 0 on the stack
    FConst0,
    /// Pushes the float 1 on the stack
    FConst1,
    /// Pushes the float 2 on the stack
    FConst2,

    /// Pushes the double 0 on the stack
    DConst0,
    /// Pushes the double 1 on the stack
    DConst1,

    /// Pushes a byte on the stack
    BIPush(i8),
    /// Pushes a short on the stack
    SIPush(i16),

    /// Pushes a constant from the constant pool on the stack
    LDC(u16),

    /// Pushes the int at a specific local variable index on the stack
    ILoad(u16),
    /// Pushes the long at a specific local variable index on the stack
    LLoad(u16),
    /// Pushes the float at a specific local variable index on the stack
    FLoad(u16),
    /// Pushes the double at a specific local variable index on the stack
    DLoad(u16),
    /// Pushes the reference at a specific local variable index on the stack
    ALoad(u16),

    /// Pushes the int at local variable index 0 on the stack
    ILoad0,
    /// Pushes the int at local variable index 1 on the stack
    ILoad1,
    /// Pushes the int at local variable index 2 on the stack
    ILoad2,
    /// Pushes the int at local variable index 3 on the stack
    ILoad3,

    /// Pushes the long at local variable index 0 on the stack
    LLoad0,
    /// Pushes the long at local variable index 1 on the stack
    LLoad1,
    /// Pushes the long at local variable index 2 on the stack
    LLoad2,
    /// Pushes the long at local variable index 3 on the stack
    LLoad3,

    /// Pushes the float at local variable index 0 on the stack
    FLoad0,
    /// Pushes the float at local variable index 1 on the stack
    FLoad1,
    /// Pushes the float at local variable index 2 on the stack
    FLoad2,
    /// Pushes the float at local variable index 3 on the stack
    FLoad3,

    /// Pushes the double at local variable index 0 on the stack
    DLoad0,
    /// Pushes the double at local variable index 1 on the stack
    DLoad1,
    /// Pushes the double at local variable index 2 on the stack
    DLoad2,
    /// Pushes the double at local variable index 3 on the stack
    DLoad3,

    /// Pushes the reference at local variable index 0 on the stack
    ALoad0,
    /// Pushes the reference at local variable index 1 on the stack
    ALoad1,
    /// Pushes the reference at local variable index 2 on the stack
    ALoad2,
    /// Pushes the reference at local variable index 3 on the stack
    ALoad3,

    /// Pushes the value from an int array, which is popped from the stack,
    /// at an index, which is popped from the stack as well, on the stack.
    IALoad,
    /// Pushes the value from an long array, which is popped from the stack,
    /// at an index, which is popped from the stack as well, on the stack.
    LALoad,
    /// Pushes the value from an float array, which is popped from the stack,
    /// at an index, which is popped from the stack as well, on the stack.
    FALoad,
    /// Pushes the value from an double array, which is popped from the stack,
    /// at an index, which is popped from the stack as well, on the stack.
    DALoad,
    /// Pushes the value from an reference array, which is popped from the stack,
    /// at an index, which is popped from the stack as well, on the stack.
    AALoad,
    /// Pushes the value from an byte array, which is popped from the stack,
    /// at an index, which is popped from the stack as well, on the stack.
    BALoad,
    /// Pushes the value from an char array, which is popped from the stack,
    /// at an index, which is popped from the stack as well, on the stack.
    CALoad,
    /// Pushes the value from an short array, which is popped from the stack,
    /// at an index, which is popped from the stack as well, on the stack.
    SALoad,

    IStore(u16),
    LStore(u16),
    FStore(u16),
    DStore(u16),
    AStore(u16),

    IStore0,
    IStore1,
    IStore2,
    IStore3,

    LStore0,
    LStore1,
    LStore2,
    LStore3,

    FStore0,
    FStore1,
    FStore2,
    FStore3,

    DStore0,
    DStore1,
    DStore2,
    DStore3,

    AStore0,
    AStore1,
    AStore2,
    AStore3,

    IAStore,
    LAStore,
    FAStore,
    DAStore,
    AAStore,
    BAStore,
    CAStore,
    SAStore,

    Pop,
    Pop2,

    Dup,
    DupX1,
    DupX2,

    Dup2,
    Dup2X1,
    Dup2X2,

    Swap,

    IAdd,
    LAdd,
    FAdd,
    DAdd,

    ISub,
    LSub,
    FSub,
    DSub,

    IMul,
    LMul,
    FMul,
    DMul,

    IDiv,
    LDiv,
    FDiv,
    DDiv,

    IRem,
    LRem,
    FRem,
    DRem,

    INeg,
    LNeg,
    FNeg,
    DNeg,

    IShL,
    LShL,

    IShR,
    LShR,

    IUShR,
    LUShR,

    IAnd,
    LAnd,

    IOr,
    LOr,

    IXOr,
    LXOr,

    IInc(u16, i16),

    I2L,
    I2F,
    I2D,

    L2I,
    L2F,
    L2D,

    F2I,
    F2L,
    F2D,

    D2I,
    D2L,
    D2F,

    I2B,
    I2C,
    I2S,

    LCmp,

    FCmpL,
    FCmpG,

    DCmpL,
    DCmpG,

    IfEq(i16),
    IfNE(i16),
    IfLT(i16),
    IfGE(i16),
    IfGT(i16),
    IfLE(i16),

    IfICmpEq(i16),
    IfICmpNE(i16),
    IfICmpLT(i16),
    IfICmpGE(i16),
    IfICmpGT(i16),
    IfICmpLE(i16),

    IfACmpEq(i16),
    IfACmpNE(i16),

    GoTo(i32),
    JSR(i32),
    Ret(u16),

    TableSwitch {
        default: i32,
        low: i32,
        high: i32,
        offsets: Vec<i32>,
    },
    LookupSwitch {
        default: i32,
        offsets: BTreeMap<i32, i32>,
    },

    IReturn,
    LReturn,
    FReturn,
    DReturn,
    AReturn,
    Return,

    GetStatic(u16),
    PutStatic(u16),
    GetField(u16),
    PutField(u16),

    InvokeVirtual(u16),
    InvokeSpecial(u16),
    InvokeStatic(u16),
    InvokeInterface(u16, u8),
    InvokeDynamic(u16),

    New(u16),
    NewArray(ArrayType),
    ANewArray(u16),
    ArrayLength,

    AThrow,

    CheckCast(u16),
    InstanceOf(u16),

    MonitorEnter,
    MonitorExit,

    MultiANewArray(u16, u8),

    IfNull(i16),
    IfNonNull(i16),

    BreakPoint,
    ImpDep1,
    ImpDep2,
}

#[derive(Debug)]
pub enum ArrayType {
    Boolean,
    Char,
    Float,
    Double,
    Byte,
    Short,
    Int,
    Long,
}

#[derive(Debug)]
pub struct InnerClass {
    pub inner_class_info: u16,
    pub outer_class_info: u16,
    pub inner_name: u16,
    pub inner_class_access_flags: AccessFlags,
}

#[derive(Debug)]
pub enum StackMapFrame {
    Same {
        offset_delta: u16,
    },
    Same1 {
        offset_delta: u16,
        stack: VerificationType,
    },
    Chop {
        offset_delta: u16,
        count: u8,
    },
    Append {
        offset_delta: u16,
        locals: Vec<VerificationType>,
    },
    Full {
        offset_delta: u16,
        locals: Vec<VerificationType>,
        stack: Vec<VerificationType>,
    },
}

#[derive(Debug)]
pub enum VerificationType {
    Top,
    Integer,
    Float,
    Double,
    Long,
    Null,
    UninitializedThis,
    Object(u16),
    Uninitialized(u16),
}

#[derive(Debug)]
pub struct Annotation {
    /// Must be an index to the constant pool with an `Item::UTF8(_)`
    /// representing a field descriptor
    pub type_index: u16,
    /// The value every single pair holds.
    /// The first part is an index to the constant pool,
    /// which must be an `Item::UTF8(_)`.
    /// The second one is the value itself.
    pub element_value_pairs: Vec<(u16, ElementValue)>,
}

#[derive(Debug)]
pub enum ElementValue {
    /// The index to the constant pool
    /// which must be an `Item::Integer(_)`.
    Byte(u16),
    /// The index to the constant pool
    /// which must be an `Item::Integer(_)`.
    Short(u16),
    /// The index to the constant pool
    /// which must be an `Item::Integer(_)`.
    Char(u16),
    /// The index to the constant pool
    /// which must be an `Item::Integer(_)`.
    Int(u16),
    /// The index to the constant pool
    /// which must be an `Item::Long(_)`.
    Long(u16),
    /// The index to the constant pool
    /// which must be an `Item::Float(_)`.
    Float(u16),
    /// The index to the constant pool
    /// which must be an `Item::Double(_)`.
    Double(u16),
    /// The index to the constant pool
    /// which must be an `Item::Integer(_)`.
    /// Yes, it really needs a constant pool entry for this.
    Boolean(u16),
    /// The index to the constant pool
    /// which must be an `Item::UTF8(_)`.
    String(u16),
    /// An enum constant.
    Enum {
        /// The index to the constant pool,
        /// which must be an `Item::UTF8(_)`.
        /// It results in the internal form of the binary name
        /// of the type of this enum constant.
        type_name: u16,
        /// The index to the constant pool,
        /// which must be an `Item::UTF8(_)`.
        /// It results in the simple name
        /// of this enum constant.
        const_name: u16,
    },
    /// A class literal.
    /// The index to the constant pool
    /// which must be an `Item::UTF8(_)`
    /// representing a return descriptor.
    Class(u16),
    /// Another annotation.
    Annotation(Box<Annotation>),
    /// Multiple `ElementValue`s
    Array(Vec<ElementValue>),
}

#[derive(Debug)]
pub struct TypeAnnotation {
    pub target_type: TargetType,
    pub target_path: Vec<TypePathElement>,
    pub annotation: Annotation,
}

#[derive(Debug)]
pub enum TargetType {
    /// Indicates that an annotation is present
    /// on the type parameter of a class.
    /// The index of the type parameter.
    TypeParameterClass(u8),
    /// Indicates that an annotation is present
    /// on the type parameter of a method.
    /// The index of the type parameter.
    TypeParameterMethod(u8),
    /// Indicates that an annotation is present
    /// on the implements or extends clause of a class.
    /// The index of the super type,
    /// 0xFFFF is the extends clause.
    SuperType(u16),
    /// Indicates that an annotation is present
    /// on a bound of a type parameter of a class.
    TypeParameterBoundClass {
        /// The index of the type parameter.
        type_parameter: u8,
        /// The index of the bound.
        bound_index: u8,
    },
    /// Indicates that an annotation is present
    /// on a bound of a type parameter of a method.
    TypeParameterBoundMethod {
        /// The index of the type parameter.
        type_parameter: u8,
        /// The index of the bound.
        bound_index: u8,
    },
    /// Indicates that an annotation is present
    /// on the type of a field declaration.
    EmptyField,
    /// Indicates that an annotation is present
    /// on the return type of a method
    /// or the type of a newly constructed object.
    EmptyReturn,
    /// Indicates that an annotation is present
    /// on the receiver type of a method.
    EmptyReceiver,
    /// Indicates that an annotation is present
    /// on the type in a formal parameter declaration.
    /// The index of the formal parameter.
    FormalParameter(u8),
    /// Indicates that an annotation is present
    /// on the type in a throws clause.
    /// The index into the table of the `Exceptions` attribute of the method.
    Throws(u16),
    /// Indicates that an annotation is present
    /// on the type in a local variable declaration.
    LocalVariable(Vec<LocalVariableTarget>),
    /// Indicates that an annotation is present
    /// on the type in a local variable declaration.
    ResourceVariable(Vec<LocalVariableTarget>),
    /// Indicates that an annotation is present
    /// on the type in an exception parameter declaration.
    Catch(u16),
    /// Indicates that an annotation is present
    /// on the type in an instanceof expression.
    OffsetInstanceOf(u16),
    /// Indicates that an annotation is present
    /// on the type in a new expression.
    OffsetNew(u16),
    /// Indicates that an annotation is present
    /// on the type before the ::new
    /// of a method reference expression.
    OffsetNewRef(u16),
    /// Indicates that an annotation is present
    /// on the type before the ::name
    /// of a method reference expression.
    OffsetRef(u16),
    /// Indicates that an annotation is present
    /// on the type of a cast expression.
    TypeArgumentCast { offset: u16, type_argument: u8 },
    /// Indicates that an annotation is present
    /// on the type of a method call expression.
    TypeArgumentMethod { offset: u16, type_argument: u8 },
    /// Indicates that an annotation is present
    /// on the type of a new expression.
    TypeArgumentConstructor { offset: u16, type_argument: u8 },
    /// Indicates that an annotation is present
    /// on the type of a ::new expression.
    TypeArgumentNewRef { offset: u16, type_argument: u8 },
    /// Indicates that an annotation is present
    /// on the type of a ::name expression.
    TypeArgumentRef { offset: u16, type_argument: u8 },
}

#[derive(Debug)]
pub struct TypePathElement {
    pub path_kind: TypePathKind,
    pub argument_index: u8,
}

#[derive(Debug)]
pub enum TypePathKind {
    /// Annotation is deeper in an array kind
    ArrayType,
    /// Annotation is deeper in a nested type
    NestedType,
    /// Annotation is on the bound of a wildcard type argument of a parameterized type
    WildcardType,
    /// Annotation is on a type argument of a parameterized type
    Type,
}

#[derive(Debug)]
pub struct LocalVariableTarget {
    /// Start of the Code.
    pub start: u16,
    /// Length of the Code.
    pub length: u16,
    /// The index in the local variable array of the current frame.
    /// double and long do occupy two spaces.
    pub index: u16,
}

/// An entry of the `LocalVariableTable`
#[derive(Debug)]
pub struct LocalVariable {
    /// Start of the Code.
    pub start: u16,
    /// Length of the Code.
    pub length: u16,
    /// The index to an `Item::UTF8(_)` representing a valid unqalified name.
    pub name: u16,
    /// The index to an `Item::UTF8(_)` representing a field/type descriptor.
    pub descriptor: u16,
    /// The index in the local variable array of the current frame.
    /// double and long do occupy two spaces.
    pub index: u16,
}

/// An entry of the `LocalVariableTypeTable`
#[derive(Debug)]
pub struct LocalVariableType {
    /// Start of the Code.
    pub start: u16,
    /// Length of the Code.
    pub length: u16,
    /// The index to an `Item::UTF8(_)` representing a valid unqalified name.
    pub name: u16,
    /// The index to an `Item::UTF8(_)` representing a field signature.
    pub signature: u16,
    /// The index in the local variable array of the current frame.
    /// double and long do occupy two spaces.
    pub index: u16,
}

#[derive(Debug)]
pub struct MethodParameter {
    pub name: u16,
    pub access_flags: AccessFlags,
}

#[derive(Debug)]
pub struct Requirement {
    pub index: u16,
    pub flags: AccessFlags,
    pub version: u16,
}

#[derive(Debug)]
pub struct Export {
    pub index: u16,
    pub flags: AccessFlags,
    pub to: Vec<u16>,
}

#[derive(Debug)]
pub struct Opening {
    pub index: u16,
    pub flags: AccessFlags,
    pub to: Vec<u16>,
}

#[derive(Debug)]
pub struct Provider {
    pub index: u16,
    pub with: Vec<u16>,
}

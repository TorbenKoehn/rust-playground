# ubni = Unary & Binary Node Interpreter

Basic Interpreter:

## Terms

Terms are atomic values. Nothing can be smaller than a Term.

Autoboxers exist that can automatically box a value to its respective type
depending on some syntax element.

```
Term:
  value:
    String

Autoboxer:
  boundaries:
    (String, String)
  precedence: 100
  pattern:
    RegularExpression
  escapeSequences:
    Map<Character, Function(returnType: String)>
  constructor:
    Function(parameters: (Term))

@Autoboxer
MyAutoboxer = Autoboxer(
  boundaries: ("{{", "}}")
  constructor: Interpolation
)


a = {{ some.value }}
a // Interpolation(Term(value: "some value"))

Term("1")
// Term(value: "1")

Term("hello world")
// Term(value: "hello world")

a = 1
a // Long(Term(value: "1"))
```

## Functions

### What is a function?

```
Function:
  parameters:
    Sequence<Parameter>
  typeParameters:
    Sequence<TypeParameter>
  returnType:
    Function
  expressions:
    Sequence<Expression>

Addition = Function(
  parameters : (a : Long, b : Long),
  computations : Sequence({
    // Computation for addition of a and b
    // This is all is defined and handled rust-internally, but the API is exposed
    a plus b
  })
)

// Syntax Sugar through => binary expression:
Addition = (a : Long, b : Long) => {
  a plus b
}

c = Addition(Long(5), Long(10))
c // Long(15)

c = Long(5) Addition Long(10)
c // Long(15)
```

### Type equals Constructor equals Function

```
// Define a data structure

MyInt = (value : Term) => {

}

```

```
1 + x * y
Addition(Long(1), Multiplication(Reference(x), Reference(y)))

Type == Function

x = 5
Assignment(Reference(x), Long(5))

x => 5
Function(Parameter(x), () => Long(5))

(a, b) => a + b
Function(Sequence(Parameter(a), Parameter(b)), () => Addition(A)

x : Long = 5
TypeAssignment(Assignment(Reference(x), Long(5)), Long)

x > 5 then {
  DoA()
} else {
  DoB()
}

// alternatively: x > 5 then => DoA() else => DoB()

Else(
  If(
    GreaterComparison(Reference(x), Long(5)),
    () => DoA()
  ),
  () => DoB()
)

x = x > 5 then { 10 } // alternatively 5 then => 10
x // Option<Long> -> Some(10) or None

y = x else { 100 } // alternatively x else => 100
y // Option<Long> -> Some(10) or Some(100)

Point class {
  x : Long
  y : Long

  area : Long

  construct method (x : Long, y : Long) {
    area = x + y
  }

  getX method () => x
  setX method (x : Long) => this . x = x
}

Class(Reference(Point), () => Sequence(
  ClassProperty(Long, Reference(x))
  ClassProperty(Long, Reference(y))
))
```

Type Hierarchy:

- Any (Universe)
  - Empty (0 Terms)
  - Unit (1 Term: \*)
  - Boolean (2 Terms: true, false)
  - Product
  - Primitive
    - Number
      - Integer
        - SignedInteger
          - Long
          - Int
          - Short
          - SByte
        - UnsignedInteger
          - ULong
          - UInt
          - UShort
          - Byte
      - Float
        - Float32
        - Float64
        - Decimal
    - Character
    - String : Sequence<Character>
      - characters: Character
  - Composition
    - Tuple
    - Sequence
    - Structure

```
Type Definition:



```

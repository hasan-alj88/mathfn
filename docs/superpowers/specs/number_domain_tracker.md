# Number Domain Operations Tracker

Tracks the implementation status of binary and unary operations across all number domains in the codebase.

## Design Rules
1. **Trait Separation**: Each trait implementation MUST reside in its own separate file.
2. **Type Symmetry**: Binary operations are always performed between operands of the same type (e.g., `Sub<T> for T`).

---

## 1. NaturalNumber (`NaturalNumber`)
Represented as a vector of base-$2^{128}$ limbs.

### Standard Binary Operations
| Trait | Rhs | Output | Status | File Path |
|---|---|---|---|---|
| `Add` | `NaturalNumber` | `NaturalNumber` | Implemented | [addition.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/NaturalNumbers/arthimitic/addition.rs) |
| `AddAssign` | `NaturalNumber` | `()` | Implemented | [addition.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/NaturalNumbers/arthimitic/addition.rs) |
| `Sub` | `NaturalNumber` | `IntegerNumber` | Implemented | [subtraction.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/NaturalNumbers/arthimitic/subtraction.rs) |
| `SubAssign` | `NaturalNumber` | `()` | N/A | Cannot store negative result in-place |
| `Mul` | `NaturalNumber` | `NaturalNumber` | Implemented | [multiplication.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/NaturalNumbers/arthimitic/multiplication.rs) |
| `MulAssign` | `NaturalNumber` | `()` | Implemented | [multiplication.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/NaturalNumbers/arthimitic/multiplication.rs) |
| `Div` | `NaturalNumber` | `RationalNumber` | `todo!()` | [division.rs] (Pending creation) |
| `DivAssign` | `NaturalNumber` | `()` | N/A | Division yields non-integer rational |
| `Rem` | `NaturalNumber` | `NaturalNumber` | `todo!()` | [remainder.rs] (Pending creation) |
| `RemAssign` | `NaturalNumber` | `()` | `todo!()` | [remainder.rs] (Pending creation) |
| `Pow<u32>` | `u32` | `NaturalNumber` | Implemented | [exponentiation.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/NaturalNumbers/arthimitic/exponentiation.rs) |
| `PowAssign<u32>` | `u32` | `()` | Implemented | [exponentiation.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/NaturalNumbers/arthimitic/exponentiation.rs) |

### Try Binary Operations
| Trait | Rhs | Output | Status | File Path |
|---|---|---|---|---|
| `TryAdd` | `NaturalNumber` | `NaturalNumber` | `todo!()` | - |
| `TryAddAssign` | `NaturalNumber` | `()` | `todo!()` | - |
| `TrySub` | `NaturalNumber` | `NaturalNumber` | Implemented | [subtraction.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/NaturalNumbers/arthimitic/subtraction.rs) |
| `TrySubAssign` | `NaturalNumber` | `()` | Implemented | [subtraction.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/NaturalNumbers/arthimitic/subtraction.rs) |
| `TryMul` | `NaturalNumber` | `NaturalNumber` | `todo!()` | - |
| `TryMulAssign` | `NaturalNumber` | `()` | `todo!()` | - |
| `TryDiv` | `NaturalNumber` | `NaturalNumber` | `todo!()` | - |
| `TryDivAssign` | `NaturalNumber` | `()` | `todo!()` | - |
| `TryRem` | `NaturalNumber` | `NaturalNumber` | `todo!()` | - |
| `TryRemAssign` | `NaturalNumber` | `()` | `todo!()` | - |
| `TryPow` | `u32` | `NaturalNumber` | `todo!()` | - |
| `TryPowAssign` | `u32` | `()` | `todo!()` | - |

### Unary Operations
| Trait | Output | Status | File Path |
|---|---|---|---|
| `Neg` | `IntegerNumber` | `todo!()` | - |
| `Double` | `NaturalNumber` | `todo!()` | - |
| `DoubleAssign` | `()` | `todo!()` | - |
| `Abs` | `NaturalNumber` | N/A (Always positive) | - |
| `Square` | `NaturalNumber` | `todo!()` | - |
| `SquareAssign` | `()` | `todo!()` | - |
| `Sqrt` | `RealNumber` | `todo!()` | - |
| `Log` | `RealNumber` | `todo!()` | - |
| `Ln` | `RealNumber` | `todo!()` | - |

### Try Unary Operations
| Trait | Output | Status | File Path |
|---|---|---|---|
| `TryDouble` | `NaturalNumber` | `todo!()` | - |
| `TryDoubleAssign` | `()` | `todo!()` | - |
| `TryAbs` | `NaturalNumber` | N/A | - |
| `TrySquare` | `NaturalNumber` | `todo!()` | - |
| `TrySquareAssign` | `()` | `todo!()` | - |
| `TrySqrt` | `NaturalNumber` | `todo!()` | - |
| `TryLog` | `NaturalNumber` | `todo!()` | - |
| `TryLn` | `NaturalNumber` | `todo!()` | - |

---

## 2. IntegerNumber (`IntegerNumber`)
Represented as sign + magnitude (`NaturalNumber`).

### Standard Binary Operations
| Trait | Rhs | Output | Status | File Path |
|---|---|---|---|---|
| `Add` | `IntegerNumber` | `IntegerNumber` | `todo!()` | [addition.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/IntegerNunbers/arthimatic/addition.rs) (Skeleton exists) |
| `AddAssign` | `IntegerNumber` | `()` | `todo!()` | - |
| `Sub` | `IntegerNumber` | `IntegerNumber` | `todo!()` | - |
| `SubAssign` | `IntegerNumber` | `()` | `todo!()` | - |
| `Mul` | `IntegerNumber` | `IntegerNumber` | `todo!()` | - |
| `MulAssign` | `IntegerNumber` | `()` | `todo!()` | - |
| `Div` | `IntegerNumber` | `RationalNumber` | `todo!()` | - |
| `DivAssign` | `IntegerNumber` | `()` | N/A | - |
| `Rem` | `IntegerNumber` | `IntegerNumber` | `todo!()` | - |
| `RemAssign` | `IntegerNumber` | `()` | `todo!()` | - |
| `Pow<u32>` | `u32` | `IntegerNumber` | `todo!()` | - |
| `PowAssign<u32>` | `u32` | `()` | `todo!()` | - |

### Try Binary Operations
| Trait | Rhs | Output | Status | File Path |
|---|---|---|---|---|
| `TryAdd` | `IntegerNumber` | `IntegerNumber` | `todo!()` | - |
| `TryAddAssign` | `IntegerNumber` | `()` | `todo!()` | - |
| `TrySub` | `IntegerNumber` | `IntegerNumber` | `todo!()` | - |
| `TrySubAssign` | `IntegerNumber` | `()` | `todo!()` | - |
| `TryMul` | `IntegerNumber` | `IntegerNumber` | `todo!()` | - |
| `TryMulAssign` | `IntegerNumber` | `()` | `todo!()` | - |
| `TryDiv` | `IntegerNumber` | `IntegerNumber` | `todo!()` | - |
| `TryDivAssign` | `IntegerNumber` | `()` | `todo!()` | - |
| `TryRem` | `IntegerNumber` | `IntegerNumber` | `todo!()` | - |
| `TryRemAssign` | `IntegerNumber` | `()` | `todo!()` | - |
| `TryPow` | `u32` | `IntegerNumber` | `todo!()` | - |
| `TryPowAssign` | `u32` | `()` | `todo!()` | - |

### Unary Operations
| Trait | Output | Status | File Path |
|---|---|---|---|
| `Neg` | `IntegerNumber` | `todo!()` | - |
| `Double` | `IntegerNumber` | `todo!()` | - |
| `DoubleAssign` | `()` | `todo!()` | - |
| `Abs` | `NaturalNumber` | Implemented | [abs.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/IntegerNunbers/arthimatic/abs.rs) |
| `Square` | `IntegerNumber` | `todo!()` | - |
| `SquareAssign` | `()` | `todo!()` | - |
| `Sqrt` | `RealNumber` | `todo!()` | - |
| `Log` | `RealNumber` | `todo!()` | - |
| `Ln` | `RealNumber` | `todo!()` | - |

### Try Unary Operations
| Trait | Output | Status | File Path |
|---|---|---|---|
| `TryDouble` | `IntegerNumber` | `todo!()` | - |
| `TryDoubleAssign` | `()` | `todo!()` | - |
| `TryAbs` | `NaturalNumber` | Implemented | [try_abs.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/IntegerNunbers/arthimatic/try_abs.rs) |
| `TrySquare` | `IntegerNumber` | `todo!()` | - |
| `TrySquareAssign` | `()` | `todo!()` | - |
| `TrySqrt` | `IntegerNumber` | `todo!()` | - |
| `TryLog` | `IntegerNumber` | `todo!()` | - |
| `TryLn` | `IntegerNumber` | `todo!()` | - |

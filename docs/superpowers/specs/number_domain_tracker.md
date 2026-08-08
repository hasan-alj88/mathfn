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
| `Add` | `NaturalNumber` | `NaturalNumber` | Implemented | [add.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/NaturalNumbers/arthimitic/add.rs) |
| `AddAssign` | `NaturalNumber` | `()` | Implemented | [add_assign.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/NaturalNumbers/arthimitic/add_assign.rs) |
| `Sub` | `NaturalNumber` | `IntegerNumber` | Implemented | [sub.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/NaturalNumbers/arthimitic/sub.rs) |
| `SubAssign` | `NaturalNumber` | `()` | N/A | Cannot store negative result in-place |
| `Mul` | `NaturalNumber` | `NaturalNumber` | Implemented | [mul.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/NaturalNumbers/arthimitic/mul.rs) |
| `MulAssign` | `NaturalNumber` | `()` | Implemented | [mul_assign.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/NaturalNumbers/arthimitic/mul_assign.rs) |
| `Div` | `NaturalNumber` | `RationalNumber` | Skeleton (`todo!()`) | [div.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/NaturalNumbers/arthimitic/div.rs) |
| `DivAssign` | `NaturalNumber` | `()` | N/A | Division yields non-integer rational |
| `Rem` | `NaturalNumber` | `NaturalNumber` | Skeleton (`todo!()`) | [rem.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/NaturalNumbers/arthimitic/rem.rs) |
| `RemAssign` | `NaturalNumber` | `()` | Skeleton (`todo!()`) | [rem_assign.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/NaturalNumbers/arthimitic/rem_assign.rs) |
| `Pow<u32>` | `u32` | `NaturalNumber` | Implemented | [pow.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/NaturalNumbers/arthimitic/pow.rs) |
| `PowAssign<u32>` | `u32` | `()` | Implemented | [pow_assign.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/NaturalNumbers/arthimitic/pow_assign.rs) |

### Try Binary Operations
| Trait | Rhs | Output | Status | File Path |
|---|---|---|---|---|
| `TryAdd` | `NaturalNumber` | `NaturalNumber` | Skeleton (`todo!()`) | [try_add.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/NaturalNumbers/arthimitic/try_add.rs) |
| `TryAddAssign` | `NaturalNumber` | `()` | Skeleton (`todo!()`) | [try_add_assign.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/NaturalNumbers/arthimitic/try_add_assign.rs) |
| `TrySub` | `NaturalNumber` | `NaturalNumber` | Implemented | [try_sub.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/NaturalNumbers/arthimitic/try_sub.rs) |
| `TrySubAssign` | `NaturalNumber` | `()` | Implemented | [try_sub_assign.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/NaturalNumbers/arthimitic/try_sub_assign.rs) |
| `TryMul` | `NaturalNumber` | `NaturalNumber` | Skeleton (`todo!()`) | [try_mul.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/NaturalNumbers/arthimitic/try_mul.rs) |
| `TryMulAssign` | `NaturalNumber` | `()` | Skeleton (`todo!()`) | [try_mul_assign.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/NaturalNumbers/arthimitic/try_mul_assign.rs) |
| `TryDiv` | `NaturalNumber` | `NaturalNumber` | Skeleton (`todo!()`) | [try_div.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/NaturalNumbers/arthimitic/try_div.rs) |
| `TryDivAssign` | `NaturalNumber` | `()` | Skeleton (`todo!()`) | [try_div_assign.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/NaturalNumbers/arthimitic/try_div_assign.rs) |
| `TryRem` | `NaturalNumber` | `NaturalNumber` | Skeleton (`todo!()`) | [try_rem.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/NaturalNumbers/arthimitic/try_rem.rs) |
| `TryRemAssign` | `NaturalNumber` | `()` | Skeleton (`todo!()`) | [try_rem_assign.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/NaturalNumbers/arthimitic/try_rem_assign.rs) |
| `TryPow` | `u32` | `NaturalNumber` | Skeleton (`todo!()`) | [try_pow.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/NaturalNumbers/arthimitic/try_pow.rs) |
| `TryPowAssign` | `u32` | `()` | Skeleton (`todo!()`) | [try_pow_assign.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/NaturalNumbers/arthimitic/try_pow_assign.rs) |

### Unary Operations
| Trait | Output | Status | File Path |
|---|---|---|---|
| `Neg` | `IntegerNumber` | Skeleton (`todo!()`) | [neg.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/NaturalNumbers/arthimitic/neg.rs) |
| `Double` | `NaturalNumber` | Skeleton (`todo!()`) | [double.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/NaturalNumbers/arthimitic/double.rs) |
| `DoubleAssign` | `()` | Skeleton (`todo!()`) | [double_assign.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/NaturalNumbers/arthimitic/double_assign.rs) |
| `Abs` | `NaturalNumber` | N/A (Always positive) | - |
| `Square` | `NaturalNumber` | Skeleton (`todo!()`) | [square.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/NaturalNumbers/arthimitic/square.rs) |
| `SquareAssign` | `()` | Skeleton (`todo!()`) | [square_assign.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/NaturalNumbers/arthimitic/square_assign.rs) |
| `Sqrt` | `RealNumber` | Skeleton (`todo!()`) | [sqrt.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/NaturalNumbers/arthimitic/sqrt.rs) |
| `Log` | `RealNumber` | Skeleton (`todo!()`) | [log.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/NaturalNumbers/arthimitic/log.rs) |
| `Ln` | `RealNumber` | Skeleton (`todo!()`) | [ln.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/NaturalNumbers/arthimitic/ln.rs) |

### Try Unary Operations
| Trait | Output | Status | File Path |
|---|---|---|---|
| `TryDouble` | `NaturalNumber` | Skeleton (`todo!()`) | [try_double.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/NaturalNumbers/arthimitic/try_double.rs) |
| `TryDoubleAssign` | `()` | Skeleton (`todo!()`) | [try_double_assign.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/NaturalNumbers/arthimitic/try_double_assign.rs) |
| `TryAbs` | `NaturalNumber` | N/A | - |
| `TrySquare` | `NaturalNumber` | Skeleton (`todo!()`) | [try_square.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/NaturalNumbers/arthimitic/try_square.rs) |
| `TrySquareAssign` | `()` | Skeleton (`todo!()`) | [try_square_assign.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/NaturalNumbers/arthimitic/try_square_assign.rs) |
| `TrySqrt` | `NaturalNumber` | Skeleton (`todo!()`) | [try_sqrt.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/NaturalNumbers/arthimitic/try_sqrt.rs) |
| `TryLog` | `NaturalNumber` | Skeleton (`todo!()`) | [try_log.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/NaturalNumbers/arthimitic/try_log.rs) |
| `TryLn` | `NaturalNumber` | Skeleton (`todo!()`) | [try_ln.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/NaturalNumbers/arthimitic/try_ln.rs) |

---

## 2. IntegerNumber (`IntegerNumber`)
Represented as sign + magnitude (`NaturalNumber`).

### Standard Binary Operations
| Trait | Rhs | Output | Status | File Path |
|---|---|---|---|---|
| `Add` | `IntegerNumber` | `IntegerNumber` | Skeleton (`todo!()`) | [add.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/IntegerNunbers/arthimatic/add.rs) |
| `AddAssign` | `IntegerNumber` | `()` | Skeleton (`todo!()`) | [add_assign.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/IntegerNunbers/arthimatic/add_assign.rs) |
| `Sub` | `IntegerNumber` | `IntegerNumber` | Skeleton (`todo!()`) | [sub.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/IntegerNunbers/arthimatic/sub.rs) |
| `SubAssign` | `IntegerNumber` | `()` | Skeleton (`todo!()`) | [sub_assign.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/IntegerNunbers/arthimatic/sub_assign.rs) |
| `Mul` | `IntegerNumber` | `IntegerNumber` | Skeleton (`todo!()`) | [mul.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/IntegerNunbers/arthimatic/mul.rs) |
| `MulAssign` | `IntegerNumber` | `()` | Skeleton (`todo!()`) | [mul_assign.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/IntegerNunbers/arthimatic/mul_assign.rs) |
| `Div` | `IntegerNumber` | `RationalNumber` | Skeleton (`todo!()`) | [div.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/IntegerNunbers/arthimatic/div.rs) |
| `DivAssign` | `IntegerNumber` | `()` | N/A | - |
| `Rem` | `IntegerNumber` | `IntegerNumber` | Skeleton (`todo!()`) | [rem.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/IntegerNunbers/arthimatic/rem.rs) |
| `RemAssign` | `IntegerNumber` | `()` | Skeleton (`todo!()`) | [rem_assign.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/IntegerNunbers/arthimatic/rem_assign.rs) |
| `Pow<u32>` | `u32` | `IntegerNumber` | Skeleton (`todo!()`) | [pow.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/IntegerNunbers/arthimatic/pow.rs) |
| `PowAssign<u32>` | `u32` | `()` | Skeleton (`todo!()`) | [pow_assign.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/IntegerNunbers/arthimatic/pow_assign.rs) |

### Try Binary Operations
| Trait | Rhs | Output | Status | File Path |
|---|---|---|---|---|
| `TryAdd` | `IntegerNumber` | `IntegerNumber` | Skeleton (`todo!()`) | [try_add.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/IntegerNunbers/arthimatic/try_add.rs) |
| `TryAddAssign` | `IntegerNumber` | `()` | Skeleton (`todo!()`) | [try_add_assign.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/IntegerNunbers/arthimatic/try_add_assign.rs) |
| `TrySub` | `IntegerNumber` | `IntegerNumber` | Skeleton (`todo!()`) | [try_sub.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/IntegerNunbers/arthimatic/try_sub.rs) |
| `TrySubAssign` | `IntegerNumber` | `()` | Skeleton (`todo!()`) | [try_sub_assign.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/IntegerNunbers/arthimatic/try_sub_assign.rs) |
| `TryMul` | `IntegerNumber` | `IntegerNumber` | Skeleton (`todo!()`) | [try_mul.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/IntegerNunbers/arthimatic/try_mul.rs) |
| `TryMulAssign` | `IntegerNumber` | `()` | Skeleton (`todo!()`) | [try_mul_assign.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/IntegerNunbers/arthimatic/try_mul_assign.rs) |
| `TryDiv` | `IntegerNumber` | `IntegerNumber` | Skeleton (`todo!()`) | [try_div.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/IntegerNunbers/arthimatic/try_div.rs) |
| `TryDivAssign` | `IntegerNumber` | `()` | Skeleton (`todo!()`) | [try_div_assign.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/IntegerNunbers/arthimatic/try_div_assign.rs) |
| `TryRem` | `IntegerNumber` | `IntegerNumber` | Skeleton (`todo!()`) | [try_rem.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/IntegerNunbers/arthimatic/try_rem.rs) |
| `TryRemAssign` | `IntegerNumber` | `()` | Skeleton (`todo!()`) | [try_rem_assign.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/IntegerNunbers/arthimatic/try_rem_assign.rs) |
| `TryPow` | `u32` | `IntegerNumber` | Skeleton (`todo!()`) | [try_pow.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/IntegerNunbers/arthimatic/try_pow.rs) |
| `TryPowAssign` | `u32` | `()` | Skeleton (`todo!()`) | [try_pow_assign.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/IntegerNunbers/arthimatic/try_pow_assign.rs) |

### Unary Operations
| Trait | Output | Status | File Path |
|---|---|---|---|
| `Neg` | `IntegerNumber` | Skeleton (`todo!()`) | [neg.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/IntegerNunbers/arthimatic/neg.rs) |
| `Double` | `IntegerNumber` | Skeleton (`todo!()`) | [double.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/IntegerNunbers/arthimatic/double.rs) |
| `DoubleAssign` | `()` | Skeleton (`todo!()`) | [double_assign.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/IntegerNunbers/arthimatic/double_assign.rs) |
| `Abs` | `NaturalNumber` | Implemented | [abs.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/IntegerNunbers/arthimatic/abs.rs) |
| `Square` | `IntegerNumber` | Skeleton (`todo!()`) | [square.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/IntegerNunbers/arthimatic/square.rs) |
| `SquareAssign` | `()` | Skeleton (`todo!()`) | [square_assign.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/IntegerNunbers/arthimatic/square_assign.rs) |
| `Sqrt` | `RealNumber` | Skeleton (`todo!()`) | [sqrt.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/IntegerNunbers/arthimatic/sqrt.rs) |
| `Log` | `RealNumber` | Skeleton (`todo!()`) | [log.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/IntegerNunbers/arthimatic/log.rs) |
| `Ln` | `RealNumber` | Skeleton (`todo!()`) | [ln.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/IntegerNunbers/arthimatic/ln.rs) |

### Try Unary Operations
| Trait | Output | Status | File Path |
|---|---|---|---|
| `TryDouble` | `IntegerNumber` | Skeleton (`todo!()`) | [try_double.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/IntegerNunbers/arthimatic/try_double.rs) |
| `TryDoubleAssign` | `()` | Skeleton (`todo!()`) | [try_double_assign.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/IntegerNunbers/arthimatic/try_double_assign.rs) |
| `TryAbs` | `NaturalNumber` | Implemented | [try_abs.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/IntegerNunbers/arthimatic/try_abs.rs) |
| `TrySquare` | `IntegerNumber` | Skeleton (`todo!()`) | [try_square.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/IntegerNunbers/arthimatic/try_square.rs) |
| `TrySquareAssign` | `()` | Skeleton (`todo!()`) | [try_square_assign.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/IntegerNunbers/arthimatic/try_square_assign.rs) |
| `TrySqrt` | `IntegerNumber` | Skeleton (`todo!()`) | [try_sqrt.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/IntegerNunbers/arthimatic/try_sqrt.rs) |
| `TryLog` | `IntegerNumber` | Skeleton (`todo!()`) | [try_log.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/IntegerNunbers/arthimatic/try_log.rs) |
| `TryLn` | `IntegerNumber` | Skeleton (`todo!()`) | [try_ln.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/IntegerNunbers/arthimatic/try_ln.rs) |

---

## 3. PositiveNaturalNumber (`PositiveNaturalNumber`)
Mathematically represents $N \ge 1$ by storing $N - 1$ internally using the natural number implementation.
Defines constructors `new` and `from_value`, and conversion helper `to_value`.

- **File Path**: [positive_natural_numbers.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/PositiveNaturalNumbers/positive_natural_numbers.rs)
- **Status**: Struct and helper methods Implemented. Operations pending.

---

## 4. RationalNumber (`RationalNumber`)
Represented as numerator (`NaturalNumber`), denominator (`PositiveNaturalNumber`), and universal sign (`Sign`).

- **File Path**: [rational_numbers.rs](file:///home/hhj/RustroverProjects/mathfn/src/math/RationalNumbers/rational_numbers.rs)
- **Status**: Struct and constructor Implemented. Operations pending.

// Copyright (C) 2019-2022 Aleo Systems Inc.
// This file is part of the snarkVM library.

// The snarkVM library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The snarkVM library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the snarkVM library. If not, see <https://www.gnu.org/licenses/>.

use super::*;

impl<E: Environment, I: IntegerType, M: Magnitude> PowFlagged<Integer<E, M>> for Integer<E, I> {
    type Output = (Self, Boolean<E>);

    /// Returns the `power` of `self` to the power of `other` and a `flag` indicating whether the operation overflowed.
    #[inline]
    fn pow_flagged(&self, other: &Integer<E, M>) -> Self::Output {
        // Determine the variable mode.
        if self.is_constant() && other.is_constant() {
            // Compute the result and the flag and return them as constants.
            witness!(|self, other| {
                // The cast to `u32` is safe since `Magnitude`s can only be `u8`, `u16`, or `u32`.
                let (result, flag) = self.overflowing_pow(&other.to_u32().unwrap());
                (console::Integer::new(result), flag)
            })
        } else {
            let mut result = Self::one();
            let mut flag = Boolean::constant(false);

            // TODO (@pranav) In each step, we check that we have not overflowed,
            //  yet we know that in the first step, we do not need to check and
            //  in general we do not need to check for overflow until we have found
            //  the second bit that has been set. Optimize.
            for bit in other.bits_le.iter().rev() {
                let (squared_result, squared_flag) = result.mul_flagged(&result);
                result = squared_result;
                flag = flag.bitor(&squared_flag);

                let result_times_self = if I::is_signed() {
                    // Multiply the absolute value of `self` and `other` in the base field.
                    // Note that it is safe to use abs_wrapped since we want Integer::MIN to be interpreted as an unsigned number.
                    let (product, carry) = Self::mul_with_carry(&(&result).abs_wrapped(), &self.abs_wrapped());

                    // We need to check that the abs(a) * abs(b) did not exceed the unsigned maximum.
                    let carry_bits_nonzero = carry.iter().fold(Boolean::constant(false), |a, b| a | b);

                    // If the product should be positive, then it cannot exceed the signed maximum.
                    let operands_same_sign = &result.msb().is_equal(self.msb());
                    let positive_product_overflows = operands_same_sign & product.msb();

                    // If the product should be negative, then it cannot exceed the absolute value of the signed minimum.
                    let negative_product_underflows = {
                        let lower_product_bits_nonzero = product.bits_le[..(I::BITS as usize - 1)]
                            .iter()
                            .fold(Boolean::constant(false), |a, b| a | b);
                        let negative_product_lt_or_eq_signed_min =
                            !product.msb() | (product.msb() & !lower_product_bits_nonzero);
                        !operands_same_sign & !negative_product_lt_or_eq_signed_min
                    };

                    let overflow = carry_bits_nonzero | positive_product_overflows | negative_product_underflows;
                    flag = flag.bitor(&(overflow & bit));

                    // Return the product of `self` and `other` with the appropriate sign.
                    Self::ternary(operands_same_sign, &product, &(!&product).add_wrapped(&Self::one()))
                } else {
                    let (product, carry) = Self::mul_with_carry(&result, self);

                    // For unsigned multiplication, check that the none of the carry bits are set.
                    let overflow = carry.iter().fold(Boolean::constant(false), |a, b| a | b);
                    flag = flag.bitor(&(overflow & bit));

                    // Return the product of `self` and `other`.
                    product
                };

                result = Self::ternary(bit, &result_times_self, &result);
            }
            (result, flag)
        }
    }
}

impl<E: Environment, I: IntegerType, M: Magnitude> Metrics<dyn PowFlagged<Integer<E, M>, Output = Integer<E, I>>>
    for Integer<E, I>
{
    type Case = (Mode, Mode);

    fn count(case: &Self::Case) -> Count {
        match (case.0, case.1) {
            (Mode::Constant, Mode::Constant) => Count::is(I::BITS, 0, 0, 0),
            (Mode::Constant, _) | (_, Mode::Constant) => {
                let mul_count = count!(Integer<E, I>, MulWrapped<Integer<E, I>, Output=Integer<E, I>>, case);
                (2 * M::BITS * mul_count) + Count::is(2 * I::BITS, 0, I::BITS, I::BITS)
            }
            (_, _) => {
                let mul_count = count!(Integer<E, I>, MulWrapped<Integer<E, I>, Output=Integer<E, I>>, case);
                (2 * M::BITS * mul_count) + Count::is(2 * I::BITS, 0, I::BITS, I::BITS)
            }
        }
    }
}

impl<E: Environment, I: IntegerType, M: Magnitude> OutputMode<dyn PowFlagged<Integer<E, M>, Output = Integer<E, I>>>
    for Integer<E, I>
{
    type Case = (Mode, CircuitType<Integer<E, M>>);

    fn output_mode(case: &Self::Case) -> Mode {
        match (case.0, (case.1.mode(), &case.1)) {
            (Mode::Constant, (Mode::Constant, _)) => Mode::Constant,
            (Mode::Constant, (mode, _)) => match mode {
                Mode::Constant => Mode::Constant,
                _ => Mode::Private,
            },
            (_, (Mode::Constant, case)) => match case {
                // Determine if the constant is all zeros.
                CircuitType::Constant(constant) => match constant.eject_value().is_zero() {
                    true => Mode::Constant,
                    false => Mode::Private,
                },
                _ => E::halt("The constant is required for the output mode of `pow_wrapped` with a constant."),
            },
            (_, _) => Mode::Private,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utilities::*;
    use snarkvm_circuit_environment::Circuit;

    use std::{ops::RangeInclusive, panic::RefUnwindSafe};

    // Lowered to 4; we run (~5 * ITERATIONS) cases for most tests.
    const ITERATIONS: u64 = 4;

    fn check_pow<I: IntegerType + RefUnwindSafe, M: Magnitude + RefUnwindSafe>(
        name: &str,
        first: console::Integer<<Circuit as Environment>::Network, I>,
        second: console::Integer<<Circuit as Environment>::Network, M>,
        mode_a: Mode,
        mode_b: Mode,
    ) {
        let a = Integer::<Circuit, I>::new(mode_a, first);
        let b = Integer::<Circuit, M>::new(mode_b, second);
        let (expected_result, expected_flag) = first.overflowing_pow(&second.to_u32().unwrap());
        Circuit::scope(name, || {
            let (candidate_result, candidate_flag) = a.pow_flagged(&b);
            assert_eq!(expected_result, *candidate_result.eject_value());
            assert_eq!(console::Integer::new(expected_result), candidate_result.eject_value());
            // assert_count!(PowFlagged(Integer<I>, Integer<M>) => Integer<I>, &(mode_a, mode_b));
            // assert_output_mode!(PowFlagged(Integer<I>, Integer<M>) => Integer<I>, &(mode_a, CircuitType::from(&b)), candidate);
            assert!(Circuit::is_satisfied_in_scope(), "(is_satisfied_in_scope)");
        });
        Circuit::reset();
    }

    fn run_test<I: IntegerType + RefUnwindSafe, M: Magnitude + RefUnwindSafe>(mode_a: Mode, mode_b: Mode) {
        let mut rng = TestRng::default();

        for i in 0..ITERATIONS {
            let first = Uniform::rand(&mut rng);
            let second = Uniform::rand(&mut rng);

            let name = format!("Pow: {} ** {} {}", mode_a, mode_b, i);
            check_pow::<I, M>(&name, first, second, mode_a, mode_b);

            let name = format!("Pow Zero: {} ** {} {}", mode_a, mode_b, i);
            check_pow::<I, M>(&name, first, console::Integer::zero(), mode_a, mode_b);

            let name = format!("Pow One: {} ** {} {}", mode_a, mode_b, i);
            check_pow::<I, M>(&name, first, console::Integer::one(), mode_a, mode_b);

            // Check that the square is computed correctly.
            let name = format!("Square: {} ** {} {}", mode_a, mode_b, i);
            check_pow::<I, M>(&name, first, console::Integer::one() + console::Integer::one(), mode_a, mode_b);

            // Check that the cube is computed correctly.
            let name = format!("Cube: {} ** {} {}", mode_a, mode_b, i);
            check_pow::<I, M>(
                &name,
                first,
                console::Integer::one() + console::Integer::one() + console::Integer::one(),
                mode_a,
                mode_b,
            );
        }

        // Test corner cases for exponentiation.
        check_pow::<I, M>("MAX ** MAX", console::Integer::MAX, console::Integer::MAX, mode_a, mode_b);
        check_pow::<I, M>("MIN ** 0", console::Integer::MIN, console::Integer::zero(), mode_a, mode_b);
        check_pow::<I, M>("MAX ** 0", console::Integer::MAX, console::Integer::zero(), mode_a, mode_b);
        check_pow::<I, M>("MIN ** 1", console::Integer::MIN, console::Integer::one(), mode_a, mode_b);
        check_pow::<I, M>("MAX ** 1", console::Integer::MAX, console::Integer::one(), mode_a, mode_b);
    }

    fn run_exhaustive_test<I: IntegerType + RefUnwindSafe, M: Magnitude + RefUnwindSafe>(mode_a: Mode, mode_b: Mode)
    where
        RangeInclusive<I>: Iterator<Item = I>,
        RangeInclusive<M>: Iterator<Item = M>,
    {
        for first in I::MIN..=I::MAX {
            for second in M::MIN..=M::MAX {
                let first = console::Integer::<_, I>::new(first);
                let second = console::Integer::<_, M>::new(second);

                let name = format!("Pow: ({} ** {})", first, second);
                check_pow::<I, M>(&name, first, second, mode_a, mode_b);
            }
        }
    }

    test_integer_binary!(run_test, i8, u8, pow);
    test_integer_binary!(run_test, i8, u16, pow);
    test_integer_binary!(run_test, i8, u32, pow);

    test_integer_binary!(run_test, i16, u8, pow);
    test_integer_binary!(run_test, i16, u16, pow);
    test_integer_binary!(run_test, i16, u32, pow);

    test_integer_binary!(run_test, i32, u8, pow);
    test_integer_binary!(run_test, i32, u16, pow);
    test_integer_binary!(run_test, i32, u32, pow);

    test_integer_binary!(run_test, i64, u8, pow);
    test_integer_binary!(run_test, i64, u16, pow);
    test_integer_binary!(run_test, i64, u32, pow);

    test_integer_binary!(run_test, i128, u8, pow);
    test_integer_binary!(run_test, i128, u16, pow);
    test_integer_binary!(run_test, i128, u32, pow);

    test_integer_binary!(run_test, u8, u8, pow);
    test_integer_binary!(run_test, u8, u16, pow);
    test_integer_binary!(run_test, u8, u32, pow);

    test_integer_binary!(run_test, u16, u8, pow);
    test_integer_binary!(run_test, u16, u16, pow);
    test_integer_binary!(run_test, u16, u32, pow);

    test_integer_binary!(run_test, u32, u8, pow);
    test_integer_binary!(run_test, u32, u16, pow);
    test_integer_binary!(run_test, u32, u32, pow);

    test_integer_binary!(run_test, u64, u8, pow);
    test_integer_binary!(run_test, u64, u16, pow);
    test_integer_binary!(run_test, u64, u32, pow);

    test_integer_binary!(run_test, u128, u8, pow);
    test_integer_binary!(run_test, u128, u16, pow);
    test_integer_binary!(run_test, u128, u32, pow);

    test_integer_binary!(#[ignore], run_exhaustive_test, u8, u8, pow, exhaustive);
    test_integer_binary!(#[ignore], run_exhaustive_test, i8, u8, pow, exhaustive);
}

//! This is a library for basic mathematical utilities.
/// It includes functions for arithmetic operations, number checks.
/// 
/// 
/// Additional functions can be added as needed.
/// # Functions Included
/// 1. Arithmetic operations: add, subtract, multiply, divide, power, abs, max, min
/// 2. Number checks: is_even, is_odd, is_prime, gcd, lcm, factorial    
/// # Examples
/// ```
/// use math_utils::add;
/// let sum = add(2, 3);
/// assert_eq!(sum, 5);
/// ```
///
/// # Additional Functions that can be implemented
/// 3. Rounding & Measurement
/// 4. Conversions
/// 5. Geometry
/// 6. Vector & List Utilities
/// 7. Algebra / Utility

// 1.Arithmetic operations
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

pub fn divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 { None } else { Some(a / b) }
}

pub fn power(base: i32, exponent: u32) -> i32 {
    base.pow(exponent)
}

pub fn abs(value: i32) -> i32 {
    if value < 0 { -value } else { value }
}

pub fn max(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}

pub fn min(a: i32, b: i32) -> i32 {
    if a < b { a } else { b }
}

// 2.Number checks
pub fn is_even(n: i32) -> bool {
    n % 2 == 0
}

pub fn is_odd(n: i32) -> bool {
    n % 2 != 0
}

pub fn is_prime(n: i32) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..=(n as f64).sqrt() as i32 {
        if n % i == 0 {
            return false;
        }
    }
    true
}

pub fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 { a.abs() } else { gcd(b, a % b) }
}

pub fn lcm(a: i32, b: i32) -> i32 {
    (a * b).abs() / gcd(a, b)
}

pub fn factorial(n: u32) -> u32 {
    (1..=n).product()
}

//Additional functions that can be implemented:

// 3. Rounding & Measurement

// round(x)

// floor(x)

// ceil(x)

// clamp(value, min, max)

// percentage(part, whole)

// average(numbers)

// median(numbers)

// mode(numbers)

// 4. Conversions

// deg_to_rad(deg)

// rad_to_deg(rad)

// km_to_miles(km)

// miles_to_km(miles)

// 5. Geometry

// Circle

// circle_area(radius)

// circle_circumference(radius)

// Rectangle & Square

// rectangle_area(width, height)

// square_area(side)

// Triangle

// triangle_area(base, height)

// triangle_area_heron(a, b, c) (Heron’s formula)

// 6. Vector & List Utilities

// sum(vec)

// product(vec)

// min_in_vec(vec)

// max_in_vec(vec)

// range(start, end)

// normalize(vec) (vector normalization)

// 7. Algebra / Utility

// solve_quadratic(a, b, c) → returns roots

// distance_2d(x1, y1, x2, y2)

// distance_3d(x1, y1, z1, x2, y2, z2)

// dot_product(v1, v2)

// cross_product(v1, v2)
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_subtract() {
        assert_eq!(subtract(5, 3), 2);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(4, 3), 12);
    }

    #[test]
    fn test_divide() {
        assert_eq!(divide(10, 2), Some(5));
        assert_eq!(divide(10, 0), None);
    }

    #[test]
    fn test_power() {
        assert_eq!(power(2, 3), 8);
    }

    #[test]
    fn test_abs() {
        assert_eq!(abs(-5), 5);
        assert_eq!(abs(5), 5);
    }

    #[test]
    fn test_max() {
        assert_eq!(max(5, 10), 10)
    }

    #[test]
    fn test_min() {
        assert_eq!(min(5, 10), 5)
    }
}

use bevy::log::warn;

/// Generic Newton-Raphson solver
pub fn solve_newton_raphson<F, Fd>(
    f: F,                // The function f(x)
    f_prime: Fd,         // The derivative of the function f'(x)
    initial_guess: f64,  // Initial guess for the root
    tolerance: f64,      // Tolerance for convergence
    max_iterations: u32, // Maximum number of iterations
) -> f64
where
    F: Fn(f64) -> f64,
    Fd: Fn(f64) -> f64,
{
    let mut x = initial_guess;

    for _ in 0..max_iterations {
        // Newton-Raphson update step
        let next_x = x - f(x) / f_prime(x);

        if (next_x - x).abs() < tolerance {
            return next_x;
        }

        x = next_x;
    }

    // Return the final approximation after max_iterations
    warn!("Returning from solver after max iterations");
    x
}

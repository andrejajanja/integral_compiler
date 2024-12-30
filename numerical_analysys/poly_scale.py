import numpy as np
import sympy as sp

def scale_polynomial(coeffs, old_range, new_range):
    """
    Scale a polynomial from old_range to new_range.
    
    Args:
        coeffs (list): Coefficients of the polynomial [a0, a1, a2, ...] (highest degree last).
        old_range (tuple): (a, b) range of the original polynomial.
        new_range (tuple): (c, d) target range for scaling.

    Returns:
        list: Coefficients of the scaled polynomial.
    """
    a, b = old_range
    c, d = new_range

    # Symbolic variable
    x = sp.Symbol('x')
    x_scaled = (x - a) * (d - c) / (b - a) + c
    
    # Create the original polynomial
    poly = sum(c_i * x**i for i, c_i in enumerate(coeffs))
    # Substitute scaled variable
    scaled_poly = sp.expand(poly.subs(x, x_scaled))
    
    # Extract new coefficients
    scaled_coeffs = [scaled_poly.coeff(x, i) for i in range(sp.degree(scaled_poly, x) + 1)]
    return scaled_coeffs

def rescale_value(x_scaled, old_range, new_range):
    """
    Rescale a value from new_range back to old_range.
    
    Args:
        x_scaled (float): Scaled value.
        old_range (tuple): (a, b) original range.
        new_range (tuple): (c, d) target range.

    Returns:
        float: Rescaled value.
    """
    a, b = old_range
    c, d = new_range
    return (x_scaled - c) * (b - a) / (d - c) + a

def evaluate_polynomial(coeffs, x):
    """
    Evaluate a polynomial at a given point using Horner's method.
    
    Args:
        coeffs (list): Coefficients of the polynomial [a0, a1, ...].
        x (float): Point to evaluate the polynomial at.
    
    Returns:
        float: Polynomial value at x.
    """
    result = 0
    for coeff in reversed(coeffs):
        result = result * x + coeff
    return result

def scale_back_polynomial(coeffs, old_range, new_range):
    """
    Scale a polynomial back from new_range to old_range.
    
    Args:
        coeffs (list): Coefficients of the scaled polynomial [a0, a1, a2, ...] (highest degree last).
        old_range (tuple): (a, b) range of the original polynomial.
        new_range (tuple): (c, d) target range for scaling.

    Returns:
        list: Coefficients of the polynomial scaled back to the original range.
    """
    a, b = old_range
    c, d = new_range

    # Symbolic variable
    x = sp.Symbol('x')
    x_rescaled = (x - c) * (b - a) / (d - c) + a
    
    # Create the scaled polynomial
    poly = sum(c_i * x**i for i, c_i in enumerate(coeffs))
    
    # Substitute the rescaled variable
    original_poly = sp.expand(poly.subs(x, x_rescaled))
    
    # Extract new coefficients for the original range
    original_coeffs = [original_poly.coeff(x, i) for i in range(sp.degree(original_poly, x) + 1)]
    return original_coeffs


# Example Usage
if __name__ == "__main__":
    cos_coeffs = [0.9999978498487038, 1.9164197396536896e-5, -0.5000757040113136, 
              0.0001737237775432626, 0.04141206540354817, 0.0002459702429402897, 
              -0.001543919635752731, 5.9755690265824737e-5, 1.3400354808237594e-5]
    
    ln_coeffs = [-2.7178571428571425, 8.0, -14.0, 18.666666666666664, -17.5, 11.2, -4.666666666666667, 1.1428571428571428, -0.125]

    # Define correct ranges
    old_range = (1-3, 1+3)  # Original domain of the Taylor polynomial
    new_range = (-1, 1)     # Scaled range for numerical stability

    # Scale the polynomial
    scaled_cos = scale_polynomial(cos_coeffs, old_range, new_range)
    scaled_ln = scale_polynomial(ln_coeffs, old_range, new_range)



    back_multi = scale_back_polynomial(scaled_cos, old_range, new_range)

    # printable = ""
    # for i, c in enumerate(scaled_cos):
    #     printable += "{}{:.10f}*d^{}".format('+' if c >= 0 else '',c,i)
    # print("Scaled Coefficients cos:\n", printable)

    # Evaluate at a point in the scaled range
    x_scaled = 0.02  # Example point in [-1,1]
    scaled_value = evaluate_polynomial(scaled_cos, x_scaled)
    print(f"Value at scaled x={x_scaled}:", scaled_value)

    # Rescale the result back to the original range
    x_original = rescale_value(x_scaled, old_range, new_range)
    print("Rescaled x back to original range:", x_original)

    # Verify against the original polynomial evaluated at x_original
    original_value = evaluate_polynomial(cos_coeffs, x_original)
    print(f"Value at original x={x_original}:", original_value)

    original_value_back = evaluate_polynomial(back_multi, x_original)
    print(f"Value at original x={x_original}:", original_value_back)

# scaled_ln = scale_polynomial(ln_coeffs, old_range, new_range)
    # print("Scaled Coefficients ln:", scaled_ln)
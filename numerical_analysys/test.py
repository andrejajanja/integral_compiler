import numpy as np

def f_of_x(x: np.float64) -> np.float64:
    n = 8
    coefs = np.array([1.7056303009832585e-6, 0.999992959504906, 0.9999878744993761, 0.33347694187223775, -0.00042950356808058244, -0.03263952873623189, -0.011781862869650836, -0.0012174869673450101, -7.518200587348887e-5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], dtype=np.float64)
    px = coefs[0] + coefs[1]*x
    current_pow = x

    for i in range(2,n+1):
        current_pow *= x
        px += coefs[i]*current_pow

    return px

x = 1.1

temp = np.sin(x)*np.exp(x)

print(f"P(x) = {f_of_x(x)} - f(x) = {temp}")
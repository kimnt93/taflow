# Momentum benchmark (`MOM` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 186.57M | 0.004 | 230.44M | 0.032 | 5.94× | 7.33× |
| 10,000 | 0.023 | 440.09M | 0.020 | 493.95M | 0.037 | 1.62× | 1.81× |
| 100,000 | 0.202 | 494.11M | 0.180 | 556.60M | 0.075 | 0.37× | 0.42× |
| 1,000,000 | 2.735 | 365.58M | 1.975 | 506.38M | 1.078 | 0.39× | 0.55× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.087 | 0.152 | 1.75× |
| 1 | 5 | 0.275 | 0.489 | 1.78× |
| 1 | 10 | 0.470 | 1.123 | 2.39× |
| 10 | 1 | 0.057 | 0.097 | 1.72× |
| 10 | 5 | 0.236 | 0.479 | 2.03× |
| 10 | 10 | 0.457 | 0.951 | 2.08× |
| 100 | 1 | 0.048 | 0.095 | 1.98× |
| 100 | 5 | 0.227 | 0.455 | 2.00× |
| 100 | 10 | 0.468 | 0.933 | 2.00× |
| 1,000 | 1 | 0.053 | 0.090 | 1.70× |
| 1,000 | 5 | 0.243 | 0.437 | 1.80× |
| 1,000 | 10 | 0.473 | 1.058 | 2.24× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

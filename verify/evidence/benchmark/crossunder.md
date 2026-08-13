# Crossunder benchmark (`causal crossunder` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.038 | 26.33M | 0.031 | 31.95M | 0.017 | 0.44× | 0.53× |
| 10,000 | 0.242 | 41.37M | 0.219 | 45.73M | 0.029 | 0.12× | 0.13× |
| 100,000 | 2.116 | 47.26M | 2.078 | 48.12M | 0.142 | 0.07× | 0.07× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.157 | 0.111 | 0.70× |
| 1 | 5 | 0.403 | 0.367 | 0.91× |
| 1 | 10 | 0.591 | 0.661 | 1.12× |
| 10 | 1 | 0.062 | 0.067 | 1.08× |
| 10 | 5 | 0.276 | 0.310 | 1.12× |
| 10 | 10 | 0.601 | 0.659 | 1.10× |
| 100 | 1 | 0.065 | 0.066 | 1.02× |
| 100 | 5 | 0.282 | 0.309 | 1.09× |
| 100 | 10 | 0.554 | 0.676 | 1.22× |
| 1,000 | 1 | 0.090 | 0.067 | 0.74× |
| 1,000 | 5 | 0.300 | 0.409 | 1.36× |
| 1,000 | 10 | 0.638 | 0.869 | 1.36× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

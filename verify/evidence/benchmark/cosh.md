# MathCosh benchmark (`COSH` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 120.45M | 0.008 | 128.25M | 0.031 | 3.76× | 4.00× |
| 10,000 | 0.056 | 178.42M | 0.055 | 181.24M | 0.079 | 1.41× | 1.43× |
| 100,000 | 0.562 | 177.78M | 0.532 | 188.08M | 0.577 | 1.03× | 1.09× |
| 1,000,000 | 5.952 | 168.00M | 5.471 | 182.79M | 5.371 | 0.90× | 0.98× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.117 | 0.124 | 1.06× |
| 1 | 5 | 0.279 | 0.429 | 1.54× |
| 1 | 10 | 0.467 | 0.893 | 1.91× |
| 10 | 1 | 0.050 | 0.086 | 1.71× |
| 10 | 5 | 0.214 | 0.417 | 1.95× |
| 10 | 10 | 0.464 | 0.885 | 1.91× |
| 100 | 1 | 0.058 | 0.089 | 1.53× |
| 100 | 5 | 0.230 | 0.408 | 1.78× |
| 100 | 10 | 0.478 | 0.893 | 1.87× |
| 1,000 | 1 | 0.056 | 0.095 | 1.68× |
| 1,000 | 5 | 0.229 | 0.444 | 1.94× |
| 1,000 | 10 | 0.471 | 0.938 | 1.99× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

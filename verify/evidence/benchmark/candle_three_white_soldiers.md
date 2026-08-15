# CandleThreeWhiteSoldiers benchmark (`CDL3WHITESOLDIERS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 136.60M | 0.004 | 223.58M | 0.043 | 5.84× | 9.56× |
| 10,000 | 0.067 | 149.32M | 0.069 | 145.45M | 0.181 | 2.70× | 2.63× |
| 100,000 | 0.827 | 120.91M | 0.801 | 124.78M | 1.466 | 1.77× | 1.83× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.142 | 0.108 | 0.76× |
| 1 | 5 | 0.229 | 0.479 | 2.09× |
| 1 | 10 | 0.379 | 0.873 | 2.30× |
| 10 | 1 | 0.042 | 0.087 | 2.08× |
| 10 | 5 | 0.189 | 0.402 | 2.13× |
| 10 | 10 | 0.371 | 0.899 | 2.42× |
| 100 | 1 | 0.041 | 0.085 | 2.07× |
| 100 | 5 | 0.192 | 0.409 | 2.13× |
| 100 | 10 | 0.400 | 0.913 | 2.28× |
| 1,000 | 1 | 0.056 | 0.104 | 1.85× |
| 1,000 | 5 | 0.221 | 0.494 | 2.23× |
| 1,000 | 10 | 0.392 | 1.023 | 2.61× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

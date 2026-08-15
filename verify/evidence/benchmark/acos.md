# MathAcos benchmark (`ACOS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 123.01M | 0.007 | 145.80M | 0.035 | 4.29× | 5.09× |
| 10,000 | 0.067 | 148.95M | 0.067 | 150.12M | 0.131 | 1.96× | 1.97× |
| 100,000 | 0.711 | 140.60M | 0.650 | 153.82M | 0.691 | 0.97× | 1.06× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.053 | 0.106 | 1.99× |
| 1 | 5 | 0.226 | 0.439 | 1.94× |
| 1 | 10 | 0.436 | 0.946 | 2.17× |
| 10 | 1 | 0.045 | 0.085 | 1.90× |
| 10 | 5 | 0.195 | 0.443 | 2.28× |
| 10 | 10 | 0.387 | 0.924 | 2.39× |
| 100 | 1 | 0.043 | 0.085 | 1.96× |
| 100 | 5 | 0.194 | 0.428 | 2.21× |
| 100 | 10 | 0.410 | 0.884 | 2.15× |
| 1,000 | 1 | 0.056 | 0.096 | 1.71× |
| 1,000 | 5 | 0.211 | 0.493 | 2.34× |
| 1,000 | 10 | 0.415 | 0.958 | 2.31× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

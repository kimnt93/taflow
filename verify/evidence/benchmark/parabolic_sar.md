# ParabolicSar benchmark (`SAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 112.09M | 0.008 | 126.18M | 0.039 | 4.38× | 4.93× |
| 10,000 | 0.085 | 117.80M | 0.083 | 120.40M | 0.094 | 1.11× | 1.13× |
| 100,000 | 0.921 | 108.62M | 0.885 | 113.00M | 0.681 | 0.74× | 0.77× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.077 | 0.128 | 1.67× |
| 1 | 5 | 0.219 | 0.474 | 2.16× |
| 1 | 10 | 0.390 | 1.053 | 2.70× |
| 10 | 1 | 0.044 | 0.090 | 2.03× |
| 10 | 5 | 0.191 | 0.457 | 2.39× |
| 10 | 10 | 0.383 | 0.988 | 2.58× |
| 100 | 1 | 0.047 | 0.103 | 2.21× |
| 100 | 5 | 0.218 | 0.496 | 2.28× |
| 100 | 10 | 0.412 | 0.993 | 2.41× |
| 1,000 | 1 | 0.052 | 0.095 | 1.84× |
| 1,000 | 5 | 0.193 | 0.504 | 2.61× |
| 1,000 | 10 | 0.425 | 1.053 | 2.48× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

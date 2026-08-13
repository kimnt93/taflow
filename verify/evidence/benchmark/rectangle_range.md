# RectangleRange benchmark (`RectangleRange` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.057 | 17.49M | 0.049 | 20.48M | 0.212 | 3.71× | 4.35× |
| 10,000 | 0.385 | 25.98M | 0.393 | 25.43M | 1.333 | 3.46× | 3.39× |
| 100,000 | 3.882 | 25.76M | 3.749 | 26.67M | 12.678 | 3.27× | 3.38× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.105 | 0.211 | 2.01× |
| 1 | 5 | 0.390 | 0.868 | 2.23× |
| 1 | 10 | 0.678 | 1.639 | 2.42× |
| 10 | 1 | 0.069 | 0.170 | 2.45× |
| 10 | 5 | 0.312 | 1.090 | 3.50× |
| 10 | 10 | 0.694 | 1.843 | 2.66× |
| 100 | 1 | 0.092 | 0.182 | 1.98× |
| 100 | 5 | 0.321 | 1.157 | 3.60× |
| 100 | 10 | 0.682 | 1.792 | 2.63× |
| 1,000 | 1 | 0.116 | 0.296 | 2.55× |
| 1,000 | 5 | 0.331 | 1.731 | 5.23× |
| 1,000 | 10 | 0.650 | 2.981 | 4.59× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

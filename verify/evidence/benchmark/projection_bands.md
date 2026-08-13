# ProjectionBands benchmark (`rolling projection mean` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.195 | 5.12M | 0.196 | 5.11M | 0.090 | 0.46× | 0.46× |
| 10,000 | 1.924 | 5.20M | 1.911 | 5.23M | 0.274 | 0.14× | 0.14× |
| 100,000 | 19.666 | 5.08M | 19.349 | 5.17M | 2.130 | 0.11× | 0.11× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.081 | 0.126 | 1.56× |
| 1 | 5 | 0.400 | 0.511 | 1.28× |
| 1 | 10 | 0.600 | 1.008 | 1.68× |
| 10 | 1 | 0.069 | 0.098 | 1.42× |
| 10 | 5 | 0.283 | 0.496 | 1.76× |
| 10 | 10 | 0.607 | 1.038 | 1.71× |
| 100 | 1 | 0.082 | 0.140 | 1.71× |
| 100 | 5 | 0.295 | 0.692 | 2.34× |
| 100 | 10 | 0.628 | 1.411 | 2.25× |
| 1,000 | 1 | 0.270 | 0.159 | 0.59× |
| 1,000 | 5 | 0.423 | 0.740 | 1.75× |
| 1,000 | 10 | 0.792 | 1.588 | 2.00× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

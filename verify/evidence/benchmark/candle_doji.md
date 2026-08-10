# CandleDoji benchmark (`CDLDOJI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 84.49M | 0.009 | 114.59M | 0.029 | 2.46× | 3.34× |
| 10,000 | 0.041 | 242.80M | 0.037 | 269.50M | 0.054 | 1.32× | 1.46× |
| 100,000 | 0.327 | 306.16M | 0.319 | 313.79M | 0.238 | 0.73× | 0.75× |
| 1,000,000 | 3.671 | 272.43M | 3.820 | 261.79M | 3.322 | 0.91× | 0.87× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.101 | 0.157 | 1.55× |
| 1 | 5 | 0.298 | 0.457 | 1.54× |
| 1 | 10 | 0.526 | 0.928 | 1.76× |
| 10 | 1 | 0.057 | 0.085 | 1.50× |
| 10 | 5 | 0.245 | 0.429 | 1.75× |
| 10 | 10 | 0.537 | 0.915 | 1.70× |
| 100 | 1 | 0.055 | 0.087 | 1.60× |
| 100 | 5 | 0.236 | 0.426 | 1.81× |
| 100 | 10 | 0.541 | 0.892 | 1.65× |
| 1,000 | 1 | 0.061 | 0.091 | 1.48× |
| 1,000 | 5 | 0.255 | 0.422 | 1.65× |
| 1,000 | 10 | 0.568 | 0.905 | 1.59× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

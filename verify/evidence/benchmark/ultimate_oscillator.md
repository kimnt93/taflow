# UltimateOscillator benchmark (`ULTOSC` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 71.62M | 0.013 | 79.06M | 0.055 | 3.91× | 4.32× |
| 10,000 | 0.117 | 85.49M | 0.112 | 89.15M | 0.198 | 1.69× | 1.77× |
| 100,000 | 1.254 | 79.74M | 1.139 | 87.80M | 1.669 | 1.33× | 1.47× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.129 | 0.158 | 1.22× |
| 1 | 5 | 0.250 | 0.530 | 2.12× |
| 1 | 10 | 0.459 | 1.041 | 2.27× |
| 10 | 1 | 0.046 | 0.098 | 2.11× |
| 10 | 5 | 0.212 | 0.564 | 2.66× |
| 10 | 10 | 0.438 | 0.985 | 2.25× |
| 100 | 1 | 0.047 | 0.098 | 2.08× |
| 100 | 5 | 0.197 | 0.484 | 2.45× |
| 100 | 10 | 0.438 | 1.110 | 2.53× |
| 1,000 | 1 | 0.056 | 0.124 | 2.22× |
| 1,000 | 5 | 0.208 | 0.585 | 2.81× |
| 1,000 | 10 | 0.446 | 1.268 | 2.85× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

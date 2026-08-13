# AroonOscillator benchmark (`AROONOSC` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.188 | 5.33M | 0.180 | 5.57M | 0.035 | 0.19× | 0.20× |
| 10,000 | 1.689 | 5.92M | 1.611 | 6.21M | 0.128 | 0.08× | 0.08× |
| 100,000 | 16.473 | 6.07M | 16.190 | 6.18M | 1.002 | 0.06× | 0.06× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.110 | 0.127 | 1.16× |
| 1 | 5 | 0.408 | 0.468 | 1.15× |
| 1 | 10 | 0.638 | 0.932 | 1.46× |
| 10 | 1 | 0.069 | 0.090 | 1.30× |
| 10 | 5 | 0.327 | 0.440 | 1.34× |
| 10 | 10 | 0.739 | 0.905 | 1.22× |
| 100 | 1 | 0.087 | 0.096 | 1.10× |
| 100 | 5 | 0.311 | 0.422 | 1.36× |
| 100 | 10 | 0.642 | 0.891 | 1.39× |
| 1,000 | 1 | 0.242 | 0.102 | 0.42× |
| 1,000 | 5 | 0.439 | 0.479 | 1.09× |
| 1,000 | 10 | 0.814 | 1.015 | 1.25× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

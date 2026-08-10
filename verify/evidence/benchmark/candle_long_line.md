# CandleLongLine benchmark (`CDLLONGLINE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.022 | 46.45M | 0.018 | 56.22M | 0.041 | 1.93× | 2.33× |
| 10,000 | 0.173 | 57.94M | 0.163 | 61.50M | 0.191 | 1.11× | 1.18× |
| 100,000 | 1.739 | 57.51M | 2.052 | 48.74M | 1.755 | 1.01× | 0.86× |
| 1,000,000 | 17.307 | 57.78M | 19.235 | 51.99M | 17.342 | 1.00× | 0.90× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.077 | 0.113 | 1.46× |
| 1 | 5 | 0.285 | 0.509 | 1.78× |
| 1 | 10 | 0.607 | 1.164 | 1.92× |
| 10 | 1 | 0.078 | 0.120 | 1.53× |
| 10 | 5 | 0.310 | 0.511 | 1.65× |
| 10 | 10 | 0.650 | 1.181 | 1.82× |
| 100 | 1 | 0.087 | 0.114 | 1.31× |
| 100 | 5 | 0.327 | 0.514 | 1.57× |
| 100 | 10 | 0.628 | 1.233 | 1.97× |
| 1,000 | 1 | 0.086 | 0.133 | 1.55× |
| 1,000 | 5 | 0.340 | 0.596 | 1.75× |
| 1,000 | 10 | 0.653 | 1.372 | 2.10× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

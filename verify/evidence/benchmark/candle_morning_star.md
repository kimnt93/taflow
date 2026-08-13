# CandleMorningStar benchmark (`CDLMORNINGSTAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.117 | 8.57M | 0.107 | 9.37M | 0.038 | 0.32× | 0.35× |
| 10,000 | 1.006 | 9.94M | 1.002 | 9.98M | 0.103 | 0.10× | 0.10× |
| 100,000 | 9.719 | 10.29M | 10.170 | 9.83M | 0.833 | 0.09× | 0.08× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.112 | 0.150 | 1.34× |
| 1 | 5 | 0.407 | 0.514 | 1.26× |
| 1 | 10 | 0.651 | 1.006 | 1.55× |
| 10 | 1 | 0.080 | 0.106 | 1.33× |
| 10 | 5 | 0.304 | 0.480 | 1.58× |
| 10 | 10 | 0.653 | 0.974 | 1.49× |
| 100 | 1 | 0.082 | 0.098 | 1.20× |
| 100 | 5 | 0.315 | 0.455 | 1.45× |
| 100 | 10 | 0.647 | 0.992 | 1.53× |
| 1,000 | 1 | 0.181 | 0.109 | 0.60× |
| 1,000 | 5 | 0.443 | 0.500 | 1.13× |
| 1,000 | 10 | 0.721 | 1.055 | 1.46× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

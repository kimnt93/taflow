# CandleThrusting benchmark (`CDLTHRUSTING` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 76.28M | 0.010 | 98.39M | 0.034 | 2.60× | 3.36× |
| 10,000 | 0.143 | 69.75M | 0.135 | 74.20M | 0.123 | 0.86× | 0.91× |
| 100,000 | 1.423 | 70.26M | 1.392 | 71.84M | 0.948 | 0.67× | 0.68× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.062 | 0.161 | 2.60× |
| 1 | 5 | 0.276 | 0.466 | 1.69× |
| 1 | 10 | 0.373 | 0.911 | 2.44× |
| 10 | 1 | 0.040 | 0.106 | 2.64× |
| 10 | 5 | 0.175 | 0.506 | 2.88× |
| 10 | 10 | 0.392 | 0.916 | 2.34× |
| 100 | 1 | 0.049 | 0.100 | 2.04× |
| 100 | 5 | 0.178 | 0.437 | 2.46× |
| 100 | 10 | 0.416 | 1.011 | 2.43× |
| 1,000 | 1 | 0.061 | 0.100 | 1.66× |
| 1,000 | 5 | 0.193 | 0.512 | 2.65× |
| 1,000 | 10 | 0.458 | 1.063 | 2.32× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

# CandleSeparatingLines benchmark (`CDLSEPARATINGLINES` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.125 | 8.02M | 0.112 | 8.95M | 0.035 | 0.28× | 0.31× |
| 10,000 | 1.014 | 9.86M | 1.002 | 9.98M | 0.121 | 0.12× | 0.12× |
| 100,000 | 9.888 | 10.11M | 9.940 | 10.06M | 0.950 | 0.10× | 0.10× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.128 | 0.127 | 0.99× |
| 1 | 5 | 0.381 | 0.454 | 1.19× |
| 1 | 10 | 0.643 | 0.941 | 1.46× |
| 10 | 1 | 0.074 | 0.096 | 1.29× |
| 10 | 5 | 0.321 | 0.438 | 1.36× |
| 10 | 10 | 0.635 | 0.909 | 1.43× |
| 100 | 1 | 0.082 | 0.093 | 1.14× |
| 100 | 5 | 0.325 | 0.448 | 1.38× |
| 100 | 10 | 0.702 | 0.930 | 1.33× |
| 1,000 | 1 | 0.177 | 0.105 | 0.60× |
| 1,000 | 5 | 0.334 | 0.485 | 1.45× |
| 1,000 | 10 | 0.702 | 1.025 | 1.46× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

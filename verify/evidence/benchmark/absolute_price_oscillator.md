# AbsolutePriceOscillator benchmark (`APO` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.033 | 30.15M | 0.027 | 36.47M | 0.039 | 1.17× | 1.41× |
| 10,000 | 0.182 | 54.96M | 0.173 | 57.84M | 0.074 | 0.41× | 0.43× |
| 100,000 | 1.616 | 61.87M | 1.722 | 58.08M | 0.468 | 0.29× | 0.27× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.116 | 0.110 | 0.95× |
| 1 | 5 | 0.451 | 0.522 | 1.16× |
| 1 | 10 | 0.730 | 0.972 | 1.33× |
| 10 | 1 | 0.075 | 0.100 | 1.33× |
| 10 | 5 | 0.360 | 0.458 | 1.27× |
| 10 | 10 | 0.709 | 1.129 | 1.59× |
| 100 | 1 | 0.096 | 0.096 | 1.00× |
| 100 | 5 | 0.342 | 0.457 | 1.34× |
| 100 | 10 | 0.739 | 0.940 | 1.27× |
| 1,000 | 1 | 0.089 | 0.100 | 1.12× |
| 1,000 | 5 | 0.345 | 0.483 | 1.40× |
| 1,000 | 10 | 0.746 | 1.003 | 1.34× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

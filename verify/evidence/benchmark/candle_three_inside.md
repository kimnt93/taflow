# CandleThreeInside benchmark (`CDL3INSIDE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.087 | 11.45M | 0.077 | 12.96M | 0.043 | 0.49× | 0.56× |
| 10,000 | 0.810 | 12.34M | 0.623 | 16.06M | 0.127 | 0.16× | 0.20× |
| 100,000 | 6.238 | 16.03M | 6.652 | 15.03M | 1.104 | 0.18× | 0.17× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.224 | 0.148 | 0.66× |
| 1 | 5 | 0.553 | 0.651 | 1.18× |
| 1 | 10 | 0.810 | 1.291 | 1.59× |
| 10 | 1 | 0.129 | 0.132 | 1.03× |
| 10 | 5 | 0.444 | 0.651 | 1.47× |
| 10 | 10 | 0.845 | 1.287 | 1.52× |
| 100 | 1 | 0.132 | 0.131 | 0.99× |
| 100 | 5 | 0.447 | 0.534 | 1.20× |
| 100 | 10 | 0.766 | 1.095 | 1.43× |
| 1,000 | 1 | 0.172 | 0.105 | 0.61× |
| 1,000 | 5 | 0.407 | 0.555 | 1.36× |
| 1,000 | 10 | 0.711 | 0.981 | 1.38× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

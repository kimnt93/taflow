# LaguerreRelativeStrengthIndex benchmark (`LaguerreRSI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 111.80M | 0.009 | 116.71M | 0.180 | 20.11× | 20.99× |
| 10,000 | 0.085 | 118.10M | 0.081 | 123.06M | 0.563 | 6.65× | 6.93× |
| 100,000 | 0.804 | 124.45M | 0.783 | 127.73M | 4.452 | 5.54× | 5.69× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.067 | 0.277 | 4.12× |
| 1 | 5 | 0.268 | 1.432 | 5.35× |
| 1 | 10 | 0.415 | 2.356 | 5.68× |
| 10 | 1 | 0.046 | 0.227 | 4.91× |
| 10 | 5 | 0.208 | 1.449 | 6.96× |
| 10 | 10 | 0.412 | 2.398 | 5.83× |
| 100 | 1 | 0.048 | 0.222 | 4.66× |
| 100 | 5 | 0.201 | 1.365 | 6.78× |
| 100 | 10 | 0.383 | 2.570 | 6.70× |
| 1,000 | 1 | 0.057 | 0.275 | 4.86× |
| 1,000 | 5 | 0.196 | 1.609 | 8.23× |
| 1,000 | 10 | 0.463 | 2.988 | 6.45× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

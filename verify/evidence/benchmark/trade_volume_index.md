# TradeVolumeIndex benchmark (`TradeVolumeIndex` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 196.77M | 0.004 | 262.79M | 0.212 | 41.70× | 55.69× |
| 10,000 | 0.064 | 156.45M | 0.059 | 170.24M | 0.781 | 12.21× | 13.29× |
| 100,000 | 0.739 | 135.38M | 0.683 | 146.48M | 6.809 | 9.22× | 9.97× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.060 | 0.302 | 5.01× |
| 1 | 5 | 0.224 | 1.040 | 4.64× |
| 1 | 10 | 0.405 | 2.332 | 5.75× |
| 10 | 1 | 0.047 | 0.222 | 4.77× |
| 10 | 5 | 0.181 | 1.233 | 6.80× |
| 10 | 10 | 0.416 | 2.372 | 5.70× |
| 100 | 1 | 0.049 | 0.213 | 4.36× |
| 100 | 5 | 0.188 | 1.274 | 6.78× |
| 100 | 10 | 0.438 | 2.457 | 5.61× |
| 1,000 | 1 | 0.056 | 0.280 | 5.02× |
| 1,000 | 5 | 0.203 | 1.649 | 8.11× |
| 1,000 | 10 | 0.434 | 2.971 | 6.84× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

# DetrendedPriceOscillator benchmark (`dpo` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 155.38M | 0.005 | 182.86M | 0.300 | 46.65× | 54.90× |
| 10,000 | 0.049 | 203.82M | 0.047 | 212.29M | 0.397 | 8.10× | 8.44× |
| 100,000 | 0.452 | 221.47M | 0.439 | 227.67M | 1.243 | 2.75× | 2.83× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.063 | 0.193 | 3.04× |
| 1 | 5 | 0.316 | 0.788 | 2.50× |
| 1 | 10 | 0.434 | 1.492 | 3.44× |
| 10 | 1 | 0.047 | 0.146 | 3.07× |
| 10 | 5 | 0.187 | 0.743 | 3.98× |
| 10 | 10 | 0.405 | 1.500 | 3.70× |
| 100 | 1 | 0.046 | 0.414 | 9.00× |
| 100 | 5 | 0.202 | 1.950 | 9.65× |
| 100 | 10 | 0.390 | 3.834 | 9.82× |
| 1,000 | 1 | 0.047 | 0.386 | 8.15× |
| 1,000 | 5 | 0.191 | 2.033 | 10.66× |
| 1,000 | 10 | 0.455 | 4.196 | 9.23× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

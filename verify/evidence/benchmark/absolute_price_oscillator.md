# AbsolutePriceOscillator benchmark (`APO` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 94.56M | 0.010 | 102.56M | 0.047 | 4.47× | 4.85× |
| 10,000 | 0.057 | 175.20M | 0.053 | 189.94M | 0.085 | 1.49× | 1.62× |
| 100,000 | 0.515 | 194.32M | 0.497 | 201.28M | 0.501 | 0.97× | 1.01× |
| 1,000,000 | 5.566 | 179.68M | 5.030 | 198.81M | 5.263 | 0.95× | 1.05× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.123 | 0.144 | 1.16× |
| 1 | 5 | 0.414 | 0.558 | 1.35× |
| 1 | 10 | 0.584 | 1.176 | 2.01× |
| 10 | 1 | 0.057 | 0.103 | 1.80× |
| 10 | 5 | 0.235 | 0.543 | 2.31× |
| 10 | 10 | 0.590 | 1.153 | 1.95× |
| 100 | 1 | 0.057 | 0.095 | 1.65× |
| 100 | 5 | 0.256 | 0.520 | 2.03× |
| 100 | 10 | 0.559 | 1.263 | 2.26× |
| 1,000 | 1 | 0.079 | 0.129 | 1.63× |
| 1,000 | 5 | 0.473 | 0.649 | 1.37× |
| 1,000 | 10 | 0.587 | 1.264 | 2.15× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

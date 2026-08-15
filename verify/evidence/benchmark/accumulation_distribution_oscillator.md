# AccumulationDistributionOscillator benchmark (`ADOSC` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 125.58M | 0.006 | 165.15M | 0.063 | 7.88× | 10.37× |
| 10,000 | 0.050 | 200.55M | 0.045 | 224.30M | 0.060 | 1.20× | 1.34× |
| 100,000 | 0.495 | 202.01M | 0.547 | 182.84M | 0.337 | 0.68× | 0.62× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.061 | 0.150 | 2.46× |
| 1 | 5 | 0.310 | 0.585 | 1.88× |
| 1 | 10 | 0.438 | 0.979 | 2.23× |
| 10 | 1 | 0.045 | 0.093 | 2.06× |
| 10 | 5 | 0.183 | 0.465 | 2.54× |
| 10 | 10 | 0.411 | 1.006 | 2.45× |
| 100 | 1 | 0.049 | 0.095 | 1.93× |
| 100 | 5 | 0.187 | 0.451 | 2.42× |
| 100 | 10 | 0.416 | 1.044 | 2.51× |
| 1,000 | 1 | 0.049 | 0.094 | 1.91× |
| 1,000 | 5 | 0.218 | 0.480 | 2.20× |
| 1,000 | 10 | 0.416 | 0.994 | 2.39× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

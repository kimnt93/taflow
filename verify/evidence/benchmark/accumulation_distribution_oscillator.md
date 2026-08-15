# AccumulationDistributionOscillator benchmark (`ADOSC` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 186.55M | 0.004 | 276.96M | 0.039 | 7.30× | 10.84× |
| 10,000 | 0.032 | 315.10M | 0.027 | 377.23M | 0.060 | 1.89× | 2.26× |
| 100,000 | 0.281 | 356.03M | 0.244 | 410.07M | 0.300 | 1.07× | 1.23× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.064 | 0.104 | 1.63× |
| 1 | 5 | 0.249 | 0.561 | 2.25× |
| 1 | 10 | 0.409 | 1.022 | 2.50× |
| 10 | 1 | 0.041 | 0.094 | 2.27× |
| 10 | 5 | 0.186 | 0.487 | 2.61× |
| 10 | 10 | 0.424 | 1.154 | 2.72× |
| 100 | 1 | 0.083 | 0.113 | 1.37× |
| 100 | 5 | 0.198 | 0.534 | 2.69× |
| 100 | 10 | 0.396 | 1.076 | 2.71× |
| 1,000 | 1 | 0.064 | 0.099 | 1.54× |
| 1,000 | 5 | 0.233 | 0.514 | 2.20× |
| 1,000 | 10 | 0.444 | 1.021 | 2.30× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

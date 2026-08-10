# InstantaneousTrendline benchmark (`InstantaneousTrendline` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 80.96M | 0.008 | 126.47M | 0.167 | 13.54× | 21.15× |
| 10,000 | 0.072 | 138.63M | 0.060 | 165.53M | 0.486 | 6.73× | 8.04× |
| 100,000 | 0.557 | 179.38M | 0.502 | 199.09M | 3.528 | 6.33× | 7.02× |
| 1,000,000 | 5.448 | 183.54M | 5.279 | 189.44M | 38.606 | 7.09× | 7.31× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.062 | 0.315 | 5.11× |
| 1 | 5 | 0.290 | 1.129 | 3.89× |
| 1 | 10 | 0.548 | 2.303 | 4.20× |
| 10 | 1 | 0.056 | 0.196 | 3.53× |
| 10 | 5 | 0.250 | 0.984 | 3.93× |
| 10 | 10 | 0.499 | 2.198 | 4.40× |
| 100 | 1 | 0.059 | 0.200 | 3.41× |
| 100 | 5 | 0.231 | 0.967 | 4.18× |
| 100 | 10 | 0.491 | 8.616 | 17.56× |
| 1,000 | 1 | 0.066 | 0.224 | 3.38× |
| 1,000 | 5 | 0.256 | 1.201 | 4.70× |
| 1,000 | 10 | 0.520 | 2.506 | 4.82× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

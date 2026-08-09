# LogReturn benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 100.07M | 0.009 | 111.77M | 0.191 | 19.09× | 21.32× |
| 10,000 | 0.074 | 135.88M | 0.072 | 138.60M | 0.222 | 3.01× | 3.07× |
| 100,000 | 0.700 | 142.95M | 0.676 | 148.01M | 0.640 | 0.91× | 0.95× |
| 1,000,000 | 11.613 | 86.11M | 6.774 | 147.63M | 8.263 | 0.71× | 1.22× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.074 | 0.261 | 3.53× |
| 1 | 5 | 0.345 | 1.280 | 3.71× |
| 1 | 10 | 0.462 | 2.402 | 5.20× |
| 10 | 1 | 0.053 | 0.241 | 4.52× |
| 10 | 5 | 0.226 | 1.200 | 5.31× |
| 10 | 10 | 0.500 | 2.531 | 5.06× |
| 100 | 1 | 0.055 | 0.248 | 4.51× |
| 100 | 5 | 0.227 | 1.210 | 5.32× |
| 100 | 10 | 0.479 | 2.445 | 5.10× |
| 1,000 | 1 | 0.056 | 0.250 | 4.47× |
| 1,000 | 5 | 0.236 | 1.292 | 5.48× |
| 1,000 | 10 | 0.505 | 2.734 | 5.42× |

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.250 | 0.171 | 5.84M | nan | — | — |
| 100,000 | 10 | 1.009 | 0.540 | 18.51M | nan | — | — |
| 100,000 | 1,000 | 8.892 | 8.397 | 119.08M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 122.22M | 112.54M | 1.00× | 3.31M | 3.85M | 1.00× | — |
| 5 | 219.08M | 271.16M | 2.41× | 2.33M | 2.57M | 0.67× | — |
| 10 | 374.44M | 417.67M | 3.71× | 2.26M | 2.40M | 0.62× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

# ParabolicSar benchmark (`SAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 100.97M | 0.009 | 107.42M | 0.039 | 3.95× | 4.21× |
| 10,000 | 0.104 | 96.15M | 0.100 | 100.18M | 0.099 | 0.95× | 0.99× |
| 100,000 | 1.093 | 91.45M | 1.147 | 87.16M | 0.773 | 0.71× | 0.67× |
| 1,000,000 | 12.750 | 78.43M | 10.701 | 93.45M | 6.423 | 0.50× | 0.60× |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.076 ms**; native kernel **1.018 ms**; TA-Lib 0.690 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.295 | 0.196 | 5.10M | 676.032 | 3445.94× | 197.33× |
| 100,000 | 10 | 1.133 | 1.079 | 9.27M | 644.066 | 597.10× | 29.93× |
| 100,000 | 1,000 | 16.529 | 12.243 | 81.68M | 684.063 | 55.88× | 3.36× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 80.23M | 92.45M | 1.00× | 2.66M | 3.17M | 1.00× | 130.08M |
| 2 | 154.26M | 173.68M | 1.88× | 2.53M | 3.30M | 1.04× | 131.44M |
| 4 | 266.40M | 327.18M | 3.54× | 2.43M | 2.64M | 0.83× | 125.68M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

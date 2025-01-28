import time
from statistics import mean, median
from PIL import Image
import imgdd as dd
import imagehash
import os


def collect_image_count(path: str) -> int:
    """Count number of images in given directory."""
    return sum(
        len(files) for _, _, files in os.walk(path)
        if any(file.lower().endswith((".png", ".jpg", ".jpeg")) for file in files)
    )

def benchmark_function(func, num_runs=50, warmup=3, **kwargs):
    """Benchmark a function and return timing metrics."""
    for _ in range(warmup):  # Warm-up runs
        func(**kwargs)
    
    timings = []
    for _ in range(num_runs):
        start_time = time.perf_counter_ns()
        func(**kwargs)
        end_time = time.perf_counter_ns()
        timings.append(end_time - start_time)
    
    return {
        "min_time": min(timings),
        "max_time": max(timings),
        "avg_time": mean(timings),
        "median_time": median(timings),
    }


def imgdd_benchmark(path: str, algo: str, num_runs: int, num_images: int) -> dict:
    """Benchmark imgdd library."""
    def run_imgdd_hash():
        dd.hash(path=path, algo=algo, filter="Nearest", sort=False)

    results = benchmark_function(run_imgdd_hash, num_runs=num_runs)
    for key in results:
        results[key] /= num_images  # Convert to per-image timing
    return results


def imagehash_benchmark(path: str, algo: str, num_runs: int, num_images: int) -> dict:
    """Benchmark imagehash library."""
    def run_imagehash(algo: str):
        for root, _, files in os.walk(path):
            for file in files:
                file_path = os.path.join(root, file)
                try:
                    algo = algo.lower()
                    image = Image.open(file_path)
                    if algo == "ahash":
                        imagehash.average_hash(image)
                    elif algo == "phash":
                        imagehash.phash(image)
                    elif algo == "dhash":
                        imagehash.dhash(image)
                    elif algo == "whash":
                        imagehash.whash(image)
                    else:
                        raise ValueError(f"Unsupported algorithm: {algo}")
                except Exception as e:
                    print(f"Error processing {file_path}: {e}")
    
    results = benchmark_function(run_imagehash, num_runs=num_runs, algo=algo)
    for key in results:
        results[key] /= num_images  # Convert to per-image timing
    return results


def compare_benchmarks(imgdd_result: dict, imagehash_result: dict, algo: str):
    """Prints a comparison of benchmark results."""
    print(f"Benchmark Results for {algo} (in seconds per image):\n")
    print(f"{'Metric':<12}{'imgdd (ns)':<12}{'imagehash (ns)':<12}")
    print("-" * 46)
    for metric in ["min_time", "max_time", "avg_time", "median_time"]:
        print(f"{metric:<12}{imgdd_result[metric]:<12.6f} {imagehash_result[metric]:<12.6f}")


def calc_diff(imgdd_result: dict, imagehash_result: dict):
    """Calculate and print the percentage difference for each metric."""
    print("\nPercentage Difference (imgdd vs. imagehash):\n")
    print(f"{'Metric':<12}{'Difference (%)':<15}")
    print("-" * 28)
    for metric in ["min_time", "max_time", "avg_time", "median_time"]:
        difference = ((imagehash_result[metric] - imgdd_result[metric]) / imagehash_result[metric]) * 100
        print(f"{metric:<12}{difference:<15.2f}")
    print("\n")


if __name__ == "__main__":
    IMAGE_DIR = "../../../imgs/test/"
    ALGORITHMS = ["dHash", "aHash", "pHash", "wHash"] # mHash has no equivalent in imagehash
    NUM_RUNS = 100
    WARM_UP = 5

    num_images = collect_image_count(IMAGE_DIR)
    if num_images == 0:
        raise ValueError("No images found in directory.")

    print(f"Found {num_images} images in {IMAGE_DIR}. Running benchmarks for {NUM_RUNS} runs...\n")

    for algo in ALGORITHMS:
        print(f"Benchmarking {algo}...\n")
        
        # Benchmark imgdd
        imgdd_result = imgdd_benchmark(IMAGE_DIR, algo, NUM_RUNS, num_images)

        # Benchmark imagehash
        imagehash_result = imagehash_benchmark(IMAGE_DIR, algo, NUM_RUNS, num_images)

        # Compare results
        compare_benchmarks(imgdd_result, imagehash_result, algo)

        # Calculate percentage difference
        calc_diff(imgdd_result, imagehash_result)
import time
from statistics import mean, median
from PIL import Image
import imgdd as dd
import imagehash
import os


def collect_image_count(path: str) -> int:
    """Count the number of images in given directory."""
    return sum(
        len(files) for _, _, files in os.walk(path)
        if any(file.lower().endswith((".png", ".jpg", ".jpeg")) for file in files)
    )


def benchmark_function(func, num_runs=10, **kwargs) -> dict:
    """Benchmark a function and return timing metrics."""
    timings = []
    for _ in range(num_runs):
        start_time = time.perf_counter()
        func(**kwargs)
        end_time = time.perf_counter()
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
        dd.hash(path=path, algo=algo)

    results = benchmark_function(run_imgdd_hash, num_runs=num_runs)
    for key in results:
        results[key] /= num_images  # Convert to per-image timing
    return results


def imagehash_benchmark(path: str, algo: str, num_runs: int, num_images: int) -> dict:
    """Benchmark imagehash library."""
    def run_imagehash():
        for root, _, files in os.walk(path):
            for file in files:
                file_path = os.path.join(root, file)
                try:
                    image = Image.open(file_path)
                    if algo == "aHash":
                        imagehash.average_hash(image)
                    elif algo == "pHash":
                        imagehash.phash(image)
                    elif algo == "dHash":
                        imagehash.dhash(image)
                    elif algo == "wHash":
                        imagehash.whash(image)
                    else:
                        raise ValueError(f"Unsupported algorithm: {algo}")
                except Exception as e:
                    print(f"Error processing {file_path}: {e}")
    
    results = benchmark_function(run_imagehash, num_runs=num_runs)
    for key in results:
        results[key] /= num_images  # Convert to per-image timing
    return results


def compare_benchmarks(imgdd_result: dict, imagehash_result: dict):
    """Prints a comparison of benchmark results."""
    print("Benchmark Results (in seconds per image):\n")
    print(f"{'Metric':<12}{'imgdd':<12}{'imagehash':<12}")
    print("-" * 36)
    for metric in ["min_time", "max_time", "avg_time", "median_time"]:
        print(f"{metric:<12}{imgdd_result[metric]:<12.6f}{imagehash_result[metric]:<12.6f}")


def calc_diff(imgdd_result: dict, imagehash_result: dict):
    """Calculate and print the percentage difference for each metric."""
    print("\nPercentage Difference (imgdd vs. imagehash):\n")
    print(f"{'Metric':<12}{'Difference (%)':<15}")
    print("-" * 28)
    for metric in ["min_time", "max_time", "avg_time", "median_time"]:
        difference = ((imagehash_result[metric] - imgdd_result[metric]) / imagehash_result[metric]) * 100
        print(f"{metric:<12}{difference:<15.2f}")


if __name__ == "__main__":
    IMAGE_DIR = "../imgs/test/"
    ALGORITHM = "dHash" 
    NUM_RUNS = 50

    num_images = collect_image_count(IMAGE_DIR)
    if num_images == 0:
        print("No images found in the directory.")
        exit(1)

    print(f"Found {num_images} images in {IMAGE_DIR}. Running benchmarks for {NUM_RUNS} runs...\n")

    # Benchmark imgdd
    imgdd_result = imgdd_benchmark(IMAGE_DIR, ALGORITHM, NUM_RUNS, num_images)
    # print(f"imgdd benchmark result (per image): {imgdd_result}\n")

    # Benchmark imagehash
    imagehash_result = imagehash_benchmark(IMAGE_DIR, ALGORITHM, NUM_RUNS, num_images)
    # print(f"imagehash benchmark result (per image): {imagehash_result}\n")

    # Compare results
    compare_benchmarks(imgdd_result, imagehash_result)

    # Calculate difference in percentage
    calc_diff(imgdd_result, imagehash_result)

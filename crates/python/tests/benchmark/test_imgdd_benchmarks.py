import pytest
import imgdd as dd
from pathlib import Path

# Directory for testing
TEST_IMAGE_DIR = Path(__file__).parent / "../../../../imgs/test/single"

@pytest.mark.benchmark
def test_imgdd_hash_benchmark(benchmark):
    result = benchmark(dd.hash, path=str(TEST_IMAGE_DIR), algo="dHash", filter="triangle", sort=False)
    assert isinstance(result, dict), "Expected a dictionary of hashes"

@pytest.mark.benchmark
def test_imgdd_dupes_benchmark(benchmark):
    result = benchmark(dd.dupes, path=str(TEST_IMAGE_DIR), algo="aHash", filter="nearest", remove=False)
    assert isinstance(result, dict), "Expected a dictionary of duplicates"

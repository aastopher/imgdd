import pytest
import imgdd as dd
from pathlib import Path

# Directory for testing
TEST_IMAGE_DIR = Path(__file__).parent / "../../../../imgs/test/"

# WARNING! 
# dd.hash function benchmarks will be inaccurate because; this metric relies heavily on system calls.
# Since they cannot be consistently instrumented, those calls are not included in the final measure.
# To resolve this we must use hosted codspeed macro runners which require a pro plan. 
# For now I will just leave this warning here.

# aHash
@pytest.mark.benchmark
def test_imgdd_hash_ahash_benchmark(benchmark):
    result = benchmark(dd.hash, path=str(TEST_IMAGE_DIR), algo="aHash", filter="triangle", sort=False)
    assert isinstance(result, dict), "Expected a dictionary of hashes"
    assert len(result) > 0, "Expected non-empty hash results"

@pytest.mark.benchmark
def test_imgdd_dupes_ahash_benchmark(benchmark):
    result = benchmark(dd.dupes, path=str(TEST_IMAGE_DIR), algo="aHash", filter="triangle", remove=False)
    assert isinstance(result, dict), "Expected a dictionary of duplicates"


# mHash
@pytest.mark.benchmark
def test_imgdd_hash_mhash_benchmark(benchmark):
    result = benchmark(dd.hash, path=str(TEST_IMAGE_DIR), algo="mHash", filter="triangle", sort=False)
    assert isinstance(result, dict), "Expected a dictionary of hashes"
    assert len(result) > 0, "Expected non-empty hash results"

@pytest.mark.benchmark
def test_imgdd_dupes_mhash_benchmark(benchmark):
    result = benchmark(dd.dupes, path=str(TEST_IMAGE_DIR), algo="mHash", filter="triangle", remove=False)
    assert isinstance(result, dict), "Expected a dictionary of duplicates"


# dHash
@pytest.mark.benchmark
def test_imgdd_hash_dhash_benchmark(benchmark):
    result = benchmark(dd.hash, path=str(TEST_IMAGE_DIR), algo="dHash", filter="triangle", sort=False)
    assert isinstance(result, dict), "Expected a dictionary of hashes"
    assert len(result) > 0, "Expected non-empty hash results"

@pytest.mark.benchmark
def test_imgdd_dupes_dhash_benchmark(benchmark):
    result = benchmark(dd.dupes, path=str(TEST_IMAGE_DIR), algo="dHash", filter="triangle", remove=False)
    assert isinstance(result, dict), "Expected a dictionary of duplicates"


# pHash
@pytest.mark.benchmark
def test_imgdd_hash_phash_benchmark(benchmark):
    result = benchmark(dd.hash, path=str(TEST_IMAGE_DIR), algo="pHash", filter="triangle", sort=False)
    assert isinstance(result, dict), "Expected a dictionary of hashes"
    assert len(result) > 0, "Expected non-empty hash results"

@pytest.mark.benchmark
def test_imgdd_dupes_phash_benchmark(benchmark):
    result = benchmark(dd.dupes, path=str(TEST_IMAGE_DIR), algo="pHash", filter="triangle", remove=False)
    assert isinstance(result, dict), "Expected a dictionary of duplicates"


# wHash
@pytest.mark.benchmark
def test_imgdd_hash_whash_benchmark(benchmark):
    result = benchmark(dd.hash, path=str(TEST_IMAGE_DIR), algo="wHash", filter="triangle", sort=False)
    assert isinstance(result, dict), "Expected a dictionary of hashes"
    assert len(result) > 0, "Expected non-empty hash results"

@pytest.mark.benchmark
def test_imgdd_dupes_whash_benchmark(benchmark):
    result = benchmark(dd.dupes, path=str(TEST_IMAGE_DIR), algo="wHash", filter="triangle", remove=False)
    assert isinstance(result, dict), "Expected a dictionary of duplicates"

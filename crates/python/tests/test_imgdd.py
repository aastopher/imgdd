# crates/python/tests/test_imgdd.py
import imgdd as dd
from pathlib import Path
import pytest


@pytest.fixture
def test_images_path():
    return Path(__file__).parent / "../../../imgs/test" 


def test_hash(test_images_path):
    results = dd.hash(path=str(test_images_path), algo="dhash")
    assert isinstance(results, dict), "Expected a dictionary of hashes"
    assert len(results) > 0, "Expected non-empty hash results"


def test_dupes(test_images_path):
    duplicates = dd.dupes(path=str(test_images_path), algo="dhash", remove=False)
    assert isinstance(duplicates, dict), "Expected a dictionary of duplicates"
    assert len(duplicates) >= 0, "Expected no errors for duplicates"

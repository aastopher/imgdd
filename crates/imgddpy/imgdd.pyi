from typing import Literal, Dict, Optional

def hash(
    path: str,
    filter: Literal["Nearest", "Triangle", "CatmullRom", "Gaussian", "Lanczos3"] = "Nearest",
    algo: Literal["aHash", "mHash", "dHash", "pHash", "wHash"] = "dHash",
    hash_size: Optional[int] = None,
    sort: bool = False,
) -> Dict[str, str]:
    """
    Calculate the hash of images in a directory.

    Args:
        path (str): Path to the directory containing images.
        filter (str): Resize filter to use.
        algo (str): Hashing algorithm.
        hash_size (int): Hash size for pHash algorithm (e.g., 8, 16). Only used for pHash.
        sort (bool): Whether to sort the results by hash values.

    Returns:
        Dict[str, str]: A dictionary mapping file paths to their hashes.
    """
    ...

def dupes(
    path: str,
    filter: Literal["Nearest", "Triangle", "CatmullRom", "Gaussian", "Lanczos3"] = "Nearest",
    algo: Literal["aHash", "mHash", "dHash", "pHash", "wHash"] = "dHash",
    hash_size: Optional[int] = None,
    remove: bool = False,
) -> Dict[str, list[str]]:
    """
    Find duplicate images in a directory.

    Args:
        path (str): Path to the directory containing images.
        filter (str): Resize filter to use.
        algo (str): Hashing algorithm.
        hash_size (int): Hash size for pHash algorithm (e.g., 8, 16). Only used for pHash.
        remove (bool): Whether to remove duplicate files.

    Returns:
        Dict[str, list[str]]: A dictionary mapping hashes to lists of file paths.
    """
    ...

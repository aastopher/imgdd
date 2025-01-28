from typing import Literal, Dict

def hash(
    path: str,
    filter: Literal["Nearest", "Triangle", "CatmullRom", "Gaussian", "Lanczos3"] = "Nearest",
    algo: Literal["aHash", "mHash", "dHash", "pHash", "wHash"] = "dHash",
    sort: bool = False,
) -> Dict[str, str]:
    """
    Calculate the hash of images in a directory.

    Args:
        path (str): Path to the directory containing images.
        filter (str): Resize filter to use.
        algo (str): Hashing algorithm.

    Returns:
        Dict[str, str]: A dictionary mapping file paths to their hashes.
    """
    ...

def dupes(
    path: str,
    filter: Literal["Nearest", "Triangle", "CatmullRom", "Gaussian", "Lanczos3"] = "Nearest",
    algo: Literal["aHash", "mHash", "dHash", "pHash", "wHash"] = "dHash",
    remove: bool = False,
) -> Dict[str, list[str]]:
    """
    Find duplicate images in a directory.

    Args:
        path (str): Path to the directory containing images.
        filter (str): Resize filter to use.
        algo (str): Hashing algorithm.
        remove (bool): Whether to remove duplicate files.

    Returns:
        Dict[str, list[str]]: A dictionary mapping hashes to lists of file paths.
    """
    ...
